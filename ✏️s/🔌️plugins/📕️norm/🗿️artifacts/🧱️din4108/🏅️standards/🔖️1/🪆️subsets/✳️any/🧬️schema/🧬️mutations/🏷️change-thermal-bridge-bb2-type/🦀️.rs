//! 🧬️ `change-thermal-bridge-bb2-type`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeThermalBridgeBb2Type {
    pub bridge_id: String,
    pub new_bb2_type: String,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeThermalBridgeBb2Type where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "thermal-bridge-bb2-type",
        kind: "change-thermal-bridge-bb2-type",
        record: "ChangedThermalBridgeBb2Type",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Din4108Snapshot>>::Diff> {
        crate::change_thermal_bridge_bb2_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_thermal_bridge_bb2_type::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Supplement 2 thermal bridge category", "Wärmebrückenkategorie nach Beiblatt 2 ändern")
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
