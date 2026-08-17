# Flow Extensions + Editor — Warning Cleanup Report

Scope: 8 small crates, verified via `cargo check -p <crate> --message-format=short`. All
sequential, foreground (synchronous) checks per the ticket's anti-background-hang instruction.

## Result summary

| # | Crate | Starting `(lib)` warnings | Ending `(lib)` warnings | Errors |
|---|---|---|---|---|
| 1 | `semio-s-plugin-flow-extension-brep` | 5 | 0 | 0 |
| 2 | `semio-s-plugin-flow-extension-dictionary` | 0 (already clean) | 0 | 0 |
| 3 | `semio-s-plugin-flow-extension-list` | 0 (already clean) | 0 | 0 |
| 4 | `semio-s-plugin-flow-extension-logic` | 0 (already clean) | 0 | 0 |
| 5 | `semio-s-plugin-flow-extension-math` | 0 (already clean) | 0 | 0 |
| 6 | `semio-s-plugin-flow-extension-primitive` | 0 (already clean) | 0 | 0 |
| 7 | `semio-s-plugin-flow-extension-text` | 0 (already clean) | 0 | 0 |
| 8 | `semio-framework-editor` | 1 | 0 | 0 |

All 8 crates re-verified clean in a final pass after the fixes (each run individually,
synchronously, with a generous timeout — no backgrounding used).

## 1. `semio-s-plugin-flow-extension-brep` — 5 → 0

File: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs` (this crate's own dedicated
component file — confirmed via its `📦️glue.rs`'s `#[path = "../../🦀️component.rs"]`, it is
**not** shared with sibling extensions; each extension under `🧩️extensions/<name>/` has its own
`🦀️component.rs` one level up from its `📦️glue.rs`).

5 unused-import warnings, all top-level `use` items (`evaluate_json`, `Atom`, `Value`, `Brep`,
`BrepKernel`, `Mutex`, `OnceLock`) that were only actually used inside the crate's own
`#[cfg(test)] mod tests { ... }` block (confirmed via crate-wide grep before touching anything —
per the ticket's dead-code/unused-import triage method: a plain `cargo check` doesn't compile
`#[cfg(test)]` code, so these warned dead in the lib-only compilation unit despite being real test
dependencies). `BrepKernel` specifically had zero references anywhere in the file, including
tests (only mentioned in a doc comment) — genuinely unused, dropped entirely. `block_on` from the
same import line stayed at module scope — it's used by the always-compiled `geo_operation!` /
`num_operation!` macro bodies outside tests.

Fix: trimmed the top-level imports to only what the always-compiled (non-test) code uses, and
added the removed names (`Atom`, `Value`, `Brep`, `Mutex`, `OnceLock`) as new local imports inside
`mod tests` (which already has `use super::*;`, so these needed their own explicit `use` lines
since they no longer exist at module scope for `super::*` to pull in).

Also confirmed (read-only, no edit) that this crate's `mod extension_guest { ... }` (gated
`#[cfg(feature = "component-guest")]`, default-enabled) calls `bundle()` via
`semio_framework_plugin::extension_exports!(bundle)` — traced the macro definition in the shared
`🔌️plugin/🦀️component.rs` and confirmed `bundle()` is invoked unconditionally on native builds too
(only one inner `static` inside the macro expansion is wasm32-gated), so `bundle`/`extension_guest`
were never actually dead code here and needed no `#[cfg(target_arch = "wasm32")]` gating — unlike
the `imperative-*` family pattern documented earlier in this ticket's `📓️progress.md`. Left as-is.

## 2–7. Six flow extensions already clean

`dictionary`, `list`, `logic`, `math`, `primitive`, `text` all reported 0 warnings / 0 errors on
first check — no changes needed. Consistent with the ticket brief's expectation that another
agent's earlier session-wide pass on the shared `🔌️plugin/component.rs` (and, evidently, prior
work on these specific extension files too) already cleaned most of this family.

Note: `semio-s-plugin-flow-extension-primitive`'s check ran past the 5-minute foreground timeout
once and the harness auto-backgrounded it; per the ticket's hazard warning about subagents never
receiving background-task notifications, I polled its output file directly via `Read` rather than
waiting on a notification, and separately re-ran it synchronously in the final verification pass
to confirm 0 warnings — both agree.

## 8. `semio-framework-editor` — 1 → 0

File: `🧰️framework/🔨️modules/✍️editor/🦀️component.rs`.

Single warning: `hidden_glob_reexports` ("private item shadows public glob re-export") on
`use canvas::camera::{Camera, Viewport};` at line 6. Diagnosed with the full (non-short) rustc
output, which pointed at `pub use infinite_canvas::{self as canvas, *};` (line 8) as the glob that
already makes `Camera` publicly reachable (traced the actual re-export chain back through
`infinite_canvas`'s `📦️glue.rs` → `board::ports::directed_normal::board_host` → its private
`use crate::infinite::canvas::camera::Camera;` — same underlying `canvas::camera::Camera` type,
just surfaced publicly through a different, deeper glob path than this crate's own explicit
import). Since it's the identical type either way, the local private import of `Camera` was purely
redundant with — and shadowing — the already-public glob re-export.

Fix: dropped `Camera` from the explicit import, keeping only `use canvas::camera::Viewport;`
(`Viewport` wasn't flagged — it has no public glob path, so it still needs the explicit import).
`Camera` now resolves for all in-file uses (lines 84, 88, 92, 193, 227, etc.) via the existing
`pub use infinite_canvas::{self as canvas, *};` glob. Verified 0 warnings after.

## Left alone / out of scope

- `semio-s-plugin-stdio` (lib): still showed ~22–107 warnings (count fluctuated between runs —
  another session appears to be actively editing it concurrently) while checking dependents of
  these crates. Not one of this pass's 8 assigned crates — untouched, per scope.
- `semio-framework-os-flow` (lib, package `flow_extension_sdk`): 11 warnings (unused
  `directed_dag as dag` / `crate::drawing` imports across several files, plus one
  `FlowExtensionRegistryState` privacy-mismatch warning). This is a shared dependency of all 7
  flow extensions but is not itself one of the 8 assigned crates — left untouched, noted here for
  visibility in case a future pass wants to pick it up.
- The shared `🔌️plugin/🦀️component.rs` file: not re-touched, per explicit instruction (another
  agent already worked on it this session). No warnings attributed to it surfaced while checking
  these 8 crates.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
- `🧰️framework/🔨️modules/✍️editor/🦀️component.rs`

No files deleted. No `git` commands run. No `#[allow(...)]` used anywhere.
