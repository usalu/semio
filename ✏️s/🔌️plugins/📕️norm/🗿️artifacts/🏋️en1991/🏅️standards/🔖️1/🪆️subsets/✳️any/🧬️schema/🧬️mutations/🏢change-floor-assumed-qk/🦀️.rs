//! 🧬️ `change-floor-assumed-qk`.

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFloorAssumedQk {
    pub index: usize,
    pub new_assumed_qk: f64,
}

impl<Op> protocol::MutationKind<En1991Snapshot, Op> for ChangeFloorAssumedQk where Op: protocol::Mutation<En1991Snapshot, Diff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::change_floor_assumed_qk::ChangeFloorAssumedQk> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "floor-assumed-qk",
        kind: "change-floor-assumed-qk",
        record: "ChangedFloorAssumedQk",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1991Snapshot>>::Diff> {
        crate::change_floor_assumed_qk::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_floor_assumed_qk::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change assumed imposed floor load", "Angenommene Nutzlast der Decke ändern")
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
