//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::wfc2d::config::mutations::*;
use crate::editor::wfc2d::config::Wfc2dConfig;
use crate::editor::wfc2d::config::mutations::ReplaceConfig;
use crate::editor::wfc2d::config::mutations::ChangeCamera;
use crate::editor::wfc2d::config::mutations::ChangeActiveTile;

impl protocol::OpText for Wfc2dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
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
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::editor::wfc2d::config::mutations::*;
use crate::editor::wfc2d::config::Wfc2dConfig;
use crate::editor::wfc2d::config::mutations::ReplaceConfig;
use crate::editor::wfc2d::config::mutations::ChangeCamera;
use crate::editor::wfc2d::config::mutations::ChangeActiveTile;

/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Wfc2dConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn wfc2d_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Wfc2dConfig, Wfc2dConfigMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_wire_codec::*;
