//! `upsert-plated-panel` — upsert a `PlatedPanel` by id into `plated_panels`.

use crate::{PlatedPanel, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdatePlatedInputs {
    pub plated_panel: PlatedPanel,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdatePlatedInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "platedPanel", kind: "update-plated-inputs", record: "UpdatedPlatedPanel" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update plated panel {}", self.plated_panel.id),
            &format!("Beulfeld {} aktualisieren", self.plated_panel.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.plated_panel.id.clone()]
    }
}
//#endregion 🔖️Payload
