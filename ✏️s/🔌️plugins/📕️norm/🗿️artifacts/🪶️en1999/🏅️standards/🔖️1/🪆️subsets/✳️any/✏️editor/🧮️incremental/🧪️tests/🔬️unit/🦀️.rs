use crate::En1999Family;
use semio_s_artifact_norm_en1999_inferences::evaluate;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub(super) fn documents() -> Vec<En1999Snapshot> {
    vec![En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail(), En1999Snapshot::empty(), En1999Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1999Family, full = evaluate, live = semio_s_artifact_norm_en1999_inferences::live::report, documents = documents());

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <En1999Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("sections.0.id") && global.covers("members.0.sectionId") && global.covers("shells.0.id"));
    assert!(!global.overlaps("members.0.length") && !global.overlaps("connections.0.boltCount") && !global.overlaps("cases.0.entryId"), "a member or connection field edit leaves the integrity unit clean");
    for (part, ids) in [("part:members-empty", "members.0.id"), ("part:fire-empty", "fireScenarios.0.id"), ("part:shells-empty", "shells.0.id")] {
        let own = reads(part);
        assert!(own.covers("annex") && own.covers(ids), "{part} reads the annex and the emptiness of its collection");
        assert!(!own.overlaps(&ids.replace(".id", ".length")), "{part} ignores the fields of its rows");
    }
}
