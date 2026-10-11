//! 🫧️ Neutral lifecycle trace for the actual Flow empty-transient owner adapter.

use semio_framework_plugin::{ArtifactApp, ArtifactOwnedDisposer, EditorApp, PluginLifecycleStep};
use semio_framework_value::RetainedCloneGrant;
use std::sync::Arc;

type Store = store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>;

fn quoted_grant(disposer: &dyn ArtifactOwnedDisposer<Store>, owner: &Store, items: usize) -> RetainedCloneGrant {
    let demand = disposer.retirement_demands(owner, 64).expect("empty transient closure quotes its next turn");
    RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes.max(64), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

#[semio_framework_async_macros::async_test]
async fn flow_empty_transient_close_matches_neutral_trace_and_exact_owner() {
    type App = EditorApp<crate::editor::flow::FlowPlayApp>;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🫧️transient-owners/🔣️.json")).unwrap();
    let mut owner = Store::default();
    let original_root = Arc::downgrade(&owner.current_root());
    let mut disposer = App::build_transient_store_disposer().unwrap();
    for row in fixture["trace"].as_array().unwrap() {
        let items = row["items"].as_u64().unwrap() as usize;
        let grant = quoted_grant(disposer.as_ref(), &owner, items);
        let status = match disposer.close_step(&mut owner, grant).unwrap() {
            PluginLifecycleStep::Progress(progress) => {
                assert!(progress.fits(grant));
                "pending"
            }
            PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant));
                "complete"
            }
            PluginLifecycleStep::Blocked { .. } => panic!("empty transient closure cannot block"),
            PluginLifecycleStep::AwaitingInput { reason } => panic!("fixture has no active worker input to await: {reason}"),
        };
        assert_eq!(status, row["status"].as_str().unwrap());
        assert_eq!(original_root.upgrade().is_none(), row["rootRetired"].as_bool().unwrap());
        assert_eq!(disposer.terminal_is_empty(&owner), row["terminalEmpty"].as_bool().unwrap());
    }
    let mut foreign = Store::default();
    let foreign_grant = quoted_grant(disposer.as_ref(), &owner, 1);
    assert_eq!(disposer.close_step(&mut foreign, foreign_grant).is_err(), fixture["rejectForeignOwner"].as_bool().unwrap());
    assert!(!disposer.terminal_is_empty(&foreign));
    assert!(disposer.terminal_is_empty(&owner));
    let mut foreign_disposer = App::build_transient_store_disposer().unwrap();
    for _ in 0..11 {
        let grant = quoted_grant(foreign_disposer.as_ref(), &foreign, 1);
        if matches!(foreign_disposer.close_step(&mut foreign, grant).unwrap(), PluginLifecycleStep::Complete(_)) { break; }
    }
    assert!(foreign_disposer.terminal_is_empty(&foreign));
    let terminal_generation = owner.generation_now();
    owner.reset(semio_framework_plugin::NoTransient::default()).await;
    assert_eq!(owner.generation_now(), terminal_generation.wrapping_add(1));
    let reset_grant = quoted_grant(disposer.as_ref(), &owner, 1);
    assert_eq!(disposer.close_step(&mut owner, reset_grant).is_err(), fixture["rejectResetOwner"].as_bool().unwrap());
    assert!(!disposer.terminal_is_empty(&owner));
    let mut reset_disposer = App::build_transient_store_disposer().unwrap();
    for _ in 0..11 {
        let grant = quoted_grant(reset_disposer.as_ref(), &owner, 1);
        if matches!(reset_disposer.close_step(&mut owner, grant).unwrap(), PluginLifecycleStep::Complete(_)) { break; }
    }
    assert!(reset_disposer.terminal_is_empty(&owner));
}
