//! 🛏️ `create-component` payload. Places an instance of a family: a storey, a family, a plan position with an elevation above the storey, a rotation, a mirror flag, an optional host wall and an optional system for a terminal.

use crate::{Component, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateComponent {
    pub id: String,
    pub component: Component,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateComponent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "component", kind: "create-component", record: "CreateComponent" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Place component \"{}\"", self.component.name), &format!("Komponente \"{}\" platzieren", self.component.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
