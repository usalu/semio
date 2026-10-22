//! 📐️ `scale-object` — sets an object's absolute scale.

use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ScaleObject {
    pub id: String,
    pub new_scale: [f32; 3],
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for ScaleObject
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::scale_object::ScaleObject>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "object", kind: "scale-object", record: "ScaledObject" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale object \"{}\"", self.id), &format!("Objekt \"{}\" skalieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LowpolySnapshot, Op> for ScaleObject
where
    Op: protocol::ColdMutation<LowpolySnapshot, ReferenceDiff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::scale_object::ScaleObject>,
{
    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<LowpolySnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
