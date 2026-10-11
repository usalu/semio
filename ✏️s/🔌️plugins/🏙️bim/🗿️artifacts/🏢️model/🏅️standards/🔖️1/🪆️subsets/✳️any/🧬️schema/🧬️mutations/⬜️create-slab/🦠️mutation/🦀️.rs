//! ⬜️ `create-slab` payload. Brings a new slab onto a storey; its boundary and holes are closed counter-clockwise loops, area and solid are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Slab};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSlab {
    pub id: String,
    pub slab: Slab,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSlab {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slab", kind: "create-slab", record: "CreateSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create slab \"{}\"", self.slab.name), &format!("Decke \"{}\" anlegen", self.slab.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
