//! 🏷️ `change-case-label-de` — changes `labelDe` (Label (DE)) on one addressed catalogue case.
use semio_s_artifact_norm_din4108_model::{Din4108Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCaseLabelDe { pub case_id: String, pub new_value: String }
impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeCaseLabelDe where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_case_label_de::ChangeCaseLabelDe> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "case-labelDe", kind: "change-case-label-de", record: "ChangedCaseLabelDe" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Change Label (DE) of case {}", self.case_id), &format!("Bezeichnung (DE) von Fall {} ändern", self.case_id)) }
    fn target(&self) -> Vec<String> { vec![self.case_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeCaseLabelDe where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_case_label_de::ChangeCaseLabelDe> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> { crate::change_case_label_de::diff::diff(self, base) }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_case_label_de::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    use crate::change_case_label_de::ChangeCaseLabelDe;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    use semio_s_plugin_norm_compliance::design_case::{CaseRowDelta, CaseRowPatch};
    pub fn diff(payload: &ChangeCaseLabelDe, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {
            return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
        };
        protocol::MutationOutcome::new(Din4108Diff { cases: CaseRowDelta::modification(&case.id, CaseRowPatch { label_de: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
    }
}
pub mod inverse {
    use crate::change_case_label_de::ChangeCaseLabelDe;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
    pub fn inverse<Op>(payload: &ChangeCaseLabelDe, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_case_label_de::ChangeCaseLabelDe> {
        Ok((|| {
        let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else { return Vec::new(); };
        vec![Op::from(ChangeCaseLabelDe { case_id: payload.case_id.clone(), new_value: item.label_de.clone() })]

        })())
    }
}
//#endregion 🪢️Mounts
