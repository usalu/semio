//! 🚛 `change-assumed-bridge-lm3`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAssumedBridgeLm3 {
    pub new_assumed_bridge_lm3: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeAssumedBridgeLm3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "assumed-bridge-lm3",
        kind: "change-assumed-bridge-lm3",
        record: "ChangedAssumedBridgeLm3",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change assumed load model 3 special vehicle load", "Angenommene Sonderfahrzeuglast des Lastmodells 3 ändern")
    }
}
//#endregion 🔖️Payload
