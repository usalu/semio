//! 🦠️ `🏷️rename` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameGeneration {
    pub id: String,
    pub name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_generation<Op: From<RenameGeneration>>(id: String, name: String) -> Op {
    Op::from(RenameGeneration { id, name })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for RenameGeneration
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<RenameGeneration>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "generation", kind: "rename-generation", record: "RenamedGeneration" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::rename_generation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rename_generation::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename generation \"{}\" to \"{}\"", self.id, self.name), &format!("Erzeugung \"{}\" in \"{}\" umbenennen", self.id, self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
