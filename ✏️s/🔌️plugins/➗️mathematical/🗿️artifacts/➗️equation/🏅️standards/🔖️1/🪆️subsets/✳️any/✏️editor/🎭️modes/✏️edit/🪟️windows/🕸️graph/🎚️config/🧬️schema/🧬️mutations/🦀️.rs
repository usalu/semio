//! 🧬️ Equation configuration mutations with explicit source descriptors.

use super::{EquationCamera, EquationGraphWindowConfig};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
// 🔮️ The test-only serde mirror is the INDEPENDENT oracle `🧫️fixtures/🔁️mutations.json` is read
// through (`language_neutral_mutations_match_json_oracle_and_restore_base` decodes the same vector
// twice — once with `dsl::json`, once with `serde_json` — and asserts they agree). It must therefore
// spell the same wire shape as `#[value(..)]` below: internally tagged on `kind`, kebab-case variant
// names. Without the mirror serde used its default EXTERNALLY tagged form and refused every committed
// vector with `invalid value: map, expected map with a single key`.
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "kebab-case"))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = EquationGraphWindowConfig, diff = EquationGraphWindowConfig, schema = "mathematical.equationgraphwindowconfig")]
pub enum EquationGraphWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}

impl protocol::OpText for EquationGraphWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for EquationGraphWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `EquationGraphWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn equation_graph_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<EquationGraphWindowConfig, EquationGraphWindowConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
