use super::*;

/// ⚖️ LAW: a rotate gesture is refused as a whole — a zero axis is no motion the gumball tool yields, and a selection with
/// a missing shape splices nothing, so the document is untouched.
#[test]
fn rotate_selection_refuses_invalid_motions_and_targets_atomically() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let snapshot = semio_s_artifact_procedural_3d_io_text::snapshot::example_snapshot(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let count = snapshot.host_snapshot.widgets.len();
    let still = RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 0.0, angle: 1.0, phase: None, reason: None, window_id: None };
    assert!(!still.motion().moves(), "a zero axis is no motion");
    assert!(semio_s_artifact_procedural_3d_transforms::gumball_splice(&snapshot.host_snapshot, &["extrude".into(), "missing".into()], "rotate").is_err());
    assert_eq!(snapshot.host_snapshot.widgets.len(), count);
    snapshot.retire_cold();
}
