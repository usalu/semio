//! 🛡️ `change-limits` — atomically updates the untrusted-input security limits facet
//! (`max_file_bytes`/`max_records`/`max_field_length`/`max_nesting_depth` are one security policy,
//! never set one-field-at-a-time).

use crate::{SecurityLimits, Vdi3805Mutation, Vdi3805Snapshot};

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

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeLimits {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "limits", kind: "change-limits", record: "ChangedLimits" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update security limits (maximum file size {} bytes)", self.new_limits.max_file_bytes), &format!("Sicherheitsgrenzwerte (maximale Dateigröße {} Byte) aktualisieren", self.new_limits.max_file_bytes))
    }
}
//#endregion 🔖️Payload
