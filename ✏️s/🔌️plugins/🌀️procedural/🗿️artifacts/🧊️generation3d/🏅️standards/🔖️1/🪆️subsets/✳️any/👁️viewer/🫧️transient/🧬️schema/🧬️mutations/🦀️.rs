//! 🫧️ Generation3d viewer transient — the closed semantic mutation aggregate.

use super::Generation3dViewTransient;

#[path = "👁️set-preview-eval/🦀️.rs"]
mod set_preview_eval;

pub use set_preview_eval::SetPreviewEval;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Generation3dViewTransient, diff = Generation3dViewTransient, schema = "generation3dview.transient")]
pub enum Generation3dViewTransientMutation {
    #[dsl(key = "set-preview-eval")]
    SetPreviewEval(SetPreviewEval),
}

impl protocol::OpText for Generation3dViewTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
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
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
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
impl store::retirement::RetireOwned for Generation3dViewTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let Self::SetPreviewEval(SetPreviewEval { eval_text }) = self;
        store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(eval_text)])
    }
}
