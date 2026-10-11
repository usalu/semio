//! 🏷️ `rename-generation` payload — changes a generation's identity `name` field.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
//#region 🔖️RenameGeneration
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameGeneration {
    pub id: String,
    pub new_name: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for RenameGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "generation", kind: "rename-generation", record: "RenamedGeneration" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::rename_generation::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::rename_generation::inverse::inverse(self, base)?
    
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
