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
//! All twenty-two triads are mounted directly as `mutations`-sibling modules in `🦀️.rs` (this
//! lane's agent owns `🦀️.rs`, so no self-wiring `#[path = "."]` blocks are needed for the TRIADS — unlike
//! the wave-2 precedent in sibling facets that could not touch glue).

use crate::diff::En1996Diff;
use crate::En1996Snapshot;

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
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1996Snapshot, diff = En1996Diff, schema = "norm.en1996")]
pub enum En1996Mutation {
    ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm),
    ChangeNEdKn(change_n_ed_kn::ChangeNEdKn),
    ChangeVEdKn(change_v_ed_kn::ChangeVEdKn),
    ChangeHEdKn(change_h_ed_kn::ChangeHEdKn),
    ChangeZMm3(change_z_mm3::ChangeZMm3),
    ChangeAreaMm2(change_area_mm2::ChangeAreaMm2),
    ChangeShearAreaMm2(change_shear_area_mm2::ChangeShearAreaMm2),
    ChangeFKMpa(change_f_k_mpa::ChangeFKMpa),
    ChangeFVkMpa(change_f_vk_mpa::ChangeFVkMpa),
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeMasonryClass(change_masonry_class::ChangeMasonryClass),
    ChangeDesignSituation(change_design_situation::ChangeDesignSituation),
    ChangeMu(change_mu::ChangeMu),
    ChangeWallThicknessMm(change_wall_thickness_mm::ChangeWallThicknessMm),
    ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin),
    ChangeUnit(change_unit::ChangeUnit),
    ChangeExposure(change_exposure::ChangeExposure),
    ChangeMortar(change_mortar::ChangeMortar),
    ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::ChangeBedJointThicknessMm),
    ChangeStoreys(change_storeys::ChangeStoreys),
    ChangeHEfMm(change_h_ef_mm::ChangeHEfMm),
    ChangeTEfMm(change_t_ef_mm::ChangeTEfMm),
}

/// 🏷️ Every declared kind of [`En1996Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🔣️oracle.json` publishes as the `en1996-1-any`
/// mutation catalog and `../../../../../🧪️tests/🪨️mutate-en1996-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-m-ed-knm",
    "change-n-ed-kn",
    "change-v-ed-kn",
    "change-h-ed-kn",
    "change-z-mm3",
    "change-area-mm2",
    "change-shear-area-mm2",
    "change-fk-mpa",
    "change-f-vk-mpa",
    "change-annex",
    "change-masonry-class",
    "change-design-situation",
    "change-mu",
    "change-wall-thickness-mm",
    "change-fire-resistance-min",
    "change-unit",
    "change-exposure",
    "change-mortar",
    "change-bed-joint-thickness-mm",
    "change-storeys",
    "change-h-ef-mm",
    "change-t-ef-mm",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1996Mutation {
    /// 📤️ Decomposes a whole `En1996Snapshot` into one `change-<field>` mutation per
    /// persistent field — the closed-vocabulary replacement for the banned whole-document-replace
    /// variant, used by `import_media`'s `"model:in"` port and the `set-snapshot` app
    /// command to bundle a bulk document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &En1996Snapshot) -> Vec<En1996Mutation> {
        let mut mutations = Vec::with_capacity(22);
        mutations.push(En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: snapshot.m_ed_knm }));
        mutations.push(En1996Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: snapshot.n_ed_kn }));
        mutations.push(En1996Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn }));
        mutations.push(En1996Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn: snapshot.h_ed_kn }));
        mutations.push(En1996Mutation::ChangeZMm3(change_z_mm3::ChangeZMm3 { new_z_mm3: snapshot.z_mm3 }));
        mutations.push(En1996Mutation::ChangeAreaMm2(change_area_mm2::ChangeAreaMm2 { new_area_mm2: snapshot.area_mm2 }));
        mutations.push(En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::ChangeShearAreaMm2 { new_shear_area_mm2: snapshot.shear_area_mm2 }));
        mutations.push(En1996Mutation::ChangeFKMpa(change_f_k_mpa::ChangeFKMpa { new_f_k_mpa: snapshot.f_k_mpa }));
        mutations.push(En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::ChangeFVkMpa { new_f_vk_mpa: snapshot.f_vk_mpa }));
        mutations.push(En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
        mutations.push(En1996Mutation::ChangeMasonryClass(change_masonry_class::ChangeMasonryClass { new_masonry_class: snapshot.masonry_class }));
        mutations.push(En1996Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: snapshot.design_situation }));
        mutations.push(En1996Mutation::ChangeMu(change_mu::ChangeMu { new_mu: snapshot.mu }));
        mutations.push(En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::ChangeWallThicknessMm { new_wall_thickness_mm: snapshot.wall_thickness_mm }));
        mutations.push(En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin { new_fire_resistance_min: snapshot.fire_resistance_min }));
        mutations.push(En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit: snapshot.unit.clone() }));
        mutations.push(En1996Mutation::ChangeExposure(change_exposure::ChangeExposure { new_exposure: snapshot.exposure }));
        mutations.push(En1996Mutation::ChangeMortar(change_mortar::ChangeMortar { new_mortar: snapshot.mortar }));
        mutations.push(En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: snapshot.bed_joint_thickness_mm }));
        mutations.push(En1996Mutation::ChangeStoreys(change_storeys::ChangeStoreys { new_storeys: snapshot.storeys }));
        mutations.push(En1996Mutation::ChangeHEfMm(change_h_ef_mm::ChangeHEfMm { new_h_ef_mm: snapshot.h_ef_mm }));
        mutations.push(En1996Mutation::ChangeTEfMm(change_t_ef_mm::ChangeTEfMm { new_t_ef_mm: snapshot.t_ef_mm }));
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
/// specification vectors carry — into a real [`En1996Mutation`]. The generated test host of
/// `../../../../../🧪️tests/🪨️mutate-en1996-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1996_mutation_json(text: &str) -> Result<En1996Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_en1996_mutation(base: &En1996Snapshot, mutation: &En1996Mutation) -> Result<(En1996Snapshot, Vec<String>), String> {
    let raised = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1996Diff as protocol::MutationDiff<En1996Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `🪨️mutate-en1996-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_en1996_mutation(mutation: &En1996Mutation, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
