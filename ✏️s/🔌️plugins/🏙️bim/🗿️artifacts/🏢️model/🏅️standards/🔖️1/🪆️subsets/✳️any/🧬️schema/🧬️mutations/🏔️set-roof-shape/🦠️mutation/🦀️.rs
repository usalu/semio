//! 🏔️ `set-roof-shape` payload. Sets exactly the provided fields of a roof's form: shape, overhang and base offset; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, RoofShape};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRoofShape {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<RoofShape>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overhang: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetRoofShape {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "roof", kind: "set-roof-shape", record: "SetRoofShape" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change the form of roof \"{}\"", self.id), &format!("Form des Dachs \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
