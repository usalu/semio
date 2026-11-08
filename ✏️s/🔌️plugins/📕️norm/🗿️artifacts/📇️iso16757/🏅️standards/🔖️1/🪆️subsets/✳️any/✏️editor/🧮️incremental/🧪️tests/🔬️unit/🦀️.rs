use crate::Iso16757Family;
use semio_s_artifact_norm_iso16757_inferences::evaluate;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

pub(super) fn documents() -> Vec<Iso16757Snapshot> {
    vec![Iso16757Snapshot::reference_fixture(), Iso16757Snapshot::broken_fixture(), Iso16757Snapshot::default()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = Iso16757Family, full = evaluate, live = semio_s_artifact_norm_iso16757_inferences::live::report, documents = documents());

#[test]
fn a_geometry_edit_leaves_the_dictionary_and_exchange_only_sections_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <Iso16757Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0.ends_with(id)).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    assert!(reads("global:document").covers("catalogue.metadata") && !reads("global:document").overlaps("geometry.objects.g.spaces.0.bounds.max.0"));
    assert!(reads("referential-integrity").covers("geometry.objects.g.spaces.0.bounds.max.0"));
    assert!(!reads("dictionary-structure").overlaps("geometry.objects.g.spaces.0.bounds.max.0") && !reads("edition-profile").overlaps("geometry.objects.g.spaces.0.bounds.max.0"));
}
