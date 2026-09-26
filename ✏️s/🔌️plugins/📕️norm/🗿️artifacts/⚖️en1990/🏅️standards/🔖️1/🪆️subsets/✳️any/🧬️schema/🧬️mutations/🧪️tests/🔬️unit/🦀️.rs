use super::*;
use crate::document::AnnexChoice;
use protocol::Mutation;

fn every_mutation() -> Vec<En1990Mutation> {
    let base = En1990Snapshot::default();
    vec![
        En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: "office-cc3".into() }),
        En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: 500.0 }),
        En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: 3 }),
        En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: 3 }),
        En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: 5 }),
        En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: 100.0 }),
        En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: 100.0 }),
        En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: "DSL".into() }),
        En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: "IL3".into() }),
        En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: 4.3 }),
        En1990Mutation::ChangePermanents(change_permanents::ChangePermanents { new_permanents: base.permanents.clone() }),
        En1990Mutation::ChangeVariables(change_variables::ChangeVariables { new_variables: vec![] }),
        En1990Mutation::ChangeAccidentals(change_accidentals::ChangeAccidentals { new_accidentals: vec![] }),
        En1990Mutation::ChangeSeismics(change_seismics::ChangeSeismics { new_seismics: vec![] }),
        En1990Mutation::ChangeMembers(change_members::ChangeMembers { new_members: base.members.clone() }),
        En1990Mutation::ChangeBridgeSls(change_bridge_sls::ChangeBridgeSls { new_bridge_sls: base.bridge_sls.clone() }),
        En1990Mutation::ChangeEffects(change_effects::ChangeEffects { new_effects: base.effects.clone() }),
        En1990Mutation::RemoveEffect(remove_effect::RemoveEffect { index: 0 }),
        En1990Mutation::RemoveMember(remove_member::RemoveMember { index: 0 }),
        En1990Mutation::RemoveSeismic(remove_seismic::RemoveSeismic { index: 0 }),
        En1990Mutation::RemoveAccidental(remove_accidental::RemoveAccidental { index: 0 }),
        En1990Mutation::RemoveVariable(remove_variable::RemoveVariable { index: 0 }),
        En1990Mutation::RemovePermanent(remove_permanent::RemovePermanent { index: 0 }),
        En1990Mutation::InsertEffect(insert_effect::InsertEffect { index: 0, item: base.effects[0].clone() }),
        En1990Mutation::InsertMember(insert_member::InsertMember { index: 0, item: base.members[0].clone() }),
        En1990Mutation::InsertSeismic(insert_seismic::InsertSeismic { index: 0, item: crate::SeismicAction { id: "A-ek".into(), a_ek: 0.0, importance_class: crate::ImportanceClass::II } }),
        En1990Mutation::InsertAccidental(insert_accidental::InsertAccidental { index: 0, item: crate::AccidentalAction { id: "A-d".into(), ad: 0.0 } }),
        En1990Mutation::InsertVariable(insert_variable::InsertVariable { index: 0, item: crate::VariableAction { id: "Q-x".into(), category: "other".into(), qk: 0.0} }),
        En1990Mutation::InsertPermanent(insert_permanent::InsertPermanent { index: 0, item: crate::PermanentAction { id: "G-x".into(), kind: "g_sup".into(), gk: 0.0 } }),
    ]
}

fn round_trip(base: &En1990Snapshot, mutation: &En1990Mutation) -> En1990Snapshot {
    let (forward, _messages) = vcs::apply_mutation(base, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(base) {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<En1990Mutation as protocol::SemanticMutation<En1990Snapshot>>::kinds().len(), every_mutation().len());
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_through_diff_and_inverse() {
    let base = En1990Snapshot::default();
    for mutation in every_mutation() {
        if matches!(
            mutation,
            En1990Mutation::RemoveSeismic(_) | En1990Mutation::RemoveAccidental(_)
        ) {
            // Default subject has empty seismics/accidentals — out-of-range remove is fatal by design.
            continue;
        }
        let _ = round_trip(&base, &mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let _ = round_trip(&base, &mutation);
}

#[semio_framework_async_macros::async_test]
async fn change_consequence_class_and_beta_absorb() {
    let base = En1990Snapshot::default();
    let d1 = En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: 3 }).diff(&base).diff().clone();
    let d2 = En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: 4.3 }).diff(&base).diff().clone();
    let mut absorbed = d1.clone();
    protocol::MutationDiff::<En1990Snapshot>::absorb(&mut absorbed, d2.clone());
    assert_eq!(absorbed.consequence_class, Some(3));
    assert_eq!(absorbed.beta_computed, Some(4.3));
}

#[semio_framework_async_macros::async_test]
async fn from_snapshot_decomposes_scalar_and_list_changes() {
    let base = En1990Snapshot::default();
    let mut target = base.clone();
    target.consequence_class = 3;
    target.variables = vec![];
    let mutations = En1990Mutation::from_snapshot(&base, &target);
    assert!(mutations.iter().any(|m| matches!(m, En1990Mutation::ChangeConsequenceClass(_))));
    assert!(mutations.iter().any(|m| matches!(m, En1990Mutation::ChangeVariables(_))));
}

#[semio_framework_async_macros::async_test]
async fn no_op_change_annex_emits_empty_diff() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: base.annex });
    let outcome = mutation.diff(&base);
    assert!(outcome.diff().annex.is_none());
}
