//! ➕️ Fem2d mutation — `AddLoad` payload + `MutationKind` impl.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::remove_load;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ➕️ Attaches a [`FemLoad`] to an existing load case's `loads` member collection.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-load")]
pub struct AddLoad {
    pub case_id: String,
    #[dsl(statements)]
    pub load: Box<FemLoad>,
    /// 📍 Zero-based insertion position among the case's loads; `None` or past the end appends.
    pub index: Option<usize>,
}

impl<Op> MutationKind<Fem2dSnapshot, Op> for AddLoad
where
    Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<remove_load::RemoveLoad>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "load", kind: "add-load", record: "AddedLoad" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add load \"{}\" to case \"{}\"", load_id(&self.load), self.case_id), &format!("Last \"{}\" zu Lastfall \"{}\" hinzufügen", load_id(&self.load), self.case_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
