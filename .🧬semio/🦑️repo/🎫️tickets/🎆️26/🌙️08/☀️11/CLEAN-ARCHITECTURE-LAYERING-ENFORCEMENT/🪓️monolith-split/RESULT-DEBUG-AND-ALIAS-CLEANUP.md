# 🧹 Result DEBUG-AND-ALIAS-CLEANUP (prefix `dac`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Run 2026-10-16 11:40 to 12:30 CEST.
Tools: `dac-tools/` (`reach.ts`, `alias_scan.py`, `alias_consumers.py`, `ta_plan.py`, `ta_apply.py`, `draw_apply.py`, `batch_c.py`, `oracle.py`, `classify.py`). Logs and tables: `🗑️generated/dac-*`.

## 0. Verdict

| Defect | Result |
| --- | --- |
| 1. `[DEBUG]` leftovers (107 unique lines, 49 files) | 8 temporary lines removed in 5 files (Rust only). 2 more files were cleaned by peers during the run. 113 occurrences in 42 files remain: 94 are deliberate diagnostics (listed in section 1.2 with the reason), 19 are test narration inside the running `pi1` hub (handed off, not touched). |
| 2. Old-name aliases (183 audited rows) | 58 alias lines removed or tidied (section 2.2). 120 lines kept and justified (b/c) or left to a running slice (x), or blocked (section 2.3 to 2.5). The 2 `.ts` rows are test text, not aliases. The other 3 rows were changed by peers meanwhile (2 guest-runtime lines, 1 deleted file). |
| 3. Paths over 240 bytes (11 paths) | **Premise false: no slice created them.** All 11 exist unchanged since 2026-09-09 to 2026-10-09. Of the 7,995 paths added by the last-day commits plus untracked files, 0 exceed 240 bytes. Nothing renamed (section 3). |

No compile verdict: the launcher refused at admission (`Insufficient physical compiler initial headroom`, 4,262,895,616 of 4,294,967,296 bytes; `🗑️generated/dac-cargo-probe-1.log`). Every claim below is static: the patched resolution differential (section 4), rustfmt parse checks and residual-token greps.

## 1. `[DEBUG]` leftovers

Method: read all 114 occurrences in context (`🗑️generated/dac-debug-now.txt`), attributed them with read-only `git log -S`, grepped every message stem for other readers (`dac-debug-refs.tsv`: tests and tools that quote a message), and read the RESULT files. No RESULT file claims any of the removed lines as deliberate. All first appeared in auto-commits of the shared account: DAG test helper `5487a5cce52` (10-09), store lifted `2604f70cac1` (10-08), time-travel `2604f70cac1` and `b4bf1a54b37` (10-08, 10-13), fault `cede6fc227b` (10-09), winit `7af1259dba8` (10-14). The audit counted them as "added last day" because `1bae6115bc5` re-added the moved copies.

### 1.1 Removed (8 lines, 5 files)

| File | Line | Why temporary |
| --- | --- | --- |
| `os/♾️infinite/🗿️artifacts/🕸️dag/🦀️.rs` | `println!("[DEBUG] Exact original DAG store close turns=…")` in `close_dag_test_store` (loop variable became `_`) | test-helper narration |
| `os/🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs` | whole `#[cfg(debug_assertions)] { static TURNS … eprintln!("[DEBUG] deferred reprojection turn=…") }` block | production-reachable `eprintln!` scaffold with a power-of-two throttle: a debugging probe |
| `os/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs` | `#[cfg(debug_assertions)] eprintln!("[DEBUG] deferred reprojection refused …")` | same, error arm |
| same | `#[cfg(debug_assertions)]` block `reprojection driver cleanup turn=…` (the `if step.progress()!=default` shell is gone, `if let Some(step)` became `.is_some()`) | same |
| `os/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/⚠️fault/🦀️.rs` | `eprintln!("[DEBUG] Completion original fault has zero-heap carrier birth …")` | test narration |
| `os/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` | 3 unconditional `log_debug(&format!("[DEBUG] Native frame original …"/"Native root admission …"/"Native host retirement admission …"))` calls (a stderr line per redraw) | hot-path per-frame stderr output; the receipts themselves are still used by the code |

Removed by peers while this ran (not mine): `os/🏪️store/🦀️.rs` and `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`.

### 1.2 Kept (deliberate diagnostics), 113 occurrences in 42 files

Rule applied: keep a `[DEBUG] ` line when it is (a) env- or flag-gated, (b) a receipt or progress line of an expensive operation in a build, cache, bootstrap or launcher tool (AGENTS: progress for expensive operations), or (c) quoted by a test. The launcher's own `[DEBUG] Physical compiler admission refused` / `initial headroom admitted` pair (file `…/🏗️compiler/📏️capacity/🟦️.ts`, line 23) is the example the brief names, and it was the log that told this run why the compile was refused. The 94 kept lines in tooling are the same family: 335 such lines already existed in `repo/` at 2026-10-14 02:26, 119 files. Renaming the prefix is a separate change (peers parse these lines); it was not done here.

Gated (23 of them): `SEMIO_BUILD_PROFILE_INPUTS`, `SEMIO_BUILD_TRACE_AUTHORITY`, `SEMIO_CARGO_PREPARATION_TIMING`, `NX_VERBOSE_LOGGING`, the `profile`/`metrics` flags, `runtimeDiagnosticsEnabled()` and `progress` callbacks. Quoted by a test: `Actual original physical compiler locks …` and `Inactive Cargo retirement recensus refusal` (`pruning/budget/📜️history/🧪️tests/📦️compact/🟦️.ts`). Quoted by a ticket script or note only: `invalidated completed graph`, `cargo-preparation`, `plugin-runner-oracle cases=`.

Per file (count, gated, category):

| File (shortened) | Occ. | Gated | Category |
| --- | ---: | ---: | --- |
| `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏗️build/🟦️.ts` | 1 | 0 | deliberate: stdio composition build receipt |
| `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts` | 1 | 0 | deliberate: stdio composition build receipt |
| `🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` | 1 | 1 | deliberate: build-tool receipt/progress of an expensive operation |
| `🖼️canvas/📦️packages/🦀️rust/📜️script.ts` | 1 | 0 | deliberate: asset-staging status of the package script |
| `os/♾️infinite/📦️packages/🦀️rust/📜️script.ts` | 1 | 0 | deliberate: asset-staging status of the package script |
| `os/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧱️instance-layers/🟦️.tsx` | 1 | 1 | deliberate: opt-in runtime diagnostics gate |
| `os/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` | 14 | 5 | deliberate: test-lane progress of the plugin package script |
| `os/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts` | 4 | 1 | deliberate: test-lane progress of the plugin package script |
| `os/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs` | 19 | 0 | x (pi1 hub, test narration; hand-off) |
| `repo/library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/♻️recovery/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts` | 1 | 1 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/📜️script.ts` | 8 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📤️publisher/🟦️.ts` | 2 | 2 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts` | 3 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📊️material/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts` | 4 | 2 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/📥️publication/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts` | 3 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🦀️cargo/📦️units/📜️script.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🟦️.ts` | 1 | 1 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts` | 6 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏗️build/📖️storybook/🏃️execution/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏗️build/📥️inputs/🎯️selected/🦀️rust/🟦️.ts` | 2 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🗑️reset/🟦️.ts` | 1 | 0 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏗️build/🕸️graph/🟦️.ts` | 4 | 4 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🏗️build/🕸️graph/🦀️rust/🟦️.ts` | 5 | 4 | deliberate: build-tool receipt/progress of an expensive operation |
| `repo/library/🗂️workspaces/🦀️cargo/🟦️.ts` | 2 | 1 | deliberate: build-tool receipt/progress of an expensive operation |

`os/…retained-command` is the `pi1` hub file (`🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs`, edited at 11:14, 18 lines, 19 occurrences). Its `eprintln!("[DEBUG] held …")` calls are test narration and temporary. The same messages are duplicated in `🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`. Hand-off to `pi1`: delete them when the hub is free (section 5).

## 2. Old-name aliases

### 2.1 Method

1. `dac-tools/reach.ts` walks every manifest root and every `#[path]`, `mod` and `include!` mount: 30,627 Rust files, owner crates per file (`🗑️generated/dac-reach.json`).
2. `alias_consumers.py` counts, per alias, in-crate path uses and external uses (`<lib>::<alias>`, use groups). Output `dac-alias-consumers.txt`. Glob re-exports (`pub use <crate>::*`) were checked separately because they leak aliases to importers (found: `actor`, `os_flow`, `artifact_flow_flow`).
3. Rule: **(a)** crate-private aliases and public aliases without consumers are re-pointed to the real crate path and deleted; **(b)** a public alias that consumers use is kept, because re-pointing means new direct dependencies in every consumer manifest plus `Cargo.lock` updates, which is a dependency-rewiring slice, not an alias cleanup; **(c)** macro/path-root requirement; **(x)** owned by a running slice.
4. Verification after each batch: no residual token (`grep`), `rustfmt` parse check of every changed file (`--config skip_children=true`), and the scoped patched differential `dac-tools/oracle.py` (section 4). Compile: refused (headroom).

### 2.2 Done (58 lines)

| Batch | What | Lines |
| --- | --- | ---: |
| TA | `pub(crate) use semio_framework_trace_memory_testing as test_allocation;` was the per-crate nickname of the old `mod test_allocation` (class a). 33 definitions deleted (26 of them in the audited rows, 7 plain `use` forms of the same alias), 114 files rewritten, 665 tokens re-pointed: `crate::test_allocation::` and `super::`/bare `test_allocation::` became `semio_framework_trace_memory_testing::`; also `semio_framework_os_kernel::test_allocation::` in `🗣️dsl/🧪️tests/🔬️unit`; the `use …::test_allocation;` import of the ephemeral store test and the preceding `#[cfg(test)]` attribute lines went with it. Every owner crate of every rewritten file had the alias, hence the dependency. | 26 |
| DRAW | `pub(crate)` aliases of `semio_framework_ui_wgpu_render` (`draw`, `draw_types`, `prepared`, `shaders`, `theme`, `raster_ownership`): 42 qualified uses in 6 files re-pointed, `pub use draw::{…}` re-pointed, aliases deleted. | 6 |
| ACTOR | `actor-vocabulary` (`pack`, `job`, `retained_turn`; 330 + 7 + 1 uses incl. `📥️cold-pair`), `actor-job-publication` (`pack` unused, `job` 255 uses in root and tests), `actor-retained-turn` (`pack`, 48 `crate::pack::` uses). `semio_framework_actor` keeps its own explicit `pack/job/retained_turn/retained_context` aliases (class b), so its `vocabulary::*` and `job_publication::*` globs lose nothing. | 6 |
| SCENE | `ui-scene` package `pack`: unused. | 1 |
| FLOW | `os-flow` facade: `host`, `drawing`, `vcs`, `wasm_session`, `extensions::wasm`: 0 consumers (greps of `flow::<name>`, `extensions::wasm`, the two `os_flow::*` glob importers). The doc line "re-exports the pre-split public API" was rewritten. `artifacts/flow` `retained::copy`: unused. | 6 |
| HOST | `os-host` `codec_abi`: unused. `semio_framework` facade `source_projection`: 0 consumers. | 2 |
| TIDY | `infinite` package: 11 distinct `pub use crate::X as X;` lines (44 sites in `board`, `directed`, `directed_dag`, `directed_normal`, `schema`) were self-named: `as X` removed, same semantics. | 11 |

### 2.3 Blocked (3 rows, class a)

`pub(crate) use … as test_allocation;` stays in the `pack` package root and its two test roots (`🎒️pack/📦️packages/🦀️rust/🦀️.rs`, `🧪️tests/🔬️record-laws`, `🧪️tests/🧮️record-schema-laws`), together with 7 users. The shared test file `🎒️pack/🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs` is also compiled by `semio_framework_os_pack`, which defines no alias and has no dependency on `semio-framework-trace-memory-testing`, so rewriting would need a dev-dependency and a lock update in `os_pack`. Left for a dependency slice (section 5). Also untouched in the same family: the real `mod test_allocation` of `mcp/foundation` and its users, `stdio` `dxf` (pch), the plugin-host test component (smg), the `os_vcs` users without a definition, and 5 files whose owner the mount walk cannot find (dead or inline-path mounts; none belongs to a crate that lost its alias).

### 2.4 Kept as (b) or (c): justification

(b) means the alias name is the facade's public API: consumers use it today, or an accepted plan fixes it (PLAN-OS-KERNEL-CRATES section 6.4 "Public paths stay", D5; 2,857 consumer files name the kernel). Counts are `ext` (uses outside the crate) / `in` (word hits inside).

| File (shortened) | Aliases | Class | Reason |
| --- | --- | --- | --- |
| `framework/📦️packages/🦀️rust/🦀️.rs` | `ui`, `abi`, `action_bus`, `interaction`, `onboarding`, `history_edit`, `tool_run` | b | framework facade; ext 640, 48, 328, 10, 210, 48, 4. Its doc says "explicit re-exports under the module paths the framework always exposed": re-pointing 1,300 uses needs new deps in every consumer (hand-off). |
| `◻️2d/🦀️.rs` | `booleans`, `compute`, `flatten`, `stroke`, `regions`, `retirement`, `text`, `trace` | b | 2d facade, ext 19/367/5/4/8/4/16/5. |
| `🎒️pack/🌱️value`, `📐️format` | `EncodeOptions`, `DecodeOptions`, `VerificationLevel` | b | type renames of the real types, ext 4/3/7. |
| `🎒️pack/📦️packages` | `codec`, `value` | b | ext 13, 303. |
| `🎭️actor/🦀️.rs` | `pack`, `retained_context`, `retained_turn` | b | ext 96, 34, 15; `job` is also exported by the same root. |
| `📡️replication/📦️packages`, `🎮️mutation/📨️messages` | `crypto`, `value`, `prepared_retirement`, one rename list | b | ext 2, 223, 1. |
| `🖱️ui/🎯️targets/🧊️wgpu/…/📥️platform` | `TransferredCanvas`, `NativeWindow` | b | external types behind a repo name (AGENTS: external libraries behind an interface). |
| `🗺️surface/📦️packages` | `node_graph`, `paint`, `terrain`, `tiled_map` | b | ext 2, 8, 7, 34. |
| `🐾️pets/…/✨️effects` | `unit` | b | local function binding used by the effects code (38 hits), not a module path. |
| `os/📦️packages/…/📥️mounts/🌱️wp1-leaves` | `os_dsl`, `os_extension`, `list_delta`, `cli` (two cfg variants), `os_plugin_module_schema`, `os_semio` | b | kernel hub of the carved WP1 crates (plan section 6.4); ext 346 (`os_dsl`), 3 (`os_semio`), 0 for the rest but plan-fixed. |
| `os/♾️infinite/📦️packages` | `graph`, `canvas`, `BOARD_CAMERA_ZOOM_MAX/MIN` | b | `graph` ext 1, 18 in-file; renames of canvas constants to the board vocabulary. |
| `os/🌊️flow/📦️packages` | `geometry`, `mesh` | b | ext 45, 18 (plugins and framework). |
| `os/🌊️flow/🗿️artifacts/🌊️flow/🧱️model` | `neural` | b | ext 1 (generation2d plugin). |
| `os/🏪️store/📜️space-history/…/📸️snapshot` | `decode_native_cst` | b | new public name of `decode_with`, not an old path; 0 uses. |
| `os/🏪️store/🦀️.rs` | `PackDecodeOptions`, `PackEncodeOptions`, `PackVerificationLevel`, `semio_format`, `backbone`, `artifact_child`, `codec_registry`, `composition_coordinator`, `document_store_owners`, `ledger`, `snapshot_clone_preparation`, `space_history`, `test_support` | b | store hub: plan section 6.4 maps `os_store::<module>` to these crates; `ledger` in 105, `test_support` in 350. `composition_coordinator` has 0 uses but is in the plan table. |
| `os/📡️spr/🦀️.rs` | `channel`, `command`, `history` | b | kernel hub, ext 1, 61, 16. |
| `…/EngineCanvas/…/📦️prepared-engine-canvas`, `IconRenderHost/…/📤️export` | `EngineGpuBuildPhase`, `asset` | b | type rename; `asset` is a pub module nickname used 53 times in-crate (ext 0). Candidate for a later narrowing to private. |
| `os/🔌️plugin/🖥️host/🔗️wit-bindings` | `wit_jobs`, `wit_reactor`, `wit_capabilities`, `wit_effects`, `wit_events`, `wit_host_async`, `wit_lifetime`, `wit` | b | disambiguating names for generated WIT modules, ext 1 to 3 each. Not an old name. |
| `os/🚪️io/🦀️.rs` | `io_schema` | b | kernel hub, 2 in-crate uses. |
| `os/🛢️db/⚙️engine/🎫️handle` | `HistoryEntry` | b | type rename. |
| `os/🧠️neural/⚙️engine` | `evaluation`, `registry` | b | `registry` 38 in-crate uses, `evaluation` ext 2. |
| `os/🖥️host/🦀️.rs`, `📦️packages` | `instance`, `media_export_raster`, `registry`, `workflow`, `space` | b | root `pub use instance::{…}` lists and the mounted tree use them (28, 1, 40, 117 hits); `workflow` ext 19, `space` ext 1. |
| `server/📦️packages` | `authority`, `contract`, `policy`, `storage`, `throttle` | b | ext 6, 21, 4, 32, 4. |
| `repo/⌨️cli/🦀️.rs`, `🧭️repo-cli` | `mcp_verb`, `verbs` | b | `mcp_verb` is called by the package's `mcp-main.rs` bin; `verbs` is the sibling module hub (83 uses). |
| `os/🏪️store/🚪️io/🦀️.rs` | `os_pack`, `__diagnostic`, `__pack_json`, `__value` | c | emitted by `$crate::…` macros (plan section 6.6). |
| `os/🔌️plugin/🪟️window/🦀️.rs`, `os/🪡️list-delta/🦀️.rs` | `__kernel`, `__pack_json`, `__dsl_record_derive`, `__mutation_diff`, `__value_derive` | c | hidden macro-expansion re-exports. |

### 2.5 Left alone (x)

| File | Rows | Owner |
| --- | ---: | --- |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/**` (root, `🧾️payload-wire`, `🪡️list-delta`), `…/🖋️dxf/🦀️.rs` | 14 | `pch`. By the numbers they are facade roots: `testing` ext 1,797, `part21` 199, `media_export` 21, `list_delta` 12, `kernel` 7; `patch` has 0 uses. |
| `os/🔌️plugin/🪝️mounts/👥️wp3-owners/🦀️.rs` (11 `pub(crate) use … as …` renames of `tool_jobs_*` and carved crates) and `🌱️wp1-foundation/🧪️test-hosts/📬️window-mutation` (1) | 12 | `pi1` |
| `replication/⚠️error/🧪️tests/🟦️.ts`, `interactivity-audit/🖥️mounted/🟦️.ts` | 2 | not aliases: a test that pins the glue text, and a law that asserts the absence of `pub use ui_contract::BuiltNode as TreeNode`. |

Also changed by peers during the run: two `guest-runtime` lines and one deleted `➕️normal` file.

## 3. Long paths

| Check | Result |
| --- | --- |
| The 11 flagged paths (`🗑️generated/dac-long-paths.txt`, 245 to 258 bytes) | each existed, byte for byte, at commit `0c3e78944af` (2026-10-14) and nine of them at `25bb77059d6` (2026-10-01). First added 2026-09-09 to 2026-10-09 (`git log --diff-filter=A`). No slice of this fleet created them. |
| Paths added by `1bae6115bc5`, `f856dff0813`, `36fbe9da786` plus untracked files (7,995 paths, `dac-long-added.txt`) | **0 over 240 bytes.** |
| Whole tree (tracked plus untracked, `dac-long-all.txt`) | 11,373 paths over 240 bytes (3,484 under `bim`, 2,376 `energy`, 1,691 `architect`, …), maximum 314 bytes. By code points the maximum is 207, so the AGENTS limit of 256 characters holds everywhere. |
| What a rename could reach | The leaf folders (`🔬️window-config-ownership`, `🪪️document-contract`, `🔬️window-ownership`) are plugin-wide conventions (109 and 52 repo references), and the shortest meaningful names save 7 to 14 bytes: 258 becomes 244, still over 240. Reaching 240 would need renaming `🧵️serializers` or the `🏅️standards/🔖️1/🪆️subsets` spine under hundreds of manifests. |

Decision: no rename. A byte-limit of 240 on the plugin standards tree is a taxonomy decision (it affects 11,373 existing paths), not a cleanup of slice-made folders. Hand-off in section 5.

## 4. Verification (every command and its real result)

| Command | Result |
| --- | --- |
| `bun repo cargo --store=shared --configuration=prototype … check -p semio-framework-trace-memory-testing --lib … --manifest-path 🧰️framework/Cargo.toml` (mutex held, 11:50) | refused: `Insufficient physical compiler initial headroom` (`availableBytes` 4,262,895,616, required 4,294,967,296). `dac-cargo-probe-1.log`. |
| same launcher, `-p semio-framework-actor-vocabulary` (12:55) | refused again, `availableBytes` 4,262,871,040. `dac-cargo-probe-2.log`. Mutex released both times. |
| `dac-tools/oracle.py` on 36 lib roots, 2 configurations each (native, `--test`), `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=short`, sandbox in the scratchpad (HEAD copy versus working copy, same layout, globs from `semio_*` crates replaced by `const _`, `extern crate` removed). Roots: all 33 `test_allocation` crates, `ui_wgpu_render`, `actor_vocabulary`, `actor_job_publication`, `actor_retained_turn`, `ui_scene`, `os_flow`, `artifact_flow_flow`, `semio_framework_os`, `semio_framework`. Messages naming an unresolved `semio_*` crate are filtered on both sides (re-pointing makes 665 more of them, by design). | 72 of 72 runs: 0 new and 0 vanished non-external errors (`dac-oracle-run1.log`). Positive control: the first run reported `cannot find module or crate `pack`` and found the leftover `max_frames:pack::read_u32` in `actor-vocabulary`, which was fixed and re-run clean. Limit as in the oracle file: unresolved external crates can hide type errors, so this is a name-resolution proof, not a type-check. |
| `rustfmt +nightly-2026-07-20 --edition 2021 --unstable-features --config skip_children=true --check` on the 114 TA files, 6 draw files, 11 batch-C files, infinite root | no parse error except `💡️inference/🧪️tests/🔬️unit/🦀️.rs` (`comparison operators cannot be chained`); the same error exists in the HEAD copy of that file, so it is not from this change. |
| `git grep -w test_allocation` after the TA batch | 127 tokens left in 24 files, all in the blocked and untouched files of section 2.3; none in a crate that lost its alias. |
| `git grep -F 'semio_framework_trace_memory_testing as test_allocation'` | 5 definitions left (pack package ×3, `dxf`, plugin-host test component) from 38. |
| Dependency check | every owner manifest of every rewritten file lists `semio-framework-trace-memory-testing` (`ta_apply.py` `NODEP` check: none). The actor, scene, flow and host re-points use crates the deleted alias already named. |

Not run: any cargo check or test, the full-monolith differential (the brief allows at most one; the scoped runs above were enough and the load average stayed above 15).

## 5. Hand-offs and foreign findings

- **pi1**: delete the 18 test-narration `eprintln!("[DEBUG] held …")` lines in `🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs` and its duplicate under `🧪️tests/🔬️app-typed-command-full-operation`. The 11 `pub(crate) use … as …` lines in `wp3-owners/🦀️.rs` and the one in `window-mutation` are class (a) once you are done with the hub.
- **pch**: `stdio` contract aliases (14 rows) and the `dxf` `test_allocation` alias.
- **Dependency slice** (needs `Cargo.toml` and `Cargo.lock` edits, so not an alias cleanup): (1) `os_pack` dev-dependency on `semio-framework-trace-memory-testing` so the `pack` test group can lose its alias; (2) re-pointing the consumers of the facade aliases in section 2.4 (`semio_framework` 1,300 uses, `2d` 428, `os_flow::{geometry,mesh}` 63, `surface` 51, `server` 67, `semio_framework_actor` 145, `os_dsl` 346) to the carved crates; (3) the kernel hub aliases only if the plan decision "public paths stay" is reversed.
- **Candidates for narrowing `pub use` to `use`** (0 external consumers, many in-crate uses; not done because glob importers were not proven absent): `icon_export_asset as asset`, neural-engine `registry`, `os_host` `instance`/`registry`/`media_export_raster`, `os_io::io_schema`.
- **Foreign**: `💡️inference/🧪️tests/🔬️unit/🦀️.rs` has a parse error in HEAD; `🫴️receiving/🎟️turn/🧪️tests/🦀️.rs`, `🪶️sqlite-snapshot/🧪️tests/🫙️prefix/🦀️.rs` and `NodeGraph/📡️scene/🧪️tests/🦀️.rs` use `crate::test_allocation` in crates that define none (unmounted or broken); `RESULT-INFRA-UNBLOCK-2.md` said a `[DEBUG]` probe was "removed" (not re-checked here); the flow facade still carries other compatibility re-exports without `as` (`os_dsl`, `os_pack`, `os_spr`, `os_store`, `os_vcs`, `infinite`, `dag`, `canvas`, `neural`; comments say "Components still use former kernel path names"). They are outside the 183-row scope but are the same defect.
- Taxonomy: no folder created, no registration changed. `.vscode/launch.json` untouched (no new command).
- Files changed by this run: the 5 Rust files of section 1.1; the 114 files in `🗑️generated/dac-ta-changed.txt` (plus `ta_apply.py` list `dac-ta-apply.log`); the 6 files re-pointed by `draw_apply.py`; the 11 files of `batch_c.py` (plus the hand fix at `actor-vocabulary` line 344); `infinite/📦️packages/🦀️rust/🦀️.rs`. Every one was re-read from disk inside the script immediately before writing, with an equality assertion.
