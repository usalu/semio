//! 🧬️ En1995 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1995Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (twenty persistent scalar fields feeding the EN 1995 timber
//! bending/shear/deflection, connection, fire and bridge-fatigue checks) — no id-keyed
//! collections, no index-keyed ordered collections, no hierarchy, no relationships, no name/
//! identity field to `rename`. Every field becomes its own `change-<field>` mutation per the
//! rule's "change-<field> per remaining scalar" clause; none qualify for the `update-<facet>`
//! grouping exception (each check input is independently measured/entered in the host UI, never
//! validated as an atomic multi-field bundle — mirrors the `en1992`/`en1994` precedent, not
//! `en1993`'s per-part grouping, because `⚙️engine/🦀️component.rs`'s EN 1995 checks read this
//! snapshot as one flat bag of fields, not as named per-part sub-structs). `SetSnapshot` — the
//! pre-migration whole-document replace — is gone: banned outright per `📓️taxonomy.md`/
//! `📓️derivation-rules.md` rule 6, with NO replacement mutation; file-open/import/load-example now
//! goes through `store::ArtifactStore::reset`, entirely outside this enum.
//!
//! `📄set-snapshot` keeps its pre-migration directory name — `📦️glue.rs` path-includes that exact
//! triad outside this facet's writable boundary, so it was repurposed in place (same path,
//! rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) to hold `ChangeAnnex` instead of being
//! renamed; see this ticket's wave2 report `sharedFileRequests` for the rename once a later pass
//! can touch `📦️glue.rs` (mirrors the `en1990`/`en1992` precedent). The other nineteen triads have
//! no pre-migration slot and are self-wired directly below via nested `#[path = "."] pub mod
//! <name> { ... }` blocks — `#[path]` resolves per physical file, not per logical mod nesting, so
//! this works without touching `📦️glue.rs`.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️NewLeaves
#[path = "."]
pub mod change_m_ed_knm {
    #[path = "🔧change-m-ed-knm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-m-ed-knm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-m-ed-knm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_n_ed_kn {
    #[path = "🔧change-n-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-n-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-n-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_v_ed_kn {
    #[path = "🔧change-v-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-v-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-v-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_w_mm3 {
    #[path = "🔧change-w-mm3/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-w-mm3/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-w-mm3/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_a_mm2 {
    #[path = "🔧change-a-mm2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-a-mm2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-a-mm2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_b_mm {
    #[path = "🔧change-b-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-b-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-b-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_h_mm {
    #[path = "🔧change-h-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-h-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-h-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_f_m_k {
    #[path = "🔧change-f-m-k/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-f-m-k/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-f-m-k/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_f_c_0_k {
    #[path = "🔧change-f-c-0-k/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-f-c-0-k/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-f-c-0-k/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_service_class {
    #[path = "🔧change-service-class/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-service-class/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-service-class/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_load_duration {
    #[path = "🔧change-load-duration/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-load-duration/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-load-duration/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_m_crit_knm {
    #[path = "🔧change-m-crit-knm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-m-crit-knm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-m-crit-knm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_f_ed_kn {
    #[path = "🔧change-f-ed-kn/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-f-ed-kn/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-f-ed-kn/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_a_ef_mm2 {
    #[path = "🔧change-a-ef-mm2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-a-ef-mm2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-a-ef-mm2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_f_v_k {
    #[path = "🔧change-f-v-k/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-f-v-k/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-f-v-k/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fire_duration_min {
    #[path = "🔧change-fire-duration-min/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-fire-duration-min/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-fire-duration-min/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_section_depth_mm {
    #[path = "🔧change-section-depth-mm/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-section-depth-mm/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-section-depth-mm/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_a_vert_m_s2 {
    #[path = "🔧change-a-vert-m-s2/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-a-vert-m-s2/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-a-vert-m-s2/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_n_cycles_bridge {
    #[path = "🔧change-n-cycles-bridge/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔧change-n-cycles-bridge/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔧change-n-cycles-bridge/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ `set_snapshot` is declared by `📦️glue.rs` as a sibling of `component` (this file) under
// `pub mod mutations { ... }` — brought into this file's own scope the same way this ticket's
// `en1990`/`en1992` precedent reaches its own repurposed `set_snapshot` sibling.
use super::set_snapshot;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the En1995 document, derived per
/// `📓️derivation-rules.md` from `En1995Snapshot`'s flat scalar shape. `impl protocol::Mutation`/
/// `SemanticMutation` below are generated by `#[derive(dsl::Mutations)]` — never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = En1995Snapshot, diff = En1995Diff, schema = "s.norm.en1995")]
pub enum En1995Mutation {
    ChangeAnnex(set_snapshot::mutation::ChangeAnnex),
    ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm),
    ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn),
    ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn),
    ChangeWMm3(change_w_mm3::mutation::ChangeWMm3),
    ChangeAMm2(change_a_mm2::mutation::ChangeAMm2),
    ChangeBMm(change_b_mm::mutation::ChangeBMm),
    ChangeHMm(change_h_mm::mutation::ChangeHMm),
    ChangeFMK(change_f_m_k::mutation::ChangeFMK),
    ChangeFC0K(change_f_c_0_k::mutation::ChangeFC0K),
    ChangeServiceClass(change_service_class::mutation::ChangeServiceClass),
    ChangeLoadDuration(change_load_duration::mutation::ChangeLoadDuration),
    ChangeMCritKnm(change_m_crit_knm::mutation::ChangeMCritKnm),
    ChangeFEdKn(change_f_ed_kn::mutation::ChangeFEdKn),
    ChangeAEfMm2(change_a_ef_mm2::mutation::ChangeAEfMm2),
    ChangeFVK(change_f_v_k::mutation::ChangeFVK),
    ChangeFireDurationMin(change_fire_duration_min::mutation::ChangeFireDurationMin),
    ChangeSectionDepthMm(change_section_depth_mm::mutation::ChangeSectionDepthMm),
    ChangeAVertMS2(change_a_vert_m_s2::mutation::ChangeAVertMS2),
    ChangeNCyclesBridge(change_n_cycles_bridge::mutation::ChangeNCyclesBridge),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, SemanticMutation};

    /// ⚖️ One value per `En1995Mutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring this ticket's `en1992`/`en1993` precedent's own `every_mutation()` fixture.
    fn every_mutation() -> Vec<En1995Mutation> {
        vec![
            En1995Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1995Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 999.0 }),
            En1995Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 111.0 }),
            En1995Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 77.0 }),
            En1995Mutation::ChangeWMm3(change_w_mm3::mutation::ChangeWMm3 { new_w_mm3: 2_000_000.0 }),
            En1995Mutation::ChangeAMm2(change_a_mm2::mutation::ChangeAMm2 { new_a_mm2: 30_000.0 }),
            En1995Mutation::ChangeBMm(change_b_mm::mutation::ChangeBMm { new_b_mm: 250.0 }),
            En1995Mutation::ChangeHMm(change_h_mm::mutation::ChangeHMm { new_h_mm: 400.0 }),
            En1995Mutation::ChangeFMK(change_f_m_k::mutation::ChangeFMK { new_f_m_k: 28.0 }),
            En1995Mutation::ChangeFC0K(change_f_c_0_k::mutation::ChangeFC0K { new_f_c_0_k: 24.0 }),
            En1995Mutation::ChangeServiceClass(change_service_class::mutation::ChangeServiceClass { new_service_class: "sc2".into() }),
            En1995Mutation::ChangeLoadDuration(change_load_duration::mutation::ChangeLoadDuration { new_load_duration: "short".into() }),
            En1995Mutation::ChangeMCritKnm(change_m_crit_knm::mutation::ChangeMCritKnm { new_m_crit_knm: 95.0 }),
            En1995Mutation::ChangeFEdKn(change_f_ed_kn::mutation::ChangeFEdKn { new_f_ed_kn: 22.0 }),
            En1995Mutation::ChangeAEfMm2(change_a_ef_mm2::mutation::ChangeAEfMm2 { new_a_ef_mm2: 14_000.0 }),
            En1995Mutation::ChangeFVK(change_f_v_k::mutation::ChangeFVK { new_f_v_k: 4.5 }),
            En1995Mutation::ChangeFireDurationMin(change_fire_duration_min::mutation::ChangeFireDurationMin { new_fire_duration_min: 60.0 }),
            En1995Mutation::ChangeSectionDepthMm(change_section_depth_mm::mutation::ChangeSectionDepthMm { new_section_depth_mm: 350.0 }),
            En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::mutation::ChangeAVertMS2 { new_a_vert_m_s2: 0.5 }),
            En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::mutation::ChangeNCyclesBridge { new_n_cycles_bridge: 750_000.0 }),
        ]
    }

    fn round_trip(base: &En1995Snapshot, mutation: &En1995Mutation) -> En1995Snapshot {
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
        assert_eq!(<En1995Mutation as protocol::SemanticMutation<En1995Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1995Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::testkit`), exercised against the three most structurally
    /// distinct variants: the enum-typed scalar (`change-annex`), a typical `f64` scalar
    /// (`change-m-ed-knm`), and a `String` scalar (`change-service-class`).
    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = En1995Snapshot::default();
        let mutation = En1995Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1995Mutation::ChangeServiceClass(change_service_class::mutation::ChangeServiceClass { new_service_class: "sc2".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_m_ed_knm_satisfies_the_inverse_and_absorb_laws() {
        let base = En1995Snapshot::default();
        let mutation = En1995Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 999.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1995Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 77.0 }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_service_class_satisfies_the_inverse_and_absorb_laws() {
        let base = En1995Snapshot::default();
        let mutation = En1995Mutation::ChangeServiceClass(change_service_class::mutation::ChangeServiceClass { new_service_class: "sc2".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1995Mutation::ChangeLoadDuration(change_load_duration::mutation::ChangeLoadDuration { new_load_duration: "short".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
