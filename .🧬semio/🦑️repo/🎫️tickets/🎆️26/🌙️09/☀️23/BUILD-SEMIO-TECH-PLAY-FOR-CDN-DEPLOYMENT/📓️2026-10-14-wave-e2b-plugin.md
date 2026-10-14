# 🔌️ Wave E2b: plugin crate back-edges, visibility and the cut to the source admission ceiling (2026-10-13/14)

Executor: E2b. Scope: `semio-framework-plugin` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`, without `🖨️describe`, `📇️registry`, `🏗️build` and the separate crates `🖥️host`, `🏇️mounted-owner`, `🌐️browser-bundle`).
Brief: `📓️2026-10-13-scalable-build-architecture.md` (L3, L8), `📓️2026-10-13-wave-e2-plugin.md` (R1–R6, C1–C5), and after the 2026-10-14 restart the Codex fleet's ceiling of 4,000 active lines / 262,144 bytes / 32 files per unit (`🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/crate-production-boundaries.md`, `architecture-audit-current.md`).

Rules kept: no git-modifying command, no worktree, no sub-agent, no ticket operation, `🗑️generated` untouched. Every cargo invocation ran through `🚦️cargo-slot.sh` with the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/e2b/target` (shared build-dir per `.cargo/config.toml`). Only the per-test sweep (the test binary, not cargo) ran in the background.

## 1. Outcome

PLACEHOLDER-OUTCOME

## 2. How the order was reached (C1–C3)

The decisive design rule, applied everywhere: **what an author can hold or read is contract; what schedules, admits, publishes or commits is machinery.** Handles and read models (completion cells, output chunks, the owned context, the gesture slot, the cancellation lease, the child-content and history views, the instance-operation-owner handle, the `Host` handle and its request registry) moved down into the declaration facet. The schedulers that drive them stayed in publication and jobs. Every item kept its name, its generics and its byte layout, so no retirement demand, `size_of` law or consumer path changed.

### 2.1 Ownership and window (R3)

| Change | Why |
|---|---|
| WIT bindings `wasip2` moved from an inline root module to `🧬️schema/🦀️.rs`; `🧬️schema/` is the ownership facet (with `sqlite_wire`) | jobs (`operation_authority`) needs the ABI, so it cannot live in reactor |
| `store_lifecycle` (`♻️lifecycle`), `transient_publication`, `reserved_job_session` → ownership | they depend only on the kernel store and bounded ownership; window owners need the disposer contract |
| `ArtifactToolEmission` → `tool_jobs`, `ArtifactMutationOutcome` → `app_emission` | the only two declaration references inside `store_lifecycle` |
| `AppActionRegistry` + `validate_ui_dispatch_classification` → `window_kits`; `tool_job_registration` / `validate_tool_job_rows` became free functions in `reserved_jobs` | the registry is UI action data; tool-proof validation is jobs logic over it |
| `TransientView` → `window_transient` | it is the transient lane view and carries the window transient snapshot |

### 2.2 The app contract (R1, R2, R5)

New declaration modules, each a taxonomy node under `📜️declaration/`:

| Module | Content (moved, unchanged) |
|---|---|
| `tool_contract` (`📜️declaration/👷️job`) | bounded first-step proofs and `bounded_first_step_tool_proofs!`, factory trait, registry, registrations, public contracts, publication lanes, instance operation owner trait and handle (+ alias retirement, file moved from `🧵️instance/♻️alias`), owned/reserved job requests and inputs, raw input lease, completion and completion value/rejections, bounded tool fault, `ArtifactReservedToolJob`, reserved job outcomes, `reducer_fault_of_detail` |
| `tool_run_contract` (`📜️declaration/⏯️tool-run`) | `ToolRunJob(Request)`, retargetable job, port and its retirement, trace keys, run view, `FRAMEWORK_TOOL_RUN_BODY_KEY` |
| `gesture_contract` (`📜️declaration/🛠️tool-machine`) | `GestureSlot`, its state and decision, `GestureCapture` |
| `media_contract` (`📜️declaration/🎞️media`) | output chunks, export result/completion/credit/request/handle/poll, `validate_media_export_structure` and its test |
| `child_view` (`📜️declaration/🪆️child`) | `ChildContentView` and its pages, entries, owners and read retirement |
| `history_view` (`📜️declaration/🎮️command`) | `HistoryView`, command/mutation/alternative views, filter, `CommandLogEntry`, `ShellHistoryReplay` (also closes R6) |
| `natural_file_import` (`📜️declaration/📥️natural-file`) | the framework-owned natural-file importer the `ArtifactEditor` default builds |
| `owned_context` (hoisted `🧵️retained-command/🧬️context`, files kept for its TypeScript oracles) | `ArtifactOwnedContextHandle` and its retirement fields |

Re-faceted whole modules (deps already low): `completion_fault`, `completion_retirement`, `completion_refusal_retirement`, `child_emit_preparation`, `presence_retirement`, `tool_cancellation`, `host` (with the request registry moved from `⚛️reactor/📮️requests` to `🌐host/📮️requests`), and on 2026-10-14 the peer-added `media_operation_refusal`. Small moves: `ArtifactOperationProgress`, `CANCEL_TYPED_OPERATION_ACTION_ID` and `cancellation_action_definition` → `app_views`; `PeerMark`/`PeerPresenceRoot` (+4) → `app_views`; `is_time_travel_action_id` → `app_contract`.

Moved up:

| Change | Why |
|---|---|
| `PluginApp` + `NoPluginApp` → new reactor module `plugin_app` (`⚛️reactor/🔌️app`) | the runtime-facing trait names store and job outputs |
| inline `surface_contract::declarations` → top-level `declarations` (instance, `📜️declaration/🌳️tree`), still `app::declarations` for consumers; `EditorSurfaceApp`/`ViewerSurfaceApp` moved with it; `AppFactory` → `plugin_program`; subset example table → `surface_contract` | the tree builds `VcsArtifactApp` factories |
| `ArtifactDeclaration::install_into(Plugin)` deleted; the builder installs capabilities inline | a declaration method may not name the reactor |
| cold inference wire entry points (`cancel_artifact_inference`, `wire_artifact_infer_cold`, …) → `inference_gateway`; `ArtifactInferenceGatewayFailure` → `artifact_inference`; dead `wire_artifact_infer_with_context_cold` deleted | they read the runtime's extension bundle |
| framework-reserved job kind (`FrameworkReservedBoundedJob`, its factory/demands, the registry driver) → `reactor::jobs::reserved` (`⚛️reactor/💼️jobs/🏛️reserved`) | it implements the reactor job table's `BoundedJob`; `reactor::jobs` stays the public path consumers use |
| `runtime_lifecycle_step` → `bounded_ownership` | a generic forward-once grant helper |

2026-10-14 drift fixes (peers added three edges): `media_operation_refusal` → declaration, the cancellation lease file included by `tool_cancellation` instead of `typed_operation` (its `MountedTypedCommandFullOperation` impl went back to `typed_operation`), and the peer module `action_ingress` is instance.

### 2.3 Impl placement for the cut

`e2b-plugin/🧷️impl-placement.py` checks that every inherent impl sits in its type's facet and every trait impl in its trait's or type's facet. Fixed: the `InteractionConfigMutation` impls moved from `app_views` into `🕹️interaction/🧬️mutations`; the two inherent impls on `ArtifactFixedRegistry<…>` in `child_content` became free functions over the new generic `ArtifactFixedRegistry::partition_mut` (the unused `sibling_content_owners` was deleted). **One remains:** `impl Plugin { fn builder() -> PluginBuilder }` (instance impl on a reactor type). It is public API with 160 consumer files (`Plugin::builder(`); the cut must rewrite them to `PluginBuilder::new(` (C5 consumer codemod), no in-crate design avoids it.

## 3. Dependency map after C1–C3

Tool: `e2b-plugin/🧭️facet-imports.py --back` (E2's tool, re-run with E1's generic-brace-fixed `e1-os-kernel/🧱️rust_items.py`). Re-run on 2026-10-14 against the moved tree:

| From \ to | ownership | window | declaration | publication | jobs | reactor | instance |
|---|---:|---:|---:|---:|---:|---:|---:|
| window | 10 | – | 0 | 0 | 0 | 0 | 0 |
| declaration | 56 | 30 | – | 0 | 0 | 0 | 0 |
| publication | 31 | 2 | 42 | – | 0 | 0 | 0 |
| jobs | 46 | 28 | 131 | 12 | – | 0 | 0 |
| reactor | 43 | 9 | 118 | 17 | 38 | – | 0 |
| instance | 32 | 44 | 155 | 58 | 109 | 22 | – |

- **Edges against the order: 0** (E2 left 155 names on 54 lines; the corrected tool counted 43 lines at the start of E2b). **Facet cycles: none.**
- Module cycles remain only inside one facet: declaration 14 modules (the `ArtifactApp` contract cycle, §5.2), reactor 6, window 2 (`table_kit`↔`tree_kit`), jobs 2+2, instance 2 (`action_ingress`↔`artifact_instance`). They are legal inside one crate and are the cut obligations of §5.
- Tool corrections found on the way (E2's map over-counted): `#[cfg(test)]` stacked above `#[path]` was read as production (tests counted as edges), `include!` inside a test module inherited production, `mod` heads inside `macro_rules!` bodies were modules, method names shadowed extern aliases (`crate::store`), and `#[cfg(test)] async fn` was not stripped. Production `use crate::app::*` (in `inference_gateway`) hid edges and is replaced by exact imports.

## 4. Visibility (C4)

`e2b-plugin/🔒️narrow.py --strip` removed all 2,985 `pub(crate)` outside the component root and `macro_rules!` bodies; `🔁️fix-loop.py` re-widened what rustc named (two passes: 723 items + 333 fields, then 128 + 127; 703 errors left). The 2026-10-14 restart killed the loop, and the moved tree does not build upstream of the plugin (`ui-theme`, `ui-layout`, `os-store-message-ledger`, `job`, `os-pack`, `os-command`, `os-channel`, `ui-draw-contract`, `ui-runtime`, and a "member of the wrong workspace" manifest error), so the loop cannot run. To leave the crate compile-safe, `🔓️restore-visibility.py b4bf1a54b37` put back the 238 stripped `pub(crate)` the loop had not yet re-widened (read-only `git show` of the last pre-strip auto-commit; only item heads and struct fields outside trait impls and macro bodies).

Result: 3,100 `pub(crate)` (3,062 before, the rest are peer additions). **The minimum is structural, exactly as E1b measured for the kernel (820 of 821 given back):** the re-widening reached ~2,860 before the remaining errors, so a mechanical shrink buys under 7 %. The exposure belongs to the god object's fields and methods that every facet touches; it only shrinks by the decomposition of §5.3 (facet-owned sub-structs with constructors and accessors), which is also what the source ceiling demands.

## 5. The cut to the source admission ceiling

### 5.1 Measured sizes (`e2b-plugin/📏️unit-size.py`, authored estimate: non-blank lines with comments and `#[cfg(test)]` items blanked, full bytes of every mounted file)

Production total ≈ 68 k active lines in 189 files. Over the ceiling (4,000 lines / 262,144 bytes / 32 files):

| Module | Files | Lines | Bytes | Why it cannot be one unit |
|---|---:|---:|---:|---|
| `artifact_instance` (`🧵️instance/🦀️.rs` alone 8,622 lines / 819,648 bytes) | 13 | 13,774 | 1,270,683 | `VcsArtifactApp` god object: one inherent impl of ~7.9 k lines |
| `reactor` | 26 | 7,885 | 749,142 | `turn` 1,639, `🦀️.rs` 1,345, `patches` 1,077 …, cycle with `plugin_runtime` |
| `plugin_runtime` (`🏃️runtime/🦀️.rs` 4,805 lines / 391,248 bytes) | 3 | 5,530 | 458,987 | one file over both ceilings |
| `tool_jobs` | 11 | 5,390 | 445,754 | `time_travel` 2,779, `tool_run` 1,380, cycle with `command_history` |
| `child_content` | 15 | 3,433 | 342,220 | bytes only |

### 5.2 Owner crates that fit today (`e2b-plugin/🧩️unit-plan.py`)

Strongly connected modules are condensed, then each facet is cut along one topological order, so every crate depends only on earlier crates and lower facets:

| # | Crate (proposed name, ≤ 48 chars) | Facet | Files | Lines | Bytes | Modules |
|---|---|---|---:|---:|---:|---|
| 1 | `semio-framework-plugin-ownership` | ownership | 10 | 1,525 | 146,108 | `sdk_fault`, `bounded_ownership`, `reserved_job_session`, `store_lifecycle`, `transient_publication`, `sqlite_wire`, `wasip2` |
| 2 | `semio-framework-plugin-window-kits` | window | 17 | 3,369 | 257,741 | `window_surface`, `window_kits`, `text_kit`, `table_kit`, `tree_kit`, `interaction_topology`, `local_interaction` |
| 3 | `semio-framework-plugin-window-stores` | window | 8 | 3,131 | 255,552 | `window_mutation`, `window_config`, `window_transient` |
| 4 | `semio-framework-plugin-window-media` | window | 6 | 1,121 | 71,556 | `document_kit`, `engagement`, `image_kit`, `media_kit`, `world3d_host`, `mesh_kit` |
| 5 | `semio-framework-plugin-contract-views` | declaration | 12 | 2,839 | 252,467 | `child_view`, `history_view`, `media_operation_refusal`, `gesture_contract`, `tool_cancellation`, `artifact_io`, `host` |
| 6 | `semio-framework-plugin-identity` | declaration | 5 | 2,704 | 195,665 | `artifact_identity`, `artifact_inference`, `artifact_declaration`, `artifact_registration`, `completion_refusal_retirement` |
| 7 | `semio-framework-plugin-publication` | publication | 8 | 2,977 | 260,031 | `private_child_input`, `document_ingress`, `peer_roster`, `store_publication`, `document_backbone_binding`, `no_draft_retirement`, `no_config_retirement` (+ `transient_retirement`, 24 lines) |
| 8 | `semio-framework-plugin-retained-command` | jobs | 6 | 904 | 73,968 | `reserved_job_bytes`, `retained_command` |
| 9 | `semio-framework-plugin-typed-operation` | jobs | 7 | 2,151 | 245,205 | `reserved_jobs`, `typed_operation`, `command_admission`, `media_export` |
| 10 | `semio-framework-plugin-operation-authority` | jobs | 8 | 572 | 74,148 | `extension_invocation_failure`, `operation_authority`, `reserved_job_outcomes`, `completion_effects`, `completion_vacant` |
| 11 | `semio-framework-plugin-app-trait` | reactor | 1 | 285 | 47,485 | `plugin_app` |
| 12 | `semio-framework-plugin-builder` | instance | 2 | 895 | 81,992 | `declarations`, `builder` |

These cover ≈ 22.5 k lines. Byte headroom is thin on #2, #3, #5 and #7 (≥ 96 % of 262,144): their first growth must cut them again.

### 5.3 Decomposition obligations (≈ 45.6 k lines; none can be one crate)

1. **The app contract cycle (declaration, 14 modules, 8,461 lines).** `ArtifactApp` names its vocabulary (`tool_contract`, `tool_run_contract`, `media_contract`, `completion_retirement`, `owned_context`, `app_emission`, `app_views`), and the vocabulary is generic over `A: ArtifactApp`. Measured: the vocabulary reads only associated types (`A::Snapshot`, `Mutation`, `Config`, `Draft`, `Presence`, `Transient` and their mutations, `Command`) plus four hooks (`import_media`, `DOCUMENT_SCHEMA`, the two completion-retirement hooks). **Cut (L8, narrow traits):** an `ArtifactLanes` trait (associated types only) in a low crate, `ArtifactApp: ArtifactLanes`, the vocabulary generic over `ArtifactLanes`, and the four hook-using functions moved up beside `ArtifactApp`. Consumer cost: the 2 direct `impl ArtifactApp` outside the crate (space engine, playbook procedural) move their associated types into `impl ArtifactLanes`; the 152 editors and 153 viewers are untouched (they implement `ArtifactEditor`/`ArtifactViewer`, whose adapters are internal). Then: lanes+emission+views (~1.6 k), tool vocabulary (~1.9 k + run/media/completion ~0.7 k), `app_contract`+`app_builder` (~2.2 k), `surface_contract`+`natural_file_import` (~2.0 k).
2. **`tool_jobs` + `command_history` (jobs, 5,817 lines).** Split by child: `time_travel` (2,957 with its children) alone, `tool_run` (1,438), `tool_machine`+`operation_progress`+the parent (~1,000), `command_history` (427). Children read the parent through `use super::*`; each becomes an explicit-import module first (the same compiler-driven step as E2's decomposition), then a crate.
3. **`child_content` (publication, 342,220 bytes).** Bytes only. Split the mounted-group owner/receipt includes (`🧩️composition/📬️publication/🤝️group/**`, ~190 k bytes) from the member registry and content root (~150 k bytes).
4. **Reactor (6 modules, 14,159 lines).** First break the cycle `reactor` ↔ `plugin_runtime` ↔ `plugin_program` ↔ `describe` ↔ `inference_gateway` ↔ `owned_abi` (the runtime cell types used by the reactor turn go down into a runtime-state crate; `describe` and the WIT bridge go up). Then split `🏃️runtime/🦀️.rs` (4,805 lines) along its regions: install/manifest, instance cells and close pumps, wire entry points, extension bundle, media/document wire. `reactor` splits by submodule: `turn`+`receiving` (~2.0 k), `patches` (1.1 k), `executor`+`pending`+`cold_pair` (~1.5 k), `jobs` (0.9 k with builtins), root+context+checkpoint+lifetime+surfaces (~2.3 k).
5. **`VcsArtifactApp` (instance, 13,926 lines).** An inherent impl must live in its type's crate, so the god object must become a composition of facet-owned components, each a struct with its own impl in its own crate: store lanes (document/config/draft/presence/transient stores and their disposers), interaction and window owners, tool runtime (tool runs, gestures, typed operations, reserved jobs), time-travel host, media and document ingress, child group, close/maintenance/envelope ladders (already separate files: 719 + 1,144 + 232 lines). `VcsArtifactApp` keeps the `PluginApp` impl and delegates. `rg 'pub\(crate\)' 🧵️instance/🦀️.rs` and the field list are the worklist; this is the same work that shrinks C4 (§4).
6. **Public `Plugin::builder`** (§2.3): consumer codemod `Plugin::builder(` → `PluginBuilder::new(` (160 files).

### 5.4 Execution status

The in-crate preparation is complete and needs nothing from WS1: zero edges against the order, every root module declaration grouped in its facet region (`🗂️regions.py`), impl placement clean except `Plugin::builder`. No crate manifest was created. Reasons: (a) the plugin closure does not build at the moment (§4), so no extraction can be proven compile-clean; (b) a cut also needs, per crate, the Codex admission row and launch routes (`crate-production-boundaries.md` "Completion Evidence Required"), and the 172 lockfiles that name `semio-framework-plugin` follow the fleet's practice of syncing only the root and framework locks. Unit #1 is ready to extract first: its only obstacle is `wasip2`'s `default_bindings_module: "$crate::component::wasip2"` and `__semio_export_component_guest!`, which must name the new crate.

## 6. Verification

PLACEHOLDER-VERIFY

## 7. Inputs (kept in `e2b-plugin/`)

| File | Purpose |
|---|---|
| `🧭️facet-imports.py`, `🗺️facets.json`, `🗂️destinations.json` | E2's map with E1's splitter; fixes: stacked `#[cfg(test)] #[path]` mounts, `include!` inside test modules, `macro_rules!` bodies, `#[cfg(test)] async fn`, module-level owner index; explicit `modules` facet map |
| `🕸️item-graph.py` | item-level facet graph and move closures |
| `🚚️move.py`, `🪓️extract-inline.py` | item mover and inline-module extractor with crate-wide path rewriting |
| `🔁️fix-loop.py`, `🧹️imports.py`, `🔄️until-clean.sh`, `🩹️apply-suggestions.py` | compile-driven import/visibility fixing (crate owners first, crate conventions for externals, struct lookup by rustc module path, broken-dependency detection) |
| `🔒️narrow.py` | C4 strip/count/classify |
| `🗂️regions.py` | root module declarations regrouped one region per facet |
| `🧷️impl-placement.py`, `📏️unit-size.py`, `🧩️unit-plan.py` | impl placement, per-module size against the ceiling, owner-crate plan |
| `🏗️cargo.sh`, `🧪️each-test.sh` | slot-gated cargo, per-test runner |
