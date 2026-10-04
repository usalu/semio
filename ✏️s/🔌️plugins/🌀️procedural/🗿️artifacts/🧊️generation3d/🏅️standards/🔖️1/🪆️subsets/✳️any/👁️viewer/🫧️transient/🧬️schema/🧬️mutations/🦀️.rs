//! 🫧️ Generation3d viewer transient — the closed semantic mutation aggregate.

use super::Generation3dViewTransient;

#[path = "👁️set-preview/🦀️.rs"]
mod set_preview_eval;

pub use set_preview_eval::SetPreviewEval;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Generation3dViewTransient, diff = Generation3dViewTransient, schema = "generation3dview.transient")]
pub enum Generation3dViewTransientMutation {
    #[dsl(key = "set-preview-eval")]
    SetPreviewEval(SetPreviewEval),
}

impl protocol::OpText for Generation3dViewTransientMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation3dViewTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🧹️ The mutation's own retirement ladder — the window-transient owner's
/// `OwnedValueRetirementFactory` drains a published evaluation's bytes under a grant instead of
/// dropping them (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
impl semio_framework_value::retirement::RetireOwned for Generation3dViewTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self::SetPreviewEval(SetPreviewEval { eval_text }) = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(eval_text)])
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `Generation3dViewTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn generation3d_view_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Generation3dViewTransient, Generation3dViewTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
