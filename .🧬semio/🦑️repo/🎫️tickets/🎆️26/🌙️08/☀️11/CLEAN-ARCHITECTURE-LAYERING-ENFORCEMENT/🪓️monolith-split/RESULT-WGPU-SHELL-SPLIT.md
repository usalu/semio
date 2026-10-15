# Result: wgpu Shell monolith split

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (pub mod `shell` of `semio-framework-os-renderer-wgpu`). Seam map: `SEAM-MAP-SHELL-WGPU.md`.

Status: **split applied, statically verified, NOT compiled.** `cargo check` was never run to completion (see section 6). Do not read any line below as "compiles".

## 1. Final line counts

Parent before: 35,581 lines (35,570 at audit plus 11 lines a peer added at 02:40 to `S_SPACE_INDEX_*` / `program_space_index`). Parent after: 966 lines.

| Child | Folder (under `🧊️wgpu/`) | Lines |
|---|---|---|
| C01 | `🔖️identity-and-types` | 1689 |
| C02 | `🧵️shell-async-runners` | 839 |
| C03 | `🏛️space-admin-and-settle` | 978 |
| C04 | `⚓️dock-model-and-measures` | 1161 |
| C05 | `🧾️panel-records` | 1496 |
| C06 | `🧩️session-and-topology` | 1824 |
| C07 | `⚙️settings-and-dock-persistence` | 2044 |
| C08 | `🎬️shell-actions` | 1927 |
| C09 | `📨️command-dispatch` | 2374 |
| C10 | `🖱️pointer-input` | 2093 |
| C11 | `⌨️keyboard-and-palette` | 2321 |
| C12 | `🧱️chrome-paint-and-navbar` | 2418 |
| C13 | `🧰️window-actions-and-commands` | 2260 |
| C14 | `🗨️chrome-overlays-and-tour` | 1741 |
| C15 | `🎓️tutorial-runtime` | 822 |
| C16 | `🖼️chrome-frame-and-main-window` | 2049 |
| C17 | `🧭️navbar-footer-and-overlays` | 1561 |
| C18 | `💬️chrome-dialog-and-tooltips` | 1251 |
| C19 | `🔑️storage-prefs-and-i18n` | 1934 |
| C20 | `🎨️theme-file-import-and-accessibility` | 2120 |

Every child file is `<folder>/🦀️.rs`. All children are between 822 and 2374 lines (<= 2500). Longest path 229 UTF-8 bytes (C20), all < 240 bytes. The four existing children (`📤️ 🪟️ ⏪️ 📎️`) are untouched. `git diff --stat` of the parent: `753 insertions(+), 35357 deletions(-)` (includes the peer's earlier uncommitted edit).

## 2. What was done

- Cut exactly the 20 chunks of seam map section 3 (C01 as two ranges, 139-142 and 151-1813). Located by content anchors (unique multi-line windows derived from the audit snapshot, re-asserted unique in the current file), moved verbatim, mtime/sha guard before writing. Cut points inside `impl ShellState` (C06/C07, C08/C09, C10/C11, C16/C17/C18) got an `impl ShellState {` opener / `}` closer as in map 3.2.
- Each child starts with a `//!` doc whose first character is the folder emoji, then `use super::*;`.
- Parent keeps: file docs (plus one new `//!` paragraph), imports, `document_surface_preparation`, the 4 existing child wirings, the extension-ledger types, `ShellState`, `Drop`.
- Wiring per child in the parent: `#[path = "<folder>/🦀️.rs"]`, `mod <snake>;`, and a glob re-export chosen by content:
  - `pub use <mod>::*;` where the child holds a `pub` item (C01, C03, C04, C08, C10, C12, C14, C15, C19, C20),
  - `pub(crate) use <mod>::*;` where it holds only `pub(crate)`/crate-visible items (C02, C05, C13, C16),
  - no `use` line for impl-only children C06, C07, C09, C11, C17, C18 (a glob there is an "unused import" warning).
  The glob re-export keeps every previously `pub`/`pub(crate)` path `crate::shell::X` stable, which covers the 13 symbols of map 3.3 and every other pub item, without an explicit list.
- Region markers kept balanced per file: `//#endregion` appended / `//#region` prepended where a region crossed a cut (ShellTypes C01 + parent, ShellLifecycle C04/C07, ShellActions C08/C09, ShellInput C10/C11, ActionPanelAndUtilities C12/C13, ShellChrome C14/C19).

## 3. Visibility widenings

Coordinator rule: anything referenced outside its defining child is at least `pub(crate)` (not `pub(super)` as in the map), because each child will later become its own crate.

Method: a private top-level item (`fn/struct/enum/const/static/type/trait`, tuple-struct item and field), inherent-impl member (incl. `impl ShellState` methods) or named struct field in a child gets `pub(crate) ` iff its identifier occurs in code (or as an inline `{NAME}` format capture) in any other chunk, the parent, the four existing children, or a test file declared by another chunk. Counts per child (widened definitions): C01 222, C02 112, C03 78, C04 41, C05 38, C06 55, C07 23, C08 20, C09 21, C10 21, C11 38, C12 75, C13 34, C14 175, C15 8, C16 53, C17 10, C18 24, C19 68, C20 51. The name-based analysis over-approximates for generic names (`label`, `new`, `window`, ...); a later narrowing pass with a compiler can drop unneeded ones. Trait-impl members, enum variants, items already `pub`/`pub(crate)` and items private to `shell` that stay in the parent are unchanged.

## 4. Path rewrites and test modules

- 47 relative `#[path]` / `include_str!` literals moved with their child and got one extra `../` (map said 54; 7 differ because of the next point). All 80 relative literals in the 21 files were resolved on disk: 0 missing.
- **Seven test module declarations stay in the parent** (paths unchanged) because other test files address them as `super::<name>`: `panel_anchor_model_tests`, `shell_input_tests`, `shell_boot_isolation_tests`, `window_pane_chrome_tests`, `command_registry_tests`, `window_actions_search_pane_tests`, `appearance_tour_and_footer_pill_tests`. The first oracle run (everything moved) produced 60+ extra `cannot find ... in super` errors for exactly these, which is why. The other 44 test mods moved with their child. Identifiers of every test file count as references from the chunk that declares it.
- The renderer agent's `RENDERER_SOURCE` concat for 4 Shell tests was left alone.

## 5. Verification actually run

Static, because cargo is blocked. Tooling is in `🗑️generated/shell-split-tools/` (`split.py`, `rlex.py`, `verify.py`, `oracle_shell.sh`, `mksandbox.py`, `em.ts`, `shell-original.rs.txt` = pre-split file, `base-*.txt`/`post-*.txt` = diagnostic header multisets). `split.py` expects `orig.rs` (the 35,570-line audit snapshot) next to it.

| Check | Command / method | Result |
|---|---|---|
| Name-resolution oracle (coordinator tip) | `rustc +nightly-2026-07-20 -Zunpretty=expanded --crate-name shell_probe --edition 2021 --crate-type lib [--test] [--target wasm32-unknown-unknown] <parent>`, header lines `^(error\|warning)` sorted/uniq-counted; baseline = copy of the pre-split file in a symlink sandbox mirroring the repo (`mksandbox.py`), split = real tree | native 2243 errors / 0 warnings both; native `--test` 5494 / 3 both; wasm32 2169 / 2 both; wasm32 `--test` 4202 / 3 both. `diff` of the multisets: **identical** in all four. All errors are external-crate/`crate::` resolution noise, present in the baseline too (including a pre-existing peer syntax error in `🧪️tests/🧭️wgpu-navbar-footer-parity` in the `--test` runs). |
| Lossless | `verify.py`: multiset of trimmed non-blank lines, `pub(crate)` and `../` prefixes normalised, original vs parent + 20 children | 0 missing. Extras are only headers, wiring, openers/closers, region markers. |
| Visibility closure | `verify.py`: every non-`pub` item / impl member / struct field defined in a child must not occur as identifier in any other file or in tests declared elsewhere | 0 violations (independent re-scan of the written files). |
| Syntax | `rustfmt --edition 2021 --emit stdout < file` on parent and 20 children | 21/21 rc 0. |
| Paths | script over all `#[path]`/`include_str!`/`include_bytes!` literals | 80 checked, 0 missing. |
| Regions | script | balanced in all 21 files. |
| Duplicate top-level names across children | script | none; only pre-existing same-file `cfg` pairs. |
| Emoji admission | `bun em.ts` with `/^\p{RGI_Emoji}$/v` | all 20 RGI (bare for 🔖🧵⚓🎬🧱🧰🗨... emoji-presentation ones, with U+FE0F for ⚙🖼🖱🏛⌨🗨; folder names always carry U+FE0F per convention), 0 duplicates among the 24 sibling folders. |
| Line/size | `wc -l`, path length script | see section 1. |

Defects the oracle caught and I fixed before finishing: lexer swallowed `0..CONST` (consts used only in ranges not widened), inline `{CONST}` format captures, tuple-struct private fields, 7 cross-referenced test mods, and 6 `unused_imports` + 4 empty-glob warnings (wiring now per content).

## 6. Not done / not verified

- **No compile.** `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short` (native, `--tests`, `--target wasm32-unknown-unknown`): first failed at `cargo fetch --locked` ("cannot update the lock file", peer added a workspace member at 01:40, lock refreshed 02:03); a retry loop then sat queued in the cargo lease/fetch queue (load average ~50) for about 45 minutes with no result. After the coordinator's message that the compiler-ingress gate refuses the crate (source boundary: 4000 active lines / 32 files / 262144 bytes) I stopped the loop. Report as: **blocked by source-boundary gate; no baseline and no post-split cargo result exist.** The `--tests` and wasm builds were not run through cargo either.
- The oracle cannot see field/method privacy (typeck never runs because external crates are unresolved) nor the `E0659` ambiguities that only appear after type resolution. Mitigation is the closure check above, which is by construction name-based. First real compile should expect: (a) a few missed `pub(crate)` on members whose name only occurs inside macro token trees, (b) possible `E0659` between sibling glob re-exports for names defined in two children (none found by script), (c) over-widening to narrow later.
- No test was run (`cargo test` impossible). Test files moved with children are only checked for path resolution and identifier closure.
- The map's optional refinement (move chrome-dialog paint/key/accessibility into C14) was not applied; sizes are fine without it.
- Nothing was committed or staged; no git-modifying command was used. No debug logs were added; no `🗑️generated` folder was removed (I deleted only my own two failed cargo logs).

## Hardening (full-resolution differential)

Full report: `RESULT-HARDEN-WGPU.md`. Differential from the real lib root with `shell-split-tools/shell-original.rs.txt` restored in the baseline (20 child folders removed; Shell source-text readers `🔄️sync-owner-lifetime`, `🎡️wgpu-wheel-and-escape-routing`, `🔀️wgpu-document-relay`, `🌓️appearance-tour-and-footer-pills` reverted, and the `SHELL_SOURCE` const added to the original so the other readers resolve): native lib 450/450, native `--cfg test` 511/511, wasm32 590/590, wasm32 `--test` 654/654 distinct non-external errors, identical after the fixes. Shell needed no widening: the definition scan (family = the Shell module subtree including the mounted test files) found 0 fields, methods or items used outside their child except the skipped false positive `const _` in `🧵️shell-async-runners`. The peer test `🧪️tests/🧭️wgpu-navbar-footer-parity` has a parse error (missing `;`, lines 158 and 314) in the repo itself; it was patched in both sandboxes only. Residual: no typeck, native `--test` blind.
