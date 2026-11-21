//! ➖️ `remove-case` — removes the catalogue case by id.
use semio_s_artifact_norm_din4108_model::{Din4108Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveCase { pub id: String }
impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for RemoveCase where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::insert_case::InsertCase> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "case", kind: "remove-case", record: "RemovedCase" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Remove catalogue case {}", self.id), &format!("Katalogfall {} entfernen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for RemoveCase where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::insert_case::InsertCase> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> { crate::remove_case::diff::diff(self, base) }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::remove_case::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    use crate::remove_case::RemoveCase;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn diff(payload: &RemoveCase, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        match semio_s_plugin_norm_compliance::design_case::remove_case(&base.cases, &payload.id) {
            Ok(cases) => protocol::MutationOutcome::new(Din4108Diff { cases, ..Default::default() }),
            Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
        }
    }
}
pub mod inverse {
    use crate::remove_case::RemoveCase;
    use crate::insert_case;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn inverse<Op>(payload: &RemoveCase, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_case::InsertCase> {
        Ok((|| {
        let Some((index, case)) = base.cases.iter().enumerate().find(|(_, case)| case.id == payload.id) else { return Vec::new(); };
        vec![Op::from(insert_case::InsertCase { index, case: case.clone() })]

        })())
    }
}
//#endregion 🪢️Mounts
