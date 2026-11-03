//! 🧮️ EN 1991 check units in exactly the order of `check_full_actions`: integrity, then per action family its document-level part and its rows (floors, self-weight elements, roofs, wind faces, accidental cases) and, after the traffic part, the rows of the bridge lanes, construction stages and roof surfaces; scalar inputs are shared reads of every row.

use super::{check_full_actions_scoped, Scope};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};

/// 🧮️ The unit source of the EN 1991 actions subject: `SharedEvaluation<En1991Units>` is the one evaluation path of editor and viewer.
pub struct En1991Units;

type Group = Rows<En1991Snapshot>;

const ROW_COLLECTIONS: [&str; 35] = ["floors", "selfWeightElements", "roofs", "windFaces", "accidentalCases", "bridgeLanes", "constructionStages", "roofSurfaces", "specialVehicles", "windBridges", "dynamicElements", "impactActions", "internalExplosions", "thermalSections", "cases", "placements", "helicopterLandings", "tieRuns", "constructionLoads", "fallingLoads", "executionImperfections", "derailmentCases", "craneHoists", "footbridges", "railDynamics", "loadDispersals", "bulkSolids", "rotatingMachines", "fatigueModels", "supportCollisions", "localFires", "snowGuards", "railDerailments", "railLoadGroups", "dustExplosions"];

/// 📖️ What the integrity unit reads besides the shared scalars: the id column of every row collection it checks for duplicates and the lane and roof-surface rows of the collection integrity (lane numbers, offsets, widths, faces, extents); a floor or load edit leaves it clean.
const INTEGRITY_READS: [&str; 35] = ["floors.*.id", "selfWeightElements.*.id", "roofs.*.id", "windFaces.*.id", "accidentalCases.*.id", "constructionStages.*.id", "placements.*.id", "specialVehicles.*.id", "windBridges.*.id", "dynamicElements.*.id", "impactActions.*.id", "internalExplosions.*.id", "thermalSections.*.id", "cases.*.id", "helicopterLandings.*.id", "tieRuns.*.id", "constructionLoads.*.id", "fallingLoads.*.id", "executionImperfections.*.id", "derailmentCases.*.id", "craneHoists.*.id", "footbridges.*.id", "railDynamics.*.id", "loadDispersals.*.id", "bulkSolids.*.id", "rotatingMachines.*.id", "dustExplosions.*.id", "railLoadGroups.*.id", "railDerailments.*.id", "fatigueModels.*.id", "supportCollisions.*.id", "localFires.*.id", "snowGuards.*.id", "bridgeLanes", "roofSurfaces"];

fn part(rows: Group, name: &'static str, extra: &[&str]) -> Group {
    rows.part_sharing(name, extra, move |d| check_full_actions_scoped(d, Scope::Part(name)).checks)
}

fn group<T>(mut rows: Group, items: &[T], tag: &'static str, id: impl Fn(&T) -> &str, extra: &[&str]) -> Group {
    for (index, item) in items.iter().enumerate() {
        rows = rows.row(tag, tag, index, id(item), CostTier::T0, extra, move |d: &En1991Snapshot| check_full_actions_scoped(d, Scope::Row(tag, index)).checks);
    }
    rows
}

impl UnitSource for En1991Units {
    type Document = En1991Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::AS_EMITTED;

    fn units(d: &En1991Snapshot) -> Vec<CheckUnit<En1991Snapshot>> {
        let mut rows = Rows::except(d, &ROW_COLLECTIONS).global_sharing(&INTEGRITY_READS, |d| check_full_actions_scoped(d, Scope::Part("integrity")).checks);
        rows = part(rows, "floors-none", &["floors.*.id"]);
        rows = group(rows, &d.floors, "floors", |e| e.id.as_str(), &[]);
        rows = part(rows, "self-weight-none", &["selfWeightElements.*.id"]);
        rows = group(rows, &d.self_weight_elements, "selfWeightElements", |e| e.id.as_str(), &[]);
        rows = part(rows, "fire", &[]);
        rows = part(rows, "snow", &["roofs.*.id"]);
        rows = group(rows, &d.roofs, "roofs", |e| e.id.as_str(), &[]);
        rows = part(rows, "wind", &["windFaces.*.id"]);
        rows = group(rows, &d.wind_faces, "windFaces", |e| e.id.as_str(), &["roofs"]);
        rows = part(rows, "thermal", &[]);
        rows = part(rows, "construction", &[]);
        rows = part(rows, "accidental-none", &["accidentalCases.*.id"]);
        rows = group(rows, &d.accidental_cases, "accidentalCases", |e| e.id.as_str(), &[]);
        rows = part(rows, "traffic", &["bridgeLanes.*.width"]);
        rows = group(rows, &d.bridge_lanes, "bridgeLanes", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.construction_stages, "constructionStages", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.roof_surfaces, "roofSurfaces", |e| e.id.as_str(), &["windFaces", "roofs", "roofSurfaces"]);
        rows = group(rows, &d.special_vehicles, "specialVehicles", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.wind_bridges, "windBridges", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.dynamic_elements, "dynamicElements", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.impact_actions, "impactActions", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.internal_explosions, "internalExplosions", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.thermal_sections, "thermalSections", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.helicopter_landings, "helicopterLandings", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.tie_runs, "tieRuns", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.construction_loads, "constructionLoads", |e| e.id.as_str(), &["constructionLoads", "constructionStages"]);
        rows = group(rows, &d.falling_loads, "fallingLoads", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.execution_imperfections, "executionImperfections", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.derailment_cases, "derailmentCases", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.crane_hoists, "craneHoists", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.footbridges, "footbridges", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.rail_dynamics, "railDynamics", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.load_dispersals, "loadDispersals", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.bulk_solids, "bulkSolids", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.rotating_machines, "rotatingMachines", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.dust_explosions, "dustExplosions", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.rail_load_groups, "railLoadGroups", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.rail_derailments, "railDerailments", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.fatigue_models, "fatigueModels", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.support_collisions, "supportCollisions", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.local_fires, "localFires", |e| e.id.as_str(), &[]);
        rows = group(rows, &d.snow_guards, "snowGuards", |e| e.id.as_str(), &[]);
        group(rows, &d.cases, "cases", |e| e.id.as_str(), &[]).finish()
    }
}
