//! 🧬️ Wires wires.canvas-window-transient mutation collection.

use super::*;
#[path = "🖱️set-drag/🦀️.rs"]
mod set_drag;
pub use set_drag::SetDrag;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = WiresCanvasTransient, diff = WiresCanvasTransient, schema = "wires.canvas-window-transient")]
pub enum WiresCanvasTransientMutation {
    #[dsl(key = "set-drag")]
    SetDrag(SetDrag),
}

impl protocol::OpText for WiresCanvasTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(key, _)| key == &keyword).expect("declared variant").1();
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for WiresCanvasTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `WiresCanvasTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn wires_canvas_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<WiresCanvasTransient, WiresCanvasTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
