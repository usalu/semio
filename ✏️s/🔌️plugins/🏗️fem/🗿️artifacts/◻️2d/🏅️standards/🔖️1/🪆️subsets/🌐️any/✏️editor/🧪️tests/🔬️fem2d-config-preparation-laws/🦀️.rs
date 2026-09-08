
use super::*;
use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

#[test]
fn admitted_maximum_and_production_grant_make_bounded_progress() {
    let factory = Fem2dConfigPreparationFactory;
    let maximum = Fem2dConfigMutation::SetLocale { value: "x".repeat(FEM2D_CONFIG_TEXT_MAXIMUM_BYTES) };
    assert_eq!(factory.preflight(&maximum, None, store::HistoryLane::Document).expect("maximum admission").retained_bytes, 4_096);
    let overflow = Fem2dConfigMutation::SetLocale { value: "x".repeat(FEM2D_CONFIG_TEXT_MAXIMUM_BYTES + 1) };
    assert!(factory.preflight(&overflow, None, store::HistoryLane::Document).is_err());
    assert!(factory.preflight(&maximum, Some(&"x".repeat(65)), store::HistoryLane::Document).is_err());
    let mut work = Fem2dConfigPreparation { base: None, mutation: Some(maximum), description: None, authority: None, prepared: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false };
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4_096 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_095 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.cancel();
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.begin_close();
    assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }), Ok(store::SnapshotRetirementStep::Blocked)));
    assert!(work.mutation.is_some());
    assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 })));
    assert!(work.terminal_is_empty());
    assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::SnapshotRetirementStep::Complete)));
}
