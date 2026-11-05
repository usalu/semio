//! ➕️ insert-hollow-joint
use semio_s_artifact_norm_en1993_model::{HollowJoint, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertHollowJoint { pub index: Option<usize>, pub hollow_joint: HollowJoint }
impl<Op> protocol::MutationKind<En1993Snapshot, Op> for InsertHollowJoint where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_hollow_joint::RemoveHollowJoint> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "hollow-joint", kind: "insert-hollow-joint", record: "InsertedHollowJoint" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert hollow joint at position #{}", self.index.map_or_else(|| "Ende".to_string(), |index| index.to_string())), &format!("Hohlprofilknoten an Position #{} einfügen", self.index.map_or_else(|| "end".to_string(), |index| index.to_string()))) }
    fn target(&self) -> Vec<String> { vec![self.index.map_or_else(|| "end".to_string(), |index| index.to_string())] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for InsertHollowJoint where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_hollow_joint::RemoveHollowJoint> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::insert_hollow_joint::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_hollow_joint::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
