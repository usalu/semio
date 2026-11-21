//! 🔤️ `change-case-text` — sets, adds or (with an empty value) removes one text input of a design case; the diff and inverse are the shared `design_case` logic.
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCaseText { pub case_id: String, pub key: String, pub value: Option<String> }
impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeCaseText where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_case_text::ChangeCaseText> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "case-text", kind: "change-case-text", record: "ChangeCaseText" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Set text input {1} of case {0}", self.case_id, self.key), &format!("Texteingabe {1} von Fall {0} setzen", self.case_id, self.key)) }
    fn target(&self) -> Vec<String> { vec![self.case_id.clone(), self.key.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeCaseText where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_case_text::ChangeCaseText> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> { crate::change_case_text::diff::diff(self, base) }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_case_text::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    use crate::change_case_text::ChangeCaseText;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn diff(payload: &ChangeCaseText, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        match semio_s_plugin_norm_compliance::design_case::change_case_text(&base.cases, &payload.case_id, &payload.key, payload.value.as_deref()) {
            Ok(cases) => protocol::MutationOutcome::new(Din4108Diff { cases, ..Default::default() }),
            Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
        }
    }
}
pub mod inverse {
    use crate::change_case_text::ChangeCaseText;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn inverse<Op>(payload: &ChangeCaseText, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_text::ChangeCaseText> {
        Ok((|| {
        if !base.cases.iter().any(|item| item.id == payload.case_id) { return Vec::new(); }
        vec![Op::from(ChangeCaseText { case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::previous_text(&base.cases, &payload.case_id, &payload.key) })]

        })())
    }
}
//#endregion 🪢️Mounts
