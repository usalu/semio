//! 🧪️ Real Flow document identity and lifecycle laws.

use crate::artifacts::flow::{FlowSnapshot, FlowWorkingScene};
use crate::artifacts::flow::retirement::{SnapshotRetirementFactory, SceneRetirementFactory};
use std::sync::Arc;

//#region 🧪️LocalOwnerHandoff
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn flow_store_owners_retire_all_durable_lanes_with_neutral_byte_grants() {
        use semio_framework_plugin::{ArtifactApp, EditorApp, PluginCloseStep};
        type App = EditorApp<crate::editor::flow::FlowPlayApp>;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🏪️store-owners/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            for grant in fixture["grants"].as_array().unwrap() {
                let maximum_bytes = grant.as_u64().unwrap() as usize;
                let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
                let payload_bytes = row["payloadBytes"].as_u64().unwrap() as usize;
                assert_eq!(text.len(), payload_bytes);
                macro_rules! close_lane {
                    ($value:expr, $owners:expr, $disposer:expr) => {{
                        let envelope = store::create_document_envelope("flow.owner-law", row["id"].as_str().unwrap(), $value, None);
                        let mut owner = store::ArtifactStore::new(envelope).await.unwrap();
                        owner.install_member_store_owners_exact($owners.expect("Flow lane must declare exact owners"));
                        let mut disposer = $disposer.expect("Flow lane must declare a bounded close adapter");
                        let mut released = 0;
                        let mut completed = false;
                        for _ in 0..100_000 {
                            match disposer.close_step(&mut owner, 1, maximum_bytes).unwrap() {
                                PluginCloseStep::Pending { released_items, released_bytes } => {
                                    assert!(released_items <= 1 && released_bytes <= maximum_bytes);
                                    released += released_bytes;
                                }
                                PluginCloseStep::Complete => { completed = true; break; }
                                PluginCloseStep::Blocked { .. } => panic!("unshared Flow store must close without a retained reader"),
                            }
                        }
                        assert!(completed, "Flow durable store must report completion within the fixed bound");
                        assert_eq!(disposer.terminal_is_empty(&owner), row["terminalEmpty"].as_bool().unwrap());
                        assert!(released >= payload_bytes);
                    }};
                }
                match row["lane"].as_str().unwrap() {
                    "document" => {
                        let content = crate::artifacts::flow::flow_content_child_handle_and_cache(vec![flow::Widget::InputNote { id: "note".into(), text }], vec![], Default::default());
                        close_lane!(FlowSnapshot { schema: crate::artifacts::flow::FLOW_DOCUMENT_SCHEMA.into(), camera: Default::default(), content }, App::build_document_store_owners(), App::build_document_store_disposer());
                    }
                    "config" => close_lane!(crate::editor::flow::config::FlowConfig { preview_off_node_ids: vec![text], ..Default::default() }, App::build_config_store_owners(), App::build_config_store_disposer()),
                    "draft" => {
                        assert_eq!(std::mem::size_of::<semio_framework_plugin::NoDraft>(), 0);
                        assert_eq!(std::mem::size_of::<semio_framework_plugin::NoDraftMutation>(), 0);
                        close_lane!(semio_framework_plugin::NoDraft::default(), App::build_draft_store_owners(), App::build_draft_store_disposer());
                    }
                    _ => panic!("unknown durable Flow lane"),
                }
                eprintln!("[DEBUG] Flow durable lane={} grant={maximum_bytes} payloadBytes={payload_bytes} terminal=true", row["lane"]);
            }
        }
    }

    #[test]
    fn flow_parent_projection_and_child_identity_match_neutral_corpus() {
        use semio_framework_plugin::{ArtifactApp, EditorApp, ViewerApp};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🪪️content-identity/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let (widgets, synapses, layout) = crate::artifacts::flow::schema::mutations::decode_flow_scene_json(row["canonicalJson"].as_str().unwrap()).unwrap();
            let content = crate::artifacts::flow::flow_content_child_handle(&widgets, &synapses, &layout);
            let snapshot = Arc::new(FlowSnapshot { schema: crate::artifacts::flow::FLOW_DOCUMENT_SCHEMA.into(), camera: Default::default(), content });
            let expected_id = format!("{}{}", fixture["childIdPrefix"].as_str().unwrap(), row["expectedSha256"].as_str().unwrap());
            assert_eq!(snapshot.content.child_id, expected_id);
            assert_eq!(snapshot.content.target.artifact_id, expected_id);
            for projection in [
                <EditorApp<crate::editor::flow::FlowPlayApp> as ArtifactApp>::child_restore_projection(&snapshot).unwrap(),
                <ViewerApp<crate::viewer::flow::FlowViewer> as ArtifactApp>::child_restore_projection(&snapshot).unwrap(),
            ] {
                assert!(projection.admits_member("content", &snapshot.content.target));
                assert!(!projection.admits_member("other", &snapshot.content.target));
                for field in 0..4 {
                    let mut substituted = snapshot.content.target.clone();
                    match field {
                        0 => substituted.artifact_id.push_str("-other"),
                        1 => substituted.dialect.artifact_kind = "s.other.child".into(),
                        2 => substituted.dialect.standard.push_str("-other"),
                        _ => substituted.dialect.subset.push_str("-other"),
                    }
                    assert!(!projection.admits_member("content", &substituted));
                }
            }
            let mut retirement = store::SnapshotRetirementFactory::retire(&SnapshotRetirementFactory, snapshot);
            for _ in 0..100_000 { if retirement.close_step(1, 64).unwrap() == store::SnapshotRetirementStep::Complete { break; } }
            assert!(retirement.terminal_is_empty());
        }
        eprintln!("[DEBUG] real Flow editor/viewer parent projections matched five neutral identities and rejected each substituted coordinate");
    }

    #[test]
    fn child_typed_handoff_preserves_mismatched_owner_then_retires_exact_scene() {
        let root = Arc::new(FlowWorkingScene { widgets: vec![flow::Widget::InputNote { id: "note".into(), text: "🌊".repeat(4096) }], ..Default::default() });
        let weak = Arc::downgrade(&root);
        let mut child = crate::artifacts::flow::flow_content_child_from_digest([0; 32], root);
        assert!(child.take_local_owner::<String>().is_err()); assert!(weak.upgrade().is_some());
        let scene = child.take_local_owner::<FlowWorkingScene>().unwrap().unwrap(); assert!(child.take_local_owner::<FlowWorkingScene>().unwrap().is_none());
        let mut retirement = store::SnapshotRetirementFactory::retire(&SceneRetirementFactory, scene);
        let mut released = 0;
        loop {
            match retirement.close_step(1, 1).unwrap() {
                store::SnapshotRetirementStep::Complete => break,
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 1); released += released_bytes; }
                store::SnapshotRetirementStep::Blocked => panic!("one-byte exact scene retirement cannot block"),
            }
        }
        assert_eq!(released, 16388); assert!(retirement.terminal_is_empty()); assert!(weak.upgrade().is_none());
    }
}
//#endregion 🧪️LocalOwnerHandoff
