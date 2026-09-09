use super::*;
use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

#[test]
fn admitted_document_mutations_make_bounded_progress_and_retire() {
    let factory = Fem2dArtifactPreparationFactory;
    let mutation = Fem2dMutation::CreateNode(crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: crate::FemNode { id: "n1".into(), x: 0.0, y: 0.0 } });
    assert_eq!(factory.preflight(&mutation, None, store::HistoryLane::Document).expect("document admission").work_items, 1);
    assert!(factory.preflight(&mutation, Some(&"x".repeat(store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES + 1)), store::HistoryLane::Document).is_err());
    let mut work = Fem2dArtifactPreparation { base: None, mutation: Some(mutation), description: None, authority: None, prepared: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false };
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 1_048_576 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.cancel();
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1_048_576 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.begin_close();
    assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 1_048_576 }), Ok(store::SnapshotRetirementStep::Blocked)));
    assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1_048_576 }), Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })));
    assert!(work.terminal_is_empty());
    assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1_048_576 }), Ok(store::SnapshotRetirementStep::Complete)));
}
