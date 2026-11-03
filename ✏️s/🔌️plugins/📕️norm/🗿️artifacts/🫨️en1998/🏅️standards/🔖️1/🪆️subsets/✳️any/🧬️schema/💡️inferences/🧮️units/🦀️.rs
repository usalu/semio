//! 🧮️ EN 1998 check units in exactly the order of `check_full_seismic`: integrity (annex gate, site, references), then per part its not-applicable verdict and its rows; buildings run as slow (T2) units that poll the cancel token and report progress.

use super::{check_full_seismic_scoped, Scope};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};

/// 🧮️ The unit source of the EN 1998 seismic subject: `SharedEvaluation<En1998Units>` is the one evaluation path of editor and viewer.
pub struct En1998Units;

type Group = Rows<En1998Snapshot>;

const ROW_COLLECTIONS: [&str; 18] = ["buildings", "bridges", "assessments", "silos", "tanks", "foundations", "retainingWalls", "towers", "soilLayers", "diaphragms", "accelerograms", "piers", "bearings", "placements", "tieBeams", "slopes", "isolationSystems", "pierBases"];

fn part(rows: Group, name: &'static str) -> Group {
    rows.part(name, move |d| check_full_seismic_scoped(d, Scope::Part(name), &mut |_| true).checks)
}

fn group<T>(mut rows: Group, items: &[T], tag: &'static str, id: impl Fn(&T) -> &str, tier: CostTier, extra: &[&str]) -> Group {
    for (index, item) in items.iter().enumerate() {
        rows = rows.row(tag, tag, index, id(item), tier, extra, move |d: &En1998Snapshot| check_full_seismic_scoped(d, Scope::Row(tag, index), &mut |_| true).checks);
    }
    rows
}

impl UnitSource for En1998Units {
    type Document = En1998Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["buildings", "bridges", "assessments", "silos", "tanks", "foundations", "retainingWalls", "towers", "soilLayers", "diaphragms", "accelerograms", "piers", "bearings", "tieBeams", "slopes", "isolationSystems", "pierBases", "cases"], &["en1998.1.buildings.na", "en1998.2.na", "en1998.3.na", "en1998.4.silo.na", "en1998.4.tank.na", "en1998.5.foundation.na", "en1998.5.wall.na", "en1998.5.tie-beam.na", "en1998.5.slope.na", "en1998.6.na", "en1998.cancelled", "en1998.site.", "en1998.integrity."]);

    fn units(d: &En1998Snapshot) -> Vec<CheckUnit<En1998Snapshot>> {
        let mut rows = Rows::except(d, &ROW_COLLECTIONS).global(|d| check_full_seismic_scoped(d, Scope::Part("integrity"), &mut |_| true).checks);
        rows = part(rows, "buildings-na");
        for (index, building) in d.buildings.iter().enumerate() {
            let next = format!("buildings.{}", index + 1);
            rows = rows.slow("buildings", "buildings", index, &building.id, CostTier::T2, &["diaphragms", "accelerograms", next.as_str()], move |d: &En1998Snapshot, progress, token| {
                check_full_seismic_scoped(d, Scope::Row("buildings", index), &mut |fraction| {
                    progress.report(fraction);
                    !token.is_cancelled()
                })
                .checks
            });
        }
        rows = part(rows, "bridges-na");
        rows = group(rows, &d.bridges, "bridges", |e| e.id.as_str(), CostTier::T0, &["piers", "bearings"]);
        rows = group(rows, &d.isolation_systems, "isolationSystems", |e| e.id.as_str(), CostTier::T0, &["bridges", "buildings"]);
        rows = group(rows, &d.pier_bases, "pierBases", |e| e.id.as_str(), CostTier::T0, &["bridges", "piers"]);
        rows = part(rows, "assessments-na");
        rows = group(rows, &d.assessments, "assessments", |e| e.id.as_str(), CostTier::T0, &["buildings"]);
        rows = part(rows, "silos-na");
        rows = group(rows, &d.silos, "silos", |e| e.id.as_str(), CostTier::T0, &[]);
        rows = part(rows, "tanks-na");
        rows = group(rows, &d.tanks, "tanks", |e| e.id.as_str(), CostTier::T0, &[]);
        rows = part(rows, "foundations-na");
        rows = group(rows, &d.foundations, "foundations", |e| e.id.as_str(), CostTier::T1, &["buildings", "soilLayers"]);
        rows = part(rows, "walls-na");
        rows = group(rows, &d.retaining_walls, "retainingWalls", |e| e.id.as_str(), CostTier::T0, &["buildings"]);
        rows = part(rows, "tie-beams-na");
        rows = group(rows, &d.tie_beams, "tieBeams", |e| e.id.as_str(), CostTier::T0, &["foundations", "buildings"]);
        rows = part(rows, "slopes-na");
        rows = group(rows, &d.slopes, "slopes", |e| e.id.as_str(), CostTier::T1, &["soilLayers"]);
        rows = part(rows, "towers-na");
        rows = group(rows, &d.towers, "towers", |e| e.id.as_str(), CostTier::T0, &[]);
        rows.finish()
    }
}
