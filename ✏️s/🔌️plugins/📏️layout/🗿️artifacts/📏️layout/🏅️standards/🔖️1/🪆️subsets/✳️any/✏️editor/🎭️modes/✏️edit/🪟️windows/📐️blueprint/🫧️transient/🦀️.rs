//! 🫧️ Ephemeral Layout interaction state bound to one exact Blueprint or Preview window.

use super::transform::LayoutTransformToolState;
use crate::LayoutDropPreviewState;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LayoutWindowTransient {
    pub drop_preview: LayoutDropPreviewState,
    pub engagement_input: String,
    /// 🛠️ The Blueprint window's in-flight transform-tool gesture: statechart configuration and open transaction,
    /// persisted between dispatches so one streamed gumball gesture stays ONE transaction; `None` at rest. Boxed: the
    /// window transient publishes through the ephemeral ownership transfer, whose inline bound an inline state exceeds.
    #[value(default)]
    pub transform_tool: Option<Box<LayoutTransformToolState>>,
}

impl semio_framework_value::retirement::RetireOwned for LayoutDropPreviewState {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::sequence(vec![
            semio_framework_value::retirement::RetireOwned::retirement(self.kind),
            semio_framework_value::retirement::RetireOwned::retirement(self.x),
            semio_framework_value::retirement::RetireOwned::retirement(self.y),
        ])
    }
}

semio_framework_value::artifact_retire_struct!(LayoutWindowTransient { drop_preview, engagement_input, transform_tool });

semio_framework_plugin::transient_root! {
    state: LayoutWindowTransient,
    mutation: LayoutWindowTransientMutation,
    diff: LayoutWindowTransientDiff,
    owner: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Layout Window Transient",
    payload_schema: "layout.windowtransient",
    envelope: "s.layout.layout.windowtransient",
    extension: "layoutwindowtransient",
    fields: { drop_preview: LayoutDropPreviewState, engagement_input: String, transform_tool: Option<Box<LayoutTransformToolState>> },
}

semio_framework_plugin::window_transient_owners! {
    state: LayoutWindowTransient,
    mutation: LayoutWindowTransientMutation,
    windows: {
        LayoutBlueprintWindowTransientOwner => super::LAYOUT_PLAY_WINDOW_BLUEPRINT,
        LayoutPreviewWindowTransientOwner => crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW,
    },
}
