//! 🌱️ `create-object` — brings a new id-keyed lowpoly object into existence at a given position.

use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::LowpolyObject, snapshot::LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateObject {
    pub index: usize,
    pub object: LowpolyObject,
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for CreateObject
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::delete_object::DeleteObject>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create object \"{}\"", self.object.name), &format!("Objekt \"{}\" erstellen", self.object.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object.id.clone()]
    }
}
//#endregion 🔖️Payload
