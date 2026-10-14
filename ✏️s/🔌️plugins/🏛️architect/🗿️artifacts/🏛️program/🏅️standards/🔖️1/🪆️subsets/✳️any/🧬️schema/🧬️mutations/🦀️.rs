//! 🧬️ Architect program artifact — document mutation dispatch enum.
//!
//! Fourteen kinds. The 64 id-keyed registers share the four generic `📋️register-row` leaves
//! (`create`/`delete`/`rename`/`replace`), typed per register by the catalog generated from
//! the `x-semio-collection-catalog` of `📸️snapshot/🔣️.json` (`RegisterKey`, `RegisterRow`); the two edge registers (`adjacencies`,
//! `traces`) keep `connect`/`disconnect`, and the three document-level facets (`meta`/`project`/`governance`)
//! keep `rename`/`replace`.
//!
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ProgramSnapshot>` and
//! `impl protocol::SemanticMutation<ProgramSnapshot>` by delegating each variant to its payload's
//! `protocol::MutationKind` impl; this file is dispatch-only. Physical ownership is declared by the exact
//! `mutationDomainOwners` taxonomy registry: seven domains own the fourteen operation leaves.

use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

//#region 🔖️ProgramMutation
/// 🧩️ Every variant wraps exactly one `protocol::MutationKind<ProgramSnapshot, ProgramMutation>`
/// payload struct declared in its leaf's `🦀️.rs`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = ProgramSnapshot, diff = ProgramDiff, schema = "s.architect.program")]
pub enum ProgramMutation {
    CreateRegisterRow(super::create_register_row::CreateRegisterRow),
    DeleteRegisterRow(super::delete_register_row::DeleteRegisterRow),
    RenameRegisterRow(super::rename_register_row::RenameRegisterRow),
    ReplaceRegisterRow(super::replace_register_row::ReplaceRegisterRow),
    ConnectAdjacency(super::connect_adjacency::ConnectAdjacency),
    DisconnectAdjacency(super::disconnect_adjacency::DisconnectAdjacency),
    ConnectTrace(super::connect_trace::ConnectTrace),
    DisconnectTrace(super::disconnect_trace::DisconnectTrace),
    RenameMeta(super::rename_meta::RenameMeta),
    ReplaceMeta(super::replace_meta::ReplaceMeta),
    RenameProject(super::rename_project::RenameProject),
    ReplaceProject(super::replace_project::ReplaceProject),
    RenameGovernance(super::rename_governance::RenameGovernance),
    ReplaceGovernance(super::replace_governance::ReplaceGovernance),
}
//#endregion 🔖️ProgramMutation

//#region 🧫️FixtureTests
/// 🧫️ Handcrafted per-mutation fixture cases (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`): one
/// `🧪️tests/<case>/🦀️.rs` per edge or facet leaf, and the register-row corpus that walks every committed
/// `📋️register-row` case at run time.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
#[cfg(test)]
#[path = "📋️register-row/🧪️tests/🔬️corpus/🦀️.rs"]
mod register_row_corpus;
//#endregion 🧫️FixtureTests

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`ProgramMutation`] variant, in declaration order — the vocabulary the
/// `program-1-any` mutation catalog (`../../🔣️oracle.json`) declares and the
/// exhaustive `mutate-*` case measures itself against (the four generic register-row kinds, connect/disconnect over the two edge registers, rename/replace over the three facets). The framework never
/// parses Rust, so `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest
/// against both the enum and the committed catalog.
pub const KINDS: &[&str] = &[
    "create-register-row",
    "delete-register-row",
    "rename-register-row",
    "replace-register-row",
    "connect-adjacency",
    "disconnect-adjacency",
    "connect-trace",
    "disconnect-trace",
    "rename-meta",
    "replace-meta",
    "rename-project",
    "replace-project",
    "rename-governance",
    "replace-governance",
];

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_program_mutation_steps(mutation: &ProgramMutation, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok({
    <ProgramMutation as protocol::Mutation<ProgramSnapshot>>::inverse(mutation, base)?

    })
}






//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
