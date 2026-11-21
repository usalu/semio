//! 🗂️ `change-usage`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeUsage {
    pub new_usage: String,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeUsage where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_usage::ChangeUsage> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "usage",
        kind: "change-usage",
        record: "ChangedUsage",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change building usage", "Gebäudenutzung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeUsage where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_usage::ChangeUsage> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_usage::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_usage::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-usage` diff.

    use crate::change_usage::ChangeUsage;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeUsage, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if base.usage == payload.new_usage {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "usage already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { usage: Some(payload.new_usage.clone()), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-usage` inverse.

    use crate::change_usage::ChangeUsage;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeUsage, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_usage::ChangeUsage> {
        Ok((|| {
        vec![Op::from(ChangeUsage { new_usage: base.usage.clone() })]

        })())
    }
}
//#endregion 🪢️Mounts
