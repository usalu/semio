//! 📇️ `change-case-entry-id` — changes `entryId` (Catalogue entry) on one addressed catalogue case.
use semio_s_artifact_norm_en1998_model::{En1998Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCaseEntryId { pub case_id: String, pub new_value: String }
impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeCaseEntryId where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_case_entry_id::ChangeCaseEntryId> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "case-entryId", kind: "change-case-entry-id", record: "ChangedCaseEntryId" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Change Catalogue entry of case {}", self.case_id), &format!("Katalogeintrag von Fall {} ändern", self.case_id)) }
    fn target(&self) -> Vec<String> { vec![self.case_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeCaseEntryId where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_case_entry_id::ChangeCaseEntryId> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> { crate::change_case_entry_id::diff::diff(self, base) }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_case_entry_id::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    use crate::change_case_entry_id::ChangeCaseEntryId;
    use semio_s_artifact_norm_en1998_diff::En1998Diff;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    use semio_s_artifact_norm_en1998_model::design_case::{CaseRowDelta, CaseRowPatch};
    pub fn diff(payload: &ChangeCaseEntryId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {
            return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
        };
        protocol::MutationOutcome::new(En1998Diff { cases: CaseRowDelta::modification(&case.id, CaseRowPatch { entry_id: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
    }
}
pub mod inverse {
    use crate::change_case_entry_id::ChangeCaseEntryId;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    pub fn inverse<Op>(payload: &ChangeCaseEntryId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_entry_id::ChangeCaseEntryId> {
        Ok((|| {
        let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return Vec::new(); };
        vec![Op::from(ChangeCaseEntryId { case_id: payload.case_id.clone(), new_value: item.entry_id.clone() })]

        })())
    }
}
//#endregion 🪢️Mounts
