//! 💠️ `array-elements` payload. Repeats placed elements in a linear array (each copy a further spacing) or a radial array (each copy a further step about a centre), as one set of created records with ids minted from the prefix, the copy number and the position of the source; every copy is computed from the source, so no rounding accumulates.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ArrayElements {
    pub ids: Vec<String>,
    pub prefix: String,
    pub pattern: crate::standards::v1::subsets::any::schema::mutations::modify::ArrayPattern,
}

impl<Op> MutationKind<ModelSnapshot, Op> for ArrayElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<ArrayElements> + From<crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "elements", kind: "array-elements", record: "ArrayElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::array_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::array_elements::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Array {} element(s) {} times", self.ids.len(), self.pattern.count()), &format!("{} Element(e) {}-mal anordnen", self.ids.len(), self.pattern.count()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
