use super::*;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::retirement::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

#[test]
fn admitted_document_mutations_make_bounded_progress_and_retire() {
    let factory = Fem2dArtifactPreparationFactory;
    let id = String::from("original n1 雪 λ🙂");
    let pointer = id.as_ptr();
    let capacity = id.capacity();
    let mutation = Fem2dMutation::CreateNode(crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode { node: crate::FemNode { id, x: 0.0, y: 0.0 }, index: None });
    assert_eq!(factory.preflight(&mutation, store::HistoryLane::Document).expect("document admission").work_items, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
    let owners = store::OneItemOwners::detached(mutation, std::sync::Arc::new(OwnedValueRetirementFactory::<Fem2dMutation>::default()), std::sync::Arc::new(SharedValueRetirementFactory::<Fem2dSnapshot>::default()));
    let mut work = Fem2dArtifactPreparation { owners, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false };
    let policy = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 1048576, maximum_release_bytes: 1048576, maximum_depth: 64 };
    assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, ..policy }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.cancel();
    assert!(matches!(work.advance(policy), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
    work.begin_close();
    for grant in [store::ArtifactStoreOneItemGrant { maximum_items: 0, ..policy }, store::ArtifactStoreOneItemGrant { maximum_capacity_bytes: 0, ..policy }, store::ArtifactStoreOneItemGrant { maximum_depth: 0, ..policy }] {
        let result = work.close_step(grant);
        assert!(result.is_err() || result.as_ref().is_ok_and(|step| step.progress() == Default::default()));
        let Some(Fem2dMutation::CreateNode(value)) = work.owners.mutation.as_ref() else { panic!("denial moved the original mutation") };
        assert_eq!(value.node.id.as_ptr(), pointer);
        assert_eq!(value.node.id.capacity(), capacity);
        assert_eq!(value.node.id, "original n1 雪 λ🙂");
        assert!(!work.terminal_is_empty());
    }
    let mut allocated = 0;
    let mut released = 0;
    for _ in 0..4096 {
        let step = work.close_step(policy).expect("original immutable policy funds each bounded child");
        assert!(step.progress().fits(policy.retained_grant()));
        allocated += step.progress().retained_capacity_bytes;
        released += step.progress().released_bytes;
        if work.terminal_is_empty() {
            break;
        }
    }
    assert!(work.terminal_is_empty());
    assert_eq!(released, allocated + capacity);
    assert!(matches!(work.close_step(policy), Ok(RetainedCloneStep::Complete(progress)) if progress == Default::default()));
    println!("[DEBUG] FEM original canceled mutation pointer retained through denied axes; fixed policy closed child frames {} and original String capacity {} with exact release {}", allocated, capacity, released);
}
