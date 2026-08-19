//! 🧬️ Sequence snapshot schema — artifact-lane fields only.

use crate::artifacts::sequence::{sequence_content_child_handle_and_cache, sequence_working_scene, SequenceContentChild, SequenceEdge, SequenceStep, SEQUENCE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted sequence document snapshot. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`
/// (`sequence→C:flow`): the inline `steps: Vec<SequenceStep>` / `edges: Vec<SequenceEdge>` content
/// fields are replaced by a fixed composed `s.stdio.semio.flow` CHILD slot — the sequence plugin no
/// longer defines its own step-DAG content model, it composes stdio's `flow` subset instead.
/// `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub content: SequenceContentChild,
}

impl Default for SequenceSnapshot {
    async fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub async fn default_snapshot() -> SequenceSnapshot {
    SequenceSnapshot::from_fixture(SequenceFixture {
        schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
        steps: vec![
            SequenceStep {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: crate::artifacts::sequence::StepParams::new()
                    .insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("counter".into())))
                    .insert("value", neural_engine::Value::Atom(neural_engine::Atom::Integer(0))),
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFixture {
    pub schema: String,
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

impl SequenceSnapshot {
    /// 🌱 Builds a persisted snapshot from a plain fixture — mints and caches a fresh
    /// content-addressed handle for the fixture's steps/edges.
    pub async fn from_fixture(fixture: SequenceFixture) -> Self {
        Self { schema: fixture.schema, content: sequence_content_child_handle_and_cache(fixture.steps, fixture.edges) }
    }

    /// 🌱 Converts this snapshot into the plain fixture shape — reads the live steps/edges off the
    /// working-scene cache (see `sequence_working_scene`'s doc comment for the staleness gap this
    /// bridges).
    pub async fn to_fixture(&self) -> SequenceFixture {
        let scene = sequence_working_scene(self);
        SequenceFixture { schema: self.schema.clone(), steps: scene.steps, edges: scene.edges }
    }
}
//#endregion 🔖️Fixture

