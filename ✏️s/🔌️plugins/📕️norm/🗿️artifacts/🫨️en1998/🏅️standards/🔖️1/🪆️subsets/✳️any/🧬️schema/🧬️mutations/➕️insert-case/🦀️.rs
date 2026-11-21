//! ➕️ `insert-case` — inserts one catalogue case into the case table.
use semio_s_artifact_norm_en1998_model::{CaseRow, En1998Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertCase { pub index: usize, pub case: CaseRow }
impl<Op> protocol::MutationKind<En1998Snapshot, Op> for InsertCase where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::remove_case::RemoveCase> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "case", kind: "insert-case", record: "InsertedCase" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert catalogue case {} at #{}", self.case.id, self.index), &format!("Katalogfall {} an #{} einfügen", self.case.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.case.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for InsertCase where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::remove_case::RemoveCase> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> { crate::insert_case::diff::diff(self, base) }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_case::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    use crate::insert_case::InsertCase;
    use semio_s_artifact_norm_en1998_diff::En1998Diff;
    use semio_s_artifact_norm_en1998_schema_formulas::case_catalogue::En1998Cases;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    pub fn diff(payload: &InsertCase, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        match semio_s_artifact_norm_en1998_model::design_case::add_case::<En1998Cases>(&base.cases, payload.index, &payload.case) {
            Ok(cases) => protocol::MutationOutcome::new(En1998Diff { cases, ..Default::default() }),
            Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
        }
    }
}
pub mod inverse {
    use crate::insert_case::InsertCase;
    use crate::remove_case;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    pub fn inverse<Op>(payload: &InsertCase, _base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_case::RemoveCase> {
        Ok((|| {
        vec![Op::from(remove_case::RemoveCase { id: payload.case.id.clone() })]

        })())
    }
}
//#endregion 🪢️Mounts
