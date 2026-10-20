//! 🏷️ `change-node-label` — sets a node's display label (`id` stays the stable identity field, so
//! this is `change`, not `rename`).

use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeNodeLabel {
    pub id: String,
    pub new_label: String,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for ChangeNodeLabel
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<ChangeNodeLabel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node", kind: "change-node-label", record: "ChangedNodeLabel" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Relabel node \"{}\" to \"{}\"", self.id, self.new_label), &format!("Beschriftung von Knoten \"{}\" in \"{}\" ändern", self.id, self.new_label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
