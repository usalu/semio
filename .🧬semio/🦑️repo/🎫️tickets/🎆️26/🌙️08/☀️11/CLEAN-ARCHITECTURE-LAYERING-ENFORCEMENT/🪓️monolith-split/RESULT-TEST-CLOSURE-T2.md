# 🧪️ Result TEST-CLOSURE-T2 (slice `tc2`, 2026-10-17)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/tc2-tools/` (census fork `tc2-census.ts`, `fixrt.py`, `mkcomp.py`, `unglob.py`, `modtree.py`, `testkit.py`, `reg.py`, `splitcomp.py`, the checkers and `ttc/` = the companion-aware union oracle). Logs and census outputs: `F/🗑️generated/tc2-*`. Oracle sandboxes and the pre-edit snapshot: `/Users/ueli/.cache/tc2/`.

## 0. Verdict

**DONE (static), UNCOMPILED.** Layout and block `--check-applied` exit 0 with 0 extras; the draw gate keeps 17 `--test` rows, all inherited from the peers' carve of `host-arena` (section 7). All 16 target crates are inside the compiler source gate for their `cfg(test)` closure on all three labels (native, `wasm32-unknown-unknown`, `wasm32-wasip2`): `tc2-census-final4` (33 crates, 27,891 units, 0 error units, 0 over-limit test units, 0 over-limit dev-dependency rows). The worst crate went from 6.31x to 1.00x (`layout-layout-editor-commands`, whose lib alone is 32/32 files); 15 of 16 are at 0.98 or below. `COMPILE ADMITTED` is not in `📓️FLEET.md`: nothing was compiled; the evidence is census, test-name multisets, byte-identical fixtures, rustfmt, static path/privacy scans and the scoped union oracle (section 5).

What this does NOT prove: that the moved tests compile. Several moved laws were already uncompilable at HEAD (they `use` a crate that depends on their own crate, or `crate::standards::…` / `crate::editor::…` paths that no longer exist); the companions fix the first kind (section 6) and inherit the second kind unchanged (section 7).

## 1. Task, as executed

Sixteen crates were over the gate only in their test closure (`AUDIT-WAVE7-CRATE-GATE.md` test-only rows): the draw drawing crates `scene, transform, editor, style, editor-commands, write, editor-interaction, model, host, structure, geometry`, the layout crates `mutations-frames, mutations-content, editor-commands` and the block editors `2d, 5d`. Re-measured first (`tc2-census-baseline`, identical to the audit rows). Three causes, in the order of their weight:

1. **Compiled fixtures.** 498 test-only embedded inputs (1,821,120 bytes: JSON quintets per mutation leaf, shared corpora) counted as files and bytes. The scene tests alone embedded 47 files / 602 KB.
2. **Test files next to a lib that already fills the gate.** `layout-editor-commands` is 32/32 files, `scene` 226 KB of 256 KB, `editor` 239 KB, `block-2d/5d` 27 files; any test file pushes them over.
3. **Mutation fixture laws mounted in the wrong crate.** The transform, style and structure laws (and the frames and content laws) import the aggregate `…-mutations` crate, which depends on the crate that mounts them: those laws could not compile where they stood.

## 2. What changed

### 2.1 Fixtures are read at run time (`tc2-tools/fixrt.py`)
Every `include_str!`/`include_bytes!` of a test-only input (586 sites in 141 files, inside test files and inside inline `#[cfg(test)] mod tests` of production files) became a run-time read relative to `CARGO_MANIFEST_DIR`. `const NAME: &str = include_str!(..)` became `static NAME: LazyLock<&'static str>` with the use sites rewritten to `*NAME` (type preserving); inline sites call a file-local `runtime_fixture_text`/`runtime_fixture_bytes` (the helper is placed in the innermost enclosing inline module). Precedent: the wfc 2d/3d tests and `RESULT-TEST-CLOSURE-T1.md`. Fixture files are untouched (sha256, section 5). The one computed site (`include_str!(concat!(env!("CARGO_MANIFEST_DIR"), ..))` in the host edit laws) was re-rooted to the companion manifest.

### 2.2 Companion `-tests` crates (`tc2-tools/mkcomp.py`)
Seventeen companion crates (one per oversized unit; `editor` has two, `layout mutations-frames` and `-content` two each) mount the moved test files **in place** with `#[path]`, in the module nest the parent used (so libtest ids keep their module path below the crate name); no test file was copied or renamed. Roots: `<node>/🧪️tests/🥣️companion/🦀️.rs` (`🍯️companion-1`, `🍰️companion-2` for the second one of a pair), manifests next to them under `📦️packages/🦀️rust`. A moved root takes its descendants (law files it mounts) with it; the parent loses only the `#[cfg(test)] #[path] mod ..;` line (and the emptied `//#region 🧪️Tests` pair).
Rewrites inside the moved files: `crate::` → the parent crate (unless the path ends in a moved test module), `super::x` → the parent module path, `use super::*;` and `use crate::[a::b::]*;` of an ancestor of the file → the ancestor's own `use` items made absolute (`#[allow(unused_imports)]`, so trait imports keep working) plus a glob of its public items (`unglob.py`; `self::`/`super::` heads and relative heads naming child modules are resolved, names the file imports itself win, `pub use` items are covered by the glob). 312 import lines remain in 74 moved files after pruning; a pruned import must be unused by the file and by every descendant that inherits its scope through `use super::*` (a hub such as the host `retained-mutation-authority` root keeps the imports its four child laws use).

### 2.3 `testkit` (the repo convention of `ctc`/`rtb`)
Moved laws deserialize parent types with serde, which the parents derive only under `#[cfg_attr(test, ..)]`. In the crates whose types the companions decode (draw: `model, diff, geometry, mutations, transform, style, structure, metadata, scene, editor-interaction`; layout: `model, diff, mutations, mutations-frames, mutations-content`) every `cfg_attr(test,` became `cfg_attr(any(test, feature = "testkit"),` (379 sites) and the manifests got `testkit = [..]` (optional `serde` where it was a dev-dependency only; forwarding `<dep>/testkit` along normal edges). The companions enable it on exactly those dependencies, **in `[dev-dependencies]` only** (the idiom of `📓️WAVE7-BRIEF.md`, 14:10: owner `#[cfg(any(test, feature = "testkit"))]` + `testkit = []`, consumer enables it as a dev-dependency, no self dev-dependency); a companion has no `[dependencies]` table and its root is `#![cfg(test)]`. One `#[cfg(test)]` item was gated the same way (`DrawingMutationCandidateAuthority::try_new_from_pool`, new `host` feature). The `ctc-scan.ts` scan over the four workspaces finds 72 cross-crate references to testkit-gated items, all sanctioned (the consumer enables the feature).

### 2.4 Visibility (ledger in section 6.2)
Tests that were children of a module read its private items. Where a moved law names one, the item became `pub` (56 items: modules of the editor root and of the properties panel and edit-selection node, host `owned` modules and the three `pub(super)` candidate types, panel/clipboard/geometry helpers, 2 canvas codecs). Bare child-module paths (`selection_actions::`, `ungroup::`) inside moved laws were made absolute (`tc2-tools/postfix.py`). No `pub(crate)` field was widened by hand; field/method privacy (E0616/E0624) is left to the first real compile (section 8).

### 2.5 Registration
Per artifact workspace: `Cargo.toml` `members` (byte-sorted), `Cargo.lock` package entries (hand-written, section 7), `📋️build-owners.json` rows (17, name-sorted), the artifact `📦️packages/🦀️rust/📜️script.ts` (`testPackages` for draw and layout, which had no list; the block `packages` list). The plugin artifacts have no per-crate launch routes (`launch.json` and the seed hold no row for any of the 22 draw crates, nor for the earlier `*-tests` companions such as `dag-dag-tests`); the legacy `@semio-tech/draw-drawing-rs:test -- <filter>` routes name a project that no longer exists and were not touched.


## 3. Census: test closure before and after (`bun tc2-tools/tc2-census.ts`, native; the two wasm labels are identical row by row)

| crate | before (files / bytes / lines, ratio) | after |
|---|---|---|
| `draw-drawing-scene` | 78 / 991269 / 2412 (3.781) | 18 / 226427 / 1675 (0.864) |
| `draw-drawing-transform` | 78 / 152742 / 1806 (2.438) | 28 / 61501 / 916 (0.875) |
| `draw-drawing-editor` | 51 / 601238 / 5618 (2.294) | 14 / 239036 / 2657 (0.912) |
| `draw-drawing-style` | 69 / 122220 / 1277 (2.156) | 25 / 37447 / 529 (0.781) |
| `draw-drawing-editor-commands` | 51 / 220258 / 1743 (1.594) | 29 / 91621 / 1182 (0.906) |
| `draw-drawing-write` | 26 / 406710 / 1955 (1.551) | 15 / 189940 / 1981 (0.725) |
| `draw-drawing-editor-interaction` | 48 / 401313 / 3112 (1.531) | 24 / 213458 / 2236 (0.814) |
| `draw-drawing-model` | 46 / 302551 / 3381 (1.438) | 28 / 233243 / 2970 (0.89) |
| `draw-drawing-host` | 27 / 362035 / 4818 (1.381) | 7 / 149455 / 2318 (0.58) |
| `draw-drawing-structure` | 41 / 105552 / 946 (1.281) | 15 / 28293 / 407 (0.469) |
| `draw-drawing-geometry` | 39 / 242463 / 1948 (1.219) | 18 / 133768 / 1994 (0.563) |
| `layout-layout-mutations-frames` | 202 / 593071 / 5249 (6.313) | 29 / 113188 / 1920 (0.906) |
| `layout-layout-mutations-content` | 167 / 473802 / 4605 (5.219) | 29 / 108492 / 1828 (0.906) |
| `layout-layout-editor-commands` | 46 / 263763 / 4482 (1.438) | 32 / 168767 / 3208 (1) |
| `block-2d-editor` | 34 / 145736 / 2240 (1.063) | 30 / 143710 / 2186 (0.938) |
| `block-5d-editor` | 34 / 140156 / 2220 (1.063) | 29 / 137696 / 2151 (0.906) |

Companion crates (test closure, files / bytes / lines, ratio):

| companion | closure | mounted test files |
|---|---|---|
| `block-2d-editor-tests` | 5 / 3061 / 74 (0.156) | 4 (parent `block-2d-editor`) |
| `block-5d-editor-tests` | 6 / 3822 / 92 (0.188) | 5 (parent `block-5d-editor`) |
| `draw-drawing-editor-commands-tests` | 10 / 57575 / 672 (0.313) | 9 (parent `draw-drawing-editor-commands`) |
| `draw-drawing-editor-interaction-tests` | 8 / 82001 / 955 (0.313) | 7 (parent `draw-drawing-editor-interaction`) |
| `draw-drawing-editor-tests-2` | 8 / 51184 / 631 (0.25) | 7 (parent `draw-drawing-editor`) |
| `draw-drawing-editor-tests` | 13 / 201597 / 2422 (0.769) | 12 (parent `draw-drawing-editor`) |
| `draw-drawing-host-tests` | 10 / 185313 / 2557 (0.707) | 9 (parent `draw-drawing-host`) |
| `draw-drawing-model-tests` | 9 / 35919 / 499 (0.281) | 8 (parent `draw-drawing-model`) |
| `draw-drawing-scene-tests` | 15 / 167531 / 887 (0.639) | 14 (parent `draw-drawing-scene`) |
| `draw-drawing-structure-tests` | 5 / 41486 / 578 (0.158) | 4 (parent `draw-drawing-structure`) |
| `draw-drawing-style-tests` | 9 / 59529 / 820 (0.281) | 8 (parent `draw-drawing-style`) |
| `draw-drawing-transform-tests` | 10 / 69421 / 970 (0.313) | 9 (parent `draw-drawing-transform`) |
| `layout-layout-editor-commands-tests` | 13 / 95778 / 1321 (0.406) | 12 (parent `layout-layout-editor-commands`) |
| `layout-layout-mutations-content-tests-1` | 18 / 110959 / 1549 (0.563) | 17 (parent `layout-layout-mutations-content`) |
| `layout-layout-mutations-content-tests-2` | 17 / 103358 / 1429 (0.531) | 16 (parent `layout-layout-mutations-content`) |
| `layout-layout-mutations-frames-tests-1` | 16 / 133328 / 1790 (0.509) | 15 (parent `layout-layout-mutations-frames`) |
| `layout-layout-mutations-frames-tests-2` | 15 / 128142 / 1715 (0.489) | 14 (parent `layout-layout-mutations-frames`) |

## 4. Per crate

| crate | cause | change |
|---|---|---|
| `scene` (3.78) | 47 fixtures (602 KB) + 14 unit laws (163 KB) on a 226 KB lib | all 14 law files to `scene-tests` (their cross-test helper `preparation::tests::close_*` moves with them); `raster::IDENTITY` pub; `testkit` extended (serde) |
| `transform` (2.44) | 41 fixtures; 9 mutation laws importing `…-mutations` | 9 laws to `transform-tests` |
| `editor` (2.29) | 18 fixtures + 20 test files (247 KB) on a 239 KB lib | 19 files to `editor-tests` (shared unit tree + archive-load, 12 files) and `editor-tests-2` (7 independent laws); the 648-byte `identity_test` helper stays a `testkit` module of the parent (the `ctc` gate) because the editor-commands laws name it; the job and owner laws needed their types made `pub` (section 6.2) |
| `style` (2.16) | 36 fixtures; 8 mutation laws | 8 laws to `style-tests` |
| `editor-commands` (1.59) | 13 fixtures + 9 unit laws on a 29-file lib | 9 laws to `editor-commands-tests` (it now depends on `editor[testkit]` for `identity_test`: a dependency the in-crate laws already named but could not have) |
| `write` (1.55) | 11 fixtures (218 KB) | run-time reads only (20 sites); stays in place |
| `editor-interaction` (1.53) | 22 fixtures + 8 unit laws on a 183 KB lib | 7 laws to `editor-interaction-tests`; the white-box geometry-session law (thread-local registry) stays in place, with run-time fixtures (24 files, 0.81) |
| `model` (1.44) | 10 fixtures + 10 unit laws | 8 laws to `model-tests` (schema unit law 41 KB and sqlite snapshot law stay in place); 6 inline-module fixtures read at run time |
| `host` (1.38) | 11 fixtures + 9 retained-mutation-authority laws (181 KB) | the law tree to `host-tests`; 3 `owned` modules `pub mod`; `testkit` feature for one `cfg(test)` constructor |
| `structure` (1.28) | 22 fixtures | 4 laws to `structure-tests` (they import the aggregate) |
| `geometry` (1.22) | 21 fixtures | run-time reads only (26 sites); stays in place |
| `layout mutations-frames` (6.31) | 144 fixtures + 37 laws | 29 fixture laws to `frames-tests-1/-2`; 8 tiny unit laws stay (29 files, 0.91) |
| `layout mutations-content` (5.22) | 105 fixtures + 33 laws | all 33 laws to `content-tests-1/-2` (29 normal files + 13 unit laws would exceed 32) |
| `layout editor-commands` (1.44) | 12 unit laws on a 32-file lib | 12 laws to `editor-commands-tests`; it depends on `layout-layout-editor[testkit]` (the laws use its `unit_tests::context`; the in-crate laws could not name it); 2 canvas codecs pub |
| `block-2d-editor` / `5d` (1.06) | 8 production-embedded schema projections + 7/8 test files | 4/5 small public-surface laws to `block-2d/5d-editor-tests` (30/29 files left) |

## 5. Verification (every line names the command and what it returned)

| check | command (from `F`) | result |
|---|---|---|
| census, native + both wasm labels, 33 crates | `tc2-tools/census.sh final4 manifests-after.txt` (`bun tc2-census.ts`, copy of the `tc1`/`rtb` census over `readRustSourceClosureV1`) | 27,891 units, **0 error units, 0 over-limit test units, 0 over-limit dev-dependency rows**; wasm rows identical to native for all 33 crates; test-only embedded inputs 498 files / 1,821,120 B -> 0 |
| test-name multiset | `python3 tc2-tools/testnames.py` (`ttc` closure walk, `#[test]`/`async_test` fn names) | 1,065 tests / 682 names in the 16 pre-edit closures (snapshot `tc2-snap/base`) = 1,065 / 682 now in the 16 crates plus the 17 companions: IDENTICAL multiset |
| fixtures byte-identical | `python3 tc2-tools/ttc/ttc.py hashes base <crates>` | 169 embedded or raw inputs of the baseline closures of the 16 crates: 0 changed or missing (fixture files were never edited) |
| run-time reads resolve | inline script over the census input dumps | 586 `runtime_fixture_*` reads in 141 files, all resolve below their crate manifest dir, 0 missing |
| rustfmt | `python3 tc2-tools/fmtcheck.py` (`rustfmt +nightly-2026-07-20 --emit stdout --config skip_children=true`); companion roots `--check` | 367 touched or new `.rs` files parse (0 failures); the 17 new roots are formatted |
| every moved file reached exactly once | `python3 tc2-tools/dupmounts.py` (module-tree walk from every lib root of the four workspaces) | 93 lib roots, 170 moved files, each reached by exactly its companion; 0 test files reached by more than one crate |
| manifests | `bun tc2-tools/manifestcheck.ts` | 30 manifests parse; every `{ workspace = true }` key exists in `workspace.dependencies`; path deps resolve; every `testkit` feature forwards only to declared normal dependencies or `dep:` of an optional one; each companion is a member exactly once |
| lock | script over the four `Cargo.lock` | package lists still sorted; every dependency of the 17 new entries is a package of the lock except `semio-framework-io-ports` (see 7) |
| owners / taxonomy / path length | `owners-tools/scan.ts` and `validate.ts` (admission reader: `registry valid, owners: 3134`); `tc2-tools/tax.ts` over 51 new dirs; script | 17 owner rows (sorted, manifests exist), 0 draw/layout/block packages missing; 0 unresolved directory kinds (`🥣️companion`, `🍯️companion-1`, `🍰️companion-2` under `🧪️tests`); longest new file path 235 bytes (limit 240) |
| cross-crate `cfg(test)` items | `ctc-tools/ctc-scan.ts --manifests <97 manifests of the four workspaces>` | 72 references to testkit-gated items, 72 sanctioned (the consumer enables the feature); no plain `#[cfg(test)]` item is named across a crate boundary (`tc2-tools/cfgtestitems.py`: one hit, gated, see 2.3) |
| static path/privacy scans | `tc2-tools/pathcheck2.py`, `assoccheck.py`, `unglob`/`mkcomp` gap lists | every `parent::path` a moved file imports reaches a `pub` item/module or is the same unresolved path text the pre-edit file already had (section 7); 0 private associated functions called from outside; 0 gaps left in the generator |
| union oracle (companion-aware, test cfg) | `tc2-tools/orc.sh <family> v8 test` (v7 where the family was unchanged by the last fixes) (`rustc +nightly-2026-07-20 --emit=metadata` over the parent, its companions and the first-party crates of the workspace they use, stubs for everything else) | table below |
| layout gate on the applied state | `bun ./📜️script.ts new crates <pkg> --check-applied --base=f856dff0813` | table below |
| `lib-wiring-budget` | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library) | 1 pass / 2 fail, all failing roots foreign (sequence, animate presentation, forms, shooting, puzzle 2d and 5d support, flow-brep mesh, stdio semio graph); no draw, layout or block root |
| TS laws that read these trees | `bun test` of `🧰️owned/🧪️tests/📋️native-owner/🟦️.ts`; the two `editor` tests that read Rust roots | host native-owner 22/22; edit-selection 9/9; interaction "Select All" fails on `fn interaction_topology(`, which HEAD does not contain either |

### 5.1 Oracle (test cfg; the union is parent + companions + the same-workspace crates they use)

| family | crates in the union | test cfg: cur / base errors | NEW | GONE | NEW rows (all) |
|---|---:|---|---:|---:|---|
| block-2d-editor + tests (`v7`) | 12 | 597 / 739 | 0 | 142 | - |
| block-5d-editor + tests (`v7`) | 11 | 506 / 640 | 0 | 134 | - |
| draw model + model-tests (`v7`) | 16 | 1249 / 2152 | 1 | 904 | 1x E0432 unresolved import `_` |
| layout editor-commands + tests (`v7`) | 13 | 1400 / 2426 | 5 | 1031 | 4x E0432 unresolved import `_`; 1x E0433 cannot find `_` in `_` |
| layout mutations-frames + 2 companions (`v7`) | 8 | 970 / 1602 | 0 | 632 | - |
| layout mutations-content + 2 companions (`v7`) | 10 | 1309 / 1993 | 0 | 684 | - |
| draw scene + scene-tests (`v7`) | 13 | 1466 / 2221 | 8 | 763 | 7x E0574 expected struct, variant or union type, found enum `_`; 1x E0432 unresolved import `_` |
| draw editor-commands + tests (`v8`) | 20 | 2242 / 3707 | 0 | 1465 | - |
| draw editor-interaction + tests (`v8`) | 15 | 1431 / 2366 | 2 | 937 | 2x E0432 unresolved import `_` |
| draw host + host-tests (`v8`) | 14 | 1219 / 2017 | 3 | 801 | 3x E0432 unresolved import `_` |
| draw editor + 2 companions (`v8`) | 21 | 2574 / 3688 | 3 | 1117 | 3x E0432 unresolved import `_` |
| draw transform + tests (`v7`) | 11 | 996 / 1720 | 2 | 726 | 2x E0432 unresolved import `_` |
| draw style + tests (`v7`) | 11 | 1031 / 1720 | 2 | 691 | 2x E0432 unresolved import `_` |
| draw structure + tests (`v7`) | 11 | 976 / 1720 | 2 | 746 | 2x E0432 unresolved import `_` |

Reading: `cur`/`base` are all error rows of the union (most are stub-layer rows: external crates, derive-generated items, macro-defined names); NEW = rows whose normalised message and span are not in the baseline run (baseline = the same union built from the pre-edit snapshot, parent without companions). 34 NEW rows remain over the 14 families; none is a name the moved laws lost:
* **macro-defined names the oracle cannot expand** (`DrawingCommand` enum of the editor, the layout `LayoutCommand` enum, `app_labels!` -> `DrawingPlayLabels`, `physical_work_retirement!` -> `PathRasterRetirement`): editor 2, edint 2, lcmd 5, scene 1;
* **stub-layer enum**: `RasterImage` is rendered as an enum by the stub of `framework-pixels`, so each struct-literal site (7 rows in scene, none in a moved file's own logic) errors in the union and in the baseline alike; the count differs because the moved laws now sit in another crate;
* **inherited and foreign**: `args_bridge` has no module anywhere (the pre-edit file imports it too); scene `testkit` `macro_export` close macros (`draw_scene_physical_turn/close`, a peer's `ctc` module, 2 rows in each of host, transform, style, structure); the new `host-digest` crate's own `use store::..` (peer carve, 1 row in host); `🔺️diff` crate root `framework_schema` (model, a peer's carve, file identical to the base commit).
Two earlier waves of NEW rows were real and are fixed: (1) `x:crate::T` / `x:super::T` sites after a type-ascription colon were skipped by the path rewriter (24 sites) and the dependency-root scan (one missing dev-dependency on `scene` in `editor-tests-2`); (2) a hub test file lost imports that only its child laws use (361 rows in host), now covered by pruning against the descendants. `serde` attribute rows of the first draw runs (lib cfg with `testkit` active) are gone.

### 5.2 Layout gate (`--check-applied`, native and `--test` lanes)

| package | crates (base / combined) | native: base / combined / onlyCombined | `--test`: base / combined / onlyCombined | exit |
|---|---|---|---|---|
| block 2d | 15 / 16 | 1490 / 1495 / 0 | 2052 / 2054 / 0 | 0 |
| block 5d | 18 / 19 | 1860 / 1865 / 0 | 2517 / 2520 / 0 | 0 |
| layout | 17 / 22 | 2578 / 2508 / 0 | 6064 / 5766 / 0 | 0 |
| draw | 22 / 32 | 4654 / 4659 / 0 | 8100 / 8511 / 17 | 1 (onlyCombined rows) |

## 6. Ledgers

### 6.1 `testkit` features (manifest line `testkit = [..]`)
Draw: `model, geometry, diff, mutations, transform, style, structure, metadata, scene, editor-interaction, host`. Layout: `model, diff, mutations, mutations-frames, mutations-content`. 379 `cfg_attr(test,` sites became `cfg_attr(any(test, feature = "testkit"),`. Each companion enables the feature (as a dev-dependency) on its parent, on the editor crate where it names `identity_test`/`unit_tests::context` (`draw editor`, `layout editor`) and on the serde sets above.

### 6.2 Visibility widened to `pub` (every item below was private, `pub(super)` or `pub(crate)`)
Total: 56 items (file | declaration).

* `✳️any/✏️editor/🖼️canvas/🦀️.rs \| pub fn decode_base64(value: &str) -> Result<Vec<u8>, ()> {`
* `✳️any/✏️editor/🖼️canvas/🦀️.rs \| pub fn base64_encode(bytes: &[u8]) -> String {`
* `✳️any/✏️editor/🎮️commands/🎛️edit-selection/🦀️.rs \| pub mod ungroup;`
* `✳️any/✏️editor/📋️clipboard/🦀️.rs \| pub mod assets;`
* `✳️any/✏️editor/📋️clipboard/🦀️.rs \| pub fn world(document:&DrawingSnapshot,id:&str)->Result<[f64;6],ClipboardError> {`
* `✳️any/✏️editor/📋️clipboard/🦀️.rs \| pub fn validate(packet:&DrawingClipboard)->Result<(),ClipboardError> {`
* `✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs \| pub mod selection_actions;`
* `✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs \| pub const ROOT: &str = "drawing-inspector";`
* `✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs \| pub fn node_row(layer_id: &(impl std::fmt::Display + ?Sized), index: usize, segment: &PathSegment, join_target`
* `✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs \| pub fn fill_controls(layer_id: &(impl std::fmt::Display + ?Sized), fill: Option<&FillStyle>, disabled: bool, l`
* `✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs \| pub fn stop_row(layer_id: &(impl std::fmt::Display + ?Sized), index: usize, stop: &semio_s_artifact_draw_drawi`
* `✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs \| pub enum LayersRow<'a> {`
* `✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs \| pub fn layers_rows<'a>(document: &'a DrawingSnapshot, labels: &DrawingPlayLabels) -> Vec<LayersRow<'a>> {`
* `✳️any/🧬️schema/🎬️scene/📷️raster/🦀️.rs \| pub const IDENTITY:[f64;6]=[1.0,0.0,0.0,1.0,0.0,0.0];`
* `✳️any/✏️editor/🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs \| pub type App=semio_framework_plugin::EditorApp<DrawingPlayApp>;`
* `✳️any/✏️editor/🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs \| pub struct Work {workspace:Option<ControlledRetirement<Workspace>>,identity:Option<usize>,complete:bool,closin`
* `✳️any/✏️editor/🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs \| pub fn new()->Self {Self {workspace:None,identity:None,complete:false,closing:false}}`
* `✳️any/✏️editor/🎮️commands/📤️export-document/🧵️export/🦀️.rs \| pub struct Work{read:Option<store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>,read_close:Option<B`
* `✳️any/✏️editor/🎮️commands/📤️export-document/🧵️export/🦀️.rs \| pub fn new(read:store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>,birth_grant:RetainedCloneGrant,h`
* `✳️any/✏️editor/🎮️commands/📥️import-image/🧵️admission/🦀️.rs \| pub type App=semio_framework_plugin::EditorApp<DrawingPlayApp>;`
* `✳️any/✏️editor/🎮️commands/📥️import-image/🧵️admission/🦀️.rs \| pub struct Work {workspace:Option<ControlledRetirement<Workspace>>,identity:Option<usize>,complete:bool,closin`
* `✳️any/✏️editor/🦀️.rs \| pub fn drawing_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {`
* `✳️any/✏️editor/🦀️.rs \| pub use gesture_owners::*;`
* `✳️any/✏️editor/🦀️.rs \| pub use gesture_jobs::*;`
* `✳️any/✏️editor/🦀️.rs \| pub use bounded_commands::*;`
* `✳️any/✏️editor/🦀️.rs \| pub mod export_job;`
* `✳️any/✏️editor/🦀️.rs \| pub mod image_job;`
* `✳️any/✏️editor/🦀️.rs \| pub mod path_job;`
* `✳️any/✏️editor/🦀️.rs \| pub mod completion;`
* `✳️any/✏️editor/📋️clipboard/🧵️job/🦀️.rs \| pub mod metadata;`
* `/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs \| pub mod mutation_candidate;`
* `/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs \| pub mod store_initialization;`
* `/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs \| pub mod initialization_turn;`
* `rtifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs \| pub enum DrawingMutationCandidatePhase {`
* `rtifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs \| pub enum DrawingContainerRebuildRole {`
* `rtifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs \| pub struct DrawingMutationCandidateAuthority {`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub const DRAWING_GESTURE_RAW_BYTES: usize = 8_192;`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub const DRAWING_GESTURE_RETAINED_BYTES: usize = 131_072;`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub struct DrawingGestureOperationOwner {`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub fn new(active_utility_id: &str, authoring_seed: &str) -> Self {`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub struct DrawingInstanceOperationOwner {`
* `✳️any/✏️editor/🖐️gesture-owners/🦀️.rs \| pub fn new() -> Self {`
* `✳️any/✏️editor/⚙️gesture-jobs/🦀️.rs \| pub struct DrawingRetainedCommandDecoder {`
* `✳️any/✏️editor/⚙️gesture-jobs/🦀️.rs \| pub fn new(expected_verb: &'static str) -> Self {`
* `✳️any/✏️editor/⚙️gesture-jobs/🦀️.rs \| pub struct DrawingGestureOperationJob {`
* `✳️any/✏️editor/⚙️gesture-jobs/🦀️.rs \| pub struct DrawingGestureOperationJobFactory {`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_BOUNDED_TOOL_IDS: &[&str] = &[`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_IMAGE_RAW_BYTES:usize=89_481_584;`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_IMAGE_TOOL_IDS: &[&str] = &["importImage"];`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_IMAGE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_PATH_TOOL_IDS: &[&str] = &["editPath"];`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_PATH_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub const DRAWING_BOUNDED_PAYLOAD_SCHEMA: &str = "drawing.tool-command.v1";`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub struct DrawingWindowCommandWork {`
* `✳️any/✏️editor/🧾️bounded-commands/🦀️.rs \| pub struct DrawingBoundedCommandJobFactory {`

### 6.3 Moved import lines
312 generated `use` lines in 74 moved files (down from 644 before unused first-party imports were pruned); each carries `#[allow(unused_imports)]` because it replaces the glob `use super::*;` whose private imports reach the test through trait scope.

## 7. What is not clean, and why it is not this slice's

* **Inherited unresolved paths.** 39 `parent::path` references in moved files carry the same text the pre-edit files had (`crate::standards::v1::…::diff::*` in ten content-leaf files, `crate::editor::…`, `crate::spr::…`, `crate::mutations::…`, `crate::diff::…`, the editor `args_bridge` and `create_drawing_app`, scene `PreparedSceneCloseJob`). They fail identically in the in-crate form (the leaf production files carry the same imports), i.e. they are carve leftovers for `bd7`/the rollout slices, not regressions. Macro-defined names (`app_labels!`, `statechart!`, the layout command enum, `macro_export` close macros of scene `testkit`) are unseen by the static scans.
* **`--check-applied` draw `--test` lane stays red** (17 `onlyCombined` rows, down from 49; native lane 0; `base` side also fails at HEAD). All 17 are host retained-mutation-authority laws naming items of the peers' carved `host-arena`/`host-digest` crates that are private there (`DrawingAssetBoundsCursor`, `DrawingMutationRetirementFactory`, `DrawingMutationArenaPoolBootstrap`, `…BootstrapAdmission`, `…ProcessState`, `DRAWING_MUTATION_ARENA_POOL{,_CAPACITY}`) or defined nowhere (`DrawingOwnedRetirement`, `next_initialization_catalog_close_byte_demand`, `exact_native_layer_backing`); the in-crate host tests fail on the same names in the live tree, so this is a carve gap (hand-off 3), not a move regression. The `--test` combined counts of different runs are not comparable: a unit that stops at name resolution reports fewer rows than one that reaches type checking.
* **Layout `Cargo.lock` was already stale** (the workspace lock has no `semio-framework-io-ports` although `layout-model` and the frames/content crates depend on it): the new entries name it anyway.
* **Parents keep dev-dependencies that only the moved laws used** (e.g. `png`, `trace`, `ui-scene`, `serde_json`, `async-macros` in the editor/scene/host manifests). They were not removed because the `Cargo.lock` is edited by hand; drop them with the first lock refresh.
* **Hand-written lock entries.** No cargo ran. First compile: run `bun repo cargo ... update --workspace --offline` per workspace; expect no change except the stale `io-ports` row.
* **`layout-layout-editor-commands`** is at 32/32 files in its lib alone; its test closure equals its lib closure (ratio 1.00). Any new lib file needs a carve.
* **Field and method privacy** (E0616/E0624) of the widened types is only partly visible to the stub oracle: `Work`/`Workspace` fields in the job laws, the owner types in `gesture-owners` and `gesture-jobs`, `Registry`-style white-box state. The scans (`assoccheck.py`) found 0 private associated functions; fields are not scanned.
* **Companion test ids.** Companion crates carry the old module nest, so libtest ids keep their module path under the companion crate name instead of the parent crate name; `bun repo run ...:test -- <filter>` filters on the module path continue to match.
* **`testPackages`** of the draw and layout `📜️script.ts` now lists the root package plus the companions only (draw/layout had no list; the parents' own remaining unit laws were not run by the route before either).

## 8. Hand-offs

1. First real compile, per workspace under the fleet mutex (`COMPILE ADMITTED` required), one launcher call per line, `--keep-going`:
```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- update --workspace --offline --manifest-path "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml"
... -- check -p semio-s-artifact-draw-drawing-model-tests -p semio-s-artifact-draw-drawing-transform-tests -p semio-s-artifact-draw-drawing-style-tests -p semio-s-artifact-draw-drawing-structure-tests -p semio-s-artifact-draw-drawing-scene-tests -p semio-s-artifact-draw-drawing-editor-commands-tests -p semio-s-artifact-draw-drawing-editor-interaction-tests -p semio-s-artifact-draw-drawing-editor-tests -p semio-s-artifact-draw-drawing-editor-tests-2 -p semio-s-artifact-draw-drawing-host-tests --tests --keep-going --message-format=short --manifest-path "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml"
```
and the same with the layout companions (`-p semio-s-artifact-layout-layout-{editor-commands,mutations-frames,mutations-content}-tests*`, layout `Cargo.toml`) and the block companions (`-p semio-s-artifact-block-2d-editor-tests`, `-5d-`). Then `test -p <companion> --lib`, and `check -p <parent> --lib` (default features) to prove the lib cfg did not move.
2. Expect first: E0616/E0624 on the `Work`/owner types; `serde` derive gaps for types outside the testkit sets (`host-arena`, `viewer`, `io` crates); the inherited unresolved paths of section 7; the missing `io-ports` lock row.
3. Make the 7 private `host-arena` items above `pub` (or `testkit`-gated) and restore the three undefined names, then the draw `--test` lane should reach 0 rows; they belong to the host-arena carve owner.
4. Next smallest unit if a new law is added: `block-2d-editor` has 2 free files, `editor` 14/32 files but 0.91 bytes (a law must go to a companion).

## 9. Changed files

Created: 17 companion roots `<node>/🧪️tests/<🥣️companion|🍯️companion-1|🍰️companion-2>/🦀️.rs` and 17 manifests `…/📦️packages/🦀️rust/Cargo.toml` (draw: transform, style, structure, model, scene, editor-commands, editor-interaction, editor ×2, host; layout: editor-commands, mutations-frames ×2, mutations-content ×2; block: 2d, 5d), `F/RESULT-TEST-CLOSURE-T2.md`, `F/tc2-tools/` (originals of every edited file in `orig/`, `specs/`).
Edited (about 375 existing files; `F/tc2-tools/orig/index.tsv` lists each with its pre-edit copy): 182 test `.rs` files (fixture reads, imports), 163 production `.rs` files (mount removal, `cfg_attr` gates, 56 `pub` widenings), 20 `Cargo.toml` (testkit features), 4 workspace `Cargo.toml` (members) and 4 `Cargo.lock`, 4 artifact `📜️script.ts`, `📋️build-owners.json`.
