# Result c6ui (census 6, packages P18 to P22: rust framework/ui)

Executor `c6ui`, Sonnet 5.5, foreground, no sub-agents, no cargo/bun repo/nx/bazel, no modifying git command. `C` = this ticket folder. Tools: `C/c6ui-tools/` (descendant of `c6db2-tools`, `c6db1-tools`, `F/c5b-tools`). Logs: `C/🗑️generated/c6ui-*` (nothing deleted). Sandboxes and stages: `/Users/ueli/.cache/c6ui/`.

## 0. Verdict

- All 31 files of P18 to P22 are split and installed on the live tree. Every root is now at most 487 lines, every new child at most 506 lines (target 600 met everywhere). 202 new folders (197 module/fragment children, 5 row modules for `proj`).
- Static evidence only (no compile): lossless line-multiset oracle, structural oracle, definition scan, scoped typeck differential (`pos` oracle, HEAD against the working tree), taxonomy resolution, RGI check, path lengths, canonical closure census. Details in sections 2 and 3.
- Nothing in this slice could be compiled on windows (two inline windows modules were moved) and the `semio-framework-ui` crate is still far above its own 12,000 line gate for the all-features closure (pre-existing, section 5).

## 1. Verdict table per file

`Before` is the live line count when the snapshot was taken (equal to the census 6 counts, except `proj` which a peer touched by one line before the split). `Root` is the line count of the original path after the split. Mechanism: `mod` = module split via `#[path]` children (`modsplit.py`), `frag` = `include!` fragments of a flat test root (`frag.py`), `inner` = inline `mod` converted to a file module with children.

| Pkg | tag | file (tail under `ui/`) | before | root | children (max) | mechanism | crate |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
| P18 | eng | `⌨️tui/⚙️engine` | 1,042 | 146 | 5 (239) | mod; `impl Tui` cut into 5 impl blocks at method boundaries | `semio-framework-ui` (tui) |
| P18 | vtu | `⌨️tui/📟️vt/🧪️tests/🔬️unit` | 1,204 | 17 | 7 (244) | frag, 79 tests to 79 | `semio-framework-ui` (tui, test) |
| P18 | ansi | `⌨️tui/🔡️ansi` | 1,035 | 46 | 5 (408) | mod; `impl AnsiParser` cut in 2 | `semio-framework-ui` (tui) |
| P18 | pty | `⌨️tui/🚇️pty` | 1,091 | 487 | 3 + shell (350) | inner (`windows_impl`); unix and readiness stay inline | `semio-framework-ui` (tui-terminal) |
| P18 | nodes | `🌳️retained/🌳️tree/🍃️nodes` | 1,198 | 60 | 5 (346) | mod; `impl UiTree` cut in 3 at region boundaries | `semio-framework-ui-retained-tree-tree` |
| P18 | plane | `🎬️scene/🎬️scenes/🖼️plane` | 1,142 | 72 | 5 (382) | mod, one scene per child (macro wire struct stays with its scene) | `semio-framework-ui-scene-plane` |
| P18 | mesh | `🎬️scene/📐️math/🕸️mesh` | 1,097 | 70 | 5 (374) | mod | `semio-framework-ui-scene-mesh` |
| P18 | rscene | `🖌️render/🎬️scene` | 1,057 | 56 | 4 (356) | mod | `semio-framework-ui-render` |
| P19 | screen | `⌨️tui/📟️vt/🧱️screen` | 1,233 | 32 | 6 (270) | mod; `impl VtScreen` cut in 5; `nest` + `rebase_super` (49 `pub(super)` members, 6 `super::palette` paths) | `semio-framework-ui` (tui) |
| P19 | action | `🎬️action` | 1,282 | 53 | 5 (347) | mod | `semio-framework-ui-action` |
| P19 | w3dm | `🎬️scene/📏️world3d-modelling` | 1,278 | 182 | 6 (222) | mod; macro `string_enum!` and the codec helpers stay in the root; the 5 macro-generated enums are re-exported by hand | `semio-framework-ui-scene-modelling` |
| P19 | tcomp | `🔤️typography/🧩️component` | 1,259 | 85 | 8 (240) | mod; `impl FontAtlas` cut in 4 | `semio-framework-ui-text` |
| P19 | window | `🖥️host/🪟️window` | 1,459 | 344 | 2 + inner `browser` 5 + shell (229) | mod + inner (`browser`, `impl CanvasHost` cut in 3); `native` stays inline | `semio-framework-ui-host` |
| P19 | ccomp | `🧬️contract/🧩️component` | 1,368 | 91 | 6 (297) | mod | `semio-framework-ui-contract-components` |
| P19 | proj | `🧬️contract/🧬️schema/🟦️projection` | 1,230 | 180 | 5 (343) | row-table split (see 4.1) | `semio-framework-ui-contract-typegen` |
| P20 | backend | `⌨️tui/🔌️backend` | 1,470 | 162 | 2 + inner `native_unix` 2 + inner `native_windows` 3 + 2 shells (250) | mod + two inner modules | `semio-framework-ui` (tui-terminal) |
| P20 | shader | `🖌️render/✨️shader-contract` | 1,479 | 55 | 5 (387) | mod | `semio-framework-ui-render-shader-contract` |
| P20 | cursor | `🧠️runtime/⏭️cursor` | 1,584 | 46 | 5 (506) | mod; `impl SurfaceReconcileCursor` cut in 3 (the 498-line `step` stays one method) | `semio-framework-ui-runtime-reconcile-cursor` |
| P20 | recon | `🧠️runtime/♻️reconcile` | 1,562 | 95 | 8 (299) | mod | `semio-framework-ui-runtime-reconcile` |
| P20 | values | `🧬️contract/🌱️values` | 1,479 | 125 | 5 (353) | mod | `semio-framework-ui-contract-values` |
| P20 | limits | `🧬️contract/🛡️limits` | 1,493 | 88 | 6 (432) | mod | `semio-framework-ui-contract-validation` |
| P21 | tuiu | `⌨️tui/🧪️tests/🔬️unit` | 1,593 | 20 | 7 (273) | frag, 102 tests to 102 | `semio-framework-ui` (tui, test) |
| P21 | treec | `🌳️retained/🌳️tree/🔀️reconcile` | 1,941 | 103 | 9 (289) | mod | `semio-framework-ui-retained-tree` |
| P21 | decl | `📜️declaration` | 2,184 | 7 | 11 + shell (407) | inner (`pub mod ui`, 2,178 lines) | `semio-framework-ui-declaration` |
| P21 | rtext | `🖌️render/🖋️text` | 1,008 | 71 | 4 (361) | mod | `semio-framework-ui-render-text` |
| P21 | dispatch | `🖌️render/🖱️dispatch` | 1,587 | 119 | 8 (307) | mod; `impl Dispatcher` cut in 4 at region boundaries | `semio-framework-ui-render` |
| P21 | paintu | `🧪️tests/🔬️targets-wgpu-paint-unit` | 1,679 | 14 | 7 (316) | frag, 68 tests to 68 | `semio-framework-ui` (wgpu-engine, test) |
| P22 | layout | `📜️declaration/📐️layout` | 1,909 | 472 | 6 + shell (419) | inner (`pub mod layout`, 1,437 lines); `utilities`, `role_chrome` stay inline | `semio-framework-ui-layout` |
| P22 | engu | `🧪️tests/🔬️targets-wgpu-engine-unit` | 3,318 | 29 | 9 (487) | frag, 105 tests to 105 | `semio-framework-ui` (wgpu-engine, test) |
| P22 | evu | `🧪️tests/🔬️targets-wgpu-events-unit` | 2,198 | 16 | 8 (414) | frag, 87 tests to 87 | `semio-framework-ui` (wgpu-engine, test) |
| P22 | builder | `🧬️contract/🏗️builder` | 2,573 | 100 | 9 (385) | mod | `semio-framework-ui-contract-builder` |

Method: census-5 Rust method (`F/c5b-tools` lineage). New folders are `<emoji>️<kebab>/🦀️.rs` next to the root, wired by `#[path = "…"] mod …;` plus explicit `pub use child::{…}` / `use child::{…}` lists (no globs added, one `use super::*;` per child as in all earlier slices). Private moved items, fields and members are widened to `pub(super)` only when another file of the split scope names them; `pub(super)` becomes `pub(in super::super)` where the child sits one level deeper (`nest`).

## 2. Oracle results

| check | result |
| --- | --- |
| losslessness | `modsplit.py lossless <tag>` for the 25 module/inner splits: `missing 0` and every extra line explained (wiring, child docstring, `use super::*;`, re-export, region marker, or a declared impl-split pair or declared parent patch). `chk.py live` for the 5 fragment roots: spliced sequence equal to the snapshot, `gone=0 extra(non-marker)=0`, `#[test]` multisets equal (79, 102, 68, 105, 87) |
| structure | `vmod.py live` (25 tags) and `chk.py live` (5 tags): children mounted once, docstring emoji first/equal to the folder emoji/unique/different from the root and the shell emoji, no sibling emoji clash, region balance, brace depth, rustfmt parse, `include!`/`include_str!` targets equal to the original ones as absolute paths and existing: all OK |
| row table | `projsplit.py verify live`: 92 rows before and after, name/version/typescript tuples identical and in the same order, also when the module tree is read the way `readModuleTreeSource` reads it |
| definition scan | `defscan.py` (extended to inner modules): no cross-file private leftovers, except 3 same-name false positives in `backend` (windows and unix variants of one method name) and 1 in `pty` (same-file call) |
| serde string paths | `serdecheck.py` (new): every `skip_serializing_if`/`default`/`with`… function named by a string sits in the same file as its attribute (it found and fixed `is_default`/`is_false` in `builder`) |
| re-export classification | `testonlycheck.py` (new): no `#[cfg(test)] use child::…` in a root or shell names an item that a production file mounted from the same scope uses. It found two real defects (4.3) that the typeck oracle did not report in the default-feature family |
| visibility paths | `vischeck.py` (new): every top-level `pub`, `pub(crate)` and `pub(super)` item of the 25 module/inner snapshots is still nameable at its original path with at least its original visibility (defined or imported in the root or inner shell): OK |
| glob ambiguity | `ambig.py`: 0 candidates, except `limits` (`apply_patch`, an explicit `#[cfg(test)]` re-export that shadows a glob: not an ambiguity) |
| module name clash | `modclash.py`: OK |
| typeck differential (`pos` oracle, `rustc +nightly-2026-07-20 --emit=metadata` over a stub-layered union, base = git `HEAD`, which equals the snapshots) | see section 3 |
| self-contained crate | `semio-framework-ui-contract-typegen` has no dependencies: `rustc +nightly-2026-07-20 -Zno-codegen` over the split tree (a sandbox copy) is clean, and the same command reports `E0432` when one row constant is misspelled (mutation probe) |
| taxonomy | `tax.ts` (live `semanticDirectoryKindId`): 202 new directories, 0 unresolved (a bogus directory is reported unresolved) |
| emoji | `rgi.mjs`: 202 folder names, bad=0; every folder emoji is already used elsewhere in the repo (`folder-emojis.txt`) |
| paths | every new file at most 164 UTF-8 bytes relative and 256 characters absolute |
| source gate | `census-ui.ts` (canonical `readRustSourceClosureV1`, lib, test and all-feature closures) before and after, section 4.2 |

## 3. pos oracle runs (typeck differential, current tree against git `HEAD`)

`POS_PREFIX=c6ui python3 c6ui-tools/posrun.py run c6ui-tools/families/<family>.json --cfgs <cfgs> --label <label>`. Both sides are rewritten into one stub-layered union crate and compiled with `rustc +nightly-2026-07-20 --emit=metadata`; the keyed error multisets are diffed. The typeck canary (E0308, E0616, E0624 reached on both sides) proves the typeck phase ran. HEAD equals the snapshots of all 31 files except `proj` (a peer's one-line change), so the base side is the unsplit code. Reports and raw errors: `C/🗑️generated/c6ui-<family>-<label>-*`.

| run | crates | cfgs (errors cur / base) | privacy | trait-scope | resolve | type | other |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| `c6ui-all` `all2` (final tree, after the `recon`/`builder` re-export fix) | the 21 owning crates, `semio-framework-ui` with `tui`, `tui-terminal` | lib 2,857 / 2,857, test 5,860 / 5,860, wasm32-unknown-unknown 2,495 / 2,495 | 0 new | 0 new | 0 new | 0 new | 0 new |
| `c6ui-wgpu` `wgpu2` (final tree) | `semio-framework-ui` with `wgpu-engine`, `testkit`, `wgpu` plus render, retained-tree, retained-tree-tree, declaration, layout, contract-builder, text, action; the engine, events and paint unit-test fragments are in this closure | lib 4,769 / 4,770, test 9,718 / 9,719 | 0 new | 0 new | 0 new | 0 new | 0 new |

Earlier runs of the same families (`all1`, `wgpu1`, and the per-package `p18a`, `p19a`, `p1820a`) found the defects listed in 4.3; `wgpu1` was the only run that reported the `builder` E0425, so both families were rerun after the fix (`all2`, `wgpu2`).

The few "NEW" rows (3 or 4 per cfg) are flagged as stub-layer noise and are paired with an equal number of "GONE" rows (same error at the moved location). Failures the oracle did find earlier in the slice (the `screen` privacy and path regressions) are in section 4.3; the final tree is clean.

## 4. Decisions and defects found

### 4.1 `proj` (a 1,185-line `const TYPES` table)

The file is one `pub const TYPES: &[SchemaMetadata] = &[ 92 rows ]` plus three small items. A const cannot be moved in parts, so each row became `pub(super) const <NAME>: SchemaMetadata = SchemaMetadata { … };` in one of five contiguous alphabetical row modules, and `TYPES` lists the row consts in the original order. This is a declared deviation from "move items only". `validate` and `render_typescript` are untouched and see the identical sequence; the TS law `🧪️tests/🧱️owners/🟦️.ts` (92 rows, golden bytes, declaration order) now reads the module tree, and the tree order equals the table order (checked). Contiguous alphabetical ranges were chosen over semantic groups on purpose: the law concatenates rows in file-tree order.

### 4.2 Source gate

Closure census (canonical counter) for the 20 crates with the default 4,000 line / 32 file / 256 KiB gate: every crate stays inside it. Highest ratios after the split: `ui-contract-values` 0.963 (3,850 of 4,000 active lines, 18 files), `ui-runtime-reconcile` 0.885 (27 files), `ui-contract-builder` 0.875 (25 files), `ui-contract-validation` 0.831, `ui-render-shader-contract` 0.824 (bytes). Full before/after table: `C/🗑️generated/c6ui-census-before.jsonl` and `c6ui-census-after.jsonl`.

### 4.3 Defects the oracles caught during the slice (all fixed before the final run)

- `screen`: the first install used `pub(super)` unchanged; the `parser` sibling then lost access to the moved methods (E0624) and `super::palette` paths broke (E0433). Fixed with `nest` and `rebase_super`, reinstalled (uninstall restored the snapshot first). The first oracle run reported 71 privacy rows (E0603/E0624) and 6 resolve rows (E0433).
- `modsplit.py rebase_super` rewrote the `super` inside the generated `pub(in super::super)` to three supers; fixed (masked) in `c6ui-tools/modsplit.py`.
- `window`, `backend`: `include!` literals do not get the `../` rebase that `include_str!` gets; `vmod` flagged the missing target and the specs patch the paths.
- `builder`: serde `skip_serializing_if` string paths cannot be seen by the splitter; the helper fns were co-located (found by `serdecheck.py`).
- `recon` and `builder`: the splitter marked a moved private item as a test-only re-export (`#[cfg(test)] use …`) because tests also name it, although a production sibling module mounted by the root (`📤️output`, `♻️retirement/📋️children`) uses it (`register_surface_reconcile_backing`, `BUILT_CHILD_RETIRE_AUTHORITY`). The `wgpu1` oracle family reported the builder one (E0425). `modsplit.py` now classifies production use first; both roots were reinstalled; `testonlycheck.py` guards it.
- `modsplit.py` now supports several inline modules per spec (`inners`), used by `backend`.

## 5. Text readers re-pointed

- `ui/🧪️tests/♻️physical-job-close/🟦️.ts`: the two reads of `🖥️host/🪟️window/🦀️.rs` use `readModuleTreeSource` (the `NativeClipboardJob` impl moved to a child). Verified by emulation: first `fn close_step(&mut self,cx:` and the slice up to `fn next_close_copy_byte_demand` are the same 1,500 characters before and after.
- `ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts`: the projection test reads the module tree of `proj`.
- `…/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts`: `uiSchema` (builder, component, limits, layout, … files) and `mountedTree` (nodes) now use `policyReadModuleTreeSource`.
- `…/🔍️discovery/📖️source-access/🟦️.ts` `POLICY_SPLIT_SOURCE_ROOTS`: added the roots of `nodes`, `rscene` (`🖌️render/🎬️scene`), `ccomp`, `limits` and `builder`, because `policyReadRustPolicySource` only reads module trees for registered roots. `cursor` and `recon` were already registered. No other TS, JSON, py, go or rust reader names these files as text (`readers.py` for all 31 tags; fixtures that mention them only name the path: crate-boundaries `entry`, topology `owner`, provenance strings).
- `🌳️wgpu-document-reconcile` already used `readModuleTreeSource` for `treec` and `nodes`.
- Not runnable here: `bun test` cannot load `web-tree-sitter` (dependencies image not mounted); the edits were checked by the emulation above and by re-reading the call sites.

## 6. Open risks

- `semio-framework-ui` (limits 12,000 lines, 128 files, 1 MiB) is far over its own gate for the all-features closure and was before: 199 files, 56,577 lines before, 258 files, 56,932 lines after (+59 files, +355 lines). The feature closures actually built (`tui`, `wgpu-engine`) were not measured separately. Carving the `tui` and `wgpu` target trees into sibling crates is the real fix and is outside this slice.
- Windows-only code was moved and cannot be compiled here: `pty` (`windows_impl`, 3 children) and `backend` (`native_windows`, 3 children). Only text equality, definition scan and path checks cover it. Compile with `--target x86_64-pc-windows-msvc` where a toolchain exists.
- `cargo test` was not run for anything (no build allowed): the five test-root fragment splits are text-equal to the snapshots but unproven by a compiler.
- `proj` row-table rewrite (4.1) is a code change; the language-agnostic law `🧪️tests/🧱️owners` plus the typegen golden still need to run.
- `values` has 150 lines of headroom under its 4,000 line gate (each new child costs about 4 to 8 active lines).
- Cargo.lock needs no refresh (no manifest changed).

## 7. Crates to compile

`cargo check -p <crate> --lib --tests` for each (lib and test closures changed):

`semio-framework-ui-retained-tree-tree`, `semio-framework-ui-scene-plane`, `semio-framework-ui-scene-mesh`, `semio-framework-ui-render` (also `--features backend-testing`), `semio-framework-ui-action`, `semio-framework-ui-scene-modelling`, `semio-framework-ui-text`, `semio-framework-ui-host` (also `--target wasm32-unknown-unknown`), `semio-framework-ui-contract-components`, `semio-framework-ui-contract-typegen`, `semio-framework-ui-render-shader-contract`, `semio-framework-ui-runtime-reconcile-cursor`, `semio-framework-ui-runtime-reconcile`, `semio-framework-ui-contract-values`, `semio-framework-ui-contract-validation`, `semio-framework-ui-retained-tree`, `semio-framework-ui-declaration`, `semio-framework-ui-render-text`, `semio-framework-ui-layout`, `semio-framework-ui-contract-builder`,

and `semio-framework-ui` three times: `--features tui,tui-terminal`, `--features wgpu-engine,testkit`, and `--features tui,tui-terminal --target wasm32-unknown-unknown` for the wasm-safe `tui` parts (without `tui-terminal`).

TS laws to run after the compile: `physical-job-close`, `🧬️contract/🧪️tests/🧱️owners`, `rust-policy-source-evidence`, and the interactivity audit (`bun repo` law runner).
