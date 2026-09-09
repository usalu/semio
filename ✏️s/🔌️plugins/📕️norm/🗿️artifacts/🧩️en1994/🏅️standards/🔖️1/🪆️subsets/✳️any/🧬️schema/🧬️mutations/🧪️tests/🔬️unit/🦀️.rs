use super::*;
use crate::document::AnnexChoice;
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(base: &En1994Snapshot, operation: &En1994Mutation) -> En1994Snapshot {
    let forward = operation.diff(base).diff().apply(base).expect("valid mutation diff");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(base).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

/// 🧪️ One representative value per variant — reused by the round-trip law test below and by
/// `📝️text/🦀️.rs`'s `OpText`/`OpBinary` round-trip law.
pub(crate) fn demo_mutation_cases() -> Vec<En1994Mutation> {
    vec![
        En1994Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1994Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 123.5_f64 }),
        En1994Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 123.5_f64 }),
        En1994Mutation::ChangeMPla(change_m_pla::ChangeMPla { new_m_pla: 123.5_f64 }),
        En1994Mutation::ChangeMPlRd(change_m_pl_rd::ChangeMPlRd { new_m_pl_rd: 123.5_f64 }),
        En1994Mutation::ChangeEta(change_eta::ChangeEta { new_eta: 123.5_f64 }),
        En1994Mutation::ChangeVLRd(change_v_l_rd::ChangeVLRd { new_v_l_rd: 123.5_f64 }),
        En1994Mutation::ChangeInsulationThicknessMm(change_insulation_thickness_mm::ChangeInsulationThicknessMm { new_insulation_thickness_mm: 123.5_f64 }),
        En1994Mutation::ChangeFireRating(change_fire_rating::ChangeFireRating { new_fire_rating: "fire_rating-demo".to_string() }),
        En1994Mutation::ChangeDeckType(change_deck_type::ChangeDeckType { new_deck_type: "deck_type-demo".to_string() }),
        En1994Mutation::ChangeDeltaSigmaMpa(change_delta_sigma_mpa::ChangeDeltaSigmaMpa { new_delta_sigma_mpa: 123.5_f64 }),
        En1994Mutation::ChangeFatigueDetail(change_fatigue_detail::ChangeFatigueDetail { new_fatigue_detail: "fatigue_detail-demo".to_string() }),
        En1994Mutation::ChangeDMm(change_d_mm::ChangeDMm { new_d_mm: 123.5_f64 }),
        En1994Mutation::ChangeHScMm(change_h_sc_mm::ChangeHScMm { new_h_sc_mm: 123.5_f64 }),
        En1994Mutation::ChangeFCkMpa(change_f_ck_mpa::ChangeFCkMpa { new_f_ck_mpa: 123.5_f64 }),
        En1994Mutation::ChangeFUMpa(change_f_u_mpa::ChangeFUMpa { new_f_u_mpa: 123.5_f64 }),
        En1994Mutation::ChangeECmMpa(change_e_cm_mpa::ChangeECmMpa { new_e_cm_mpa: 123.5_f64 }),
        En1994Mutation::ChangeVEdPerStudKn(change_v_ed_per_stud_kn::ChangeVEdPerStudKn { new_v_ed_per_stud_kn: 123.5_f64 }),
        En1994Mutation::ChangeSpanM(change_span_m::ChangeSpanM { new_span_m: 123.5_f64 }),
        En1994Mutation::ChangeFYMpa(change_f_y_mpa::ChangeFYMpa { new_f_y_mpa: 123.5_f64 }),
        En1994Mutation::ChangeNCyclesStud(change_n_cycles_stud::ChangeNCyclesStud { new_n_cycles_stud: 123.5_f64 }),
        En1994Mutation::ChangeDeltaTauStudMpa(change_delta_tau_stud_mpa::ChangeDeltaTauStudMpa { new_delta_tau_stud_mpa: 123.5_f64 }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_and_restores_base() {
    let base = En1994Snapshot::default();
    for mutation in demo_mutation_cases() {
        round_trip(&base, &mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn change_annex_round_trips() {
    let base = En1994Snapshot::default();
    let mutation = En1994Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.annex, AnnexChoice::En);
    assert_ne!(base.annex, AnnexChoice::En, "fixture default must differ from the new value to exercise a real change");
}

#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_round_trips() {
    let base = En1994Snapshot::default();
    let mutation = En1994Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.m_ed_knm, 999.0);
    let undo = mutation.inverse(&base);
    assert_eq!(undo, vec![En1994Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: base.m_ed_knm })]);
}

#[semio_framework_async_macros::async_test]
async fn change_fire_rating_round_trips() {
    let base = En1994Snapshot::default();
    let mutation = En1994Mutation::ChangeFireRating(change_fire_rating::ChangeFireRating { new_fire_rating: "r120".into() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.fire_rating, "r120");
}

#[semio_framework_async_macros::async_test]
async fn change_eta_inverse_restores_base_value() {
    let base = En1994Snapshot { eta: 0.6, ..En1994Snapshot::default() };
    let mutation = En1994Mutation::ChangeEta(change_eta::ChangeEta { new_eta: 0.9 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.eta, 0.9);
    let undo = mutation.inverse(&base);
    assert_eq!(undo, vec![En1994Mutation::ChangeEta(change_eta::ChangeEta { new_eta: 0.6 })]);
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(En1994Mutation::kinds().len(), 22);
    let mutation = En1994Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::De });
    assert_eq!(mutation.semantics().kind, "change-annex");
    assert_eq!(mutation.semantics().record, "ChangedAnnex");
    assert_eq!(mutation.semantics().verb, "change");
}

#[semio_framework_async_macros::async_test]
async fn labels_are_human_readable() {
    let mutation = En1994Mutation::ChangeSpanM(change_span_m::ChangeSpanM { new_span_m: 12.0 });
    assert_eq!(mutation.label(), "Change span to 12");
}

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// this facet is entirely one verb family (root-scoped `change-<field>`) — see en1992's own
/// `🔖️OutcomeLaws` note for why `assert_missing_target_is_error`/`assert_outcome_policy_matrix`
/// don't apply/aren't landed yet.
#[semio_framework_async_macros::async_test]
async fn change_eta_non_finite_is_fatal() {
    let base = En1994Snapshot::default();
    let mutation = En1994Mutation::ChangeEta(change_eta::ChangeEta { new_eta: f64::NAN });
    let outcome = mutation.diff(&base);
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn change_annex_same_value_is_no_op() {
    let base = En1994Snapshot::default();
    let mutation = En1994Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: base.annex });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert_eq!(outcome.diff(), &En1994Diff::default());
}

#[semio_framework_async_macros::async_test]
async fn change_span_m_is_deterministic() {
    let base = En1994Snapshot::default();
    let mutation = En1994Mutation::ChangeSpanM(change_span_m::ChangeSpanM { new_span_m: 12.0 });
    protocol::os_spr::testkit::assert_outcome_deterministic(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws
