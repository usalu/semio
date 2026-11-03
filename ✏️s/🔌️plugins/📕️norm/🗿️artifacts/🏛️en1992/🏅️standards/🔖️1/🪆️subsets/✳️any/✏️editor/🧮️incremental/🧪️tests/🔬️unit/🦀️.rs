use crate::En1992Family;
use semio_s_artifact_norm_en1992_inferences::evaluate;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub(super) fn documents() -> Vec<En1992Snapshot> {
    vec![En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced(), En1992Snapshot::failing_prestressed_beam(), En1992Snapshot::compliant_prestressed_beam(), En1992Snapshot::liquid_retaining_fem_anchor(), En1992Snapshot::flat_slab_punching(), En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1992Family, full = evaluate, live = semio_s_artifact_norm_en1992_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <En1992Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("members.0.id") && global.covers("concreteGrades.0.fCk") && global.covers("prestressSteels.0.fPk"));
    assert!(!global.overlaps("members.0.fire") && !global.overlaps("anchors.0.depth") && !global.overlaps("cases.0.entryId"), "a member or anchor field edit leaves the global unit clean");
}
