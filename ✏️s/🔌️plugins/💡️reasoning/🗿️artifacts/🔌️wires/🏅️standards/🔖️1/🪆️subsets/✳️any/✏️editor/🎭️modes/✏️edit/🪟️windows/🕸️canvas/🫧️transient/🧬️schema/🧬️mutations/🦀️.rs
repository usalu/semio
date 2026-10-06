//! 🧬️ Wires wires.canvas-window-transient mutation collection.

use super::*;
#[path = "🖱️set-drag/🦀️.rs"]
mod set_drag;
pub use set_drag::SetDrag;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = WiresCanvasTransient, diff = WiresCanvasTransient, schema = "wires.canvas-window-transient")]
pub enum WiresCanvasTransientMutation {
    #[dsl(key = "set-drag")]
    SetDrag(SetDrag),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
