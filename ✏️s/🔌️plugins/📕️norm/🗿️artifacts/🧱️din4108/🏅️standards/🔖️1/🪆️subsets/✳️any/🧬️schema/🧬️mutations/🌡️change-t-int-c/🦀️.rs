//! 🌡️ `change-t-int-c`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTIntC {
    pub new_t_int_c: f64,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeTIntC where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_t_int_c::ChangeTIntC> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "t-int-c",
        kind: "change-t-int-c",
        record: "ChangedTIntC",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change indoor air temperature", "Raumlufttemperatur ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeTIntC where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_t_int_c::ChangeTIntC> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_t_int_c::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_t_int_c::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-t-int-c` diff — sets the indoor air temperature θ_i in °C, refused as a `mutation.invariant` outside the bound its leaf payload schema states.

    use crate::change_t_int_c::ChangeTIntC;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeTIntC, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if !(payload.new_t_int_c.is_finite() && payload.new_t_int_c > -273.15) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "t_int_c must be a finite temperature above absolute zero (-273.15 °C).", Vec::<String>::new());
        }
        if base.t_int_c == payload.new_t_int_c {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "t_int_c already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { t_int_c: Some(payload.new_t_int_c), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-t-int-c` inverse.

    use crate::change_t_int_c::ChangeTIntC;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeTIntC, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_t_int_c::ChangeTIntC> {
        Ok((|| {
        vec![Op::from(ChangeTIntC { new_t_int_c: base.t_int_c })]

        })())
    }
}
//#endregion 🪢️Mounts
