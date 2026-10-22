//! 💣️ `delete-elements` payload. Removes any set of elements together with everything that depends on them (buildings of a site, storeys and grid lines of a building, the contents of a storey, openings of walls and curtain walls) and with their properties and classifications; refuses while a surviving element's top constraint still points at a removed storey.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteElements {
    pub ids: Vec<String>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteElements> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "elements", kind: "delete-elements", record: "DeleteElements" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete {} element(s) with their dependants", self.ids.len()), &format!("{} Element(e) samt Abhängigen löschen", self.ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteElements
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteElements> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_elements::inverse::inverse(self, base))
    }
}
