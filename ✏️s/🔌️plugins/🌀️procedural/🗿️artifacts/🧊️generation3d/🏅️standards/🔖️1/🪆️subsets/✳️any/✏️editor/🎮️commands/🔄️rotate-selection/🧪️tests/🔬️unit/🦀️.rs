use super::*;

#[test]
fn rotate_selection_reports_invalid_parameters_and_targets_atomically() {
    let _serial = crate::test_serial::lock();
    let snapshot = crate::standards::v1::subsets::any::schema::example_snapshot(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let count = snapshot.host_snapshot.widgets.len();
    assert!(rotate_ids(&snapshot.host_snapshot, &["extrude".into()], 0.0, 0.0, 0.0, 1.0).is_err());
    assert!(rotate_ids(&snapshot.host_snapshot, &["extrude".into(), "missing".into()], 0.0, 0.0, 1.0, 1.0).is_err());
    assert_eq!(snapshot.host_snapshot.widgets.len(), count);
    snapshot.retire_cold();
}
