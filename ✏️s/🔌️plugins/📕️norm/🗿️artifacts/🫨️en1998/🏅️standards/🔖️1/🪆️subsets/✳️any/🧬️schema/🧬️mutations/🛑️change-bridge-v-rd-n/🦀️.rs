//! 🛑️ `change-bridge-v-rd-n` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBridgeVRdN {
    pub index: usize,
    pub new_v_rd_n: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeBridgeVRdN {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "bridge-v-rd-n",
        kind: "change-bridge-v-rd-n",
        record: "ChangeBridgeVRdN",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change design shear resistance of the bridge", "Bemessungswert der Querkrafttragfähigkeit der Brücke ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-bridge-v-rd-n".into()]
    }
}
