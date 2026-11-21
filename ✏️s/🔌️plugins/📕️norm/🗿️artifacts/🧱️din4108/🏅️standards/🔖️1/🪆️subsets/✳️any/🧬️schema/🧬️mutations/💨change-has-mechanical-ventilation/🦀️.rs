//! 💨 `change-has-mechanical-ventilation`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHasMechanicalVentilation {
    pub new_has_mechanical_ventilation: bool,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeHasMechanicalVentilation where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "has-mechanical-ventilation",
        kind: "change-has-mechanical-ventilation",
        record: "ChangedHasMechanicalVentilation",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change mechanical ventilation", "Mechanische Lüftung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeHasMechanicalVentilation where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_has_mechanical_ventilation::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_has_mechanical_ventilation::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-has-mechanical-ventilation` diff.

    use crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeHasMechanicalVentilation, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if base.has_mechanical_ventilation == payload.new_has_mechanical_ventilation {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "has_mechanical_ventilation already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { has_mechanical_ventilation: Some(payload.new_has_mechanical_ventilation), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-has-mechanical-ventilation` inverse.

    use crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeHasMechanicalVentilation, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation> {
        Ok((|| {
        vec![Op::from(ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: base.has_mechanical_ventilation })]

        })())
    }
}
//#endregion 🪢️Mounts
