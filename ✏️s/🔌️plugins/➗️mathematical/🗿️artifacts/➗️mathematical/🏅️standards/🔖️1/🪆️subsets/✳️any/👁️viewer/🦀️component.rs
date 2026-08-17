//! 👁️ Mathematical viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `MathematicalViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<MathematicalViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an
//! artifact or draft mutation. MUST NOT import anything from the sibling editor module (the purity
//! check forbids it outright).
//!
//! `Config`/`Presence`/`Transient` are the framework's `NoConfig`/`NoPresence`/`NoTransient` — this
//! viewer needs no persisted per-session state to render (no camera, no locale): the Geometry
//! window's table has nothing view-dependent to remember between renders.

use crate::artifacts::mathematical::{MathematicalSnapshot, MATHEMATICAL_DIALECT, MATH_DOCUMENT_SCHEMA};
use crate::viewer::mathematical::modes::view;
use crate::viewer::mathematical::modes::view::windows::geometry;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer, ui_text};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MathematicalViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for MathematicalViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(MathematicalViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct MathematicalViewer;

impl ArtifactViewer for MathematicalViewer {
    type Snapshot = MathematicalSnapshot;
    type Mutation = crate::artifacts::mathematical::op::MathematicalMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = MathematicalViewCommand;

    const DIALECT: Dialect = MATHEMATICAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> MathematicalSnapshot {
        MathematicalSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `MathematicalViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect,
    /// no dirty scope. Kept as a real dispatch (not `unreachable!()`) so a future view-only action
    /// is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            geometry::BODY_KEY => geometry::render(doc.snapshot),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_mathematical_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MATHEMATICAL_DIALECT)
        .document(["semio", "mathematical"])
        .icon_id("math-app")
        .mode_def(view::definition())
        .default_mode_id(view::MATH_VIEW_MODE_VIEW)
        .window_kind_def(geometry::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_mathematical_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_mathematical_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, MATHEMATICAL_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<MathematicalViewer as ArtifactViewer>::DIALECT, MATHEMATICAL_DIALECT);
    }
}
//#endregion 🧪️Tests
