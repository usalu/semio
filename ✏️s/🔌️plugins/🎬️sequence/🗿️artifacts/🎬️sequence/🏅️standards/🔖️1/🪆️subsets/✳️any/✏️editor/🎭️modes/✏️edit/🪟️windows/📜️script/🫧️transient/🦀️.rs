//! 🫧️ Ephemeral run result bound to one exact Sequence script window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SequenceScriptWindowTransient {
    pub last_run_json: String,
}

semio_framework_value::artifact_retire_struct!(SequenceScriptWindowTransient { last_run_json });

semio_framework_plugin::transient_root! {
    state: SequenceScriptWindowTransient,
    mutation: SequenceScriptWindowTransientMutation,
    diff: SequenceScriptWindowTransientDiff,
    owner: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Sequence Script Window Transient",
    payload_schema: "sequence.scriptwindowtransient",
    envelope: "s.sequence.sequence.scriptwindowtransient",
    extension: "sequencescriptwindowtransient",
    fields: { last_run_json: String },
}

semio_framework_plugin::window_transient_owners! {
    state: SequenceScriptWindowTransient,
    mutation: SequenceScriptWindowTransientMutation,
    windows: {
        SequenceScriptWindowTransientOwner => super::SEQUENCE_PLAY_WINDOW_SCRIPT,
    },
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
