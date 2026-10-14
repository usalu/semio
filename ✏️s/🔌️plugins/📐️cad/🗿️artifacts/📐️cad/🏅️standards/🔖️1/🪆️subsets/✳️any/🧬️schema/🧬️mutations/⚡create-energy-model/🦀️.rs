//! ⚡️ `create-energy-model` — sets the cad document's `energy_model` CHILD slot (composed
//! `s.stdio.semio.model`) to a new owned handle. If the slot was already occupied, this OVERWRITES
//! it (the inverse restores whichever handle was there before, not merely "delete" — see `↩️inverse`).

use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-energy-model")]
pub struct CreateEnergyModel {
    pub child_id: String,
    /// 🪪️ Exact composed child identity.
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

impl<Op> MutationKind<CadSnapshot, Op> for CreateEnergyModel
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::create_energy_model::CreateEnergyModel> + From<crate::delete_energy_model::DeleteEnergyModel>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "energy-model", kind: "create-energy-model", record: "CreatedEnergyModel" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<semio_s_artifact_cad_cad_model_diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create energy-model child {}", self.child_id), &format!("Energiemodell-Kind {} erstellen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["energy_model".to_string()]
    }
}
//#endregion 🔖️Mutation
