//! 🧱 `change-emb-wall-delta-passive-deg` mutation leaf: sets `delta_passive_deg` of one embedded wall.

use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeEmbWallDeltaPassiveDeg {
    pub id: String,
    pub new_delta_passive_deg: f64,
}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for ChangeEmbWallDeltaPassiveDeg where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "embedded-wall-delta-passive-deg",
        kind: "change-emb-wall-delta-passive-deg",
        record: "ChangedEmbWallDeltaPassiveDeg",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Passive wall friction δ_p of the embedded wall", "Eingespannte Wand: Wandreibung passiv δ_p ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for ChangeEmbWallDeltaPassiveDeg where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::change_emb_wall_delta_passive_deg::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_emb_wall_delta_passive_deg::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
