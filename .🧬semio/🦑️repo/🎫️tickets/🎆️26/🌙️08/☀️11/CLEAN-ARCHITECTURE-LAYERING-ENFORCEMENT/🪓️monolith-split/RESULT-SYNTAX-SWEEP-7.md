# 🧹 RESULT SYNTAX-SWEEP-7 (prefix `ss7`)

## 0. Verdict

**DONE.** Every tracked and untracked-not-ignored `.rs` file outside ticket scratch parses. Real parse errors: **16 files / 27 sites before, 0 after** (final full sweep over 34 151 files, plus a last delta sweep over the 1 537 files modified afterwards: 0 failures in the normal parse). The only remaining non-ticket parse failure is one fragment-by-design template (class F1 below). No sub-agents, no git-modifying command, no compile.

Method note: `rustc +nightly-2026-07-20 --edition 2021 -Zunpretty=normal` (a real parse; `-Zparse-only` no longer exists). The workspace edition is 2021 (6 crates are 2024; none of their files failed). A second pass with `-Zunpretty=expanded --test` expands built-in macros and in-file `macro_rules!`, so it also parses the arguments of `assert_eq!`, `format!`, `vec!`, `matches!`, `write!` and so on. Known peer damage patterns were checked against it on a probe file: `assert_eq!*(` and `<X as A + ::B>::f` are reported, so they are covered. No such damage remained in the repo.

## 1. Counts

| Pass | Edition / mode | Files | Failing |
|---|---|---|---|
| 1 | 2024, normal | 33 757 | 104 |
| 1b | the 104 re-checked under 2021 | 104 | 41 (63 were `gen` keyword false positives of 2024 mode) |
| 3 | 2021, expanded `--test`, noise filtered | 33 757 | 31 error lines in 28 files: 26 ticket scratch, the store template (F1) and one new real error (jpg schema, B5, edited by a peer after pass 1) |
| final | 2021, normal | 34 151 | 43: 41 ticket-folder fragments or snapshots, 1 template (F1), 1 transient peer mid-edit (cad sqlite snapshot, parses on re-read) |
| final expanded | 2021, expanded, files modified after pass 2 | 604 | 0 non-ticket |
| delta | 2021, normal + expanded, files modified after the final list | 1 537 (list of 34 496) | normal 0; expanded 10, all `*-tools/orig/*.rs` fleet snapshots in this folder |

Expanded-mode noise filtered out (not syntax): `cannot find attribute/derive/macro` (serde, value, dsl, error, graphql, other crates), `include_str!`/`#[path]` file-not-found (standalone file, no crate root), `env!` (`CARGO_MANIFEST_DIR`, `OUT_DIR`), and every `error[E....]` resolution error.

Limits: macros from other crates (for example `#[semio_framework_async_macros::async_test]`, serde, graphql) are not expanded, so their argument text is not parsed; `cfg(feature = ..)` gated items are skipped by expansion (normal mode still parsed them); semantic errors are not in scope.

## 2. Fixed sites (all at the source, intent restored, no logic deleted)

Introducing commits are the repo's auto-commits (the date text is fake, the `🚩️NNN` counter is the order). Snapshots of every file before editing: `ss7-tools/orig/*.orig`.

| # | File (shortened) | Cause | Fix | Introducing commit |
|---|---|---|---|---|
| A1 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` | `#![allow(clippy::result_large_err)]` after a `use` line | hoisted the inner attribute behind the `//!` header | 793f63533f2 (688) |
| A2 | `…/🧊️generation3d/🔨️modules/🏠️host/✉️envelope/🦀️.rs` | `#![allow(unused_imports)]` after `extern crate` lines | hoisted | 7c8af563c74 (687) |
| A3 | `…/🧊️generation3d/🔨️modules/🏠️host/🌱️initialization/🦀️.rs` | two inner attributes after `use`/`extern crate` | hoisted both, order kept | 7c8af563c74 (687) |
| A4 | `…/🧊️generation3d/🔨️modules/🏠️host/🦀️.rs` | `#![allow(unused_imports)]` after `extern crate` | hoisted | 7c8af563c74 (687) |
| A5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️.rs` | `#![cfg(not(target_arch = "wasm32"))]` after `extern crate` | hoisted | 1bae6115bc5 (690) |
| B1 | `…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs` | `///` doc left in an emptied `🔁️Re-exports` region (the `pub use` is gone) | removed the dangling doc line, markers kept | ebbace9b320 (606) |
| B2 | same path with `✏️editor` | same | same | de617a7c17f (602) |
| B3 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/…/📊️table/🧬️schema/🧬️mutations/🦀️.rs` | `🧪️FixtureTests` region kept only its docs; commit 687 deleted `🧪️tests/🔬️fixture/🦀️.rs` and its `mod fixture_tests;` | removed the now-empty stale region (docs only, no code) | 7c8af563c74 (687) |
| B4 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🎬️execution/🧪️tests/🔬️unit/🦀️.rs` | `/// 🎨️ LAW (… WG11)` doc at end of file with no item; its test `every_painted_shader_edit_finds_its_anchor_in_the_lit_shader_once` was carved to `🎨️shaders/🧪️tests/🦀️.rs` without the doc | moved the 3-line doc onto that test fn | 1bae6115bc5 (690) |
| B5 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/…/🧾️document/🧬️schema/🦀️.rs` | `/// 🆕️ A new jpg document …` stacked on the demo fn; a peer then moved `demo_jpg_snapshot` to `🧬️schema/📸️snapshot` and left the doc dangling (`blank_jpg_snapshot` already has its own doc in `🚪️io`) | removed the 2 orphan doc lines | doc stacking from the carve in HEAD, orphaned by a working-tree peer edit (09:09) |
| C1 | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧠️inference-service/🦀️.rs` | stray `,` starting the line `, cx:&mut …` in `infer_gis_map_controlled` | restored `cx: &mut …,` as its own parameter line | 0c3e78944af (685) |
| D1 | `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` | `pub macro_rules! puzzle_progress_step` (`pub` cannot qualify `macro_rules!`; `#[macro_export]` is already there) | dropped `pub` | 793f63533f2 (688) |
| E1 | `🌎️hub/🧩️compositions/➗️mathematical/🧪️tests/🔬️surface/🦀️.rs` | a sweep that added an `ActorId` argument to `with_registry(…)` also pasted it into the test fn definition `equation_viewer_instantiates_through_new_viewer(…)` | restored `()`; the `with_registry(.., ActorId(..))` call keeps the argument | d7a0223e6f6 (673) |
| F2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🪶️sqlite/🫴️semantic/🦀️.rs` | minified match arms `"null"if scalar==empty=>` lex as a string literal with suffix `if` (5 sites: null, boolean, number, string, array/object) | restored the space before `if` | 5487a5cce52 (679) |
| F3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️preview/🦀️.rs` | same literal-suffix lexing in the `progress_phase` match (7 arms) | restored the space before `if` | 7af1259dba8 (686) |
| G1 | `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧪️tests/🔬️unit/🦀️.rs` | `…maximum_copy_bytes<size_of::<Arc<AtomicUsize>>>()` (one `>` too many, "comparison operators cannot be chained") | `>>()` | 7c8af563c74 (687) |

The handoff cases named in the brief are all covered: the generation3d envelope root (A2), the gismap signature (C1). The `✏️s/…/🗄️stdio/…/☁️las/🦀️.rs` file with 3 errors was not reproducible: it parses in every pass (a peer fixed it before this sweep started, or its error was in the macro-body class that normal parse accepts).

Tools: `ss7-tools/parse-one.sh` (normal parse), `ss7-tools/expand-one.sh` (expanded parse with noise filter), `ss7-tools/hoist_inner_attrs.py` (A-class fixer, drives rustc errors, preserves attribute order, asserts single-line attributes). Logs: `🗑️generated/ss7-*`.

## 3. False-positive and non-compiled classes (not fixed on purpose)

* F1, template: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🔬️isolated/🦀️.rs` holds `#[path = @MODULE@]` placeholders; `🧪️tests/🟦️.ts` (line 28) reads the file and substitutes them before compiling. Intended.
* T1, ticket-folder scratch: 41 files under `.🧬semio/🦑️repo/🎫️tickets/**` fail the normal parse (25 genuine scratch files: code snippets with unbalanced braces such as `history-fn-snippet.rs` and `📋️retained-authority-law-input.rs`, `*.HEAD.rs`/`*.old.rs`/`backup-*.rs` copies of old monoliths, generator templates `*-template.rs`/`*.tpl.rs` with `@PLACEHOLDER@`, `w4b_png_engine.rs`). They are not part of any crate and are not compiled. The other 16 were this sweep's own pre-fix snapshots in `ss7-tools/orig/` (renamed to `*.rs.orig` afterwards, so they no longer count as sources).
* T2, fleet snapshots: the delta expanded pass flagged 10 files in `*-tools/orig/` of other fleet slices (`bd8`, `r5a`, `r5b`); these are pre-split monolith copies with `#[path]`/fragment content, owned by those slices.
* 2024-only: pass 1 under edition 2024 flagged `gen` as a reserved identifier (for example in `🀄️wfc/⚙️engine/🧬️evolve`); the crates are edition 2021, so no change.
* Transient: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/…/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` failed once at 10:07 while a peer was rewriting it (written 10:08); it parses on re-read and in the delta sweep.

## 4. Observations for the compile queue (not parse errors, not touched)

* `…/🧊️generation3d/…/👁️viewer/👥️presence/🧬️schema/🦀️.rs` uses `Generation3dViewCamera` without an import or definition in that file.
* `…/🌀️generation2d/🔨️modules/🏠️host/🦀️.rs` has `#[cfg(test)]` followed by a blank line and then `use …::camera_from_dsl;`, so the attribute binds to that single import only (a sweep artifact; check at compile time).
* Several host roots carry `extern crate semio_framework_os_kernel as dsl/protocol/store;` aliases added by commits 687/688; the gate for aliases belongs to the compile step.

## 5. Re-run

`sh ss7-tools/parse-one.sh <file>` per file, or `git -c core.quotepath=false ls-files -z '*.rs'` plus `git ls-files --others --exclude-standard -z '*.rs'` through `xargs -0 -P4 -n1 nice -n 10 ss7-tools/parse-one.sh` (about 20 minutes at machine load 58). Expected output today: only the 25 ticket-folder scratch files plus F1 (the 16 snapshots are renamed).
