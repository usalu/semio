//! 💨️ `change-airtightness-n50`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAirtightnessN50 {
    pub new_airtightness_n50: f64,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeAirtightnessN50 where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_airtightness_n50::ChangeAirtightnessN50> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "airtightness-n50",
        kind: "change-airtightness-n50",
        record: "ChangedAirtightnessN50",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change air change rate n50", "Luftwechselrate n50 ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeAirtightnessN50 where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_airtightness_n50::ChangeAirtightnessN50> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_airtightness_n50::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_airtightness_n50::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-airtightness-n50` diff — sets the air change rate at 50 Pa n₅₀ in 1/h, refused as a `mutation.invariant` outside the bound its leaf payload schema states.

    use crate::change_airtightness_n50::ChangeAirtightnessN50;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if !(payload.new_airtightness_n50.is_finite() && payload.new_airtightness_n50 >= 0.0) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "airtightness_n50 must be a non-negative finite number.", Vec::<String>::new());
        }
        if base.airtightness_n50 == payload.new_airtightness_n50 {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "airtightness_n50 already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { airtightness_n50: Some(payload.new_airtightness_n50), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-airtightness-n50` inverse.

    use crate::change_airtightness_n50::ChangeAirtightnessN50;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_airtightness_n50::ChangeAirtightnessN50> {
        Ok((|| {
        vec![Op::from(ChangeAirtightnessN50 { new_airtightness_n50: base.airtightness_n50 })]

        })())
    }
}
//#endregion 🪢️Mounts
