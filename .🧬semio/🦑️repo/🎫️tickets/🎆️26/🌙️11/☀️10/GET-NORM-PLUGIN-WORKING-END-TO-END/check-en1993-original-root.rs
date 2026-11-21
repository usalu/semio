//! 🏗️ Retire the authored EN 1993 genesis snapshot with its funded original Arc owner.
use semio_framework_value::{AuthoringWallet, ErasedSnapshotRetirement, RetainedCloneGrant, close_owner};
use semio_framework_value_factory::{OwnedValueRetirementFactory, shared::root::ArcRootRetirement};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use std::sync::Arc;

fn main() {
    let snapshot = En1993Snapshot::compliant_heb240_frame();
    assert!(!snapshot.joints.is_empty());
    let factory = Arc::new(OwnedValueRetirementFactory::<En1993Snapshot>::default());
    let mut root = ArcRootRetirement::new(Arc::new(snapshot), factory);
    let mut wallet = AuthoringWallet::new(RetainedCloneGrant {
        maximum_items: 1 << 16, maximum_copy_bytes: 1 << 27, maximum_capacity_bytes: 1 << 26,
        maximum_release_bytes: 1 << 26, maximum_depth: 64,
    }, 1 << 23);
    let mut deepest = 0;
    let mut releases = 0;
    for turn in 0..20_000 {
        if root.terminal_is_empty() {
            assert!(releases > 0);
            println!("DISPLAY retired EN1993 authored Arc root on turn {turn}, peak depth {deepest}, released {releases} bytes");
            return;
        }
        let demand = root.next_demand(32768).unwrap();
        deepest = deepest.max(demand.depth);
        assert!(demand.depth < 48, "body quote {demand:?}");
        let grant = RetainedCloneGrant {
            maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth,
        };
        let work = root.next_computational_work(32768).unwrap();
        let (outcome, _) = wallet.run_settled(grant, work, |effect| close_owner(&mut root, 32768, effect)).unwrap().unwrap();
        let progress = outcome.result.unwrap().progress();
        assert_ne!(progress, Default::default(), "turn {turn}, body quote {demand:?}");
        releases += progress.released_bytes;
        wallet.post(progress).unwrap();
    }
    panic!("EN1993 authored root did not become terminal");
}
