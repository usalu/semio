//! 🔭️ Inference projections of the text facet: the canonical JSON of one inferred table of a model snapshot, by slug, which the language-neutral inference and export oracles compare.

use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::snapshot::OpeningKind;
use crate::standards::v1::subsets::any::schema::inferences::rule_results::json_number;
use semio_framework_value::DslValue;

/// 💡️ The canonical JSON of one inferred table the inference oracles compare, by slug: `storey-levels`, `wall-layout`, `opening-frames`, `stair-runs`, `spaces`, `quantities` (the closed-form kinds), `plan-metrics`, `view-metrics`, `diagnostics`, or a derived table such as `wall-solids` (base, top and volume of every straight wall) and `frame-solids`.
pub fn encode_inference_projection_json(snapshot: &ModelSnapshot, slug: &str) -> Option<String> {
    crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(None, snapshot, |inferred| projection_of(snapshot, slug, inferred)).ok().flatten()
}

fn projection_of(snapshot: &ModelSnapshot, slug: &str, inferred: &crate::standards::v1::subsets::any::schema::inferences::ModelInference) -> Option<String> {
    match slug {
        "storey-levels" => Some(semio_framework_pack_json::to_json_string(&inferred.storey_levels)),
        "wall-layout" => Some(semio_framework_pack_json::to_json_string(&inferred.wall_layout)),
        "opening-frames" => Some(semio_framework_pack_json::to_json_string(&inferred.opening_frames)),
        "stair-runs" => Some(crate::standards::v1::subsets::any::io::text::inferences::stair_runs::table_json(&inferred.stair_runs)),
        "spaces" => Some(crate::standards::v1::subsets::any::io::text::inferences::spaces::table_json(&inferred.spaces)),
        "quantities" => Some(crate::standards::v1::subsets::any::io::text::inferences::quantities::table_json(snapshot, &inferred.quantities)),
        "frame-solids" => Some(crate::standards::v1::subsets::any::io::text::inferences::element_solids::planar_projection_json(snapshot, &inferred.element_solids)),
        "plan-metrics" => Some(crate::standards::v1::subsets::any::io::text::inferences::plan_linework::metrics_json(&inferred.plan_linework)),
        "view-metrics" => Some(crate::standards::v1::subsets::any::io::text::inferences::view_linework::metrics_json(&inferred.view_linework)),
        "diagnostics" => Some(crate::standards::v1::subsets::any::io::text::inferences::diagnostics::table_json(&inferred.diagnostics)),
        "components" => Some(crate::standards::v1::subsets::any::io::text::inferences::components::table_json(inferred)),
        "ramp-runs" => Some(crate::standards::v1::subsets::any::io::text::inferences::ramp_runs::table_json(&inferred.ramp_runs)),
        "families" => Some(crate::standards::v1::subsets::any::schema::inferences::families::metrics::table_json(&inferred.families)),
        "annotations" => Some(crate::standards::v1::subsets::any::schema::inferences::annotation_layout::metrics_json(&inferred.annotations)),
        "clashes" => Some(crate::standards::v1::subsets::any::schema::inferences::clash_sets::table_json(&inferred.clash_sets)),
        "clash-meshes" => Some(crate::standards::v1::subsets::any::schema::inferences::clash_sets::meshes_json(snapshot, &inferred.element_solids)),
        "bcf" => Some(crate::standards::v1::subsets::any::io::bcf::table_json(snapshot)),
        "rules" => Some(crate::standards::v1::subsets::any::schema::inferences::rule_results::table_json(&inferred.rule_results)),
        "rule-measures" => Some(measures_json(snapshot, inferred)),
        "sheets" => Some(crate::standards::v1::subsets::any::schema::inferences::sheet_layout::metrics_json(&inferred.sheet_layouts)),
        "sheet-svg" => Some(crate::standards::v1::subsets::any::io::export::sheets::tables::svg_table_json(&crate::standards::v1::subsets::any::io::export::sheets::layouts(snapshot, inferred, None), &crate::standards::v1::subsets::any::io::export::sheets::TitleLabels::english())),
        "sheet-pdf" => Some(crate::standards::v1::subsets::any::io::export::sheets::tables::pdf_table_json(&crate::standards::v1::subsets::any::io::export::sheets::layouts(snapshot, inferred, None))),
        "finishes" => Some(crate::standards::v1::subsets::any::schema::inferences::finishes::table_json(&inferred.quantities)),
        "schedules" => Some(crate::standards::v1::subsets::any::io::text::inferences::schedules::table_json(&inferred.schedules)),
        "phase-visibility" => Some(crate::standards::v1::subsets::any::schema::inferences::phase_visibility::table_json(&inferred.phase_visibility)),
        "zones" => Some(crate::standards::v1::subsets::any::schema::inferences::zones::table_json(&inferred.zone_totals, &inferred.scheme_totals)),
        "energy-envelope" => Some(crate::standards::v1::subsets::any::schema::inferences::energy_envelope::table_json(&inferred.energy_envelopes, &inferred.energy_totals)),
        "effective-properties" => Some(crate::standards::v1::subsets::any::schema::inferences::effective_properties::table_json(&inferred.effective_properties)),
        "wall-solids" => {
            let rows: Vec<String> = snapshot.walls.iter().filter(|(_, wall)| matches!(wall.axis, crate::standards::v1::subsets::any::schema::snapshot::Axis::Line { .. })).filter_map(|(id, _)| inferred.wall_layout.get(id).map(|layout| format!("{}:{{\"base_z\":{},\"top_z\":{},\"volume\":{}}}", semio_framework_pack_json::to_json_string(id), layout.base_z, layout.top_z, layout.volume))).collect();
            Some(format!("{{{}}}", rows.join(",")))
        }
        _ => None,
    }
}

/// 📏️ The measures the rules read, `{ stairs, frames, ramps, zones, rooms }`: the committed input of the rules oracle, written by the blessing test from the inferred runs, door frames, ramp runs, zone totals and rooms
/// (the outlines of the rooms are what the oracle measures with shapely).
pub fn measures_json(snapshot: &ModelSnapshot, inferred: &crate::standards::v1::subsets::any::schema::inferences::ModelInference) -> String {
    let by_id = |rows: Vec<(String, DslValue)>| DslValue::object(rows);
    let stairs = by_id(
        inferred
            .stair_runs
            .iter()
            .map(|(id, run)| (id.clone(), DslValue::object([("riser_height".to_string(), json_number(run.riser_height)), ("tread".to_string(), json_number(run.tread)), ("width".to_string(), json_number(run.width)), ("riser_count".to_string(), DslValue::int(i64::from(run.riser_count))), ("tread_count".to_string(), DslValue::int(i64::from(run.tread_count)))])))
            .collect(),
    );
    let frames = by_id(snapshot.openings.iter().filter(|(_, opening)| matches!(opening.kind, OpeningKind::Door { .. })).filter_map(|(id, _)| inferred.opening_frames.get(id).map(|frame| (id.clone(), json_number(frame.width)))).collect());
    let ramps = by_id(inferred.ramp_runs.iter().map(|(id, run)| (id.clone(), DslValue::object([("slope".to_string(), json_number(run.slope)), ("length".to_string(), json_number(run.length))]))).collect());
    let zones = by_id(inferred.zone_totals.iter().map(|(id, zone)| (id.clone(), DslValue::object([("net_area".to_string(), json_number(zone.net_area)), ("resolved".to_string(), DslValue::int(i64::from(zone.resolved)))]))).collect());
    let loop_value = |vertices: &[crate::standards::v1::subsets::any::schema::snapshot::Vertex]| DslValue::Array(vertices.iter().map(|vertex| semio_framework_value::ToValue::to_value(vertex)).collect());
    let rooms = by_id(
        inferred
            .spaces
            .iter()
            .map(|(id, room)| {
                let row = DslValue::object([
                    ("status".to_string(), DslValue::String(format!("{:?}", room.status))),
                    ("clear_height".to_string(), json_number(room.clear_height)),
                    ("outline".to_string(), loop_value(&room.outline)),
                    ("holes".to_string(), DslValue::Array(room.holes.iter().map(|hole| loop_value(hole)).collect())),
                ]);
                (id.clone(), row)
            })
            .collect(),
    );
    semio_framework_pack_json::to_json_string(&DslValue::object([("stairs".to_string(), stairs), ("frames".to_string(), frames), ("ramps".to_string(), ramps), ("zones".to_string(), zones), ("rooms".to_string(), rooms)]))
}
