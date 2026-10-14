//! 🏷️ `rename-generation` payload — changes a generation's identity `name` field.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
//#region 🔖️RenameGeneration
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameGeneration {
    pub id: String,
    pub new_name: String,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for RenameGeneration
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<RenameGeneration>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "generation", kind: "rename-generation", record: "RenamedGeneration" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::rename_generation::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rename_generation::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename generation \"{}\" to \"{}\"", self.id, self.new_name), &format!("Erzeugung \"{}\" in \"{}\" umbenennen", self.id, self.new_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️RenameGeneration
