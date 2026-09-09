//! 🧬️ En1997 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `En1997Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (twenty-two persistent scalar/enum fields describing the EN 1997 geotechnical (shallow-footing, pile) design check's actions, resistances and ground parameters) — no id-keyed
//! collections, no name/identity field to `rename`. Every field becomes its own `change-<field>`
//! mutation per the rule's "change-<field> per remaining scalar" clause; none qualify for the
//! `update-<facet>` grouping exception (each parameter is independently entered on its own input row,
//! never validated as an atomic multi-field bundle). The pre-migration whole-document-replace variant
//! is gone: banned outright per `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement
//! mutation; file-open/import/load-example now goes through `store::ArtifactStore::reset`, entirely
//! outside this enum. The old whole-document-replace macro call is removed with it.
//!
//! All triads are mounted directly as `mutations`-sibling modules in `🦀️.rs` (this lane's agent
//! owns `🦀️.rs`, so no self-wiring `#[path = "."]` blocks are needed for the TRIADS).

use crate::diff::En1997Diff;
use crate::En1997Snapshot;

//#region 🔖️Leaves
use super::change_alpha_s;
use super::change_annex;
use super::change_b_m;
use super::change_c_kpa;
use super::change_d_f_m;
use super::change_design_approach;
use super::change_e_s_mpa;
use super::change_footing_area_m2;
use super::change_gamma_kn_m3;
use super::change_h_ed_kn;
use super::change_n_pile_ed_kn;
use super::change_nu;
use super::change_phi_deg;
use super::change_pile_base_area_m2;
use super::change_pile_d_m;
use super::change_pile_l_m;
use super::change_pile_n_profiles;
use super::change_q_b_kpa;
use super::change_q_s_kpa;
use super::change_settlement_limit_mm;
use super::change_v_ed_kn;
use super::change_z_investigated_m;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1997 document, derived per
/// `📓️derivation-rules.md` from `En1997Snapshot`'s flat scalar/enum shape.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1997Snapshot, diff = En1997Diff, schema = "norm.en1997")]
pub enum En1997Mutation {
    ChangeVEdKn(change_v_ed_kn::ChangeVEdKn),
    ChangeHEdKn(change_h_ed_kn::ChangeHEdKn),
    ChangeFootingAreaM2(change_footing_area_m2::ChangeFootingAreaM2),
    ChangePhiDeg(change_phi_deg::ChangePhiDeg),
    ChangeCKpa(change_c_kpa::ChangeCKpa),
    ChangeGammaKnM3(change_gamma_kn_m3::ChangeGammaKnM3),
    ChangeBM(change_b_m::ChangeBM),
    ChangeDFM(change_d_f_m::ChangeDFM),
    ChangeESMpa(change_e_s_mpa::ChangeESMpa),
    ChangeNu(change_nu::ChangeNu),
    ChangeDesignApproach(change_design_approach::ChangeDesignApproach),
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeSettlementLimitMm(change_settlement_limit_mm::ChangeSettlementLimitMm),
    ChangeNPileEdKn(change_n_pile_ed_kn::ChangeNPileEdKn),
    ChangeAlphaS(change_alpha_s::ChangeAlphaS),
    ChangePileDM(change_pile_d_m::ChangePileDM),
    ChangeQSKpa(change_q_s_kpa::ChangeQSKpa),
    ChangePileLM(change_pile_l_m::ChangePileLM),
    ChangeQBKpa(change_q_b_kpa::ChangeQBKpa),
    ChangePileBaseAreaM2(change_pile_base_area_m2::ChangePileBaseAreaM2),
    ChangePileNProfiles(change_pile_n_profiles::ChangePileNProfiles),
    ChangeZInvestigatedM(change_z_investigated_m::ChangeZInvestigatedM),
}

/// 🏷️ Every declared kind of [`En1997Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🔣️oracle.json` publishes as the `en1997-1-any`
/// mutation catalog and `../../../../../🧪️tests/🌍️mutate-en1997-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-v-ed-kn",
    "change-h-ed-kn",
    "change-footing-area-m2",
    "change-phi-deg",
    "change-c-kpa",
    "change-gamma-kn-m3",
    "change-bm",
    "change-dfm",
    "change-es-mpa",
    "change-nu",
    "change-design-approach",
    "change-annex",
    "change-settlement-limit-mm",
    "change-n-pile-ed-kn",
    "change-alpha-s",
    "change-pile-dm",
    "change-qs-kpa",
    "change-pile-lm",
    "change-qb-kpa",
    "change-pile-base-area-m2",
    "change-pile-n-profiles",
    "change-z-investigated-m",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1997Mutation {
    /// 📤️ Decomposes a whole `En1997Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app command to
    /// bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1997Snapshot) -> Vec<En1997Mutation> {
        let mut mutations = Vec::with_capacity(22);
        mutations.push(En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn }));
        mutations.push(En1997Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn: snapshot.h_ed_kn }));
        mutations.push(En1997Mutation::ChangeFootingAreaM2(change_footing_area_m2::ChangeFootingAreaM2 { new_footing_area_m2: snapshot.footing_area_m2 }));
        mutations.push(En1997Mutation::ChangePhiDeg(change_phi_deg::ChangePhiDeg { new_phi_deg: snapshot.phi_deg }));
        mutations.push(En1997Mutation::ChangeCKpa(change_c_kpa::ChangeCKpa { new_c_kpa: snapshot.c_kpa }));
        mutations.push(En1997Mutation::ChangeGammaKnM3(change_gamma_kn_m3::ChangeGammaKnM3 { new_gamma_kn_m3: snapshot.gamma_kn_m3 }));
        mutations.push(En1997Mutation::ChangeBM(change_b_m::ChangeBM { new_b_m: snapshot.b_m }));
        mutations.push(En1997Mutation::ChangeDFM(change_d_f_m::ChangeDFM { new_d_f_m: snapshot.d_f_m }));
        mutations.push(En1997Mutation::ChangeESMpa(change_e_s_mpa::ChangeESMpa { new_e_s_mpa: snapshot.e_s_mpa }));
        mutations.push(En1997Mutation::ChangeNu(change_nu::ChangeNu { new_nu: snapshot.nu }));
        mutations.push(En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: snapshot.design_approach.clone() }));
        mutations.push(En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
        mutations.push(En1997Mutation::ChangeSettlementLimitMm(change_settlement_limit_mm::ChangeSettlementLimitMm { new_settlement_limit_mm: snapshot.settlement_limit_mm }));
        mutations.push(En1997Mutation::ChangeNPileEdKn(change_n_pile_ed_kn::ChangeNPileEdKn { new_n_pile_ed_kn: snapshot.n_pile_ed_kn }));
        mutations.push(En1997Mutation::ChangeAlphaS(change_alpha_s::ChangeAlphaS { new_alpha_s: snapshot.alpha_s }));
        mutations.push(En1997Mutation::ChangePileDM(change_pile_d_m::ChangePileDM { new_pile_d_m: snapshot.pile_d_m }));
        mutations.push(En1997Mutation::ChangeQSKpa(change_q_s_kpa::ChangeQSKpa { new_q_s_kpa: snapshot.q_s_kpa }));
        mutations.push(En1997Mutation::ChangePileLM(change_pile_l_m::ChangePileLM { new_pile_l_m: snapshot.pile_l_m }));
        mutations.push(En1997Mutation::ChangeQBKpa(change_q_b_kpa::ChangeQBKpa { new_q_b_kpa: snapshot.q_b_kpa }));
        mutations.push(En1997Mutation::ChangePileBaseAreaM2(change_pile_base_area_m2::ChangePileBaseAreaM2 { new_pile_base_area_m2: snapshot.pile_base_area_m2 }));
        mutations.push(En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::ChangePileNProfiles { new_pile_n_profiles: snapshot.pile_n_profiles }));
        mutations.push(En1997Mutation::ChangeZInvestigatedM(change_z_investigated_m::ChangeZInvestigatedM { new_z_investigated_m: snapshot.z_investigated_m }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
/// 🧪️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`),
/// self-wired here rather than in `🦀️.rs`: that file is shared with the other artifact lanes
/// running concurrently, and a `#[path]` on a module declared at the top level of this non-mod-rs
/// file already resolves relative to this very directory.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`En1997Mutation`]. The generated test host of
/// `../../../../../🧪️tests/🌍️mutate-en1997-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1997_mutation_json(text: &str) -> Result<En1997Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_en1997_mutation(base: &En1997Snapshot, mutation: &En1997Mutation) -> Result<(En1997Snapshot, Vec<String>), String> {
    let raised = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `🌍️mutate-en1997-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_en1997_mutation(mutation: &En1997Mutation, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
