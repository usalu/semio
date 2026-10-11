use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

#[test]
fn admitted_maximum_and_production_grant_make_bounded_progress() {
    let factory = Generation2dConfigPreparationFactory;
    let maximum = Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value: "x".repeat(GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES) });
    assert_eq!(factory.preflight(&maximum, store::HistoryLane::Document).expect("maximum admission").retained_bytes, 4_096);
    let overflow = Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value: "x".repeat(GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES + 1) });
    assert!(factory.preflight(&overflow, store::HistoryLane::Document).is_err());
    let mut work = Generation2dConfigPreparation {
        owners: store::OneItemOwners::detached(
            maximum,
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dConfigMutation>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<Generation2dConfig>::default()),
        ),
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        cancelled: false,
    };
    let page = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 4_096, maximum_release_bytes: 4_096, maximum_depth: 8 };
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, ..page }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_copy_bytes: 4_095, ..page }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.cancel();
    assert!(matches!(work.advance(page), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.begin_close();
    let mut turns = 0;
    while !work.terminal_is_empty() {
        let copy = work.next_close_copy_byte_demand().expect("copy demand").max(4_096);
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: work.next_close_capacity_byte_demand(copy).expect("capacity demand"),
            maximum_release_bytes: work.next_close_release_byte_demand().expect("release demand"),
            maximum_depth: work.next_close_depth_demand().expect("depth demand").max(1),
        };
        let step = work.close_step(store::ArtifactStoreOneItemGrant::from_retained(grant)).expect("one owner per fully granted turn");
        assert!(step.progress().fits(grant));
        turns += 1;
        assert!(turns < 64, "the close ladder terminates");
    }
    assert!(matches!(work.close_step(page), Ok(RetainedCloneStep::Complete(_))));
}
