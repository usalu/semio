//! 🫧️ Ephemeral local Flow state bound to the exact invoking window.

use crate::playbook::GenerationPlayState;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowWindowTransient {
    pub generation_json: String,
    pub duplicate_widget_progress_json: String,
}

impl FlowWindowTransient {
    pub fn generation(&self) -> Result<semio_framework_dsl_record::__rt::DecodedFieldOwner<GenerationPlayState>, semio_framework_value::ValueError> {
        let state = if self.generation_json.is_empty() { GenerationPlayState::default() } else { semio_framework_pack_json::from_json_str(&self.generation_json, semio_framework_pack_json::JsonMemberPolicy::Reject)? };
        Ok(semio_framework_dsl_record::__rt::DecodedFieldOwner::new(state, |state| crate::playbook::GenerationPlayRoot::from(state).retire_cold()))
    }
}

semio_framework_value::artifact_retire_struct!(FlowWindowTransient { generation_json, duplicate_widget_progress_json });

semio_framework_plugin::transient_root! {
    state: FlowWindowTransient,
    mutation: FlowWindowTransientMutation,
    owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Flow Window Transient",
    payload_schema: "flow.windowtransient",
    envelope: "s.flow.flow.windowtransient",
    extension: "flowwindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: FlowWindowTransient,
    mutation: FlowWindowTransientMutation,
    windows: {
        FlowMainWindowTransientOwner => super::FLOW_PLAY_WINDOW_MAIN,
        FlowGenerationsWindowTransientOwner => crate::editor::flow::modes::generate::windows::generations::FLOW_PLAY_WINDOW_GENERATIONS,
        FlowFormWindowTransientOwner => crate::editor::flow::modes::generate::windows::form::FLOW_PLAY_WINDOW_GENERATE_FORM,
        FlowPreviewWindowTransientOwner => crate::editor::flow::modes::generate::windows::preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW,
    },
}
