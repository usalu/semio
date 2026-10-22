//! ➕️ `insert-point` — places a new point into the geometry playground's anonymous, index-keyed
//! point cloud. `index` is FINAL-state, per the addressing convention for index-keyed collections.

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertPoint {
    pub index: usize,
    pub x: f64,
    pub y: f64,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for InsertPoint
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<InsertPoint> + From<crate::standards::v1::subsets::geometry::schema::mutations::remove_point::RemovePoint>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "point", kind: "insert-point", record: "InsertedPoint" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert point at {}", self.index), &format!("Punkt an {} einfügen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationSnapshot, Op> for InsertPoint
where
    Op: protocol::ColdMutation<EquationSnapshot, ReferenceDiff = crate::EquationDiff> + From<InsertPoint> + From<crate::standards::v1::subsets::geometry::schema::mutations::remove_point::RemovePoint>,
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
