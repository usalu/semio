use crate::En1990Family;
use semio_s_artifact_norm_en1990_inferences::evaluate;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub(super) fn documents() -> Vec<En1990Snapshot> {
    { use semio_s_artifact_norm_en1990_examples as e; vec![En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), e::high_consequence_office::reference_snapshot(), e::road_bridge_compliant::reference_snapshot(), e::road_bridge_failing::reference_snapshot(), e::accidental_seismic_compliant::reference_snapshot(), e::accidental_seismic_failing::reference_snapshot(), e::fatigue_compliant::reference_snapshot(), e::fatigue_failing::reference_snapshot()] }
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1990Family, full = evaluate, live = semio_s_artifact_norm_en1990_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <En1990Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("permanents.0.qk") && global.covers("effects.0.memberId") && global.covers("members.0.id") && global.covers("cases.0.id"));
    assert!(!global.overlaps("members.0.length") && !global.overlaps("cases.0.entryId") && !global.overlaps("placements.0.x") && !global.overlaps("reliabilityAnalyses.0.beta"), "a drag of a member, case or placement field leaves the global unit clean");
}
