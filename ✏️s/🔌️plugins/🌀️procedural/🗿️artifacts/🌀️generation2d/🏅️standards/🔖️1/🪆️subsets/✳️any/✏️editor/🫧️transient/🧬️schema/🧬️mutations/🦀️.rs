//! 🫧️ Generation2d app-transient mutation aggregate.

use super::Generation2dTransient;

#[path = "👁️set-generation-preview/🦀️.rs"]
mod set_generation_preview;
pub use set_generation_preview::SetGenerationPreview;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Generation2dTransient, diff = Generation2dTransient, schema = "procedural.generation2dtransient")]
pub enum Generation2dTransientMutation {
    #[dsl(key = "set-generation-preview")]
    SetGenerationPreview(SetGenerationPreview),
}

impl protocol::OpText for Generation2dTransientMutation {
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
        let spec_fn = <Self as dsl::DslVariants>::variants().into_iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec).expect("declared transient variant");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation2dTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Generation2dTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn generation2d_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Generation2dTransient, Generation2dTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
