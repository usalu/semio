//! ➕️ `insert-case` — inserts one catalogue case into the case table.
use semio_s_artifact_norm_din4108_model::{CaseRow, Din4108Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertCase { pub index: usize, pub case: CaseRow }
impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for InsertCase where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::remove_case::RemoveCase> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "case", kind: "insert-case", record: "InsertedCase" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert catalogue case {} at #{}", self.case.id, self.index), &format!("Katalogfall {} an #{} einfügen", self.case.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.case.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for InsertCase where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::remove_case::RemoveCase> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> { crate::insert_case::diff::diff(self, base) }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_case::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    use crate::insert_case::InsertCase;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_checks::design_cases::Din4108Cases;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn diff(payload: &InsertCase, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        match semio_s_plugin_norm_compliance::design_case::add_case::<Din4108Cases>(&base.cases, payload.index, &payload.case) {
            Ok(cases) => protocol::MutationOutcome::new(Din4108Diff { cases, ..Default::default() }),
            Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
        }
    }
}
pub mod inverse {
    use crate::insert_case::InsertCase;
    use crate::remove_case;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn inverse<Op>(payload: &InsertCase, _base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_case::RemoveCase> {
        Ok((|| {
        vec![Op::from(remove_case::RemoveCase { id: payload.case.id.clone() })]

        })())
    }
}
//#endregion 🪢️Mounts
