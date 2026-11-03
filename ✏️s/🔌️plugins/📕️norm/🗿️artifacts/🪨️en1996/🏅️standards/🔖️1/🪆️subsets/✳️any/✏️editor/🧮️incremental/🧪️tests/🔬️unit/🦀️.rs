use crate::En1996Family;
use semio_s_artifact_norm_en1996_inferences::evaluate;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub(super) fn documents() -> Vec<En1996Snapshot> {
    vec![En1996Snapshot::compliant_clay_wall(), En1996Snapshot::noncompliant_multi_fail(), En1996Snapshot::opening_wall_example(), En1996Snapshot::basement_wall_example(), En1996Snapshot::concentrated_load_example(), En1996Snapshot::reinforced_wall_example(), En1996Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1996Family, full = evaluate, live = semio_s_artifact_norm_en1996_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <En1996Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let (global, detailing) = (reads("global:document"), reads("part:detailing"));
    assert!(global.covers("annex") && global.covers("walls.0.height") && !global.overlaps("cases.0.entryId") && !global.overlaps("chases.0.depth") && !global.overlaps("movementJoints"), "the global unit follows the walls only");
    assert!(detailing.covers("chases.0.depth") && detailing.covers("movementJoints.0.width") && !detailing.overlaps("cases.0.entryId"), "the detailing verdict follows walls, chases and joints");
}
