//! 🧬️ `change-layer-application-type`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerApplicationType {
    pub element_id: String,
    pub index: usize,
    pub new_application_type: String,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeLayerApplicationType where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_layer_application_type::ChangeLayerApplicationType> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "layer-application-type",
        kind: "change-layer-application-type",
        record: "ChangedLayerApplicationType",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change insulation application type", "Anwendungstyp der Wärmedämmung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeLayerApplicationType where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_layer_application_type::ChangeLayerApplicationType> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_layer_application_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_layer_application_type::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
