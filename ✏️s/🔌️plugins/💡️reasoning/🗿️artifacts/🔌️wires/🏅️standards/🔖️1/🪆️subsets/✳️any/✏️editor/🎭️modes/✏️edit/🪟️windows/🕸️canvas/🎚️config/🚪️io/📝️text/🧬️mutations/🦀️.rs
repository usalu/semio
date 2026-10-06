//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::wires::modes::edit::windows::canvas::config::mutations::*;
use crate::editor::wires::modes::edit::windows::canvas::config::{WiresCanvasCamera, WiresCanvasWindowConfig};
use set_camera::SetCamera;

impl protocol::OpText for WiresCanvasWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec exists");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::editor::wires::modes::edit::windows::canvas::config::mutations::*;
use crate::editor::wires::modes::edit::windows::canvas::config::{WiresCanvasCamera, WiresCanvasWindowConfig};
use set_camera::SetCamera;

/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `WiresCanvasWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn wires_canvas_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<WiresCanvasWindowConfig, WiresCanvasWindowConfigMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_wire_codec::*;
