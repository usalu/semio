//! 🪆️ `set-family` payload. Sets exactly the provided fields of a family, its name and its category; a profile family cannot leave its category while a type uses it as a profile.

use crate::standards::v1::subsets::any::schema::{snapshot::{FamilyCategory, ModelSnapshot}, diff::{patches::FamilyPatch, ModelDiff}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFamily {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<FamilyCategory>,
}

impl SetFamily {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> FamilyPatch {
        FamilyPatch { name: self.name.clone(), category: self.category, ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: FamilyPatch) -> Self {
        Self { id, name: patch.name, category: patch.category }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetFamily
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetFamily>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "family", kind: "set-family", record: "SetFamily" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_family::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_family::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change family \"{}\"", self.id), &format!("Familie \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
