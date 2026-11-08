use crate::DinEn16798Family;
use semio_s_artifact_norm_din16798_inferences::evaluate;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub(super) fn documents() -> Vec<Din16798Snapshot> {
    vec![Din16798Snapshot::compliant_office(), Din16798Snapshot::residential_method3(), Din16798Snapshot::noncompliant_office(), Din16798Snapshot { zones: vec![], vent_systems: vec![], ..Din16798Snapshot::compliant_office() }]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = DinEn16798Family, full = evaluate, live = semio_s_artifact_norm_din16798_inferences::live::report, documents = documents());

#[test]
fn a_zone_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <DinEn16798Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("zones.0.id") && global.covers("ventSystems.0.id"));
    assert!(!global.overlaps("zones.0.co2Ppm") && !global.overlaps("ventSystems.0.sfpWM3S"), "a design value edit leaves the integrity unit clean");
    let envelope = reads("part:envelope");
    assert!(envelope.covers("envelopeN50HInv") && !envelope.overlaps("zones.0.co2Ppm"));
}
