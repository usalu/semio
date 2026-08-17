//! 👁️ Assembly viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `AssemblyViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<AssemblyViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact mutation. Must
//! not import anything from the sibling mutation-capable surface (`policyViewerPurityBreaches`).

use crate::artifacts::assembly::{ASSEMBLY_DIALECT, ASSEMBLY_DOCUMENT_SCHEMA, AssemblyMutation, AssemblySnapshot};
use crate::viewer::assembly::modes::view;
use crate::viewer::assembly::modes::view::windows::structure;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — same shape as `energy.model`'s own view-command precedent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AssemblyViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for AssemblyViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(AssemblyViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct AssemblyViewer;

impl ArtifactViewer for AssemblyViewer {
    type Snapshot = AssemblySnapshot;
    type Mutation = AssemblyMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = AssemblyViewCommand;

    const DIALECT: Dialect = ASSEMBLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ASSEMBLY_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> AssemblySnapshot {
        AssemblySnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `AssemblyViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_assembly_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(ASSEMBLY_DIALECT)
        .document(["semio", "assembly"])
        .icon_id("network")
        .mode_def(view::definition())
        .default_mode_id(view::ASSEMBLY_VIEW_MODE_ID)
        .window_kind_def(structure::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_assembly_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_assembly_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, ASSEMBLY_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<AssemblyViewer as ArtifactViewer>::DIALECT, ASSEMBLY_DIALECT);
    }

    #[test]
    fn viewer_declares_the_structure_window() {
        let def = create_assembly_viewer();
        assert!(def.window_kinds.iter().any(|w| w.id == structure::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
