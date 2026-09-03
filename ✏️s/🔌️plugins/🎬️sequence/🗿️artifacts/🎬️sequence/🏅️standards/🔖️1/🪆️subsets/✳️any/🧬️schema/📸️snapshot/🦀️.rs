//! 🧬️ Sequence snapshot schema — artifact-lane fields only.

use crate::artifacts::sequence::{require_sequence_working_scene, sequence_content_child_with_owner, SequenceContentChild, SequenceEdge, SequenceStep, SEQUENCE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted sequence document snapshot. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`
/// (`sequence→C:flow`): the inline `steps: Vec<SequenceStep>` / `edges: Vec<SequenceEdge>` content
/// fields are replaced by a fixed composed `s.stdio.semio.flow` CHILD slot — the sequence plugin no
/// longer defines its own step-DAG content model, it composes stdio's `flow` subset instead.
/// `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub content: SequenceContentChild,
}

impl Default for SequenceSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> SequenceSnapshot {
    SequenceSnapshot::from_fixture(SequenceFixture {
        schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
        steps: vec![
            SequenceStep {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: crate::artifacts::sequence::StepParams::new().insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("counter".into()))).insert("value", neural_engine::Value::Atom(neural_engine::Atom::Integer(0))),
                x: 0.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
            SequenceStep {
                id: "step-2".into(),
                kind: "log.print".into(),
                params: crate::artifacts::sequence::StepParams::new().insert("message", neural_engine::Value::Atom(neural_engine::Atom::String("hello sequence".into()))),
                x: 280.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
        ],
        edges: vec![SequenceEdge { id: "edge-1".into(), from: "step-1".into(), to: "step-2".into() }],
    })
}
//#endregion 🔖️Snapshot

//#region 🔖️Fixture
/// 🌊️ The plain pre-migration document shape (`{schema, steps, edges}`) — this plugin's own
/// analog of `flow::FlowFixture`: the live editing representation `SequenceHost` and the WASM
/// bridge operate on, and the JSON wire contract `SequenceHost::to_json`/`load_json` still speak.
/// Bridges to/from the composed-child `SequenceSnapshot` via `to_fixture`/`from_fixture` below.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SequenceFixture {
    pub schema: String,
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

impl SequenceSnapshot {
    /// 🌱 Builds a persisted snapshot from a plain fixture and transfers its scene to the exact
    /// composed child owner.
    pub fn from_fixture(fixture: SequenceFixture) -> Self {
        Self { schema: fixture.schema, content: sequence_content_child_with_owner(fixture.steps, fixture.edges) }
    }

    /// 🌱 Converts this snapshot into the plain fixture shape from its exact child-owned scene.
    pub fn try_to_fixture(&self) -> Result<SequenceFixture, store::ArtifactChildMaterializationError> {
        let scene = require_sequence_working_scene(&self.content)?;
        Ok(SequenceFixture { schema: self.schema.clone(), steps: scene.steps, edges: scene.edges })
    }

    /// 🌱 Converts a snapshot known to be materialized into its fixture shape.
    pub fn to_fixture(&self) -> SequenceFixture {
        self.try_to_fixture().expect("sequence child scene must be materialized before fixture projection")
    }
}
//#endregion 🔖️Fixture
