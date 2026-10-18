//! ⚖️ `create-rule` payload. Brings a new numeric code check into the model: the measure and direction (minimum clear height, maximum riser, minimum tread, minimum stair width, minimum door width, maximum ramp slope, minimum corridor width, maximum compartment area), the limit, the severity and the scope. Its findings are inferred.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Rule}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRule {
    pub id: String,
    pub rule: Rule,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateRule
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreateRule" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_rule::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_rule::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create rule \"{}\"", self.rule.name), &format!("Regel \"{}\" anlegen", self.rule.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
