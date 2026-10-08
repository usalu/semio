# W0 — TUI Monolith Split Into One File Per Module

Slice w0 of the dashboard terminal-application fleet. Structural only: zero behaviour change, zero API
change. Done 2026-10-07 19:21:43 (local), verified until 19:28.

## Result

`🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️.rs` went from one 5451-line file to a 56-line manifest plus 15
module files `⌨️tui/<emoji>️<name>/🦀️.rs`. Every `crate::tui::<mod>::…` / `ui_tui::tui::<mod>::…` path
resolves as before (the module tree is unchanged: `crate::tui::component::<mod>`, re-exported by
`🎯️targets/⌨️tui/🦀️.rs` with `pub use component::*`). No item was renamed, reordered, added or removed.

The only edit inside any body is the path of the three test `include!`s in `backend`, because
`include!` resolves against the including file: `include!("🧪️tests/…")` → `include!("../🧪️tests/…")`.

## Module → Folder Mapping

All folders are `emoji + U+FE0F + lowercase module name`, NFC, same byte form as the siblings
`🪟️windows` / `🧪️tests`. The 18 siblings of `⌨️tui/` (15 new + `🦀️.rs`, `🧪️tests`, `🪟️windows`) carry 18
distinct emoji, which is what the repo's path-emoji statute requires (`pathEmojiPolicy.siblingNamespace
= "files-and-directories"`, finding kind `duplicate`).

| Module | Folder | Lines | Old lines (body) | Precedent |
|---|---|---:|---|---|
| `geometry` | `📐️geometry` | 74 | 9–82 | `🖱️ui/🎯️targets/🧊️wgpu/📐️geometry` (sibling target); 20 `📐️geometry` dirs repo-wide; registry rows in 6 member kinds |
| `theme` | `🎨️theme` | 113 | 88–200 | taxonomy kind `theme` = 🎨️ `^theme$` whose `parentKindIds` name `tui-target`; `🎯️targets/🧊️wgpu/🎨️theme` |
| `text` | `📝️text` | 459 | 206–664 | taxonomy kind `text` = 📝️ `^text$`; `🎯️targets/🧊️wgpu/📝️text`; 202 dirs |
| `cell` | `🔲️cell` | 165 | 670–834 | **invented** — no `cell` dir exists (`📏️cells` are sqlite snapshot tables). 🔲 is the repo's raster/grid-unit emoji: `🧰️framework/🔨️modules/🔲️pixels`, `🔲️grid-2d`, `🔲️grid` |
| `ansi` | `🔡️ansi` | 367 | 840–1206 | **invented** — no precedent for ANSI/escape codecs. 🔡 has one registry use (`imperative-extension-text`), no conflicting meaning |
| `vt` | `📟️vt` | 893 | 1212–2104 | **invented** — the only `vt` dir is `🧾️vt` = PDF/VT subset (different concept); `🖥️terminal` (dashboard) would collide with `🖥️chrome`. 📟 is unused in the registry |
| `event` | `📡️event` | 64 | 2110–2173 | `🖱️ui/🖥️host/📡️event`; registry `members-of-members-of-members-of-modules` already lists `📡️event` (exactly this nesting level) |
| `scene` | `🎬️scene` | 245 | 2179–2423 | `🖱️ui/🎬️scene`, `🖱️ui/🖌️render/🎬️scene`; kinds `ui-scene` / `ui-retained-scene`; 17 dirs |
| `layout` | `📏️layout` | 534 | 2429–2962 | `🖱️ui/🖌️render/📏️layout`, `✏️s/🔌️plugins/📏️layout` (4 registry rows). `📐️layout` collides with `📐️geometry`; `🧮️layout` (wgpu) would take 🧮, which the registry reserves for kind `math` under `tui-target` |
| `widget` | `🪀️widget` | 437 | 2969–3405 | `🎯️targets/🧊️wgpu/🪀️widgets`, taxonomy kind `widgets` = 🪀️ (singular kept: the folder is named after the module) |
| `chrome` | `🖥️chrome` | 570 | 3412–3981 | taxonomy kind `chrome` = 🖥️; `🎯️targets/🧊️wgpu/🖥️chrome` (`🪟️chrome` of react collides with `🪟️windows`) |
| `engine` | `⚙️engine` | 207 | 3987–4193 | taxonomy kind `engine` = ⚙️; `🎯️targets/🧊️wgpu/⚙️engine`; 27 dirs |
| `backend` | `🔌️backend` | 621 | 4199–4819 | taxonomy kind `ui-host-backend` = 🔌️ `^backend$` with parent `members-of-members-of-modules` (which `⌨️tui` is); `🖱️ui/🖥️host/🔌️backend`, `🖌️render/🔌️backend` |
| `pty` | `🚇️pty` | 525 | 4827–5351 | **invented** — no pty/tty/process-pipe dir. 🚇 has one use, `🌎️hub/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1` (a byte pipe), the same concept class |
| `host` | `🏃️host` | 87 | 5357–5443 | taxonomy kind `host` = 🏃️ `^host$`; `🎯️targets/🧊️wgpu/🏃️host` (`🖥️host` collides with `🖥️chrome`) |
| (root) | `🦀️.rs` | 56 | — | manifest, same shape as `🎛️dashboard/📦️packages/🦀️rust/🦀️.rs` |

Sum: 5361 body lines + 56 manifest lines = 5417; the missing 34 are the 30 `pub mod x {` / `}` lines
replaced by 30 `#[path]` / `pub mod x;` lines (net 0), the 32 `// #region` / `// #endregion` marker lines
and the 2 blank lines after the `Widget` / `Chrome` markers.

### Translating Old Line Citations

The audits in this ticket cite the monolith by line (`TUI:<line>`, 5451-line state, sha256
`d8e1709d…dffb`). New location: find the module whose old body range contains the line, then
`new line = old line − N` with N = geometry 8, theme 87, text 205, cell 669, ansi 839, vt 1211,
event 2109, scene 2178, layout 2428, widget 2968, chrome 3411, engine 3986, backend 4198, pty 4826,
host 5356. Columns shift by −4. Spot checks: old 4551 → `🔌️backend/🦀️.rs:353`, old 5365 →
`🏃️host/🦀️.rs:9`. I did not rewrite the audits (`tui-chrome-parity-audit.md`,
`tui-interaction-audit.md`): they are other agents' reports.

## Attributes Preserved

| Attribute | Before | After |
|---|---|---|
| `#[cfg(all(feature = "tui-terminal", windows))] #[path = "🪟️windows/🦀️.rs"] mod windows_abi;` | root | root, untouched |
| `/// ?? Pseudo-terminal child process spawn …` + `#[cfg(feature = "tui-terminal")]` on `pty` | on `pub mod pty {` | on `#[path = "🚇️pty/🦀️.rs"] pub mod pty;` (same two lines, same order) |
| `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` | root | root, untouched |
| nested `attr`, `mods`, `native_unix`, `native_windows`, `unix_impl`, `windows_impl`, `bindgen_host` and every inner `#[cfg]` | inside the body | inside the module file, byte-identical |
| inner `#![…]` attributes | none existed | none |

No module had inner docs (`//!`); none were added, so the doc attributes are identical.

The 32 top-level `// #region ???Name` / `// #endregion ???Name` comment lines are gone: they wrapped the
inline blocks and are not tokens. The repo's Rust statute flags `// #region` comments as legacy
(`BreachCodeRustRegionComment`), and the dashboard manifest has none around its `#[path]` modules. Region
comments *inside* bodies are untouched.

## Lossless Proof

Script: `w0-tui-module-split.py` (ticket input; `snapshot | plan | write | verify | inline`).

```
$ python3 …/w0-tui-module-split.py verify
snapshot   sha256 d8e1709d95c9550d37f29ff369e07636c6186d90cad49ded42bac88d89bfdffb bytes 212703 lines 5451
re-inlined sha256 d8e1709d95c9550d37f29ff369e07636c6186d90cad49ded42bac88d89bfdffb bytes 212703 lines 5451
byte-identical: True
whitespace-normalised identical: True
token streams identical: True (44588 vs 44588 tokens)
tokens in split tree (manifest + 15 files): 44663
```

1. **Byte-identical round trip.** Re-inlining the live tree (each `#[path] pub mod x;` back to
   `pub mod x { … }`, body re-indented by 4 spaces, include paths restored, region markers regenerated
   from the script's table) reproduces the pre-split monolith byte for byte. That is stronger than the
   requested whitespace-normalised diff, which is therefore also empty.
2. **The de-indent is exactly invertible on this file.** `plan` reported 0 anomalies in all 15 modules:
   no line starts inside a string literal, no whitespace-only line, no under-indented line.
3. **Independent of my lexer.** A separate scan of the snapshot found 0 lines with an odd count of
   unescaped `"`, 0 raw strings, 0 block comments, 0 line-continuation backslashes, 0 tabs, 0 CR — so no
   multi-line string could have been altered by the de-indent.
4. **Independent parser (rustfmt, i.e. rustc's parser).** For each module, `rustfmt` of the original
   block and of `pub mod x {` + new file (not re-indented) + `}` are byte-identical — 15/15.
5. Token count: 44663 = 44588 + 15×6 (`#[path = "…"]`) − 15 (`{ }` → `;`).

Concurrency: the script compared the live monolith with the snapshot immediately before the single
`os.replace` of the root (same process, after all 15 files were written and re-inlined from disk). Live
sha256 at 19:21:43 = snapshot sha256; nobody changed it between read and write. The earlier uncommitted
change of the day (`libc::select` in `NativeTerminal::poll`) is in `🔌️backend/🦀️.rs:358`. At 19:28 all 16
files still carried mtime 19:21:43 and `verify` was still byte-identical; no background re-inlining seen.

## Verification

All foreground, one at a time, logs in `🗑️generated/tui-split/`.

| Command | Result lines |
|---|---|
| `cargo check -p semio-framework-ui --features tui-terminal --message-format=short` | `Checking semio-framework-ui v0.1.0 (…/🖱️ui/📦️packages/🦀️rust)` · ``Finished `dev` profile [unoptimized] target(s) in 5.30s`` · exit 0 |
| `cargo check -p semio-framework-ui --features tui --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished `dev` profile [unoptimized] target(s) in 0.47s`` · exit 0 |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features tui --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished `dev` profile [unoptimized] target(s) in 10.22s`` · exit 0 |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features tui-bindgen --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished `dev` profile [unoptimized] target(s) in 0.64s`` · exit 0 |
| `cargo check -p semio-framework-repo-dashboard --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · `Checking semio-framework-repo-dashboard v0.1.0 (…)` · ``Finished `dev` profile [unoptimized] target(s) in 0.90s`` · exit 0 |

The two wasm commands are the ones `📦️packages/🦀️rust/📜️script.ts check wasm` runs for `tui` /
`tui-bindgen` (installed targets: `aarch64-apple-darwin`, `wasm32-unknown-unknown`, `wasm32-wasip2`).

Proof the code was really type-checked (the `semio-framework-ui` lib itself emits no warning under these
feature sets, before or after, so "warnings present" is not available as a signal for the checks):

- rustc's own dep-info of each fresh unit lists the new files. `tui-terminal` units
  (`…/semio-framework-ui/f906e7701e8ecd4b`, `…/838648b6fe75efc1` for the dashboard) read all 15 module
  files; `tui` units (native `9be7b2505efcfb5c`, wasm `2b94540e69dc4c9c` and `d0a01d973652ee75`) read 14 —
  `🚇️pty` is absent exactly because its `#[cfg(feature = "tui-terminal")]` gate survived.
- Fresh `.rmeta` written at the check times: 19:22:02, 19:22:23, 19:22:42 (×2), 19:22:48 (ui + dashboard).
- The test build below emits the type-dependent lint `unused_must_use` at the *new* include path
  `…/⌨️tui/🔌️backend/../🧪️tests/🔬️backend-clipboard-mailbox/🦀️.rs:21:9` (same warning, old path, before).

### Tests

`CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-tui-split cargo test -p semio-framework-ui --features tui-terminal --lib tui`

| State | Result line |
|---|---|
| Before the split (monolith sha256 `d8e1709d…`, 19:21:26) | `test result: FAILED. 105 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.26s` |
| After the split (19:23:08, test binary rebuilt 19:23:09) | `test result: FAILED. 105 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.27s` |

`diff` of the sorted `test <name> ... <status>` lists (106 lines each) is empty: same names (all still
`tui::component::…`, e.g. `tui::component::backend::native_unix::tests::…`), same statuses.

The one failure is the same test in both runs and is not caused by the move:
`tui::component::tests::shell_window_wizard_body_paints_options_after_remount` —
`🧪️tests/🔬️unit/🦀️.rs:198:5: wizard options missing from paint: " /   …"`.

### Path-Emoji Statute

`bun …/w0-tui-module-split-statutes.ts` runs the repo's own `pathEmojiStatuteFindings`
(`🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts`) and the `canonicalTaxonomyEmoji` admission rule over the
whole `⌨️tui` subtree (42 entries, 21 directories): 0 findings introduced, 0 emoji rejected, longest path
110 of 240 bytes. It also reports 3 inherited `duplicate` findings under `🧪️tests` (the four pre-existing
`🔬️…` siblings) — untouched by this slice.

## Files

Created (all under `🧰️framework/🔨️modules/🖱️ui/⌨️tui/`):
`📐️geometry/🦀️.rs`, `🎨️theme/🦀️.rs`, `📝️text/🦀️.rs`, `🔲️cell/🦀️.rs`, `🔡️ansi/🦀️.rs`, `📟️vt/🦀️.rs`,
`📡️event/🦀️.rs`, `🎬️scene/🦀️.rs`, `📏️layout/🦀️.rs`, `🪀️widget/🦀️.rs`, `🖥️chrome/🦀️.rs`, `⚙️engine/🦀️.rs`,
`🔌️backend/🦀️.rs`, `🚇️pty/🦀️.rs`, `🏃️host/🦀️.rs`.

Updated: `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️.rs` (5451 → 56 lines).

Removed: none. Untouched: `🪟️windows/🦀️.rs`, the four `🧪️tests/🔬️…/🦀️.rs`, `🎯️targets/⌨️tui/🦀️.rs`, the
crate root, `Cargo.toml`, the 13 element targets, the taxonomy, every `AGENTS.md`.

Ticket inputs (kept): `w0-tui-module-split.py`, `w0-tui-module-split-statutes.ts`, this report.
Generated (in `🗑️generated/tui-split/`): `monolith.before.rs` (the pre-split copy `verify` compares
against), `check-*.log` ×5, `test-before.log`, `test-after.log`, `test-*.names.txt`. The private
`target-fleet-tui-split` dir holds 4 KB.

Docs: no README or doc comment names a region or line of the old monolith (searched `🧰️framework`,
`✏️s`, `🧪️tests`, `🧬️schema`, `🧫️fixtures`, `.vscode` for `⌨️tui`); nothing needed updating. Remaining
mentions of the old single file: `.cursor/plans/*.plan.md` (historic plans) and the gitignored, already
stale `💻️os/…/📤️distribution/🧾️manifest.json` — left alone.

## Unverified And Open

- **Windows halves — WRITTEN BUT UNVERIFIED by compilation.** `backend::native_windows`,
  `pty::windows_impl` and the `🔬️backend-native-windows-unit` include are `cfg(windows)`; no Windows
  target is installed. They are token-identical to before and the rewritten include path resolves on
  disk (all 20 `#[path]` / `include!` targets of the subtree were resolved against the filesystem).
- **Taxonomy registry — WRITTEN BUT UNVERIFIED against `verify taxonomy` (not run).** I did not edit
  `🔣️taxonomy.json`. Nine folder names match an existing kind or member row; `🔲️cell`, `🔡️ansi`, `📟️vt`,
  `📏️layout`, `🪀️widget`, `🚇️pty` have no row for this parent — the same state as the dashboard
  sub-folders (`🌀️daemon`, `🌳️command-tree`, `🖥️terminal`, …), none of which is registered either.
- **Corrupted emoji kept.** 160 comment lines of the old file carry a literal `??` / `???` where an
  emoji used to be (107 doc comments, 52 region markers; already so in the oldest commit of this path).
  The 32 top-level region markers left with the wrappers; the other 128 lines moved byte-identically,
  including the manifest's first line `//! ??? Handcrafted retained-mode terminal UI: …`. Not fixed here.
- **Pre-existing red test** `shell_window_wizard_body_paints_options_after_remount` (see above).
- `verify` stays byte-identical only until the sibling agents start editing the module files; after
  that it is expected to differ.

# Contract landing

Follow-up slice, landed 2026-10-07 19:40–19:46 (local): the §4.1 types and signatures of `fleet-plan.md`
are in place across `semio-framework-ui` and its consumer `semio-framework-repo-dashboard`, with the
smallest behaviour-preserving bodies. No signature deviates from §4.1.

Paths below are relative to `🧰️framework/🔨️modules/🖱️ui/⌨️tui/`; `DASH` is
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs`.

## Where Each §4.1 Item Lives

| §4.1 item | Location |
|---|---|
| `MouseButton { Left, Middle, Right }` | `📡️event/🦀️.rs:40` |
| `MouseKind { Down, Up, Drag, Move, Scroll { dx, dy } }` | `📡️event/🦀️.rs:48` |
| `MouseEvent { kind, pos, mods, clicks }` | `📡️event/🦀️.rs:58` (`clicks` :62) |
| `Event { …, Wake }` | `📡️event/🦀️.rs:67` (`Wake` :74) |
| `CursorShape { Block, Underline, Bar }` | `🪀️widget/🦀️.rs:44` |
| `CursorSpec { pos, shape, blink }` | `🪀️widget/🦀️.rs:52` |
| `WidgetSignal` (18 variants, §4.1 order) | `🪀️widget/🦀️.rs:20` |
| `WidgetState::on_key` | `🪀️widget/🦀️.rs:449` (unchanged) |
| `WidgetState::on_mouse` | `🪀️widget/🦀️.rs:467` |
| `WidgetState::on_paste` | `🪀️widget/🦀️.rs:475` |
| `WidgetState::set_hover` | `🪀️widget/🦀️.rs:481` |
| `WidgetState::cursor` | `🪀️widget/🦀️.rs:487` |
| `WidgetState::interactive` | `🪀️widget/🦀️.rs:493` |
| `WidgetState::tick` | `🪀️widget/🦀️.rs:498` |
| `Tui::cursor` / `hovered` / `capture` / `tick` / `layout` | `⚙️engine/🦀️.rs:70` / `:79` / `:84` / `:89` / `:95` |
| `Tui::dispatch` / `render` | `⚙️engine/🦀️.rs:126` / `:208` (signatures unchanged) |
| `ColorDepth`, `UnicodeLevel`, `Capabilities` | `🔌️backend/🦀️.rs:227`, `:236`, `:243` |
| `Waker` (`Clone`, `Send + Sync`, `new`, `wake`) | `🔌️backend/🦀️.rs:251` |
| `trait TerminalBackend` (8 methods, §4.1 order) | `🔌️backend/🦀️.rs:264` |
| unix `impl TerminalBackend for NativeTerminal` | `🔌️backend/🦀️.rs:361` (`wait` :422, `waker` :452, `copy` :457) |
| windows `impl TerminalBackend for NativeTerminal` | `🔌️backend/🦀️.rs:583` (`wait` :653, `waker` :679, `copy` :684) |

Removed: `TerminalBackend::poll`, `WidgetSignal::TerminalPassthrough`, `MouseKind::ScrollUp` /
`ScrollDown`, the dashboard's `key_to_pty_bytes` and its `🔖️Keys` region.

## What The Bodies Do Today

- **Parser** (`🔡️ansi/🦀️.rs:334`): SGR button code `& 3` maps 0 → `Left`, 1 → `Middle`, 2 **and 3** →
  `Right`. Code 3 ("no button") was `Down(3)` / `Up(3)` / `Drag(3)` before; the engine treated any
  non-zero `Down` as "focus and chrome hit, no widget click" and ignored `Up` / `Drag`, which is
  exactly what it does with `Right` now. Extra buttons (bit 7) are still masked to their low two bits,
  as before. Wheel: code 64 → `Scroll { dx: 0, dy: -1 }`, every other wheel code (65, 66, 67) →
  `Scroll { dx: 0, dy: 1 }`, mirroring the old `ScrollUp` / `ScrollDown` split (horizontal wheel is
  still reported as "down"). `clicks` is always 1. `Move` is still never emitted.
- **Sign convention** I had to choose: `dy > 0` scrolls down, `dx > 0` scrolls right (doc on
  `MouseKind`).
- **Engine dispatch**: same routing. A `Down(Left)` on the hit node now calls the generic
  `WidgetState::on_mouse(rect, event)` (`⚙️engine/🦀️.rs:155-159`); the wizard click lives behind it
  (`🪀️widget/🦀️.rs:469`). `focusable` asks `WidgetState::interactive()`, which is `true` for every
  variant — the old `matches!(…, NodeContent::Widget(_))`. `render` calls `layout()`.
- **`WindowClose(usize)`** carries `WindowState::active_stack_tab` (`🖥️chrome/🦀️.rs:128`), whichever
  tab's ✕ was hit; the dashboard ignores the index (`DASH:644`).
- **`TerminalInput`**: `key_to_pty_bytes` moved verbatim (same 14 arms, same bytes) to
  `🪀️widget/🦀️.rs:320`; the terminal widget returns `TerminalInput(bytes)` where it returned
  `TerminalPassthrough`, and `None` where the table has no encoding (the dashboard forwarded nothing
  there). `DASH:865` forwards `data` to the focused session. The dashboard had no unit vector for the
  table, so none moved; the ui vector `terminal_widget_scroll_search_and_passthrough` now asserts
  `TerminalInput(b"x")`.
- **Backend**: `wait(deadline)` is the old `poll` body inside a loop of at most 80 ms slices
  (`INTERIM_WAKE_SLICE`, the dashboard's former poll interval). For the dashboard's
  `wait(Some(now + 80 ms))` (`DASH:629`) that is one `select` and one lone-ESC flush, as before.
  **`waker` is the interim, not a self-pipe**: an `Arc<AtomicBool>` that `wait` checks before each
  slice, on both platforms, so a wake is seen within 80 ms and `wait(None)` returns on it.
  `capabilities` returns truecolor, full Unicode, no synchronized output. `present` ignores the
  cursor. `copy` writes `osc52_copy_sequence(text)` to the terminal.

Choices §4.1 leaves open (provisional, owner T-A): `ColorDepth { Monochrome, Ansi16, Ansi256,
TrueColor }` and `UnicodeLevel { Ascii, Full }` (both `Ord`); `Waker` wraps
`Arc<dyn Fn() + Send + Sync>` with a public `Waker::new(closure)` so any backend or test double can
build one. Outside §4.1 I added `impl From<io::Error> for BackendError` so
`NativeTerminal::new() -> Result<Self, BackendError>` keeps its signature while the trait speaks
`io::Result`; `BackendError` otherwise remains only in the clipboard types.

## Interim Bodies Each Slice Must Replace

**T-A terminal I/O**
- `🔌️backend/🦀️.rs:276-284` `INTERIM_WAKE_SLICE`, `interim_wait_slice`; `wait` on unix `:422-450` and
  windows `:653-677` — 80 ms polling, no resize detection, ESC flush tied to the slice.
- `waker` unix `:452`, windows `:679` — atomic flag; replace with self-pipe / event handle.
- `assumed_capabilities` `:287` and both `capabilities` — constants, no detection.
- `present` unix `:416`, windows `:648` — cursor dropped.
- `copy` unix `:457`, windows `:684` — raw OSC 52, no capability check, no native fallback.
- `ColorDepth` / `UnicodeLevel` variants `:227`, `:236` — provisional.
- `🔡️ansi/🦀️.rs:340-359` — button code 3 → `Right`, extra buttons masked, horizontal wheel → `dy: 1`,
  no `Move`, `clicks: 1`.
- `🏃️host/🦀️.rs` — untouched; `WasmHost` neither implements `TerminalBackend` nor surfaces signals
  or the cursor.

**T-B engine and windows**
- `⚙️engine/🦀️.rs:89` `tick` returns `false` and visits no widget (note `NodeMut::widget()` marks
  paint-dirty on every access; `node_raw_mut` does not).
- `:79` `hovered` is always `None` (nothing sets the field); `:84` `capture` only stores
  (`#[allow(dead_code)]` on the field, `:32`).
- `:70` `cursor` delegates, and like `dispatch` panics on a stale focus id (`Scene::node` expects).
- `:95` `layout` solves unconditionally.
- `:141-175` mouse routing: only `Down`; `on_mouse` only for `Down(Left)` on the hit node; `Up`,
  `Drag`, `Move`, `Scroll`, `Paste`, focus events and `Wake` are dropped; `on_paste` / `set_hover`
  are never called.
- `🖥️chrome/🦀️.rs:128` `WindowClose(active_stack_tab)` regardless of the tab hit; `WindowFocus`,
  `TabMoved`, `SplitterDragged`, `Hovered`, `ContextMenu` are never emitted.
- `🪀️widget/🦀️.rs:493` `interactive` is `true` for labels, dividers and chips too.

**T-C embedded terminal**
- `🪀️widget/🦀️.rs:319-339` `key_to_pty_bytes` — fixed table: no child-mode awareness, no Alt, no
  shifted or modified keys beyond Ctrl+a–z, no F-keys / Delete / Insert (those yield `None`).
- `:341-397` `terminal_on_key` still keeps PageUp, PageDown, Ctrl+Home, End, `/`, Ctrl+P for itself.
- `on_mouse` / `on_paste` / `cursor` / `tick` do nothing for `Terminal`, so `TerminalInput` carries
  keys only; `Copy` / `OpenUrl` are never emitted.

**T-D text and lists**
- `🪀️widget/🦀️.rs:467-501`: `on_mouse` is `None` for List, Table, Log, Input, Select (Wizard: left
  press selects and activates in one step, `wizard_hit`); `set_hover`, `on_paste`, `cursor`, `tick`
  are neutral for all of them, so no hover row, no input cursor, no paste.
- No painter reads `Capabilities`.

**A-3 view** (`DASH`, rewritten later): still polls every 80 ms (`:629`), passes `None` as cursor
(`:623`, `:907`), ignores the `WindowClose` index (`:644`), and lets `Wake` fall into the catch-all
arm that repaints.

## Verification

Five checks on the final sources (last ui edit 19:45:45), foreground, one at a time:

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --features tui-terminal --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished `dev` profile [unoptimized] target(s) in 0.78s`` · exit 0 |
| `cargo check -p semio-framework-ui --features tui --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished … in 0.76s`` · exit 0 |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features tui --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished … in 0.61s`` · exit 0 |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features tui-bindgen --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · ``Finished … in 0.50s`` · exit 0 |
| `cargo check -p semio-framework-repo-dashboard --message-format=short` | `Checking semio-framework-ui v0.1.0 (…)` · `Checking semio-framework-repo-dashboard v0.1.0 (…)` · ``Finished … in 0.72s`` · exit 0; again at 19:50:04 after a peer edited `DASH`: exit 0 |

Neither crate emits a warning of its own in any of the five. That the new code was type-checked: the
first check after the edit reported three `unnecessary qualification` warnings at
`🪀️widget/🦀️.rs:251`, `:252`, `:412` caused by an import I had added (removed again), and the test
and probe runs below execute the new paths.

Tests (`CARGO_TARGET_DIR=…/target-fleet-tui-split`):

| Command | Before | After |
|---|---|---|
| `cargo test -p semio-framework-ui --features tui-terminal --lib tui` | `105 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out` | `test result: FAILED. 105 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.42s` |
| `cargo test -p semio-framework-repo-dashboard --lib` | `test result: FAILED. 49 passed; 6 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.09s` (19:39) | `test result: FAILED. 49 passed; 6 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.11s` (19:46) |

Both sorted name-and-status lists are identical before and after (`diff` empty: 106 and 59 lines).
ui: the one failure is still `shell_window_wizard_body_paints_options_after_remount`. Dashboard: the
six failures are all `daemon::tests::quick::*`, each `task-launch environment before Cargo:
NotPresent`, before and after; all seven `terminal::tests::*` pass. No test was added or moved.

Mechanical test edits: `🧪️tests/🔬️unit/🦀️.rs` (8 lines: new event shape, `WindowClose(0)`,
`TerminalInput(b"x")`), `🧪️tests/🔬️backend-clipboard-mailbox/🦀️.rs` (`use std::time::Duration;`, which
it used to get from `backend`'s imports), dashboard `🖥️terminal/🧪️tests/🔬️unit/🦀️.rs` (`KeyEvent`
import, new event shape).

Runtime, unix: `w0-tui-contract-probe/` (ticket-local standalone crate) driven by
`w0-tui-contract-probe.py` on a real pseudo-terminal, macOS:

```
size 80x24 Capabilities { color: TrueColor, synchronized_output: false, unicode: Full }
idle-80ms events=[] elapsed_ms=90
idle-400ms events=[] elapsed_ms=405
wake-before-wait events=[Wake] elapsed_ms=0
wake-from-thread-at-150ms events=[Wake] elapsed_ms=180
input Key(KeyEvent { key: Char('a'), mods: 0 }) after_ms=30
input Key(KeyEvent { key: Up, mods: 0 }) after_ms=39
input Mouse(MouseEvent { kind: Down(Left), pos: Pos { x: 9, y: 4 }, mods: 0, clicks: 1 }) after_ms=33
input Mouse(MouseEvent { kind: Down(Right), pos: Pos { x: 9, y: 4 }, mods: 0, clicks: 1 }) after_ms=40
input Mouse(MouseEvent { kind: Scroll { dx: 0, dy: -1 }, pos: Pos { x: 4, y: 4 }, mods: 0, clicks: 1 }) after_ms=29
input Mouse(MouseEvent { kind: Scroll { dx: 0, dy: 1 }, pos: Pos { x: 4, y: 4 }, mods: 0, clicks: 1 }) after_ms=35
input Paste("pasted") after_ms=49
input Key(KeyEvent { key: Esc, mods: 0 }) after_ms=80
input Key(KeyEvent { key: Char('q'), mods: 0 }) after_ms=56
exit_status=0
terminal_bytes=b'\x1b[?1049h\x1b[?25l\x1b[?1002h\x1b[?1006h\x1b[?2004h\x1b[2JPROBE-READY\x1b]52;c;cHJvYmU=\x07\x1b[?2004l\x1b[?1006l\x1b[?1002l\x1b[?25h\x1b[?1049l\x1b[0m'
```

So `wait` returns at its deadline, on input and on a wake (from another thread while blocked in
`wait(None)`: 30 ms after the wake), a lone ESC is still flushed after 80 ms, and `present` / `copy`
write the patch and the OSC 52 sequence (`cHJvYmU=` is `probe`).

## Deviations And Unverified

- **Deviations from §4.1: none.**
- **Windows half of `backend` — WRITTEN BUT UNVERIFIED.** No Windows target is installed. It mirrors
  the unix half line for line (same helpers, `WAIT_TIMEOUT` from the existing ABI, used as in `pty`)
  and `rustfmt` parses the file; it has not been type-checked.
- **Dashboard on a real terminal — not run** (no daemon start on the live workspace). Its backend
  calls are the ones the probe exercised; its own loop is covered only by the unchanged unit tests.
- The change set was not instantaneous: the dashboard library did not compile from 19:40:56 (first
  ui edit) to 19:44:28 (`DASH` adapted), about three and a half minutes, and its *test* build stayed
  broken until 19:46:33 (a `KeyEvent` import I had missed in its test file).
- A peer edited `DASH` at 19:48:55 on top of this change (daemon message fields); my six adaptations
  are intact and the crate checks.
- `w0-tui-module-split.py verify` no longer reports byte-identity, as expected once modules change.

Ticket inputs added: `w0-tui-contract-probe/Cargo.toml`, `w0-tui-contract-probe/main.rs`,
`w0-tui-contract-probe.py`. Generated: `🗑️generated/tui-split/contract-*` (logs, name lists, pre-edit
copies under `contract-before/`).
