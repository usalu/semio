use crate::En1998Family;
use semio_s_artifact_norm_en1998_inferences::evaluate;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub(super) fn documents() -> Vec<En1998Snapshot> {
    vec![En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1998Snapshot::compliant_de_office(), En1998Snapshot::noncompliant_de_office(), En1998Snapshot::compliant_de_multipart(), En1998Snapshot::noncompliant_de_multipart(), En1998Snapshot::compliant_en_office(), En1998Snapshot::noncompliant_en_office(), En1998Snapshot::compliant_de_torsion_regular(), En1998Snapshot::noncompliant_de_torsion_irregular()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1998Family, full = evaluate, live = semio_s_artifact_norm_en1998_inferences::live::report, documents = documents());

#[test]
fn design_cases_can_be_edited_and_removed_through_the_project_commands() {
    use semio_s_artifact_norm_en1998_inferences::case_catalogue::En1998Cases;
    semio_s_plugin_norm_app_surface::assert_case_edit_paths::<En1998Family, En1998Cases>(
        En1998Snapshot::default(),
        |document| document.cases.clone(),
        semio_s_artifact_norm_en1998_mutations::resolve_edit,
    );
}

#[test]
fn archived_design_cases_reach_the_seismic_report() {
    let cases = serde_json::from_str(include_str!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/📋️design-cases.json")).unwrap();
    let document = En1998Snapshot { cases, ..En1998Snapshot::default() };
    let full = evaluate(&document);
    let live = semio_s_artifact_norm_en1998_inferences::live::report(&document);
    for key in [
        "en1998.c98-11.bond.buried-pipe",
        "en1998.c98-12.set-a.bridge-spatial",
        "en1998.c98-19.global-buckling.pipe-differential",
        "en1998.c98-22.backfill-a.pipe-fault-crossing",
        "en1998.c98-22.trench-b.pipe-fault-crossing",
        "en1998.c98-23.increment.granular-silo",
    ] {
        assert!(full.checks.iter().any(|check| check.id == key), "{key}");
        assert!(live.checks.iter().any(|check| check.id == key), "{key}");
    }
}
