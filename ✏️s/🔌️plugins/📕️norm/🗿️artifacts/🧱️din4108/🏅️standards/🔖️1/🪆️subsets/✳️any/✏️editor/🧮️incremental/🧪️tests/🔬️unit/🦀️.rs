use crate::Din4108Family;
use semio_s_artifact_norm_din4108_inferences::evaluate;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub(super) fn documents() -> Vec<Din4108Snapshot> {
    vec![Din4108Snapshot::default(), Din4108Snapshot::failing_thin_insulation(), Din4108Snapshot { zones: vec![], elements: vec![], thermal_bridges: vec![], ..Din4108Snapshot::default() }, Din4108Snapshot::default()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = Din4108Family, full = evaluate, live = semio_s_artifact_norm_din4108_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <Din4108Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("zones.0.id") && global.covers("elements.0.zoneId") && global.covers("thermalBridges.0.id"));
    assert!(!global.overlaps("airtightnessN50") && !global.overlaps("tIntC"), "a scalar edit leaves the integrity unit clean");
    let airtightness = reads("part:airtightness");
    assert!(airtightness.covers("airtightnessN50") && airtightness.covers("hasMechanicalVentilation"));
    assert!(!airtightness.overlaps("elements.0.layers.0.thicknessM"), "a layer edit leaves the airtightness verdict clean");
}
