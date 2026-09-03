//! 👁️ Equation viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EquationViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EquationViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an
//! artifact or draft mutation. MUST NOT import anything from the sibling editor module (the purity
//! check forbids it outright).
//!
//! `Config`/`Presence`/`Transient` are the framework's `NoConfig`/`NoPresence`/`NoTransient` — this
//! viewer needs no persisted per-session state to render (no camera, no locale): the Geometry
//! window's table has nothing view-dependent to remember between renders.

use crate::artifacts::equation::{EquationSnapshot, EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
use crate::viewer::equation::modes::view;
use crate::viewer::equation::modes::view::windows::geometry;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ui_text, ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EquationViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for EquationViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(EquationViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EquationViewer;

impl ArtifactViewer for EquationViewer {
    type Snapshot = EquationSnapshot;
    type Mutation = crate::artifacts::equation::op::EquationMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EquationViewCommand;

    const DIALECT: Dialect = EQUATION_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> EquationSnapshot {
        EquationSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `EquationViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect,
    /// no dirty scope. Kept as a real dispatch (not `unreachable!()`) so a future view-only action
    /// is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            geometry::BODY_KEY => geometry::render(doc.snapshot),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_equation_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(EQUATION_DIALECT)
        .document(["semio", "equation"])
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

    #[semio_framework_async_macros::async_test]
    async fn create_equation_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_equation_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, EQUATION_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<EquationViewer as ArtifactViewer>::DIALECT, EQUATION_DIALECT);
    }
}
//#endregion 🧪️Tests
