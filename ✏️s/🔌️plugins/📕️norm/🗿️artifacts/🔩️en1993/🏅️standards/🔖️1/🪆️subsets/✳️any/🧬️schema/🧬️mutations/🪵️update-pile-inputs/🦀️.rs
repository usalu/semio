//! 🧬️ `upsert-pile` — upsert a `SteelPile` by id into `piles`.

use semio_s_artifact_norm_en1993_model::{SteelPile, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdatePileInputs {
    pub pile: SteelPile,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdatePileInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_pile_inputs::UpdatePileInputs> + From<crate::remove_pile::RemovePile> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "pile", kind: "update-pile-inputs", record: "UpdatedPile" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1993Snapshot>>::Diff> {
        crate::update_pile_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_pile_inputs::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update pile {}", self.pile.id),
            &format!("Pfahl {} aktualisieren", self.pile.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.pile.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
