//! 🧬️ En1996 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1996Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (twenty-two persistent scalar/enum fields describing the EN 1996
//! masonry design check's actions, resistances, unit/mortar classes and effective geometry) — no
//! id-keyed collections, no name/identity field to `rename`. Every field becomes its own
//! `change-<field>` mutation per the rule's "change-<field> per remaining scalar" clause; none
//! qualify for the `update-<facet>` grouping exception (each parameter is independently entered on
//! its own input row, never validated as an atomic multi-field bundle). The pre-migration
//! whole-document-replace variant is gone: banned outright per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement mutation; file-open/import/
//! load-example now goes through `store::ArtifactStore::reset`, entirely outside this enum. The old
//! `crate::impl_norm_set_snapshot_ops!` macro call is removed with it.
//!
//! All twenty-two triads are mounted directly as `mutations`-sibling modules in `📦️glue.rs` (this
//! lane's agent owns `📦️glue.rs`, so no self-wiring `#[path = "."]` blocks are needed here — unlike
//! the wave-2 precedent in sibling facets that could not touch glue).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::change_annex;
use super::change_area_mm2;
use super::change_bed_joint_thickness_mm;
use super::change_design_situation;
use super::change_exposure;
use super::change_f_k_mpa;
use super::change_f_vk_mpa;
use super::change_fire_resistance_min;
use super::change_h_ed_kn;
use super::change_h_ef_mm;
use super::change_m_ed_knm;
use super::change_masonry_class;
use super::change_mortar;
use super::change_mu;
use super::change_n_ed_kn;
use super::change_shear_area_mm2;
use super::change_storeys;
use super::change_t_ef_mm;
use super::change_unit;
use super::change_v_ed_kn;
use super::change_wall_thickness_mm;
use super::change_z_mm3;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1996 document, derived per
/// `📓️derivation-rules.md` from `En1996Snapshot`'s flat scalar/enum shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1996Snapshot, diff = En1996Diff, schema = "norm.en1996")]
pub enum En1996Mutation {
    ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm),
    ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn),
    ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn),
    ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn),
    ChangeZMm3(change_z_mm3::mutation::ChangeZMm3),
    ChangeAreaMm2(change_area_mm2::mutation::ChangeAreaMm2),
    ChangeShearAreaMm2(change_shear_area_mm2::mutation::ChangeShearAreaMm2),
    ChangeFKMpa(change_f_k_mpa::mutation::ChangeFKMpa),
    ChangeFVkMpa(change_f_vk_mpa::mutation::ChangeFVkMpa),
    ChangeAnnex(change_annex::mutation::ChangeAnnex),
    ChangeMasonryClass(change_masonry_class::mutation::ChangeMasonryClass),
    ChangeDesignSituation(change_design_situation::mutation::ChangeDesignSituation),
    ChangeMu(change_mu::mutation::ChangeMu),
    ChangeWallThicknessMm(change_wall_thickness_mm::mutation::ChangeWallThicknessMm),
    ChangeFireResistanceMin(change_fire_resistance_min::mutation::ChangeFireResistanceMin),
    ChangeUnit(change_unit::mutation::ChangeUnit),
    ChangeExposure(change_exposure::mutation::ChangeExposure),
    ChangeMortar(change_mortar::mutation::ChangeMortar),
    ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm),
    ChangeStoreys(change_storeys::mutation::ChangeStoreys),
    ChangeHEfMm(change_h_ef_mm::mutation::ChangeHEfMm),
    ChangeTEfMm(change_t_ef_mm::mutation::ChangeTEfMm),
}
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1996Mutation {
    /// 📤️ Decomposes a whole `En1996Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app
    /// command to bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1996Snapshot) -> Vec<En1996Mutation> {
        let mut mutations = Vec::with_capacity(22);
        mutations.push(En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: snapshot.m_ed_knm.clone() }));
        mutations.push(En1996Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: snapshot.n_ed_kn.clone() }));
        mutations.push(En1996Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn.clone() }));
        mutations.push(En1996Mutation::ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn { new_h_ed_kn: snapshot.h_ed_kn.clone() }));
        mutations.push(En1996Mutation::ChangeZMm3(change_z_mm3::mutation::ChangeZMm3 { new_z_mm3: snapshot.z_mm3.clone() }));
        mutations.push(En1996Mutation::ChangeAreaMm2(change_area_mm2::mutation::ChangeAreaMm2 { new_area_mm2: snapshot.area_mm2.clone() }));
        mutations.push(En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::mutation::ChangeShearAreaMm2 { new_shear_area_mm2: snapshot.shear_area_mm2.clone() }));
        mutations.push(En1996Mutation::ChangeFKMpa(change_f_k_mpa::mutation::ChangeFKMpa { new_f_k_mpa: snapshot.f_k_mpa.clone() }));
        mutations.push(En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::mutation::ChangeFVkMpa { new_f_vk_mpa: snapshot.f_vk_mpa.clone() }));
        mutations.push(En1996Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: snapshot.annex.clone() }));
        mutations.push(En1996Mutation::ChangeMasonryClass(change_masonry_class::mutation::ChangeMasonryClass { new_masonry_class: snapshot.masonry_class.clone() }));
        mutations.push(En1996Mutation::ChangeDesignSituation(change_design_situation::mutation::ChangeDesignSituation { new_design_situation: snapshot.design_situation.clone() }));
        mutations.push(En1996Mutation::ChangeMu(change_mu::mutation::ChangeMu { new_mu: snapshot.mu.clone() }));
        mutations.push(En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::mutation::ChangeWallThicknessMm { new_wall_thickness_mm: snapshot.wall_thickness_mm.clone() }));
        mutations.push(En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::mutation::ChangeFireResistanceMin { new_fire_resistance_min: snapshot.fire_resistance_min.clone() }));
        mutations.push(En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit: snapshot.unit.clone() }));
        mutations.push(En1996Mutation::ChangeExposure(change_exposure::mutation::ChangeExposure { new_exposure: snapshot.exposure.clone() }));
        mutations.push(En1996Mutation::ChangeMortar(change_mortar::mutation::ChangeMortar { new_mortar: snapshot.mortar.clone() }));
        mutations.push(En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: snapshot.bed_joint_thickness_mm.clone() }));
        mutations.push(En1996Mutation::ChangeStoreys(change_storeys::mutation::ChangeStoreys { new_storeys: snapshot.storeys.clone() }));
        mutations.push(En1996Mutation::ChangeHEfMm(change_h_ef_mm::mutation::ChangeHEfMm { new_h_ef_mm: snapshot.h_ef_mm.clone() }));
        mutations.push(En1996Mutation::ChangeTEfMm(change_t_ef_mm::mutation::ChangeTEfMm { new_t_ef_mm: snapshot.t_ef_mm.clone() }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;
    use protocol::SemanticMutation;

    /// ⚖️ One value per `En1996Mutation` variant — the closed set the semantics/round-trip
    /// tests iterate, mirroring `din16798`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<En1996Mutation> {
        vec![
            En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 12.5 }),
            En1996Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 250.0 }),
            En1996Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 42.0 }),
            En1996Mutation::ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn { new_h_ed_kn: 28.0 }),
            En1996Mutation::ChangeZMm3(change_z_mm3::mutation::ChangeZMm3 { new_z_mm3: 9_500_000.0 }),
            En1996Mutation::ChangeAreaMm2(change_area_mm2::mutation::ChangeAreaMm2 { new_area_mm2: 540_000.0 }),
            En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::mutation::ChangeShearAreaMm2 { new_shear_area_mm2: 320_000.0 }),
            En1996Mutation::ChangeFKMpa(change_f_k_mpa::mutation::ChangeFKMpa { new_f_k_mpa: 6.5 }),
            En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::mutation::ChangeFVkMpa { new_f_vk_mpa: 0.18 }),
            En1996Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1996Mutation::ChangeMasonryClass(change_masonry_class::mutation::ChangeMasonryClass { new_masonry_class: crate::artifacts::en1996::MasonryClass::Class4 }),
            En1996Mutation::ChangeDesignSituation(change_design_situation::mutation::ChangeDesignSituation { new_design_situation: crate::document::DesignSituation::Seismic }),
            En1996Mutation::ChangeMu(change_mu::mutation::ChangeMu { new_mu: 0.35 }),
            En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::mutation::ChangeWallThicknessMm { new_wall_thickness_mm: 300.0 }),
            En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::mutation::ChangeFireResistanceMin { new_fire_resistance_min: 90 }),
            En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit: "calcium_silicate".to_string() }),
            En1996Mutation::ChangeExposure(change_exposure::mutation::ChangeExposure { new_exposure: crate::artifacts::en1996::part_2::ExposureClass::Mx3 }),
            En1996Mutation::ChangeMortar(change_mortar::mutation::ChangeMortar { new_mortar: crate::artifacts::en1996::part_2::MortarClass::M10 }),
            En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: 15.0 }),
            En1996Mutation::ChangeStoreys(change_storeys::mutation::ChangeStoreys { new_storeys: 4 }),
            En1996Mutation::ChangeHEfMm(change_h_ef_mm::mutation::ChangeHEfMm { new_h_ef_mm: 2800.0 }),
            En1996Mutation::ChangeTEfMm(change_t_ef_mm::mutation::ChangeTEfMm { new_t_ef_mm: 200.0 }),
        ]
    }

    fn round_trip(base: &En1996Snapshot, mutation: &En1996Mutation) -> En1996Snapshot {
        let (forward, _messages) =
            vcs::apply_mutation(base, mutation).expect("valid mutation");
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            let (next, _messages) =
                vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
            restored = next;
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
        assert_eq!(<En1996Mutation as protocol::SemanticMutation<En1996Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1996Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
    /// variants: an enum scalar (`change-annex`), a plain `f64` scalar (`change-m-ed-knm`), and a
    /// `String` scalar (`change-unit`).
    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit: "calcium_silicate".to_string() }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_m_ed_knm_satisfies_the_inverse_and_absorb_laws() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 12.5 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1996Mutation::ChangeStoreys(change_storeys::mutation::ChangeStoreys { new_storeys: 4 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_unit_satisfies_the_inverse_and_absorb_laws() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit: "calcium_silicate".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1996Mutation::ChangeFKMpa(change_f_k_mpa::mutation::ChangeFKMpa { new_f_k_mpa: 6.5 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// this facet is entirely one verb family (root-scoped `change-<field>`) — see en1992's own
    /// `🔖️OutcomeLaws` note for why `assert_missing_target_is_error`/`assert_outcome_policy_matrix`
    /// don't apply/aren't landed yet.
    #[test]
    fn change_m_ed_knm_non_finite_is_fatal() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: f64::NAN });
        let outcome = mutation.diff(&base);
        protocol::os_spr::testkit::assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    fn change_masonry_class_same_value_is_no_op() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::ChangeMasonryClass(change_masonry_class::mutation::ChangeMasonryClass { new_masonry_class: base.masonry_class });
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert_eq!(outcome.diff(), &En1996Diff::default());
    }

    #[test]
    fn change_m_ed_knm_is_deterministic() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 12.5 });
        protocol::os_spr::testkit::assert_outcome_deterministic(&base, &mutation);
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
