//! ⚖️ Real-store authoring: a shooting document edit authors, applies, undoes and redoes through the shooting editor's own catalog and the runtime's canonical operation wire.
use semio_framework_plugin::{authoring_law::author_through_document_store, EditorApp};
use semio_framework_value::ToValue;
use semio_s_artifact_shooting_shooting::editor::shooting::ShootingPlayApp;
use semio_s_artifact_shooting_shooting::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot;
use semio_s_artifact_shooting_shooting::{ShootingMutation, ShootingShot};

fn assert_row_is_the_serde_document(row: &[u8], mutation: &ShootingMutation) {
    assert!(row.starts_with(b"op"), "the canonical wire carries its operation header");
    let wire: serde_json::Value = serde_json::from_slice(&row[2..]).expect("the wire body is JSON an independent parser accepts");
    assert_eq!(wire, serde_json::to_value(mutation.to_value()).expect("independent serde_json document"));
}

/// 📄️ Creating a shot publishes one forward row equal to the independent serde_json document, and its inverse removes the shot again.
#[semio_framework_async_macros::async_test]
async fn a_shooting_document_edit_authors_applies_undoes_and_redoes_through_its_real_store() {
    let shot = ShootingShot { id: "shot-1".into(), label: "Hero".into(), width: 1280, height: 720, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None };
    let mutation = ShootingMutation::CreateShot(CreateShot { shot: shot.clone(), index: None });
    let law = author_through_document_store::<EditorApp<ShootingPlayApp>>(mutation.clone()).await;
    assert_eq!(law.applied.shots.last(), Some(&shot));
    assert_eq!(law.applied.shots.len(), law.undone.shots.len() + 1, "undo removes exactly the created shot");
    assert_eq!(law.applied.shots[..law.undone.shots.len()], law.undone.shots[..]);
    assert_eq!(law.redone.shots, law.applied.shots, "redo restores the created shot");
    assert_eq!(law.forward_rows.len(), 1);
    assert_row_is_the_serde_document(&law.forward_rows[0], &mutation);
    assert_eq!(law.inverse_rows.len(), 1, "creating one shot is undone by exactly one row");
    assert!(law.inverse_rows[0].starts_with(b"op"));
}
