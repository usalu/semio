//! 🫧️ Ephemeral local interaction state of one exact BIM window: what an author is typing, how many pointer gestures the window has seen and the marks of the gesture in progress (`preview`, the JSON of a tool `Preview`, empty when no gesture shows anything). One record serves the plan, world and
//! section windows; each window instance holds its own copy, and nothing of it is shared or persisted.

use crate::editor::bim::modes::edit::windows::{plan, section, world};

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct BimWindowTransient {
    pub engagement_input: String,
    pub pointer_generation: u64,
    pub preview: String,
}

semio_framework_value::artifact_retire_struct!(BimWindowTransient { engagement_input, pointer_generation, preview });

semio_framework_plugin::transient_root! {
    state: BimWindowTransient,
    mutation: BimWindowTransientMutation,
    diff: BimWindowTransientDiff,
    owner: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set BIM Window Transient",
    payload_schema: "bim.window.transient",
    envelope: "s.bim.model.window.transient",
    extension: "bimwindowtransient",
    fields: { engagement_input: String, pointer_generation: u64, preview: String },
}

semio_framework_plugin::window_transient_owners! {
    state: BimWindowTransient,
    mutation: BimWindowTransientMutation,
    windows: {
        BimPlanTransientOwner => plan::WINDOW_KIND_ID,
        BimWorldTransientOwner => world::WINDOW_KIND_ID,
        BimSectionTransientOwner => section::WINDOW_KIND_ID,
    },
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
