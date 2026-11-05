//! 🫧️ WFC 3D app-transient mutation aggregate — one verb, because the solve result is replaced
//! wholesale by whoever last ran the inference.

use crate::editor::grid3d::transient::{Grid3dTransient};

#[path = "🏁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Grid3dTransient, diff = Grid3dTransientDiff, schema = "wfc.grid3d.transient", reference = Grid3dTransient, reference_diff = Grid3dTransientDiff, retained)]
pub enum Grid3dTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}






/// 🔺️ Field-sparse diff of [`Grid3dTransient`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Grid3dTransientDiff {
    pub payload_json: Option<String>,
}

protocol::retained_row_patch!(pub retained_fields for Grid3dTransientDiff => Grid3dTransient { payload_json: String });
protocol::retained_patch_diff!(Grid3dTransientDiff => Grid3dTransient);

impl protocol::DiffAlgebra<Grid3dTransient> for Grid3dTransientDiff {
    fn is_empty(&self) -> bool {
        self.payload_json.is_none()
    }
}

impl protocol::ColdDiffAlgebra<Grid3dTransient> for Grid3dTransientDiff {
    fn inverse(&self, base: &Grid3dTransient) -> Self {
        Self {
            payload_json: self.payload_json.as_ref().map(|_|base.payload_json.clone()),
        }
    }
}

impl protocol::MutationDiff<Grid3dTransient> for Grid3dTransientDiff {}

impl protocol::ColdMutationDiff<Grid3dTransient> for Grid3dTransientDiff {
    fn absorb(&mut self, later: Self) {
        if later.payload_json.is_some(){self.payload_json=later.payload_json;}
    }

    fn apply(&self, unpublished: &mut Grid3dTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value)=&self.payload_json {next.payload_json.clone_from(value);}
        *unpublished = next;
        Ok(())
    }
}
//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

impl protocol::OpBinary for Grid3dTransientMutation {
    fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{dsl::variants_binary::encode_op(self)}
    fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{dsl::variants_binary::decode_op(bytes)}
}

impl protocol::OpText for Grid3dTransientMutation {
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
