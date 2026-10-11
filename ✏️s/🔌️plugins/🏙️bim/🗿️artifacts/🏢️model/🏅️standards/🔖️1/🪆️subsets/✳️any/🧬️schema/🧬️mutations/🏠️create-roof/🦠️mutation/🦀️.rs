//! 🏠️ `create-roof` payload. Brings a new roof onto a storey; footprint is a closed counter-clockwise loop, pitched shapes need pitches in (0, 89 degrees); the solid is inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Roof};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRoof {
    pub id: String,
    pub roof: Roof,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateRoof {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "roof", kind: "create-roof", record: "CreateRoof" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create roof \"{}\"", self.roof.name), &format!("Dach \"{}\" anlegen", self.roof.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
