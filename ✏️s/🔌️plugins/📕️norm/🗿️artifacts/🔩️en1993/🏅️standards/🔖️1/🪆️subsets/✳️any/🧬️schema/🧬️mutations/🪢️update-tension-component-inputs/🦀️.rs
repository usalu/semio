//! 🧬️ `upsert-tension-component` — upsert a `TensionComponent` by id into `tension_components`.

use semio_s_artifact_norm_en1993_model::{TensionComponent, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateTensionComponentInputs {
    pub tension_component: TensionComponent,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateTensionComponentInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_tension_component_inputs::UpdateTensionComponentInputs> + From<crate::remove_tension_component::RemoveTensionComponent> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "tensionComponent", kind: "update-tension-component-inputs", record: "UpdatedTensionComponent" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1993Snapshot>>::Diff> {
        crate::update_tension_component_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_tension_component_inputs::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update tension component {}", self.tension_component.id),
            &format!("Zugglied {} aktualisieren", self.tension_component.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.tension_component.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
