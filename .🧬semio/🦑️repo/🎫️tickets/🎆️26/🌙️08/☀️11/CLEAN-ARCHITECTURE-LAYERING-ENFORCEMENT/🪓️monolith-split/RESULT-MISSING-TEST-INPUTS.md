# 🧯️ RESULT: Missing Test Inputs (slice `mti`, 2026-10-16)

Executor: Sonnet, foreground, no sub-agents, no ticket tool, no modifying git command, no cargo build.
Source: `TEST-CLOSURE-CENSUS.md` section 6 (11 native / 32 total test units the counter refused).

## 0. Verdict

- **10 of the 11 native units are measurable again**, and 30 of the 32 error units across the three cfg labels (32 before, 2 after; section 3). The counter now returns a size for each of them.
- **1 unit stays refused: `semio-framework-os-renderer-wgpu` lib-test** (native and wasm32-unknown-unknown; it had no wasm32-wasip2 error row in the baseline either). Every other input of its closure now resolves (verified with a stubbed read, section 4). The two inputs that are left are not moved files. They belong to the in-flight carve of `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw` and are handed off (section 6.1).
- No fixture content was fabricated. All 8 root causes are a **moved or deeper-carved location**; every target file exists in HEAD with identical content (renames were `R100` in the sweep commits). Nothing was restored from history, nothing had to be generated.
- Cargo was not run (compiler admission is closed for this fleet). Nothing here proves the touched tests compile; it proves the inputs they read exist and that the canonical source reader accepts the closures.

## 1. Root causes and evidence

| # | Unit(s) | What was missing | Where it went (evidence) | Fix |
| --- | --- | --- | --- | --- |
| A | `plugin`, `ui-runtime-reconcile`, `ui-runtime-reconcile-primitives` (lib-test) | `🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🧫️fixtures/🔣️.json` | Commit `1bae6115bc5` (2026-10-16 04:57) moved the whole `♻️retirement/🌲️built` tree to `🧬️contract/🏗️builder/♻️retirement` (`R100` for the fixture, `🎟️original`, `📋️children`; the `🦀️.rs`, `🧪️tests/🔬️unit` and the TS test appear as `D` plus `A`/`M` at the new location because they were edited). `git log --diff-filter=DR` on the old folder lists exactly this commit. New fixture: `🧬️contract/🏗️builder/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json` (875 bytes, exists). | 3 `include_str!` literals re-pointed (section 2.A) |
| A2 | repo library TS tests (same moved tree) | TS import, `native-source-ownership` fixture (`built.source`, `built.fixture`) and the Nx input glob still named `🌲️built` | same commit | 4 files re-pointed (section 2.A) |
| A3 | `native-source-ownership` fixture, `surface.fixture` | `🗺️surface/🧫️fixtures/📇️bindings.json` | Commit `0c3e78944af` (2026-10-14 02:26, fixtures sweep): `D` of the old path; present at `🗺️surface/🧪️tests/🧫️fixtures/📇️bindings.json`. Found because `bun test` of the touched TS test failed on it. | 1 literal |
| B | `os-renderer-wgpu` lib-test | `✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json` and 22 more fixture literals of the same shape | Commit `0c3e78944af` moved every `🧫️fixtures` folder to `🧪️tests/🧫️fixtures` (TextEditor, EventFeedHost, Table, `✍️editor`, `🖱️ui`, `🗺️surface`, VirtualFileSystem, Canvas2dHost, GraphTimelineHost, `engine/🧫️fixtures`). The TS importers were updated by the sweep; the Rust `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), …))` literals were not. Empty `🧫️fixtures/…` folder shells remain at the old places (TextEditor 4, EventFeedHost 2, Table 2). | 23 literals re-pointed (section 2.B) |
| C | `os-renderer-wgpu-scene-feeds`, `-scene-tables`, `-scene-text-vfs` (lib-test) | the same fixtures, **and** wrong `..` depth | These three crates were carved one level deeper (manifest dir `engine/🎯️targets/🧊️wgpu/<crate>/📦️packages/🦀️rust`, depth 11) than the crate the test files were written for (`os-renderer-wgpu`, depth 10). The reader's error path `…/🎯️targets/🧱️elements/…` and `🧰️framework/🛍️products/🔨️modules/…` shows the lost level. Each test file is mounted exactly once (by `🎞️Scenes/🎯️targets/🧊️wgpu/<unit>/🦀️.rs` from one scene crate), so the depth is unambiguous. | one more `..` plus `🧪️tests/🧫️fixtures` in 14 literals (section 2.C) |
| D | `os-spr-laws-tests` (lib-test) | `/../../🔨️modules/🏪️store/🚪️io/🧪️tests/🧫️fixtures/🎚️field-set/🔣️.json` | Written for the os root crate (`os/📦️packages/🦀️rust`, `../..` = `os`). The test now lives under the `🥣️laws-companion` crate (`📡️spr/🧪️tests/🥣️laws-companion/📦️packages/🦀️rust`); its own `Cargo.toml` already uses 5 `..` to reach `os/🔨️modules`. The fixture `🏪️store/🚪️io/🧪️tests/🧫️fixtures/🎚️field-set/🔣️.json` exists unchanged (307 bytes). | 1 literal |
| E | `plugin-reactor-lifecycle` (lib-test) | `…/📨️pending/🚪️lifetime/🧪️tests/🧪️aggregate-admission/🦀️.rs` | The `include!` in `🔬️plugin-runtime-native-aggregate-backing` used `/../../🚪️lifetime/…`, valid for the old plugin crate (`🔌️plugin/📦️packages/🦀️rust`). The file is now compiled by the reactor-lifecycle carve (manifest `⚛️reactor/📨️pending/📦️packages/🦀️rust`); the target `🔌️plugin/🚪️lifetime/🧪️tests/🧪️aggregate-admission/🦀️.rs` was never moved (last touched by `0c3e78944af`, content change only). Single mount (`♻️reactor-lifecycle/🗃️instance-registry/🦀️.rs:221`). | 4 `..` instead of 2 |
| F | `plugin-host-test-component` (lib-test) | `Missing or ambiguous Rust module: sqlite_snapshot_tests` | `d4597d0410a` (2026-10-06) rewrote the glue's `#[path]` from `../../🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` (correct, in the preceding version `5c7f51ee643`) to `../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, a path that never existed under the component (`🧩️component/🚪️io` has no history). The original file is intact (13,719 bytes, uses `crate::{Snapshot,Mutation}` and the post-sweep fixture paths). | `#[path]` restored |
| G | `os-kernel` integration `artifact_command_retirement` | `📡️replication/🚪️io/📝️text/🧵️canonical/🧫️fixtures/🔣️.json` (and its `🔏️seal/🧫️fixtures` sibling, hidden behind the first) | `0c3e78944af`: `R100` to `…/🧵️canonical/🧪️tests/🧫️fixtures/🔣️.json`; the `🔏️seal` one already exists as `🔏️seal/🧪️tests/🧫️fixtures/🔣️.json`. | 2 literals |
| H | `os-renderer-wgpu` lib-test (remaining) | `🖍️draw/🔧️pipeline/🦀️.rs`, `🖍️draw/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs` | Not a move: see section 6.1. | hand-off |

## 2. What changed (all one-line literal or `#[path]` re-points; nothing else)

`F` = this folder. Counts are changed lines per file (`git diff --stat`).

### 2.A `🌲️built` to `🏗️builder` (7 files)
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs` (1), `…/🔒️authority/🦀️.rs` (1): `include_str!("…/🧬️contract/♻️retirement/🌲️built/🧪️tests/🧫️fixtures/🔣️.json")` to `…/🧬️contract/🏗️builder/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🦀️.rs` (1): same replacement. **Area `pi1`, one-line path fix.**
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📁fixture-placement/🟦️.ts` (1) and `…/📚️library/🧪️tests/⚙️native-source-ownership/🟦️.ts` (1): import of `🔬️built-tree-retirement/🟦️.ts` re-pointed.
- `…/📚️library/🧪️tests/🧫️fixtures/⚙️native-source-ownership/🔣️.json` (3 lines): `built.source` now `…/🏗️builder/♻️retirement/🦀️.rs`, `built.fixture` now `…/🏗️builder/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json`, `surface.fixture` now `…/🗺️surface/🧪️tests/🧫️fixtures/📇️bindings.json`. The new `built.source` is the file whose `tests` module (`#[path = "🧪️tests/🔬️unit/🦀️.rs"]`) includes `../🧫️fixtures/🔣️.json`, which is what the TS law checks.
- `…/📚️library/📦️packages/🟦️typescript/📋️project.json` (1 line of mine, in the `test-native-source-ownership` target): input glob `🧬️contract/♻️retirement/🌲️built/**/*` to `🧬️contract/🏗️builder/♻️retirement/**/*`. (`git diff` of this file also shows one added line at 1333 from a peer's concurrent edit; not mine.)

### 2.B `🧫️fixtures` to `🧪️tests/🧫️fixtures` in the `os-renderer-wgpu` closure (23 literals, 5 files, all under `…/📺️renderer/🧑‍🎨engine/`)
The 2 TextEditor explicit-draft literals were edited by hand; the other 21 by `mti-tools/mti-fixtures-remap.py` only where the resolved candidate with `🧪️tests/` inserted exists on disk (21 of 21 did); the number of `..` is unchanged.
- `🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs` (8 lines: TextEditor explicit-draft x2, delivery, local-echo; `✍️editor` text-input x3 and `🧬️scene` corpus).
- `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` (7) and `…/🖥️wgpu-display-conflicts-marketplace/🦀️.rs` (1): ui fixtures, `engine/🧫️fixtures` window-lifecycle, Canvas2dHost.
- `🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs` (6): editor, Table x2, EventFeedHost, GraphTimelineHost.
- `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` (1): `🛒️canvas-catalogue-terminal`.

### 2.C Scene crates (5 files, `🧱️elements/🎞️Scenes/🧪️tests/`)
`🔬️wgpu-block-list` (2 lines), `🔬️wgpu-event-feed` (2), `🔬️wgpu-graph-timeline` (1), `🔬️wgpu-table` (6), `🔬️wgpu-virtual-file-system` (3): `"/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/…"` (9 `..`) to 10 `..` plus `🧪️tests/🧫️fixtures`; `"/../../../../🧱️elements/<Host>/🧫️fixtures/…"` (4 `..`) to 5 `..` plus `🧪️tests/🧫️fixtures`; the VirtualFileSystem descriptors literal likewise.

### 2.D Others (one-line each)
- `🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws-unit/🦀️.rs` (1): `/../../🔨️modules/🏪️store/…` to `/../../../../../🏪️store/…` (5 `..` = the laws-companion manifest's own depth).
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs` (1): `/../../🚪️lifetime/…` to `/../../../../🚪️lifetime/…`. **Area `pi1`, one-line path fix.**
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/📦️packages/🦀️rust/🦀️.rs` (1): `#[path]` back to `../../🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`. **Area `phc` (plugin-host test component), one-line path fix.**
- `🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🧪️tests/🦀️.rs` (2): the two `…/🧵️canonical/…/🧫️fixtures/🔣️.json` literals now name `🧪️tests/🧫️fixtures`. **Area `okeng`/`okwp8` (os-kernel store tests), one-line path fix.**

Touched list: `F/mti-tools/touched.txt` (21 paths, plus none created). No file was created in the repo, deleted or moved. Tools: `F/mti-tools/` (`mti-census.ts`, `mti-missing.ts`, `mti-literals.ts`, `mti-envpaths.ts`, `mti-fixtures-remap.py`, `mti-edit.py`, `mti-checks.py`). Logs: `F/🗑️generated/mti-*`.

## 3. Census before and after

`mti-tools/mti-census.ts` is a verbatim copy of `tcc-tools/tcc-census.ts` whose only change is the output file prefix (`mti-census-*` instead of `tcc-*`), so the audit's raw `tcc-*` outputs stay untouched. Run with `ONLY=<the 11 crates>` before the first edit and after the last (`mti-baseline.log`, `mti-after.log`, `mti-baseline-errors.tsv`, `mti-after-errors.tsv`).

| Run | Error units (all labels) | native | wasm32-unknown-unknown | wasm32-wasip2 |
| --- | ---: | ---: | ---: | ---: |
| baseline (before) | 32 | 11 | 11 | 10 |
| after | **2** | **1** | 1 | 0 |

Newly measured native units (files / bytes / active lines / ratio against the limit; ratios under 1 are within limits):

| Crate (unit) | Files | Bytes | Lines | Ratio | Over |
| --- | ---: | ---: | ---: | ---: | --- |
| `os-kernel` integration `artifact_command_retirement` | 7 | 22,275 | 89 | 0.219 | no |
| `os-renderer-wgpu-scene-feeds` lib-test | 11 | 128,187 | 2,323 | 0.581 | no |
| `os-renderer-wgpu-scene-tables` lib-test | 14 | 171,948 | 2,639 | 0.660 | no |
| `os-renderer-wgpu-scene-text-vfs` lib-test | 8 | 118,536 | 2,047 | 0.512 | no |
| `os-spr-laws-tests` lib-test | 28 | 79,877 | 1,479 | 0.875 | no |
| `plugin-host-test-component` lib-test | 21 | 55,940 | 564 | 0.656 | no |
| `plugin-reactor-lifecycle` lib-test | 18 | 183,253 | 3,216 | 0.804 | no |
| `ui-runtime-reconcile` lib-test | 19 | 193,926 | 3,423 | 0.856 | no |
| `ui-runtime-reconcile-primitives` lib-test | 12 | 74,819 | 1,492 | 0.375 | no |
| **`plugin` lib-test** | **571** | **8,186,165** | **81,723** | **31.228** | **yes** |

New finding for the census: **`semio-framework-plugin` lib-test is 31.2 times over the limit** (571 files, 8.19 MB, 81,723 lines; wasm32-unknown-unknown 81,706 lines, wasm32-wasip2 81,727 lines). It was hidden behind the missing fixture. The `plugin` carve (`pi1` and the PLUGIN-WP slices) owns it. The 9 other newly measured units are within limits. The wasm rows equal the native ones within a few lines.

`os-renderer-wgpu` lib-test, measured with only the two section 6.1 inputs stubbed to empty (`mti-missing-2.log`, a lower bound): 585 files, 8,869,987 bytes, 38,076 active lines, ratio 33.8 (over).

## 4. Verification (every command was run; results are the real ones)

1. Baseline and after: `cd /Users/ueli/Documents/semio && ONLY=<11 crates> bun F/mti-tools/mti-census.ts` (227 to 285 s each under load 44). Results in section 3.
2. All-missing-inputs read (`mti-missing.ts`: the census with missing non-`.rs` files returned as empty and listed, so one run shows every missing input instead of the first): run 1 (after the first 6 edits) listed the remaining stale fixture paths; run 2 (after all edits, with only the two section 6.1 `.rs` inputs stubbed) lists **only those two**, zero other missing inputs, 0 error units across 11 crates and 3 labels (`mti-missing-1.log`, `mti-missing-2.log`).
3. `mti-envpaths.ts`: every `include_*!(concat!(env!("CARGO_MANIFEST_DIR"), "<rel>"))` literal in the touched closures resolved against the real manifest dir of its mounting crate (os-renderer-wgpu, the 3 scene crates, os-kernel, spr laws companion, reactor-lifecycle): **0 missing** after, 21 before the remap in the os-renderer-wgpu closure alone (`mti-envpaths-wgpu-before.tsv`, `mti-envpaths-after.log`).
4. `mti-literals.ts` (all plain `include_str!/include_bytes!/include!/#[path]` literals of 5,707 tracked `.rs` under `🧰️framework`): 38 misses before, 34 after. The 4 removed are the 4 literals I fixed (builder x3 in ui/plugin; the host glue). The 34 left are: doc-comment/macro placeholders (`…`, `...`, `literal`), the 2 hand-off inputs of section 6.1, the `🌊️flow/🗿️artifacts/…/🚪️io/🦀️.rs` `#[path]` list, a workflow escape-sequence false positive and `🖥️host/🧪️tests/🧫️fixtures/🛡️hostile-batch-edge/🦀️.rs` (`../../🦀️.rs`); none is in a section 6 unit.
5. `python3 mti-tools/mti-checks.py touched.txt` (21 paths): rustfmt (`+nightly-2026-07-20 --edition 2021 --emit stdout`, stdin, no child traversal) failures **0**; plain path literals checked 88, missing 2 (the section 6.1 inputs); JSON parse failures 0; longest touched path 186 bytes / 128 characters (limit 240 bytes / 256 characters), 0 over.
6. TS laws that read the moved tree: `bun test …/📚️library/🧪️tests/⚙️native-source-ownership/🟦️.ts` from the library package: first run 2 pass / 1 fail (the `surface.fixture` path of A3, found only by running it), after fixing it **3 pass / 0 fail** (`mti-ts-native-source-ownership{,-2}.log`). `bun test …/🖱️ui/🧪️tests/📁fixture-placement/🟦️.ts`: the file now loads (before, the moved import crashed it) and the moved-built-retirement reader test completes; **3 pass / 2 fail**. The 2 failures (`UI testing example folders belong beneath actual tests folders`, `UI native testing example includes resolve to physical test owners`) are other `🧫️fixtures` folders outside `🧪️tests` in the ui tree (the reported one is `🎬️scene/📐️math/📦️resource/🧫️fixtures`); they are not caused by my edits and are not changed.
7. Not run: any `cargo check/test` (compiler admission closed). The exact check for the compile queue, once the draw carve of 6.1 is settled (hold the fleet compile mutex; one launcher call per workspace):
   `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-kernel -p semio-framework-os-renderer-wgpu -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-spr-laws-tests -p semio-framework-plugin -p semio-framework-plugin-host-test-component -p semio-framework-plugin-reactor-lifecycle -p semio-framework-ui-runtime-reconcile -p semio-framework-ui-runtime-reconcile-primitives --lib --tests --keep-going --message-format=short` (add `--manifest-path 🧰️framework/Cargo.toml` for the members of the framework workspace, run separately). A compile of `plugin-host-test-component` is the real proof for hand-off F, since the restored `sqlite_snapshot_tests` module has not been compiled since 2026-10-06.

## 5. Not changed on purpose

- The empty `🧫️fixtures/…` folder shells left by the 2026-10-14 sweep (TextEditor, EventFeedHost, Table). They hold no file; removing them is a taxonomy-sweep decision.
- `🧰️framework/🛍️products/📓️print/🔮️oracles/🔣️.json:97` still lists `…/🌲️built/📜️script.ts`. That script was renamed away on 2026-09-08 (`de617a7c17f`), so the entry is stale since September and unrelated to this move.
- No `Cargo.toml`, lock, launch or registry file was touched.

## 6. Hand-offs

### 6.1 To the owner of the `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw` carve (the `1bae6115bc5` UI GPU cut): `os-renderer-wgpu` lib-test
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` (mounted by `🧊️renderer/📥️mounts/🧵️wp3-frame-core`) reads two sources as text and both are gone:
- line 9 `DRAW_SOURCE = include_str!(…/🖍️draw/🔧️pipeline/🦀️.rs)`. `🖍️draw/🔧️pipeline/` holds only the two crates `🎬️execution` (1,900 lines) and `🏭️factory` (1,387 lines). The law needles (`pub(crate) fn ensure_mesh_step`, `pub fn close_upload_step`, `RASTER_TEXTURE_TABLE_CAPACITY`, `claim_texture_allocation` and 29 more; 33 of 34 are absent from `pipeline/*` and the 25-line `🖍️draw/🦀️.rs` hub) now live in `🖍️draw/🕸️mesh/🦀️.rs` and `🖍️draw/🖼️raster/🦀️.rs` and the other carved draw crates. The pre-carve source was the 5,645-line `🖍️draw/🦀️.rs` (`git show 1bae6115bc5~1:<path>`). A correct fix has to decide which carved files `DRAW_SOURCE` is the concatenation of (the file already uses `concat!(include_str!(a), include_str!(b))` for `ENGINE_CANVAS_SOURCE`), so it is not a one-line path fix and I did not guess.
- line 12 `DRAW_LAWS_SOURCE = include_str!(…/🖍️draw/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs)`. That folder is empty. The pre-carve file `🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs` (688 lines, `D` in `1bae6115bc5`) is not identical to the 323-line `🔧️pipeline/🎬️execution/🧪️tests/🔬️unit/🦀️.rs`, so the laws were split, not moved.
Once both resolve, re-run `ONLY=semio-framework-os-renderer-wgpu bun F/mti-tools/mti-census.ts` to get the unit size.

### 6.2 To `pi1` / `phc` / `okeng` / `okwp8`
Informational: the one-line path fixes of section 2.D (plugin aggregate-backing, fixture-projection, test-component glue, store digest tests). Please do not revert them when re-carving; the underlying cause (a carve or sweep that moves a file without updating its relative `include!`/`#[path]` literals) will recur. The `plugin` lib-test is now measurable and 31 times over (section 3).

### 6.3 To the taxonomy sweep owner
The 2026-10-14 sweep (`0c3e78944af`) and the 2026-10-16 builder move (`1bae6115bc5`) updated TS importers but not Rust `env!("CARGO_MANIFEST_DIR")`-relative literals nor `#[path]` in glue files. `mti-tools/mti-envpaths.ts <manifest-dir> <pathspec>` and `mti-literals.ts <pathspec>` reproduce the check read-only in a few seconds and can run as a post-sweep gate. Two foreign `fixture-placement` failures are listed in section 4.6.

## 7. Files

Created: this file; `F/mti-tools/{mti-census.ts,mti-missing.ts,mti-literals.ts,mti-envpaths.ts,mti-fixtures-remap.py,mti-edit.py,mti-checks.py,touched.txt}`; `F/🗑️generated/mti-*` (logs, `mti-baseline-errors.tsv`, `mti-after-errors.tsv`, `mti-census-*.tsv`, `mti-old-draw*.rs` snapshots from history for 6.1). Edited: the 21 paths of `F/mti-tools/touched.txt`. No `[DEBUG]` line was added. No ticket tool, no git write, no sub-agent, no cargo.
