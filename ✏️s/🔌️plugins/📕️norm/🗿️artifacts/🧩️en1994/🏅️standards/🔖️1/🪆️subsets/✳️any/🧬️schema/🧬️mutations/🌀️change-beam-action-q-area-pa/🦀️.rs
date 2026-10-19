//! 🧬️ `change-beam-action-q-area-pa` mutation leaf.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBeamActionQAreaPa {
    pub index: usize,
    pub action_index: usize,
    pub new_q_area_pa: f64,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for ChangeBeamActionQAreaPa where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_action_q_area_pa::ChangeBeamActionQAreaPa> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beam-action-q-area-pa", kind: "change-beam-action-q-area-pa", record: "ChangedBeamActionQAreaPa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1994Snapshot>>::Diff> {
        crate::change_beam_action_q_area_pa::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_beam_action_q_area_pa::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change imposed area load on the beam", "Flächenlast auf dem Träger ändern")
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
