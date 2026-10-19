//! 🧬️ `upsert-section` — upsert a `SteelSection` by id into `sections`.

use semio_s_artifact_norm_en1993_model::{SteelSection, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateThroughThicknessInputs {
    pub section: SteelSection,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateThroughThicknessInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_through_thickness_inputs::UpdateThroughThicknessInputs> + From<crate::remove_section::RemoveSection> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "section", kind: "update-through-thickness-inputs", record: "UpdatedSection" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1993Snapshot>>::Diff> {
        crate::update_through_thickness_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_through_thickness_inputs::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update section {}", self.section.id),
            &format!("Querschnitt {} aktualisieren", self.section.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.section.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
