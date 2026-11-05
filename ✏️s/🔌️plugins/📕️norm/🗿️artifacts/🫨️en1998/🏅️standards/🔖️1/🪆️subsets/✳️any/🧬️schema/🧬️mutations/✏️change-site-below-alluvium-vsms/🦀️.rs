//! ✏️ `change-site-below-alluvium-vsms` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSiteBelowAlluviumVSMS {
    pub new_below_alluvium_v_s_m_s: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeSiteBelowAlluviumVSMS where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_site_below_alluvium_v_s_m_s::ChangeSiteBelowAlluviumVSMS> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "site-below-alluvium-v-s-m-s",
        kind: "change-site-below-alluvium-vsms",
        record: "ChangeSiteBelowAlluviumVSMS",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Shear wave velocity of the substratum below the alluvium (E indicator)", "Scherwellengeschwindigkeit des Untergrunds unter der Alluvionsschicht (E-Kennzeichen) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-site-below-alluvium-vsms".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeSiteBelowAlluviumVSMS where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_site_below_alluvium_v_s_m_s::ChangeSiteBelowAlluviumVSMS> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_site_below_alluvium_v_s_m_s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_site_below_alluvium_v_s_m_s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
