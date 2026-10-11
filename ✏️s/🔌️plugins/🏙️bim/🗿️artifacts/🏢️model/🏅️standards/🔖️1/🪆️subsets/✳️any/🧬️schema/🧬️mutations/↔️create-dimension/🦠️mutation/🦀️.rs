//! ↔️ `create-dimension` payload. Brings a new dimension onto a storey plan: two or more anchors (free points or elements of the model) measured along a direction; its values and text are inferred from the geometry of the anchors.

use crate::{Dimension, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateDimension {
    pub id: String,
    pub dimension: Dimension,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateDimension {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "dimension", kind: "create-dimension", record: "CreateDimension" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create dimension \"{}\"", self.dimension.name), &format!("Bemaßung \"{}\" anlegen", self.dimension.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
