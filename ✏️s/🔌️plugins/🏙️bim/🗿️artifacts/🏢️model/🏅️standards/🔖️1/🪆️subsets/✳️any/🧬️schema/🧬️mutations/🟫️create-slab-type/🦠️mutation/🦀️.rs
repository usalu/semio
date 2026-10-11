//! 🟫️ `create-slab-type` payload. Brings a new layered slab type into the library; every layer names an existing material and has a positive thickness.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, SlabType};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSlabType {
    pub id: String,
    pub slab_type: SlabType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSlabType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slab-type", kind: "create-slab-type", record: "CreateSlabType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create slab type \"{}\"", self.slab_type.name), &format!("Deckentyp \"{}\" anlegen", self.slab_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
