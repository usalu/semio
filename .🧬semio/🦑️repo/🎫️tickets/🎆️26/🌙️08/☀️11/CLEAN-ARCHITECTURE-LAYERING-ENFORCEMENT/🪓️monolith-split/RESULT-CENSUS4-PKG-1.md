# Result CENSUS4-PKG-1 (prefix `c4p1`): os/♾️infinite world crate roots split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (read-only `git cat-file`, `diff --stat`, `ls-files`, `grep`), no ticket tool. `F` = this folder, `W` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world`. Tools: `F/c4p1-tools/`. Logs and reports: `F/🗑️generated/c4p1-*` (nothing deleted). Scratch sandboxes: `scratchpad/c4p1-work`. Baseline = `HEAD b84bf526c7b` (the nine files were byte-identical to it before the first edit; the installer asserted this per target).

## 0. Verdict

**GREEN against the oracles, UNCOMPILED.** All nine files of Package 1 (24,174 lines) are split. No crate root and no child exceeds 963 lines (largest child: `cursors/🕹️gumball-and-brush`, 963). 31 new folders, each `<emoji>️<kebab>/🦀️.rs`.

* The seven lib roots keep wiring (imports, `#[path] mod`, `pub use <mod>::*;`) plus their real top-level API (state structs, entry points). Public paths are unchanged: every `pub` item is still reachable at the crate root, so the facade `W/🦀️.rs` (`pub use semio_framework_os_infinite_world_<crate>::{names}`) and every renderer import stay valid. No caller changed.
* The two host unit-test roots became `include!` fragments, because the 157 test names must stay identical (a child module would add a path segment). 86 + 71 `#[test]` fns before and after.
* No file was large because of data or tables (the biggest literals are 8 to 10 line JSON consts), so no fixture move was needed.
* The oracle found one real defect in my first cut (3 `E0425`, two private functions behind a `/** .. */` comment that my widener skipped); it was fixed and the rerun is clean (section 4).
* Real `bun repo cargo` was not run (compile admission is closed this wave); commands for the queue are in section 7.

## 1. What was cut

| target | before | root after | children (lines) |
| --- | ---: | ---: | --- |
| `🌉️bridge` | 2525 | 88 | `🎞️scene-bridge` 943, `🖌️overlays-and-gumball` 714, `🎥️camera-and-retirement` 801 |
| `🎛️host` | 2003 | 270 | `🔄️sync-and-overlay` 690, `🎬️render` 499, `📥️snapshot-and-input` 565 |
| `🗃️model` | 2967 | 56 | `🎒️prepared-resources` 422, `🧬️scene-records` 573, `🗂️registries` 881, `🖍️tool-run-trace-layer` 414, `🔖️interaction-transaction` 656 |
| `🧩️components` | 2018 | 607 | `🫧️face-overlay-and-fit` 664, `🪄️placeholder-mesh-and-gumball` 761 |
| `🧲️cursors` | 2931 | 274 | `🔲️marquee` 937, `🕹️gumball-and-brush` 963, `🎯️ray-and-component-pick` 778 |
| `🧱️scene` | 2407 | 139 | `📡️asset-io` 695, `🥽️brush-lod-and-style` 862, `🏔️environment-and-terrain` 732 |
| `🧵️interaction` | 3347 | 505 | `🖱️pointer-and-picking` 779, `🌪️vortex-relocate-and-terrain` 603, `⚙️authority-core` 911, `📜️plan-and-publish` 577 |
| `🎛️host/🧪️tests/🔬️unit` (include) | 2950 | 171 | `🔲️cursor-and-marquee-laws` 699, `🕹️gumball-and-authority-laws` 705, `🖼️reference-and-pick-lane-laws` 701, `🧲️selection-and-retirement-laws` 686 |
| `🎛️host/🧪️tests/🔭️unit-2` (include) | 3026 | 174 | `🥽️brush-environment-and-hover-laws` 854, `🌉️scene-bridge-and-camera-laws` 741, `🖼️reference-and-component-laws` 697, `🧲️native-gumball-and-primary-laws` 572 |

(Each target lives at `W/<crate>/🦀️.rs`, the test roots at `W/🎛️host/🧪️tests/<dir>/🦀️.rs`; the children are folders next to the root file. The line count of a child includes its docstring and `use super::*;`.)

**New folders (31), repo-relative tails:** `🌍️world/🌉️bridge/{🎞️scene-bridge,🖌️overlays-and-gumball,🎥️camera-and-retirement}`, `🌍️world/🎛️host/{🔄️sync-and-overlay,🎬️render,📥️snapshot-and-input}`, `🌍️world/🗃️model/{🎒️prepared-resources,🧬️scene-records,🗂️registries,🖍️tool-run-trace-layer,🔖️interaction-transaction}`, `🌍️world/🧩️components/{🫧️face-overlay-and-fit,🪄️placeholder-mesh-and-gumball}`, `🌍️world/🧲️cursors/{🔲️marquee,🕹️gumball-and-brush,🎯️ray-and-component-pick}`, `🌍️world/🧱️scene/{📡️asset-io,🥽️brush-lod-and-style,🏔️environment-and-terrain}`, `🌍️world/🧵️interaction/{🖱️pointer-and-picking,🌪️vortex-relocate-and-terrain,⚙️authority-core,📜️plan-and-publish}`, `🌍️world/🎛️host/🧪️tests/🔬️unit/{🔲️cursor-and-marquee-laws,🕹️gumball-and-authority-laws,🖼️reference-and-pick-lane-laws,🧲️selection-and-retirement-laws}`, `🌍️world/🎛️host/🧪️tests/🔭️unit-2/{🥽️brush-environment-and-hover-laws,🌉️scene-bridge-and-camera-laws,🖼️reference-and-component-laws,🧲️native-gumball-and-primary-laws}`. The list is in `F/🗑️generated/c4p1-folders.txt`.

**Correction (SPLIT-HARDENING-7).** The sentence below, "0 region pairs split across files", was wrong: `c4pcheck.py` compared region *names* per file, not nesting, and ten children had unpaired markers. SH7 repeated the spanning regions balanced in every child (21 comment lines, no code line touched; `F/RESULT-SPLIT-HARDENING-7.md` section 1). The three roots `🎛️host`, `🧩️components`, `🧱️scene` stay unbalanced exactly as at HEAD.

### Mechanism

* **Lib roots (mod children).** Each child starts with `//! <emoji> ...` (first emoji unique among the root and its siblings) and `use super::*;`, then the cut lines verbatim. The root gets, at the position of the cut, `#[path = "<folder>/🦀️.rs"]`, `mod <snake>;` and `pub use <snake>::*;`. No qualified path was introduced; the root's own `use` block stays where it was. Mounts that were already there (`tool_run_trace`, `testkit`) are untouched.
* **Visibility.** A private definition (item, struct field, inherent method) moved into a child is widened to `pub(super)` when its name occurs in any other file of the crate closure (conservative token scan, `frs-tools/splitter.py` helpers) and no wider form exists. 20 definition lines, each one removed line plus the same line with `pub(super) `: `selection_mode_label` (bridge); `append_tool_run_trace_draws`, `append_world3d_section_cap`, `paint_world3d_annotations`, `paint_world3d_legend`, `paint_world3d_scalar_mismatch` (host); `ToolRunTraceSlot`, `ToolRunTraceLayer.{lane_digest,records,batches,next_stamp,newest_testing}`, `apply_page`, `upsert`, `lane_digest` (model); `pick_reference_at`, `object_world_position`, `update_dragged_instance_position`, `plan_world3d_pointer_leave`, `plan_world3d_non_primary_press_pick` (interaction). Components, cursors and scene needed none: the W4 carve had already made their cross-file items `pub`.
* **Block-comment fix.** The frs outline reads a `/** .. */`-led function as one `other` item, so the first cut missed `plan_world3d_pointer_leave` and `plan_world3d_non_primary_press_pick`. `c4psplit.py` now has `block_comment_edits`; the oracle caught the miss (section 4, run `a1`), the fix was reinstalled with `c4pupdate.py` (it refuses unless the live file is still byte-identical to what this slice installed).
* **Test roots (include fragments).** A fragment is spliced back into the same module, so test paths, visibility and `use` scope are unchanged and nothing is widened. The fragment cannot carry `//!` (E0753), so it starts with an outer `///` line whose first character is the unique emoji (the first item of every fragment is a `#[test]` fn). Precedent: `RESULT-RUST-FILE-SPLITS-8.md` package 2. The test files contain no relative `include*!`, `#[path]`, `file!()`, `line!()` or `module_path!()`, so no re-basing was needed.
* **Regions.** Every `//#region` / `//#endregion` marker moved with its cluster; the cuts were chosen so that each region that has both markers in the original keeps both in one file (check below). Existing unbalanced markers of the W4 carve (for example `components` 7 opens, 1 close) were not touched.

## 2. Source-text readers (re-pointed)

* **Rust tests that read the world sources:** `W/🗃️model/🧪️tests/🔬️unit/🦀️.rs` and `W/🧱️scene/🧪️tests/🔬️unit/🦀️.rs` built `WORLD_SOURCE()` from the seven lib roots with `read_joined`. They now use a new helper `read_world_tree(path)`, which reads a root and splices every child that is mounted as `#[path] mod x; <vis> use x::*;` in at its mount, so the text keeps the exact item order of the unsplit file (`tool_run_trace` and `testkit` are not spliced, as before). In the scene test `read_joined` became unused and was removed; the model test still uses it for the renderer and shell sources. Test names unchanged.
* **TypeScript readers need no edit:** `readModuleTreeSource` / `listModuleTree` / `policyReadModuleTreeSource` / `policyReadRustPolicySource` follow `#[path]` mounts. The seven roots are all listed in `POLICY_SPLIT_SOURCE_ROOTS` (`🔍️discovery/📖️source-access/🟦️.ts`), `INTERACTIVITY_AUDIT_WORLD3D_FILES`, the fem script `frameworkWorld` and `WORLD_CRATES` of `🧲️scene-input-residency/🟦️.ts` already read them as trees. `F/c4p1-tools/c4p1-tree.ts` (bun, the repo's own reader) resolves 32 files over the seven roots (7 + 3 + 4 + 4 + 4 + 5 + 5, including the existing `⏯️tool-run-trace` and `🧰️testkit`), and the four assertions of the scene-input-residency source law give the same answers as the W4 result: `grid_snap_enabled: bool` true, `snap_world_point_to_grid(` count 4, `cancel_reference_image_url` true, `world3d_reference_url_is_current` true. `bun test` / vitest of the whole law set was not run (bun crashes on that file, `RESULT-W4-OS-INFINITE.md`).
* The 10 widened names occur in no TS, JSON or Python file (`git grep`), and the fixture `📇️surface-verbs.json` addresses `🧩️components/🦀️.rs::world3d_offers_transform_gumball`, which stays in the components root.
* **Renderer imports:** `RESULT-RENDERER-INFINITE-IMPORTS.md` shows the renderer crates import board items from `semio-framework-os-infinite-board-*`, not from these files. No `.rs` outside `W` names `semio_framework_os_infinite_world_*` (`git grep`); the facade is the only consumer and the oracle family contains the facade crate.

## 3. Losslessness and structure checks (all on the live tree)

1. `python3 c4p1-tools/c4plossless.py live`: stripped non-empty line multiset of each pre-edit blob versus the union of its files. **LOSSLESS OK, removed 20 / added 151.** Removed = exactly the 20 widened lines (each with its `pub(super)` twin among the added ones; the tool fails otherwise). Added = 100 wiring lines (23 each of `#[path]`, `mod`, `pub use x::*;`, `use super::*;`, plus 8 `include!`), 31 child docstrings and the 20 widened twins. Fragment targets: removed 0, added 8 each (4 `include!` + 4 `///`). Log `c4p1-lossless-live.log`; `c4p1-seq-live.log` and `c4p1-check-live.log` hold the output of checks 2 and 3.
2. `python3 c4p1-tools/c4pseq.py`: every child is spliced back at its mount (as `read_world_tree` and `readModuleTreeSource` do) and the stripped non-empty line **sequence** is compared with the pre-edit blob after dropping wiring, each child header and `use super::*;`, and the `pub(super) ` prefix. **SEQUENCE OK: all nine targets are order-identical** (2371, 1950, 2634, 1792, 2775, 2186, 3225, 2774, 2849 non-empty lines).
3. `python3 c4p1-tools/c4pcheck.py`: **CHECKS OK**. Per target: every `#[path]` / `include!` target exists (0 missing), every new file is reachable exactly once in its crate closure, 0 duplicate mounts (0 new; none existed), no repeated first emoji among children or against the root, region markers preserved (counts equal per target, 0 region pairs split across files), `#[test]` fn multisets identical (86 = 86, 71 = 71). Longest repo path 175 UTF-8 bytes (limit 240), largest file 963 lines.
4. `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true` over all 40 files (nine roots + 31 children): 0 parse failures. `rustfmt --check` hunk counts (repo `rustfmt.toml`, `skip_children=true`), original blob versus root plus children: bridge 48 = 48, host 39 = 39, model 24 = 24, components 45 = 45, cursors 38 = 38, scene 20 = 20, interaction 51 = 51, unit 47 = 47, unit2 101 = 101, total **413 = 413**: the split adds no formatting debt.
5. Taxonomy: `bun c4p1-tools/c4ptaxcheck.ts` (copy of `rf8-tools/taxcheck.ts`) over the 31 folders: all resolve (23 `crate-module-children`, 8 `members-of-tests`), `checked=31 unresolved=0`. Every folder emoji is already used as a folder prefix in the repo (`git ls-files` scan, counts from 86 to 81,177).

## 4. Typeck oracle (`pos.py`, `POS_PREFIX=c4p1`, family `pos-tools/families/c4p1.json`)

Family: the seven world crates plus the facade crate `semio-framework-os-infinite` (the only crate that names them), current tree against `b84bf526c7b`, configurations lib and `--test`.

| run | what | lib | test |
| --- | --- | --- | --- |
| `a0` (before the first edit) | cur = base | 3071 / 3071 errors, NEW 0, GONE 0, canary reached | 3560 / 3560, NEW 0, GONE 0, canary reached |
| `a1` (first cut) | **3 real rows** | NEW 4: `E0425` x3 (`plan_world3d_non_primary_press_pick` x2, `plan_world3d_pointer_leave`, inaccessible private fns in `⚙️authority-core`) + 1 re-keyed stub row | the same |
| `a2` (after the fix) | final | 3071 / 3071, NEW 1 / GONE 1, privacy 0, trait-scope 0, resolve 0, type 0, other 0, canary `E0308 E0616 E0624` reached | 3560 / 3560, NEW 1 / GONE 1, all categories 0, canary reached |

The single NEW row of `a2` is the `E0308` at `🧩️components/🪄️placeholder-mesh-and-gumball/🦀️.rs:328` (`Box::new(closure)` against the stub type `__ext::semio_framework_async::Job`), flagged stub noise by the tool; its twin at the old line in the monolith root is the GONE row, and the totals are equal (3071 = 3071, 3560 = 3560). **0 new rows, canary reached in lib and test.** Reports: `c4p1-c4p1-a2-report.txt`, `-new-lib.txt`, `-new-test.txt`; log `c4p1-pos-a2.log`. `a1` is the sensitivity proof: the oracle does see a missing `pub(super)` on a function used by a sibling child.

The `--mutate` flag of `pos.py` was tried on a field and a function (`n1`, `c4p1-pos-n1.log`) and produced no extra error (3071 = 3071), so its mutation is not applied in this tool path; I do not rely on it and report it as an unverified control.

`lit.py` (E0451 scanner, `POS_PREFIX=c4p1 python3 rpv-tools/lit.py pos-tools/families/c4p1.json <tag>`): **0 candidate E0451 sites** (149 brace structs, 158 definitions, 36 files in the module tree, which includes all 23 lib children), run before and after the widening fix (`c4p1-lit-live.log`, `c4p1-c4p1-live2-e0451.json`); the self-test (`LIT_SELFTEST=1`) finds its seeded probe (2 sites), so the scanner sees this module tree.

Not covered by the oracle: the `[[test]]` integration roots (the host unit fragments) are not part of a `--test` union. For them the proofs are the sequence check (the spliced fragments equal the original text line for line), rustfmt parse, and the unchanged scope rules of `include!`.

## 5. Census and test-closure census (canonical `readRustSourceClosureV1`, gate 4000 lines / 32 files / 262,144 bytes)

Tool `c4p1-tools/c4p1-census.ts` (copy of `rii-tools/rii-census.ts`; `C4P1_BASE_MAP` serves the nine pre-edit blobs, so "before" is measured with the same counter on the old text). Outputs `c4p1-census-{before,after,final}-*.tsv|json`; `final` is the run after the widening fix. Native lib closure files / bytes / active lines, and the worst test-unit ratio (1.0 = gate):

| crate | lib before | lib after | lib + cfg(test) before | lib + cfg(test) after | worst unit ratio |
| --- | --- | --- | --- | --- | --- |
| infinite-world-model | 2 / 149,793 / 3,331 | 7 / 150,988 / 3,371 | 5 / 193,968 / 3,604 | 10 / 195,163 / 3,644 | 0.901 > 0.911 |
| infinite-world-components | 1 / 111,485 / 2,009 | 3 / 111,997 / 2,025 | same | same | 0.502 > 0.506 |
| infinite-world-cursors | 1 / 154,076 / 2,932 | 4 / 154,717 / 2,956 | same | same | 0.733 > 0.739 |
| infinite-world-scene | 1 / 126,078 / 2,391 | 4 / 126,770 / 2,415 | same | same | 0.598 > 0.604 |
| infinite-world-bridge | 1 / 147,387 / 2,526 | 4 / 148,141 / 2,550 | same | same | 0.631 > 0.637 |
| infinite-world-interaction | 1 / 208,907 / 3,348 | 5 / 209,919 / 3,380 | same | same | 0.837 > 0.845 |
| infinite-world-host | 2 / 126,594 / 2,051 | 5 / 127,287 / 2,075 | same | same | 0.884 > 0.887 |
| infinite (facade) | 3 / 43,632 / 315 | unchanged | unchanged | unchanged | 0.166 |

Host test units: `world_host_unit` 1 / 193,749 / 2,951 > 5 / 194,461 / 2,967 (ratio 0.739 > 0.742), `world_host_unit_2` 1 / 231,827 / 3,026 > 5 / 232,556 / 3,042 (0.884 > 0.887); the other six are unchanged. Summary of `final`: 4,845 units, 0 error units, **0 over-gate test units (native and all labels)**, wasm32-unknown-unknown lib lines equal the native ones, wasip2 closures stay 0 (the world crates are empty there). The 14 over-gate dev-dependency rows (7 native) are identical before and after (`semio-framework-ui` reached through the dev edges, the known rii finding; not caused by this slice). The splits add files (at most 10 per crate against 32) and a few lines of wiring (at most 32 lines per crate, `interaction`); nothing crosses a limit. The binding limit stays bytes: 0.911 for the model `lib + cfg(test)` closure (3,604 > 3,644 active lines, still below the 4,000 gate and above the 3,500 target of the W4 result; the follow-up named there, moving the trace unit tests to an integration target, is still open).

## 6. Decisions and residual risks

* Glob wiring (`use super::*;` plus `pub use x::*;`) follows `RESULT-INFINITE-SPLIT.md`; it keeps the roots' import blocks unchanged (lossless) at the price of children whose imports are implicit. The explicit per-child import form of `frs-tools` would be a follow-up if the crates are cut further.
* The widener is name-based and conservative; the conservative cases are the generic names `records`, `batches`, `upsert`, `apply_page` of the trace layer (they became `pub(super)` although a sibling might not use them). Narrowing them needs a compile.
* Not verified by a compiler: borrowck, trait-method resolution through glob re-exports, and the lint set. A `glob import`-related `ambiguous_glob_reexports` warning would show only in a real build.
* `include!` fragments are invisible to rustfmt (it does not follow `include!`); they were parse-checked as files.
* A concurrent `.ts` change set (`r3f`, `palette`, `script.ts`, `fonts`, `fixtures`) sits in the same git status; none is mine.

## 7. Real-check commands for the compile queue (not run: compile admission is closed)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-infinite-world-model -p semio-framework-os-infinite-world-components -p semio-framework-os-infinite-world-cursors -p semio-framework-os-infinite-world-scene -p semio-framework-os-infinite-world-bridge -p semio-framework-os-infinite-world-interaction -p semio-framework-os-infinite-world-host -p semio-framework-os-infinite --lib --keep-going --message-format=short
```
then the same with `--tests` instead of `--lib` (runs the fragment roots `world_host_unit` and `world_host_unit_2` and the two source-reading tests), and with `--target wasm32-unknown-unknown` (the world crates are `cfg`-gated empty on wasip2). Tests to run afterwards: `cargo test -p semio-framework-os-infinite-world-host --test world_host_unit --test world_host_unit_2`, `-p semio-framework-os-infinite-world-model --test unit`, `-p semio-framework-os-infinite-world-scene --test unit` (the two that call `WORLD_SOURCE()`). Hold the fleet compile mutex (`📓️WAVE5-BRIEF.md`) while the launcher runs. If a build exposes a missing `pub(super)`, widen that one definition line (`c4psplit.py` + `c4pupdate.py` do this reproducibly).

## 8. Files

* Edited (11): the nine targets (`W/{🌉️bridge,🎛️host,🗃️model,🧩️components,🧲️cursors,🧱️scene,🧵️interaction}/🦀️.rs`, `W/🎛️host/🧪️tests/{🔬️unit,🔭️unit-2}/🦀️.rs`) and the two source readers `W/🗃️model/🧪️tests/🔬️unit/🦀️.rs`, `W/🧱️scene/🧪️tests/🔬️unit/🦀️.rs`.
* Created (31): the children listed in section 1.
* Tools, kept: `F/c4p1-tools/{c4plib,plans,c4psplit,c4pinstall,c4pupdate,c4plossless,c4pseq,c4pcheck,outl,ctx}.py`, `c4p1-census.ts`, `c4p1-tree.ts`, `c4ptaxcheck.ts`; family `F/pos-tools/families/c4p1.json`. Journal `F/🗑️generated/c4p1-installed.json` (installed paths and shas), pre-edit blobs in `scratchpad/c4p1-work/base/`.
* No debug logs were added (`[DEBUG]` is absent from every new file). No registry, manifest, `launch.json` or lock edit was necessary: no crate, test target or command was added or renamed.
