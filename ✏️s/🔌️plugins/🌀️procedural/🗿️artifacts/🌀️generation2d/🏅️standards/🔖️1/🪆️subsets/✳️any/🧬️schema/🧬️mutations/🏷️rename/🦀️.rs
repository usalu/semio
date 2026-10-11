//! 🦠️ `🏷️rename` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
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
pub fn rename_generation(id: String, name: String) -> Generation2dMutation {
    Generation2dMutation::RenameGeneration(RenameGeneration { id, name })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for RenameGeneration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "generation", kind: "rename-generation", record: "RenamedGeneration" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
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
