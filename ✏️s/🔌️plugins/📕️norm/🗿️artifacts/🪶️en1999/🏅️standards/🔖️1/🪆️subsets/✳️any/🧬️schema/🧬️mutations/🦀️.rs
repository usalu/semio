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
//! All triads are mounted directly as `mutations`-sibling modules in `🦀️.rs` (this lane's agent
//! owns `🦀️.rs`, so no self-wiring `#[path = "."]` blocks are needed here).

use crate::diff::En1999Diff;
use crate::En1999Snapshot;

//#region 🔖️Leaves
use super::change_a_mm2;
use super::change_alloy;
use super::change_annex;
use super::change_beta_w;
use super::change_chi;
use super::change_delta_sigma_c;
use super::change_delta_sigma_ed;
use super::change_fatigue_m;
use super::change_i_t_mm4;
use super::change_l_cr_mm;
use super::change_m_ed_knm;
use super::change_n_cycles;
use super::change_n_ed_kn;
use super::change_sheet_b_mm;
use super::change_sheet_k_sigma;
use super::change_sheet_m_ed_knm;
use super::change_sheet_t_mm;
use super::change_sheet_w_el_mm3;
use super::change_shell_r_mm;
use super::change_shell_t_mm;
use super::change_sigma_ed_shell_mpa;
use super::change_theta_c;
use super::change_v_weld_ed_kn;
use super::change_w_el_mm3;
use super::change_weld_length_mm;
use super::change_weld_throat_mm;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1999 document, derived per
/// `📓️derivation-rules.md` from `En1999Snapshot`'s flat scalar/enum shape.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1999Snapshot, diff = En1999Diff, schema = "norm.en1999")]
pub enum En1999Mutation {
    ChangeNEdKn(change_n_ed_kn::ChangeNEdKn),
    ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm),
    ChangeAMm2(change_a_mm2::ChangeAMm2),
    ChangeWElMm3(change_w_el_mm3::ChangeWElMm3),
    ChangeAlloy(change_alloy::ChangeAlloy),
    ChangeChi(change_chi::ChangeChi),
    ChangeITMm4(change_i_t_mm4::ChangeITMm4),
    ChangeLCrMm(change_l_cr_mm::ChangeLCrMm),
    ChangeThetaC(change_theta_c::ChangeThetaC),
    ChangeDeltaSigmaEd(change_delta_sigma_ed::ChangeDeltaSigmaEd),
    ChangeDeltaSigmaC(change_delta_sigma_c::ChangeDeltaSigmaC),
    ChangeFatigueM(change_fatigue_m::ChangeFatigueM),
    ChangeNCycles(change_n_cycles::ChangeNCycles),
    ChangeVWeldEdKn(change_v_weld_ed_kn::ChangeVWeldEdKn),
    ChangeWeldThroatMm(change_weld_throat_mm::ChangeWeldThroatMm),
    ChangeWeldLengthMm(change_weld_length_mm::ChangeWeldLengthMm),
    ChangeBetaW(change_beta_w::ChangeBetaW),
    ChangeSheetBMm(change_sheet_b_mm::ChangeSheetBMm),
    ChangeSheetTMm(change_sheet_t_mm::ChangeSheetTMm),
    ChangeSheetKSigma(change_sheet_k_sigma::ChangeSheetKSigma),
    ChangeSheetWElMm3(change_sheet_w_el_mm3::ChangeSheetWElMm3),
    ChangeSheetMEdKnm(change_sheet_m_ed_knm::ChangeSheetMEdKnm),
    ChangeShellTMm(change_shell_t_mm::ChangeShellTMm),
    ChangeShellRMm(change_shell_r_mm::ChangeShellRMm),
    ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa),
    ChangeAnnex(change_annex::ChangeAnnex),
}

/// 🏷️ Every declared kind of [`En1999Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🔣️oracle.json` publishes as the `en1999-1-any`
/// mutation catalog and `../../../../../🧪️tests/🪶️mutate-en1999-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-n-ed-kn",
    "change-m-ed-knm",
    "change-a-mm2",
    "change-w-el-mm3",
    "change-alloy",
    "change-chi",
    "change-it-mm4",
    "change-l-cr-mm",
    "change-theta-c",
    "change-delta-sigma-ed",
    "change-delta-sigma-c",
    "change-fatigue-m",
    "change-n-cycles",
    "change-v-weld-ed-kn",
    "change-weld-throat-mm",
    "change-weld-length-mm",
    "change-beta-w",
    "change-sheet-b-mm",
    "change-sheet-t-mm",
    "change-sheet-k-sigma",
    "change-sheet-w-el-mm3",
    "change-sheet-m-ed-knm",
    "change-shell-t-mm",
    "change-shell-r-mm",
    "change-sigma-ed-shell-mpa",
    "change-annex",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1999Mutation {
    /// 📤️ Decomposes a whole `En1999Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app command to
    /// bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1999Snapshot) -> Vec<En1999Mutation> {
        let mut mutations = Vec::with_capacity(26);
        mutations.push(En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: snapshot.n_ed_kn }));
        mutations.push(En1999Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: snapshot.m_ed_knm }));
        mutations.push(En1999Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: snapshot.a_mm2 }));
        mutations.push(En1999Mutation::ChangeWElMm3(change_w_el_mm3::ChangeWElMm3 { new_w_el_mm3: snapshot.w_el_mm3 }));
        mutations.push(En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: snapshot.alloy.clone() }));
        mutations.push(En1999Mutation::ChangeChi(change_chi::ChangeChi { new_chi: snapshot.chi }));
        mutations.push(En1999Mutation::ChangeITMm4(change_i_t_mm4::ChangeITMm4 { new_i_t_mm4: snapshot.i_t_mm4 }));
        mutations.push(En1999Mutation::ChangeLCrMm(change_l_cr_mm::ChangeLCrMm { new_l_cr_mm: snapshot.l_cr_mm }));
        mutations.push(En1999Mutation::ChangeThetaC(change_theta_c::ChangeThetaC { new_theta_c: snapshot.theta_c }));
        mutations.push(En1999Mutation::ChangeDeltaSigmaEd(change_delta_sigma_ed::ChangeDeltaSigmaEd { new_delta_sigma_ed: snapshot.delta_sigma_ed }));
        mutations.push(En1999Mutation::ChangeDeltaSigmaC(change_delta_sigma_c::ChangeDeltaSigmaC { new_delta_sigma_c: snapshot.delta_sigma_c }));
        mutations.push(En1999Mutation::ChangeFatigueM(change_fatigue_m::ChangeFatigueM { new_fatigue_m: snapshot.fatigue_m }));
        mutations.push(En1999Mutation::ChangeNCycles(change_n_cycles::ChangeNCycles { new_n_cycles: snapshot.n_cycles }));
        mutations.push(En1999Mutation::ChangeVWeldEdKn(change_v_weld_ed_kn::ChangeVWeldEdKn { new_v_weld_ed_kn: snapshot.v_weld_ed_kn }));
        mutations.push(En1999Mutation::ChangeWeldThroatMm(change_weld_throat_mm::ChangeWeldThroatMm { new_weld_throat_mm: snapshot.weld_throat_mm }));
        mutations.push(En1999Mutation::ChangeWeldLengthMm(change_weld_length_mm::ChangeWeldLengthMm { new_weld_length_mm: snapshot.weld_length_mm }));
        mutations.push(En1999Mutation::ChangeBetaW(change_beta_w::ChangeBetaW { new_beta_w: snapshot.beta_w }));
        mutations.push(En1999Mutation::ChangeSheetBMm(change_sheet_b_mm::ChangeSheetBMm { new_sheet_b_mm: snapshot.sheet_b_mm }));
        mutations.push(En1999Mutation::ChangeSheetTMm(change_sheet_t_mm::ChangeSheetTMm { new_sheet_t_mm: snapshot.sheet_t_mm }));
        mutations.push(En1999Mutation::ChangeSheetKSigma(change_sheet_k_sigma::ChangeSheetKSigma { new_sheet_k_sigma: snapshot.sheet_k_sigma }));
        mutations.push(En1999Mutation::ChangeSheetWElMm3(change_sheet_w_el_mm3::ChangeSheetWElMm3 { new_sheet_w_el_mm3: snapshot.sheet_w_el_mm3 }));
        mutations.push(En1999Mutation::ChangeSheetMEdKnm(change_sheet_m_ed_knm::ChangeSheetMEdKnm { new_sheet_m_ed_knm: snapshot.sheet_m_ed_knm }));
        mutations.push(En1999Mutation::ChangeShellTMm(change_shell_t_mm::ChangeShellTMm { new_shell_t_mm: snapshot.shell_t_mm }));
        mutations.push(En1999Mutation::ChangeShellRMm(change_shell_r_mm::ChangeShellRMm { new_shell_r_mm: snapshot.shell_r_mm }));
        mutations.push(En1999Mutation::ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: snapshot.sigma_ed_shell_mpa }));
        mutations.push(En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
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
/// 🧪️ Handcrafted mutation fixtures — one case per `change-*` leaf, each self-wired here so the
/// shared plugin-root `🦀️.rs` stays untouched while the other norm artifacts land theirs.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests


//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`En1999Mutation`]. The generated test host of
/// `../../../../../🧪️tests/🪶️mutate-en1999-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1999_mutation_json(text: &str) -> Result<En1999Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_en1999_mutation(base: &En1999Snapshot, mutation: &En1999Mutation) -> Result<(En1999Snapshot, Vec<String>), String> {
    let raised = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `🪶️mutate-en1999-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_en1999_mutation(mutation: &En1999Mutation, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    <En1999Mutation as protocol::Mutation<En1999Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
