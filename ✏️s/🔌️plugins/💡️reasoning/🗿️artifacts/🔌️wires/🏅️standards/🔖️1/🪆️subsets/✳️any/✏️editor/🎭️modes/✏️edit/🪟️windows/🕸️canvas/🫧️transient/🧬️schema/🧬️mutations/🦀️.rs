//! 🧬️ Wires wires.canvas-window-transient mutation collection.

use super::*;
use super::schema::{WiresCanvasOptionalNode, WiresCanvasTransientDiff};
#[path = "🖱️set-drag/🦀️.rs"]
mod set_drag;
pub use set_drag::SetDrag;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = WiresCanvasTransient, diff = WiresCanvasTransientDiff, schema = "wires.canvas-window-transient")]
pub enum WiresCanvasTransientMutation {
    #[dsl(key = "set-drag")]
    SetDrag(SetDrag),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
