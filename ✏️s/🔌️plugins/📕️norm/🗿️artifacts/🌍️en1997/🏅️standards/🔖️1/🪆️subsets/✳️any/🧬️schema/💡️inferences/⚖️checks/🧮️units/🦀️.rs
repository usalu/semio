//! 🧮️ EN 1997 check units in exactly the order of `check_project`: integrity, soil layers, footings, piles, retaining walls, slopes, uplift cases, embedded walls, anchors, pile groups, laboratory tests and layer waters, each group after its not-applicable verdict; the governing summary is the post-fold aggregate. Slopes run as slow (T2) units whose slip-circle search reports progress and polls the cancel token.

use super::{check_project_scoped, governing_summary};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, Scope, UnitSource};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult};

/// 🧮️ The unit source of the EN 1997 geotechnical subject: `SharedEvaluation<En1997Units>` is the one evaluation path of editor and viewer.
pub struct En1997Units;

type Group = Rows<En1997Snapshot>;

const ROW_COLLECTIONS: [&str; 15] = ["layers", "footings", "piles", "retainingWalls", "slopes", "upliftCases", "anchors", "embeddedWalls", "pileGroups", "labTests", "layerWaters", "cases", "placements", "investigations", "characteristicBases"];
const LAYERS: [&str; 3] = ["layers", "investigations", "characteristicBases"];
const GROUND: [&str; 4] = ["layers", "investigations", "characteristicBases", "footings"];

fn part(rows: Group, name: &'static str) -> Group {
    rows.part(name, move |d| check_project_scoped(d, Scope::Part(name), &mut |_| true).checks)
}

fn group<T>(mut rows: Group, items: &[T], tag: &'static str, id: impl Fn(&T) -> &str, tier: CostTier, extra: &[&str]) -> Group {
    for (index, item) in items.iter().enumerate() {
        rows = rows.row(tag, tag, index, id(item), tier, extra, move |d: &En1997Snapshot| check_project_scoped(d, Scope::Row(tag, index), &mut |_| true).checks);
    }
    rows
}

impl UnitSource for En1997Units {
    type Document = En1997Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["layers", "footings", "piles", "retainingWalls", "slopes", "upliftCases", "embeddedWalls", "anchors", "pileGroups", "labTests", "layerWaters", "characteristicBases", "investigations", "cases"], &["en1997.6.6.pairs.na", "en1997.gk.category", "en1997.design.", "en1997.footing.na", "en1997.pile.na", "en1997.wall.na", "en1997.slope.na", "en1997.upl.na", "en1997.governing.", "en1997.integrity."]);

    fn units(d: &En1997Snapshot) -> Vec<CheckUnit<En1997Snapshot>> {
        let mut rows = Rows::except(d, &ROW_COLLECTIONS).global(|d| check_project_scoped(d, Scope::Part("integrity"), &mut |_| true).checks);
        rows = part(rows, "investigation-extent");
        rows = group(rows, &d.layers, "layers", |e| e.id.as_str(), CostTier::T0, &LAYERS);
        rows = part(rows, "footing-na");
        rows = group(rows, &d.footings, "footings", |e| e.id.as_str(), CostTier::T1, &LAYERS);
        rows = part(rows, "sls-pairs");
        rows = part(rows, "pile-na");
        rows = group(rows, &d.piles, "piles", |e| e.id.as_str(), CostTier::T1, &GROUND);
        rows = part(rows, "wall-na");
        rows = group(rows, &d.retaining_walls, "retainingWalls", |e| e.id.as_str(), CostTier::T0, &GROUND);
        rows = part(rows, "slope-na");
        for (index, slope) in d.slopes.iter().enumerate() {
            rows = rows.slow("slopes", "slopes", index, &slope.id, CostTier::T2, &GROUND, move |d: &En1997Snapshot, progress, token| {
                check_project_scoped(d, Scope::Row("slopes", index), &mut |fraction| {
                    progress.report(fraction);
                    !token.is_cancelled()
                })
                .checks
            });
        }
        rows = part(rows, "upl-na");
        rows = group(rows, &d.uplift_cases, "upliftCases", |e| e.id.as_str(), CostTier::T0, &GROUND);
        rows = group(rows, &d.embedded_walls, "embeddedWalls", |e| e.id.as_str(), CostTier::T1, &LAYERS);
        rows = group(rows, &d.anchors, "anchors", |e| e.id.as_str(), CostTier::T0, &["embeddedWalls"]);
        rows = group(rows, &d.pile_groups, "pileGroups", |e| e.id.as_str(), CostTier::T0, &["piles"]);
        rows = group(rows, &d.lab_tests, "labTests", |e| e.id.as_str(), CostTier::T0, &LAYERS);
        rows = group(rows, &d.layer_waters, "layerWaters", |e| e.id.as_str(), CostTier::T0, &LAYERS);
        rows = group(rows, &d.characteristic_bases, "characteristicBases", |e| e.id.as_str(), CostTier::T0, &["layers"]);
        rows = group(rows, &d.investigations, "investigations", |e| e.id.as_str(), CostTier::T0, &["layers"]);
        rows = group(rows, &d.cases, "cases", |e| e.id.as_str(), CostTier::T0, &[]);
        rows.finish()
    }

    fn summarise(d: &En1997Snapshot, report: &CheckReport) -> Vec<CheckResult> {
        governing_summary(d, report)
    }
}
