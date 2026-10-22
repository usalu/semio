//! 🧱 `change-assumed-silo-wall-friction`.

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAssumedSiloWallFriction {
    pub new_assumed_silo_wall_friction: f64,
}

impl<Op> protocol::MutationKind<En1991Snapshot, Op> for ChangeAssumedSiloWallFriction where Op: protocol::Mutation<En1991Snapshot, Diff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "assumed-silo-wall-friction",
        kind: "change-assumed-silo-wall-friction",
        record: "ChangedAssumedSiloWallFriction",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change assumed silo wall friction", "Angenommene Silo-Wandreibung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1991Snapshot, Op> for ChangeAssumedSiloWallFriction where Op: protocol::ColdMutation<En1991Snapshot, ReferenceDiff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction> {
    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1991Snapshot>>::ReferenceDiff> {
        crate::change_assumed_silo_wall_friction::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_assumed_silo_wall_friction::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
