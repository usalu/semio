# 🧹 Round 2 Slice Q-1a: TUI Framework Rules Cleanup

Scope: `🧰️framework/🔨️modules/🖱️ui/⌨️tui/**` and the `⌨️tui` targets under `🧱️elements/**`, except `🔡️ansi`, `🔌️backend`, `🪟️windows`, `🏃️host`, `🚇️pty` (and their `backend-native-*` / `ansi-unit` tests, T-A).

## Verified results

| check | result |
|---|---|
| `cargo test -p semio-framework-ui --features tui-terminal --lib tui::` | 356 passed, 0 failed (354 before, +2 feature-binding tests) |
| `cargo check -p semio-framework-ui --features tui` | exit 0 |
| `cargo check -p semio-framework-ui --features tui --target wasm32-unknown-unknown` | exit 0 |
| `cargo check -p semio-framework-repo-dashboard` and `--tests` | exit 0 |
| `cargo +nightly-2026-07-07-x86_64-pc-windows-msvc clippy -p semio-framework-ui --features tui-terminal --all-targets` | 0 warnings in my files (61 before); other crates and T-A modules untouched |
| `r2-q1a-emoji-check.ts` | 59 files, 0 duplicates, 0 missing (was 57 duplicate groups and 18 missing) |
| leftover scan (placeholder `?`, U+FFFD, `[DEBUG]`, `println!`, indented `//` comments) | none in scope |

## What changed

1. Emoji restore (`r2-q1a-restore-emoji.ts`). The brief named `d7a0223e6f6` as the source, but that monolith already carries `???` (110 lines, in every revision back to `5ac47258a60`). The last revision with real emoji is `506c4f39d5e` (`⌨️tui/🦀️component.rs`); the script restored 77 docstring and region lines from it. Four lines new since then (event `Key`, `Modifiers`, `Event`; `tui/🦀️.rs` pty line) and five rewritten ones got a chosen emoji. Region markers use `//#region 🔖️Name`, as in the untouched files.
2. Unique emoji (`r2-q1a-assign-emoji.ts`, checker `r2-q1a-emoji-check.ts`): reassigned the repeated ones in engine, theme, scene, layout, geometry, rows, cell, chrome, text, tables, widget and 17 element targets; gave emoji to the 15 docstrings that had none (widget signals, tables). Module `//!` keeps the element emoji, the first item docstring got a new one.
3. Comments inside definitions: 40 `//#region` markers removed from `impl` blocks in `⚙️engine` (14), `📟️vt/🧱️screen` (18), `📟️vt/🖥️pane` (8) (`r2-q1a-strip-comments.ts`); the `📏️layout` in-body comment moved into the `resize_window` docstring; four test comments in `🧪️tests/🔬️unit` became test docstrings. Top-level region markers stay.
4. Debug probes: four `println!("[DEBUG] ...")` (three in `📟️vt/🧪️tests/🔬️unit`, one in `⌨️input-decoding`) replaced by non-empty-fixture assertions (`r2-q1a-debug-probes.ts`).
5. Clippy fixes, behaviour-preserving: `is_multiple_of`, `checked_div` / `NonZeroU32`, `map_or`, `is_none_or`, `repeat_n`, `mem::take`, `sort_by_key`, derived `Default`, `Option` wrappers dropped from always-`Some` helpers (Input `changed`, Toggle `flip`, Select `cycle`, pane `resume_passthrough`), by-value params consumed, `tables::` qualifications removed, `ZWJ`/`LVT` break classes renamed `Zwj`/`Lvt`. Argument-count fixes: `Cell` template for `CellBuffer::put_cluster`, `RangeInclusive` for `paint_span`, `StackSlot` for `style_window`, `Ink` for the table cell painter, `Frame` for the three window outline painters. Large enum variants: `TerminalState.screen` is `Box<VtScreen>`, `TreeState.index` is `Box<FilterIndex>`, `ChromeState::Window` is `Box<WindowState>`.
6. Feature runners: `🔬️render-equivalence` and `🔬️pointer-routing` adapters now `include_str!` their `🥒️.feature` and assert its `@capability-` tag; the three `⌨️tui-terminal-*` features are bound the same way in the vt unit tests (`names_capability`). A rename of any feature file now breaks the build.

## Outside my scope (one line)

`🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs:253` now reads `ChromeState::Window(Box::new(WindowState::new(...)...))` because the variant is boxed. Nothing else in the dashboard needed to change (it compiles, tests included).

## Notes and traps

- Files under `ui` are rewritten by running scripts through a temp file plus rename (`r2-q1a-edit.ts`): a plain `writeFileSync` left 11 stale tail bytes in `✏️Input/.../🦀️.rs` (fixed with Edit; compile and tests confirm no other file has a stale tail). `truncateSync` fails on these files.
- Bun turns non-ASCII characters inside `String.raw` into `\uXXXX`; patch scripts use plain strings.
- Feature folders and runner folders differ by emoji (`🎞️render-equivalence` feature vs `🔬️render-equivalence` runner, `🖱️pointer-routing` vs `🔬️pointer-routing`); I left the layout and bound them by path.
- UNVERIFIED: native (non-wasm) `tui` build on macOS/Linux; the `cfg(unix)` pty and backend tests are T-A's.

## Scripts (all in the ticket folder)

`r2-q1a-restore-emoji.ts`, `r2-q1a-assign-emoji.ts`, `r2-q1a-emoji-check.ts`, `r2-q1a-strip-comments.ts`, `r2-q1a-debug-probes.ts`, `r2-q1a-bind-features.ts`, `r2-q1a-edit.ts`, `r2-q1a-clippy-layout.ts`, `r2-q1a-clippy-patches.ts`, `r2-q1a-clippy-qualifications.ts`, `r2-q1a-rename-break-classes.ts`, `r2-q1a-box-variants.ts`, `r2-q1a-lifetimes.ts`, `r2-q1a-remove-probe.ts`, `r2-q1a-clippy-filter.ts`. Build and clippy output sits in `🗑️generated/q1a-*.txt`. The patch scripts are one-shot records, not idempotent.

## Final round (coordinator, after peers' edits)

- `r2-q1a-emoji-check-changed.ts` scans every changed or untracked `.rs` under `🖱️ui` and `🎛️dashboard` (from `git status --short -uall`, skipping A-1's `📜️root-delegation`, `⌨️usage`, `🧭️cli`), reporting duplicate or missing docstring emoji, indented `//` comments and `[DEBUG]`. First run: 109 files, 22 duplicate groups (`📝️text` 7, `🚇️pty` 2, `🔲️cell` 1, dashboard `🌀️daemon/✉️ipc`, `🎮️registry`, `🧪️tests/🧭️journeys/*`) plus 1 `[DEBUG]` println in `🌀️daemon/🧪️tests/🧊️integration` (removed).
- Fixed with `r2-q1a-assign-final.ts` and `r2-q1a-assign-final-2.ts`; keycap `#️⃣` docstrings are recognised as emoji. Last run: 109 files, 0 problems (a peer's new duplicate in `🎛️dashboard/🧪️tests/🔬️unit` appeared meanwhile and was fixed too).
- `cargo test -p semio-framework-ui --features tui-terminal --lib tui::`: 363 passed, 0 failed. `cargo check -p semio-framework-repo-dashboard --tests`: exit 0 (a peer's transient `&char` compile error in `🧪️tests/🔬️unit` resolved on its own).
- A-1 files not scanned; run `bun r2-q1a-emoji-check-changed.ts` after A-1 finishes (it skips them, so temporarily remove `skipped` entries).
