//! 🗽️ `set-column-tilt` payload. Sets the lean of a column about its base point: the top leans towards the direction by the angle from the vertical, at most 60 degrees; leaving the tilt out makes the column plumb. The tilted solid, its volume and joins follow by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Slope};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetColumnTilt {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tilt: Option<Slope>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetColumnTilt {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "column", kind: "set-column-tilt", record: "SetColumnTilt" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the tilt of column \"{}\"", self.id), &format!("Neigung von Stütze \"{}\" setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
