//! 📐️ `change-storey-stiffness-x` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeStoreyStiffnessX {
    pub building_index: usize,
    pub storey_index: usize,
    pub new_stiffness_x: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeStoreyStiffnessX {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-stiffness-x",
        kind: "change-storey-stiffness-x",
        record: "ChangeStoreyStiffnessX",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change storey stiffness", "Geschosssteifigkeit ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-storey-stiffness-x".into()]
    }
}
