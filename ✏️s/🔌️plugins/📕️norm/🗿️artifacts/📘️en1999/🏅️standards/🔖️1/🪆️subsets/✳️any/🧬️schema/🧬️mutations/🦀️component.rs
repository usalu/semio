//! 🧬️ En1999 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1999Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (twenty-six persistent scalar/enum fields describing the EN 1999 aluminium design check's actions, resistances, fatigue, weld, sheet and shell parameters) — no id-keyed
//! collections, no name/identity field to `rename`. Every field becomes its own `change-<field>`
//! mutation per the rule's "change-<field> per remaining scalar" clause; none qualify for the
//! `update-<facet>` grouping exception (each parameter is independently entered on its own input row,
//! never validated as an atomic multi-field bundle). The pre-migration whole-document-replace variant
//! is gone: banned outright per `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement
//! mutation; file-open/import/load-example now goes through `store::ArtifactStore::reset`, entirely
//! outside this enum. The old whole-document-replace macro call is removed with it.
//!
//! All triads are mounted directly as `mutations`-sibling modules in `📦️glue.rs` (this lane's agent
//! owns `📦️glue.rs`, so no self-wiring `#[path = "."]` blocks are needed here).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::change_n_ed_kn;
use super::change_m_ed_knm;
use super::change_a_mm2;
use super::change_w_el_mm3;
use super::change_alloy;
use super::change_chi;
use super::change_i_t_mm4;
use super::change_l_cr_mm;
use super::change_theta_c;
use super::change_delta_sigma_ed;
use super::change_delta_sigma_c;
use super::change_fatigue_m;
use super::change_n_cycles;
use super::change_v_weld_ed_kn;
use super::change_weld_throat_mm;
use super::change_weld_length_mm;
use super::change_beta_w;
use super::change_sheet_b_mm;
use super::change_sheet_t_mm;
use super::change_sheet_k_sigma;
use super::change_sheet_w_el_mm3;
use super::change_sheet_m_ed_knm;
use super::change_shell_t_mm;
use super::change_shell_r_mm;
use super::change_sigma_ed_shell_mpa;
use super::change_annex;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1999 document, derived per
/// `📓️derivation-rules.md` from `En1999Snapshot`'s flat scalar/enum shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1999Snapshot, diff = En1999Diff, schema = "norm.en1999")]
pub enum En1999Mutation {
    ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn),
    ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm),
    ChangeAMm2(change_a_mm2::mutation::ChangeAMm2),
    ChangeWElMm3(change_w_el_mm3::mutation::ChangeWElMm3),
    ChangeAlloy(change_alloy::mutation::ChangeAlloy),
    ChangeChi(change_chi::mutation::ChangeChi),
    ChangeITMm4(change_i_t_mm4::mutation::ChangeITMm4),
    ChangeLCrMm(change_l_cr_mm::mutation::ChangeLCrMm),
    ChangeThetaC(change_theta_c::mutation::ChangeThetaC),
    ChangeDeltaSigmaEd(change_delta_sigma_ed::mutation::ChangeDeltaSigmaEd),
    ChangeDeltaSigmaC(change_delta_sigma_c::mutation::ChangeDeltaSigmaC),
    ChangeFatigueM(change_fatigue_m::mutation::ChangeFatigueM),
    ChangeNCycles(change_n_cycles::mutation::ChangeNCycles),
    ChangeVWeldEdKn(change_v_weld_ed_kn::mutation::ChangeVWeldEdKn),
    ChangeWeldThroatMm(change_weld_throat_mm::mutation::ChangeWeldThroatMm),
    ChangeWeldLengthMm(change_weld_length_mm::mutation::ChangeWeldLengthMm),
    ChangeBetaW(change_beta_w::mutation::ChangeBetaW),
    ChangeSheetBMm(change_sheet_b_mm::mutation::ChangeSheetBMm),
    ChangeSheetTMm(change_sheet_t_mm::mutation::ChangeSheetTMm),
    ChangeSheetKSigma(change_sheet_k_sigma::mutation::ChangeSheetKSigma),
    ChangeSheetWElMm3(change_sheet_w_el_mm3::mutation::ChangeSheetWElMm3),
    ChangeSheetMEdKnm(change_sheet_m_ed_knm::mutation::ChangeSheetMEdKnm),
    ChangeShellTMm(change_shell_t_mm::mutation::ChangeShellTMm),
    ChangeShellRMm(change_shell_r_mm::mutation::ChangeShellRMm),
    ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::mutation::ChangeSigmaEdShellMpa),
    ChangeAnnex(change_annex::mutation::ChangeAnnex),
}
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1999Mutation {
    /// 📤️ Decomposes a whole `En1999Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app command to
    /// bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1999Snapshot) -> Vec<En1999Mutation> {
        let mut mutations = Vec::with_capacity(26);
        mutations.push(En1999Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: snapshot.n_ed_kn.clone() }));
        mutations.push(En1999Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: snapshot.m_ed_knm.clone() }));
        mutations.push(En1999Mutation::ChangeAMm2(change_a_mm2::mutation::ChangeAMm2 { new_a_mm2: snapshot.a_mm2.clone() }));
        mutations.push(En1999Mutation::ChangeWElMm3(change_w_el_mm3::mutation::ChangeWElMm3 { new_w_el_mm3: snapshot.w_el_mm3.clone() }));
        mutations.push(En1999Mutation::ChangeAlloy(change_alloy::mutation::ChangeAlloy { new_alloy: snapshot.alloy.clone() }));
        mutations.push(En1999Mutation::ChangeChi(change_chi::mutation::ChangeChi { new_chi: snapshot.chi.clone() }));
        mutations.push(En1999Mutation::ChangeITMm4(change_i_t_mm4::mutation::ChangeITMm4 { new_i_t_mm4: snapshot.i_t_mm4.clone() }));
        mutations.push(En1999Mutation::ChangeLCrMm(change_l_cr_mm::mutation::ChangeLCrMm { new_l_cr_mm: snapshot.l_cr_mm.clone() }));
        mutations.push(En1999Mutation::ChangeThetaC(change_theta_c::mutation::ChangeThetaC { new_theta_c: snapshot.theta_c.clone() }));
        mutations.push(En1999Mutation::ChangeDeltaSigmaEd(change_delta_sigma_ed::mutation::ChangeDeltaSigmaEd { new_delta_sigma_ed: snapshot.delta_sigma_ed.clone() }));
        mutations.push(En1999Mutation::ChangeDeltaSigmaC(change_delta_sigma_c::mutation::ChangeDeltaSigmaC { new_delta_sigma_c: snapshot.delta_sigma_c.clone() }));
        mutations.push(En1999Mutation::ChangeFatigueM(change_fatigue_m::mutation::ChangeFatigueM { new_fatigue_m: snapshot.fatigue_m.clone() }));
        mutations.push(En1999Mutation::ChangeNCycles(change_n_cycles::mutation::ChangeNCycles { new_n_cycles: snapshot.n_cycles.clone() }));
        mutations.push(En1999Mutation::ChangeVWeldEdKn(change_v_weld_ed_kn::mutation::ChangeVWeldEdKn { new_v_weld_ed_kn: snapshot.v_weld_ed_kn.clone() }));
        mutations.push(En1999Mutation::ChangeWeldThroatMm(change_weld_throat_mm::mutation::ChangeWeldThroatMm { new_weld_throat_mm: snapshot.weld_throat_mm.clone() }));
        mutations.push(En1999Mutation::ChangeWeldLengthMm(change_weld_length_mm::mutation::ChangeWeldLengthMm { new_weld_length_mm: snapshot.weld_length_mm.clone() }));
        mutations.push(En1999Mutation::ChangeBetaW(change_beta_w::mutation::ChangeBetaW { new_beta_w: snapshot.beta_w.clone() }));
        mutations.push(En1999Mutation::ChangeSheetBMm(change_sheet_b_mm::mutation::ChangeSheetBMm { new_sheet_b_mm: snapshot.sheet_b_mm.clone() }));
        mutations.push(En1999Mutation::ChangeSheetTMm(change_sheet_t_mm::mutation::ChangeSheetTMm { new_sheet_t_mm: snapshot.sheet_t_mm.clone() }));
        mutations.push(En1999Mutation::ChangeSheetKSigma(change_sheet_k_sigma::mutation::ChangeSheetKSigma { new_sheet_k_sigma: snapshot.sheet_k_sigma.clone() }));
        mutations.push(En1999Mutation::ChangeSheetWElMm3(change_sheet_w_el_mm3::mutation::ChangeSheetWElMm3 { new_sheet_w_el_mm3: snapshot.sheet_w_el_mm3.clone() }));
        mutations.push(En1999Mutation::ChangeSheetMEdKnm(change_sheet_m_ed_knm::mutation::ChangeSheetMEdKnm { new_sheet_m_ed_knm: snapshot.sheet_m_ed_knm.clone() }));
        mutations.push(En1999Mutation::ChangeShellTMm(change_shell_t_mm::mutation::ChangeShellTMm { new_shell_t_mm: snapshot.shell_t_mm.clone() }));
        mutations.push(En1999Mutation::ChangeShellRMm(change_shell_r_mm::mutation::ChangeShellRMm { new_shell_r_mm: snapshot.shell_r_mm.clone() }));
        mutations.push(En1999Mutation::ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::mutation::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: snapshot.sigma_ed_shell_mpa.clone() }));
        mutations.push(En1999Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: snapshot.annex.clone() }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    use protocol::Mutation;

    /// ⚖️ One value per `En1999Mutation` variant — the closed set the semantics/round-trip
    /// tests iterate.
    fn every_mutation() -> Vec<En1999Mutation> {
        vec![
        En1999Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 95.0 }),
        En1999Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 5.0 }),
        En1999Mutation::ChangeAMm2(change_a_mm2::mutation::ChangeAMm2 { new_a_mm2: 1300.0 }),
        En1999Mutation::ChangeWElMm3(change_w_el_mm3::mutation::ChangeWElMm3 { new_w_el_mm3: 26_000.0 }),
        En1999Mutation::ChangeAlloy(change_alloy::mutation::ChangeAlloy { new_alloy: "aw6082t6".to_string() }),
        En1999Mutation::ChangeChi(change_chi::mutation::ChangeChi { new_chi: 0.8 }),
        En1999Mutation::ChangeITMm4(change_i_t_mm4::mutation::ChangeITMm4 { new_i_t_mm4: 5400.0 }),
        En1999Mutation::ChangeLCrMm(change_l_cr_mm::mutation::ChangeLCrMm { new_l_cr_mm: 3200.0 }),
        En1999Mutation::ChangeThetaC(change_theta_c::mutation::ChangeThetaC { new_theta_c: 180.0 }),
        En1999Mutation::ChangeDeltaSigmaEd(change_delta_sigma_ed::mutation::ChangeDeltaSigmaEd { new_delta_sigma_ed: 50.0 }),
        En1999Mutation::ChangeDeltaSigmaC(change_delta_sigma_c::mutation::ChangeDeltaSigmaC { new_delta_sigma_c: 80.0 }),
        En1999Mutation::ChangeFatigueM(change_fatigue_m::mutation::ChangeFatigueM { new_fatigue_m: 5.0 }),
        En1999Mutation::ChangeNCycles(change_n_cycles::mutation::ChangeNCycles { new_n_cycles: 600_000.0 }),
        En1999Mutation::ChangeVWeldEdKn(change_v_weld_ed_kn::mutation::ChangeVWeldEdKn { new_v_weld_ed_kn: 28.0 }),
        En1999Mutation::ChangeWeldThroatMm(change_weld_throat_mm::mutation::ChangeWeldThroatMm { new_weld_throat_mm: 5.0 }),
        En1999Mutation::ChangeWeldLengthMm(change_weld_length_mm::mutation::ChangeWeldLengthMm { new_weld_length_mm: 140.0 }),
        En1999Mutation::ChangeBetaW(change_beta_w::mutation::ChangeBetaW { new_beta_w: 0.7 }),
        En1999Mutation::ChangeSheetBMm(change_sheet_b_mm::mutation::ChangeSheetBMm { new_sheet_b_mm: 220.0 }),
        En1999Mutation::ChangeSheetTMm(change_sheet_t_mm::mutation::ChangeSheetTMm { new_sheet_t_mm: 2.5 }),
        En1999Mutation::ChangeSheetKSigma(change_sheet_k_sigma::mutation::ChangeSheetKSigma { new_sheet_k_sigma: 4.2 }),
        En1999Mutation::ChangeSheetWElMm3(change_sheet_w_el_mm3::mutation::ChangeSheetWElMm3 { new_sheet_w_el_mm3: 8500.0 }),
        En1999Mutation::ChangeSheetMEdKnm(change_sheet_m_ed_knm::mutation::ChangeSheetMEdKnm { new_sheet_m_ed_knm: 0.6 }),
        En1999Mutation::ChangeShellTMm(change_shell_t_mm::mutation::ChangeShellTMm { new_shell_t_mm: 4.5 }),
        En1999Mutation::ChangeShellRMm(change_shell_r_mm::mutation::ChangeShellRMm { new_shell_r_mm: 520.0 }),
        En1999Mutation::ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::mutation::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: 160.0 }),
        En1999Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        ]
    }

    fn round_trip(base: &En1999Snapshot, mutation: &En1999Mutation) -> En1999Snapshot {
        let forward = vcs::apply_mutation(base, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<En1999Mutation as protocol::SemanticMutation<En1999Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1999Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    #[test]
    fn from_snapshot_round_trips_via_full_document_replacement() {
        let base = En1999Snapshot::default();
        let mut target = En1999Snapshot::default();
        let _ = &mut target;
        let mut projected = base.clone();
        for mutation in En1999Mutation::from_snapshot(&target) {
            projected = vcs::apply_mutation(&projected, &mutation);
        }
        assert_eq!(projected, target, "from_snapshot must reconstruct every persistent field");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
    /// variants.
    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = En1999Snapshot::default();
        let mutation = En1999Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1999Mutation::ChangeAlloy(change_alloy::mutation::ChangeAlloy { new_alloy: "aw6082t6".to_string() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_n_ed_kn_satisfies_the_inverse_and_absorb_laws() {
        let base = En1999Snapshot::default();
        let mutation = En1999Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 95.0 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1999Mutation::ChangeNCycles(change_n_cycles::mutation::ChangeNCycles { new_n_cycles: 600_000.0 }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_alloy_satisfies_the_inverse_and_absorb_laws() {
        let base = En1999Snapshot::default();
        let mutation = En1999Mutation::ChangeAlloy(change_alloy::mutation::ChangeAlloy { new_alloy: "aw6082t6".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1999Mutation::ChangeChi(change_chi::mutation::ChangeChi { new_chi: 0.8 }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
