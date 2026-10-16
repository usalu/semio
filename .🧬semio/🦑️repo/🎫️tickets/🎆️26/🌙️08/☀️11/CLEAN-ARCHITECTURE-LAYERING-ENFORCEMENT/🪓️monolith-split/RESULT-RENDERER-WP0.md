# 🧭️ Result RENDERER-WP0: skeletons, mount hubs and hand-off contract of the wgpu renderer partition

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-RENDERER-WGPU-CRATES.md` (updated in place; contract = section 10.4, Appendices F, G, H). Tools: `rwp0-tools/`. Logs: `🗑️generated/rwp0-*`. `T` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`, `R` = `T/🧊️renderer`.

## 0. Verdict

Phase 0 is done: the census was re-run and the plan reconciled, the crate root is split into five `include!`d mount hubs, 88 skeleton crates exist with manifests, owner rows, workspace membership, launch routes in both files and taxonomy names, and the hand-off contract is in the plan. The crate is still one compilation unit and resolution-equivalent: **the patched full-resolution differential shows no new error in any of the four configurations** (native lib, native `--cfg test`, `wasm32-unknown-unknown` lib and test). The launcher was down at the start (`Finite original maintenance authority required`) and worked at the end: the five closure-admitted leaf skeletons pass `cargo check`, the other 83 are refused by the gate through peer crates. Two defects of my own first cut were caught by the evidence and fixed (section 5).

## 1. What changed

1. **Census and reconciliation** (section 2): the live lib closure is 402 files / 5,966,814 B / 102,014 lines (plan: 146 / 5,861,374 / 101,831). The only structural drift is the icon embeds (+251 files); every other file moved by at most 170 bytes. The plan's tables are updated in place and marked `[WP0 change]` / `(was …)`.
2. **Mount hubs**: `R/🦀️.rs` lost 256 active lines (921 to 665); the 61 module mounts, the root-level test mounts and the 12 renderer partitions moved into `R/📥️mounts/{🌱️wp1-foundation,🎞️wp2-scenes,🧵️wp3-frame-core,🐚️wp4-shell-state,🎛️wp5-shell-views}/🦀️.rs`, included from the root; path literals rebased by `../../`; regions kept balanced (`🔖️OsHostDecomposition` repeated in the two hubs holding pieces). The library root lost the `native_runtime_modules` mount (now in the WP1 hub); `renderer_registration` stays (section 5).
3. **88 skeleton crates** under `T/<emoji>️<suffix>/`: 45 non-Shell, 24 Shell, 4 companions, 14 Shell test bins, `test-support`. Each: `Cargo.toml` (recipe: three boundary keys at the ceiling, `workspace` = repository root, `[lib] path = "../../🦀️.rs"`, first-party edges of the plan, external crates with the old specs and target tables, conditional dev-dependencies), `📋️project.json`, docstring library root, test stub mounted under `cfg(test)` (69 production crates).
4. **Registries**: 88 workspace members (root `Cargo.toml`, byte-sorted block 2 to 90), 88 owner rows, 528 launch routes in each of `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (unused order keys, identical in both), 88 names in `members-of-wgpu-target` of `🔣️taxonomy.json`, 69 transitional dependency lines in the top crate manifest.
5. **TS module-tree helper** follows `include!("literal")` (new `inspectRustModuleChildren`, included files stay in the includer's scope) with 3 fixture rows and a tree-sitter plus rustc-dep-info test.
6. **Plan**: header, section 0/1/2/6/9/10/12/13 numbers and notes, new section 10.4, Appendix A rows (17 re-measured, 6 added), Appendices F, G, H.

## 2. Census

Lib and test closures through `readRustSourceClosureV1` (`rwp0-tools/closure.ts`, `🗑️generated/rwp0-closure-*`). Before = first run of this session (16:5x) and the planner's values; after = 17:58 for lib rows.

| closure | plan (15:20) | before hubs (17:30) | after hubs (17:58) |
|---|---|---|---|
| lib native | 146 / 5,861,374 / 101,831 / 11 embedded | 397 / 5,967,294 / 102,107 / 261 | **402 / 5,966,814 / 102,014 / 261** |
| lib wasm32-unknown-unknown | 130 / 5,459,031 / 91,304 / 8 | 381 / 5,564,951 / 91,564 / 258 | 386 / 5,564,510 / 91,472 / 258 |
| lib wasm32-wasip2 | 2 / 10,079 / 221 | 2 / 10,079 / 221 | 2 / 9,943 / 219 |
| test native | 505 / 11,217,952 / 157,068 / 223 | 756 / 11,325,874 / 157,346 / 473 | not measurable (a peer removed `🖱️ui/…/🧊️gpu/🦀️.rs`; counter: `Missing Rust source input`) |
| test wasm32 | 375 / 9,383,249 / 123,640 / 155 | 626 / 9,489,058 / 123,883 / 405 | same |

Drift by file (`appendix_a_drift.py`, 138 rows): runtime-assets 64 to 88 lines (3,647 to 5,219 B), mailbox-impl +2 lines, IconRenderHost root 185 to 179, tutorial-runtime +1 line, 11 further files +15 to +383 bytes; one planner slip (Appendix A row 71 named `icon-host`, the folder is `icon-render`); **new in the closure**: `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs` (255 lines) with 250 `include_str!` SVGs (164,569 B). Consequence: `icon-host` is at 259 files against 28 (plan section 6.2, risk 7). Also not in the plan: crate counts (89 not 87, 14 Shell bins not 12), `semio_framework_canvas` / `os_renderer_camera` / `framework_editor` / `os_node_graph` / `surface` in the external lists of host-base, engine-*, scene-state.

**New crates** (`ONLY=… bun census.ts`, `🗑️generated/rwp0-census-after.log`): all 88 lib closures are 1 file, at most 342 bytes, at most 6 lines, no error, none over the limit. Only the five leaves `canvas2d-paint`, `hub-contracts`, `mailbox-core`, `shell-vocab`, `test-support` have a closure inside the gate; the other 83 sit behind the oversized frontier crates (`semio-framework`, `os-kernel`, `plugin`, `canvas`, …) as before. The top crate row: 402 files / 5,966,814 B / 102,014 lines, over the limit as before, 8 oversized dependencies.

## 3. Crate graph

88 nodes, 879 first-party edges, longest chain 22 crates (`contracts` to a Shell test bin); most depended on: `contracts` 71, `host-base` 63, `shell-vocab` 36, `shell-async` 36. WP split: WP1 18, WP2 16, WP3 15, WP4 21, WP5 17, plus `test-support` held by WP0 until WP3 fills it. The 6.1 `deps` column of the plan is consistent with the rank order of 4.3 (no violation, no unknown name). The Shell tier-1 edges come from the field-touch evidence of the live sources (`shelledges.py`: 619 methods scanned, 13 components, every edge points down the plan's rank, which confirms the order of 7.3): 104 edges. Full table: plan Appendix F.

## 4. Files

Created: `R/📥️mounts/*/🦀️.rs` (5); per crate `T/<folder>/{🦀️.rs, 📦️packages/🦀️rust/Cargo.toml, 📦️packages/🦀️rust/📋️project.json}` (264) and `🧪️tests/🦀️.rs` (69) = 333 files; `rwp0-tools/*` (this ticket); this file; `🗑️generated/rwp0-*`.
Edited: `R/🦀️.rs`; `PKG/📚️library/🦀️.rs`; `PKG/Cargo.toml` (+69 lines); root `Cargo.toml` (+88 members); `📋️build-owners.json` (+88 rows); `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (+5,808 lines each); `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (+88 names); `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/{🟦️.ts, 🧪️tests/🟦️.ts, 🧪️tests/🧫️fixtures/🔣️.json}`; `PLAN-RENDERER-WGPU-CRATES.md`. Longest new path: 214 bytes (manifest); longest possible lifted path 215 bytes.
`Cargo.lock`: all 88 packages were already present (a peer's launcher run refreshed the lock after my registry edits); I ran `bun repo cargo … -- update --workspace --offline` twice at the end, which only added other peers' new crates (`mesh-component-references`, `ui-render-device`, `ui-render-prepared-shaders`, `ui-wgpu-device`, then `ui-wgpu-draw-execution`, `-mesh`, `-pipelines`, `-raster`). Not touched: `🧑‍💻dev/📤️distribution/🧾️manifest.json`, any element source file, any peer-owned crate.

## 5. Defects found by the evidence (fixed)

1. `renderer_registration` moved into the WP1 hub: the first differential run reported **73 new errors** `macro-expanded macro_export macros from the current crate cannot be referred to by absolute paths` (`crate::action_args_json!` in Shell, Scenes, EngineCanvas). Cause: a `#[macro_export]` macro in a module declared inside an `include!`d file is macro-expanded. Reproduced on a 5-file toy (`scratchpad/macro-test`, error in the included-module shape, `7` when the module sits in the root). Fix: the mount is back in the library root at its original position; only `native_runtime_modules` (no macro) lives in the hub.
2. The TS helper did not follow `include!`: with the hubs, `listModuleTree` of the renderer root would have lost the 12 partitions (and the Rgba lint of the wgpu package script would have silently covered one file). Fix and tests in section 1 item 5; `tree_compare.ts`: the root tree is the old 17 files plus the 5 hubs, no file lost.

## 6. Verification (commands run, real results)

| check | result |
|---|---|
| `verify_static.ts` (88 manifests parsed by Bun TOML; name, id, role, boundary keys, workspace = repo root, lints, lib path; 1,722 path dependencies resolve and name the right package; dependency set equals the frozen table; DAG acyclic and strictly decreasing rank; members unique and byte-sorted; owner rows and project files; 6 launch routes x 88 x 2 files with exact command, group, key order and unused identical order keys; folder emoji RGI and unique among siblings; path lengths; top-crate block per table; companions not top dependencies) | `manifests 88 … failures 0`, `ALL STATIC CHECKS PASSED` |
| `verify_hub.py` | 68 mounts/includes old = 68 new: identical module name, absolute target file and cfg attributes; all targets and all `include_str!` targets exist; regions balanced in all 7 files; rustfmt parses all 7 (`PASSED 0`) |
| `hubsplit.py` losslessness | no lost non-blank line; the two library units resolve to the same absolute files |
| rustfmt (nightly, `skip_children`) on the 157 new `.rs` stubs | 0 parse failures, 0 formatting diffs |
| `rustc -Zunpretty=expanded` on the 88 library roots, lib and `--test` | 176 runs, 0 failures; 69 of 69 production crates expand the test stub only under `--test` |
| module-tree helper | `bun test` 8 pass, 0 fail, 69 expects (includes the rustc dep-info oracle for nested includes: rustc resolves `#[path]` against the included file's directory at depth 2, and the tree-sitter extraction of `include!` items) |
| 20 renderer TS readers, `bun test`, live tree vs baseline copy | identical pass/fail sets in all 20 (many baseline failures, unchanged) |
| 24 vitest readers with the wgpu config, live vs baseline | 24 of 24 identical (13 fail identically at baseline: 10 `ENOENT` on empty `E/🧫️fixtures/<name>` folders, 3 unresolved `@semio-tech/framework-os-node-graph-rs`) |
| registry guard assertion "every split source root reads as one text" (`policy_roots.ts`; the test file itself cannot load: root `📜️script.ts` lacks an export) | 27 roots, 0 bad; renderer root reads 22 files incl. the 5 hubs |
| taxonomy verifier per crate folder (`taxonomy-scope.ts`, ~10 s each) | before registration: `directory-kind-unresolved` per folder (same as every existing child of `🧊️wgpu`); after: only the existing `generator-activation-invalid` (frame-worker catalogue digest, pre-existing) for contracts, test-support, shell-tests-docs-3, browser-worker, kernel-runtime |
| wasm export attributes | the 9 `wasm_bindgen` occurrences of the root are unchanged; no hub holds one |
| toy proof of the carve protocol (`extern crate dep as dock;` in an included hub, `crate::dock::make()`, `super::dock::…`, `pub(crate) use dock::*`) | compiled and ran (`12`) |
| `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short --lib` | start (17:00): `error: Finite original maintenance authority required` (`retireInactiveCargoStoreOwnedV1`); end (19:20): launcher runs. `check -p …-mailbox-core -p …-canvas2d-paint -p …-test-support -p …-shell-vocab -p …-hub-contracts --lib`: `Finished prototype profile … in 1m 06s`, 0 errors from the new crates. The other 83 in batches of 6 with `--keep-going`: `error: Rust crate Source refused before compiler: semio-framework-os-kernel` / `semio-framework-os-infinite` (`Rust crate Source boundary exceeded before compiler byte allocation`), plus `cargo fetch --locked … cannot update the lock file` whenever a peer added a crate in between (log `🗑️generated/rwp0-cargo-check-all.log`) |
| `bun ./📜️script.ts verify taxonomy report --scope …` | cannot start: the root script lacks the export `INTERACTIVITY_AUDIT_WORLD3D_FILE` (peer churn); the normalization module was called directly instead |

**Full-resolution differential** (`oracle.py`: sandbox A = live engine copy, B = A with the hubs inlined back and the original library root; patches of `📓️RESOLUTION-ORACLE.md`: extern crates to empty modules, external globs to `const _: () = ();`; `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --error-format=json --emit=metadata -o /dev/null`; error key = code, message, span text, label, normalized file; renderer-root and hub files folded into one token):

| config | errors A (hubs) | errors B (baseline) | only in A | only in B | seconds A/B | log |
|---|---:|---:|---:|---:|---|---|
| native lib | 6,157 | 6,157 | **0** | 0 | 363 / 262 | `rwp0-oracle-hub4-lib.out` |
| native `--test` | 1,649 | 1,654 | **0** | 5 (artifact) | 1501 / 1074 | `rwp0-oracle-hub4-test.out` |
| `wasm32-unknown-unknown` lib | 5,693 | 5,693 | **0** | 0 | 203 / 224 | `rwp0-oracle-hub3-wasm.out` |
| `wasm32-unknown-unknown` `--test` | 8,696 | 8,701 | **0** | 5 (artifact) | 864 / 1057 | `rwp0-oracle-hub3-wasm-test.out` |

The 6,000-odd baseline errors are the unresolved external crates of the sandbox (same in both). The 5 B-only errors in the two test configurations are one artifact: B is built by inlining the hubs back, and its `cfg(test)` const `RENDERER_SOURCE` still `include_str!`s the five hub files that B no longer has (`couldn't read …/📥️mounts/…/🦀️.rs`). Runs before the last two fixes (kept as evidence of the method, not of the final tree): `hub1` (snapshot baseline, 73 new `macro_export` errors, section 5 item 1) and `hub2`/`hub3` lib (one new error `unresolved import ui_gpu` that was the peer's added root line, then 5 old-only errors from a duplicated `native_runtime_modules` mount in B).

Seeded sensitivity (`--seed`: glob re-export `pub(crate) use frame_actions::*;` removed from the WP3 hub, scenes path broken in the WP2 hub, the `camera_storage` import removed from the root): **caught**. Path seed (`seed1`): a single fatal `couldn't find file …/📥️mounts/🎞️wp2-scenes/…/📛️absent/🦀️.rs` (rustc aborts at module loading). Glob and import seeds together (`seed2`, `rwp0-oracle-seed2-lib.out`): **105 new errors** (E0425 x47, E0433 x51, E0422 x7, among them `cannot find \`camera_storage\` in \`crate\``) against 6,157 in the baseline.

## 7. Residual risks and hand-offs

1. **Cargo evidence is partial**: 5 of 88 skeletons compile through the launcher; the other 83 are behind the oversized frontier (os-kernel, os-infinite, …) and were checked statically and by the rustc front-end (`-Zunpretty=expanded`). The lock goes stale whenever a peer adds a crate: run `update --workspace --offline` before a check.
2. **`icon-host` cannot pass the file gate** while the 250 SVG embeds stay (plan 6.2, risk 7); decision for WP2 with the Canvas owner.
3. **Pre-existing red that WP1..WP5 will meet**: 13 TS readers fail identically before and after (empty fixture folders, unresolved node-graph package); `🔬️rust-policy-source-evidence` and `verify taxonomy` cannot load the root `📜️script.ts`; taxonomy `generator-activation-invalid` (catalogue digest); the live test closure cannot be counted until `🖱️ui/…/🧊️gpu/🦀️.rs` exists again.
4. **Peers edit the renderer root** (`use ui_gpu::{GpuContext};` appeared mid-run; a launch normalizer appended `env` to every Rust route). The differential supports `--baseline inverse` for that; WP1 owns the root.
5. **`test-support` is a skeleton**; WP3 implements `read_module_tree_source` (contract in 10.4.7) and removes the `*_SOURCE` consts afterwards.
6. **Shell non-Shell edges** are intentionally open (only `contracts` and `host-base` declared); WP4/WP5 add the downward edges they use, `verify_static.ts` rejects upward ones.
7. After the first crate leaves the monolith the WP0 oracle can no longer answer (`extern crate … as …;` becomes an empty module in the sandbox); use the union oracles of the earlier carves.
8. Large tool output files (`rwp0-oracle-*.jsonl`, up to 17 MB each) in `🗑️generated` are removed at the end of this task; summaries stay.
