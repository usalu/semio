//! 🫧️ Ephemeral local engagement state bound to one exact CAD world window (design §17.4 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the action line, the REPL step, the live interaction session and the
//! interaction the window last finalized. Per-frame state never lives in config, so typing a line, a pointer move and a
//! possible-select publish nothing durable and list no history row; only a commit lands, as one transform-tool transaction.

use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};

/// 🫧️ One CAD world window's engagement state — see the module doc and `🧬️schema/🔣️.json`.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadWorldWindowTransient {
    pub engagement_input: String,
    pub engagement_step: String,
    pub engagement_pane: Option<String>,
    pub engagement_session_json: Option<String>,
    pub last_finalized_interaction_id: Option<String>,
}

impl Default for CadWorldWindowTransient {
    fn default() -> Self {
        Self { engagement_input: String::new(), engagement_step: "Idle".into(), engagement_pane: None, engagement_session_json: None, last_finalized_interaction_id: None }
    }
}

semio_framework_value::artifact_retire_struct!(CadWorldWindowTransient { engagement_input, engagement_step, engagement_pane, engagement_session_json, last_finalized_interaction_id });

semio_framework_plugin::transient_root! {
    state: CadWorldWindowTransient,
    mutation: CadWorldWindowTransientMutation,
    owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set CAD World Window Transient",
    payload_schema: "cad.worldwindowtransient",
    envelope: "cad.worldwindowtransient",
    extension: "cadworldwindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: CadWorldWindowTransient,
    mutation: CadWorldWindowTransientMutation,
    windows: {
        CadShapeWindowTransientOwner => shape::WINDOW_KIND_ID,
        CadBuildingWindowTransientOwner => building::WINDOW_KIND_ID,
        CadEnergyWindowTransientOwner => energy::WINDOW_KIND_ID,
        CadStructureClassicWindowTransientOwner => structure_classic::WINDOW_KIND_ID,
    },
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
