# Seam map: plugin instance, plugin host, manifest, hub bootstrap

Read-only audit, 2026-10-15. No source file was edited, no cargo/nx/bun build was run, no modifying git command was used. Coordinator context: `📓️FLEET.md` in this folder (Wave 0 audit, row "plugin instance/host, manifest, hub bootstrap seam map").

Machine outputs (analyzer scripts, raw matches, name lists): `🗑️generated/seam-plugin-manifest-hub/` next to this file. Re-run with `python3 -I seam.py <file> <spec.json>`; specs are `spec-A.json` to `spec-D.json`.

Paths below are repo-relative from `/Users/ueli/Documents/semio`. Abbreviations: `plugin/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`, `fw/` = `🧰️framework/`.

| ID | File | Lines |
| --- | --- | --- |
| A | `plugin/🧵️instance/🦀️.rs` | 10,927 |
| B | `plugin/🖥️host/🦀️.rs` | 7,295 |
| C | `fw/🔨️modules/🛂️manifest/🦀️.rs` | 8,138 |
| D | `🌎️hub/🏗️bootstrap/🦀️.rs` | 11,963 |

## 0. Summary

| | A instance | B host | C manifest | D hub bootstrap |
| --- | --- | --- | --- | --- |
| Owning crate | `semio-framework-plugin` (`plugin/📦️packages/🦀️rust/Cargo.toml` line 3) | `semio-framework-plugin-host` (`plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` line 3) | `semio-framework` facade (`fw/📦️packages/🦀️rust/Cargo.toml` line 3) | `semio-hub`, bin `os-hub` (`🌎️hub/📦️packages/🦀️rust/Cargo.toml` lines 19-21) |
| Target | both: SDK for wasm32-wasip2 guests and native tests (cfg at A:1373, 1384, 7349) | native-only (wasmtime component model, OS-thread worker pool; only gate is B:18 on `mod activation`) | both (pure data and serde types, no cfg) | native-only (axum, tokio, sqlx, rusqlite; `#[cfg(unix)]` at D:11538) |
| Proposed parent residual | 1,242 | 825 | 571 | 904 |
| Proposed children | 4 (impl bands) | 3 (regions) | 6 (regions) | 7 (regions), but see D |
| Largest child | 2,843 | 2,371 | 1,980 | 2,587 |
| Private items that must become `pub(super)` (analyzer, trait items excluded) | 84 | 14 | 34 | 285 (over-approximated; see D) |
| Biggest risk | 2 test files read this file by substring (9 assertions); external `crate::artifact_instance::` consumers | source-text laws and `#[path]` depth | explicit re-export of about 320 `pub` / `pub(crate)` items; test `#[path]`s | bin root is a policy layer; one test file reads it by substring at 4 places; split is only sane after moving routes into the hub lib |

Recommendation in one paragraph: A, B and C can be split mechanically along the cut points below (A is the largest impl block and needs the most `pub(super)` promotions). D should not be split in place: move the route families into the `semio-hub` library first (the bin keeps `main`, `serve` and `router`), then split the library modules. Order: C, then B, then A, then D (section 8). The four trees are disjoint, so A, B and C can run in parallel with separate owners; D waits for the library decision.

## 1. Method and limits

- Top-level index: `^(pub(...)? )?(fn|struct|enum|impl|trait|mod|const|static|type|macro_rules)`, `//#region` markers, `#[cfg]`/`#[path]`, `include!`, `include_str!`, `concat!(env!("CARGO_MANIFEST_DIR"), ...)`, and 4-space member starts inside impl blocks.
- Cut points were checked by reading the lines on each side: attribute lines, doc comments and `//#region` markers were kept with the item they belong to (examples: A:3544 `#[cfg(test)]`, C:3359-3362 attributes of `ActionRef`, C:6915-6919 attributes of `ViewModel`, B:5172-5176 dangling doc above `//#region RuntimeMetricsPublisher`).
- Coupling: each defined item name (word match, `//` comment lines skipped) was searched in lines of other partition groups. Counts are distinct item names. Generic names (`render`, `scope`, `label`, `cancel`, `changed`, `acquire`) also match locals and fields, so the visibility lists are an upper bound. Treat them as the compiler's to-do list, not as proof.
- Trait-impl members (`impl PluginApp for ...`, `impl GuestRuntime for ...`) inherit the trait's visibility and were excluded from the visibility counts.
- Nothing here was compiled. Every "must" below is a prediction until the coordinator's cargo gate runs.

## 2. Ownership and wiring

### A `plugin/🧵️instance/🦀️.rs`

- Crate: `semio-framework-plugin`, `plugin/📦️packages/🦀️rust/Cargo.toml` line 3; lib root `plugin/📦️packages/🦀️rust/🦀️.rs` lines 32-33 `#[path = "../../🦀️.rs"] pub mod component;`.
- Chain: lib `component` = `plugin/🦀️.rs` (1,152 lines); `plugin/🦀️.rs:471-472` `#[path = "🧵️instance/🦀️.rs"] pub(crate) mod artifact_instance;`. No `cfg` on that declaration, so it builds for every target the SDK builds for.
- Consumers (all `crate::artifact_instance::...`, 13 files): `🧵️retained-command/🔄️full-operation/🦀️.rs:37, 93, 95` (`VcsArtifactApp`, `MAINTENANCE_STAGES`, `typed_operation_document_is_fresh`); `🏗️builder/🦀️.rs:393-535` (`VcsArtifactApp::with_registry`); `🧪️testing/🦀️.rs:146, 154` (`MAINTENANCE_STAGES`, `settle_framework_reserved_admission`); `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` (`settle_framework_reserved_admission` and others at 13, 2214, 2223, 2304, 2549, 2726, 6796, 9182); plus `👷️job/`, `📜️declaration/🌳️tree/`, `🪆️child/`, `🧪️tests/...`. Public paths must stay valid after the split (section 8, step 2).

### B `plugin/🖥️host/🦀️.rs`

- Crate: `semio-framework-plugin-host`, `plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` line 3; lib root `.../🦀️.rs` lines 12-13 `#[path = "../../🦀️.rs"] mod component;` then line 14 `pub use component::*;`.
- Native-only evidence: `wasmtime` with `component-model` and `component-model-async` (Cargo line 76), `wasmtime-wasi` (line 77), `B:18-20` `#[cfg(not(target_arch = "wasm32"))] mod activation`. No other target branch in the file.
- Note the name collision: `🌊️flow/🖥️host/🦀️.rs` (352 KB) is a different file, mounted by `🌊️flow/📦️packages/🦀️rust/🦀️.rs:43`. Do not match on the bare name.

### C `fw/🔨️modules/🛂️manifest/🦀️.rs`

- Crate: `semio-framework` facade, `fw/📦️packages/🦀️rust/Cargo.toml` line 3; `fw/📦️packages/🦀️rust/🦀️.rs:36-37` `#[path = "../../🔨️modules/🛂️manifest/🦀️.rs"] pub mod manifest;`.
- The other manifest crate, `🕸️graph/🛂️manifest/` (`semio-framework-graph-manifest`), is a different file and is not in scope. `⚛️reactor/🏃️runtime/🦀️.rs:3012` mounts a different file with the same basename (`🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`); do not confuse the two when grepping.
- The facade crate does not depend on any os-kernel crate (no match in `fw/📦️packages/🦀️rust/Cargo.toml`).

### D `🌎️hub/🏗️bootstrap/🦀️.rs`

- Crate: `semio-hub` (`🌎️hub/📦️packages/🦀️rust/Cargo.toml`), `[lib] path = "🦀️.rs"` (lines 16-17) and `[[bin]] name = "os-hub" path = "../../🏗️bootstrap/🦀️.rs"` (lines 19-21). This file is the bin root; there are no external consumers of its items, but one test file (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`) reads its text at four places (section 6).

## 3. File A: `plugin/🧵️instance/🦀️.rs` (10,927 lines)

Partition folder: `plugin/🧵️instance/` (existing children: `⏪️time-travel`, `⏯️tool-run`, `⏳️operation-progress`, `🔄️full-operation`, `🛠️tool-machine`, `🤝️group`; new names below do not collide). Precedent for the pattern: `🔄️full-operation/🦀️.rs` is 59 lines of `use super::*;` plus `impl<A,M> VcsArtifactApp<A,M>`.

### 3.1 Top-level sections

| Lines | Contents | Count |
| --- | --- | --- |
| 1-371 | `use` imports, including `#[cfg(test)]` imports at 20-22, 164-165, 369-370 | 371 |
| 372-572 | free helpers: `original_render_posted_since` (372), `OriginalRenderTreeIdentity` (377, impl 388), `OriginalRenderInteractionStamp` (401, impl 412), `MediaExportPreparation<A>` (521, impl 534) | 201 |
| 573-937 | `pub struct VcsArtifactApp<A, M>` (573) with its fields | 365 |
| 938-1045 | `impl VcsArtifactApp` helper block (stamp_load_document_identity 947, stamp_load_document_effects 964, stamp_polled_load_document_effects 974, archive_member_entries 994, tool-run ledger accessor 941) | 108 |
| 1046-1199 | free fns: `artifact_handle_of` 1046, `genesis_member_schema` 1061, `catalogue_snapshot_from_document` 1073, `framework_reserved_action_kind` 1084, `PRESENCE_TTL_MS` 1108, `time_travel_freezes_reserved_action` 1112, `time_travel_frozen_fault` 1117, `viewer_read_only_fault` 1128, `note_shell_command_label` 1135, `tool_transaction_shape_fault` 1155, `artifact_lane_commands` 1171, `store_publication_fault` 1189, `typed_operation_document_is_fresh` 1196 | 154 |
| 1200-1202 | `include!("../🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs")` (1201), `include!("../🎮️command/🎟️lease/🦀️.rs")` (1202) | 3 |
| 1204-9148 | one `impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M>` block (header 1204, closing brace 9148). 122 + 96 + 81 member lines by band, see 3.2 | 7,945 |
| 9150-9164 | `impl<A, M> Drop for VcsArtifactApp<A, M>` | 15 |
| 9166-9247 | free items: `history_changed_event` (9168), `settle_framework_reserved_admission` (9173, pub), `FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS` (9186, pub), `settle_framework_reserved_commit` (9191, pub), `drive_framework_reserved_spawn_like_host` (9207, `#[cfg(test)]`, pub), `MAINTENANCE_STAGES` (9222, pub), `MAINTENANCE_DOCUMENT_DISPLACED_STAGE` (9225, pub), `MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE` (9228, pub), `MAINTENANCE_CHILD_ROOT_STAGE` (9231), `MAINTENANCE_CHILD_MEMBER_STAGE` (9234), `MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY` (9238, pub(crate)), `PUBLICATION_RETURNED_ROOT_ALLOWANCE` (9246) | 82 |
| 9248-9258 | second inherent block (`impl<A, M ... + Send + 'static>`), one method `child_retirements_under_pressure` (9254) | 11 |
| 9260-9268 | free fn `loaded_with_app_dialect` (9263) | 9 |
| 9270-10904 | `impl<A, M ... + Send + 'static> PluginApp for VcsArtifactApp<A, M>` (trait impl; members from `admit_cancellation_issuer` 9271 to `media_fingerprint` 10893) | 1,635 |
| 10905-10926 | nine `#[path]` child mods: `child_frame_scene_view` (10905-10907, cfg test), `time_travel_turns` 10909, `tool_run_turns` 10911, `tool_machine_turns` 10913, `operation_progress_turns` 10915, `full_operation_turns` 10917, `child_group_turns` 10919, `close_ladder` 10921 (`../`), `envelope_ladder` 10923 (`../`), `maintenance_ladder` 10925 (`../`) | 22 |
| 10927 | `include!("../♻️lifecycle/🏪️stores/🦀️.rs")` | 1 |

Line 9149 is blank. Total covered: 10,926 of 10,927 lines.

### 3.2 Member clusters inside the 7,945-line impl (1204-9148)

Cut points are member starts (attributes included). Band boundaries are 3543/3544, 6305/6306 and 9148.

| Band (child) | Sub-cluster | Lines | Members (first, last) |
| --- | --- | --- | --- |
| X1 | test and emit-admission helpers | 1204-1307 | `test_registered_tool_keys` 1206 .. `test_reserved_emit_busy_completion_owner` 1284 |
| X1 | reserved-job driving and command-wire proof | 1308-1609 | `run_framework_reserved_job` 1308 .. `admit_host_configuration_json` 1602 |
| X1 | constructors | 1610-1962 | `new` 1610, `isolate_artifact_app_initial_snapshot` 1619, `with_registry` 1623, `with_registry_on_bus` 1628 |
| X1 | store-constructor units, genesis children | 1963-2140 | `has_constructor_receipt` 1963 .. `retire_followed_child` 2122 |
| X1 | generation and snapshot readers | 2141-2230 | `artifact_generation_now` 2141 .. `next_maintenance_stage` 2225 |
| X1 | envelope ingress, decode, store replacement | 2231-2629 | `begin_artifact_envelope_ingress` 2231 .. `cancel_artifact_store_replacement` 2609 |
| X1 | document archive genesis and merge | 2630-2793 | `complete_document_archive_genesis` 2630 .. `reclaim_child_retirements` 2785 |
| X1 | child content, peer roster, child members | 2794-3002 | `admit_child_content_publication` 2794 .. `test_child_admission_state` 2969 |
| X1 | typed-op lane, host-event delivery, child retirement | 3003-3327 | `record_typed_operation_lane` 3003 .. `validate_parent_child_restore` 3313 |
| X1 | child open, register, checkpoint, checkout | 3328-3543 | `open_child` 3328 .. `checkout_child_unit` 3531 |
| X2 | test snapshot helpers (cfg test) and command log | 3544-3927 | `test_snapshot` 3545 (cfg test) .. `child_group_history_target` 3847 |
| X2 | history view build, patch, refresh | 3929-4384 | `build_history_view` 3929 .. `current_open_conflicts` 4375 |
| X2 | preview, result projection, extension invocations | 4385-4542 | `preview` 4385 .. `mint_extension_invocations` 4519 |
| X2 | dispatch_emit and transaction groups | 4543-5206 | `dispatch_emit` 4543 .. `dispatch_emit_group` 5021 |
| X2 | composition graph and interaction state | 5207-5832 | `sync_composition_graph` 5207, `dispatch_group_history_action` 5246, `interaction_state` 5309 .. `leftover_interaction_view_from` 5786 |
| X2 | render, media jobs, media export | 5833-6305 | `stamp_and_cache_interaction_ui` 5833 .. `cancel_owned_media_export` 6275 |
| X3 | framework-reserved routes | 6306-6801 | `framework_reserved_work_items` 6309 .. `dispatch_catalogue_example` 6791 |
| X3 | action and command dispatch | 6802-7322 | `document_loading_refusal` 6802, `dispatch_action` 6814, `dispatch_command` 6918, agent lane preview 7169-7289, `dispatch_command_frame` 7299 |
| X3 | typed-operation publication | 7323-8207 | `drive_typed_operation_worker` 7323 .. `advance_mounted_window_config_refresh` 8190 |
| X3 | typed-command dispatch, configuration, history panel | 8208-9148 | `publish_mounted_typed_operation_unit` 8208 .. `typed_operation_result_state_for_test` 9143 |
| X4 | free items, second impl, loaded_with_app_dialect | 9166-9268 | see 3.1 |
| X4 | PluginApp impl | 9270-10904 | see 3.1 |

Full name list per band (generated, `*` = not private): `🗑️generated/.../appx-A-members.txt`, reproduced in Appendix A.

### 3.3 Proposed partition

Parent stays `plugin/🧵️instance/🦀️.rs`. Child folders are created next to it. Each child opens with a `//!` doc starting with its emoji, then `use super::*;` (inherits the parent's imports and private items; same mechanism as the `#[cfg(test)] use super::*` pattern), then its impl block re-wrapped with the header from line 1204.

| Child | Folder / module | Ranges | Lines | Items | Outbound coupling (distinct names used from other groups) | Promotions to `pub(super)` |
| --- | --- | --- | --- | --- | --- | --- |
| parent | `🧵️instance/🦀️.rs` / (root) | 1-1203, 9150-9165, 10905-10927 | 1,242 | imports, struct, Drop, impl 0, free helpers, the 9 `#[path]` mods, the `include!` | uses X4 3, X2 2 | 0 (parent items used by children are already visible to descendants) |
| X1 | `🚪️ingress/🦀️.rs` / `ingress` | 1204-3543 (impl band 1; header copied from 1204) | 2,340 | reserved-job driving, constructors, store-constructor units, envelope ingress/decode, document archive, child members and packs | PARENT 7, X2 5, X4 5, X3 2 | 33 |
| X2 | `📨️dispatch/🦀️.rs` / `dispatch` | 3544-6305 (impl band 2) | 2,762 | test helpers, history view, dispatch_emit, interaction, render and media jobs | PARENT 13, X1 9, X4 8, X3 2 | 34 |
| X3 | `🧷️typed-commands/🦀️.rs` / `typed_commands` | 6306-9148 (impl band 3, includes the closing brace at 9148) | 2,843 | framework-reserved routes, action/command dispatch, typed-op publication, typed-command dispatch | X2 28, X1 21, PARENT 12, X4 5 | 15 |
| X4 | `🔌️plugin-app-impl/🦀️.rs` / `plugin_app_impl` | 9166-10904 | 1,739 | free items and consts (9166-9247), second inherent block (9248-9258), loaded_with_app_dialect (9260-9268), PluginApp impl (9270-10904) | X2 25, X1 21, X3 18, PARENT 8 | 2 (`history_changed_event`, `PUBLICATION_RETURNED_ROOT_ALLOWANCE`); the `PluginApp` impl needs none |

Sum: 1,242 + 2,340 + 2,762 + 2,843 + 1,739 = 10,926 (line 9149 is blank).

Parent re-exports needed (external consumers, section 2A): `pub(crate) use plugin_app_impl::{MAINTENANCE_STAGES, settle_framework_reserved_admission, drive_framework_reserved_spawn_like_host, MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY};` (`drive_framework_reserved_spawn_like_host` is cfg(test)). Re-export the rest of X4's pub items as well (`FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS`, `settle_framework_reserved_commit`, `MAINTENANCE_DOCUMENT_DISPLACED_STAGE`, `MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE`) so the module surface does not change. `MAINTENANCE_CHILD_ROOT_STAGE` and `MAINTENANCE_CHILD_MEMBER_STAGE` are private consts and stay private to X4. Items that stay in the parent keep their paths: `typed_operation_document_is_fresh` (1196), `artifact_handle_of` (1046), `genesis_member_schema` (1061), and the `include!`d `retire_document_window_registry_*` (from 1201).

Child modules: `#[path = "🚪️ingress/🦀️.rs"] mod ingress;` and the same form for the other three, declared in the parent next to the existing `#[path]` mods. Keep them private to `artifact_instance`; siblings reach each other through the `pub(super)` promotions.

### 3.4 Visibility promotions

- 84 distinct inherent items (33 + 34 + 15 + 2) need `pub(super)`. The full names are in Appendix B. Examples: `admit_reserved_emit_producer` (X1 defines, X3 calls), `validate_framework_reserved_commit` (X1 defines, X2 and X3 call), `dispatch_emit_group` (X2 defines, X3 calls), `dispatch_command` (X3 defines, X4 calls), `live_render_operation` (X3 defines, X2 and X4 call).
- Struct fields of `VcsArtifactApp` stay as they are: children are descendants of the module that declares the struct, so private fields remain visible.
- `pub(crate)` is not needed for the sibling calls; `pub(super)` is the narrowest form that works.

### 3.5 Hazards (file A)

1. Source-text tests read this file by substring. They break as soon as the text moves:
   - `plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2804, 2808, 2811, 2817, 2821` reads `../../🧵️instance/🦀️.rs` through `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), ...))` and asserts substrings such as `} else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {`, `if action==CANCEL_TYPED_OPERATION_ACTION_ID{return self.dispatch_operation_cancellation(args,meta).await;}` (whitespace-collapsed), `if is_time_travel_action_id(action){`, `if action==semio_framework...` (hostEvent arm) and `if matches!(action,semi...` (document transfer arm). The `setActiveUtility` / `setActiveTool` arm is in `handle_action_invocation` (9758, inside the PluginApp impl, so X4); the cancel, time-travel, hostEvent and document-transfer arms are at the head of `dispatch_action` (6814, X3).
   - `plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs:1379, 1444, 1526` reads the same file; `1406` concatenates `🧵️retained-command/🔄️full-operation/🦀️.rs` with it. Anchors used: `async fn publish_mounted_typed_child_operation_unit(` (X3), `admit_command_json_with_proof(&self` (X1), `pub(crate) async fn dispatch_action` (X3).
   - Fix: point these tests at the concatenation of the child files (same approach as line 1406), or move the assertions next to the code they describe. Do this in the same change as the split, not after.
2. `include!` in the parent: `1201-1202` pull in code from `🪟️window/` and `🎮️command/`, and `10927` pulls `♻️lifecycle/🏪️stores/`. Their relative paths resolve from the parent folder and stay correct only while they stay in the parent. Converting them to `mod` would move the names under a module and break the consumers that call `artifact_instance::retire_document_window_registry_step` / `_demands` (defined in `🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`).
3. `#[path]` depth: the child mods at 10905-10926 are relative to `plugin/🧵️instance/`. If a child ever declares one of them, add `../`. Keep them in the parent.
4. `#[cfg(test)]` members: 31 cfg(test) lines in the file. Keep each attribute line with its item when cutting. Test-only members: `test_*` helpers at 1206-1284 and 3545-3627, 8778, 8785, 9143, and the free `drive_framework_reserved_spawn_like_host` at 9207 (cfg test on the line above each). The `#[cfg(test)] use` imports at 21-22, 164-165, 369-370 are inherited through `use super::*`.
5. Target cfg: `1373`, `1384`, `7349` branch on `target_arch = "wasm32"`. Those arms must move as whole statements; do not split a `cfg` from its block.
6. Macros: none in this file.
7. Each child re-wraps the impl header from 1204; the header at 9248 and the `PluginApp` header at 9270 differ (`+ Send + 'static`). Copy the right one.
8. Trait items: the `PluginApp` impl members need no visibility change. Only inherent members count.

## 4. File B: `plugin/🖥️host/🦀️.rs` (7,295 lines)

Partition folder: `plugin/🖥️host/` (existing children include `🏗️instance`, `🔁️lifecycle`, `🧵️shard`, `🧬️component-codec`, `🔔️wake`; no collision with the names below).

### 4.1 Top-level sections (region markers from the file)

| Lines | Region | Count |
| --- | --- | --- |
| 1-73 | `#[path]` mod declarations (3-43), use imports (45-72), `//#region` opens at 74 | 73 |
| 74-601 | `🔧️SharedWasmtimeEngine`: `SharedEngineConfig` 89, `PluginHostWorkerPool` 186-206, `PeriodicPoolTimer` 208-256, `EpochDeadlines` 258-417, `BudgetLimiter` 426, compiled-cache helpers 486-541, `IsolatedCompile` 543-593, `include!("🧪️tests/🔬️standalone/🦀️.rs")` at 596 (cfg test), tests mod 599-600 | 528 |
| 602-800 | `🎭️GuestRuntime` opens (603): `PackageRef` 620, `CompiledHandle` 629, `GuestInstance` 652 with `#[path] mod original_instance_opening` at 657, `GuestInstanceState` 715, `GuestRuntime` trait 778 | 199 |
| 801-1186 | `🔖️MockGuestRuntime` (cfg test): scripted outcomes, `MockGuestRuntime` 875, impl GuestRuntime 1072 | 386 |
| 1188-1266 | `🧠️OwnedRuntime`: `OwnedRuntime` impl GuestRuntime 1218 | 79 |
| 1268-2799 | `🐎️WasmtimeRuntime`: `actor_bindings` inline module (1276, `bindgen!` at 1293), `ActorHostState` 1309, `WasmtimeRuntime` 1640, `impl GuestRuntime for WasmtimeRuntime` 1982, host-async 1484-1622, `GuestCodecDispatch` 1886-1980, `GuestRuntimes` enum 2273 with impls 2285-2385, `EffectEventMarshal` 2387-2794 | 1,532 |
| 2800 | `//#endregion 🎭️GuestRuntime` | 1 |
| 2802-5170 | `🔀️PostTurnRelay`: `GuestRelayPoolFuture` 2810, relay requests 2913-3002, `GuestInstanceSlot` 3169, `GuestInstanceLease` 3213, `run_guest_relay_request` 3387, `GuestRelayPublication` 3508, `GuestColdRelayJob` 3665, lifecycle probes 3897-4003, mounted registry 4184-4642, `PluginInstanceHandle` 4999-5156, cold-relay test mod 5166-5169 | 2,369 |
| 5172-5176 | dangling doc comment (belongs to the next item) | 5 |
| 5177-5238 | `📈️RuntimeMetricsPublisher` | 62 |
| 5240-5347 | `🔖️ArtifactSession`: `SessionLanePack` 5245, `ArtifactSession` 5312 | 108 |
| 5349-5379 | `🔖️MutationReports`: decode_dispatch_report 5356, decode_merge_report 5365, decode_conflicts 5371 | 31 |
| 5381-5821 | `🔖️IoRouter` | 441 |
| 5823-6223 | `💡️InferenceRouter` | 401 |
| 6225-6389 | `🔖️PluginGraph` | 165 |
| 6391-6629 | `🎯️MutationRouter` | 239 |
| 6631-6711 | `🔖️InstanceDirectory` | 81 |
| 6713-7027 | `🎯️TransactionCoordinator` | 315 |
| 7029-7239 | `🔖️AppRouter` | 211 |
| 7241-7277 | `🔖️OpeningResolver` | 37 |
| 7280-7295 | `#[cfg(test)]` mods (`sqlite_refusal_tests` 7281, `tests` 7285-7286), trailing `impl ... HostWithStore<ActorHostState>` 7289 | 16 |

Covered: all 7,295 lines, with 5172-5176 (dangling doc) and 5380, 2801, 601 blank or region markers.

### 4.2 Proposed partition

| Child | Folder / module | Ranges | Lines | Items | Coupling | Promotions |
| --- | --- | --- | --- | --- | --- | --- |
| parent | `🖥️host/🦀️.rs` / root | 1-601, 5172-5379, 7280-7295 | 825 | mods, shared engine, metrics, session, reports, tests | uses B1 4, B2 2 | 0 |
| B1 | `🎭️guest-runtime/🦀️.rs` / `guest_runtime` | 602-2800 | 2,199 | GuestRuntime trait, Mock, Owned, Wasmtime runtime, actor_bindings, GuestRuntimes | PARENT 25, B2 7, B3 5 | 9 (`ActorHostState`, `cancel_admissions`, `cancel_job`, `compile`, `release`, `start_job`, `step_job`, `take_relay_start_failure_release`, `take_relay_step_failure_release`) |
| B2 | `🔀️post-turn-relay/🦀️.rs` / `post_turn_relay` | 2801-5171 | 2,371 | guest relay pool, instance lease, cold relay, mounted registry, PluginInstanceHandle | B1 11, PARENT 9, B3 4 | 5 (`bytes`, `close_step`, `guest`, `invalid`, `schedule`; generic names, verify) |
| B3 | `🧭️routers/🦀️.rs` / `routers` | 5380-7279 | 1,900 | IoRouter, InferenceRouter, PluginGraph, MutationRouter, InstanceDirectory, TransactionCoordinator, AppRouter, OpeningResolver | B2 8, PARENT 5, B1 1 | 0 |

Re-exports: the lib re-exports the parent with `pub use component::*;` (`plugin/🖥️host/📦️packages/🦀️rust/🦀️.rs:14`). Keep every `pub` top-level item reachable at the same path: add explicit `pub use guest_runtime::{...}` / `post_turn_relay::{...}` / `routers::{...}` lines in the parent. Top-level pub counts: B1 10, B2 5, B3 19. The `pub(crate) mod actor_bindings` (inline) and `pub(crate) use actor_bindings::...` must remain reachable as `crate::actor_bindings` or `guest_runtime::actor_bindings` with a re-export for B2 and B3 users (`impl actor_bindings::semio::framework::pure::Host` at 7289 is in the parent).

### 4.3 Hazards (file B)

1. `#[path]` depth. Every `#[path]` in 602-7279 is relative to `plugin/🖥️host/`. Moved into a child folder, each needs `../`. Counts: B1 has 7 (`657` `🏗️instance/`, `753`, `756` `📥️cold-pair/`, `1184`, `1264`, `2517`, `2797`); B2 has 3 (`3662`, `4093` `🔔️wake/`, `5168`) plus one `include_str!` at 5164; B3 has 8 test mods (`6221`, `6387`, `6627`, `6709`, `6762`, `7025`, `7237`, `7275`). Simplest: keep all `cfg(test)` test `#[path]` mods in the parent and adjust only the non-test ones.
2. `include_str!("⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")` at 5164 (B2) needs `../` after the move.
3. `include!("🧪️tests/🔬️standalone/🦀️.rs")` at 596 is in the parent and stays.
4. `bindgen!` at 1293 (`path: "../../../🧬️schema"`) is resolved against `CARGO_MANIFEST_DIR`, not the file, so moving the whole `actor_bindings` block is safe. Do not split a `bindgen!` invocation and do not invoke it twice in one module (the file's comment at 1271 says so).
5. Module names: `actor_bindings` is `pub(crate)` and inline. Keep it inline in B1.
6. The cfg gate at 18 (`activation`) stays with its `#[path]` in the parent.
7. Trait items: `impl GuestRuntime for ...` members (1072, 1218, 1982, 2285) need no visibility change.
8. Test-only fixtures: `MockGuestRuntime` (cfg test) is referenced by `GuestRuntimes::From<Arc<MockGuestRuntime>>` (2374). Keep the cfg attributes with their items.

## 5. File C: `fw/🔨️modules/🛂️manifest/🦀️.rs` (8,138 lines)

Partition folder: `fw/🔨️modules/🛂️manifest/`. Existing folders there: `🌲️canonical`, `🎛️public-invocation`, `🎞️media`, `🎬️tutorial` (contains only `🏠️local-interaction`), `🔖️action-args` (contains `♻️retirement`), `🔖️channel-version`, `🔮️oracles`, `🤖️generated`, `🧪️tests`, `🧬️schema`, `🪛️utilities`, `🪟️view-context`, `node_modules`. Chosen names below avoid all of them.

### 5.1 Top-level sections

| Lines | Region / contents | Count |
| --- | --- | --- |
| 1-48 | `// #region 🛂️Manifest` (1), `//!` doc (2), imports 9-47 | 48 |
| 49-115 | `🔖️Manifest` core: `Keybinding` 54, `Platform` 64, `PlatformKeybinding` 75, `ActionKind` 97 | 67 |
| 116-439 | `🔖️ArgSchema`: `ArgFormat` 133, `ArgSchema` 171, `ArgPresentation` 342, `SnapSource` 360, `OptionSource` 373, `NumberScale` 381, `ReferenceIdType` 391 | 324 |
| 441-1079 | `🔖️ActionArgs`: `ActionArgControl` 469, `ActionArgDef` 682, impls 712-942, `apply_arg_format` 943, `arg_schema_json_schema` 980 | 639 |
| 1081-1189 | `🔖️ActionArgFacets`: `ActionArgNumberFacets` 1090; `#[cfg(test)] mod number_facets_tests` 1186-1188 | 109 |
| 1191-2083 | `🔖️MutationInputs`: `InputSchemaResolver` 1193, `include_str!("🔣️input-labels.json")` 1237, `mutation_input_defs` 1257, `include_str!("../🌱️value/🧬️schema/🔣️.json")` 1349, `input_numeric_transport` 1385, `InputSchemaReader` 1493-1955, `input_number` 2008; cfg test mod 2080-2082 | 893 |
| 2086-2095 | `default_action_icon_id` | 10 |
| 2097-2461 | `🔖️ActionSemantics`: `ResourceSelector` 2115, `CapabilityEffects` 2143, `ApprovalMode` 2167, `CapabilityPolicy` 2182, `PreviewMode` 2196, `UndoMode` 2208, `IdempotencyMode` 2224, `ExecutionClass` 2237, `CapabilityAudience` 2260, `derive_audience` 2276, `GUMBALL_CHROME_ACTION_IDS` 2296, `InteractiveJobClassification` 2320, `CapabilityExecution` 2334, `ActionSemantics` 2369 | 365 |
| 2468-2663 | `ActionDefinition` 2468, `history_action_definitions` 2620, `history_lane_action_definitions` 2632 | 196 |
| 2664-2871 | `🔖️HistoryEdit`: 13 `HISTORY_EDIT_*` action ids, `history_edit_action_definitions` 2740, `history_edit_finalize_dialog` 2853; cfg test mod 2868-2870 | 208 |
| 2875-2907 | history command filter consts and `set_history_command_filter_action_definition` 2888 | 33 |
| 2909-2969 | `🔖️HostEvent` (2909-2946), `note_shell_command_action_definition` 2955 | 61 |
| 2971-2998 | `🔖️Clipboard` | 28 |
| 3000-3203 | `🔖️Interaction`: `InteractionVerb` 3029, `interaction_action_definitions` 3169, `🔖️InteractionRefreshScope` 3076-3160 | 204 |
| 3205-3287 | `🔖️ToolRun`: `pub use semio_framework_tool_run::{...}` 3206, `tool_run_action_definitions` 3222, cfg test mod 3284-3286 | 83 |
| 3291-3302 | `SET_ACTIVE_UTILITY_ACTION_ID` 3291, `set_active_utility_action_definition` 3296 | 12 |
| 3303-3318 | `SET_ACTIVE_TOOL_ACTION_ID` 3303, `set_active_tool_action_definition` 3308 | 16 |
| 3319-3362 | `START_INTRODUCTION_ACTION_ID` 3319, document-transfer consts 3332-3346, `document_transfer_action_definitions` 3350 | 44 |
| 3359-3412 | `ActionRef` 3363 (with attrs 3359-3362), `ActionAddress` 3394, `ActionInvocation` 3409 | 54 |
| 3414-3493 | `🔖️Utilities`: `UtilityDefinition` 3421, `initial_window_utility` mod 3468-3470 | 80 |
| 3495-3635 | `🔖️Commands`: `CommandDefinition` 3504, `CommandAddress` 3607, `CommandInvocation` 3617, `OsDefinition` 3630 | 141 |
| 3637-3701 | `🔖️Tools`: `ToolDefinition` 3648, `ToolRef` 3676 | 65 |
| 3703-3787 | `🆔️ElementId`: `is_element_id` 3710, `element_id_segment` 3729, `child_element_id` 3755, window/panel element ids 3770-3784 | 85 |
| 3789-4254 | `🔖️Introduction`: `IntroductionDefinition` 3798 ... `IntroductionDemonstration` 4221 | 466 |
| 4256-5058 | `🔖️Tutorial`: `TutorialDefinition` 4269, `TutorialNarrationCue` 4408, `TutorialVideoCue` 4470, `TutorialUiSnapshot` 4550, `TutorialCameraKeyframe` 4718, start/record tutorial actions 4785-4815, `🔖️TutorialEngine` 4817-5057 | 803 |
| 5060-5256 | `🔖️Dialog`: `DialogDefinition` 5070, `DialogChoice` 5109, cfg test mod 5253-5255 | 197 |
| 5262-5563 | `ModeDefinition` 5262, `NonEmptyVec<T>` 5284 with 10 impls, `WindowKindDefinition` 5418, `PanelGroup` 5467, `PanelTabKind` 5505, `PanelTabDefinition` 5545 | 302 |
| 5564-5651 | `🔖️Surface`: `AppRole` 5567 (Viewer, Editor), `AppRef` 5630, `surface_app_id` 5636 | 88 |
| 5653-5789 | `🔖️FaultNotices`: `FaultNoticeDefinition` 5661, `validate_fault_notices` 5734, cfg test mod 5786-5788 | 137 |
| 5794-5922 | `AppDefinition` 5794 (no region) | 129 |
| 5923-6060 | `🔖️action-args`: `effective_action_args` 5935, `resolve_window_actions` 6037, `resolve_mode_tools` 6045 | 138 |
| 6063-6163 | breadcrumb, examples: `ExampleDefinition` 6088, asset names 6104-6125, `examples_for_*` 6133-6147, `program_contribution_tree` mod 6155-6156 | 101 |
| 6164-6241 | `🔖️TopicContribution` | 78 |
| 6243-6489 | `🔖️PluginDependency`: `Version` 6248, `VersionPin` 6340, `tree_pin!` macro 6466 (`#[macro_export]`) at 6466-6478, `PluginDependency` 6479 | 247 |
| 6491-6624 | `🔖️ArtifactContribution`: `ContributedMutationSemantics` 6499, `InferencePayloadContract` 6561, `ArtifactContributionDescriptor` 6615 | 134 |
| 6629-6668 | `PluginManifest` | 40 |
| 6669-6775 | `🔖️HostResolvedArgs`: `ArtifactKindChoice` 6679, `SurfaceAppChoice` 6692, encode/decode fns 6702-6758 | 107 |
| 6777-6904 | `🔖️DependencyGraph`: `resolve_load_order` 6824, `dependents` 6899 | 128 |
| 6906-6918 | cfg test mod `plugin_dependency_tests` 6906-6908; derive lines for `ViewModel` 6915-6919 | 13 |
| 6920-7125 | `ViewModel` 6920 (attributes from 6915) through impl 7064 | 206 |
| 7129-7216 | `📏️ViewContextCapacity`: capacity consts 7134-7207, cfg test mods 7209-7215 | 88 |
| 7218-7280 | `📏️PublicInvocationCapacity` | 63 |
| 7282-7339 | `🔖️UiRefreshSection` | 58 |
| 7347-7350 | `🔖️Kernel`: `#[path = "../🎠️kernel/🦀️.rs"] pub mod kernel;` | 4 |
| 7352-7631 | `🔖️PackageDescriptor`: `PackageRole` 7358 (Plugin, Extension), `ExecutionMode` 7372, `IoEntryDescriptor` 7473, `PackageDescriptor` 7569, `CanonicalDescriptorValue` 7599 | 280 |
| 7633-7690 | `🔖️AgentContributions` | 58 |
| 7692-8111 | `🔖️MediaVocabulary`: `ArtifactKindSpec` 7724, `OsMediaCapability` 7707 (MeshOnly, Brep), `MediaClass` 7768, `MediaForm` 7785 (Any, Vector, Raster, Brep, Mesh, Document, Value, Dag), `MediaType` 7810, `media_types_compatible` 7886, `AppIo` 7937, `ConfigSpec` 7989, `CommandGrammar` 8020, `#[path="🎞️media/🦀️.rs"] mod intrinsic_media` 8050, `MediaConverter` 8105 | 420 |
| 8113-8138 | cfg test mods `media_vocabulary_tests` 8113-8115, `app_label_tests` 8117-8119, `example_picker_tests` 8121-8123, `resolved_host_context_tests` 8125-8127, `gumball_verb_audience_tests` 8129-8131, `argument_retirement_tests` 8136-8138 (path `🔖️action-args/♻️retirement/...`) | 26 |

Covered: 1-8138 with blank lines at 3358, 3788, 5059, 6914 and region markers.

### 5.2 Proposed partition

| Child | Folder / module | Ranges | Lines | Items | Coupling | Promotions |
| --- | --- | --- | --- | --- | --- | --- |
| parent | `🛂️manifest/🦀️.rs` / root | 1-115, 3359-3788, 8113-8138 | 571 | header, ActionKind core, ActionRef/Address/Invocation, Utilities, Commands, Tools, ElementId, the cfg test mods | uses C2 16, C4 6, C1 3, C6 2, C3 1 | 0 |
| C1 | `🧮️arg-schema/🦀️.rs` / `arg_schema` | 116-2095 | 1,980 | ArgSchema, ActionArgs, facets, MutationInputs, default_action_icon_id | C4 10, C5 6, PARENT 3, C6 3, C2 2, C3 2 | 12 |
| C2 | `🕹️actions/🦀️.rs` / `actions` | 2096-3358 | 1,263 | ActionSemantics, ActionDefinition, HistoryEdit, filters, HostEvent, Clipboard, Interaction, ToolRun re-export, set-active, document transfer | C1 19, C4 19, PARENT 13, C6 5, C5 4, C3 3 | 2 (`from_value`, `to_value`) |
| C3 | `📚️introduction-tutorial/🦀️.rs` / `introduction_tutorial` | 3789-5059 | 1,271 | IntroductionDefinition and Demonstration, TutorialDefinition and TutorialEngine | PARENT 7, C1 6, C4 6, C2 5, C5 2, C6 1 | 0 (uses `from_str` from std) |
| C4 | `🧱️app-surface/🦀️.rs` / `app_surface` | 5060-6163 | 1,104 | Dialog, Mode, NonEmptyVec, WindowKind, PanelGroup, PanelTab, Surface, FaultNotices, AppDefinition, action-args, breadcrumb, examples, program_contribution_tree | PARENT 11, C6 11, C1 10, C5 10, C2 8, C3 3 | 10 |
| C5 | `🧾️plugin-manifest/🦀️.rs` / `plugin_manifest` | 6164-6914 | 751 | TopicContribution, Version, VersionPin, tree_pin! macro, PluginDependency, ArtifactContribution, PluginManifest, HostResolvedArgs, DependencyGraph | C4 16, C1 11, C6 6, C2 3, PARENT 3, C3 2 | 8 |
| C6 | `🖼️view-model/🦀️.rs` / `view_model` | 6915-8112 | 1,198 | ViewModel, capacities, UiRefreshSection, kernel mod, PackageDescriptor, AgentContributions, MediaVocabulary | C5 9, C4 8, C1 5, C2 2, PARENT 2, C3 1 | 2 |

Sum: 571 + 1,980 + 1,263 + 1,271 + 1,104 + 751 + 1,198 = 8,138.

Notes:
- The coupling is cyclic (C1 to C4 to C5 to C2 to C1). Rust allows that, but it means this file is one type module cut into pieces, not a layered split. Keep the children together in one folder and do not expect a layering win from C.
- C5 is 751 lines, just under the 800 floor; it is cohesive (plugin and dependency types), so keep it separate.
- Re-exports: the facade is `pub mod manifest` (`fw/📦️packages/🦀️rust/🦀️.rs:36-37`). Top-level items with `pub` or `pub(crate)` visibility per child: C1 26, C2 93, C3 45, C4 44, C5 25, C6 65, PARENT 25 (323 in total, generated in `pub-toplevel.json`). Each child's pub items need an explicit `pub use child::{...}` in the parent (repo rule: explicit re-export, no glob).

### 5.3 Hazards (file C)

1. `#[path]` depth. Relative to `🛂️manifest/` today. After the move: `🖼️view-model/` line 8050 `#[path="🎞️media/🦀️.rs"]` becomes `../🎞️media/🦀️.rs`; line 7348 `#[path="../🎠️kernel/🦀️.rs"]` becomes `../../🎠️kernel/🦀️.rs`; `🧱️app-surface/` line 6155 `#[path="🌲️canonical/🦀️.rs"]` becomes `../🌲️canonical/🦀️.rs`; `🧮️arg-schema/` lines 1237 and 1349 `include_str!` become `../🔣️input-labels.json` and `../../🌱️value/🧬️schema/🔣️.json`. Every test `#[path]` (`🧪️tests/...`) needs `../`. Simplest: keep all test mods and the `kernel` mod in the parent.
2. `pub mod kernel` (7348-7349) mounts the framework kernel's source file inside the manifest module. Keep it in the parent so the public path `semio_framework::manifest::kernel` does not change.
3. `macro_rules! tree_pin` (6466) has `#[macro_export]` (6465). It goes to the crate root regardless of which child holds it; doc links `crate::tree_pin` stay valid. Keep the macro before any use within its file.
4. Explicit re-exports: the facade re-exports many names by path. Missing one breaks a consumer. Generate the list from `pub-toplevel.json` before cutting.
5. `use semio_framework_tool_run::{...}` (3206) is a re-export inside C2. Keep it in C2 and re-export it from the parent.
6. `use ui_wgpu::wgpu::...` (24-31) puts UI product types into every child that inherits the parent's imports through `use super::*`. This is a dependency-direction issue (section 7), not a split issue.
7. No `include!`, no bindgen, no cfg(target_arch) in this file. Safe to move.
8. Source-text tests: none found reading this file (the grep over `fw/` and `🌎️hub/` found no `include_str!` of `🛂️manifest/🦀️.rs`).

## 6. File D: `🌎️hub/🏗️bootstrap/🦀️.rs` (11,963 lines)

### 6.1 Top-level sections

| Lines | Region / contents | Count |
| --- | --- | --- |
| 1-111 | imports (`directory::`, `db::`, `protocol::`, `semio_hub::`, `server::`, axum, tokio); `#[cfg(test)] extern crate directory as semio_framework_os_kernel;` 17-18 | 111 |
| 112-199 | `⚠️ Errors`: `HubError` 117 with From impls to 199 | 88 |
| 200-749 | misc: `DirectoryJson` 202, `StartupCatalogControl` 223, `StartupProgressCellV1` 254, `verify_pending_catalog_guests` 275, `HubBootstrapControl` 347, `ArtifactCasMaintenanceSupervisor` 399, `SagaDrainSupervisor` 544, `drain_instance_sagas` 605, `DocumentOpenCatalogAuthorityV1` 634, `StartupArtifactAuthority` 674, `configured_artifact_authority` 714 | 550 |
| 750-2595 | `🔖️State`: presence 767-858, color leases 904-916, test gates 918-1106, socket grants 1108-1620, document open plan 1621-2072, `SocketLiveLeaseV1` 2073, `HubState` 2107-2564, `bearer` 2565 | 1,846 |
| 2597-2688 | `🔖️Auth`: `AuthOutcome` 2600, `resolve_auth` 2608, `authorized*` 2671-2687 | 92 |
| 2690-3350 | `🔖️AdminAuth`: `AdminSubject` 2692, `AdminPrincipalV1` 2698, `HubMode` 2740, readiness structs 2759-2850, checkpoint policy 2853-2933, readiness fns 2963-3110, `configured_admin_subjects` 3111, forwarded TLS 3140-3193, middlewares 3208-3265, admin auth 3305-3348 | 661 |
| 3353-4967 | `🔖️Rest`: socket receipts and issuance 3355-3513, document open plan routes 3514-3720, `🪪️ExecutionTargetLease` 3721-3935, `issue_document_plan_socket_grant` 3937, directory socket grants 3958-3994, canonical pair 3995-4257, document status 4245, `Blobs` 4258-4323, `📣️CheckpointPublication` 4325-4502, `📌️CheckIn` 4504-4966 (includes `include!("../🧪️tests/🔬️standalone/🦀️.rs")` at 3956) | 1,615 |
| 4969-5995 | `🔖️WebSocket`: `socket_grant_from_protocol_header` 4970, `document_ws_v1` 5169, `serve_document_socket` 5505-5971, close/drain 5976-5993 | 1,027 |
| 5997-8583 | `🔖️Directory`: `AuthedUser` 6003, compose server 6031, rebootstrap 6096-6150, wire encoders 6152-6278, `HubArtifactCreationCommitAuthorityV1` 6339, artifact creation HTTP 6402-6932, `DirectoryEventPageSource` 6942, directory commands 7094-7356, space administration 7376-7731, invite redeem 7732, events 7760-8085, directory WebSocket 8086-8386, `handle_directory_ws_v1` 8386-8583 | 2,587 |
| 8584-9472 | `session/agent/rate-limit/CORS`: `get_session_me` 8584, `post_session_sign_out` 8647, `post_auth_session` 8715, `post_auth_credential` 8811, agent delegation 8941-9246, `auth_error_response` 9250, `rate_limit_middleware` 9271, `CrossOriginPolicyV1` 9331, `cors_middleware` 9445 | 889 |
| 9473-10705 | `🔖️Admin`: cursors and paging 9477-9620, intents 9621-9913, `execute_admin_intent` 10150-10306, `admin_intents` 10323, overview 10441, observability 10474-10501, spaces 10503-10604, users, connections, documents, events 10605-10704 | 1,232 |
| 10707-10769 | `🔖️Extensions`: `list_extensions`, `get_extension_asset` | 63 |
| 10771-10804 | `🔖️PluginModules`: trusted plugin module index, manifest, file | 34 |
| 10806-10977 | `🔖️AdminPage`: asset paths 10815-10843, `admin_page` 10844, liveness 10875-10893, `BootReadinessServerV1` 10894-10946, healthz/readyz 10949-10976 | 172 |
| 10979-11169 | `💡️Inference`: GIS job routes 11061-11140, `inference_routes` 11157, `INFERENCE_REQUEST_MAX_BYTES` 11168 | 191 |
| 11171-11938 | `🔖️Main`: `artifact_creation_routes` 11172, `router` 11189, `hub_worker_pool` 11278, connect_db / connect_artifact_cas / connect_directory 11291-11385, socket drain 11386-11467, termination 11506-11570, `main` 11574, `serve` 11617-11924 | 768 |
| 11940-11944 | `#[path]` mods `trusted_catalog_command`, `credential_command` (parent-relative `../`) | 5 |
| 11946-11963 | `🔖️Tests`: `#[path]` mods `test_artifact_root`, `tests`, `two_client_document` (cfg test) | 18 |

### 6.2 Proposed partition (only after the library move in 6.4)

Shown here for completeness. The coupling makes this split a poor first step.

| Child | Folder / module | Ranges | Lines | Items | Coupling | Promotions (analyzer, upper bound) |
| --- | --- | --- | --- | --- | --- | --- |
| parent | `🏗️bootstrap/🦀️.rs` / root | 1-111, 11171-11963 | 904 | imports, router, main, serve, connect fns, tests and mods | uses D2 39, D1 30, D6 30, D4 23, D5 13 | 0 |
| D1 | `🔖️state/🦀️.rs` / `state` | 112-2596 | 2,485 | errors, startup, HubState, socket and plan ledgers | D2 22, D4 16, D6 8 | 112 distinct |
| D2 | `🔐️auth-rest/🦀️.rs` / `auth_rest` | 2597-4967 | 2,371 | auth, admin readiness, rest handlers, check-in | D1 54, D4 16, D6 6 | 67 |
| D3 | `🔌️websocket/🦀️.rs` / `websocket` | 4968-5996 | 1,029 | document socket | D1 50, D4 16, D2 13 | 6 |
| D4 | `📇️directory/🦀️.rs` / `directory` | 5997-8583 | 2,587 | directory commands, spaces, invites, events, directory socket | D1 43, D2 14, D6 8 | 50 |
| D5 | `🪪️session-agent/🦀️.rs` / `session_agent` | 8584-9472 | 889 | session, auth, agent delegation, rate limit, CORS | D1 21, D2 9, D4 8 | 15 |
| D6 | `🛠️admin/🦀️.rs` / `admin` | 9473-10977 | 1,505 | admin ops, extensions, plugin modules, admin page | D1 30, D2 19, D4 19 | 34 |
| D7 | `💡️inference/🦀️.rs` / `inference` | 10978-11170 | 193 | GIS job routes | D1 10, D4 7, D2 4 | 1 |

Sum: 904 + 2,485 + 2,371 + 1,029 + 2,587 + 889 + 1,505 + 193 = 11,963.

### 6.3 Hazards (file D)

1. Source-text tests read this file. These break on any split: `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:223, 1685, 2117` (`include_str!("../../🏗️bootstrap/🦀️.rs")`) and `4652`, which asserts `matches("ActorId(HUB_CATCH_UP_ORIGIN.into())").count() == 2`. The count assertion must be re-derived from the new file set.
2. `include!("../🧪️tests/🔬️standalone/🦀️.rs")` at 3956 (inside D2): from a child folder it becomes `../../🧪️tests/🔬️standalone/🦀️.rs`.
3. Bin-root `#[path]` mods (11940-11961) use `../` relative to `🏗️bootstrap/`. They stay in the parent and are unaffected.
4. `#[cfg(test)] extern crate directory as semio_framework_os_kernel;` (17-18) aliases a product crate under the framework name. Remove it when the code moves to the lib (the lib needs the same alias only in its own tests, if at all).
5. `#[cfg(unix)]` and `#[cfg(not(unix))]` on `termination_signal` (11538, 11561): keep both in the same module.
6. Test-only statics and cfg items: 89 cfg(test) lines (`LAST_DOCUMENT_PLAN_REFUSAL` 930, `DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS` 1680, `ADMIN_SECRET_WIPE_BYTES` 9950). Move with their items.
7. Startup and shutdown ordering: `serve()` (11617-11924) sequences readiness, catalog load, sockets and the shutdown record (`hub_shutdown_record` 11925). Keep `serve` in the parent in the same change; move it only after the lib split is verified.

### 6.4 Recommended order for D: library first

The bin is a policy layer (route handlers, admin intents, readiness, auth, CORS) and reaches into the OS kernel directly. The clean target is:
- `🌎️hub/📦️packages/🦀️rust/🦀️.rs` (the lib root) gains the route families as modules (for example `http::state`, `http::auth`, `http::websocket`, `http::directory`, `http::admin`, `http::inference`).
- The bin root keeps `main`, `serve`, `router` and the connect functions.
- Then split each family as in 6.2 inside the lib.
This also removes the bin-to-OS-kernel imports (section 7, W4) from the bin, because the lib is the only crate that should own them. Decide this before starting; it is the coordinator's call (section 9).

## 7. Wrong-direction dependencies

Each item cites file:line. "Layer" is the usual direction: framework below product, plugin SDK below apps, hub bin on top.

**W1. Plugin SDK instance (A) compiles product, tool-run and time-travel domains.**
- `A:12-15` imports `tool_run_contract::{ToolRunJobRequest, ToolRunJobPurpose, ToolRunJobPort, ToolRunTraceKeys}`; `A:355-359` `is_time_travel_action_id`, `tool_jobs::is_tool_run_action_id`, `tool_jobs::time_travel`, `tool_jobs::tool_run`.
- `A:17-23` `reserved_jobs::{HISTORY_ACTION_IDS, CLIPBOARD_ACTION_IDS}`, `interaction_topology::INTERACTION_ACTION_IDS`; `A:21-22` cfg test `command_history::HISTORY_ROWS_BUILT`, `reserved_jobs::FrameworkCutJob`.
- `A:71-75` depends on `ui_wgpu::wgpu::{ContextMenuItemSpec, ContextMenuRequest, WindowEngagement, WindowMeasure, FRAMEWORK_HISTORY_BODY_KEY}`; plugin `Cargo.toml:87` declares `ui_wgpu` (dependency is a UI product).
- Plugin lib root `plugin/📦️packages/🦀️rust/🦀️.rs:9-12` aliases the OS kernel crate four times (`extern crate semio_framework_os_kernel as dsl; ... as protocol; ... as store; ... as vcs;`); `plugin/📦️packages/🦀️rust/Cargo.toml:73` depends on `semio-framework-os-kernel`.
- `A:10927` `include!("../♻️lifecycle/🏪️stores/🦀️.rs")` and `A:1201-1202` `include!` of window and command files: the instance module is assembled from other domains' text.

**W2. Plugin host (B) signature leaks OS-kernel VCS internals.**
- `B:783, 796, 797` (trait `GuestRuntime`, declared at 778) take and return `semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::...`. Same at `B:1094, 1160, 1169` (test-only impl).
- Host lib root `plugin/🖥️host/📦️packages/🦀️rust/🦀️.rs:16` `pub use semio_framework_os_kernel::os_vcs;` and `:28-29` `pub use semio_framework_os_config::{opening_config, io as config_io};` re-export OS internals from the plugin host.
- Host `Cargo.toml:36` os-kernel, `:37` os-config, `:57` os-services, `:35` `ui_wgpu` (product UI crate in a native host).

**W3. Framework manifest (C) hard-codes product and domain vocabulary.** The manifest is the framework-level contract, so each of these is a candidate to move to a product or plugin crate.
- Named product action: `C:2296` `GUMBALL_CHROME_ACTION_IDS = ["setTransformGumballFlag"]`.
- Domain feature inside the manifest: `C:2664-2871` `HISTORY_EDIT_*` action ids, dialog `finalizeHistoryEdit` (`C:2736`), and history-edit definitions.
- Re-export of a domain crate: `C:3206` `pub use semio_framework_tool_run::{...}`; `C:3222` `tool_run_action_definitions`.
- Tutorial model with narration and video cues: `C:4269` `TutorialDefinition`, `C:4408` `TutorialNarrationCue`, `C:4470` `TutorialVideoCue`, `C:4785-4815` start and record tutorial actions.
- Closed enums listing plugin or product topics: `C:97-115` `ActionKind` (Mutation ... Interaction, whose variant docs describe ephemeral view state such as camera, selection, hover and active utility); `C:2320-2328` `InteractiveJobClassification` with `Migrated`, `BatchOnlyPendingRewrite`, `Deleted` (migration-state labels inside the manifest, against the no-migrations rule); `C:5567-5570` `AppRole` {Viewer, Editor}; `C:7358-7361` `PackageRole` {Plugin, Extension}; `C:7707-7716` `OsMediaCapability` {MeshOnly, Brep}; `C:7768` `MediaClass` (TwoD, ThreeD, Text, Data, Graph, Kit, Computation, Presentation); `C:7785` `MediaForm` (Any, Vector, Raster, Brep, Mesh, Document, Value, Dag).
- Kernel source compiled inside the manifest: `C:7348-7349` `#[path = "../🎠️kernel/🦀️.rs"] pub mod kernel;`.
- UI product types in the manifest: `C:24-31` (`ui_wgpu::wgpu::{ActionDescriptor, NamedLayout, SurfaceKind, WindowLayout, WindowOptions}`).
- Naming: `C:50, 167, 444, 465, 678` (and more) are comments that name `✏️s/🔌️plugins/**` and `🛍️products/💻️os` as consumers. These are documentation only; the four files have no code edge to `✏️s`. The Cargo-level edges are in the fleet's dependency audit, not here.

**W4. Hub bin (D) carries the GIS product domain, storage drivers and OS-kernel internals.**
- GIS domain in the bin: `D:4337-4502` `GisMapApprovalPublicationControlV1`, `GisMapApprovalVerifiedPublisherV1`, `GisMapApprovalCheckpointPublisherV1Impl`; `D:10994-11047` GIS ingress authority; `D:11061-11140` `post_inference_gis_map_job*` routes; `D:93` `semio_hub::inference::{verified_gis_map_binding, ...}`. The word "gis" appears 66 times in the file.
- Storage backends in the bin: `D:39-41` import `Neo4jArtifactChunkCasStorage`, `PostgresArtifactChunkCasStorage`, `SqliteArtifactChunkCasStorage`. Hub `Cargo.toml:79-81` links `rusqlite`, `sqlx-core`, `sqlx-postgres` into the bin.
- OS-kernel internals: `D:14-16` `directory::os_directory::io::text::...`, `D:28-31` `directory::os_directory::schema::...` and `directory::os_spr::channel::...`, `D:27` `db::db_storage::PayloadStorage`, `D:35` `protocol::{...}`. Hub `Cargo.toml:67` (`directory` = os-kernel) and `:70` (`db` = os-kernel-db).
- Policy logic in the bin: `D:10150-10306` `execute_admin_intent`, `D:10323` `admin_intents`, `D:3541` `issue_document_open_plan_inner`.

**W5. Composition by text.** `A:1201-1202`, `A:10927`, and `D:3956` pull code into a module with `include!`, and `B:596` does the same for a test. These hide the real dependency graph from the compiler and from the grep-based gates. Convert to `mod` only with explicit re-exports (section 9).

## 8. Ordered extraction sequence

Common steps for each file (run them in order; one file per change):
1. Check ownership: `git status` (read-only) and the coordinator's file-ownership table. Other agents edit these files (the fleet says so).
2. Create the child folders and copy the ranges with `sed -n 'A,Bp'` (no retyping). Each child gets its `//!` header with the emoji, `use super::*;`, and the impl header copied from the band start where it applies.
3. In the parent, replace the moved ranges with `#[path = "<folder>/🦀️.rs"] mod <module>;` in the existing order. Add the explicit `pub use` / `pub(crate) use` lines from sections 3.3, 4.2 and 5.2.
4. Fix relative `#[path]`, `include!`, `include_str!` (add `../` where the target is not moved).
5. Promote the listed items to `pub(super)` (Appendix B).
6. Update the source-text tests in the same change (section 3.5 item 1, section 6.3 item 1).
7. Verify with the repo's compiler gate. Per the repo memory, raw cargo is refused; use `bun repo cargo --store=shared ... -- <cmd>`. Warnings count as proof here, not only errors. Run the tests named in the hazards lists. Do not claim a test passes until it ran.

Per file:
1. C (manifest) first. Pure types, no cfg, no `include!`, one re-export list. Smallest blast radius. Steps: create the six folders; move ranges 116-2095, 2096-3358, 3789-5059, 5060-6163, 6164-6914, 6915-8112; keep the parent lines listed in 5.2; fix `include_str!` at 1237 and 1349 and the `#[path]` lines at 6155, 7348, 8050; generate `pub use` lists from `pub-toplevel.json`; no visibility promotions are needed except the 34 names in Appendix B.
2. B (host). Create `guest-runtime`, `post-turn-relay`, `routers`. Fix the `#[path]` depth (4.3 item 1) and the `include_str!` at 5164. Keep `actor_bindings` and its `bindgen!` whole. Promote 15 names (Appendix B). Re-export the public items from the parent.
3. A (instance). Create `ingress`, `dispatch`, `typed_commands`, `plugin_app_impl`. Re-wrap the impl header in each band. Add the parent `pub(crate) use` lines for `MAINTENANCE_*`, `settle_framework_reserved_*`, `drive_framework_reserved_spawn_like_host` (cfg test). Promote 84 names (Appendix B). Update the two source-text tests (3.5 item 1) in the same change. The `include!` lines stay.
4. D (hub). Do section 6.4 first: move the route families into the hub lib. Then split the lib modules as in 6.2 and update `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` assertions (lines 223, 1685, 2117, 4652).

Parallelism: A, B and C touch disjoint trees and can run in parallel with separate owners. D needs the lib design decision first. Keep one file per change so the cargo gate stays attributable.

## 9. Decisions for the coordinator

1. D: move route families into the hub lib before splitting (recommended), or split the bin in place (not recommended; 285 analyzer candidates would become `pub(super)` on a bin root).
2. A: keep the two `include!` lines (1201-1202, 10927) in the parent (recommended, no path changes), or convert them to `mod` with explicit re-exports (cleaner, touches `🪟️window/` and `🎮️command/` consumers).
3. C: keep the `kernel` `#[path]` (7348) and the test `#[path]` mods in the parent (recommended), so public module paths do not change.
4. Dependency direction (section 7) is out of scope for the mechanical split. The split itself keeps every dependency where it is. If the coordinator wants the W1-W4 edges removed, that is a separate ticket per edge.

## Appendix A: instance member index by band (generated)

Lines are `line name`; `*` marks non-private members. Source: `🗑️generated/seam-plugin-manifest-hub/appx-A-members.txt`.

```text
### X1-ingress (1204-3543): 122 member lines
  1206 test_registered_tool_keys* | 1211 test_framework_tool_registrations* | 1218 tool_cancellation_handle* | 1223 admit_reserved_emit_producer | 1231 release_reserved_emit_slot | 1237 finish_reserved_emit_producer
  1244 test_reserved_emit_admission_case* | 1271 test_reserved_emit_cancelled_worker_handoff* | 1284 test_reserved_emit_busy_completion_owner* | 1308 run_framework_reserved_job | 1320 run_framework_reserved_job_inner | 1325 drive_framework_reserved_worker
  1496 validate_framework_reserved_commit | 1512 qualified_tool_proof | 1528 qualified_host_configuration_tool_proof | 1549 require_ui_safe_declaration | 1561 admit_command_wire_with_proof | 1578 admit_command_wire*
  1584 admit_command_json_with_proof | 1602 admit_host_configuration_json | 1610 new* | 1619 isolate_artifact_app_initial_snapshot | 1623 with_registry* | 1628 with_registry_on_bus*
  1963 has_constructor_receipt | 1964 constructor_receipt | 1965 take_constructor_receipt | 1966 store_constructor_is_pending | 1967 drive_store_constructor_unit* | 1968 drive_store_constructor_unit_in_phase
  1993 constructor_terminal_is_empty* | 1994 constructor_close_demands* | 2003 close_store_constructor_unit* | 2029 seed_genesis_children | 2063 follow_derivable_children* | 2122 retire_followed_child
  2141 artifact_generation_now* | 2146 window_transient_snapshot* | 2150 window_transient_generation* | 2154 window_config_generation* | 2164 advance_local_interaction_query_one | 2179 snapshot_read_returns_demands
  2184 advance_snapshot_read_returns_one | 2203 reclaim_document_snapshot_read_returns | 2208 snapshot_read_returns_terminal_is_empty | 2213 next_snapshot_read_return_stage | 2225 next_maintenance_stage* | 2231 begin_artifact_envelope_ingress*
  2247 begin_persisted_document_store_replacement | 2268 preflight_artifact_envelope_ingress_page* | 2285 construct_and_admit_artifact_envelope_ingress_page* | 2299 admit_artifact_envelope_ingress_page* | 2317 seal_artifact_envelope_ingress* | 2356 advance_artifact_envelope_load*
  2386 cancel_artifact_envelope_load* | 2402 submit_artifact_envelope_decode* | 2421 admit_artifact_envelope_decode_owner* | 2442 poll_artifact_envelope_decode* | 2447 artifact_envelope_decode_worker_owners_are_terminal* | 2452 advance_ready_envelope_decode_worker_owners*
  2461 try_publish_artifact_envelope_decode* | 2491 cancel_artifact_envelope_decode* | 2503 try_begin_artifact_store_replacement* | 2542 try_begin_owned_document_members* | 2553 admit_owned_document_member* | 2561 seal_owned_document_members*
  2570 poll_artifact_store_replacement* | 2592 artifact_store_replacement_refusal* | 2596 acknowledge_artifact_store_replacement* | 2609 cancel_artifact_store_replacement* | 2630 complete_document_archive_genesis | 2679 complete_store_replacement_genesis
  2729 merge_document_archive | 2748 merge_decoded_document_archive | 2773 derivable_follow_awaits_retirement | 2779 child_retirements_admit | 2785 reclaim_child_retirements | 2794 admit_child_content_publication*
  2802 admit_child_content_publication_span | 2812 publish_child_content_member* | 2817 publish_member_content | 2834 send_member_lane | 2844 peer_roster_slot | 2855 orphaned_peer_roster_generation
  2866 validate_peer_roster_publication | 2886 publish_peer_roster_candidate_admitted | 2908 declared_child_reference | 2917 admit_child_member | 2933 prepare_child_member | 2954 commit_child_member
  2969 test_child_admission_state | 3003 record_typed_operation_lane | 3034 deliver_host_event | 3053 deliver_utility_host_events | 3081 deliver_host_event_to_every_window | 3092 deliver_base_moved
  3100 dispatch_host_event_action | 3120 record_settled_typed_operation_command | 3148 typed_operation_command_kind | 3163 typed_operation_retirement_demands* | 3175 retire_typed_operation_unit* | 3180 retire_typed_operation_granted
  3222 retire_typed_operation_run | 3260 child_root_retirement_quote | 3265 child_root_retirement_demands | 3270 child_root_retirement_step | 3275 child_root_retirement_unit | 3282 child_member_retirement_demands
  3287 child_member_retirement_step | 3291 child_member_retirement_quote | 3297 child_member_retirement_unit | 3313 validate_parent_child_restore | 3328 open_child* | 3369 child_store*
  3379 announce_member_event_logs | 3395 announce_member_tail_edits | 3416 fold_member_inbound | 3465 register_child* | 3491 checkpoint_child_unit | 3508 stamp_checkpoint_composition_pins
  3522 checkout_cascade_pins | 3531 checkout_child_unit

### X2-dispatch (3544-6305): 96 member lines
  3545 test_snapshot* | 3552 test_store* | 3558 test_store_mut* | 3564 test_segmented_closure_contains* | 3570 test_config* | 3577 test_app*
  3584 test_history* | 3594 test_dispatch_emit* | 3605 test_result_from_last_edit* | 3613 test_edit_tail_lengths* | 3620 test_last_edit_id* | 3627 test_interaction_selection_snapshot*
  3634 snapshot* | 3640 test_document_revision* | 3645 backbone_ref* | 3652 push_log_entry* | 3691 record_command* | 3707 verb_label*
  3728 backfill_command_log | 3824 child_history_tails | 3847 child_group_history_target | 3929 build_history_view* | 3963 patch_history_view | 4011 history_view_patch
  4051 history_row | 4149 history_row_position | 4170 history_view_stacks | 4178 history_alternatives | 4199 refresh_history_view | 4235 history_patch*
  4299 refresh_cache* | 4324 command_cache_inputs* | 4332 history_command_authors | 4337 history_command | 4350 reset_dispatch_report | 4354 record_dispatch_receipt
  4358 record_rejected_dispatch | 4363 apply_merge_policy | 4368 resolve_store_conflict | 4375 current_open_conflicts | 4385 preview* | 4396 empty_result
  4431 result_from_last_edit | 4508 transaction_fault | 4519 mint_extension_invocations | 4543 dispatch_emit* | 4632 dispatch_emit_inner | 4812 close_streamed_transaction_unit
  4838 open_transaction_admission_fault | 4853 absorb_created_children* | 4894 require_operation_emitting_kind | 4904 declared_dispatch_kind | 4916 commit_transaction_group | 4933 transaction_group_history
  5021 dispatch_emit_group | 5207 sync_composition_graph* | 5246 dispatch_group_history_action | 5300 interaction_selection_witness | 5309 interaction_state* | 5322 interaction_selection_snapshot
  5381 resolve_domain_topology | 5392 app_interaction_topology | 5422 build_full_interaction_topology | 5451 revalidate_and_persist_interaction_state | 5493 revalidate_interaction_on_document_change | 5509 apply_interaction_writes
  5555 dispatch_interaction_action | 5576 apply_interaction_verb | 5746 fold_inline_interaction_verbs | 5771 overlay_leftover_ids_into_vortex | 5786 leftover_interaction_view_from | 5833 stamp_and_cache_interaction_ui*
  5837 stamp_pending_render_interaction_ui | 5842 build_artifact_reserved_action_job | 5871 build_artifact_reserved_media_job | 5901 receive_owned_media_poll | 5910 render_failure_terminal_is_empty* | 5911 render_failure_demands*
  5919 close_render_failure* | 5935 close_pending_render_tree_on_original | 5969 retain_pending_render_tree | 5975 retain_render_failure | 5980 retain_render_closure_failure | 5985 prepare_render_closure_failure
  6002 receive_render_failure | 6017 media_submission_refusal_terminal_is_empty* | 6018 media_submission_refusal_demands* | 6033 close_media_submission_refusal* | 6062 retain_media_submission_refusal | 6067 receive_media_submission_refusal
  6074 receive_owned_media_export | 6083 submit_owned_media_export | 6150 quarantine_media_snapshot | 6154 finish_media_poll | 6165 drive_owned_media_export | 6275 cancel_owned_media_export

### X3-typed-commands (6306-9148): 81 member lines
  6309 framework_reserved_work_items | 6314 framework_reserved_route_items | 6349 retire_pending_reserved_latest_wins | 6373 dispatch_framework_reserved_action | 6413 admit_framework_reserved_spawn | 6430 admit_framework_reserved_commit
  6448 framework_reserved_commit_progress* | 6455 step_framework_reserved_commit | 6482 take_framework_reserved_commit_outcome | 6486 run_framework_reserved_commit_unit | 6503 run_framework_reserved_commit_unit_admitted | 6587 dispatch_chrome_history_action
  6629 commit_framework_history_route | 6674 begin_framework_revert_route | 6696 framework_revert_unit | 6709 commit_framework_clipboard_completion | 6718 dispatch_reserved_original_producer | 6735 finish_mounted_reserved_completion
  6750 commit_framework_shared_host_route | 6791 dispatch_catalogue_example | 6802 document_loading_refusal* | 6810 declared_action_kind | 6814 dispatch_action* | 6824 dispatch_action_received
  6918 dispatch_command | 6983 fill_agent_revisions | 7048 preview_addressed_action* | 7154 authoring_seed | 7169 admit_agent_lane_preview_key | 7188 begin_agent_lane_preview
  7258 advance_agent_lane_preview | 7289 close_agent_lane_preview* | 7299 dispatch_command_frame | 7323 drive_typed_operation_worker | 7355 has_runnable_artifact_envelope_decode | 7359 has_runnable_artifact_envelope_decode_worker_step
  7369 next_advanceable_typed_operation | 7390 typed_operation_stall_witness | 7421 typed_operation_stall_streak | 7433 advance_typed_operation_publication_one* | 7469 fault_typed_operation_worker | 7497 fault_stalled_typed_operation_publication
  7526 advance_typed_operation_publication_unit | 7606 publish_mounted_typed_operation_run* | 7630 publish_mounted_typed_operation_run_admitted | 7642 publish_mounted_typed_operation_pump | 7705 mounted_typed_interaction_writes_are_next | 7728 publish_mounted_typed_interaction_unit
  7758 mounted_typed_inline_interaction_verbs_are_next | 7793 publish_mounted_typed_inline_interaction_unit | 7818 advance_latest_wins_command_one* | 7935 advance_latest_wins_keys | 7950 advance_latest_wins_admission_unit | 7971 publish_mounted_typed_child_operation_unit
  8158 handoff_mounted_durable_publication | 8190 advance_mounted_window_config_refresh | 8208 publish_mounted_typed_operation_unit* | 8605 require_tool_operation_authority | 8621 require_complete_tool_operation_pipeline | 8631 can_admit_typed_operation*
  8637 typed_operation_slot_is_vacant* | 8649 live_typed_operation_slots | 8659 admit_typed_operation_slot* | 8667 dispatch_typed_command_inner* | 8736 capture_typed_command_roots | 8778 test_capture_presence_roots*
  8785 test_mount_typed_command* | 8793 start_typed_command_operation | 9013 dispatch_typed* | 9029 dispatch_import_media | 9049 dispatch_config_command_inner | 9070 finish_configuration_command
  9085 receive_configuration_failure | 9086 configuration_failure_terminal_is_empty* | 9087 configuration_failure_demands* | 9088 close_configuration_failure* | 9099 verb_dirties_history_panel | 9107 finish_recorded
  9119 retained_fields_terminal_is_empty | 9130 live_render_operation | 9143 typed_operation_result_state_for_test*

### X4-plugin-app (9166-10904): 125 member lines
  9254 child_retirements_under_pressure | 9271 admit_cancellation_issuer | 9272 mounted_owner_maintenance_phase | 9277 mounted_owner_maintenance_step | 9283 mounted_owner_close_step | 9289 close_retirement_demands
  9295 maintenance_retirement_demands | 9299 bind_instance_id | 9303 bind_actor | 9312 close_step | 9321 close_terminal_is_empty | 9385 maintenance_step
  9389 maintenance_terminal_is_empty | 9393 maintenance_under_pressure | 9397 app_id | 9401 artifact_schema | 9405 tool_owner_witness | 9409 tool_public_contracts
  9432 advance_typed_operation_publication | 9469 take_typed_operation_preparation_receipt | 9482 take_typed_operation_retirement_receipt | 9486 take_history_command_receipt | 9490 take_tool_run_receipt | 9493 take_local_interaction_query_receipt
  9495 store_constructor_is_pending | 9497 take_constructor_receipt | 9499 tool_run_presence | 9503 tool_run_trace_delta | 9507 typed_operation_refusal | 9514 close_diagnostic
  9518 typed_operation_diagnostic | 9525 has_pending_typed_operations | 9541 live_typed_operation_slots | 9545 has_runnable_typed_operations | 9562 begin_local_interaction_query | 9589 acknowledge_local_interaction_query
  9593 cancel_local_interaction_query | 9597 take_local_interaction_query_reply | 9606 publish_local_interaction_query_reply | 9633 take_typed_operation_result_page | 9642 acknowledge_typed_operation_result | 9672 take_typed_operation_effect
  9676 take_typed_operation_event | 9680 take_typed_operation_ui_scope | 9683 take_typed_operation_ui_progress | 9705 take_typed_operation_completion | 9714 take_typed_operation_composed_result | 9718 begin_dispatch_report
  9722 dispatch_report | 9726 set_merge_policy | 9730 resolve_conflict | 9734 open_conflicts | 9738 handle_action | 9746 admit_reserved_spawned_job
  9750 take_reserved_commit_outcome | 9754 reserved_commit_progress | 9758 handle_action_invocation | 9788 handle_response_action | 9805 handle_command | 9812 original_command_frame_pending
  9813 handle_command_frame | 9822 handle_intent_frame | 9837 resume_task_command | 9847 resume_task_emit | 9864 take_last_emit_wire | 9868 take_pending_presence
  9872 take_pending_transaction_proposal | 9886 transaction_prepare | 9959 transaction_commit | 9988 transaction_rollback | 9998 transaction_undo | 10010 transaction_redo
  10024 history_snapshot | 10034 hydrate_pure_head | 10061 ephemeral_snapshot | 10084 reserve_presence_ingress | 10101 admit_presence_ingress | 10110 push_presence_ingress
  10117 reject_presence_ingress | 10125 take_presence_outcome | 10132 config_pack | 10136 window_config_packs | 10140 load_window_config_pack | 10144 load_child_pack
  10153 child_packs | 10166 child_head_packs | 10179 begin_document_archive_load | 10222 begin_document_archive_merge | 10229 poll_document_archive_load | 10236 cancel_document_archive_load
  10248 acknowledge_document_archive_load | 10262 document_load_archive | 10269 document_identity | 10274 document_archive | 10285 load_config_pack | 10292 dispatch_config_command
  10298 ingest_operations | 10312 ingest_operations_text | 10325 document_text | 10329 document_pack | 10333 attach_backbone | 10340 attach_hot_backbone
  10346 tick_backbone | 10360 detach_backbone | 10374 render | 10471 original_render_tree_source | 10480 window_engagements | 10526 window_measures
  10566 tool_measures | 10593 app_catalogue_json | 10613 pending_effects | 10649 context_menu | 10676 produce_media | 10723 prepare_media_import
  10734 consume_media | 10772 export_media | 10790 submit_media_export | 10796 submit_media_export_checked | 10804 poll_media_export | 10815 cancel_media_export
  10819 retain_media_export_result | 10860 take_media_export_chunk | 10872 take_segmented_download_chunk | 10887 import_media | 10893 media_fingerprint
```

## Appendix B: private items that need `pub(super)` (generated)

Source: `🗑️generated/seam-plugin-manifest-hub/res-A.txt`, `res-B.txt`, `res-C.txt` (trait-impl items excluded). File D is summarized by count only; full list in `res-D.txt`.

#### File A
- `X1-ingress` (pairs=38 distinctNames=33): `admit_child_content_publication_span`, `admit_host_configuration_json`, `admit_reserved_emit_producer`, `advance_local_interaction_query_one`, `announce_member_event_logs`, `announce_member_tail_edits`, `checkout_cascade_pins`, `checkout_child_unit`, `checkpoint_child_unit`, `constructor_receipt`, `deliver_base_moved`, `deliver_utility_host_events`, `derivable_follow_awaits_retirement`, `dispatch_host_event_action`, `finish_reserved_emit_producer`, `fold_member_inbound`, `has_constructor_receipt`, `merge_document_archive`, `peer_roster_slot`, `publish_member_content`, `qualified_tool_proof`, `reclaim_child_retirements`, `reclaim_document_snapshot_read_returns`, `record_settled_typed_operation_command`, `record_typed_operation_lane`, `release_reserved_emit_slot`, `retire_typed_operation_run`, `run_framework_reserved_job`, `snapshot_read_returns_terminal_is_empty`, `stamp_checkpoint_composition_pins`, `store_constructor_is_pending`, `take_constructor_receipt`, `validate_framework_reserved_commit`
- `X2-dispatch` (pairs=36 distinctNames=34): `apply_interaction_writes`, `apply_merge_policy`, `build_artifact_reserved_action_job`, `build_artifact_reserved_media_job`, `cancel_owned_media_export`, `child_group_history_target`, `child_history_tails`, `close_streamed_transaction_unit`, `commit_transaction_group`, `current_open_conflicts`, `declared_dispatch_kind`, `dispatch_emit_group`, `dispatch_group_history_action`, `dispatch_interaction_action`, `empty_result`, `fold_inline_interaction_verbs`, `history_command`, `interaction_selection_snapshot`, `mint_extension_invocations`, `open_transaction_admission_fault`, `receive_media_submission_refusal`, `receive_owned_media_export`, `receive_owned_media_poll`, `receive_render_failure`, `require_operation_emitting_kind`, `reset_dispatch_report`, `resolve_store_conflict`, `retain_media_submission_refusal`, `retain_pending_render_tree`, `retain_render_failure`, `revalidate_interaction_on_document_change`, `stamp_pending_render_interaction_ui`, `transaction_fault`, `transaction_group_history`
- `X3-typed-commands` (pairs=16 distinctNames=15): `admit_framework_reserved_commit`, `authoring_seed`, `commit_framework_clipboard_completion`, `dispatch_command`, `dispatch_command_frame`, `dispatch_config_command_inner`, `dispatch_import_media`, `finish_recorded`, `has_runnable_artifact_envelope_decode_worker_step`, `live_render_operation`, `live_typed_operation_slots`, `require_complete_tool_operation_pipeline`, `retained_fields_terminal_is_empty`, `step_framework_reserved_commit`, `take_framework_reserved_commit_outcome`
- `X4-plugin-app` (pairs=3 distinctNames=2): `PUBLICATION_RETURNED_ROOT_ALLOWANCE`, `history_changed_event`

#### File B
- `B1-guest-runtime` (pairs=9 distinctNames=9): `ActorHostState`, `cancel_admissions`, `cancel_job`, `compile`, `release`, `start_job`, `step_job`, `take_relay_start_failure_release`, `take_relay_step_failure_release`
- `B2-post-turn-relay` (pairs=8 distinctNames=5): `bytes`, `close_step`, `guest`, `invalid`, `schedule`

#### File C
- `C1-arg-schema` (pairs=21 distinctNames=12): `fields`, `input`, `inputs`, `members`, `parse`, `plain_string`, `reference`, `resolve`, `schema`, `target`, `variants`, `with_schema`

#### File D (counts only; full list in `res-D.txt`)
- `D1-state`: pairs=226 distinctNames=112
- `D2-auth-rest`: pairs=103 distinctNames=67
- `D3-websocket`: pairs=8 distinctNames=6
- `D4-directory`: pairs=78 distinctNames=50
- `D5-session-agent`: pairs=22 distinctNames=15
- `D6-admin`: pairs=44 distinctNames=34
- `D7-inference`: pairs=1 distinctNames=1

## Appendix C: generated files

- `🗑️generated/seam-plugin-manifest-hub/seam.py`: analyzer (ranges in, coupling and visibility out).
- `hazards.py`: lists `#[path]`, `include!`, `include_str!`, `concat!` with their group after the split.
- `spec-A.json` ... `spec-D.json`: the partitions used above.
- `A-top.txt`, `B-top.txt`, `C-top.txt`, `D-top.txt`: the raw top-level greps.
- `pub-toplevel.json`: top-level `pub` items per child (re-export input).
