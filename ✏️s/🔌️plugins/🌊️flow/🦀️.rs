//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the flow editor and viewer surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum FlowApps: PluginApp {
        FlowEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::flow::FlowPlayApp>, semio_s_plugin_stdio::artifacts::semio::SemioMembers>),
        FlowViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::flow::FlowViewer>, semio_s_plugin_stdio::artifacts::semio::SemioMembers>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M3, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"computation.flow"` artifact (`crate::artifacts::flow::artifact_kind().id`) is opened, this plugin's
/// own actor runs `Isolated` (its 9 `🧩️extensions/` run `Linked` instead — see each extension's own
/// `bundle()`), and it asks the broker for document write access to persist edits.
pub fn plugin() -> Result<Plugin<FlowApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<FlowApps>::builder("flow")
        .label("Flow")
        .version("0.1.0")
        .package_id("semio:flow")
        .artifact(crate::artifacts::flow::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor_with_members::<crate::editor::flow::FlowPlayApp, semio_s_plugin_stdio::artifacts::semio::SemioMembers>(crate::editor::flow::create_flow_app())
        .editor_mutation_roster::<crate::editor::flow::FlowPlayApp>()
        .viewer_with_members::<crate::viewer::flow::FlowViewer, semio_s_plugin_stdio::artifacts::semio::SemioMembers>(crate::viewer::flow::create_flow_viewer())
        .viewer_mutation_roster::<crate::viewer::flow::FlowViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::flow::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist flow graph edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use crate::editor::flow::FlowPlayApp;
    use crate::viewer::flow::FlowViewer;

    #[semio_framework_async_macros::async_test]
    async fn flow_actual_surface_factories_close_all_owners_under_neutral_grants() {
        use semio_framework_plugin::{AppRole, PluginApp, PluginCloseStep};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧹️surface-owners/🔣️.json")).unwrap();
        let plugin = super::plugin().expect("the actual Flow package must assemble every registered surface");
        assert_eq!(plugin.manifest.apps.len(), fixture["expected"]["factories"].as_u64().unwrap() as usize);
        let roles = plugin.manifest.apps.iter().map(|definition| match definition.role {
            AppRole::Editor => "editor", AppRole::Viewer => "viewer",
        }).collect::<Vec<_>>();
        assert_eq!(roles, fixture["roles"].as_array().unwrap().iter().map(|role| role.as_str().unwrap()).collect::<Vec<_>>());
        let items = fixture["items"].as_u64().unwrap() as usize;
        for definition in &plugin.manifest.apps {
            for grant in fixture["byteGrants"].as_array().unwrap() {
                let bytes = grant.as_u64().unwrap() as usize;
                let mut app = plugin.create_app(&definition.id).expect("every declared surface has an actual factory");
                assert!(matches!((&app, &definition.role), (super::FlowApps::FlowEditor(_), AppRole::Editor) | (super::FlowApps::FlowViewer(_), AppRole::Viewer)));
                let mut completed = false;
                for _ in 0..fixture["maximumSteps"].as_u64().unwrap() {
                    match app.close_step(items, bytes).expect("the actual Flow app owns every store and instance close stage") {
                        PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= items && released_bytes <= bytes),
                        PluginCloseStep::Blocked { reason } => panic!("fresh Flow surface retains no external reader: {reason}"),
                        PluginCloseStep::Complete => { completed = true; break; }
                    }
                }
                assert_eq!(completed, fixture["expected"]["complete"].as_bool().unwrap(), "{} bytes={bytes}", definition.id);
                assert_eq!(app.close_terminal_is_empty(), fixture["expected"]["terminalEmpty"].as_bool().unwrap());
                assert!(matches!(app.close_step(0, 0).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
                assert!(app.close_terminal_is_empty());
                assert!(matches!(app.close_step(items, bytes).unwrap(), PluginCloseStep::Complete));
                eprintln!("[DEBUG] actual Flow surface={} bytes={} closed all stores and app-instance owners", definition.id, bytes);
            }
        }
    }

    /// 👁️ A viewer instance never mutates the document store, even when dispatched.
    #[semio_framework_async_macros::async_test]
    async fn flow_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<FlowViewer>();
    }

    /// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
    #[semio_framework_async_macros::async_test]
    async fn flow_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<FlowPlayApp, FlowViewer>();
    }
}
//#endregion 🧪️SurfaceTests
