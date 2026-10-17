# Result CENSUS4-PKG-45 (prefix `c4p45`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `M` = `🧰️framework/🔨️modules`. Tools: `F/c4p45-tools/` (descendant of `c4p37-tools`). Logs: `F/🗑️generated/c4p45-*`. Sandbox/stage root: `/Users/ueli/.cache/c4p45/`.

## 0. Verdict

**DONE (static evidence; real compile not admitted).** All 7 files are split into 39 cohesive children in 39 new folders (largest child 534 lines, parents 10 to 174 lines). Lossless 7 of 7 on the live tree, region markers balanced per file (46 of 46 files), rustfmt parses 46 of 46 files, every child mounted exactly once, taxonomy 39 of 39, RGI emoji, sibling emoji unique, longest path 119 bytes. The scoped patched differential on the live tree is identical in every group and configuration (native, `--test`, wasm32-wasip2 lib/test, wasm32-unknown-unknown lib/test; 7 groups x 6 configurations). The full-crate typeck oracle found one real regression on its first run (`Scene::vello_scene`, original `pub(super)`, now `pub(crate)`) and shows 0 new and 0 gone errors on the final tree (lib 2195 = 2195, test 3830 = 3830). The census keeps all seven crates inside the source gate, lib and `cfg(test)` (binding case: `semio-framework-dsl-record` test closure 30 of 32 files and 258,436 of 262,144 bytes). Six source-text readers were re-pointed to module-tree reads. No `COMPILE ADMITTED` line existed in `📓️FLEET.md` (14:53), so the real `bun repo cargo` check is queued (section 9).

## 1. Found state (fresh slice)

- No earlier c4p45 work. The 7 files are byte-identical to `HEAD b84bf526c7b` (git blob hashes equal) and match the census line counts (8,647 lines). Snapshots: `F/c4p45-tools/orig/<tag>.rs`, hashes in `F/c4p45-tools/orig/SHA256`.

| tag | file (tail under `M/`) | lines | SHA256 (first 16) | owning crate | mount chain |
| --- | --- | ---: | --- | --- | --- |
| pix | `🔲️pixels/🦀️.rs` | 1,417 | `79a79c3e1390f01d` | `semio-framework-pixels` | `mod component` of the glue root `📦️packages/🦀️rust/🦀️.rs` |
| gdt | `🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` | 1,056 | `5437e8fc78c6b2b0` | `semio-framework-graph-dsl` | test module of lib root `🕸️graph/🗣️dsl/🦀️.rs` |
| rnd | `🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs` | 1,237 | `815ea76e2be7528c` | `semio-framework-canvas-scene` | module of lib root `🖼️canvas/🎬️scene/🦀️.rs` |
| dfl | `🗜️deflate/🦀️.rs` | 1,099 | `3f97d17a069f6f76` | `semio-framework-deflate` | `mod component` of the glue root |
| drv | `🗣️dsl/🧬️schema/✨️derive/🦀️.rs` | 1,421 | `84728ad68dd801b4` | `semio-framework-dsl-record-derive` | the file is the lib root (proc-macro) |
| sch | `🗣️dsl/🧬️schema/🦀️.rs` | 1,134 | `24acd95c554b0727` | `semio-framework-dsl-record` | the file is the lib root |
| pnt | `🗺️surface/🎨️paint/🦀️.rs` | 1,283 | `cdea5bd3e18f31ee` | `semio-framework-surface-paint` | the file is the lib root |

Baseline census (`census-own.ts`, files / bytes / active lines; log `🗑️generated/c4p45-census-own-before.log`):

| crate | lib | lib + cfg(test) |
| --- | --- | --- |
| pixels | 13 / 203,607 / 3,204 | same |
| graph-dsl | 7 / 97,086 / 2,290 | 10 / 155,524 / 3,548 |
| canvas-scene | 7 / 122,625 / 2,527 | 21 / 177,172 / 3,220 |
| deflate | 4 / 65,999 / 1,397 | 13 / 108,542 / 1,984 |
| dsl-record-derive | 2 / 125,648 / 1,474 | 8 / 139,364 / 1,582 |
| dsl-record | 3 / 111,574 / 1,443 | 25 / 256,522 / 2,873 |
| surface-paint | 3 / 74,064 / 1,306 | 10 / 159,119 / 2,262 |

Binding constraint: dsl-record `cfg(test)` is at 256,522 of 262,144 bytes and 25 of 32 files.

## 2. What was cut

Children sit next to their parent as `<emoji>️<kebab>/🦀️.rs`, wired with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;` (or `use`/`pub(crate) use` by the best visibility of the child's named items; none-compat). Each child starts with an `//!` docstring whose first character is the folder emoji, then `use super::*;` (omitted for the constants-and-types-only deflate child). Mounts of existing modules (`#[path]` to `🧪️tests`, `🫳️measure`, `🎒️zip`, `✍️gesture`, ...) stay in the parent because a `#[path]` is relative to the declaring file. Tool: `F/c4p45-tools/c4p45.py` (descendant of the c4p37 splitter with `inner` file promotion of an inline module, the c4p35 `include!` fragment builder, `external_ranges`, `pre`/`patch_parent` hooks), specs in `F/c4p45-tools/specs/`.

| tag | file | before | parent after | children (lines) |
| --- | --- | ---: | ---: | --- |
| pix | `🔲️pixels/🦀️.rs` | 1,417 | 174 | 🗜️deflate 534 (inline `mod deflate` promoted to a file), 🧱️png-chunks-and-header 127, 🧮️png-filters-and-samples 178, 📤️png-encoding-core 74, 🔄️scanline-decoder 362 |
| gdt | `🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` | 1,056 | 10 | `include!` fragments: 🧭️match-parse-and-run-laws 300, 🕸️graph-and-error-laws 239, 🔤️lexer-and-language-service-laws 208, ⚙️parser-and-executor-laws 311 |
| rnd | `🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs` | 1,237 | 102 | 🗒️draw-list 295 (inline `pub mod draw_list` promoted), 🔧️kurbo-boundary 67, 🖊️stroke-style 98, 🎨️paint-values 180, 📦️shape-and-command 241, 🎬️scene-and-retirement 294 |
| dfl | `🗜️deflate/🦀️.rs` | 1,099 | 44 | 📋️errors-and-tables 84, 🔁️bit-io-and-huffman 175, 🧱️inflate-state 213, 🚰️inflater 441, 📤️deflate-encoder 191 |
| drv | `🗣️dsl/🧬️schema/✨️derive/🦀️.rs` | 1,421 | 70 | 🏷️attribute-parsing 130, 🧩️type-shape 131, 🏗️record-field-codegen 406, 🧹️lifecycle-codegen 133, 🚦️emitters 306, 🌿️variant-helpers 130, 🪢️receiving-and-projection 90, 📚️borrowed-owner-codegen 82 |
| sch | `🗣️dsl/🧬️schema/🦀️.rs` | 1,134 | 72 | 🧭️cursor-and-expressions 301, 🗺️shape-parsing 235, 📜️dsl-value-and-wire 176, 📒️records-and-tables 272, 🔎️canonicalize-and-language-service 111 |
| pnt | `🗺️surface/🎨️paint/🦀️.rs` | 1,283 | 85 | 📄️document-json 205, ⚠️errors-and-pixels 112, 🖥️raster-host 363, 🎯️picking 261, 🧹️host-retirement 94, 🌐️wasm-session 211 |

Specific handling (each verified in the live text):

- **Inline modules promoted to files (pix, rnd).** `mod deflate` (536 lines) and `pub mod draw_list` (313 lines) keep their module path; the body is dedented by four and becomes the child file (pix: the inner `//!` doc is the file doc; rnd: a new `//!` doc, the outer `///` docs stay on the `#[path] pub mod draw_list;` mount). The `testkit` module of pix still reaches `super::super::deflate`. Their outer region markers stay in the parent around the mount.
- **Test file (gdt, 73 `#[test]` fns).** Four `include!` fragments spliced back into the same module, so test paths, visibility and `use super::*;` scope are unchanged (test-name multiset 73 = 73). Each fragment starts with an outer `///` line whose first character is the unique emoji. The one relative `include_str!("../../../🛂️manifest/...")` was re-based to `../../../../` in its fragment.
- **Proc-macro root (drv).** The seven `#[proc_macro*]` entry points stay in the crate root (rustc requires it); all children are private, so nothing but proc macros is exported. The file had two orphan markers (`//#endregion 🔖️Attrs`, `//#endregion 🔖️VariantHelpers` without openers); the missing `//#region` lines were inserted before `struct ContainerAttrs` and `fn to_kebab` (declared as `pre_extra`) so that every new file is balanced (orig 2 openers / 4 closers, new 4 / 4).
- **Misplaced doc (sch).** A two-line `///` doc about the table list printer sat in front of `//#endregion 🔖️Table` and documented `fn base64_decode` (rustc attaches it across the comment). It was moved below the marker so that it stays with `base64_decode`, which moved to the canonicalize child (the multiset is unchanged).
- **Original `pub(super)` (rnd).** `Scene::vello_scene` was `pub(super)` (visible crate-wide because `renderer`'s parent is the crate root; used by `native_render` and `gpu-session`). In the child module `pub(super)` would shrink to `renderer`, so it was widened to `pub(crate)`, the narrowest equivalent. The other original `pub(super)` items stay in their module (`vello_backend` in the parent, `ZlibEncodeCursor` inside `mod deflate`).
- **cfg gating (rnd, pnt).** `🔧️kurbo-boundary` is mounted with the host/browser cfg of all its items (no empty module and no unused glob on wasm32-wasip2); `🌐️wasm-session` with `cfg(all(target_arch = "wasm32", not(target_env = "p2")))`; the five cfg-gated browser `use` lines stay in the parent. pnt's `picking` items are only used by the mounted unit tests, so its glob import is `#[cfg(test)] use picking::*;` (otherwise `unused import` in non-test builds).
- **`include!` in rnd.** Both `#[cfg(test)] include!("../../🧪️tests/...")` items stay in the parent (the path is relative to the file); the two included test modules were added to the widener's external text.
- **Widening.** 32 + 0 + 42 + 67 + 74 + 29 + 91 = 335 `pub(super)` insertions (items, fields, inherent members) where the name occurs in another file of the crate closure (lexical `widen2.py`, conservative; rnd also counts the promoted `draw_list` body and the included test files). Trait-impl methods are never widened. No qualified path was introduced to fix visibility.
- **Glob ambiguity scan.** pnt re-exports `canvas::*` and sch re-exports `dsl_record_emission::*` and `dsl_record_model::*`; a child item with the same name would turn the formerly shadowing local definition into an ambiguous glob pair, which the oracle's external-glob patch cannot see. `globclash.py` (child top-level names against the exported names of the external crate trees): 33 child names against 241 canvas names, 42 against 76 emission/model names, 0 clashes.
- **Restage trap.** The specs compute their `external` lists from the live parents, which now mount the children. After the install the `pnt` parent got its `#[cfg(test)]` import by a direct edit (stage dir re-synced from the live tree); no spec was re-staged after install, and `rnd` got its `pub(crate)` fix the same way.

## 3. Source-text readers (re-pointed)

`git grep -F` over the repo for each file path and for the module directory names (`*.ts`, `*.tsx`, `*.py`, `*.mjs`, `*.rs`, `*.toml`, `*.md`, `*.json`, excluding `.vscode` and `.🧬semio`), plus a scan of every TypeScript file inside the six module directories for `🦀️.rs` reads. Readers of moved text, all fixed (`readModuleTreeSource` from `@semio-tech/framework-compiler-ts/syntax/module-tree`; baseline = the same test before the split, from `F/🗑️generated/c4p45-ts-baseline.log`):

| reader | before | after | result (baseline -> after) |
| --- | --- | --- | --- |
| `os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts` (line 18, the scene renderer text) | `Bun.file(new URL(".../🖌️renderer/🦀️.rs", ...)).text()` | `readModuleTreeSource(new URL(...))` (import already present) | script exit 0 -> exit 0 |
| `os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🗜️deflate/🧪️tests/🟦️.ts` (`lower`, the deflate text) | `read(".../🔨️modules/🗜️deflate/🦀️.rs")` | `readModuleTreeSource(new URL(...,import.meta.url))` plus import | 2 pass -> 2 pass |
| `🗜️deflate/🧪️tests/🧱️ownership/🟦️.ts` (line 23) | `readFileSync(resolve(owner,"🦀️.rs"),"utf8")` | `readModuleTreeSource(resolve(owner,"🦀️.rs"))` plus import | 2 pass -> 2 pass |
| `🗣️dsl/🧬️schema/✨️derive/🧪️tests/🧱️ownership/🟦️.ts` (3 reads of `🦀️.rs`) | `read("🦀️.rs")` | `readTree("🦀️.rs")` (new helper on `readModuleTreeSource`) plus import | 7 pass, 1 fail -> 7 pass, 1 fail (same esbuild build failure) |
| `🗣️dsl/🧬️schema/🧪️tests/🧱️ownership/🟦️.ts` (the negative-spelling loop for the root, `pub struct RecordSpec`, `semio_framework_pack_json::`) | `read("🦀️.rs")` | `readTree("🦀️.rs")` (loop: root only) plus import | 11 pass, 2 fail -> 11 pass, 2 fail (the same two pre-existing failures: `RecordSpec` and the metadata package expectation) |
| `🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/🟦️.ts` (line 14, `derive`) | `read("../../✨️derive/🦀️.rs")+read("../../✨️derive/📥️owned/🦀️.rs")` | `readModuleTreeSource(resolve(import.meta.dir,"../../✨️derive/🦀️.rs"))` plus import (the tree includes `📥️owned`) | 4 pass -> 4 pass |

Probes of the nature of the failures that were already there: the schema ownership test fails at `read("🦀️.rs").includes("pub struct RecordSpec")` on the original text too (the type moved to the model crate earlier), and the second failure is the package expectation; the derive ownership test's one failure is an esbuild build error. Not caused by this slice.

Other hits that are not readers of the moved text: crate manifests (`Cargo.toml` lib paths unchanged), `#[path]` mounts of other crates to different files, docstrings and `@see` lines (including comments in `os/.../🧮️table-and-paint2d/🦀️.rs` that name `🗺️surface/🎨️paint/🦀️.rs`), the `🎒️pack/⚠️error` cause fixture (`path`/`sha256`/`predicate` for `🗜️deflate/🦀️.rs`, a fixture no code reads), raster plugin tests that `include_str!` JSON fixtures under `🗺️surface/🎨️paint/🧫️fixtures` (not Rust), and the os-owned `dsl/✨️derive` crate (a different crate).

`library` law `📏️lib-wiring-budget` (bun test): 2 pass, 1 fail, the three breaches are plugin crates of peers (`🌊️flow` 783 to 790, `🎬️sequence` 275 to 276, `🏢️model` 440 lines); none of the seven crates is subject to that law.

No compatibility re-exports: parents hold `#[path] mod x; <vis> use x::*;` wiring and the file's genuine top-level items; no alias, deprecation or shim was added.

## 4. Verification (commands, real results)

| Check | Command | Result |
| --- | --- | --- |
| Lossless line multiset (live) | `python3 c4p45-tools/live.py` (`c4p45-live.log`) | 7 of 7 `bytes-equal-to-stage` and `LOSSLESS OK`, missing 0; every added line is a docstring, `use super::*;`, `#[path]`/`mod`/`pub use` wiring, `include!`, a region marker or a declared line (`pub(crate) fn vello_scene` widening in rnd; the two `//#region` openers in drv; the `#[cfg(test)]` on pnt's picking import). The pnt stage dir was re-synced from the live tree after its direct edit (section 2), so its "equal to stage" only proves the sync; its lossless is against the pre-edit snapshot. |
| Live structure | `python3 c4p45-tools/check.py live` | `CHECKS OK`: 39 children, each mounted exactly once (0 duplicates, 0 missing), doc emoji equals folder emoji, unique among siblings and different from the root's, no clash with an existing sibling folder, folder emoji already used as a folder prefix in the repo (779 prefixes), region markers balanced per file (also the root), test-name multiset 73 = 73 (gdt), no `[DEBUG]`, longest child 534 lines, longest repo-relative path 130 bytes |
| rustfmt | `python3 c4p45-tools/fmt.py stage` before the install and `fmt.py live` after (`rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout <file>`) | 46 of 46 files parse, 0 failures, both times (the `include!` fragments are invisible to rustfmt, but they parse as files) |
| Private declarations | `python3 c4p45-tools/defscan.py <tag>` on the staged tree (before install) | 0 cross-file leftovers for pix, rnd, dfl, drv, sch, pnt; gdt reports 3 (`block_on_test`, `split_endpoint_fixture`, `list_property_fixture`), false positives: the fragments are `include!`d into one module |
| Child module name clashes | `python3 c4p45-tools/modclash.py pix rnd dfl drv sch pnt` | `modclash OK` |
| Test names | `python3 c4p45-tools/testnames.py gdt` | 73 = 73 `#[test]` fns, identical (the other six files hold none) |
| Glob ambiguity | `python3 c4p45-tools/globclash.py stage pnt ...canvas` / `... sch ...printing ...model` | 0 clashes (section 2) |
| Scoped patched differential, staged (before install) | `python3 c4p45-tools/oracle.py <tag> lib,test,p2,p2test,wasm,wasmtest` (`c4p45-oracle-<tag>-stage.log`) | identical error counts in all 42 runs; two textual-only findings fixed before install: `unused import: super::*` in dfl's tables child (`glob=False`) and `unused import: picking::*` in pnt (`reexport=False` plus the `#[cfg(test)]` import) |
| Scoped patched differential, live (final) | `C4P45_LIVE=1 C4P45_NORM=1 python3 c4p45-tools/oracle.py <tag> lib,test,p2,p2test,wasm,wasmtest` (`c4p45-oracle-<tag>-live.log`, rnd re-run after its `pub(crate)` fix: `c4p45-oracle-rnd-live2.log`) | `ORACLE <tag> CLEAN` for all 7 groups: 0 only-orig, 0 only-new in 42 of 42 runs, warnings identical |
| Taxonomy | `bun c4p45-tools/tax.ts c4p45-tools/newdirs.txt` | `dirs 39, unresolved []` |
| RGI / siblings / reuse | `node c4p5-tools/rgi.mjs c4p45-tools/newnames.txt` | `bad=0`; sibling and repo-prefix checks in `check.py` |
| Census | `bun c4p45-tools/census-own.ts <repo root>` and `zsh c4p45-tools/census.sh after` (canonical `readRustSourceClosureV1`) | table below; 0 of 7 over the gate, lib and `cfg(test)` |

Differential detail. The oracle runs `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` from the real crate lib root against a sandbox holding the ORIGINAL file, with the `extern crate` and external-glob patches applied identically to both sides, and compares error and warning multisets without path, line and column (`C4P45_NORM=1` also strips printed module paths). All seven groups stop at name resolution because their external crates are not linked (3 to 1,297 errors per configuration, identical in both trees), so privacy and type errors are not reached there; `defscan`, `globclash` and the conservative lexical widener cover that gap, and section 5 adds the full-crate typeck oracle. The one defect in this slice (`vello_scene` visibility) was found by the typeck oracle, not by the scoped differential.

Census (canonical `readRustSourceClosureV1`; gate 4,000 lines, 32 files, 262,144 bytes; files / bytes / active lines; before = baseline in section 1):

| crate | lib before | lib after | lib + cfg(test) before | lib + cfg(test) after | canonical worst ratio after (lib) |
| --- | --- | --- | --- | --- | --- |
| pixels | 13 / 203,607 / 3,204 | 18 / 203,171 / 3,241 | same as lib | same as lib | 0.810 (lines) |
| graph-dsl | 7 / 97,086 / 2,290 | 7 / 97,086 / 2,290 | 10 / 155,524 / 3,548 | 14 / 156,388 / 3,564 | 0.573 |
| canvas-scene | 7 / 122,625 / 2,527 | 13 / 123,669 / 2,572 | 21 / 177,172 / 3,220 | 27 / 178,216 / 3,265 | 0.643 |
| deflate | 4 / 65,999 / 1,397 | 9 / 68,184 / 1,451 | 13 / 108,542 / 1,984 | 18 / 110,727 / 2,038 | 0.363 |
| dsl-record-derive | 2 / 125,648 / 1,474 | 10 / 128,548 / 1,539 | 8 / 139,364 / 1,582 | 16 / 142,264 / 1,647 | 0.490 (bytes) |
| dsl-record | 3 / 111,574 / 1,443 | 8 / 113,488 / 1,481 | 25 / 256,522 / 2,873 | **30** / **258,436** / 2,911 | 0.433 (bytes) |
| surface-paint | 3 / 74,064 / 1,306 | 8 / 67,809 / 1,342 | 10 / 159,119 / 2,262 | 15 / 152,864 / 2,298 | 0.336 |

The pixels lib bytes shrink because the deflate module is dedented by four spaces; surface-paint's native lib closure shrinks because the browser session is cfg-gated out (the wasm32 closure adds its 211 lines). The only limit-touching rows: dsl-record `cfg(test)` at 30 of 32 files and 258,436 of 262,144 bytes (98.6 percent; that was 97.9 percent before), which is why its parser got 5 children rather than 7 and short docstrings; graph-dsl `cfg(test)` at 3,564 of 4,000 lines.

## 5. Full-crate typeck oracle (extra evidence)

`pos.py` (privacy and type oracle, stub layer for everything outside the family; copy in `F/c4p45-tools/pos-tools/`, `SCRATCH` moved to `/Users/ueli/.cache/c4p45`, family `families/c4p45.json` = the 7 crates, base `HEAD b84bf526c7b`, cfgs lib and test). Command: `cd F/c4p45-tools/pos-tools && ulimit -n 65536 && POS_PREFIX=c4p45 nice -n 10 python3 pos.py run families/c4p45.json --cfgs lib,test --label <a1|a2> --iters 3` (logs `c4p45-pos-a1.log`, `c4p45-pos-a2.log`, about 8 minutes each).

| run | tree | cfg | errors cur / base | NEW / GONE | privacy | trait-scope | resolve | type | other | canary |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| a1 | before the `pub(crate)` fix | lib | 2196 / 2195 | 1 / 0 | **1** | 0 | 0 | 0 | 0 | `E0308 E0616 E0624` reached on both sides |
| a1 | before the `pub(crate)` fix | test | 3831 / 3830 | 1 / 0 | **1** | 0 | 0 | 0 | 0 | reached on both sides |
| a2 | final live tree | lib | 2195 / 2195 | 0 / 0 | 0 | 0 | 0 | 0 | 0 | `E0308 E0616 E0624` reached on both sides |
| a2 | final live tree | test | 3830 / 3830 | 0 / 0 | 0 | 0 | 0 | 0 | 0 | reached on both sides |

Run a1 caught the one real regression of the split: `E0624 [canvas_scene] 🖼️canvas/🎬️scene/🦀️.rs:50 method vello_scene is private` (the original `pub(super)` method moved into a child module, section 2). The scoped differential could not see it (all groups stop at name resolution). After widening that single definition to `pub(crate)`, run a2 on the live tree shows 0 new and 0 gone errors in every category, so no missing `pub(super)` (`E0603`, `E0616`, `E0624`) and no unresolved name is introduced; the canary proves the oracle reaches type and privacy checking. Not covered: the wasm32 browser sessions of surface-paint (covered by the scoped differential only), borrowck, trait coherence against the real externals and lints; the real check in section 9 covers them.

## 6. Foreign errors and observations

- `bun test` for the schema ownership test: 2 failures and for the derive ownership test: 1 failure, identical before and after the split (section 3).
- `library` law `📏️lib-wiring-budget` fails only on plugin crates of peers (section 3).
- Peers had staged (index) edits in several files under the seven module directories (test files such as `🛬️decoding/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `package.json`); this slice read them as part of the live closure and never touched them. The typeck oracle base is `HEAD`, and both sides showed the same counts, so those edits cause no difference.
- `git` printed `daemon terminated` during some read-only calls (fsmonitor noise). No `ENFILE`/`EMFILE` occurred in any run (`ulimit -n 65536` set).

## 7. Files

- Edited (7 roots): `M/🔲️pixels/🦀️.rs`, `M/🕸️graph/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs`, `M/🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs`, `M/🗜️deflate/🦀️.rs`, `M/🗣️dsl/🧬️schema/✨️derive/🦀️.rs`, `M/🗣️dsl/🧬️schema/🦀️.rs`, `M/🗺️surface/🎨️paint/🦀️.rs`.
- Created (39 children, one `🦀️.rs` per new folder): the folders of section 2, next to their roots.
- Edited outside the package (readers, section 3): `os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts`, `os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🗜️deflate/🧪️tests/🟦️.ts`, `M/🗜️deflate/🧪️tests/🧱️ownership/🟦️.ts`, `M/🗣️dsl/🧬️schema/✨️derive/🧪️tests/🧱️ownership/🟦️.ts`, `M/🗣️dsl/🧬️schema/🧪️tests/🧱️ownership/🟦️.ts`, `M/🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/🟦️.ts` (`os` = `🧰️framework/🛍️products/💻️os`).
- No registry, manifest, `launch.json`, lock or build-owner edit was necessary: no crate, test target or command was added or renamed.
- Tools, kept: `F/c4p45-tools/` (`c4p45.py` splitter, `c4p45x.py` include builder, `specs/*.py`, `oracle.py`, `live.py`, `check.py`, `fmt.py`, `defscan.py`, `modclash.py`, `globuse.py`, `globclash.py`, `testnames.py`, `census.sh`, `census-own.ts`, `tax.ts`, `pos-tools/` copy with the family `families/c4p45.json`, `orig/` snapshots, `newdirs.txt`, `newnames.txt`). Do not re-stage after install: the specs compute their `external` lists from the live parents, which now mount the children.
- Logs, kept in `F/🗑️generated/`: `c4p45-oracle-<tag>-{stage,live}.log` (+ `-rnd-live2`), `c4p45-live.log`, `c4p45-census-*.log`, `c4p45-census-own-*.log`, `c4p45-ts-baseline.log`, `c4p45-install.log`, `c4p45-pos-{a1,a2}.log`. Sandboxes and stage trees: `/Users/ueli/.cache/c4p45/`.

## 8. Hand-offs

- Compile queue (section 9). If a real build shows a missing `pub(super)` in a child, widen that single definition line (never with a qualified path).
- The wasm32-unknown-unknown browser sessions (surface-paint `🌐️wasm-session`) and the proc-macro expansion of dsl-record-derive are only checked statically; the real `--target wasm32-unknown-unknown` check below covers them.

## 9. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 14:53 (no `cargo-mutex.lock` folder existed either, but admission is announced by the coordinator). Exact command for the compile queue, one batched call under the mutex of `📓️WAVE5-BRIEF.md` (framework workspace members need the manifest path):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-deflate -p semio-framework-pixels -p semio-framework-canvas-scene -p semio-framework-dsl-record-derive -p semio-framework-dsl-record -p semio-framework-graph-dsl -p semio-framework-surface-paint --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then `--target wasm32-wasip2` for the same set (`--lib`) and `-p semio-framework-surface-paint -p semio-framework-canvas-scene --lib --tests --target wasm32-unknown-unknown` for the browser sessions.
