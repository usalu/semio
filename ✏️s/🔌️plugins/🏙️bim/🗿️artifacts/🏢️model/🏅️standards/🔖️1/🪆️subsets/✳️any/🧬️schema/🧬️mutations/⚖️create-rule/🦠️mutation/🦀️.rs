//! ⚖️ `create-rule` payload. Brings a new numeric code check into the model: the measure and direction (minimum clear height, maximum riser, minimum tread, minimum stair width, minimum door width, maximum ramp slope, minimum corridor width, maximum compartment area), the limit, the severity and the scope. Its findings are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Rule};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRule {
    pub id: String,
    pub rule: Rule,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreateRule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create rule \"{}\"", self.rule.name), &format!("Regel \"{}\" anlegen", self.rule.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
