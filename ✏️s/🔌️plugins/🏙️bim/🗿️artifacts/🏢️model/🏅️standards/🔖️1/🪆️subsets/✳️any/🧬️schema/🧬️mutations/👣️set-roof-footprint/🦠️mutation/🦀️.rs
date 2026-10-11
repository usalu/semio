//! 👣️ `set-roof-footprint` payload. Replaces the footprint loop of a roof; the shape, overhang and solid follow by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Vertex};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRoofFootprint {
    pub id: String,
    pub footprint: Vec<Vertex>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetRoofFootprint {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "roof", kind: "set-roof-footprint", record: "SetRoofFootprint" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reshape the footprint of roof \"{}\"", self.id), &format!("Grundriss des Dachs \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
