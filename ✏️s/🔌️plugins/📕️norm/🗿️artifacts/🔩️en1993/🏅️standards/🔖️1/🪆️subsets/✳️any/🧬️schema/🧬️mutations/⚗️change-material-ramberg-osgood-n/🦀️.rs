//! ⚗️ `change-material-ramberg-osgood-n` mutation leaf: sets `ramberg_osgood_n` of one material.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMaterialRambergOsgoodN {
    pub id: String,
    pub new_ramberg_osgood_n: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeMaterialRambergOsgoodN where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_material_ramberg_osgood_n::ChangeMaterialRambergOsgoodN> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "material-ramberg-osgood-n",
        kind: "change-material-ramberg-osgood-n",
        record: "ChangedMaterialRambergOsgoodN",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Ramberg-Osgood exponent n of the stainless grade (EN 1993-1-4 Annex C; 0 = built-in grade table, unverified) of the material", "Werkstoff: Ramberg-Osgood-Exponent n der nichtrostenden Güte (EN 1993-1-4 Anhang C; 0 = eingebaute Gütetabelle, ungeprüft) ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeMaterialRambergOsgoodN where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_material_ramberg_osgood_n::ChangeMaterialRambergOsgoodN> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_material_ramberg_osgood_n::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_material_ramberg_osgood_n::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
