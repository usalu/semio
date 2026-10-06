//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::generation2d::transient::component::mutations::*;
use crate::editor::generation2d::transient::component::Generation2dTransient;
use set_generation_preview::SetGenerationPreview;

impl protocol::OpText for Generation2dTransientMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        for (keyword, spec_fn) in <Self as semio_framework_dsl_record::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let spec_fn = <Self as semio_framework_dsl_record::DslVariants>::variants().into_iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec).expect("declared transient variant");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::editor::generation2d::transient::component::mutations::*;
use crate::editor::generation2d::transient::component::Generation2dTransient;
use set_generation_preview::SetGenerationPreview;

/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Generation2dTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn generation2d_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Generation2dTransient, Generation2dTransientMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_wire_codec::*;
