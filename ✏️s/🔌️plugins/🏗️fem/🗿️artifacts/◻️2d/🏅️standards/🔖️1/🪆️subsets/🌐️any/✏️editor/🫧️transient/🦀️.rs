//! 🫧️ The FEM editors' local-only transient: the gumball gesture each window has in flight, persisted between the
//! dispatches of one streamed gesture — its tool statechart configuration by stable ids, the admission and base
//! revision it opened on, and its open `ToolTransaction`'s provisional entries in value form. Artifact-level rather
//! than window-level so the SIBLING windows paint the gesture's preview too (the results window re-solves the moved
//! structure while the drag goes on), and keyed by the owning window id so two windows never clobber each other's
//! gesture. Never history, never shared; fem 2d and fem 3d both keep their gestures here.
//! Schema of record: `🧬️schema/🔣️.json`; design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.

use semio_framework_tool_machine::{GesturePhase, GestureTool, ToolRefusal, ToolTransaction, ToolTransactionState};
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
    pub mutation: semio_framework_value::DslValue,
}

impl FemGumballGesture {
    /// 💾️ The gesture a runner's parts persist: `Some` only while its transaction is open.
    pub fn persist<M: semio_framework_value::ToValue>(states: Vec<String>, verb: &str, authoring_seed: &str, base_revision: &str, transaction: Option<ToolTransaction<M>>) -> Option<Self> {
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(Self {
            states,
            verb: verb.to_string(),
            authoring_seed: authoring_seed.to_string(),
            base_revision: base_revision.to_string(),
            transaction: transaction.reference().clone(),
            entries: transaction.entries().iter().map(|(key, mutation)| FemGumballEntry { key: key.clone(), mutation: semio_framework_value::ToValue::to_value(mutation) }).collect(),
        })
    }

    /// 🧷️ The provisional entries decoded back into mutations; one that no longer decodes refuses the gesture (`Closed`).
    pub fn entries<M: semio_framework_value::FromValue>(&self) -> Result<Vec<(String, M)>, ToolRefusal> {
        self.entries.iter().map(|entry| Ok((entry.key.clone(), semio_framework_value::FromValue::from_value(entry.mutation.clone()).map_err(|_| ToolRefusal::Closed)?))).collect()
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
    pub fn preview<S: Clone, M: semio_framework_value::FromValue + protocol::Mutation<S>>(&self, document: &S) -> Option<S> {
        let entries: Vec<M> = self.gestures.values().flat_map(|gesture| &gesture.entries).filter_map(|entry| semio_framework_value::FromValue::from_value(entry.mutation.clone()).ok()).collect();
        (!entries.is_empty()).then(|| entries.iter().fold(document.clone(), |state, mutation| protocol::apply_diff(protocol::Mutation::diff(mutation, &state).diff(), &state).unwrap_or(state)))
    }
}
//#endregion 🔖️State

//#region 🛠️Drive
/// 🧮️ What one gumball dispatch did: the transaction it committed (publish it as ONE edit) and the next transient when
/// the dispatch opened, advanced, committed or dropped the window's gesture.
pub struct FemGumballDrive<M> {
    pub committed: Option<(protocol::TransactionRef, Vec<M>)>,
    pub transient: Option<FemGumballTransient>,
}

/// 🛠️ Drives `window`'s gumball tool through ONE dispatch of a transform verb on the shared streamed-gesture runner
/// ([`semio_framework_tool_machine::drive_gesture`]) against the gesture this transient holds for that window; a refused
/// start or tick raises its tool-transaction fault (`toolTransaction.closed` | `toolTransaction.unclosed`).
pub fn fem_gumball_drive<T: GestureTool<Gesture = FemGumballGesture>>(
    transient: &FemGumballTransient,
    window: &str,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<FemGumballDrive<T::Mutation>, semio_framework_plugin::Fault> {
    let drive = semio_framework_tool_machine::drive_gesture::<T>(transient.gestures.get(window), verb, phase, tick, authoring_seed, base_revision)
        .map_err(|refusal| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, refusal.code(), "the gumball tool refused the dispatch"))?;
    Ok(FemGumballDrive { committed: drive.committed, transient: drive.next.map(|gesture| transient.with_gesture(window, gesture)) })
}
//#endregion 🛠️Drive

//#region 🔖️Mutation
semio_framework_plugin::transient_root! {
    state: FemGumballTransient,
    mutation: FemGumballTransientMutation,
    diff: FemGumballTransientDiff,
    owner: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🫧️transient",
    kind: "set-snapshot",
    display_name: "Set Gumball Transient",
    payload_schema: "🧬️schema/🔣️.json",
    envelope: "fem.gumballtransient",
    extension: "femgumballtransient",
    fields: { gestures: BTreeMap<String, FemGumballGesture> },
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
