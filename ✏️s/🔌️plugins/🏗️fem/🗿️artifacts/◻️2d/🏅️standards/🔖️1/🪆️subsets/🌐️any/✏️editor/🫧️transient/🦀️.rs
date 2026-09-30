//! 🫧️ The FEM editors' local-only transient: the gumball gesture each window has in flight, persisted between the
//! dispatches of one streamed gesture — its tool statechart configuration by stable ids, the admission and base
//! revision it opened on, and its open `ToolTransaction`'s provisional entries in value form. Artifact-level rather
//! than window-level so the SIBLING windows paint the gesture's preview too (the results window re-solves the moved
//! structure while the drag goes on), and keyed by the owning window id so two windows never clobber each other's
//! gesture. Never history, never shared; fem 2d and fem 3d both keep their gestures here.
//! Schema of record: `🧬️schema/🔣️.json`; design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.

use semio_framework_tool_machine::{ToolRefusal, ToolTransaction, ToolTransactionState};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️State
/// 🫧️ Every open gumball gesture of one FEM artifact instance, by owning window id.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FemGumballTransient {
    pub gestures: BTreeMap<String, FemGumballGesture>,
}

/// 💾️ One window's in-flight gumball gesture.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FemGumballGesture {
    pub states: Vec<String>,
    pub verb: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: protocol::TransactionRef,
    pub entries: Vec<FemGumballEntry>,
}

/// 🧷️ One provisional entry of the open transaction, its mutation in value form.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FemGumballEntry {
    pub key: String,
    pub mutation: dsl::DslValue,
}

impl FemGumballGesture {
    /// 💾️ The gesture a runner's parts persist: `Some` only while its transaction is open.
    pub fn persist<M: dsl::ToValue>(states: Vec<String>, verb: &str, authoring_seed: &str, base_revision: &str, transaction: Option<ToolTransaction<M>>) -> Option<Self> {
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(Self {
            states,
            verb: verb.to_string(),
            authoring_seed: authoring_seed.to_string(),
            base_revision: base_revision.to_string(),
            transaction: transaction.reference().clone(),
            entries: transaction.entries().iter().map(|(key, mutation)| FemGumballEntry { key: key.clone(), mutation: dsl::ToValue::to_value(mutation) }).collect(),
        })
    }

    /// 🧷️ The provisional entries decoded back into mutations; one that no longer decodes refuses the gesture (`Closed`).
    pub fn entries<M: dsl::FromValue>(&self) -> Result<Vec<(String, M)>, ToolRefusal> {
        self.entries.iter().map(|entry| Ok((entry.key.clone(), dsl::FromValue::from_value(entry.mutation.clone()).map_err(|_| ToolRefusal::Closed)?))).collect()
    }

    /// ⏯️ The statechart snapshot this gesture persisted, restored for machine `T` with `context`; a configuration the
    /// current chart cannot restore is refused (`Closed`), so the caller drops the gesture with zero trace.
    pub fn snapshot<T: machine::Machine>(&self, context: T::Context) -> Result<machine::Snapshot<T>, ToolRefusal> {
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: T::definition().fingerprint, states: self.states.clone(), history: Vec::new(), done: false };
        machine::restore::<T, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)
    }
}

impl FemGumballTransient {
    /// 🪟️ This transient with `window`'s gesture replaced by `gesture` (`None` clears it).
    pub fn with_gesture(&self, window: &str, gesture: Option<FemGumballGesture>) -> Self {
        let mut gestures = self.gestures.clone();
        match gesture {
            Some(gesture) => gestures.insert(window.to_string(), gesture),
            None => gestures.remove(window),
        };
        Self { gestures }
    }

    /// 👁️ `document` with every open gesture's provisional entries applied — the preview every window of this
    /// instance paints, never history. An entry that does not decode or apply is skipped.
    pub fn preview<S: Clone, M: dsl::FromValue + protocol::Mutation<S>>(&self, document: &S) -> Option<S> {
        let entries: Vec<M> = self.gestures.values().flat_map(|gesture| &gesture.entries).filter_map(|entry| dsl::FromValue::from_value(entry.mutation.clone()).ok()).collect();
        (!entries.is_empty()).then(|| entries.iter().fold(document.clone(), |state, mutation| protocol::MutationDiff::apply(protocol::Mutation::diff(mutation, &state).diff(), &state).unwrap_or(state)))
    }
}
//#endregion 🔖️State

//#region 🛠️Drive
/// 🎚️ Where one transform dispatch sits in a gumball gesture: a one-shot `Once`, a `Stream` tick into the window's
/// open transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FemGumballPhase {
    Once,
    Stream,
    Commit,
    Abort(semio_framework_tool_machine::ToolAbortReason),
}

impl FemGumballPhase {
    /// 🧩️ Reads a transform verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason`
    /// (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(semio_framework_tool_machine::ToolAbortReason::Tool), semio_framework_tool_machine::ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 🛠️ One FEM editor's gumball tool for one dispatch: a tool-machine runner over that editor's relative leaf, started
/// at rest or resumed from the gesture its window persisted.
pub trait FemGumballTool: Sized {
    type Leaf;
    type Mutation;
    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal>;
    fn resume(gesture: &FemGumballGesture) -> Result<Self, ToolRefusal>;
    fn verb(&self) -> &str;
    fn base_revision(&self) -> &str;
    fn at_rest(&self) -> bool;
    fn abort(&mut self, reason: semio_framework_tool_machine::ToolAbortReason);
    fn send(&mut self, phase: FemGumballPhase, tick: Option<Self::Leaf>) -> Result<semio_framework_tool_machine::ToolStep<Self::Mutation>, ToolRefusal>;
    fn persist(self) -> Option<FemGumballGesture>;
}

/// 🧮️ What one gumball dispatch did: the transaction it committed (publish it as ONE edit) and the window's next
/// transient when the dispatch opened, advanced, committed or dropped a gesture.
pub struct FemGumballDrive<M> {
    pub committed: Option<(protocol::TransactionRef, Vec<M>)>,
    pub transient: Option<FemGumballTransient>,
}

/// 🛠️ Drives `window`'s gumball tool through ONE dispatch of a transform verb. `Once` commits `tick` as one transaction;
/// `Stream` upserts it into the window's open transaction (opening it on the first tick), `Commit` folds it in and
/// commits the whole gesture, `Abort` drops the open gesture with zero trace. An open gesture another verb or a one-shot
/// interrupts is aborted `captureLost`; one whose base moved under it is aborted `baseMoved`, and a stream tick or
/// commit that found it is dropped with it.
pub fn fem_gumball_drive<T: FemGumballTool>(transient: &FemGumballTransient, window: &str, verb: &str, phase: FemGumballPhase, tick: Option<T::Leaf>, authoring_seed: &str, base_revision: &str) -> FemGumballDrive<T::Mutation> {
    use semio_framework_tool_machine::{ToolAbortReason, ToolStep};
    let persisted = transient.gestures.get(window);
    let open = persisted.and_then(|gesture| T::resume(gesture).ok());
    let dropped = FemGumballDrive { committed: None, transient: persisted.map(|_| transient.with_gesture(window, None)) };
    let open = match (open, phase) {
        (Some(mut tool), FemGumballPhase::Abort(reason)) => {
            tool.abort(reason);
            return dropped;
        }
        (None, FemGumballPhase::Abort(_)) => return dropped,
        (Some(mut tool), _) if tool.base_revision() != base_revision => {
            tool.abort(ToolAbortReason::BaseMoved);
            if phase != FemGumballPhase::Once {
                return dropped;
            }
            None
        }
        (Some(mut tool), _) if tool.verb() != verb || phase == FemGumballPhase::Once => {
            tool.abort(ToolAbortReason::CaptureLost);
            None
        }
        (open, _) => open,
    };
    let Some(mut tool) = open.or_else(|| T::start(verb, authoring_seed, base_revision).ok()) else { return dropped };
    let step = tool.send(phase, tick);
    let next = transient.with_gesture(window, tool.persist());
    let changed = (next != *transient).then_some(next);
    match step {
        Ok(ToolStep::Committed(reference, mutations)) => FemGumballDrive { committed: Some((reference, mutations)), transient: changed },
        Ok(_) | Err(_) => FemGumballDrive { committed: None, transient: changed },
    }
}
//#endregion 🛠️Drive

//#region 🔖️Mutation
/// 🫧️ The one transient mutation: replace the whole root.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum FemGumballTransientMutation {
    Snapshot { transient: FemGumballTransient },
}

impl protocol::MutationDiff<FemGumballTransient> for FemGumballTransient {
    fn apply(&self, _base: &FemGumballTransient) -> protocol::MutationApplyResult<FemGumballTransient> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl protocol::Mutation<FemGumballTransient> for FemGumballTransientMutation {
    type Diff = FemGumballTransient;

    /// 🧷️ Per-variant leaf metadata of this hand-written transient aggregate — its sole `Snapshot` variant.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🫧️transient",
        semantic_kind: "set-snapshot",
        display_name: "Set Gumball Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }

    fn diff(&self, _base: &FemGumballTransient) -> protocol::MutationOutcome<FemGumballTransient> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }

    fn inverse(&self, base: &FemGumballTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::OpText for FemGumballTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
}

impl protocol::OpBinary for FemGumballTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(dsl::json::to_json_string(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Mutation

//#region 🔖️Codecs
impl store::ArtifactDsl for FemGumballTransient {
    const EXTENSION: &'static str = "femgumballtransient";

    fn envelope_id() -> &'static str {
        "fem.gumballtransient"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        dsl::json::from_json_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid FEM gumball transient envelope");
        store::semio_format::wrap_text(&envelope, &dsl::json::to_json_string(self))
    }
}

impl store::ArtifactPack for FemGumballTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, dsl::json::to_json_string(self).as_bytes()))
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("FEM gumball transient pack envelope mismatch".into()));
        }
        let text = std::str::from_utf8(&inner).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::json::from_json_str(text).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}
//#endregion 🔖️Codecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
