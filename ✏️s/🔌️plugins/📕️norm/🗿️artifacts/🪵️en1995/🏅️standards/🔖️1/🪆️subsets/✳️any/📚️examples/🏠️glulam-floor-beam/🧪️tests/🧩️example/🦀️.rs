use semio_s_artifact_norm_en1995_schema::checks::timber::evaluate_structure;
use semio_s_artifact_norm_en1995_io::text::snapshot::decode_en1995_dsl;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;

fn decoded() -> En1995Snapshot {
    decode_en1995_dsl(crate::glulam_floor_beam::PRIMARY_TEXT).expect("glulam-floor-beam DSL decodes")
}

#[test]
fn primary_asset_decodes_to_the_default_document() {
    assert_eq!(decoded(), En1995Snapshot::default());
}

#[test]
fn decoded_example_complies_including_connection_and_floor_vibration() {
    let snap = decoded();
    let report = evaluate_structure(snap.annex, &snap.members, &snap.connections);
    assert!(report.complies(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    for id in ["en1995.6.1.6.bending.beam-B1", "en1995.7.3.f1.beam-B1", "en1995.7.3.stiffness.beam-B1", "en1995.7.3.velocity.beam-B1", "en1995.8.spacing.a1.conn-C1"] {
        assert!(report.checks.iter().any(|c| c.id == id), "missing {id}");
    }
}
