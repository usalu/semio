//! 💪️ `change-system-v-rd-n` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSystemVRdN {
    pub building_index: usize,
    pub system_index: usize,
    pub new_base_shear_resistance_n: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSystemVRdN {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "system-v-rd-n",
        kind: "change-system-v-rd-n",
        record: "ChangeSystemVRdN",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change shear resistance V_Rd of the system", "Querkraftwiderstand V_Rd des Systems ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-system-v-rd-n".into()]
    }
}
