# 🧩️ Result PLUGIN-INTEGRATION-1: flip of the WP3, WP4 and WP5 plugin crates

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no git-modifying command. Plan: `PLAN-PLUGIN-CRATES.md` (Appendices H to K), `RESULT-PLUGIN-WP3.md` (section 8), `-WP4.md` (section 10), `-WP5.md` (sections 6 and 7), method of `RESULT-RENDERER-INTEGRATION-1.md`. Tools: `pi1-tools/`. Logs: `🗑️generated/pi1-*`. Pre-edit copies of every touched file: `🗑️generated/pi1-orig/`, list in `pi1-tools/touched.json` (202 existing files, 25 created at 16:00). `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## 0. Verdict

**24 plugin crates of WP3, WP4 and WP5 are flipped to real crates; `tool-run` and `time-travel` are blocked on pwp7.** No compiler ran: the launcher never reached rustc (section 8). All evidence is static plus the patched union differential (one rustc resolution pass over all crates, external crates neutralised).

| group | crates | applied | evidence |
|---|---|---|---|
| G1 to G5 | WP3: child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command | yes | crates-only oracle lib and test: 0 new signal rows; full oracle `cp2` to `cp8` |
| G6 | WP4: reactor-app, reactor-tasks (owns 7 root-block statics) | yes | crates-only lib/test: 0 new except the sandbox-neutralised `NoPluginApp` |
| G7 | WP4: reactor-lifecycle, runtime-instances, reactor-patches (lifecycle owns `REACTOR_CLOSES`, `REACTOR_CLOSE_CURSOR`; patches owns `PATCHES`) | yes | crates-only lib/test: 0 new |
| G8 | runtime-ops, extension (WP3), reactor-surfaces (owns the 3 cold-pair statics) | yes | crates-only lib/test: 0 new except the shared fixture `dsl` alias (sandbox) |
| G9 | reactor-core, reactor-jobs, runtime-exchange, reactor-turn, reactor-wit-bridge, reactor-context | yes | crates-only lib/test: 0 new; the hub unit `reactor` is now a composed module |
| G10 | WP5: time-travel-panel | yes | full oracle lib vs `cp7`: 0 new, 0 gone (the crates-only oracle is blind for this root, PI1-14) |
| WP5 rest | tool-run, time-travel | **no** | their roots carry pwp7's `instance_turns` mounts which must not be edited; section 8 |

Census (`readRustSourceClosureV1`, section 7): every flipped crate is inside the gate. `semio-framework-plugin` went from 225 files / 4,302,715 bytes / 56,472 lines to **128 files / 2,073,206 bytes / 27,863 lines**, still over the gate (worst ratio 16.41 to 7.91, binding axis bytes).

## 1. Found state (start, 2026-10-16 05:00)

* WP1 (9 crates) and WP2 (7 crates) were real crates; the WP3, WP4 and WP5 crates were staged inside the monolith (class B roots are old module files, class A roots are stubs, hub units mount the old files).
* Launcher at the start: `bun repo cargo ... check -p semio-framework-plugin-identity --lib` stopped in `cargo fetch --locked` (stale root `Cargo.lock`), then (`RESULT-INFRA-UNBLOCK-2.md`) an orphan old-format queue claim. Later attempts, each under the `🗑️generated/cargo-mutex.lock` mkdir mutex (acquired and released every time, no stale owner met): `Insufficient physical compiler initial headroom` from 07:00 to 12:00, and at 15:45 a failure inside the launcher itself, `Exact canonical persistent Native publication required` (stack in `🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`, log `pi1-cargo-final-1.log`). No compiler ran in this pass.
* Earlier `pi1-*` logs (03:09 to 04:39: `pi1-b1-*`, `pi1-fb-b1`, `pi1-lock-1`, `pi1-uo2-*`) belong to an earlier run with the same prefix whose tools folder did not exist; the repository carried none of its edits, nothing was recovered or rolled back.
* The coordinator's note of 13:00 (SINGLE-MOUNT-AND-GATE moved the context vocabulary into `guest-contract`, edited the `reactor-context` root and manifest and the `semio-framework-plugin` manifest) was absorbed: the new `reactor-context` root (`pub use guest_contract::original_actor_context::*` plus the wasm `exports` mount) and its feature forward were kept; I only appended feature forwards to the facade manifest list.

## 2. Method (tools in `pi1-tools/`)

* `modnode.py`, `rsparse.py`, `world.py`, `layout.py`, `translate.py`, `flipcore.py`, `groupdrv.py`, `reloc.py`, `widen.py`, `manifest.py`: the planner. Per group it rewrites every file of the group (explicit imports instead of globs, foreign mounts promoted or removed, crate roots assembled, cfg(test) units that name higher layers or facade fixtures moved verbatim into facade test hosts, name-keyed visibility widening, manifests). Specs `g1spec.py` to `g10spec.py` are one-shot (the planner reads the live tree). `statics.py` moves persistent-local statics to their owner crate.
* `uo.py`: union oracle (rustc `--emit=metadata` over one sandbox crate). Crates-only mode (signal rows against a stored baseline), full-monolith mode (`--vs-full`, or A against the restored original B), configs `lib`, `test`, `lib-features` and the new `wasmp2` (`--target wasm32-unknown-unknown --cfg target_env="p2"` with the built-in cfg override) for the wasm-gated code.
* Static scans added in this pass: `cfgtest.py` (definitions under exactly `#[cfg(test)]` that files outside the crate name), `supercheck.py` (`pub(super)` / `super::` climbing above a crate root), `postapply.py` (path literals, regions, rustfmt parse, TOML, dependency idents, single owner, orphan rule), a duplicate-mount scan over all crate roots and the facade, `ts_gate.py` / `ts_diff.py` (144 TS tests that read plugin sources).

## 3. What each group changed

* **G1 to G5**: crate roots filled (class A) or module files kept as roots (class B), package placeholder stubs deleted, hub units replaced by `extern crate`/`pub use` or composed modules (`child_content`, `tool_jobs`, `reserved_jobs`, `command_history`, `typed_operation`), test hosts under `🪝️mounts/👥️wp3-owners/🧪️test-hosts/`, manifests and facade feature forwards.
* **G6**: `reactor-app` (class B root `🔌️app/🦀️.rs`, `pub use app_runtime_helpers::*` so `plugin_internal_fault` is a root name) and `reactor-tasks` (class A). The seven D8 statics `REGISTRY`, `REACTOR_EXECUTOR`, `TASK_EXECUTOR`, `INSTANCE_METADATA`, `TASK_RECORDS`, `TASK_RESUMES`, `JOB_RENDER_BINDINGS` left the reactor root block: the owner declares them `pub static` inside `semio_framework_plugin_abi::component_persistent_local!`, the root re-imports them with one `pub(crate) use`, the four registry types behind them became `pub` (`#[doc(hidden)]`).
* **G7**: `reactor-lifecycle` (root `📨️pending/🦀️.rs`; owns `REACTOR_CLOSES`, `REACTOR_CLOSE_CURSOR` and, by ownership correction, `PLUG/🚪️lifetime/`), `runtime-instances`, `reactor-patches` (owns `PATCHES`; its unit tests stay in the crate because production code calls hooks of the test module under `cfg(test)`; the fixture caller is mounted locally).
* **G8**: `runtime-ops`, `extension` (wired the WP2 declaration-tree lift as `lifted_tree`), `reactor-surfaces` (cold-pair statics).
* **G9**: `reactor-core` (root `⚛️reactor/🦀️.rs`; statics `ARMED_TIMERS`, `PACKAGE_ASSETS`, `PRESENCE` are `pub`), `reactor-jobs`, `runtime-exchange`, `reactor-turn`, `reactor-wit-bridge`, `reactor-context`. The wit-bridge and context exports are wasm-only: `crate::component::wasip2::*` resolves to `semio_framework_plugin_abi::wasip2::*`. The hub unit `reactor` of `🪝️mounts/⚛️wp4-reactor/🦀️.rs` became `pub mod reactor { pub use core::*; pub use context as original_actor_context; pub use jobs; pub use turn; pub use turn::turn_close_step::*; ... }`, so the higher crates stay reachable under the old module names without a back edge in the crates.
* **G10**: `time-travel-panel` (class A root `⏪️time-travel/🪟️panel/🦀️.rs`: pointer, sections and the WP3 lift `ui_history_panel`); the time-travel root lost the `panel_pointer` / `panel_sections` mounts.

## 4. Verification evidence

* **Union differential, crates-only** (per group, lib and test): table in section 0. The remaining signal rows are sandbox noise classes the monolith baseline carries too: neutralised external globs, E0408 constants, stale `include_str!` paths of other teams (section 8), the macro-generated `NoPluginApp`, and the shared fixture `🧪️testing/🎟️caller/🦀️.rs` mounted by three modules (the sandbox rewrites the file for one owner only).
* **Full-monolith differential against `g1base`**: `cp3` (after G5) lib new 0; `cp4` (G6) lib new 2 (`NoPluginApp`); `cp5` (G7) lib new 4 (`NoPluginApp` plus two bare `use <crate> as alias` forms the sandbox did not rewrite, fixed in `uo.py`); `cp6` (G9) lib new 5; `cp7` (all repairs, lib) **new 2 (`NoPluginApp`)**; `g10f` (G10 overlay against `cp7`, lib) new 0, gone 0. The test config of `cp5` and `cp6` found 31 and 21 real rows (test-host paths, private test modules, missing `use` heads of units, hub re-export visibility); all repaired, see PI1-2, PI1-3, PI1-12 and section 5. `cp8` (lib, after G10) new 2 (`NoPluginApp`); `cp9` (test config, all repairs, 3,872 s; `cpt` with features: same four sandbox rows) new 5, all foreign or sandbox: `NoPluginApp` x2, the shared fixture `dsl` alias x2, one stale `include_str!` of `🧪️tests/🌲️fixture-projection` (path into the vanished `🖱️ui` module). `cpw` against the restored original: `lib-features` new 3 (`NoPluginApp` x2 and `media_operation_refusal::tests`, repaired, rerun `cpw2`: new 0), `wasmp2` new 7 (`NoPluginApp` x2, three `wasip2` imports that the sandbox cannot resolve in either tree, and two real `persistent_local` private-path rows, repaired, rerun `cpw3`: new 0). Section 9.
* **Static gates**: `postapply.py` clean for all 24 crates (the region report on `💼️jobs/🦀️.rs` is that file's own unnamed `//#endregion` style, identical before); `supercheck.py`: 0; duplicate-mount scan: only the shared fixture, by design; orphan rule: 0 findings; `cfgtest.py`: see PI1-4.
* **TS gate** (`ts_gate.py`): before 258 pass / 165 fail; after the repairs 265 pass / 172 fail with 9 new failing tests, none mine: 7 in `📇️registry/🔎️discovery`, `plugin identity` and `registered owned serializers` (reads `🚪️io/📤️serialization/🦀️.rs`, removed by another team). The 20 failures my moves caused were source-text readers of moved code and are repaired (list below).

## 5. Plan defects found and handled (numbered PI1-n)

| id | defect | handling |
|---|---|---|
| PI1-1 | `component_persistent_local!` has no visibility parameter, so a static cannot be shared across crates | macro in `abi` gets `$vis:vis` (3 lines, no caller changes); owners declare `pub static`; one `pub(crate) use` per owner crate in the reactor root |
| PI1-2 | `#[path]` inside an inline wrapper module resolves against `dir/<module name>/`, which does not exist (rustc `couldn't read h/sub/unit/../../x.rs`, checked on a toy) | test-host units that mount files by `#[path]` are out-of-line files `🧱️unit-N/🦀️.rs`; generator and the three applied hosts fixed |
| PI1-3 | relocated test units kept relative `include_str!`/`include_bytes!` literals of the old file | `rebase_paths` covers all three macros; the retained-command host repaired (17 literals) |
| PI1-4 | **cfg(test) leak**: definitions under exactly `#[cfg(test)]` that facade tests use vanish when the crate is built as a dependency; the oracle compiles one crate and cannot see it | 27 definitions became `#[cfg(any(test, feature = "artifact-app-testing"))]` and `pub` (`insert`, `is_empty`, `cause`, `phases`, `HISTORY_ROWS_BUILT`, the `test_*` helpers, `UNIT`, ...); `artifact-app-testing` features added where missing (reactor-lifecycle, reactor-patches, reactor-tasks, runtime-instances, runtime-ops) and forwarded by the facade. Not converted, judged name collisions: reactor-surfaces `lifetime`, `request_close`, `retained_bytes`; patches `begin`, `commit` |
| PI1-5 | `pub(super)` at the top of a crate root is invalid in a real crate (the oracle maps it into the facade where it still resolves) | `pub(crate)`; the planner rewrites roots now; the pending root fixed |
| PI1-6 | `PLUG/🚪️lifetime/🦀️.rs` is included textually into the lifecycle registry piece through `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🚪️lifetime/🦀️.rs"))`, which changes meaning inside the new crate; the directory map said runtime-instances | file-relative literal; the folder is owned by reactor-lifecycle (`crate_map.json`) |
| PI1-7 | `🏃️runtime/🦀️.rs` (residual runtime root) was owned by runtime-ops by the longest-prefix rule and would have been rewritten into the crate | owned by the facade (composition module); same for the tasks test `🔬️job-render-binding` |
| PI1-8 | `🧪️testing/🎟️caller/🦀️.rs` (`OriginalFixtureCaller`) is mounted by patches' unit tests, ops' custody tests and the facade `app_testing`; patches' tests call hooks of their own test module (`tests::after_producer_step`), so they cannot move to a host | a shared fixture: local `#[path]` mounts in the crates that need it (as the monolith already did in four places), exempt from relocation and from the owner map; its dependencies count as dev-dependencies; `dsl` alias added under `cfg(test)` |
| PI1-9 | the driver deleted a class-B package stub only when the facade manifest changed and only for the last crate | seven leftover placeholder stubs deleted (child, composition-publication, document-media, job, command, retained-command, reactor-app) |
| PI1-10 | explicit private imports regenerated from `use super::*` shadow a module's own `pub use ...::*` re-exports (`💡️inference/🦀️.rs`: `ArtifactInferenceService*`) | the five shadowing imports removed |
| PI1-11 | the WP2 lift `🧩️extension/🛬️lifted/🌳️tree/🦀️.rs` (preflight/commit of declarations) was never wired | `pub mod lifted_tree; pub use` in the extension root |
| PI1-12 | duplicated test unit: `🌱️operation/🧪️tests` is mounted by its own parent and was also emitted as job unit 5; units that name a module alias only in a `use` (`use super::turn::..`) got no header import | unit removed; headers scan `use` heads; three headers repaired by hand |
| PI1-13 | planner and world defects (all fixed in `pi1-tools/`): second mount of one file produced an empty node; cached resolution `segs` accumulated (`X::admit::admit...`); lookup cache was poisoned by cycle truncation; a macro and a module of the same name collided (`component_persistent_local`); `macro_rules!` was never parsed as an item; macro statics had no visibility; sandbox did not map `use crate as x;` and `use <crate> as x;` | see `modnode.py`, `world.py`, `translate.py`, `flipcore.py`, `reloc.py`, `uo.py` |
| PI1-14 | the crates-only oracle goes blind for `time-travel-panel`: with one item in the lift file, rustc stops reporting body resolution errors of the whole tree (7,654 errors become 1,483; reproduced with minimal overlays and by a manual `rustc -Z time-passes`, where the pass `resolve_report_errors` is missing) | G10 was judged by the full oracle (0 new) |

### TS readers repaired (consumer edits outside the crates)

`⚛️reactor/🚪️lifetime/🧪️tests/🤝️close-peers` (`pub trait ClosePeers`, fixture string of the impl path), `⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests` (`reactorSource()` reads the core, tasks, turn, lifecycle, patches and surfaces trees), `⚛️reactor/🏃️runtime/📖️source-text` (`readPluginRuntimeSource()` also reads the ops, exchange, instances and lifecycle crate trees), `⚛️reactor/🏃️runtime/🎞️command/📥️custody/🧪️tests`, `⚛️reactor/🔄️turn/📥️receiving/📦️completed/🧪️tests` (`pub fn`), `👷️job/🚪️session/🫴️refusal/🧪️tests`, `🧩️composition/🪪️metadata/🧪️tests/🔬️unit` (visibility), `👷️job/🚪️session/🎟️pending/🧪️tests`, and the wasm flow test `🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit` (`inference_gateway::wire_artifact_infer(`).

## 6. Edits outside the owner set (mechanical, recorded in `touched.json`)

* `🧬️schema/🔁️persistent-local/🦀️.rs` (abi crate): `$vis:vis` in the macro (PI1-1).
* `⏯️tool-run/🦀️.rs`: the `lifted_job` mount replaced by `pub use semio_framework_plugin_job::lifted_job::*;` (a double mount of one file). `⏪️time-travel/🦀️.rs`: the `lifted_command` mount likewise, `panel_pointer` / `panel_sections` re-pointed to the panel crate. The `instance_turns` mount and `pub(crate) use instance_turns::TimeTravelExtension` are untouched.
* `🧪️tests/🔬️tool-run/🦀️.rs`: `use crate::{RequestId, ViewWindowInstance}` became `semio_framework::{RequestId, ViewWindowInstance}` (the file sits in a facade host now).
* The TS files of the list above.
* Facade hubs `🪝️mounts/👥️wp3-owners`, `⚛️wp4-reactor`, `⏯️wp5-features` and `📦️packages/🦀️rust/Cargo.toml` (feature forwards only).

## 7. Census

Gate: 32 files, 262,144 bytes, 4,000 active lines per crate closure. After G10 (`pi1-census-g9.log`, native rows):

| crate | files | bytes | lines | worst ratio |
|---|---|---|---|---|
| child | 1 | 65,935 | 1,086 | 0.27 |
| command | 4 | 108,449 | 1,708 | 0.43 |
| composition-emission | 4 | 46,595 | 529 | 0.18 |
| composition-publication | 4 | 130,973 | 1,478 | 0.50 |
| document-media | 6 | 190,333 | 2,549 | 0.73 |
| extension | 8 | 155,217 | 1,994 | 0.59 |
| job | 7 | 159,246 | 2,344 | 0.61 |
| peer | 1 | 54,430 | 558 | 0.21 |
| publication-mounted | 10 | 154,856 | 1,600 | 0.59 |
| reactor-app | 2 | 56,014 | 628 | 0.21 |
| reactor-context | 1 | 704 | 11 | 0.03 (wasm-only exports, not counted natively) |
| reactor-core | 1 | 21,796 | 382 | 0.10 |
| reactor-jobs | 5 | 103,382 | 1,310 | 0.39 |
| reactor-lifecycle | 8 | 137,994 | 2,494 | 0.62 |
| reactor-patches | 1 | 78,589 | 1,289 | 0.32 |
| reactor-surfaces | 2 | 30,878 | 583 | 0.15 |
| reactor-tasks | 6 | 36,864 | 728 | 0.19 |
| reactor-turn | 6 | 217,415 | 2,688 | 0.83 |
| reactor-wit-bridge | 1 | 300 | 8 | 0.03 (wasm-only) |
| retained-command | 8 | 148,847 | 1,809 | 0.57 |
| runtime-exchange | 2 | 56,471 | 844 | 0.22 |
| runtime-instances | 6 | 62,924 | 749 | 0.24 |
| runtime-ops | 8 | 185,051 | 2,250 | 0.71 |
| **semio-framework-plugin** | **128** | **2,073,206** | **27,863** | **7.91** (bytes) |

The wasm-only files (`wit-bridge`, `context/exports`) are measured by WP4's seven-row census (`pwp4-census-final.log`, worst plugin crate row 0.83).

**Distance of the facade to compliance**: 2,073,206 bytes against 262,144 (7.9x), 128 files against 32 (4x), 27,863 lines against 4,000 (7x). What is left in the facade closure are the crates that are not flipped: `tool-run` and `time-travel` (WP5, blocked), the instance family `instance-*`, `time-travel-turns`, `tool-run-turns`, `instance-app`, `builder`, `testing-laws`, `testing-acceptance` (WP6 and WP7), the `🧪️tests` hosts and the hubs. The WP6 and WP7 crates are the remaining bulk.

## 8. Blockers, hand-offs, foreign errors

1. **tool-run and time-travel** (rank 41 and 42): their roots mount pwp7's `🧵️instance/` pieces (`instance_turns`), which must not be edited, and the flip would remove those mounts as foreign. Needed: a landing for `time-travel-turns` and `tool-run-turns` (rank 53 and 54) so the roots can drop the mounts in the same change. `mod instance_turns;` and `pub(crate) use instance_turns::TimeTravelExtension;` are the only blockers; the `lifted_job` and panel mounts are already re-pointed.
2. **Real compiler**: not reached. 24 crates changed their manifests (dependencies, `artifact-app-testing` features); `Cargo.lock` is stale for the new edges and has to be regenerated by the launcher. Command for the compile queue:  
   `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo cargo --store=shared --configuration=prototype --parallelism=4 -- check <-p semio-framework-plugin-X for each key of pi1-tools/flipped.json> --keep-going --message-format=short --lib`, then the same with `--tests`, with `--features artifact-app-testing` and with `--target wasm32-wasip2`.
3. **Unverifiable natively**: the wasm-gated bodies of `reactor-wit-bridge`, the `reactor-context` exports and the wasm statics (the `wasmp2` oracle checks resolution only); type-check and borrow-check behaviour of everything flipped (trait solving of the `ClosePeers` generics, the `pub static` statics over `ComponentPersistentLocal`) is untested by a real compile.
4. **Stale paths of other teams** that break the test build of the monolith independently of the flips: about 20 `include_str!` literals with 8 to 9 `../` into `🔨️modules/🖱️ui/...` and `🎭️actor/...` that no longer exist (7 in the patches unit tests; the one in `📨️pending/🧪️tests/🔬️instance-lifetime-patch-close` I repaired because a correct twin stood two lines above), `🚪️io/📤️serialization/🦀️.rs` (referenced by a lifecycle TS test, removed by another team), 7 registry discovery TS tests, 1 plugin identity TS test.
5. **Lifts not wired by anyone**: `reactor-surfaces` (`impl Default for SurfaceContexts`, PD8) and `command` (`Inputs`, PD9) are withdrawn per WP4; `🧵️instance/🛬️lifted/🌳️tree` and `🏗️builder/🔌️program/🛬️lifted/*` belong to WP7.
6. `Cargo.lock` and `📋️build-owners.json` were not touched.

## 9. Late checkpoints and what they caught

* `cp9` (test config) is the first run that goes through name resolution of the test tree (earlier runs stopped at the first `couldn't find file`): it exposed `include!(concat!(env!("CARGO_MANIFEST_DIR"), ..))` inside the lifecycle crate. The planner had already shifted the literal by two `../` for the new manifest folder; the sandbox now substitutes the owner's manifest folder for `env!("CARGO_MANIFEST_DIR")`.
* `cpw` (`lib-features`, `wasmp2`) found two real defects that no native lib or test run can see: the repo's own `media_turn_laws` re-export (a feature-gated mount that the relocation had moved into a `cfg(test)`-only host, now `cfg(any(test, feature = "media-turn-testing"))`, re-exported from the host unit) and the translator emitting the private wrapper path `semio_framework_plugin_abi::persistent_local::owned_abi` for two wasm-only context exports (now `semio_framework_plugin_abi::owned_abi`; the translator prefers the crate-root re-export of a module).
* `cpt` (`test-features`, live tree A against the restored original B, 1,540 s / 1,412 s): signal A 100 against B 1,690, only-new 4: `NoPluginApp` x2 and the shared-fixture `dsl` alias x2 (sandbox classes). No real new row in the last, most complete configuration.
