//! ➖️ `remove-point` — takes a point out of the geometry playground's point cloud. `index` is
//! BASE-state, per the addressing convention for index-keyed collections.

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePoint {
    pub index: usize,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for RemovePoint
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<RemovePoint> + From<crate::standards::v1::subsets::geometry::schema::mutations::insert_point::InsertPoint>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "point", kind: "remove-point", record: "RemovedPoint" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove point at {}", self.index), &format!("Punkt an {} entfernen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationSnapshot, Op> for RemovePoint
where
    Op: protocol::ColdMutation<EquationSnapshot, ReferenceDiff = crate::EquationDiff> + From<RemovePoint> + From<crate::standards::v1::subsets::geometry::schema::mutations::insert_point::InsertPoint>,
{
    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<EquationSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
