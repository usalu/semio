//! 🫧️ WFC 3D app-transient mutation aggregate — one verb, because the solve result is replaced
//! wholesale by whoever last ran the inference.

use super::Wfc3dTransient;

#[path = "🏁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Wfc3dTransient, diff = Wfc3dTransient, schema = "wfc.wfc3d.transient")]
pub enum Wfc3dTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}

impl protocol::OpText for Wfc3dTransientMutation {
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

impl protocol::OpBinary for Wfc3dTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::MutationDiff<Wfc3dTransient> for Wfc3dTransient {
    fn apply(&self, _base: &Wfc3dTransient) -> protocol::MutationApplyResult<Wfc3dTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
