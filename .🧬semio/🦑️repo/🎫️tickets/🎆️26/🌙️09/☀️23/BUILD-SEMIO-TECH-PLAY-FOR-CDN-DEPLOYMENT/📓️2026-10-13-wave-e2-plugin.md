# 🔌️ Wave E2: `semio-framework-plugin` decoupling (2026-10-13)

Executor: Lane D / E2. Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` (crate `semio-framework-plugin`).
Brief: `📓️2026-10-13-scalable-build-architecture.md` (L3, L5, L6, L8; §10 E2) and the family method of `📓️2026-10-13-stdio-semio-split-plan.md`.

Rules kept:

- No git-modifying command, no worktree, no sub-agent, no ticket operation, `🗑️generated` untouched.
- Every cargo invocation ran in the foreground through `🚦️cargo-slot.sh`, with the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/e2/target`.
- Only the per-test sweeps (the test binary, not cargo) ran in the background.

## 1. Outcome

| Item | Result |
|---|---|
| Step 1: module dependency map, including cycles | Done (§2, §4). The tools are in `e2-plugin/`. |
| Step 1: break the 48,743-line crate root into owned facet modules | Done. The root is down to **1,228 lines**: wiring, the WIT bindings, the owned ABI and the public `app` vocabulary. Its 40,609-line inline `app` module now lives in **37 facet modules**, and `plugin_runtime`, `world3d_host` and `engagement` moved to their own files. |
| Step 1: one-directional facet graph | **Partially done.** `ownership`, `reactor` and `instance` have zero edges against the order. The remaining edges against the order are listed name by name in §4.3, with the change each one needs. |
| Step 2: the crate cut | **Not done, on purpose.** It is not clean in the current workspace layout (§5). A precise cut plan is in §6. |
| Behaviour | Unchanged. All five build configurations compile. Plugin `--lib`: 1,052 ok, 0 regressions against the session baseline (1,051), the 957 baseline and r15 (§7.1). |

## 2. Dependency map before the change (measured, no compile)

Tool: `e2-plugin/🕸️plugin-module-graph.py`, together with `🧱️rust_items.py` (item splitter) and `🗺️facets.json` (facet rules). It walks the mounted module tree (`#[path]`, inline modules, `include!`), splits large modules into their top-level items (an `impl` joins its type, and the `VcsArtifactApp` inherent impl is split per method), resolves references by name, and reports strongly connected components.

The crate before the change:

- **Root `🔌️plugin/🦀️.rs`:** 48,743 lines. The inline `pub mod app` alone was 40,609 lines: 1,282 top-level items, 326 `impl` blocks and 61 macro calls.
- **Production code:** 67,081 non-blank code lines. Tests: 46,055.
- **The `VcsArtifactApp` god object:** a 334-line struct with about 200 fields, a 7,877-line inherent impl with 274 methods, a 1,599-line `PluginApp` impl, and about 5.4 k more lines of `impl VcsArtifactApp` blocks spread over 12 facet files (tool-run, time-travel, tool-machine, the three ladders, frontier, mounted owner/command, and others).

Item-level strongly connected components (production):

| Graph | Largest component |
|---|---|
| All items | 279 nodes, 12,411 code lines |
| Without the `VcsArtifactApp` struct, its methods and the `PluginApp` trait | 151 nodes, 6,277 lines (runtime glue: `plugin_exchange`, `poll_kernel_turn`, `Host`, `AppBuilder`, `Emit`, …) |

Everything else (about 55 k of 67 k lines) was already acyclic at item level. The tangle was the god object and the runtime glue, not the whole crate.

**Re-validation after E1's splitter bug report.** E1 found that the first `🧱️rust_items.py` ended an item at a brace inside generic arguments (`PagedList<T, {usize::MAX}>`). I ported E1's fix and checked whether the bug could have affected this work:

- **Sources.** `rg` finds no item head with a brace inside generics, neither in the pre-change root at `c73888a85bb` nor in the current tree. The bug could not trigger on these sources.
- **Codemod.** The decomposition copies items as contiguous spans, so a wrong split could not have lost text either way. The compile confirms it.
- **After map.** Rerunning `🧭️facet-imports.py` with the fixed splitter gives the same matrix and the same 54-line edge list.
- **E1's path moves.** No plugin source names `os_store::pack_rt`. A final native check against the current os-kernel is green.

## 3. What step 1 did

### 3.1 Facet modules and layering

The facets, bottom to top. Directories are existing taxonomy nodes; the new ones reuse emojis already present in the tree.

| Facet | Modules | Prod code lines |
|---|---|---:|
| **ownership** | `bounded_ownership` (`🏛️ownership`), `sdk_fault` (`📤️fault`), `component_persistent_local` | 373 |
| **window & kits** | `window_surface` (`🪟️window`), `interaction_topology` (`🕹️interaction/🧭️topology`), `window_kits` and `tree_kit`, `text_kit`, `table_kit`, `image_kit`, `mesh_kit`, `document_kit`, `media_kit` (`🪟️window-kits/*`), `world3d_host` (`🪟️window-kits/🌍️world3d`), `engagement` (`🪟️window/🎛️engagement`), `window_config`, `window_transient`, `window_mutation`, `local_interaction` | 7,654 |
| **declaration & describe** | `artifact_identity` (`🪪️identity`), `artifact_io` (`🚪️io`), `artifact_inference` (`💡️inference`), `artifact_declaration` (`📜️declaration`), `artifact_registration` (`📜️declaration/📇️registration`), `app_builder` (`🏗️builder/🧩️app`), `app_views`, `app_emission`, `app_contract`, `surface_contract` (`📜️declaration/*`) | 9,477 |
| **publication & mounted owner** | `child_content` (`🪆️child`), `peer_roster` (`👥️peer`), `store_lifecycle` (`♻️lifecycle`), `store_publication` (`🧩️composition/📬️publication`), `document_ingress` (`🗃️document`), `child_emit_preparation`, `private_child_input`, `transient_publication`, `*_retirement`, `document_backbone_binding` | 8,961 |
| **jobs & session** | `tool_jobs` (`👷️job`, holding `tool_run`, `time_travel`, `tool_machine`, `operation_progress`), `typed_operation` (`🧵️retained-command/🔄️full-operation`), `reserved_jobs` (`👷️job/🏛️reserved`), `media_export` (`🎞️media`), `tool_cancellation` (`♻️cancellation`), `command_admission` and `command_history` (`🎮️command`), `retained_command`, `completion_*`, `reserved_job_*`, `extension_invocation_failure`, `operation_authority`, plus `reactor::jobs` | 14,245 |
| **reactor & turn** | `reactor` (turn, executor, patches, …), `plugin_runtime` (`⚛️reactor/🏃️runtime`), `host`, `plugin_program` (`🏗️builder/🔌️program`), `describe`, `inference_gateway`, `wasip2`, `owned_abi` | 14,123 |
| **instance** | `artifact_instance` (`🧵️instance`) with its turn modules `time_travel_turns`, `tool_run_turns`, `tool_machine_turns`, `operation_progress_turns`, `full_operation_turns`, `child_group_turns` (`🧵️instance/*`), the three ladders, and `builder` (`PluginBuilder`) | 14,261 |

- Every facet now meets L3's per-library target of 15 k own lines or fewer. The old crate was 67 k.
- The root `🦀️.rs` declares the facet modules in one `//#region <Facet>` per facet.
- `app` (41 `pub use super::<module>::*;` lines plus the foreign re-exports it had before) is the crate's public vocabulary for consumers. It existed before the change and is not a compatibility layer. §6 deletes it in the cut, because consumers will then name the owning crates.
- Inside the crate, no module goes through `app` any more (§3.3).

### 3.2 Moves that made the graph one-directional where they could

- **Instance code out of facet modules.** Every `impl … VcsArtifactApp` block found in a facet module moved into a child module of `artifact_instance`. The ladders and the store-stage close file are now declared by the instance.
  - Moved: 3,293 lines from `⏪️time-travel`, `⏯️tool-run`, `🛠️tool-machine`, `⏳️operation-progress`, `♻️frontier`, mounted `📦️owner` and mounted `📚️command`.
  - An inherent impl must live in the crate that owns its type, so this is a precondition of any cut.
  - Effect: jobs, publication and window code no longer names `VcsArtifactApp` at all. `rg VcsArtifactApp` outside the instance finds only tests, `PluginBuilder::document_app` and the `EditorSurfaceApp` / `ViewerSurfaceApp` aliases.
- **Items moved down to their owning facet:**
  - `plugin_grant_funds` (formerly `mounted_retirement_grant_funds` in a receipt file of `child_content`) → `bounded_ownership`.
  - `ArtifactDefinitionError` and `PluginAssemblyError` → `sdk_fault`.
  - `ActionFactory` → `window_kits`.
  - `INTERACTION_ACTION_IDS` → `interaction_topology`.
  - `HISTORY_ACTION_IDS`, `CLIPBOARD_ACTION_IDS` and `is_framework_reserved_action_id` → `reserved_jobs`.
  - `LAST_MAINTENANCE_STAGE` → `plugin_runtime`.
  - `child_frame_scene_view` (instance test seam) → `artifact_instance`.
- **Exact imports per module, compiler-driven.**
  - Each decomposed module starts from the old `app` preamble (read from the pre-change commit, so no import is ever guessed). The preamble is flattened to one leaf per line and then narrowed:
    - leaves unused by the native lib are re-gated `#[cfg(any(test, feature = "artifact-app-testing"))]`;
    - leaves unused in the test configuration are deleted;
    - the rest is regrouped statement by statement, keeping the original docs.
  - Gated leaves needed by wasm32 code were re-enabled by the build itself.
  - One silent re-resolution risk was found and closed: a guessed import picked an external item of the same name (`crate::ui_text` through the component glob, `__pack_json::Value`, `ArgFormat::ArtifactRef`). All guesses were removed, and imports are resolved through an owner index.
- **Visibility.** Items, fields and methods were widened to `pub(crate)` only where rustc named a cross-module access: E0603, E0616, E0624, E0451 and private-in-public errors. There are now 2,993 `pub(crate)`, up from 1,585. This is the god object's real coupling surface and is the input for §6, step C2.

### 3.3 The crate's own uses of `app` removed

`🔀️unfacade.py` rewrote 93 files from `crate::app::X`, `crate::component::app::X` and `super::app::X` to the owning module or to the foreign path that `app` re-exports.

- `use` trees were split by owner.
- Globs over `app` were kept only in test support (`app_testing`), and `plugin_app_close_prelude` was kept as public API.
- Without this, every module would have depended on all facets through `app`.

### 3.4 Source-scan oracles that read moved code

Seven Rust laws read the crate root text through `include_str!` or `CARGO_MANIFEST_DIR`, as did five TypeScript checks of `test-retained-raw-source` and the describe crate's `PLUGIN_SDK_SOURCE`. All of them now read the owning files:

- the instance;
- the typed operation;
- `⚛️reactor/🏃️runtime`;
- `👷️job`;
- `🧩️composition/📬️publication`;
- the instance turn modules.

## 4. Dependency map after step 1

Tool: `e2-plugin/🧭️facet-imports.py`.

- **How it builds the graph.** It reads the import paths of every production module, excluding `#[cfg(test)]` imports and test modules. Each `crate::…` path, and each name re-exported through `app`, is resolved to the module that owns the item. The result is the module graph and the facet graph.
- **Exactness.** After §3.2 every decomposed module names its crate dependencies explicitly, so these edges are compiler-verified, not heuristic.
- **Known false edges to ignore:**
  - `store` → `tool_jobs`, which is a name collision with the `store` extern alias;
  - `→ laws` edges, which come from test fixtures referenced in `#[cfg(test)]` bodies.

### 4.1 Facet matrix (distinct imported names; `!` marks an edge against the order ownership < window < declaration < publication < jobs < reactor < instance)

| From \ to | ownership | window | declaration | publication | jobs | reactor | instance |
|---|---:|---:|---:|---:|---:|---:|---:|
| ownership | – | 0 | 0 | 0 | 0 | 0 | 0 |
| window | 5 | – | !4 | !6 | !3 | 0 | 0 |
| declaration | 15 | 27 | – | !36 | !55 | !5 | !1 |
| publication | 24 | 2 | 23 | – | !14 | !1 | 0 |
| jobs | 26 | 36 | 40 | 29 | – | !20 | 0 |
| reactor | 7 | 10 | 114 | 14 | 45 | – | 0 |
| instance | 12 | 39 | 101 | 70 | 152 | 14 | – |

The modules still form one cycle of 49 modules, because of the edges below.

### 4.2 What the remaining edges against the order are

| # | Edge | Names | Nature | Change it needs |
|---|---|---|---|---|
| R1 | declaration → jobs | `app_contract` / `surface_contract` / `app_views` name `ArtifactOwnedToolJobRequest`, `ArtifactReservedToolJob(Request)`, `ArtifactReservedToolInput`, `ArtifactToolCompletionValue`, `ArtifactToolFactoryRegistry`, `ArtifactBoundedFirstStepProof`, `ArtifactInstanceOperationOwner(Handle)`, `ArtifactMediaExport{JobRequest,Handle,Poll,Result}`, `ToolRunJob(Request)`, `NaturalFileImportJob`, `derived_*_completion_*`, `HistoryView`, `ToolRunView`, `ArtifactOperationProgress` and others (55 in total) | **The app contract's vocabulary lives in the machinery facet.** `ArtifactApp` / `ArtifactEditor` / `ArtifactViewer` default methods return job requests and registries. | **C1** (§6): move the contract vocabulary (request, handle, completion and view types, plus the trait-default helpers) down into `declaration`. Estimated closure, heuristic and an upper bound: 196 items, about 4.6 k code lines. Alternatively, make the machinery generic over narrow traits (L8). |
| R2 | declaration → publication | the store-lane defaults and disposers that `ArtifactApp` defaults call (`bounded_*_store_disposer`, `ArtifactDisposal`, `ArtifactSnapshotDisposer`, `OwnerMutationRoster`, `ArtifactStoreConstructor{Kind,Phase}`, `PresenceRoster*`, `ChildContentView`, `PeerPresenceRoot`, `ChildEmitPreparation*`, `ComposedGestureResult`; 36 in total) | Same cause, on the store side. | **C1**: a store-lane contract module in `declaration`, holding the default owners, the disposer traits and the constructor phases. |
| R3 | window → declaration / publication / jobs | `window_config` / `window_transient` use `NoConfig`, `NoConfigMutation`, `TransientView`, `ArtifactOwnedDisposer`, `bounded_config_store_disposer`, `no_config_store_*` and `transient_store_disposer`; `window_kits` uses `AppActionRegistry`; two `test_window_*_context_identity` test helpers live in `tool_jobs` | Window owners are store owners. | **C1** puts the store-lane contract at or below window. Move `AppActionRegistry` to `window_kits` (its content is UI actions) and the two test helpers next to the window tests. |
| R4 | jobs → reactor | `reserved_jobs` → `reactor::jobs` (`JobBudget`, `JobStep`, `start_job`, `step_job`, …); `operation_authority` → `wasip2` (WIT types); `typed_operation` → `plugin_runtime::runtime_lifecycle_step`; `extension_invocation_failure` → `ArtifactInferenceGatewayFailure` | Module nesting, not a real dependency. The job table `⚛️reactor/💼️jobs` is jobs code mounted inside the reactor module, and the WIT bindings are ABI. | **C3**: hoist `reactor::jobs` to its own jobs-facet module. Move the WIT bindings plus `owned_abi` to an ABI module at the bottom. Move `runtime_lifecycle_step` to `bounded_ownership` and the gateway failure type to `jobs`. |
| R5 | declaration → reactor / instance | `artifact_inference` → `plugin_runtime::{cancel_extension_inference, with_extension_inference_context}`; `app_emission` → `Host` (`TaskCtx`); `surface_contract` → `VcsArtifactApp` (`EditorSurfaceApp` / `ViewerSurfaceApp` aliases and `declarations` factories); `artifact_declaration` → `Plugin` | Real inversions. | Move the `EditorSurfaceApp` / `ViewerSurfaceApp` aliases and the `declarations` app factories to `instance`. Give inference execution a context port trait in `declaration`, implemented in reactor (L8). Pass `TaskCtx`'s `Host` as a port trait. |
| R6 | publication → jobs | `child_content` / `document_ingress` → `command_history` types (`CommandLogEntry`, `ShellHistoryReplay`, `HistoryView`, `HistoryCommandFilter`); `document_ingress` → `reserved_job_session`; `store_publication` → `ArtifactBoundedToolFault`, `TypedOperationResultLane` | The history model sits too high. | **C2**: move the command-log and history model to `publication`, and the tool fault / result-lane types to `declaration` (part of C1). |

## 5. Step 2 decision: why the crate cut is not clean now

1. **The graph is not one-directional yet.** R1–R6 above sum to 155 names against the order. Cutting now would need a cycle between crates, which is impossible, or `dyn` shims added just to compile. Both break the "no hacks / no shims" rule.
2. **Workspace layout (WS1 not landed).**
   - 185 manifests and **173 `Cargo.lock` files** contain `semio-framework-plugin`, measured with `rg -l` over `Cargo.toml` / `Cargo.lock`.
   - Each new crate must appear in every one of those lockfiles. They are edited concurrently by other lanes and pinned `--locked` by the prebuild.
   - After WS1 (one product workspace and lockfile) this becomes one generator step.
3. **Consumers.**
   - **4,536 files** outside the crate name `semio_framework_plugin::…`. The top paths are `io` 2,419, `artifact_app_laws` 2,409, `app` 2,089, `retained_command` 757 and `built_to_component_tree` 624.
   - The exported macros (`app_commands!`, `plugin_exports!`, `framework_reserved_job!` users, `dyn_enum_close!` through `plugin_app_close_prelude`) expand to `$crate::…` paths.
   - Without re-export shims, the cut must rewrite every consumer to name the owning crates. That is the stdio-semio wave-3 pattern and belongs in the same wave as the cut.

## 6. Cut plan (precise; to run after WS1)

### 6.1 Target crates

The names are at most 48 characters (L4). All are `rlib` only (L5). A crate's directory is the facet's existing node, and its lib root is a new `📦️packages/🦀️rust/Cargo.toml` pointing at the facet node's wiring file.

| Crate | Lib root (wiring) | Modules (today's paths) | Depends on |
|---|---|---|---|
| `semio-framework-plugin-ownership` | `🏛️ownership/🦀️.rs` | `bounded_ownership`, `sdk_fault`, `component_persistent_local`, the WIT bindings plus `owned_abi` (C3) | kernel, value, diagnostic |
| `semio-framework-plugin-window` | `🪟️window/🦀️.rs` | window facet (§3.1) | ownership |
| `semio-framework-plugin-declaration` | `📜️declaration/🦀️.rs` | declaration facet plus the C1 contract vocabulary | ownership, window |
| `semio-framework-plugin-publication` | `🧩️composition/📬️publication/🦀️.rs` | publication facet plus the C2 history model | …, declaration |
| `semio-framework-plugin-jobs` | `👷️job/🦀️.rs` | jobs facet plus `reactor::jobs` (C3) | …, publication |
| `semio-framework-plugin-reactor` | `⚛️reactor/🦀️.rs` | reactor, plugin_runtime, host, plugin_program, describe, inference_gateway | …, jobs |
| `semio-framework-plugin` (top, existing name) | `🔌️plugin/🦀️.rs` | `artifact_instance` and its turns, ladders, `builder`, `app_testing` (feature `artifact-app-testing` stays test-only, L2) | all |

### 6.2 Waves

Each wave is compile-atomic and is gated on the five configurations, the per-test sweep and `test-retained-raw-source`.

- **C1. Contract vocabulary down (resolves R1, R2, R3).**
  - Move the request, handle, completion, view and store-lane default types that `ArtifactApp` / `ArtifactEditor` / `ArtifactViewer` / `PluginApp` name into `declaration`.
  - Use `🚚️move-items.py` and `🔁️fix-loop.py`; the heuristic closure is listed by `🧭️facet-imports.py --back`.
  - Where a type drags machinery (for example `ArtifactOwnedToolJobContext` → `ToolCancellationLease`), split it into a data part (declaration) and a driver part (jobs).
  - **Gate:** zero declaration → {publication, jobs} edges.
- **C2. History model and window owners.**
  - Move the command-log and history types to `publication`.
  - Move `AppActionRegistry` to `window_kits` and the two window test helpers.
  - **Gate:** zero window → upper and publication → jobs edges.
- **C3. ABI and nesting.**
  - Hoist `reactor::jobs` and the WIT bindings plus `owned_abi`.
  - Move `runtime_lifecycle_step` and `ArtifactInferenceGatewayFailure` down.
  - Move `EditorSurfaceApp` / `ViewerSurfaceApp` and the `declarations` factories to the instance.
  - Give inference execution a port trait (R5).
  - **Gate:** `🧭️facet-imports.py` prints no edge against the order and no module SCC.
- **C4. Visibility.**
  - Turn every cross-facet `pub(crate)` that C1–C3 left into either `pub` API of its crate or a narrower path (§3.2 gives the count).
  - Fields of `VcsArtifactApp` that facets touch become facet-owned sub-structs. This is the L8 decomposition of the god object; `rg 'pub(crate)' 🧵️instance/🦀️.rs` is its worklist.
- **C5. The cut (needs WS1).**
  - Create the six new manifests. Turn each facet wiring file into a crate root: move the `#[path]` declarations of its modules from `🔌️plugin/🦀️.rs` into it.
  - Replace `crate::<module>::` with `semio_framework_plugin_<facet>::<module>::` across crates. This is the same owner-index codemod as `🔀️unfacade.py`.
  - Delete the `app` vocabulary module and `plugin_app_close_prelude`.
  - Run the consumer codemod over the 4,536 files. Rewrite `$crate::` paths in the exported macros to absolute owner crates.
  - Regenerate the product lockfile.
  - **Gate:** G1 (one unit per crate), G3 (every crate at or under 15 k lines) and the hub wasm workspace check.
- **Expected effect** (§6.5 of the architecture note): the plugin segment of the critical path goes from 75–125 s (one unit) to about 4 parallel units of 15–25 s each. The largest unit falls from 3.6 GB to about 1 GB (estimate, 0.6 GB + 38 MB per k-line).

## 7. Verification (all run in this session)

| Check | Result |
|---|---|
| `cargo check -p semio-framework-plugin --lib` (native) | 0 errors, **126 warnings**, down from 135 before the change. The `unused_qualifications` warnings introduced by the path rewrites were fixed, and so were pre-existing unused imports in the import headers of files this work touched. |
| same, `--target wasm32-wasip2` | 0 errors |
| same, `--target wasm32-wasip2 --features component-guest` | 0 errors |
| same, `--features artifact-app-testing` | 0 errors |
| same, `--profile test` | 0 errors |
| Plugin `--lib` sweep, one process per test, 5 in parallel, 180 s alarm, `RUST_MIN_STACK=64 MiB` | see §7.1 |
| `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-plugin:test-retained-raw-source --skip-nx-cache` | **16 pass / 4 fail; all 4 have causes outside this work** (§8). The 5 suites this work broke are repaired. |
| `cargo check --keep-going --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2` | `semio-framework-plugin` checks clean in the hub workspace. The failures are 39 × `cannot find base_foundation in crate` (stdio-semio split in flight, Lane C) and one `artifact_catalog_contribution is private` (hub stdio catalog). None comes from this crate. |

### 7.1 Plugin lib sweep

Results:

- the pre-change baseline at the start of the session: `⚡️cache/e2/keep/base-results.tsv`;
- the final run, compared against the 957-ok baseline (`play-fleet/prod-fix/keep/plugin-results.tsv`) and r15: `⚡️cache/e2/keep/s4-results.tsv`.

Test names moved with their modules (`component::app::X` → `component::<facet>::X`), so they are matched on their last two path segments.

| Run | ok | FAILED | TIMEOUT | tests |
|---|---:|---:|---:|---:|
| r15 (pl-tests) | 951 | 275 | 0 (+1 ABORT, 2 EXIT133) | 1229 |
| prod-fix (the 957 baseline) | 957 | 271 | 0 (+1 ABORT, 2 EXIT133) | 1231 |
| this session, before any edit (`base`) | 1051 | 173 | 7 | 1231 |
| **after step 1 (`s4`)** | **1052** | 172 | 7 | 1231 |

- **Regressions: 0** against `base`, prod-fix and r15.
  - The one name the comparison script flags is a rename: `component::app::original_bounded_config_factory_retains_source_until_exact_physical_retirement` is now `component::store_lifecycle::…`, and it is ok under its new name.
  - Newly ok against the 957 baseline: 96. Against r15: 102. Almost all of these come from peers' fixes before this session; against `base` the net gain is +1.
- **Flaky tests.** Two time-travel laws, `replay_progress_rides_the_unsolicited_ui_frame_with_the_session_status` and `the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver`, failed in the intermediate sweep `s2`. Rerun alone, the pre-change binary also fails the first one in 2 of 2 runs and the new binary passes it in 2 of 3, so they are load-dependent. Both are ok in `s4`.

## 8. Failures that are not from this work (evidence)

- **`test-retained-raw-source` "mounted original lease …" and "yielded window refresh …".** They expect `std::mem::size_of::<…>` text. That text is already absent from the pre-change commit `c73888a85bb`, because warning-flood applied `unused_qualifications` crate-wide (corrections, warning-flood section).
- **"rejected original read return …" and "registered owned serializers …".** They read `🏪️store/🦀️.rs` (os-kernel, Lane E1, edited during this session) and `🚪️io/📤️serialization` fixtures. Neither file is touched by this work.
- **`semio-framework-plugin-describe --lib`** fails to compile in `🖨️descriptor-emission/🦀️.rs`: the actor `Host` trait has no `operation_*` methods (WIT / A2 churn). My only edit there is the `PLUGIN_SDK_SOURCE` path in its unit test (§3.4).
- **os-kernel was intermittently broken by E1 edits.** Three times during this session it did not compile (store module renames), and checks waited for it.

## 9. Inputs created (kept in `e2-plugin/`)

| File | Purpose |
|---|---|
| `🧱️rust_items.py` | literal-safe item splitter (const-generic brace fix ported from E1) |
| `🕸️plugin-module-graph.py` + `🗺️facets.json` | item graph and SCCs (before) |
| `🧭️facet-imports.py` | import-exact module and facet graph (after) |
| `🪚️decompose.py` + `🗂️destinations.json` | root decomposition codemod |
| `🔁️fix-loop.py` | compile-driven import and visibility fixer |
| `🧹️imports.py` | exact import blocks: flatten, regate, prune, group |
| `🔀️unfacade.py` | internal `app` paths → owners |
| `🧲️extract-impls.py` | instance impl extraction |
| `🚚️move-items.py` | item mover |
| `🏗️cargo.sh`, `🧪️each-test.sh` | slot-gated cargo and the per-test runner |

None of these is a permanent command; they are ticket inputs, so no launch entry was added.
