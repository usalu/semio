//! 🫧️ Ephemeral local-only state of ONE exact WFC Bitmap Input window: the brush stroke in flight. A stroke
//! spans many dispatches (one per pointer batch), so its tool state — the statechart configuration and the open
//! `ToolTransaction` holding the one provisional `paint-input-stroke` leaf — lives here between them. Tool state is
//! never config and never history; only the committed leaf is (design §5, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️State
/// 🖌️ A window's in-flight brush stroke: the statechart configuration by stable ids, the admission seed the
/// transaction was minted under, the open `TransactionRef`, and the one provisional leaf in value form.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapBrushToolState {
    pub states: Vec<String>,
    pub authoring_seed: String,
    pub transaction: protocol::TransactionRef,
    pub stroke: semio_framework_value::DslValue,
}

/// 🫧️ The input window's transient partition: the brush stroke in flight, `None` at rest. Boxed: a gesture state
/// exceeds the ephemeral ownership transfer's inline bound.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapInputWindowTransient {
    pub brush: Option<Box<BitmapBrushToolState>>,
}

semio_framework_value::artifact_retire_struct!(BitmapBrushToolState { states, authoring_seed, transaction, stroke });
semio_framework_value::artifact_retire_struct!(BitmapInputWindowTransient { brush });
//#endregion 🔖️State

//#region 🔖️Owner
semio_framework_plugin::transient_root! {
    state: BitmapInputWindowTransient,
    mutation: BitmapInputWindowTransientMutation,
    owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Bitmap Input Window Transient",
    payload_schema: "wfcbitmap.inputwindowtransient",
    envelope: "s.wfc.bitmap.inputwindowtransient",
    extension: "wfcbitmapinputwindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: BitmapInputWindowTransient,
    mutation: BitmapInputWindowTransientMutation,
    windows: {
        BitmapInputWindowTransientOwner => super::WFC_BITMAP_WINDOW_INPUT,
    },
}
//#endregion 🔖️Owner
