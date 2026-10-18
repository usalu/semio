//! 🥖️ `split-beam` payload. Splits a beam at the fraction `t` of its length into the original beam, which keeps the first part, and a new beam that continues it with the same type, top offset, phase and name.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SplitBeam {
    pub id: String,
    pub t: f64,
    pub new_id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SplitBeam
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SplitBeam> + From<crate::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam> + From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements> + From<crate::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "split", entity: "beam", kind: "split-beam", record: "SplitBeam" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::split_beam::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::split_beam::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Split beam \"{}\" at {}", self.id, self.t), &format!("Träger \"{}\" bei {} teilen", self.id, self.t))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
