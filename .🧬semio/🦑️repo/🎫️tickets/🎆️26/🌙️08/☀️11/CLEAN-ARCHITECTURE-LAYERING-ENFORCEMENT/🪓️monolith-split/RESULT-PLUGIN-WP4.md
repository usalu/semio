# 🧭️ Result PLUGIN-WP4: reactor package staged into 13 crate trees (S0), not flipped

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-PLUGIN-CRATES.md` (contract = section 12.5, Appendices H to K), on top of `RESULT-PLUGIN-WP0.md`. Tools: `pwp4-tools/` (this folder). Logs: `🗑️generated/pwp4-*`.

## 0. Verdict

WP4 is done up to the S0 stage. The 13 lower crates of the reactor package and `tests-07` have their code cut out of the old parent modules into per-crate piece directories, and the old parents mount the pieces with `#[path] mod X; pub use X::*;`. **Nothing is flipped**: the tree is still one compilation unit and resolution-equivalent to the tree before WP4 (patched full-resolution differential, section 6). The reasons for not flipping now:

* the dependencies of the WP4 crates are skeletons (`plugin-program`, `decl-registry`, `extension`, `decl-app`, `decl-emission`, `app-builder`, ...) or were only just flipped by WP1 (`abi`, `identity`, `actor-host`). A flip would make every peer's differential red (12.5.4);
* real compile is refused by the image headroom and the launcher is broken by a foreign file (section 7), so a flip could not be validated by `cargo check` anyway.

What is ready: pieces and mounts, manifests with the downward edges, the three lift-file rewrites (two written, four withdrawn), five facade macros in `📣️macros/⚛️wp4-reactor`, the F7 port that breaks the lifecycle cycle, the source-text readers, the law test, and a mechanical flip checklist with a statics table (section 10).

D8 (`reactor-tasks`, Option B) is **confirmed** by the definition scan (section 3): 16 owned members, zero users below rank 25.

## 1. Decisions

| id | decision |
|---|---|
| D8 | Option B kept. `reactor-tasks` (rank 25) owns `TASK_RECORDS`, `TASK_RESUMES`, `INSTANCE_METADATA`, `REGISTRY`, `REACTOR_EXECUTOR`, `TASK_EXECUTOR`, `JOB_RENDER_BINDINGS`, `ReactorFixedSlots`, `spawn_task`, `queue_extension_invocation`, `host_for_instance`, `cancel_instance_tasks_step`, `instance_task_quota` and the registries behind them. `users below reactor-tasks: 0` (`pwp4-d8-users.log`). The closure is wider than the two statics of the plan: `REGISTRY` and the executors are read by lifecycle, core and turn, so they sit with the tasks. |
| S0 location | Pieces of a file live under that file's directory (the TS `readModuleTreeSource` skips mounts that leave the directory). The first cut put lifecycle pieces under `⚛️reactor/♻️`; they were re-cut into `🏃️runtime/📥️staging/♻️reactor-lifecycle` and `🏃️runtime/📥️staging/🔌️reactor-app`. |
| statics | `component_persistent_local!` statics have no visibility parameter, so they stay in the root block in S0 (table in section 10). At the flip each gets an owner crate plus accessors. |
| F7 | The lifecycle rank (28) is below core/patches/surfaces, yet `GuestLifetimeOwner` called `PATCHES` and `COLD_PAIR_INGRESS`. Resolved by a port: `pub(crate) trait ClosePeers` (10 methods) in `🚪️lifetime/🦀️.rs`, generic `P: ClosePeers + ?Sized` parameters on `terminal_is_empty`, `release_terminal`, `prepare_retired`, `release_owner_step`, `request_close`, `advance_admission`, `prepare_turn`, and `pub(crate) struct ReactorClosePeers` in the root implementing it with the old `PATCHES.with(...)` / `COLD_PAIR_INGRESS.with(...)` bodies mapped through `reactor_close_fault`. An associated type `Peers = dyn ClosePeers` was rejected: it defaults to `'static`. Not an extension trait around an inherent impl; the trait is a real inversion of control with one implementation per crate level. |
| macros | Five facade macros are cut into `📣️macros/⚛️wp4-reactor/🦀️.rs` (628 lines). |
| lifts | Two lift files written (extension, plugin-program); four withdrawn (PD5, PD6, PD8, PD9). |

## 2. Census (gate counter `readRustSourceClosureV1`, files / bytes / active lines)

Limits: target 3,500 lines / 28 files / 230,000 bytes; gate 4,000 / 32 / 262,144. Measured with the mounts of each crate virtualised (`pwp4-tools/census4.ts`, `🗑️generated/pwp4-census-final.log`), seven rows per crate (lib native, wasip2, unknown-unknown, lib native features, test native, test wasip2, test native features). Worst row per crate:

| crate | files | bytes | lines | row |
|---|---|---|---|---|
| reactor-app | 2 | 54,657 | 613 | all |
| reactor-tasks | 8 | 40,783 | 740 | test |
| reactor-lifecycle | 14 | 170,787 | 2,950 | test wasip2 |
| runtime-instances | 6 | 58,302 | 739 | test wasip2 |
| runtime-ops | 13 | 187,848 | 2,160 | test native features |
| runtime-exchange | 2 | 53,836 | 821 | all |
| reactor-patches | 11 | 204,612 | 3,205 | test |
| reactor-surfaces | 9 | 67,361 | 962 | test |
| reactor-core | 9 | 54,361 | 806 | test wasip2 |
| reactor-jobs | 19 | 210,147 | 2,287 | test |
| reactor-turn | 6 | 209,888 | 2,645 | test wasip2 |
| reactor-wit-bridge | 2 | 48,443 | 599 | all |
| reactor-context | 26 | 184,696 | 749 | test wasip2 |
| tests-07 | 27 | 192,799 | 2,653 | all |

Every row is inside the target on all three axes (largest: bytes 210,147 for reactor-jobs, lines 3,205 for reactor-patches, files 27 for tests-07). `tests-07` is at 27 of 28 files: any added hosted test needs a new companion (PD13).

Longest path of a file written by WP4: 176 bytes (limit 240); the longest legacy path (270 bytes) is outside WP4's files.

## 3. D8 and definition/orphan scans

| scan | result | seeded check |
|---|---|---|
| definition scan (`defscan4.py`: member private but used outside its crate) | 625 members, 0 violations (`pwp4-defscan.log`) | dropping one `pub(super)` in `🛎️app-cell` produces exactly 1 violation (`RuntimeInstanceRefusal`, `🧯️cleanup-pumps`) |
| orphan scan (`orphan4.py`: inherent / trait impl placed in a crate other than its type's) | 117 impls, 0 violations (`pwp4-orphan.log`) | moving one `PluginRuntime` inherent impl block to another crate produces exactly 1 violation (`🏷️metadata-owner`, `runtime-instances` vs `reactor-lifecycle`) |
| D8 users (`d8users.py`) | 16 owned members, 0 users below rank 25 | not applicable |
| mount equivalence (`mounts4.py`) | `🏃️runtime/🦀️.rs` 14 mounts before, 32 after (residual plus pieces), lost 0, added non-piece 0; `⚛️reactor/🦀️.rs` 21 before, 30 after, lost 0, added non-piece 0 | n/a |
| chunk equivalence (`chunks4.py`) | 454 chunks before, 608 after; gained 160 (mounts 27, `use super::*` 27, re-exports 26, docstrings 28, other 52); lost 6 (five cold-pair close helpers replaced by `ReactorClosePeers`, plus the `mod wit_bridge { use super::* ... }` wrapper, now the unwrapped `🪢️bridge` piece) | n/a |

The orphan scan matters because an inherent impl can never span crates; the scan is the proof that `impl PluginRuntime` blocks all live with the type's owner.

## 4. Crates, pieces and ownership

Roots: class A crates (free roots) and class B (the old module file is the root). Pieces (lines at end of work):

* **reactor-tasks** (`⚛️reactor/⏲️tasks`): `🧮️fixed-slots` 87, `📇️instance-metadata` 54, `🧵️task-lane` 344, `🔗️extension-continuation` 103, `🧷️render-bindings` 75.
* **reactor-lifecycle** (root `📨️pending`): `🛎️app-cell` 170, `🧹️cleanup-vocabulary` 192, `🗃️instance-registry` 246, `🧯️cleanup-pumps` 437 (from the runtime file, in `🏃️runtime/📥️staging/♻️reactor-lifecycle`), and `📨️pending/🔒️reactor-close` 120.
* **reactor-app**: `🏃️runtime/📥️staging/🔌️reactor-app/🛠️runtime-helpers` 71 (`plugin_internal_fault` moved here, its lowest user).
* **runtime-instances** (`🏃️runtime/📦️instances`): `🖼️wire-frames` 67, `🪪️descriptor` 108, `🚪️opening` 118, `🔁️cleanup-turns` 418, `🏷️metadata-owner` 16.
* **runtime-ops** (`🏃️runtime/🧰️ops`): `🛡️wire-limits` 235, `📄️document` 366, `🎞️media` 60, `🖥️surface-render` 331, `📥️ingress` 266, `🧾️typed-operations` 669.
* **runtime-exchange**: `🔄️exchange/🔀️plugin-exchange` 815.
* **reactor-turn**: `🔄️turn/🏁️close-step` 40 (the other turn pieces stay in the existing turn files).
* **reactor-wit-bridge**: `🧬️wit-bridge/🪢️bridge` 593.
* **reactor-patches, reactor-surfaces, reactor-jobs, reactor-context**: existing module files are already their roots; no new pieces.
* **Residuals**: `🏃️runtime/🦀️.rs` 99 lines (was the 5.5k-line runtime file; mounts plus the builder-contract test `include!`), `⚛️reactor/🦀️.rs` 537 lines (statics block, `ReactorClosePeers`, timers, package assets, checkpoint and restore).
* **Lifts**: `🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs` 336 lines (ExtensionBundle, ExtensionManifest, `extension_invoke`, `foreign_kernel_value_bridge`, `extension_dispose_cold`, the two inference wire entry points, and by PD7 `cancel_extension_inference` and `with_extension_inference_context`); `🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs` 81 lines (`encode_wire_serialized`, `key_ordered_wire_value`, `decode_wire_serialized`, `PluginDescriptorExtras`, `wire_list_artifact_mutations`, `wire_artifact_mutation_plan`, contributed-mutation-wire test include).
* **Macros**: `📣️macros/⚛️wp4-reactor/🦀️.rs` 628 lines.

Manifest edges added (`+` versus the frozen Appendix H, all downward): reactor-lifecycle +reactor-tasks, runtime-instances +reactor-tasks, reactor-surfaces +reactor-tasks, reactor-core +reactor-patches, reactor-wit-bridge +reactor-core. Verified by `pwp4-tools/verify_wp4.ts`: 14 crates, manifests parse, boundary triple 262144 / 32 / 4000, workspace root, lib paths exist, every first-party dependency resolves, rank order strict, no cycle, project files exist: `WP4 STATIC CHECKS PASSED` (`pwp4-verify-wp4.log`).

## 5. Plan defects found and handled

| id | defect | handling |
|---|---|---|
| PD1 | the plan graph cannot see statics inside `component_persistent_local!` blocks | statics table in section 10; they stay in the root in S0 |
| PD2 | lifecycle (28) calls patches (32) and cold-pair (33) | F7 port `ClosePeers` |
| PD3 | inherent impls of `PluginRuntime` were spread over three crates | one block per owner crate; orphan scan keeps it so |
| PD4 | name collisions in the graph (token match) | six upward edges in `pwp4-dag.log` are all false positives: `FaultOrigin::Plugin`, the field `next_generation`, the method `has_work`; none is a real upward edge |
| PD5 | `cancel_job` lift to `actor-host`: it reads `JOBS` / `JobSlot` of `reactor-jobs` (35) | lift withdrawn (stub note in `🌐host/🛬️lifted/💼️jobs`) |
| PD6 | `drive_framework_reserved_registry_job(_counted)` lift to `job`: calls `start_job`, `step_job`, `IoRunControl` of `reactor-jobs` | lift withdrawn; the instance composer needs a `reactor-jobs` dependency (request to WP0) |
| PD7 | `cancel_extension_inference` / `with_extension_inference_context` assigned to `plugin-program` read `EXTENSION_BUNDLE` | moved to the extension lift |
| PD8 | `impl Default for SurfaceContexts` is an orphan-rule impl | lift withdrawn; the impl stays with the type in `plugin-program` (WP2) |
| PD9 | `Inputs` lift to `command`: unrelated private struct, name match only | lift withdrawn |
| PD10 | program -> extension edge via the gateway lift: `plugin-program` rank 27 would depend on `extension` rank 31 | open: WP2/WP3 must invert (pass the inference context in) or move the gateway cold entry points to `extension` |
| PD11 | declared edges missing for the code actually cut (`pwp4-edges-needed.log`): reactor-turn +decl-emission, app-builder, plugin-program; reactor-wit-bridge +decl-app; reactor-context +decl-emission, extension; runtime-ops +app-builder; reactor-surfaces +plugin-program | requests below; the manifests carry only edges whose targets exist as real crates |
| PD12 | `REGISTRY` / executor statics placement versus the 7.8 table | D8 closure widened; table in section 10 |
| PD13 | tests-07 file budget 27 / 28 | noted; a new hosted test needs a new companion |
| PD14 | declared-but-unevidenced edges (e.g. reactor-turn: abi, actor-host, operation, window-kits-rich, command) | left in the manifests (harmless); WP0 may prune |
| PD15 | source-text readers (`readFileSync` of the runtime file) went blind after the cut | eight files moved to `readModuleTreeSource`, four use the new `readPluginRuntimeSource()`; two roots added to `POLICY_SPLIT_SOURCE_ROOTS`; `📜️script.ts` line 6282 re-pointed |
| PD16 | the first S0 location of the lifecycle pieces left the cut file's directory | relocated (section 1) |

## 6. Verification

Differential: patched full-resolution oracle (`pwp4-tools/oracle4.py`): rustc nightly-2026-07-20 `--emit=metadata -o /dev/null` over A = the live plugin tree and B = the live tree with the pre-cut originals overlaid (`--b precut2`, cut from `pwp4-precut-s0.tgz`), sandbox patches neutralising externs, external globs and `pub extern crate`; compares the multiset of (error code, item) folded over file paths.

Run `full2` (state at about 00:00, all five configs, `🗑️generated/pwp4-oracle-full2.log` and `-summary.json`):

| config | A errors | B errors | only in A (new) | only in B (old) |
|---|---|---|---|---|
| lib (native) | 8,368 | 8,368 | 0 | 0 |
| test (`--cfg test`) | 3,479 | 3,479 | 0 | 0 |
| wasm32-wasip2 | 8,577 | 8,577 | 0 | 0 |
| lib + 4 testing features | 9,198 | 9,198 | 0 | 0 |
| test + 4 testing features | 3,480 | 3,480 | 0 | 0 |

Run `final1` on the final repo state (after the lease edit, the F7 generics, the `NoPeers` double and the stub notes; peers had meanwhile changed the tree, hence the different absolute counts): lib 8,359 / 8,359, test 3,482 / 3,482, onlyNew 0, onlyOld 0 in both (`pwp4-oracle-final1.log`).

The remaining errors are the unresolved externs and globs of the sandbox, identical on both sides; the claim is resolution equivalence, not a clean compile.

Seeded checks (mutating side A only, lib config), both detected:

| seed | result |
|---|---|
| `pub struct PluginRuntime<` made private in the `🗃️instance-registry` piece (`pwp4-oracle-seedp.log`) | A 8,464 / B 8,359, 105 new errors (E0425 `cannot find type PluginRuntime`) |
| `pub use life_app_cell::*;` removed from the runtime residual (`pwp4-oracle-seedm.log`) | A 8,384 / B 8,359, 25 new errors (`RuntimeAppCell`, `HostInstanceGuard`, `RuntimeInstanceRefusal` not found) |

Other checks:

| check | result |
|---|---|
| static pass over the 40 Rust files written or edited (`final_static.py`) | rustfmt parse failures 0, `#[path]` / `include!` targets all resolve, region markers balanced, every new file starts with a docstring line: `files 40 rustfmt failures 0 problems 0` |
| law test of the F7 port (`🚪️lifetime/🧪️tests/🤝️close-peers/🟦️.ts` + `🧫️fixtures/🤝️close-peers.json`: order vector of the ten peer calls, port method list, forbidden names) | `3 pass 0 fail 20 expect() calls` (`pwp4-law-close-peers.log`); seeded: swapping `reserve_patches` and `reserve_cold_pair` in `🚪️lifetime/🦀️.rs` makes the order test fail, restoring passes |
| third-party equivalence | the differential uses rustc itself as the third-party oracle: the same resolution errors, item by item, before and after |
| census | section 2, all inside the target |
| module-tree readers | re-pointed (section 9) but their TS suites were not re-run as a whole; only the close-peers law test above was run |

## 7. Real compile

Not run, and not claimed. `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-plugin-reactor-tasks --message-format=short --lib` was tried twice (`pwp4-cargo-1.log`): the first time the launcher failed with `SyntaxError: Export named 'prepareNativeLinuxImageStorageV1' not found` in `🏗️native-build/💾️storage/🐧️image/🟦️.ts`; the retry at the end failed with `Expected ";" but found ":"` at `🏗️native-build/💾️storage/🪟️image/🟦️.ts:473`, where a foreign process had written JSON ticket-close material into the TS file (`"moved": [],` ... at line 465 onwards). Both files are outside WP4 and untouched by me. Static gates stand in; the first real compile belongs to whoever flips.

## 8. Failures observed that are not WP4's

* failure-debit test 48 and the host/interpreter tests fail in the TS suites (pre-existing: SAC2 versus SAC3).
* the launcher errors above.

## 9. Files

All paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` unless stated. Full list of the 30 files written by WP4: `🗑️generated/pwp4-touched.txt`. Groups: 25 new piece files, the two residual parents, the two lift files, `📣️macros/⚛️wp4-reactor/🦀️.rs`, the F7 edits in `⚛️reactor/🚪️lifetime/🦀️.rs` (+ its two test files), `⚛️reactor/🔄️turn/🦀️.rs` (three call sites), `🚪️lifetime/🦀️.rs` (`from_cell` visibility), `🧷️companions/🍇️tests-07/🦀️.rs` (docstring), the four withdrawn-lift stub notes, the new helper `⚛️reactor/🏃️runtime/📖️source-text/🟦️.ts`, the Cargo.toml edits of section 4, and the tolerated foreign-line edits (eight TS codemod files, `…/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` +2 roots, `📜️script.ts` line 6282). Tools: `pwp4-tools/` (also holds the carve engine `carve4.py`, `apply4.py`, `f7*.py`).

## 10. Flip checklist for the owning flip (per crate, bottom-up by rank)

1. Replace the class A free root with the mounts; for class B keep the module file as root. The census virtualisation (`census4.ts`) shows the file set that belongs to each crate.
2. Statics (`🗑️generated/pwp4-statics.log`, 34 rows): the owner must declare the static and expose accessors; users listed per crate:

| static | owner | users |
|---|---|---|
| `LAST_MAINTENANCE_STAGE` | reactor-app | reactor-lifecycle (1) |
| `INSTANCE_METADATA` | reactor-tasks | runtime-instances (1), reactor-core (2), reactor-turn (3) |
| `JOB_RENDER_BINDINGS` | reactor-tasks | reactor-lifecycle (1), reactor-turn (2) |
| `REACTOR_EXECUTOR` | reactor-tasks | reactor-turn (1) |
| `REGISTRY` | reactor-tasks | reactor-lifecycle (1), reactor-core (1), reactor-turn (4) |
| `TASK_EXECUTOR` | reactor-tasks | reactor-turn (1) |
| `TASK_RECORDS` | reactor-tasks | reactor-core (1) |
| `TASK_RESUMES` | reactor-tasks | reactor-lifecycle (1), reactor-core (1), reactor-turn (2) |
| `PENDING_PATCHES`, `RUNTIME_CLOSE_PENDING_AUTHORITY_TURNS` | reactor-lifecycle | none |
| `REACTOR_CLOSES`, `REACTOR_CLOSE_CURSOR` | reactor-lifecycle | reactor-turn (1 each) |
| `NEXT_INSTANCE_ID` | runtime-instances | none |
| `PEAK_TYPED_OPERATION_SLOTS`, `TYPED_OPERATION_SLOT_DRAIN_PENDING` | runtime-ops | none |
| `PATCHES` | reactor-patches | reactor-core (1), reactor-turn (1) |
| `COLD_PAIR_DOCUMENT_LOAD`, `COLD_PAIR_INGRESS`, `RESTORED_DOCUMENT_LOADS` | reactor-surfaces | reactor-turn (3, 2, 1), reactor-core (0, 1, 1) |
| `ARMED_TIMERS` | reactor-core | reactor-turn (3), reactor-wit-bridge (1) |
| `PACKAGE_ASSETS`, `PRESENCE` | reactor-core | `PRESENCE`: reactor-turn (1) |
| `JOBS`, `KIND_REGISTRY`, `KIND_REGISTRY_EPOCH`, `REGISTERED_KIND_COUNT` | reactor-jobs | none |
| `COMMAND_INGRESS`, `PROCESS_POOL` | reactor-turn | none |
| `STAGED_PAGES` | reactor-wit-bridge | none |
| `ORIGINAL_*` (6) | reactor-context | none |

3. Remove the old mounts and the transitional `pub use` in the same change as the flip of the crate; the flipmap (`🗑️generated/pwp4-flipmap.log`) gives per crate the counts of `crate::`, `super::` and `use super::*` that must be rewritten, grouped by target crate (e.g. reactor-turn: 107 `crate::` paths, 73 to `plugin_runtime`).
4. Wire the lift files: WP3 adds one `mod` line for the extension lift and removes the `mod extension_runtime_lift; pub use ...` mount in the same change, or `EXTENSION_BUNDLE` exists twice; WP2 does the same for the program lift (`mod program_runtime_lift`).
5. Apply the edge requests below before flipping.

## 11. Requests

* **WP0**: add `reactor-jobs` to the instance composer's dependencies (PD6); add the missing edges of `pwp4-edges-needed.log` to Appendix H (reactor-turn: decl-emission, app-builder, plugin-program; reactor-wit-bridge: decl-app; reactor-context: decl-emission, extension; runtime-ops: app-builder; reactor-surfaces: plugin-program); prune declared-but-unevidenced edges (PD14).
* **WP1**: the `cancel_job` lift is withdrawn (PD5), drop the mount line in `actor-host`.
* **WP2**: invert the program -> extension edge of the gateway (PD10); keep `impl Default for SurfaceContexts` with the type (PD8); wire the program lift file and drop its transitional mount.
* **WP3**: wire the extension lift and remove its mount; `drive_framework_reserved_registry_job` lift is withdrawn (PD6); `Inputs` lift withdrawn (PD9).
* **WP6**: the instance composer and the runtime instance family use `reactor-tasks`, `reactor-jobs`; keep the single-block rule for `PluginRuntime` impls.
* **WP7**: tests-07 has 27 of 28 files; a new hosted test needs a new companion; the hosted-test list is in the docstring of `🧷️companions/🍇️tests-07/🦀️.rs`.

## 12. Residual risks

* the whole result is static plus a rustc resolution oracle; type-checking, trait-solver and borrow-checker behaviour of the new `ClosePeers` generics is untested by a real compile;
* the oracle compares resolution errors of a tree with neutralised externs, so a regression hidden behind the neutralised parts (external globs) would not show;
* `ReactorClosePeers` maps peer faults through `reactor_close_fault`: the mapping is covered by the order law test but not by a runtime test;
* the TS suites carry the pre-existing failures of section 8, so green-versus-red there says nothing about WP4.
