//! 🛡️ `change-limits` — atomically updates the untrusted-input security limits facet
//! (`max_file_bytes`/`max_records`/`max_field_length`/`max_nesting_depth` are one security policy,
//! never set one-field-at-a-time).

use semio_s_artifact_norm_vdi3805_model::{SecurityLimits, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLimits {
    pub new_limits: SecurityLimits,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for ChangeLimits where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_limits::ChangeLimits> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "limits", kind: "change-limits", record: "ChangedLimits" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        crate::change_limits::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_limits::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update security limits (maximum file size {} bytes)", self.new_limits.max_file_bytes), &format!("Sicherheitsgrenzwerte (maximale Dateigröße {} Byte) aktualisieren", self.new_limits.max_file_bytes))
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
