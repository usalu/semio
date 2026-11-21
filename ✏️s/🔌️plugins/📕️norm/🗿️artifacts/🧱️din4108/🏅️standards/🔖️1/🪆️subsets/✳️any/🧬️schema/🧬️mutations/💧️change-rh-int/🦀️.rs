//! 💧️ `change-rh-int`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeRhInt {
    pub new_rh_int: f64,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeRhInt where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_rh_int::ChangeRhInt> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "rh-int",
        kind: "change-rh-int",
        record: "ChangedRhInt",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change indoor relative humidity", "Relative Raumluftfeuchte ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeRhInt where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_rh_int::ChangeRhInt> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_rh_int::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_rh_int::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-rh-int` diff — sets the indoor relative humidity φ_i as a fraction, refused as a `mutation.invariant` outside the bound its leaf payload schema states.

    use crate::change_rh_int::ChangeRhInt;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeRhInt, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if !((0.0..=1.0).contains(&payload.new_rh_int)) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "rh_int must be a relative humidity within [0, 1].", Vec::<String>::new());
        }
        if base.rh_int == payload.new_rh_int {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "rh_int already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { rh_int: Some(payload.new_rh_int), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-rh-int` inverse.

    use crate::change_rh_int::ChangeRhInt;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeRhInt, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rh_int::ChangeRhInt> {
        Ok((|| {
        vec![Op::from(ChangeRhInt { new_rh_int: base.rh_int })]

        })())
    }
}
//#endregion 🪢️Mounts
