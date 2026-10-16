# 🧭️ Result PLUGIN-WP2: declaration contracts carved into seven producer crates

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool (repo MCP down). Contract: `PLAN-PLUGIN-CRATES.md` section 12.5 and Appendices H to K, built on `RESULT-PLUGIN-WP0.md`. Tools: `pwp2-tools/` (all mine, re-runnable; the carve is generated, not hand-edited). Real compile is still refused (see section 5); the verification is static plus the stitched union differential.

## 0. Verdict

WP2 is applied to the repository. The seven class-B crates `decl-views`, `decl-emission`, `decl-app`, `app-builder`, `decl-surface`, `decl-registry` and `plugin-program` are real crates now: their root is the existing module file, `[lib] path` points at it, the package-local stubs are deleted, hub 2 (`🪝️mounts/📜️wp2-declaration/🦀️.rs`) holds `extern crate` / re-export / thin-module lines for them, and the 12 interim lift mounts of other packages were flipped to re-exports. All seven are inside the compiler-ingress source gate on every closure (section 2). The differential is clean for WP2 except for rows that depend on peer packages (section 5.2). Nothing is compiled for real.

## 1. Decisions applied

D1 to D8 as in plan 12.4. D4 in practice: test-only helpers that tests outside their crate use became `#[doc(hidden)] pub` (never a test feature): `handle_encoding`, `test_take_download`, `test_with_name`, `with_busy_test_lock`, `encode_contributed_wire`. Names that other crates or facade tests use were widened by name (`widen.py`, 340 definitions in 19 files; `WIDEN_EXTRA` holds the reviewed leftovers of `defscan.py`).

## 2. Census (gate counter `readRustSourceClosureV1`, files / bytes / active lines)

Limits 32 / 262,144 / 4,000, internal targets 28 / 230,000 / 3,500. Measured after the apply with `bun census.ts` (row `overLimit` = false for all seven) and with `pwp2-tools/censusrun.py` (per closure).

| crate | lib native = wasip2 = wasm32-unknown | lib + testing features | test native = wasip2 |
|---|---|---|---|
| decl-views | 13 / 215,520 / 3,442 | same | 19 / 238,729 / 3,594 |
| decl-emission | 5 / 112,948 / 1,488 | same | 5 / 112,948 / 1,488 |
| decl-app | 14 / 192,898 / 2,733 | 16 / 205,811 / 2,787 | 15 / 193,768 / 2,748 |
| app-builder | 2 / 92,354 / 1,586 | 2 / 92,354 / 1,590 | 6 / 105,729 / 1,789 |
| decl-surface | 2 / 152,814 / 2,408 | 2 / 152,814 / 2,415 | 2 / 152,814 / 2,430 |
| decl-registry | 5 / 154,526 / 2,687 | same | 7 / 160,177 / 2,786 |
| plugin-program | 5 / 60,739 / 1,056 | same | 7 / 74,197 / 1,255 |

Only `decl-views` test closure exceeds an internal target (238,729 bytes > 230,000; 3,594 lines > 3,500) and stays inside the hard limit. The facade `semio-framework-plugin` is still `overLimit` (206 files / 4,244,630 bytes / 55,667 lines in the worst closure of the end census, shared with the concurrent WP1/WP3/WP4/WP7 carves; at the end of WP0 the lib native closure was 195 / 6,018,889 / 83,307). Companions `tests-03` and `tests-04` stay skeletons (1 file, 202 and 206 bytes): the tests that name facade-only items run as facade test hosts instead (section 4.4).

## 3. Crate graph

Frozen Appendix H edges, all downward; `tomlrun.py live` reports 7 manifests, 142 path dependencies, 0 problems, 0 first-party edges added to the frozen set.

| crate (layer) | root | first-party deps |
|---|---|---|
| decl-views (L2) | `📜️declaration/👁️view` | interaction, operation, window-config |
| decl-emission (L3) | `📜️declaration/📨️emission` | actor-host, operation, window-config, decl-views |
| decl-app (L4) | `📜️declaration/🧩️app` | interaction, operation, window-config, window-transient, window-kits, decl-views, decl-emission |
| app-builder (L5) | `🏗️builder/🧩️app` | window-transient, decl-views, decl-app |
| decl-surface (L6) | `📜️declaration/🪟️surface` | abi, interaction, window-config, window-transient, window-kits, decl-views, decl-emission, decl-app, app-builder |
| decl-registry (L7) | `📜️declaration` | identity, window-kits, decl-app, app-builder, decl-surface, reactor-app |
| plugin-program (L8) | `🏗️builder/🔌️program` | identity, decl-views, decl-emission, app-builder, decl-surface, job, reactor-app, decl-registry |

Owned trees mounted as children: `📜️declaration/**` (view, child, tool-machine, command, media, tool-run, emission, app, job with its tests, surface, natural-file, registration, tree), `🏗️builder/🧩️app` (with `🖐️gumball-audience`), `🏗️builder/🔌️program`, `⚛️reactor/🪟️surfaces` (plugin-program), `🫧️transient`, `🎚️config`, `📝️draft`, `👥️presence` retirement files (decl-views) and `🧵️task/🛬️source` (decl-emission). Own included files (retirement modules, the gumball audience, the job tests) are rewritten and widened like the crate roots.

## 4. What changed

1. **65 files written** by `apply.py write` (drift guard against the build input passed): the seven crate roots and their module files, lift files (`🛬️lifted/<slug>/🦀️.rs`) for the cuts below, 9 test host files and the facade macro mount file. After the apply `out/` and the live tree are byte-identical (65 compared, 0 differ).
2. **7 manifests** rewritten (`📦️packages/🦀️rust/Cargo.toml`: `[lib] path = "../../🦀️.rs"`, features, dependencies from the frozen set with the external crates found by token evidence) and the 7 package-local stub `🦀️.rs` files deleted.
3. **Cuts** (an item travels with its bound inherent impls and the trait impls it owns; orphan scan: 148 trait impls, 0 candidates):
   - `tool_contract` (job): the operation-owner family goes to decl-views (`ArtifactInstanceOperationOwner*`, `OriginalInferenceSession*`, the close and metadata helpers, `EmptyArtifactInstanceOperationOwner`) with the two inference tests.
   - `app_emission` `EphemeralEmit`, `tool_run_contract` `ToolRunJobRequest`, `media_contract` export request and retirement go to decl-app; `surface_contract` `ViewEmit` to decl-emission; `plugin_program` `AppFactory` to decl-registry.
   - Out of WP2 (lift files for the destination owner): `ArtifactOperationProgress` and the cancel action id to `operation`; `preflight_artifact_declarations` and `commit_artifact_declarations` to `extension`; `editor_surface`, `viewer_surface`, `EditorSurfaceApp`, `ViewerSurfaceApp` to `instance-app`; `MediaArtifact` and `whole_document_media` to `document-media`.
   - The `include!` lines of `🎞️media/🛫️intrinsic` and `🎞️media/💥️fault` left the app contract file and are included by the `MediaArtifact` lift (same module level as before), so the cut is self-contained.
   - The `#[macro_export]` macros of the emission and job files moved to the facade macro mount `📣️macros/📜️wp2-declaration/🦀️.rs`.
4. **Facade test hosts** (`🪝️mounts/📜️wp2-declaration/🧪️hosts/<emoji><slug>/🦀️.rs`, 9): tests that name facade-only items run beside hub 2 instead of inside a crate: `declarations-fixture`, `app-builder` (needs `reserved_jobs`, `app-builder` has no `job` edge), `codec-binding`, `app-commands`, `owned-media-error`, `export-credit`, `media-publication`, `child-receipt`, `contributed-mutation-wire`. `declarations::fixture` is re-exported from its host for the other tests that name `crate::declarations::fixture`.
5. **Hub 2** regenerated from the live hub: every unit of a carved module is `extern crate <crate> as <old name>`, a re-export of the child, or a thin module where items left (`app_contract`, `media_contract`, `tool_run_contract`, `app_emission`, `surface_contract`, `declarations`). `copy_media_text_parts` (and for tests `receive_media_artifact_fault`, `receive_media_output`) are re-exported from `document-media` in `app_contract`. Region markers: 14 old, 16 new, all old ones kept in order (`mounteq.py`).
6. **Root and reactor**: the root-resident units `app_emission` and `tool_contract` were removed from the facade root; the `reactor` mount line of `surface_context` now points at plugin-program. Both are the tolerated foreign-line edits.
7. **Consumers re-pointed** (12 interim `#[path]` mounts of other packages flipped to re-exports of the WP2 lift): `composition/emission/preparation`, `retained-command/context`, `reactor/runtime` (plugin_runtime), `wp1-foundation` hub (completion fault, refusal, retirement), `inference` (decl-registry and plugin-program), `inference/gateway`, `operation-progress`, `media/intrinsic`, `media/turn`. One TS source-text reader was re-pointed: `🎞️media/⚠️turn/🧪️tests/🟦️.ts` line 25 now reads the `ArtifactMediaExportJobRequest` struct from `📜️declaration/🧩️app/🛬️lifted/🎬️media-export/🦀️.rs`.

## 5. Verification (commands run, real results)

### 5.1 Static gates (all on the final tree)

| check | result |
|---|---|
| definition scan `defscan.py` | 50 private definitions mentioned outside their crate, all reviewed (fields of foreign structs with the same name, re-widened by `WIDEN_EXTRA` where real); seeded (`--seed`, a private definition made visible to an outside file): 51, so the scan sees the seed |
| orphan scan `orphan.py` | 148 trait impls scanned, 0 candidates |
| `localrefs.py` | 0 unknown crate-local first segments |
| `pathcheck.py` | 80 path literals, 0 unresolved |
| mount equivalence `mounteq.py` | 23 old modules, 23 defined by hub 2, none missing; markers kept |
| unique owner `uniqueown.py` | 53 crates with a tree, 634 files, 0 files reachable from two roots |
| TOML | `tomlrun.py out|live`: 7 manifests, 142 path deps, 0 problems |
| census | section 2; all seven `overLimit` false |
| rustfmt (nightly, `skip_children`) | 76 applied files incl. hub, root and 9 hosts: parsed 76, bad 0 |
| TS tests (`tsdiff.py` base vs after) | 6 differences left, none mine: `♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts` (missing companion `🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts`) and `media output birth requires caller-funded ownership…` (reads `🧵️instance/🚪️ingress/🦀️.rs`, which WP7 moved). The one difference that was mine (the struct reader) is fixed |

### 5.2 Union differential (A = carved, B = live pre-carve, sandbox patches of the oracle, rustc analysis only)

| config | A | B | signal rows only in A |
|---|---|---|---|
| lib (v10) | 9,592 | 9,481 | 9 rows (10 diagnostics) |
| test (v10) | 22,331 | 22,066 | 9 rows listed below plus the paired `couldn't read include_str!` rows (sandbox copies only `.rs`, identical on both sides) |
| wasm32-wasip2 (v9) | 9,877 | 9,766 | same 9 rows as lib |
| lib + testing features (v9) | 10,456 | 10,329 | same 9 rows as lib |
| test + testing features (v9) | 22,343 | 22,090 | the v9 test rows; the two causes fixed afterwards (below) were confirmed on test (v10) |

The residual rows, none caused by a WP2 defect:
- `NoPluginApp` x2 and `TransientStore` x1 (unresolved import): the same unresolved external is also in B under the old path (`crate::plugin_app::NoPluginApp`, `crate::store::TransientStore`); paired rows, only the path text differs. B also has two `Fault` rows that A resolves.
- `cancel_extension_inference`, `with_extension_inference_context` (4 rows) and `plugin_internal_fault` (1 row): see hand-offs. A peer cut the first two into the `extension` lift (`🧩️extension/🛬️lifted/🏃️runtime`) although the plan places them in the plugin-program runtime lift, and `reactor-app` does not export `plugin_internal_fault` yet.
- `copy_media_text_parts` (E0364) and `receive_media_artifact_fault` / `receive_media_output` (E0432, test only): they are private or `pub(crate)` in the WP3 files `🎞️media/🛫️intrinsic` and `🎞️media/💥️fault`; WP3 must widen them when it wires `document-media`.
- `Open` (E0220, lib only): cascade of the neutralised `store` trait bounds in the composition group file, absent in the test config.

Two real defects found by the differential were fixed before the final apply: `declarations::fixture` was lost by the host extraction (now re-exported from its host) and `app-app-builder` could not reach `join_framework_shared_action_dispositions` (now a facade test host). A third oracle artifact was removed from the tool (re-exports of crate items were rewritten to `pub(crate)`, producing false E0365 rows). Counts and rows are in `🗑️generated/pwp2-oracle-v*.log` and the scratchpad `pwp0-oracle-w2*.jsonl`.

### 5.3 Seeded sensitivity of the differential (lib, one seed each, run under heavy machine load)

- `seed:import` (a `use` removed from the view file): caught, `cannot find type LocalizedLabel` at `📜️declaration/👁️view/🦀️.rs:551` and `:892`; B of that run was degraded (2,183 errors), so the row counts are not comparable.
- `seed:pub` (a `pub fn` made private): inconclusive. The A run ended with a truncated diagnostic stream (4,649 errors, last JSON line cut) and no privacy row; it was not repeated because of the SPEED directive. The definition scan seed (section 5.1) covers the visibility case.
- `seed:path` (broken `#[path]`): the run was cut off before it reported. `pathcheck.py` (80 literals, 0 unresolved) is the static replacement; it resolves every literal against the final tree.

### 5.4 Real compile

`bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-plugin-decl-views --message-format=short` at the end of the run: exit 1, `error: Cargo authored profiles belong to canonical two-intent consumption` (from `…/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts:36`). The launcher refuses before cargo starts, as at the start (`🗑️generated/pwp2-cargo-start.log`, `pwp2-cargo-end.log`). No check or test was run for real; nothing here claims the crates compile.

## 6. Deviations and justification

1. The two `include!`d WP3 files (`🛫️intrinsic`, `💥️fault`) are included by the `MediaArtifact` lift instead of being mounted by WP3, so decl-app no longer contains document-media code and the lift stays equivalent to the old module scope.
2. Nine test files run as facade test hosts instead of inside a crate (they name facade-only items or crates outside the frozen edges). They keep their text; only their location changed. `tests-03` and `tests-04` stay skeletons.
3. `HOME_OVERRIDE` in the tool maps `project_media_error`, `key_media_error`, `media_projection_error` and `EphemeralParts` to their real owners and `plugin_internal_fault` to `reactor-app`; the plan table had them under the old module name.
4. The oracle's own sandbox patches were extended (module-name-preserving paths, `pub(in crate::component::pl_k)` visibility, synthesized `pl_*` modules for uncarved crates). They live only in the sandbox.
5. The post-apply differential (`uo.py --post`) and a full-monolith differential were not run after the apply: the coordinator's SPEED directive asked to stop confirmation rounds. The apply is byte-identical to the verified `out/` tree (65 files compared) and the static gates above ran on the live tree.

## 7. Hand-offs and residual risks

- **WP0/verify_static**: forward `decl-app/artifact-app-testing` through the facade feature (the hosts and tests need it); the frozen `verify_static` table and the class-B lib-path expectations must be updated (the stubs are gone, `[lib] path` is the root file); `Cargo.lock` is not refreshed.
- **WP1**: hub 1's three lines now re-export decl-app. `operation_progress::cancellation_result_lane` lives in decl-app, so `operation` cannot re-export it.
- **WP3**: wire the `document-media` lift (`🗃️document/🛬️lifted/🧩️app`) and mount `🛫️intrinsic` / `💥️fault`; widen `copy_media_text_parts` to `pub` and `receive_media_artifact_fault` / `receive_media_output` to `#[doc(hidden)] pub` (re-exported by hub 2). The receipt test is hosted in the facade.
- **WP4**: `reactor-app` must export `plugin_internal_fault` and `NoPluginApp`; `cancel_extension_inference` and `with_extension_inference_context` must live in the plugin-program runtime lift (plan 6/1691) or the plugin-program inference gateway lift must move to `extension`, because plugin-program cannot depend upward on `extension`. The contributed-mutation-wire test is hosted in the facade.
- **WP7**: wire the `instance-app` lift for `editor_surface`, `viewer_surface`, `EditorSurfaceApp`, `ViewerSurfaceApp`; two TS tests read paths WP7 moved (`🧵️instance/🚪️ingress`, `🧷️companions/🧯️test-support/🧵️instance-family`).
- **Risks**: nothing is compiled for real; the sandbox oracle hides real name resolution of the neutralised externals (`store`, `protocol`, the `semio_framework_*` globs), so first real `cargo check` may report import paths in the 9 test hosts and in the lift headers; the `decl-views` test closure sits near the internal size target; the pub-seed run is inconclusive.

## 8. Files

Tooling and logs: `pwp2-tools/` (all `.py`/`.ts`), `🗑️generated/pwp2-*.log`. Source changes: the 65 applied files under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` (crate roots and module files of the seven crates, lift files, hub 2, `🧪️hosts/*`, the macro mount file, facade `🦀️.rs`, `⚛️reactor/🦀️.rs` mount line), the 7 manifests, the 7 deleted stubs, the 12 flipped mount files of other packages and `🎞️media/⚠️turn/🧪️tests/🟦️.ts`.
