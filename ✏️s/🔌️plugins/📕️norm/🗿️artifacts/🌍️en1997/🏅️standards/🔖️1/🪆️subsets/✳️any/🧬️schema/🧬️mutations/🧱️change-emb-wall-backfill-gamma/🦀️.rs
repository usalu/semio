//! 🧱 `change-emb-wall-backfill-gamma` mutation leaf: sets `backfill_gamma` of one embedded wall.

use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeEmbWallBackfillGamma {
    pub id: String,
    pub new_backfill_gamma: f64,
}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for ChangeEmbWallBackfillGamma where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "embedded-wall-backfill-gamma",
        kind: "change-emb-wall-backfill-gamma",
        record: "ChangedEmbWallBackfillGamma",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Soil unit weight γ of the embedded wall", "Eingespannte Wand: Bodenwichte γ ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for ChangeEmbWallBackfillGamma where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::change_emb_wall_backfill_gamma::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_emb_wall_backfill_gamma::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
