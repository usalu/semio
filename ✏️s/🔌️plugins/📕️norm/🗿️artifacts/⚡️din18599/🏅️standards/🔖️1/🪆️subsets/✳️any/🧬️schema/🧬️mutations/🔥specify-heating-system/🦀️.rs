//! 🧬️ `specify-heating-system` mutation leaf.

use semio_s_artifact_norm_din18599_diff::Din18599Diff;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SpecifyHeatingSystem {
    pub new_heating: semio_s_artifact_norm_din18599_model::HeatingSystem,
}

impl<Op> protocol::MutationKind<Din18599Snapshot, Op> for SpecifyHeatingSystem where Op: protocol::Mutation<Din18599Snapshot, Diff = semio_s_artifact_norm_din18599_diff::Din18599Diff> + From<crate::specify_heating_system::SpecifyHeatingSystem> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "set",
        entity: "heating-system",
        kind: "specify-heating-system",
        record: "SetHeatingSystem",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Specify heating system", "Heizungssystem festlegen")
    }
}

impl<Op> protocol::ColdMutationKind<Din18599Snapshot, Op> for SpecifyHeatingSystem where Op: protocol::ColdMutation<Din18599Snapshot, ReferenceDiff = semio_s_artifact_norm_din18599_diff::Din18599Diff> + From<crate::specify_heating_system::SpecifyHeatingSystem> {
    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        crate::specify_heating_system::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::specify_heating_system::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
