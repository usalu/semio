//! 🧬️ `upsert-crane-runway` — upsert a `CraneRunway` by id into `crane_runways`.

use semio_s_artifact_norm_en1993_model::{CraneRunway, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateCraneInputs {
    pub crane_runway: CraneRunway,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateCraneInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_crane_inputs::UpdateCraneInputs> + From<crate::remove_crane_runway::RemoveCraneRunway> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "craneRunway", kind: "update-crane-inputs", record: "UpdatedCraneRunway" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1993Snapshot>>::Diff> {
        crate::update_crane_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_crane_inputs::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update crane runway {}", self.crane_runway.id),
            &format!("Kranbahn {} aktualisieren", self.crane_runway.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.crane_runway.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
