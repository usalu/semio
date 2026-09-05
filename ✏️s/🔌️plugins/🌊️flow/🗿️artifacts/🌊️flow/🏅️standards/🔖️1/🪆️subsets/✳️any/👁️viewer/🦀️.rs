//! 👁️ Flow viewer — the read-only counterpart of the mutation-capable module for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `FlowViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<FlowViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling mutation-capable module (`policyViewerPurityBreaches`).

use crate::artifacts::flow::op::FlowMutation;
use crate::artifacts::flow::{FlowSnapshot, FLOW_DIALECT, FLOW_DOCUMENT_SCHEMA};
use crate::viewer::flow::modes::view;
use crate::viewer::flow::modes::view::windows::main;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant —
/// real per-command payload modules the way the mutation-capable module's `🎮️commands/*` carries them
/// would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlowViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for FlowViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(FlowViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct FlowViewer;

impl ArtifactViewer for FlowViewer {
    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = FlowViewCommand;

    const DIALECT: Dialect = FLOW_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::flow::retirement::store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<NoConfig, NoConfigMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<NoConfig, NoConfigMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_framework_plugin::NoPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_framework_plugin::NoPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(NoPresence::default()), |_| true).expect("NoPresence is statically empty")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(Box::new(semio_framework_plugin::NoTransientStoreDisposer::new()))
    }

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.child-projection"), error.to_string()))
    }

    fn initial_snapshot() -> FlowSnapshot {
        FlowSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `FlowViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan, "jump to node") is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, mirroring the mutation-capable module's
/// `create_flow_app` doing the equivalent stitching for its own five windows.
pub fn create_flow_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(FLOW_DIALECT).document(["semio", "flow"]).icon_id("flow").mode_def(view::definition()).default_mode_id(view::FLOW_VIEW_MODE_VIEW).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn flow_viewer_member_factory_and_full_store_close_match_neutral_contract() {
        use semio_framework_plugin::{Plugin, PluginApp, PluginCloseStep};
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧹️owners/🔣️.json")).unwrap();
        let definition = create_flow_viewer();
        assert_eq!(definition.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(fixture["role"].as_str().unwrap(), "viewer");
        let id = definition.id.clone();
        let plugin = Plugin::<crate::plugin::FlowApps>::builder("flow-viewer-lifecycle")
            .label("Flow Viewer Lifecycle").version("0.1.0").package_id("semio:flow-viewer-lifecycle")
            .viewer_with_members::<FlowViewer, semio_s_plugin_stdio::artifacts::semio::SemioMembers>(definition)
            .try_build().unwrap();
        let document_rights = plugin.manifest.capabilities.iter().filter(|capability| matches!(capability.artifact, ArtifactKind::Document)).map(|capability| {
            assert!(matches!(capability.scope, Scope::App));
            if matches!(capability.rights, Rights::Read) { "read" } else { "unexpected" }
        }).collect::<Vec<_>>();
        assert_eq!(document_rights, fixture["documentRights"].as_array().unwrap().iter().map(|right| right.as_str().unwrap()).collect::<Vec<_>>());
        let mut app = plugin.create_app(&id).expect("registered Flow viewer factory must retain its typed member fleet");
        assert!(matches!(&app, crate::plugin::FlowApps::FlowViewer(_)));
        let items = fixture["grant"]["items"].as_u64().unwrap() as usize;
        let bytes = fixture["grant"]["bytes"].as_u64().unwrap() as usize;
        let mut completed = false;
        for _ in 0..fixture["maximumSteps"].as_u64().unwrap() {
            match app.close_step(items, bytes).expect("actual viewer closes through its declared five-lane owners") {
                PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= items && released_bytes <= bytes),
                PluginCloseStep::Blocked { .. } => panic!("fresh viewer has no outstanding reader that may block close"),
                PluginCloseStep::Complete => { completed = true; break; }
            }
        }
        assert_eq!(completed, fixture["expected"]["complete"].as_bool().unwrap());
        assert_eq!(app.close_terminal_is_empty(), fixture["expected"]["terminalEmpty"].as_bool().unwrap());
        eprintln!("[DEBUG] real Flow viewer factory retained SemioMembers with read-only document rights and closed all five lanes plus framework interaction");
    }

    #[semio_framework_async_macros::async_test]
    async fn create_flow_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_flow_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, FLOW_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<FlowViewer as ArtifactViewer>::DIALECT, FLOW_DIALECT);
    }
}
//#endregion 🧪️Tests
