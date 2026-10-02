//! 🛡 `change-wind-face-assumed-wp`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeWindFaceAssumedWp {
    pub index: usize,
    pub new_assumed_wp: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeWindFaceAssumedWp {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "wind-face-assumed-wp",
        kind: "change-wind-face-assumed-wp",
        record: "ChangedWindFaceAssumedWp",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change assumed wind pressure on the face", "Angenommenen Winddruck auf die Fläche ändern")
    }
}
//#endregion 🔖️Payload
