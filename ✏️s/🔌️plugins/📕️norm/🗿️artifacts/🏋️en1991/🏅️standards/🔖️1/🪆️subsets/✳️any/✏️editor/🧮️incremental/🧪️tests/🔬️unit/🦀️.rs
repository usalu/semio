use crate::En1991Family;
use semio_s_artifact_norm_en1991_inferences::evaluate;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub(super) fn documents() -> Vec<En1991Snapshot> {
    { use semio_s_artifact_norm_en1991_examples::example_subjects as e; vec![En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), e::de_office_compliant(), e::de_bridge_compliant(), e::de_bridge_noncompliant(), e::de_fire_parametric_compliant(), e::de_fire_parametric_noncompliant(), e::de_accidental_variants(), e::de_hazard_collections(), e::multi_fail_noncompliant()] }
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1991Family, full = evaluate, live = semio_s_artifact_norm_en1991_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <En1991Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("floors.0.id") && global.covers("bridgeLanes.0.width") && global.covers("roofSurfaces.0.extent") && global.covers("depth"));
    assert!(!global.overlaps("floors.0.area") && !global.overlaps("snowGuards.0.height") && !global.overlaps("cases.0.entryId"), "a floor or load edit leaves the integrity unit clean");
    for (part, ids) in [("part:floors-none", "floors.0.id"), ("part:snow", "roofs.0.id"), ("part:wind", "windFaces.0.id"), ("part:accidental-none", "accidentalCases.0.id")] {
        let own = reads(part);
        assert!(own.covers(ids) && own.covers("annex"), "{part} reads the emptiness of its collection and the shared scalars");
        assert!(!own.overlaps(&ids.replace(".id", ".area")), "{part} ignores the fields of its rows");
    }
    for part in ["part:fire", "part:thermal", "part:construction"] {
        assert!(!reads(part).overlaps("floors.0.area") && !reads(part).overlaps("roofs"), "{part} reads shared scalars only");
    }
}
