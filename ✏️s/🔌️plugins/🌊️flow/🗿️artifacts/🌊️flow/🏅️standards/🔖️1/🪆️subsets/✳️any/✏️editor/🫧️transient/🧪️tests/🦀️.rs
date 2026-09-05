//! 🫧️ Neutral lifecycle trace for the actual Flow empty-transient owner adapter.

use semio_framework_plugin::{ArtifactApp, EditorApp, PluginCloseStep};
use std::sync::Arc;

#[semio_framework_async_macros::async_test]
async fn flow_empty_transient_close_matches_neutral_trace_and_exact_owner() {
    type App = EditorApp<crate::editor::flow::FlowPlayApp>;
    type Store = store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🫧️transient-owners/🔣️.json")).unwrap();
    let mut owner = Store::default();
    let original_root = Arc::downgrade(&owner.current_root());
    let mut disposer = App::build_transient_store_disposer().unwrap();
    for row in fixture["trace"].as_array().unwrap() {
        let items = row["items"].as_u64().unwrap() as usize;
        let bytes = row["bytes"].as_u64().unwrap() as usize;
        let status = match disposer.close_step(&mut owner, items, bytes).unwrap() {
            PluginCloseStep::Pending { released_items, released_bytes } => { assert!(released_items <= items && released_bytes <= bytes); "pending" }
            PluginCloseStep::Complete => "complete",
            PluginCloseStep::Blocked { .. } => panic!("empty transient closure cannot block"),
        };
        assert_eq!(status, row["status"].as_str().unwrap());
        assert_eq!(original_root.upgrade().is_none(), row["retired"].as_bool().unwrap());
        assert_eq!(disposer.terminal_is_empty(&owner), row["retired"].as_bool().unwrap());
    }
    let mut foreign = Store::default();
    assert_eq!(disposer.close_step(&mut foreign, 1, 64).is_err(), fixture["rejectForeignOwner"].as_bool().unwrap());
    assert!(!disposer.terminal_is_empty(&foreign));
    assert!(disposer.terminal_is_empty(&owner));
    let mut foreign_disposer = App::build_transient_store_disposer().unwrap();
    foreign_disposer.close_step(&mut foreign, 1, 1).unwrap();
    assert_eq!(foreign_disposer.close_step(&mut foreign, 0, 0).unwrap(), PluginCloseStep::Complete);
    assert!(foreign_disposer.terminal_is_empty(&foreign));
    let terminal_generation = owner.generation_now();
    owner.reset(semio_framework_plugin::NoTransient::default()).await;
    assert_eq!(owner.generation_now(), terminal_generation.wrapping_add(1));
    assert_eq!(disposer.close_step(&mut owner, 1, 64).is_err(), fixture["rejectResetOwner"].as_bool().unwrap());
    assert!(!disposer.terminal_is_empty(&owner));
    let mut reset_disposer = App::build_transient_store_disposer().unwrap();
    reset_disposer.close_step(&mut owner, 1, 1).unwrap();
    assert_eq!(reset_disposer.close_step(&mut owner, 0, 0).unwrap(), PluginCloseStep::Complete);
    assert!(reset_disposer.terminal_is_empty(&owner));
    eprintln!("[DEBUG] Flow empty transient matched four neutral steps and rejected foreign and reset terminal ownership");
}
