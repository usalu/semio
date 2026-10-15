# Seam Map: hub and repo surface monoliths

Five Rust monoliths, one section each: A hub directory, B hub inference runtime, C repo CLI, D tiled-map surface, E repo GraphQL. Format and depth follow `SEAM-MAP-SHELL-WGPU.md` (same ticket folder).

Mode: read-only audit. No source file was edited. No git, cargo, nx or build was run. Searches used `/usr/bin/grep` with fixed strings. Line numbers refer to the files as audited (mtimes Oct 7 to Oct 14); re-verify the anchor lines before each cut.

Counting: "active" = non-blank line that does not start with `//` (after leading whitespace). This is the same proxy used in the earlier seam maps. Raw line counts are given too. Sizes per child are active lines unless marked raw. The 4000-active gate and the 2500 child target are checked against these numbers. Measured figures are stated as measured. Figures marked "not measured" were not counted in this audit and must be produced by the executor.

Scope limits of the reader search: `🌎️hub` and the root `📜️script.ts` were searched fully for A and B. `🧰️framework` and `✏️s` were searched for C, D and E (fixed-string tails). `🧰️framework` and `✏️s` were searched for A and B only with a narrow pattern (see section 10). Each `grep` over `✏️s` took about 5 to 10 minutes, so coverage is per pattern, not per repo.

## 0. Summary

| Id | Monolith (path tail) | Raw / active | Owning crate and mount | Proposed children | Main finding |
|---|---|---|---|---|---|
| A | `🌎️hub/📇️directory/🦀️.rs` | 4,355 / 3,488 | lib `semio-hub` (`#[path]` in `🌎️hub/📦️packages/🦀️rust/🦀️.rs:13`), also in bin `os-hub` tests | 7 files, largest 742 active | Directory depends on `artifact_authority` (52 `crate::artifact_authority` paths) while `artifact_authority` names a few `directory` items back, a module cycle. Text gates in `📜️script.ts` (three reads) and one `.ts` test read this file and brace-match fn bodies. |
| B | `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | 3,956 / ~3,581 | lib `semio-hub` (`💡️inference/🦀️.rs:16`), bin consumes 14 symbols | 10 files, largest 842 active | Hub inference runtime is GIS-only (`gis_map` names, `semio_s_artifact_gis_gismap` types, GIS job commit). One impl block of ~1,885 active lines must be cut at method boundaries. |
| C | `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🦀️.rs` | 4,298 / 3,768 | lib `semio-framework-repo-cli` (`[lib] path = "../../🦀️.rs"`), bins `semio-repo` and `repo` | 6 children under a facade, plus `🔌️mcp-verb`, largest 1,353 (verbs) | Clean seams except one cycle: `verbs::mcp` uses `mcp_verb`, and `mcp_verb` calls `repo_cli::format_markdown_result`. Fix by moving that function into the renderers child. |
| D | `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs` | 4,187 / ~3,615 | lib `semio-framework-surface` (`🗺️surface/📦️packages/🦀️rust/🦀️.rs:14`) | 9 children, largest ~790 (scene) | Framework tiled-map embeds GIS semantics (LOD tables, `gis_map_*` names, OSM class ranks). Third-party `image` crate used at runtime (line 2671) although `semio-framework-pixels` is already a dependency. |
| E | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/🦀️.rs` | 5,038 / 4,250 | lib `semio-framework-repo-graphql` (`Cargo.toml` `[lib] path`) | 7 children plus one new adapter crate, largest 1,014 (schema) | Wrong direction. The grammar and executor crate carries `FsRepoContext` (958 active) and provider wiring. Measured: the core region (lines 1-3955) uses only 2 of the 13 repo-domain crates it depends on (`identity`, `model`). |

## 1. A: hub directory (`🌎️hub/📇️directory/🦀️.rs`)

### 1.1 Owning crate, targets, mounting

- Lib `semio-hub` (`🌎️hub/📦️packages/🦀️rust/Cargo.toml`, `[lib]` root `🌎️hub/📦️packages/🦀️rust/🦀️.rs`). Mount: `🌎️hub/📦️packages/🦀️rust/🦀️.rs:13-14` `#[path = "../../📇️directory/🦀️.rs"] pub mod directory;`. Public path `semio_hub::directory::*` must stay.
- Bin `os-hub` (`🏗️bootstrap/🦀️.rs`). It does not mount this file. It consumes `semio_hub::directory::{DirectoryService, HubDirectory, HubVerifiedCheckpointPublisher, ProjectionRebuildControl, ..., HubDirectories, ...}` (bootstrap lines 30, 93-99) and `semio_hub::directory::sqlite::SqliteDirectory` (line 97).
- Test seams: `#[cfg(test)] #[path = "🧪️tests/🔮️backend-corpus/🦀️.rs"] pub(crate) mod backend_corpus;` (line 4349) and `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` (line 4353, 1,594 active). Both are mounted from this file, so their paths stay correct if the parent stays at this folder.
- Lib-root test oracle: `🌎️hub/📦️packages/🦀️rust/🦀️.rs:57` mounts `../../📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs` (not affected).
- Backend children already mounted from this file: `#[path = "🪶️sqlite/🦀️.rs"] pub mod sqlite;` (3532), `🐘️postgres` (3535), `🌐️neo4j` (3538). Raw sizes 3,174 / 3,460 / 3,522 lines, active 2,961 / 3,245 / 3,295. All three already exceed the 2,500 child target. They are out of this monolith's scope but must be split in a later ticket (see section 12).

### 1.2 Section table (inclusive ranges; "Active" measured)

| Lines | Content | Active | Goes to |
|---|---|---|---|
| 1-15 | `//!` file doc | 0 | P |
| 16-43 | `pub mod error { … }` inline (`DirectoryError`, `DirectoryResult`) | 21 | P (stays inline) |
| 44-675 | `pub mod model { … }` inline (DTOs: `UserRecord`, `SpaceRecord`, `SpaceRole`, `AuthSessionRecord`, `InviteRecord`, `AgentDelegationRecord`, `DirectoryCommandReceiptRecord`, admin operation records, `AdminDirectoryOverviewCounts`) | 465 | A01 |
| 676-813 | shared `use` block (677-687), consts (689-723), `ArtifactCasSweep*` structs (727-771), `ProjectionRebuildControl` trait (773-793), `UNCONTROLLED_PROJECTION_REBUILD` static (794), `checkpoint_projection_rebuild` and `bounded_event_read` (796-812) | 100 | P |
| 814-1621 | `//#region Capabilities`: `capability_type!` macro (985-1019), `CapabilityKind`, `HubCapability`, identity assertion and verification types (1075-1143), local bootstrap types and transports (1145-1226), agent and invite preflight (1303-1439), audit and command validators (1462-1619) | 673 | A02 |
| 1623-2049 | `//#region Wire`: wire role/kind conversion, `validate_document_descriptor`, checkpoint validation and lineage, `validate_document_genesis_append_v1`, `document_index_projection_v1`, `MemoryArtifactProjection` impl (1953-2049) | 351 | A03 |
| 2051-2341 | `//#region Events` (2051-2100: `now_ms`, `HubClock`, `NewDirectoryEvent`) and `//#region Decider` (2102-2341: `CommandResult`, `Decision`, `decide`, `decide_artifact_authority`, `decide_verified_checkpoint`) | 229 | A04 |
| 2343-3201 | `//#region Service`: `#[cfg(test)]` `DirectoryWriterTestFence` (2344-2359), `DirectoryProjectionRejectionV1`, `DirectoryService` struct and `impl DirectoryService` (2471-3145), `DirectoryDeliveryLeaseV1`, `HubVerifiedCheckpointPublisher` (3148-3189), `capped_socket_session_binding` (3194-3200) | 742 | A05 |
| 3202-3525 | `//#region Trait`: `pub trait HubDirectory: Send + Sync + 'static` (3208-3524, about 90 async methods) | 192 | A06 |
| 3526-3541 | `//#region Backends` mounts for sqlite/postgres/neo4j (3532-3539) | 6 | P (stays) |
| 3542-4341 | `//#region Dispatch`: `pub enum HubDirectories` (3551), `From` impls (3557-3571), `impl HubDirectory for HubDirectories` (3575-4340) | 696 | A07 |
| 4342-4355 | `//#region Tests` mounts | 6 | P |

Sum of children A01 to A07 is 3,348 active. Parent after split is about 140 active (error 21, shared 100, mounts and tests 12, docs 0).

### 1.3 Proposed children (folders under `📇️directory/`)

Names checked against existing siblings `🌐️neo4j`, `🐘️postgres`, `📣️publication`, `🔐️authorization`, `🧬️schema`, `🪶️sqlite`, `🧫️fixtures`, `🧪️tests`. No duplicates. All folder emojis RGI with U+FE0F. Longest proposed path is 86 bytes (`🌎️hub/📇️directory/🎟️capabilities/🦀️.rs`, measured with the prefix).

| Id | Folder (`📇️directory/<folder>/🦀️.rs`) | Mod name | Content lines | Active | Contents |
|---|---|---|---|---|---|
| A01 | `🧾️model` | model | 47-673 (inline body) | 465 | DTOs. The inline `pub mod model {` (46) and its `}` (674) are removed; body becomes the file. |
| A02 | `🎟️capabilities` | capabilities | 815-1620 | 673 | Capability macro and types, identity, local bootstrap, agent and invite preflight, audit validators. Macro `capability_type!` and its 4 uses stay in this file. |
| A03 | `🔤️wire` | wire | 1624-2048 | 351 | Wire conversion and checkpoint validation. |
| A04 | `🧭️decider` | decider | 2052-2340 | 229 | Events (`HubClock`, `NewDirectoryEvent`) and decider functions. |
| A05 | `🛠️service` | service | 2344-3201 | 742 | `DirectoryService`, publisher adapter, test fence, socket binding helper. Not split further (742 is well under 2,500). |
| A06 | `🔌️port` | port | 3203-3524 | 192 | The `HubDirectory` trait (port). |
| A07 | `🔀️dispatch` | dispatch | 3543-4340 | 696 | `HubDirectories` enum and its delegating impl over sqlite, postgres and neo4j. |

Parent `📇️directory/🦀️.rs` after split: docs (1-15), inline `error` module (16-43), shared block (676-813), backend mounts (3526-3541), test mounts (4342-4355), and wiring. The wiring is `#[path = "🧾️model/🦀️.rs"] pub mod model;` and the same for each child, plus `pub(crate) use <child>::*;` for children whose names the tests and the bin reach through `super::*`.

### 1.4 Coupling and visibility

- Dependencies between children (direction): decider uses model, capabilities, wire and service's `HubDirectories`. Service uses decider, wire, capabilities, model and `HubDirectory` (port). Dispatch uses port, service's backends and the backend modules. Port uses model and capabilities. Model uses only the external `os_directory` crate and `artifact_authority::chunk_cas` (line 677).
- Cycle to remove before any crate split: `directory` to `artifact_authority`. Measured: 52 lines in this file use `crate::artifact_authority::…`. The heaviest are `AuthorityError::Store` (13), `adapters::bounded_message` (11), `AuthorityError` (4), `OperationContext` (3), `creation::io::…` (4 total), `chunk_cas::…` (4 total), `AuthorityProgress` (2) and `VerifiedCheckpointPublisher` (1). In the other direction, `artifact_authority` names `crate::directory::error::DirectoryResult` (in `🗿️artifact-authority/🌱️creation/🦀️.rs`) and `crate::directory::{MemoryArtifactProjection, NewDirectoryEvent, ARTIFACT_CAS_SWEEP_*}` in the creation and CAS test files. The back-edges are small. The forward edges are large.
- Parent-owned items reached from children: `error` module (every child), `model` (every child), `ARTIFACT_CAS_*` consts and `ArtifactCasSweep*` (service, port). The parent keeps these and children use `super::{error, model}`. No change beyond the import lines.
- Visibility rule: the children are sibling modules under `directory`. A private item in one child is invisible to its siblings. Items used across children get `pub(super)`, which is visible to `directory` and all its descendants (siblings included). Use `pub(crate)` only for items that other crates or the bin reach. Measured: the file has 44 `pub(crate)` occurrences and 0 `pub(super)`. Per-name cross-region counts are not measured here.
- Struct fields: `HubClock` and the `DirectoryService` fields (`dir`, `channel`, delivery state) are read from the decider and service. Under `pub(super)` rule they widen to `pub(super)`. Not measured per field.
- Trait method names: `HubDirectory` methods are called through `HubDirectories` in dispatch. Trait stays in one child (port) and is re-exported, so trait-in-scope for method calls must be imported explicitly in dispatch, service and tests (`use super::port::HubDirectory;`). A missing import shows as E0599. Glob re-export of the port fixes this.
- `#[cfg(test)]` items: `DirectoryWriterTestFence` (2344-2359), its arm/pause methods on `DirectoryService` (2599-2653), and test-only fields. They stay in `service`. Tests (`tests` at 4353) reach them through the parent glob, so `service` re-exports them `pub(crate)` under `#[cfg(test)]`.

### 1.5 Paths, include, macros

- `#[path]` attributes inside the file: backends at 3532, 3535, 3538 and tests at 4349, 4353. Only the backend and test mounts exist. They stay in the parent. No path rewrite is needed for children unless a child mounts its own file.
- `include!`: none in this file. `include_str!`: none.
- `macro_rules! capability_type` (985-1019): defined and used only in the `capabilities` child (uses at 1020-1024). Macro textual scope means it must stay in that child, before its uses, and not be re-declared elsewhere.
- Relative `crate::` paths: `crate::artifact_authority` (52, see 1.4) and `crate::directory::error` in the backends. After the split `crate::directory::…` paths still resolve, because the module path is unchanged. `crate::directory::service::…` appears only if a child is named directly; keep paths through `directory::`.
- The `use directory::os_directory::…` lines (extern crate `directory`, the os-kernel crate) are absolute and do not change.

### 1.6 Test modules

- `tests` (`🧪️tests/🔬️unit/🦀️.rs`, 1,751 raw, 1,594 active) and `backend_corpus` (`🧪️tests/🔮️backend-corpus/🦀️.rs`, 129 active) stay mounted from the parent. They reach items by `use super::*`. Requirement: the parent glob-reexports every child name the tests use. The executor should build the set by compiling the tests after step 1.
- Bin tests (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`) refer to this file in a doc comment only (line 1053). No code path.

### 1.7 Source-text readers (must be updated with the split)

| Reader | Lines | What it does |
|---|---|---|
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 3025-3028 | Reads `📇️directory/🦀️.rs` and the sqlite, postgres, neo4j files; gate checks text. |
| same | 16613, 16616-16618 | Reads the same four files; brace-matches fn bodies by signature. |
| same | 17083-17085 | Reads this file; checks `pub fn validate_document_descriptor` and the frontier comparison string. |
| `🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts` | 30-33 | Reads this file and brace-matches `fn <name>(` bodies (for `validate_directory_event_page_event` style checks). |

Impact: the gates in `script.ts` must read the directory folder as one text, for example by concatenating `📇️directory/🦀️.rs` with `📇️directory/*/🦀️.rs` (same change the hub bootstrap split needs). The brace matcher in `ordered-append-broadcast` must search the concatenation. Both are declared, not discovered, so update them in the same step as the split.

Framework-side readers for this path were not searched (section 10).

### 1.8 Wrong-direction dependencies

1. `directory` to `artifact_authority` (cycle, see 1.4). Proposed direction: `artifact_authority` owns CAS, creation and checkpoint verification and exposes ports. `directory` depends on those ports only. The CAS sweep (`ArtifactCasSweep*`, `sweep_artifact_cas`, `reserve_artifact_cas`) and the creation service methods belong to artifact authority.
2. `DirectoryService` owns artifact CAS sweeping (2653-2690, 2967-3060). Same owner as in point 1.
3. The GIS commit path (`🌎️hub/💡️inference/🏃️runtime`, see B) reaches `HubDirectory` and `directory::os_store`. This is the reverse of intended layering: GIS depends on directory, not the reverse. Only the B side needs the port.
4. The backends are driver-specific (sqlx, neo4rs, rusqlite) but reached through `HubDirectories` in `dispatch`. Acceptable, since dispatch is the composition root, but dispatch should move to the composition root crate (`os-hub`) in a later step.

### 1.9 Extraction sequence (bottom-up, so earlier line numbers stay valid)

| Step | Child | Source range (content, markers excluded) | Anchor at cut |
|---|---|---|---|
| 1 | A07 `🔀️dispatch` | 3543-4340 | 3542 `//#region Dispatch` (remove), 4341 `//#endregion` (remove) |
| 2 | A06 `🔌️port` | 3203-3524 | 3202 `//#region Trait` (remove), 3525 `//#endregion` (remove) |
| 3 | A05 `🛠️service` | 2344-3201 | 2343 `//#region Service`, 3190 `//#endregion`, gap 3191-3201 is included |
| 4 | A04 `🧭️decider` | 2052-2340 | 2051 `//#region Events`, 2341 `//#endregion Decider` |
| 5 | A03 `🔤️wire` | 1624-2048 | 1623 `//#region Wire`, 2049 `//#endregion` |
| 6 | A02 `🎟️capabilities` | 815-1620 | 814 `//#region Capabilities`, 1621 `//#endregion` |
| 7 | A01 `🧾️model` | 47-673 | 46 `pub mod model {`, 674 `}` |
| 8 | parent | keep 1-45, 676-813, 2050 doc gap, 3526-3541, 4342-4355, plus wiring | verify `cargo` (see section 7) |

Each step: create `📇️directory/<folder>/🦀️.rs` with a `//!` doc starting with an emoji, the `use` preamble needed (copy from the parent, prune later), body, and `pub(super)` on cross-child items. Then wire the parent. Then rebuild the lib target. Keep a copy of the original file outside the repo (`🌎️hub/📇️directory/` must not hold `.orig` files).

## 2. B: hub inference runtime (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`)

### 2.1 Owning crate, targets, mounting

- Lib `semio-hub`. Mount: `🌎️hub/💡️inference/🦀️.rs:16-17` `#[path = "🏃️runtime/🦀️.rs"] pub mod runtime;`. Public path `semio_hub::inference::runtime::*` is used by the bin.
- Bin `os-hub` (`🏗️bootstrap/🦀️.rs`) consumes through `semio_hub::inference::runtime::{…}` (lines 32-33, 101-103): `GisMapApprovalIngressAuthorityV1` (9 uses, bootstrap), `InferenceRouteContextV1` (2), `InferenceCheckpointTestGateV1`, `GisMapApprovalCheckpointPublisherV1`, `HubInferenceRuntimeV1`, `RetainedGisMapApprovalCommitterV1`, `UnavailableGisMapApprovalCommitterV1`, `InferenceRouteErrorV1`, `GisMapApprovalCommitErrorV1`, and the route functions `submit_gis_map_job`, `reconcile_gis_map_job`, `read_gis_map_job_events`, `cancel_gis_map_job`, `approve_gis_map_job`, `undo_gis_map_approval`.
- Test seams: `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` at 3954-3956 (1,541 raw lines). The test file uses `super::*` and reaches `static LAST_*` recorders (for example `last_gis_map_commit_conflict_arm` at 226, `last_close_store_step` at 327).
- Feature gate: `#[cfg(feature = "integration-fixtures")]` on the checkpoint gate (2912-3066, 6 items). The bin's `os-hub` tests need this feature, so the cfg must stay on the same items.

### 2.2 Section table

| Lines | Content | Active | Goes to |
|---|---|---|---|
| 1-35 | imports (1-25), `pub const OPERATION_CAPACITY`, `DOCUMENT_GATE_CAPACITY`, `WORK_UNIT_LIMIT`, `ALLOCATION_BYTES`, `RECURSION_DEPTH`, `APPROVAL_MAX_RECORDS`, `GIS_MAP_COMMITTER_CAPACITY` (28-34) | 27 | P (parent) |
| 36-504 | `InferenceRouteErrorV1` (38-99), `GisMapApprovalCommitRequestV1`, undo request, `GisMapApprovalIngressAuthorityV1` trait (144), `GisMapDocumentWriteAuthorityV1`, `GisMapApprovalCommitErrorV1`, `GisMapCommitConflictArmV1`, the `LAST_*` test recorders and their helpers (211-334), receipts and checkpoint publisher trait (344-430), GIS type aliases (450-462) | 343 | B01 `🧾️approval-contract` |
| 505-1427 | `RetainedGisMapDocumentStateV1` enum (505), commit turn enums (567, 575), `GisMapPreparedApprovalV1` and request owner structs with `Drop` (587-640), `RetainedGisMapApprovalCommitterV1` struct (641-662), closing cursor and maintenance structs (663-767), `impl RetainedGisMapApprovalCommitterV1` part A (768-1426: new, reserve, validate, park, spawn, resume, preflight, reserve_operations, mount_document, finish_document_recovery, resume_recovered_checkpoint) | 842 | B02 `🔐️committer` |
| 1428-1711 | `impl` part B: `prepare_retained_document` (1428), `build_assembly` (1604), `restored_stores` (1699) | 281 | B03 `🧱️document-preparation` |
| 1712-2110 | `impl` part C: `drive_turn` (1712), `request_matches`, `state_has_active_request`, `drive_abandoned_turn` (1880), `drive_abandoned_request` (2058) | 394 | B04 `🔁️commit-turns` |
| 2111-2354 | `impl` part D: `finish_preflight` (2111), `publication_frontier`, `publication_wire_frontier`, `publish_checkpoint` (2168), `verify` (2238) | 239 | B05 `🔎️checkpoint-verify` |
| 2355-2720 | `impl` part E: `commit_retained` (2355), `commit_undo_retained` (2402), `closing`, `close_store`, `close_turn`, `close` (2611) ends at 2720 | 360 | B06 `📕️commit-and-close` |
| 2722-2779 | `impl GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1` | 54 | B07 `🧭️job-types-and-port` |
| 2780-2911 | `InferenceMapBaseV1`, `InferenceProposalV1`, `InferenceRunKeyV1`, retained run supervisor and owner | 109 | B07 |
| 2912-3066 | `#[cfg(feature = "integration-fixtures")]` checkpoint gate, control frames, inherited descriptor reader | 139 | B08 `🧪️checkpoint-test-gate` |
| 3067-3364 | `HubInferenceRuntimeV1` struct (3067) and `impl` (3077-3364: `new`, binding, ledger, committer, install gate, operations, `infer`, `commit_approval`, `commit_undo`) | 266 | B09 `🏭️inference-runtime` |
| 3365-3455 | free fns: `deterministic_map_inference` (3365, GIS-specific), `compare_frozen_identity` (3388), `commit_prepared_approval` (3408) | 86 | B09 |
| 3456-3896 | `//#region Routes`: `InferenceRouteContextV1` (3458), `InferenceApprovalRouteContextV1` (3469), route handlers `submit_gis_map_job` (3638), `reconcile_gis_map_job` (3710), `read_gis_map_job_events` (3729), `cancel_gis_map_job` (3765), `approve_gis_map_job` (3795), `undo_gis_map_approval` (3837) | 405 | B10 `🛣️routes` |
| 3897-3956 | `//#endregion Routes`, helpers `approval_stamp_v1`, `document_key`, `fresh_now_ms`, `parse_frontier_chain_hash`, `approval_actor`, `approval_mutation_id`, `reader`, then test mount | 36 | P (parent, used by children) |

Sum of children B01 to B10 is 3,518 active. Parent after split: 27 + 36 + mounts = about 65 active.

### 2.3 Proposed children (folders under `🏃️runtime/`)

Existing siblings under `🏃️runtime/`: only `🧪️tests`. Names are unique. Children nest under `runtime`, so private items of a parent-level struct are visible to them. Siblings still need `pub(super)`.

| Id | Folder | Content (lines, markers excluded) | Active |
|---|---|---|---|
| B01 | `🧾️approval-contract` | 37-504 | 343 |
| B02 | `🔐️committer` | 505-1426 | 842 |
| B03 | `🧱️document-preparation` | 1428-1710 (`impl` opener and closer) | 281 |
| B04 | `🔁️commit-turns` | 1712-2109 | 394 |
| B05 | `🔎️checkpoint-verify` | 2111-2353 | 239 |
| B06 | `📕️commit-and-close` | 2355-2719 | 360 |
| B07 | `🧭️job-types-and-port` | 2722-2911 (trait impl plus job types) | 163 |
| B08 | `🧪️checkpoint-test-gate` | 2912-3066 (keep `#[cfg(feature = "integration-fixtures")]`) | 139 |
| B09 | `🏭️inference-runtime` | 3067-3455 | 352 |
| B10 | `🛣️routes` | 3457-3896 | 405 |

The folder names `🧾️approval-contract` and `🧭️job-types-and-port` are long (`🧭️job-types-and-port` is 25 characters). Measured with the full prefix: the longest B path is 128 bytes (`🌎️hub/💡️inference/🏃️runtime/🔐️committer/🧩️preflight-and-identity/🦀️.rs`, a name used only as an example; the proposed names are shorter).

Parent `🏃️runtime/🦀️.rs` after split keeps: consts and imports (1-35), helpers (3897-3956), mounts for B01-B10, and `pub use` re-exports for every name the bin imports (see 2.1). Each `impl RetainedGisMapApprovalCommitterV1` block is opened and closed inside its own child.

### 2.4 Coupling and visibility

- `RetainedGisMapApprovalCommitterV1` has about 60 methods and private fields. Children B03 to B06 are sibling modules under `runtime`, so every method and field they touch must be `pub(super)`. Measured: the `impl` parts already call each other by name (for example `drive_turn` from `commit_retained`). Method-level counts are not measured here; the blanket rule in the exemplar (4.1) applies: prefix private `fn` and fields of this struct with `pub(super)`.
- `Drop` impls (627, 675, 731, 760) and their structs must stay in the same child as the struct. Keep `committer` (B02) as the single owner of guards.
- Process-global `static LAST_*` recorders (211-334, `#[cfg(test)]` where tests read them) stay in B01. Other children call their `record_*` functions through `super::`. Tests read them through the parent glob.
- Route handlers (B10) call runtime methods (`commit_approval`, `infer`) and the free fns in B09 through `super::`. This is a dependency of routes on runtime, which is the right direction.
- The integration gate (B08) is used only by `HubInferenceRuntimeV1::install_checkpoint_test_gate` and the bin. The `cfg` attribute must appear on every item that the bin references.

### 2.5 Paths, include, macros

- `#[path]` in this file: only the test mount at 3955. Stays in parent. No child mounts a file.
- `include!`, `include_str!`: none.
- `macro_rules!`: none (measured).
- `use super::catalog::…`, `super::command::…`, `super::schema::…`, `super::sqlite::…`, `super::wal::…`, `super::sha256`, `super::InferenceErrorV1`: under `inference/`. These are `super::super::…` in a child of `runtime`. Rewrite is the only change: each child file uses `super::super::…` for `inference` siblings, or imports from the parent via `use super::super::{catalog, command, schema, sqlite, wal};`. Count of such lines: not measured, but every child's preamble changes.
- `crate::directory::…`, `crate::lag_rebootstrap::…` and `crate::directory::HubDirectories`: absolute, unchanged.
- `directory::os_store::…` and `db::…`: extern crate paths, unchanged.

### 2.6 Test modules

- `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` (1,541 raw). Mounted at 3955 from the parent. Uses `super::*`. After the split, names come through the parent glob. Verify by compiling the test target (section 7).

### 2.7 Source-text readers

- Hub-wide search (`🌎️hub` plus root `📜️script.ts`) for `🏃️runtime/🦀️` and `inference/🏃️runtime`: one hit only, the mount at `💡️inference/🦀️.rs:16`. No text gate reads this file.
- Not searched: `🧰️framework` and `✏️s` for this pattern (section 10).
- The bin's bootstrap readers (`🏗️bootstrap/📜️script.ts`, mentioned in the hub bootstrap seam) read the bootstrap file, not this one.

### 2.8 Wrong-direction dependencies (GIS in hub)

1. The whole runtime is GIS-specific: type aliases at 450-462 name `semio_s_artifact_gis_gismap::GisMapSnapshot`, `semio_s_artifact_stdio_semio_drawing`, `semio_s_artifact_stdio_semio_value_model`. Three-store assembly (`GisMapParentStoreV1`, drawing, value), `GisMapCommitIdentityV1`, `deterministic_map_inference` (3365, GIS output), and `gis_map_inference_preview` (3474). The hub, which is an OS-level service, should not carry this domain.
2. Proposed direction: the hub keeps the generic parts (operation registry, document gate, retained owner, journal, route plumbing: B04 turn driver, B06 close, B08 gate, B09 runtime, B10 routes) under neutral names (`inference` not `gis_map`). The GIS-specific parts (B01 contract types, B02 commit steps that build GIS stores, `deterministic_map_inference`, `gis_map_inference_preview`) move to a hub-side GIS plugin crate that implements the existing ports `GisMapApprovalCommitterV1` and `GisMapApprovalIngressAuthorityV1`. The bin then registers the plugin.
3. The port already exists: `UnavailableGisMapApprovalCommitterV1` (430) and `GisMapApprovalIngressAuthorityV1` (144) show the intended direction. Use them as the boundary.
4. `db::Database`, `db::storage::DbBackend` and `db::document::ArtifactDurableGroupRecoveryOwnerV1` (lines 456-461 and 769) are used directly. The runtime depends on the db crate's concrete backend. Proposed: a port in `directory` or `artifact_authority`.

### 2.9 Extraction sequence (bottom-up)

| Step | Child | Range | Anchor |
|---|---|---|---|
| 1 | B10 `🛣️routes` | 3457-3896 | 3456 `//#region Routes`, 3897 `//#endregion` |
| 2 | B09 `🏭️inference-runtime` | 3067-3455 | 3065-3066 gate ends, 3067 struct |
| 3 | B08 `🧪️checkpoint-test-gate` | 2912-3066 | cfg feature on each item |
| 4 | B07 `🧭️job-types-and-port` | 2722-2911 | trait impl opener at 2722 |
| 5 | B06 `📕️commit-and-close` | 2355-2719 | 2354 blank; prepend `impl RetainedGisMapApprovalCommitterV1 {` and close with `}` |
| 6 | B05 `🔎️checkpoint-verify` | 2111-2353 | 2110 blank |
| 7 | B04 `🔁️commit-turns` | 1712-2109 | 1711 blank |
| 8 | B03 `🧱️document-preparation` | 1428-1710 | 1427 blank |
| 9 | B02 `🔐️committer` | 505-1426 | 504 blank; prepend the `impl` opener at 768 is inside the range |
| 10 | B01 `🧾️approval-contract` | 37-504 | 36 blank, `use` block stays in parent |

Each `impl` cut must close the upper child with `}` and reopen with `impl RetainedGisMapApprovalCommitterV1 {`. Cut lines verified as blank (504, 1427, 1711, 2110, 2354, 2609-2610 before `close` doc at 2610, 2720 closing brace). Re-verify before cutting.

## 3. C: repo CLI (`🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🦀️.rs`)

### 3.1 Owning crate, targets, mounting

- Lib `semio-framework-repo-cli` (`⌨️cli/📦️packages/🦀️rust/Cargo.toml`, `[lib] path = "../../🦀️.rs"`). The monolith is the lib root.
- Bin `semio-repo` (`📦️packages/🦀️rust/📦️cli-main.rs`): `semio_framework_repo_cli::repo_cli::run(&argv)`.
- Bin `repo` (`📦️packages/🦀️rust/📦️mcp-main.rs`): `semio_framework_repo_cli::mcp_verb::real_repository(root)` and `semio_framework_repo_mcp::run_with`.
- The lib root contains two inline modules: `pub mod repo_cli { … }` (14-3971) and `pub mod mcp_verb { … }` (3981-4297). `pub mod verbs { … }` is inline inside `repo_cli` (2282-3746).
- Test adapters: 8 files under `⌨️cli/🧪️tests/*/🦀️.rs` import `semio_framework_repo_cli::repo_cli` (measured). They are not declared in `Cargo.toml` (no `[[test]]` or `[[example]]`). How the harness compiles them was not traced. The names they use are listed in 3.5.
- Go twin: `⌨️cli/📦️packages/🐹️go` (not affected, the Go module is separate).

### 3.2 Section table

| Lines | Region | Active | Goes to |
|---|---|---|---|
| 1-13 | `//#region Header` and `//!` docs (crate doc comment) | 9 | P |
| 15-23 | `use` block inside `repo_cli` (15-22) | ~8 | facade F |
| 24-175 | `Events`: `KIND_*`, `ERR_*`, `EXIT_*` consts, `Progress`, `Artifact`, `ErrPayload`, `DonePayload`, `Event`, `graphql_events` (159) | 109 | C01 |
| 177-346 | `GoFormatting`: `go_json`, `go_duration`, `write_go_json`, `go_field` (Go twin formatting) | 147 | C01 |
| 348-840 | `Renderers`: `Format`, `Rendered`, `render`, `render_ndjson`, `render_human`, `render_markdown_stream`, `events_to_markdown`, `render_goal_tree`, `render_section_tree`, `format_result` (622), `format_markdown_result` (723), `format_markdown_file` (811) | 458 | C01 |
| 842-1351 | `CommandFramework`: `FlagValue`, `Flag`, `parse_duration`, `Arity`, `Action`, `Command`, `Parsed`, `parse` (1128), `CliError`, `Config`, `Invocation` (1233, with `render`, `root`, `args`, `text`) | 400 | C02 |
| 1353-1783 | `CommandTree`: `root()` (1359), flag builders, command builders (`ticket_command`, `goal_command`, ...) with 82 `verbs::` references | 403 | C03 |
| 1785-1954 | `TreeSource`: `normalize_tree_scope`, `path_within_scope`, `FsTreeSource` and `TreeSource` impl | 154 | C04 |
| 1956-2051 | `ModelIds`: `model_section_id`, `model_definition_id`, `export_entities` | 80 | C04 |
| 2053-2277 | `StreamOptions`: `StreamOptions`, `status_filter`, `tree_filter`, `searched_tree`, `write_tree_forest` | 196 | C04 |
| 2279-2282 | `//#region Verbs` and `pub mod verbs {` opener | 0 | C05 |
| 2283-3745 | `verbs` body: `use super::*;` (2283), helpers (2286-2391), 99 verb handlers (2392-3744) | 1,353 | C05 |
| 3749-3903 | `Projection`: `projection`, `usage`, `command_paths`, `render_stream_json`, `graphql_roundtrip_json`, `test_verb_lines_json`, `export_records_json`, `hook_verb_dispatch_json`, `SharedBuffer`, `mcp_conversation` | 121 | C06 |
| 3905-3970 | `Run`: `run` (3908), `run_with` (3919) | 60 | C06 |
| 3975-3980 | `//#region McpVerb` docs | 0 | P |
| 3981-4297 | `pub mod mcp_verb`: `RepoRepository` and its `Repository` impl, `real_repository` (4292) | 278 | M01 `🔌️mcp-verb` (root sibling) |

Sum: children C01 to C06 = 3,481 active, M01 = 278, root and facade about 9 active. Total 3,768, matches the measured total.

### 3.3 Proposed children

Facade folder `🧭️repo-cli/` holds `🦀️.rs` (facade: `use` preamble, `pub mod` mounts, `pub use` globs, closing region). Children are under `🧭️repo-cli/`. `🔌️mcp-verb/` is a root sibling of `🧭️repo-cli/` under `⌨️cli/`.

| Id | Folder (path under `⌨️cli/`) | Mod name (public path) | Active | Notes |
|---|---|---|---|---|
| F | `🧭️repo-cli/🦀️.rs` (facade) | `repo_cli` | ~9 + glob | Keeps `repo_cli::run`, `usage`, `projection`, etc. via `pub use`. |
| C01 | `🧭️repo-cli/📡️event-stream/🦀️.rs` | `event_stream` (glob) | 714 | Events, Go formatting, renderers. Includes `format_markdown_result`, used by mcp (see 3.4). |
| C02 | `🧭️repo-cli/🎮️command-framework/🦀️.rs` | `command_framework` (glob) | 400 | Flags, commands, parser, `Invocation`. |
| C03 | `🧭️repo-cli/🌳️command-tree/🦀️.rs` | `command_tree` (glob) | 403 | `root()` and builders. |
| C04 | `🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs` | `tree_source_and_search` (glob) | 430 | `FsTreeSource`, model ids, stream options. |
| C05 | `🧭️repo-cli/🚦️verbs/🦀️.rs` | `verbs` (pub mod) | 1,353 | 99 handlers. Under 2,500, no further split. |
| C06 | `🧭️repo-cli/🚀️run-and-projection/🦀️.rs` | `run_and_projection` (glob) | 181 | `run`, `run_with`, projection helpers used by test adapters. |
| M01 | `🔌️mcp-verb/🦀️.rs` | `mcp_verb` (pub mod, root) | 278 | Root-level `pub mod mcp_verb;`. |

Folder check: `🔌️` is used by `🔌️mcp` (a repo module elsewhere) but not as a sibling inside `⌨️cli/`. Siblings of `⌨️cli/` today: `📦️packages`, `🔮️oracles`, `🧪️tests`, `README.md`. Names `🧭️repo-cli`, `🔌️mcp-verb` are unique. Longest proposed path: `⌨️cli/🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs` is 154 bytes including the 28-byte prefix (measured).

Facade wiring (for reference): root `🦀️.rs` becomes `#[path = "🧭️repo-cli/🦀️.rs"] pub mod repo_cli;` and `#[path = "🔌️mcp-verb/🦀️.rs"] pub mod mcp_verb;`. Facade: `#[path = "📡️event-stream/🦀️.rs"] mod event_stream; pub use event_stream::*;`, same for each child. `verbs` keeps `pub mod verbs` so `repo_cli::verbs::*` stays valid.

### 3.4 Coupling and cycles (measured)

- Cycle to break: `verbs::mcp` (line 2398) calls `crate::mcp_verb::RepoRepository::open(…)`, and `mcp_verb::entity_result` (line 4081) calls `crate::repo_cli::format_markdown_result(…)`. Fix: `format_markdown_result` is defined in the renderers part of `event-stream` (C01, line 723). `mcp_verb` then depends on `event_stream`, and `verbs` depends on `mcp_verb`. No cycle after the change: `verbs` to `mcp_verb` to `event_stream`.
- `command_tree` (C03) depends on `verbs` (82 `verbs::` paths). The `root()` hits inside `verbs` are `invocation.root()` and `context.root()` methods, not the `command_tree::root()` function, so no verb calls back into the tree. Direction is correct (checked by grep, not by a full call graph).
- `run_and_projection` (C06) depends on `command_tree` (`root()` in 4 places) and `command_framework` (`parse`, `Config`, `Invocation`).
- `Invocation::render` (1147, 1155) and `Format` (C01 renderers) are used by C02 and C05. C02 depends on C01.
- Graphql dependency: `repo_cli` imports `semio_framework_repo_graphql::{Executor, FsRepoContext, RecordingContext, RepoContext}` (line 16) and `mcp_verb` imports `{Executor, FsRepoContext, RepoContext}` (line 3982). After section 5 (E), `FsRepoContext` comes from the new adapter crate. This is a `use` line change in C01/C05 and M01 only.
- Visibility: the facade re-exports globs, so `use super::*;` in `verbs` (2283) continues to resolve. Cross-child private helpers need `pub(super)`. Not measured per name. Known example: `paint` (renderers, 351) is private and used only inside renderers (same child). `resolve_section_name` and `semio_framework_repo_workspace_unslugify` (2364, 2382) are `pub` and private respectively, used inside `verbs`.
- Per-symbol coupling table was not produced in this audit. The executor produces it with the exemplar's identifier scan.

### 3.5 Public names used by test adapters (must stay reachable)

Measured (grep over `⌨️cli/🧪️tests`): `repo_cli::repo_cli` (8), `repo_cli::projection` (4), `repo_cli::usage` (3), `repo_cli::test_verb_lines_json` (3), `repo_cli::export_records_json` (3), `repo_cli::command_paths` (2), `repo_cli::run` (1), `repo_cli::render_stream_json` (1), `repo_cli::mcp_conversation` (1), `repo_cli::hook_verb_dispatch_json` (1), `repo_cli::graphql_roundtrip_json` (1). All of these are in C06 or C03 and must be re-exported by the facade.

### 3.6 Paths, include, macros

- `#[path]`, `include!`, `include_str!`: none (measured). No path rewrites are needed.
- `macro_rules!`: none (measured).
- `cfg(test)`: none in this file (measured, no `mod tests`).
- `Command::new` at 1360-1402 are builders of the framework's own `Command` type, not process spawns. Real process spawns (measured with `std::process` grep, 2 hits): `git blame --line-porcelain` at line 3273 (inside `mermaid_contributors`, 3269-3286) and `std::process::Command::new(&argv[0])` at line 3474 (inside `micro_commit`, 3464-3481, runs an arbitrary argv). These are side effects that a port should own. Proposed: a `process` port in `🚦️verbs` (or a separate adapter) so the verb logic is testable without spawning. Not a split blocker, but flag it for the executor, because moving these lines changes the `use` set of `verbs`.

### 3.7 Source-text readers

- Hub and root script search: none for `⌨️cli/🦀️`.
- `🧰️framework` and `✏️s` search (section 10): the path tail `⌨️cli/🦀️.rs` appears in JSON fixtures, which are owner declarations, not readers in code:
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json:4922`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json:6` (`"owner"` field)
  These owner entries must be updated when the file is split (they name the monolith path as owner).
- False positives (different crates with the same tail): `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/🦀️.rs:115` (`#[path = "../../⌨️cli/🦀️.rs"]`, the db CLI) and `✏️s/🔌️plugins/🗒️note/…/🏭️generator/🔁️codec/⌨️cli/🦀️.rs`. Do not change those.

### 3.8 Wrong-direction dependencies

- None of the seven repo domain crates in `Cargo.toml` is a wrong direction; the CLI is the product layer and may depend on them.
- Measured concern: the CLI depends on the GraphQL crate's concrete `FsRepoContext`, which pulls provider and tickets wiring into the GraphQL package (section 5). After section 5 the CLI depends on a named adapter crate, which is the intended composition root.
- `mcp_verb` lives in the CLI crate but is a separate server. It only depends on `semio_framework_repo_mcp`, `semio_framework_repo_providers` (`McpClientKind`, line 3985) and `event_stream`. Moving it into its own crate later is cheap; it does not need the CLI verbs.

### 3.9 Extraction sequence (bottom-up)

| Step | Target | Range | Anchor |
|---|---|---|---|
| 1 | M01 `🔌️mcp-verb` | 3981-4297 | 3975 region marker (remove), 4298 endregion (remove), root `pub mod mcp_verb;` |
| 2 | C06 `🚀️run-and-projection` | 3749-3970 | 3748 region, 3970 endregion |
| 3 | C05 `🚦️verbs` | 2283-3745 | 2279-2282 region and opener; 3746 blank; 3747 endregion |
| 4 | C04 `🗂️tree-source-and-search` | 1785-2277 | 1784 blank, 2278 blank |
| 5 | C03 `🌳️command-tree` | 1353-1783 | 1352 blank |
| 6 | C02 `🎮️command-framework` | 842-1351 | 841 blank |
| 7 | C01 `📡️event-stream` | 24-840 | 23 blank; `format_markdown_result` moves here first (step 0: change `mcp_verb` call to `event_stream::format_markdown_result`) |
| 8 | facade `🧭️repo-cli/🦀️.rs` | 14 opener, 15-23 imports, 3971-3972 closer | `pub mod repo_cli {` inline removed |

Step 0 (before step 1): move `format_markdown_result` into the renderers part and update the one call in `mcp_verb` at line 4081 and the call sites in `verbs`. This removes the cycle. Keep the repo's Go twin as is (no change).

## 4. D: tiled-map surface (`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs`)

### 4.1 Owning crate, targets, mounting

- Lib `semio-framework-surface` (`🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`, `[lib] path = "🦀️.rs"`). The crate root `🗺️surface/📦️packages/🦀️rust/🦀️.rs` wires `paint`, `terrain`, `node_graph` and `tiled_map` via `#[path]`. Mount: line 14, `#[path = "../../🗺️tiled-map/🦀️.rs"] pub mod tiled_map;`.
- Crate also has `#[cfg(test)] #[global_allocator]` (lines 17-22 of the surface root). Splitting tiled-map into several crates must not add a second global allocator to the same test binary.
- Integration test `tiled_map_mercator_oracle`: `[[test]]` in `Cargo.toml` pointing at `🗺️surface/🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs`.
- Tiled-map's unit tests (`🧪️tests/🔬️unit/🦀️.rs`, 1,509 raw) are mounted at line 4185-4186 (`#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`). Also `🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs` is pulled in by `include!` at line 1061 (`#[cfg(test)]`).
- Target: wasm32 with `not(target_env = "p2")` (`MapSession`, lines 3893-4183) is a browser bridge. wasm-bindgen exports are in `🕸️bindings` (`framework_surface.js`, `framework_surface_bg.wasm`, regenerated by the build, not by this audit).
- Consumers (measured with `tiled_map::` grep): inside the surface folder `tiled_map::projection` (3), `tiled_map::tiles::visible_tiles` (2), `tiled_map::canvas::camera` (2), `tiled_map::Point::new` (2). In the GIS plugin, at least 10 files under `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🗺️maphost/🦀️.rs` and `…/🪟️windows/🗺️map/…` import `semio_framework_surface`.

### 4.2 Section table

| Lines | Content | Active | Goes to |
|---|---|---|---|
| 1-18 | `//!` doc (GIS doctrine comments and `crate::artifacts::gismap::schema` references in docs only, line 8) | 0 | P |
| 19-27 | `pub use canvas::{self, *};` (19), `pub use std::sync::Arc;`, `use` block | ~8 | P |
| 29-76 | `map_color`, `MapPalette` struct and impl and `Default` (35-76) | ~40 | P |
| 78-145 | GIS LOD consts (`GIS_MAP_LODS` 78, `GIS_MAP_LOD_TILE_Z` 93, `GIS_MAP_LOD_MAX_SPAN_DEG` 99, `GIS_MAP_LOD_SCALE` 101), tile cache consts, FNV revision helpers (109-144) | ~50 | P |
| 146-298 | `gis_map_lod_scale_json`, LOD selection, camera zoom limits, `mod map_viewport` (299-332, inline, 28 active) | ~100 | P (inline `map_viewport` stays) |
| 333-381 | `pub mod projection { … }` inline (Web Mercator) | 37 | D01 `🧭️projection` |
| 382-582 | `pub mod tiles { … }` inline (raster tile cursor, visible tiles) | 169 | D02 `🧩️tiles` |
| 583-1481 | `pub mod vector_tiles { … }` inline: MVT decoder (584-1085, `include!` at 1061, `decode_mvt` at 1063) and vector style and detail rules (1086-1481) | 797 | D03 `🧱️vector-tiles` (facade) with D03a `📦️mvt-decoder` (456), D03b `🎨️vector-style` (341) |
| 1482-1833 | `MapExtension` trait and `DefaultMapExtension`, `PositionData`, `RouteData`, `RegionData`, `MapDescriptorJson`, `MapTileMode`, `MapVectorStyle`, layer weight keys, `MapLayerStrokeScale`, `MapLayerVisibility`, `MapFeatureTables`, `MapTileLedger` | 301 | D04 `🪢️map-state` |
| 1834-1995 | `pub struct MapHost` (1834) with fields, `CachedVectorScene`, `MapHostRetirement` and `Drop` | 125 | D05 `🏛️map-host` (facade and struct) |
| 1996-2259 | `//#region InteractionPlan` (1996-2034), geometry hit tests (2037-2169), `impl Default for MapHost` (2170), `LabelDeclutter` (2198-2259) | 242 | D06 `🖱️interaction-and-labels` |
| 2260-2307 | `//#region Errors` (`FrameworkSurfaceTiledMapError`, `From` impls) | 39 | P (crate-wide error type) |
| 2308-3053 | `impl MapHost` part A: `new` (2309), theme, camera, LOD and zoom queries, tile queries, interaction commit, uploads, `sync_map_json` (2692), pointer and hit tests, feature queries, focus | ~630 | D05a `🧭️camera-and-picking` (child of `🏛️map-host`) |
| 3054-3877 | `impl MapHost` part B: `prepare_visible_tiles` (3055), tile geometry, screen helpers, vector and raster painting (`append_*`), `build_vector_scene` (3849), `build_render_scene` (3866); ends at 3877 | ~780 | D05b `🖼️host-scene` (child of `🏛️map-host`) |
| 3878-3892 | `impl canvas_content::CanvasContent for MapHost` (3878-3891 incl. `#[cfg]`-free trait impl), comment at 3892 | 8 | D05b |
| 3893-4183 | `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` bridge: `MapSessionInner`, `MapSession` and impl | 236 | D07 `🌐️wasm-session` |
| 4184-4187 | `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` | 3 | P |

Sum of children is about 3,600. Parent after split: about 260 active (lines 1-298 minus, `map_viewport`, errors, tests mount).

### 4.3 Proposed children

| Id | Folder (under `🗺️tiled-map/`) | Mod path | Active | Notes |
|---|---|---|---|---|
| D01 | `🧭️projection/🦀️.rs` | `projection` | 37 | Pure math. |
| D02 | `🧩️tiles/🦀️.rs` | `tiles` | 169 | Raster tile cursor. |
| D03 | `🧱️vector-tiles/🦀️.rs` (facade) | `vector_tiles` | 0 | `pub use` of D03a and D03b. |
| D03a | `🧱️vector-tiles/📦️mvt-decoder/🦀️.rs` | `vector_tiles::mvt_decoder` (glob) | 456 | Pure MVT decoder (`decode_mvt`, `ProtoCursor`, raw decoders). |
| D03b | `🧱️vector-tiles/🎨️vector-style/🦀️.rs` | `vector_tiles::vector_style` (glob) | 341 | GIS style rules. Candidate for the GIS plugin (section 4.8). |
| D04 | `🪢️map-state/🦀️.rs` | `map_state` | 301 | Descriptors and layer state. |
| D05 | `🏛️map-host/🦀️.rs` (facade and struct) | `map_host` | 125 | Struct and retirement. Children below are nested here so private fields stay visible (descendants of the struct's module). |
| D05a | `🏛️map-host/🧭️camera-and-picking/🦀️.rs` | `map_host::camera_and_picking` | ~630 | `impl MapHost` part A. |
| D05b | `🏛️map-host/🖼️host-scene/🦀️.rs` | `map_host::host_scene` | ~790 | `impl MapHost` part B, plus `impl CanvasContent`. |
| D06 | `🖱️interaction-and-labels/🦀️.rs` | `interaction_and_labels` | 242 | Interaction plan, geometry, declutter, `Default for MapHost`. |
| D07 | `🌐️wasm-session/🦀️.rs` | `wasm_session` | 236 | Keep the `cfg` on every item. |

Folder names check: the surface folder's existing siblings are `🔮️oracles`, `🧪️tests`, `🧬️schema`, `🦀️.rs` (checked). No duplicate. Under `🏛️map-host/` the children are unique. Longest path: `🗺️tiled-map/🏛️map-host/🖼️host-scene/🦀️.rs` (136 bytes including the prefix, measured).

Note that `🌐️` (`wasm-session`) is also an RGI globe emoji used elsewhere; no sibling conflict.

Parent after split keeps: 1-298 (crate header, palette, LOD constants, camera limits), `mod map_viewport` (299-332), errors (2260-2307), and mounts for D01-D07 with `pub use` as needed. Note: `mod map_viewport` is private inline and stays.

### 4.4 Coupling and visibility

- `MapHost` fields (1834-1883) are private or `pub`. Children D05a and D05b are descendants of `map_host`, so they see private fields without change. This is why the struct sits in the facade folder.
- D06 (interaction) is a sibling of D05. `MapInteraction` (1987, private enum) is used by `MapHost::plan_interaction` (2603, in D05a) and by D06 declutter. Make `MapInteraction` `pub(super)` in D05 or move it to D06 with `pub(super)` (decision for the executor).
- Methods of `impl MapHost` split between D05a and D05b call each other (for example `prepare_visible_tiles` in D05b is called from D05a `pan` path; `tile_local_to_screen` in D05b used in D05a `hit_test`). Those methods become `pub(super)` (visible in `map_host` and descendants, which includes both children). Not measured per method; the blanket rule applies.
- `vector_tiles` facade: D03a (decoder) is used by D05b and by `vector_style` (D03b). D03b uses `super::Color` and `ui_styling::metrics::map::LABEL_PX_BANDS`. Visibility of decoder types (`VectorTile`, `VectorLayer`, `VectorFeature`, `GeomType`) is already `pub`. Helper functions used across: `property_str` (1086), `feature_label` (1098) and `layer_draw_rank` (1125) are `pub`.
- `pub use canvas::{self, *};` at line 19 re-exports the whole `canvas` crate from this module. Downstream code may rely on it. Replace by explicit re-exports (AGENTS: "reexport explicitly if the client needs it").
- `use super::GIS_MAP_LOD_MAX_SPAN_DEG` (14 uses) and `super::resolve_detail_lod_index` (4), `super::resolve_map_lod_index_from_span` (1), `super::Point` (2), `super::Rect` (1), `super::Color` (1), `super::MAP_CAMERA_ZOOM_MIN` (1), `super::FrameworkSurfaceTiledMapError` (1), `super::projection::WORLD_HALF` (1), `super::map_viewport` (1) are in the tests (in `tests/`, `super::` relative to `tiled_map`). The parent keeps these names, so the tests keep working. Children that call the LOD helpers call `super::super::…` or import via the crate root.

### 4.5 Paths, include, macros

- `include!("🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs");` at line 1061 is relative to the file that contains it. After moving decoder to `🧱️vector-tiles/📦️mvt-decoder/🦀️.rs`, the path becomes `../../🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs` (two folders deeper than `🗺️tiled-map/`). Measured depth: `📦️mvt-decoder` sits under `🧱️vector-tiles`, which sits under `🗺️tiled-map`.
- `#[path]` mounts inside the file: only the test mount (4185). Stays in parent.
- `macro_rules!`: none (measured).
- `cfg` gates: 10 (measured). Keep each `cfg` pair in the same child (`#[cfg(target_arch = "wasm32")]` blocks: `resolve_identity_env`-style pairs). Specifically the wasm imports at 3893-3900 and the bridge at 3903-4183 go to D07 together.
- `crate::artifacts::gismap::schema::…` (line 8): doc comment only. No code path.

### 4.6 Test modules

- `tests` (`🧪️tests/🔬️unit/🦀️.rs`) stays mounted from the parent (4186). Uses `super::…` names listed in 4.4.
- `vector-tiles-decode-geometry` test is included into the decoder (line 1061 `#[cfg(test)]`). Moves with D03a.
- `tiled_map_mercator_oracle` integration test (surface crate) imports the crate's public API. No code change if public paths hold.
- `web-mercator-tile-oracle` and `🧩️suite` under `🗺️surface/🧪️tests` (not read in this audit beyond the names).

### 4.7 Source-text readers

Measured in `📜️script.ts` and `🧰️framework`:
- `📜️script.ts:8112` (root): `policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs")`. A policy gate reads this path. Must be updated to the facade or the list of files.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts:18`: `read("🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs")`. A text test reads the file. Must be updated.
- Oracle declarations in JSON: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🔣️.json:25`, `…/📸️jpg/…/🧾️document/🔮️oracles/🔣️.json:25` and `:70`, `…/🖼️tiff/…/🧾️document/🔮️oracles/🔣️.json:23`. Path string `"🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs"` as a source or owner reference. Must be updated or the oracles regenerated.
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🦀️.rs:14` (`#[path]` mount), stays valid if D05 is the mount target (the facade path becomes `../../🗺️tiled-map/🦀️.rs` unchanged, since the parent file stays at this path). Good: the parent file keeps the same path, so this mount does not change.
- Wrong-tail false positive: `📺️renderer/…/🧊️wgpu/🦀️.rs:73` mounts `🧭️node-graph-and-tiled-map/🦀️.rs`, a different file in the wgpu renderer. Not this monolith.
- wasm bundle `🕸️bindings/framework_surface_bg.wasm` and `dist/` are generated. Regenerate after split; not an audit item.

### 4.8 Wrong-direction dependencies (GIS in framework)

1. `🧱️vector-tiles/🎨️vector-style` (341 active) and `1-298` GIS LOD tables (`GIS_MAP_LODS`, `GIS_MAP_LOD_MAX_SPAN_DEG`, `GIS_MAP_LOD_SCALE`) hold GIS-specific policy: OSM-style class ranks (`place_label_rank`, `transportation_name_rank`, `layer_draw_rank`, `boundary_visible` by admin_level). The framework surface should not carry this. Proposed: move D03b and the GIS LOD block into the GIS plugin's own crate; the framework keeps a neutral `MapStyleProvider` port (`MapExtension` at 1482 already exists as a trait with `DefaultMapExtension`).
2. Naming: `GIS_MAP_*` constants, `gis_map_lod_scale_json`, `gis_map_camera_limits_json`, `gis_map_layer_weight_slider_ids_json` (in 1-298 and 1605-1665) name a concrete plugin inside the framework. Proposed rename after move: `map_*`.
3. The doc at 1-18 states that `features` mirrors `crate::artifacts::gismap::schema::{snapshot,diff,mutations}` of `✏️s/🔌️plugins/🌍️gis`. The framework therefore knows the GIS snapshot shape only as a mirror (`sync_map_json`, 2692). Keep it as a port-level JSON, not a GIS type.
4. Third-party runtime dependency: `image::load_from_memory(png_bytes)` at 2671 (inside `upload_tile`) and `impl From<image::ImageError>` at 2295. The crate already depends on `semio-framework-pixels` (first-party PNG codec, per `Cargo.toml` comment) for terrain. AGENTS forbids runtime external libraries. Proposed: use `semio-framework-pixels` for the tile PNG decode and drop `image` from `Cargo.toml` (the `image` line in `Cargo.toml` is the only other reference). Measured: `image::` appears 3 times in the file.
5. `serde_json` (59 uses) in public `String`-returning JSON APIs. Public signatures return `String` for most functions, so external types do not leak. `serde_json::Value` is not in public signatures (measured by skim; not counted). Re-check when the API is reshaped.

### 4.9 Extraction sequence (bottom-up)

| Step | Target | Range | Anchor |
|---|---|---|---|
| 1 | D07 `🌐️wasm-session` | 3893-4183 | 3892 comment (keep with the item), 4183 region (keep tests mount in parent) |
| 2 | D05b `🖼️host-scene` | 3054-3892 (includes `impl CanvasContent`) | opener `impl MapHost {` added; 3053 blank |
| 3 | D05a `🧭️camera-and-picking` | 2309-3052 | opener `impl MapHost {`, closer `}` added; 2308 is the opener line |
| 4 | D06 `🖱️interaction-and-labels` | 1996-2259 | region markers 1996, 2259-2260 |
| 5 | D05 `🏛️map-host` | 1834-1995 | struct and retirement |
| 6 | D04 `🪢️map-state` | 1482-1833 | trait and descriptors |
| 7 | D03 facade plus D03b `🎨️vector-style` | 1086-1481 | `pub mod vector_tiles {` opener changes to facade |
| 8 | D03a `📦️mvt-decoder` | 584-1085 | include! path rewrite (section 4.5) |
| 9 | D02 `🧩️tiles` | 383-581 | inline `pub mod tiles {` removed |
| 10 | D01 `🧭️projection` | 334-380 | inline `pub mod projection {` removed |
| 11 | parent | keep 1-298, 299-332 (`map_viewport` inline), 2260-2307 errors, 4184-4187 mounts | verify |

Anchor check before each step. Cut lines chosen at blank lines between `impl` methods (3053/3054 and 2308/2309 are blank or opener, verify).

## 5. E: repo GraphQL rust (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/🦀️.rs`)

### 5.1 Owning crate, targets, mounting

- Lib `semio-framework-repo-graphql` (`📦️packages/🦀️rust/Cargo.toml`, `[lib] path = "🦀️.rs"`). `rust-version 1.88`, `[package.metadata.semio] role = "library"`.
- Test mount: lines 5034-5036 `#[cfg(test)] #[path = "../../🧪️tests/🔬️unit/🦀️.rs"] mod tests;` (path from the package folder to `🔗️graphql/🧪️tests/🔬️unit/`). Other GraphQL test folders under `🔗️graphql/🧪️tests`: `▶️query-execution`, `✏️mutation-execution`, `❌️execution-errors`, `📃️document-parsing`, `📜️sdl-dump`, `🔀️variable-coercion`, `🙅️unsupported-syntax`, `🚫️syntax-errors`, `🧫️fixtures`. Their `.rs` adapters are not mounted from this file.
- Dependencies: 13 `semio-framework-repo-*` path crates plus `serde`, `serde_json`. Measured use (line ranges: core 1-3955, adapter 3956-end): `identity` core 1 / adapter 5; `model` core 1 / adapter 10; `codebase` 0 / 7; `contributors` 0 / 7; `events` 0 / 10; `goals` 0 / 15; `languages` 0 / 5; `move` 0 / 19; `providers` 0 / 10; `statutes` 0 / 8; `tickets` 0 / 2; `todos` 0 / 4; `workspace` 0 / 4.
- Consumers: `⌨️cli/📦️packages/🦀️rust/Cargo.toml:47` (path dependency) and `repo_cli` import (C01/C05), `mcp_verb` import (M01).
- Go twin: `🔗️graphql/📦️packages/🐹️go` (byte-for-byte behavioural twin per header lines 5-7). Not affected by a Rust split, but any change to diagnostic text breaks the twin's parity fixtures. Keep text identical.

### 5.2 Section table

| Lines | Region | Active | Goes to |
|---|---|---|---|
| 1-10 | `//#region Header`, `//!` doc | 0 | P |
| 12-26 | imports (12-22), `pub use serde_json;` (26) | 12 | P |
| 28-257 | `Lexer`: `TokenKind`, `Token`, `Lexer` and `unquote`, `quote_*` | 201 | E01 `🔤️grammar` |
| 258-382 | `Ast`: `Document`, `Selection`, `Literal`, `Value`, `coerce_arguments` | 96 | E01 |
| 383-629 | `Parser`: `ParseErrorKind`, `ParseError`, `Parser`, `parse` (607) | 226 | E01 |
| 630-644 | `Validation`: `validate` (635), `operation_type` (640) | 6 | E01 |
| 647-852 | `Schema` types: `TypeRef`, `Argument`, `Field`, `ObjectType`, `InterfaceType`, `UnionType`, `EnumType`, `InputObjectType`, `NamedType`, `Schema`, `SCALARS` (851) | 125 | E02 `📜️schema` |
| 855-981 | `ContextPort`: `ContextError`, `RepoContext` trait (886-980) | 63 | E03 `🎥️context-port` |
| 983-1427 | `Sources`: JSON shaping of domain entities (`technology_source` 1192 ... `analyze_result_source` 1415) | 352 | E04 `🖼️sources` |
| 1429-2322 | `Schema*` builders: `schema_enums` (1432) through `build_schema` (2285) | 769 | E02 |
| 2324-2522 | `Execution`: `ExecutionError`, `canonical_name`, `resolve_default`, `coerce_input`, `matches_filter`, `decode_input`, `Execution` struct and core impl | 161 | E05 `⚡️execution` |
| 2524-2713 | `QueryResolvers`: `impl Execution` (query roots) | 165 | E05 |
| 2715-2842 | `EntityResolvers`: `TicketCalendar` (2718), entity resolvers | 117 | E05 |
| 2844-3052 | `MutationResolvers`: `impl Execution` mutations | 204 | E05 |
| 3054-3118 | `Executor` struct and impl | 48 | E05 |
| 3120-3254 | `Sdl`: `reachable`, `render_field`, `render_sdl` (3173), `schema_inventory` (3230) | 120 | E02 (`📜️schema`) |
| 3256-3955 | `RecordingContext`: `Records`, `RecordingContext` struct and `impl RepoContext` (3418-3954) | 624 | E06 `🩻️recording-context` |
| 3957-5030 | `FsRepoContext`: `AnalyzeScope`, `CoordinatorEmitter`, `ProviderManagement`, `FsRepoContext` struct and impl (4079-4908), `RepoContext` impl (4366), `section_record`, `prune_labels` | 958 | E07 (new adapter crate) |
| 5032-5038 | `Tests` mount | 3 | P |

### 5.3 Proposed children and the adapter crate

| Id | Location | Content | Active | Repo domain deps |
|---|---|---|---|---|
| E01 | `📦️packages/🦀️rust/🔤️grammar/🦀️.rs` | 28-644 | 529 | none |
| E02 | `📦️packages/🦀️rust/📜️schema/🦀️.rs` | 647-852, 1429-2322, 3120-3254 (three ranges in order, concatenated) | 1,014 | `identity`, `model` |
| E03 | `📦️packages/🦀️rust/🎥️context-port/🦀️.rs` | 855-981 | 63 | `model` |
| E04 | `📦️packages/🦀️rust/🖼️sources/🦀️.rs` | 983-1427 | 352 | `model`, `identity` |
| E05 | `📦️packages/🦀️rust/⚡️execution/🦀️.rs` | 2324-3118 | 695 | `model`, `identity` |
| E06 | `📦️packages/🦀️rust/🩻️recording-context/🦀️.rs` | 3256-3955 | 624 | `model`, `identity` |
| E07 | new crate `🔗️graphql/🗄️fs-context/📦️packages/🦀️rust/` (`semio-framework-repo-graphql-fs`) | 3957-5030 | 958 | codebase, contributors, events, goals, languages, move, providers, statutes, tickets, todos, workspace, model |

Folder check: `🔗️graphql/` contains `📦️packages`, `🔮️oracles`, `🧪️tests`, `README.md`. Names `🗄️fs-context` and the children under `📦️packages/🦀️rust/` are unique. The new crate folder `🔗️graphql/🗄️fs-context/` sits beside `📦️packages`, so that is a second package root under graphql. Alternatively place it under `📦️packages/🗄️fs-context/🦀️rust` (also unique). Decision for the executor; both paths are short (159 bytes measured for the first).

The parent `📦️packages/🦀️rust/🦀️.rs` keeps the header (1-10), imports (12-22), `pub use serde_json;` (26), `pub use` re-exports of E01 to E06, and the tests mount (5034-5036 path unchanged). The `pub mod` mounts for E01 to E06 are `#[path = "🔤️grammar/🦀️.rs"] mod grammar; pub use grammar::*;` and so on, same pattern as the exemplar.

Measured sizes: E02 (schema) is the largest core child (1,014). All core children are under 1,100 active.

### 5.4 Coupling and cycles

- Direction in core: E01 (grammar) has no dependency. E02 (schema) depends on model and identity. E03 (port) depends on model. E04 (sources) depends on E02 (`Schema`, `NamedType`) and model. E05 (execution) depends on E02, E03, E04, and E01 (`Document`, `Selection`). E06 (recording) depends on E03 and model. E07 (fs) depends on E03 and all repo domain crates.
- No cycle is introduced. E07 depends on E03 (port) and on model, never the other way.
- `RepoContext` trait (E03, 886-980) is the port that E07 implements (`impl RepoContext for FsRepoContext`, 4366). Good direction.
- `CoordinatorEmitter` (3969-3971) implements `semio_framework_repo_events::Emitter` and `ProviderManagement` implements `semio_framework_repo_goals::ManagementPort` (3990). Both are E07-only.
- `AnalyzeScope` (4025) is used by E07 only.
- Visibility: E05 resolvers are `impl Execution<'_>` blocks over one struct in E05 (single child), so no cross-child method visibility is needed for resolvers. `Executor` and `Execution` are used by `E05` only, plus the public `Executor` (3057) re-exported.
- `pub use serde_json;` (26) and `Json = serde_json::Value` in public trait signatures (`RepoContext` returns `Json`-typed data, `graphql_events` in the CLI takes `Map<String, Json>`). AGENTS asks for explicit re-export and no external types in exported API without an interface. Decision point, not in the split: keep `pub use serde_json` explicit and document that `Json` is the public data type.

### 5.5 Paths, include, macros

- `macro_rules!`: none (measured). `include!`/`include_str!`: none (measured). `#[path]`: only the test mount at 5035.
- `cfg` gates: 1 (`#[cfg(test)]` on the tests mount). No cfg-gated items split across children.
- No `crate::` paths (measured for core: `use semio_framework_repo_model::{…}` (13-18) and `use semio_framework_repo_identity::{…}` (12) are crate paths of the external workspace crates).

### 5.6 Test modules

- `🔗️graphql/🧪️tests/🔬️unit/🦀️.rs` mounted at 5035. Stays with the parent, unchanged path.
- The `RecordingContext` (E06) and `FsRepoContext` (E07) are used by the CLI tests (`repo_cli` adapters using `RecordingContext` via `graphql_roundtrip_json`). E06 stays in core because it has no repo domain deps beyond model and identity. Moving E06 to a test-support crate is an option for later; not required.

### 5.7 Source-text readers

- Hub and root search: none for the GraphQL package path. Framework search (`🧰️framework`, `✏️s`) for `🔗️graphql/📦️packages/🦀️rust` (fixed string) returned JSON and Cargo references:
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🔣️.json:305-306` (declaration and manifest path for `semio-framework-repo-graphql`). Must update if the package moves or a crate is added.
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/📋️project.json:9,16,36` (self reference, unaffected).
  - `⌨️cli/📦️packages/🦀️rust/Cargo.toml:47` (dependency path, unaffected for the core crate, and must add the new adapter crate dependency for CLI).
- False positives: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.toml:18,145` and `…/🧊️gltf/Cargo.toml:8,143` reference `🏅️standards/…/🧬️schema/📤️exports/🔗️graphql/📦️packages/🦀️rust`, a different crate (stdio artifact export). Do not change.
- Go twin parity: the Go package's tests (`🔗️graphql/📦️packages/🐹️go`) are independent of the Rust file path (no reader found in the Go tree during this audit's grep of `🔗️graphql` for `🦀️.rs`, `readFileSync`, `read_to_string`).

### 5.8 Wrong-direction dependencies

1. The grammar and executor crate (`semio-framework-repo-graphql`) depends on 13 repo domain crates, but its core uses 2 (measured). The other 11 come in through `FsRepoContext` (E07). Fix: E07 moves to its own crate. Core drops 11 Cargo dependencies.
2. `FsRepoContext` wires concrete providers (`semio_framework_repo_providers::{ManagementLabel, ManagementMilestone, ManagementProvider, ManagementProviders, …}`, line 3962) and file-system stores (`FileTicketStore`, `FsGoalStore`, `FsDraftStore`, `FsTodoTree`). That is composition, belongs to the product (repo) or the adapter crate.
3. `ProviderManagement` (3978-4029) implements the goals port with a concrete GitHub-style management provider. The port (`ManagementPort`, in `goals`) is correct; the implementation must be out of core.
4. `RecordingContext` (E06, 3320-3954) is an in-memory context for tests and recordings. It is in the production crate. Acceptable (it is a port implementation with no I/O), but flag as test-support candidate.
5. `TicketCalendar` and ticket-specific resolvers (E05, 2718-2842) are domain-specific (tickets). This is fine for a repo GraphQL schema (the schema is the repo's vocabulary), but the schema types for tickets (`schema_ticket` 1892) could later move to the tickets domain. Not in scope.

### 5.9 Extraction sequence (bottom-up)

| Step | Target | Range | Anchor |
|---|---|---|---|
| 1 | E07 adapter crate | 3957-5030 | 3956 region opener removed; new `Cargo.toml` created by executor |
| 2 | E06 `🩻️recording-context` | 3256-3955 | 3255 region opener, 3955 endregion |
| 3 | E05 `⚡️execution` | 2324-3118 | 2323 blank; the Sdl region starts at 3119 |
| 4 | E02 `📜️schema` (Sdl part) | 3120-3254 | moved with `schema` (concatenate after step 6) |
| 5 | E04 `🖼️sources` | 983-1427 | 982 region opener |
| 6 | E02 `📜️schema` (builders and types) | 647-852, 1429-2322 | region markers 646, 853, 1428, 2323 |
| 7 | E03 `🎥️context-port` | 855-981 | 854 region opener |
| 8 | E01 `🔤️grammar` | 28-644 | 27 blank; `use` lines stay in parent |
| 9 | parent | keep 1-27, 26 `pub use serde_json;`, re-exports, test mount 5032-5036 | verify |

Step 0 (executor): make the CLI and MCP `use` lines (`C01` and `M01`) depend on the adapter crate for `FsRepoContext`.

## 6. Cross-cutting: text gates and manifests to update

| Monolith | Gate or manifest | Lines | Action |
|---|---|---|---|
| A | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 3025-3030, 16613-16618, 17083-17085 | Read the directory folder as one concatenated text, or mark the gate for the new layout. |
| A | `🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts` | 30-33 | Same, brace-match over the concatenation. |
| B | none found in hub | | none |
| C | `🧰️framework/…/📚️library/🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json:4922`, `…/🦑️repo-source-ownership/🔣️.json:6` | owner paths | Replace the owner path with the facade or the child. |
| D | `📜️script.ts:8112` (root) | `policyReadFileSafe` | Update to the facade and the children list. |
| D | `🧰️framework/…/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts:18` | `read` | Update to the children or concatenation. |
| D | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🪟️bmp,📸️jpg,🖼️tiff}/…/🔮️oracles/🔣️.json` | 23-70 | Update path strings; regenerate oracles if they hash the source. |
| D | `🗺️surface/📦️packages/🦀️rust/🦀️.rs:14` | `#[path]` | Unchanged (the parent stays at the same path). |
| E | `📥️intent/🧪️tests/🔣️.json:305-306` | declaration and manifest | Update if a new crate is added. |
| C | `⌨️cli/📦️packages/🦀️rust/Cargo.toml:47` | dependency path | Add the adapter crate dependency for the CLI. |

## 7. Verification plan (executor)

- Hub (A and B): `bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🌎️hub/Cargo.toml -p semio-hub --bins --lib` (the hub is an excluded standalone workspace; see the hub bootstrap result for the earlier blocker, which may still apply), then `--tests` and `test -p semio-hub --bins --lib`. Raw `cargo` is refused by the compiler-ingress gate (memory note). Use the repo tool only.
- Repo CLI (C): check `semio-framework-repo-cli` lib and both bins; check tests (`test -p semio-framework-repo-cli`). Verify the 11 public names in 3.5 with a name-resolution check.
- Tiled-map (D): check `semio-framework-surface` native and `wasm32` targets (`cfg(target_arch = "wasm32")` branches must compile in each child). Run `tiled_map_mercator_oracle` and the `vector-tiles-decode-geometry` include test.
- GraphQL (E): check `semio-framework-repo-graphql` and the new adapter crate; run the GraphQL test folders under `🔗️graphql/🧪️tests`; run the Go twin parity fixtures unchanged to prove byte identity.
- Line gate: each child at most ~2,500 active lines (all children in this plan are below 1,100 except the 1,353 verbs and 1,014 schema, both below 2,500), parent below 1,000.
- Diff check: concatenation of all children and parent ranges equals the original set of non-blank lines, excluding openers, closers, `pub use`, `mod`, `#[path]` and `pub(super)` prefixes. Use the same identifier-level check as the earlier hub split (`verify.py` style).
- Text gates (section 6): run the scripts that read these files after the split (`bun` gates in `script.ts`) and confirm they pass; do not claim pass without running.
- Path length: every new path must be under 256 bytes including the `/Users/ueli/Documents/semio/` prefix. Measured with the prefix on a sample of the longest proposed paths: 159 bytes (E adapter `🔗️graphql/🗄️fs-context/📦️packages/🦀️rust/🦀️.rs`), 157 bytes (E `⚡️execution`), 154 bytes (C `🗂️tree-source-and-search`), 141 bytes (D `📦️mvt-decoder`), 136 bytes (D `🖼️host-scene`). The 256-byte rule holds. The executor re-measures every final path.

## 8. Hazards (all monoliths)

1. Brace-matching readers (A, `script.ts` lines 16613 and `ordered-append-broadcast`): a fn moved to a sibling file is reported as missing. Always update the gate in the same step as the split.
2. Facade glob re-exports (C, E, D, A): a name defined in two children causes an ambiguity error only when both are used. Check with the identifier scan at each step.
3. `pub use` of a `pub(crate)` item fails (E0364). Use `pub(super)` inside the crate and `pub` only where the public API needs it.
4. `impl` block cuts (B, D): each lower child must reopen `impl <Type> {` and each upper child must close it. Forgetting either is the most likely syntax error. Attributes (`#[cfg]`, `#[wasm_bindgen]`) must stay with their item; cut only at blank lines (checked for B 504, 1427, 1711, 2110, 2354, 2609; for D 3053, 3054, 2308, 2309).
5. Private fields and methods across sibling children (A, B, D): the rule is `pub(super)` for sibling access. Nesting a child under the struct's module (D05a and D05b under `map_host`) avoids this for fields.
6. `macro_rules!` textual scope (A `capability_type!`): keep the macro and its uses in one child.
7. `include!` relative paths (D line 1061): a moved include needs one `../` per folder level.
8. `cfg` pairs (D native/wasm pairs and B `integration-fixtures`): keep each pair in one child; the bin's test seam depends on the feature gate.
9. Process-global test recorders (B `static LAST_*` lines 211-334): splitting does not change their behaviour, but tests that read them must still link to the same static.
10. Repo CLI facade (C): `repo_cli::verbs` must keep `pub mod`; the test adapters reach names directly. Verify the 11 names in 3.5.
11. Text gates list (section 6) is the main reason a split can pass cargo and still fail CI. Run the gates.
12. The Go GraphQL twin must stay byte-identical; do not change diagnostic text while moving code.
13. Name collisions in the repo: `🔌️mcp-verb` folder under `⌨️cli/` versus the repo `🔌️mcp` module; `⌨️cli` as a tail matches the db CLI (`🛢️db/⌨️cli`) and `🏭️generator/🔁️codec/⌨️cli`. Use full paths in all reader updates.

## 9. Target outcome

| Monolith | Parent after split (active, approx) | Children | Largest child (active) |
|---|---|---|---|
| A directory | ~140 | 7 | 742 (`🛠️service`) |
| B inference runtime | ~65 | 10 | 842 (`🔐️committer`) |
| C repo CLI | ~9 + facade | 6 + `🔌️mcp-verb` | 1,353 (`🚦️verbs`) |
| D tiled-map | ~260 | 11 (incl. facade and sub-children) | ~790 (`🖼️host-scene`) |
| E repo GraphQL | ~12 (plus tests mount) | 6 core plus 1 adapter crate | 1,014 (`📜️schema`) |

All children ≤ 2,500 active (the target). Parents ≤ 1,000 (the gate). The three hub backends (`🪶️sqlite` 2,961, `🐘️postgres` 3,245, `🌐️neo4j` 3,295 active) remain above the target and are out of this plan.

## 10. Reader search coverage

| Pattern | Scope | Result |
|---|---|---|
| `📇️directory/🦀️` (A) | `🌎️hub` and root `📜️script.ts` (full) | 3 `script.ts` gates, 1 `ordered-append-broadcast` test, 1 bin-unit comment, 3 backend doc comments, 1 hub lib mount |
| `🏃️runtime/🦀️` and `inference/🏃️runtime` (B) | `🌎️hub` and root `📜️script.ts` (full) | 1 hit: the mount at `💡️inference/🦀️.rs:16` |
| `⌨️cli/🦀️`, `🗺️tiled-map/🦀️`, `🔗️graphql/📦️packages/🦀️rust` (C, D, E) | `🧰️framework`, `✏️s`, root `📜️script.ts` (full, fixed string) | Section 3.7, 4.7, 5.7 |
| `hub/📇️directory/🦀️`, `hub/💡️inference/🏃️runtime` (A, B) | `✏️s` and `🧰️framework` | Started in the background at the end of the audit. Output was still empty when this file was written, so this audit reports no result for it. The executor must run this search before cutting A or B. |

## 11. Ordering across the five monoliths

Independent except where noted: C step 0 (move `format_markdown_result`) and E step 0 (adapter crate) are coupled through `repo_cli` and `mcp_verb` `use` lines; schedule E before C's final facade step. A and B are coupled through `HubDirectory` (port) and `HubDirectories`; schedule A's port and dispatch steps before B's runtime steps that reference them. D is independent of the other four except for the GIS plugin, which consumes D's public API; do not rename `gis_map_*` items until the GIS plugin move is planned.

## 12. Out of scope, observed

- Hub backends `🪶️sqlite`, `🐘️postgres`, `🌐️neo4j` are 2,961, 3,245 and 3,295 active lines (measured). They need their own split plans.
- `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs` is 1,594 active (1,751 raw), above the target. Its split is a test-side plan.
- The hub bootstrap (`🏗️bootstrap/🦀️.rs`) already has a split result (`RESULT-HUB-BOOTSTRAP-SPLIT.md`). It consumes A and B symbols; the bootstrap result's text-gate list (16 gates in `📦️packages/🦀️rust/📜️script.ts`) overlaps with section 6.
- The `wgpu` renderer's `🧭️node-graph-and-tiled-map` mount is a different file. Not analyzed.
- Naming: `gis_map` and `GisMap` identifiers appear across B and D. Rename decisions belong to the GIS domain plan.
