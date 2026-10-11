//! `change-layer-compressive-class`.

use crate::{Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerCompressiveClass {
    pub element_id: String,
    pub index: usize,
    pub new_compressive_class: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeLayerCompressiveClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "layer-compressive-class",
        kind: "change-layer-compressive-class",
        record: "ChangedLayerCompressiveClass",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change compressive strength class", "Druckbelastbarkeitsklasse ändern")
    }
}
