//! 🔧️ `change-script-limits` — atomically updates the part-number script execution limits facet
//! (`max_steps`/`max_recursion`/`timeout_ms` are validated together, never one-field-at-a-time).

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeScriptLimits {
    pub new_max_steps: u32,
    pub new_max_recursion: u32,
    pub new_timeout_ms: u64,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for ChangeScriptLimits where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_script_limits::mutation::ChangeScriptLimits> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "script-limits", kind: "change-script-limits", record: "ChangedScriptLimits" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update script limits (max-steps={}, max-recursion={}, timeout-ms={})", self.new_max_steps, self.new_max_recursion, self.new_timeout_ms), &format!("Skriptgrenzwerte (Max-Schritte={}, Max-Rekursion={}, Timeout-ms={}) aktualisieren", self.new_max_steps, self.new_max_recursion, self.new_timeout_ms))
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for ChangeScriptLimits where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_script_limits::mutation::ChangeScriptLimits> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::change_script_limits::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_script_limits::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
