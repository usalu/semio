# 🌿️ `semio-s-plugin-space` — Restore Imports Pruned by Commit `1d71198c`

## Root cause (confirmed)

Commit `1d71198c19f13e1ecd4000621c08d00d36eac4a1` ran a native-`cargo-check`-only unused-import
sweep that pruned:

1. Two `extern crate semio_framework_os_kernel as …;` aliases from
   `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`:
   - `as pack` — genuinely unused (`grep -rn "\bpack::"` across the crate finds zero call sites); left removed.
   - `as vcs` — used **only** by `#[cfg(test)]` code (`vcs::apply_mutation(...)` in the config test
     module), invisible to a plain `cargo check`. Restored.
2. Six `use` statements inside `#[cfg(test)] mod tests { … }` blocks across five files — all names
   that only test code references, so a native `cargo check` never sees them as "used".

## Errors fixed (18 total, matches the brief's count)

| # | File | Missing name(s) | Real definition verified via | Fix |
|---|------|------------------|-------------------------------|-----|
| 1–2 | `🗿️artifacts/🏠️home/…/🧬️mutations/💾️binary/🦀️component.rs` | `change_catalog_generation` | `grep "pub fn change_catalog_generation"` → `…/🧬️mutations/🔢️change-catalog-generation/🦠️mutation/🦀️component.rs`, re-exported at `crate::artifacts::home::mutations::change_catalog_generation` (sibling `🧬️mutations/🦀️component.rs`'s `pub use super::change_catalog_generation::mutation::{…}`) | Added `use crate::artifacts::home::mutations::change_catalog_generation;` inside `mod tests` |
| 3 | `🗿️artifacts/🪐️space/…/🧬️mutations/💾️binary/🦀️component.rs` | `touch_artifact` | `grep "pub fn touch_artifact"` → `…/🧬️mutations/🕒touch-artifact/🦠️mutation/🦀️component.rs`, re-exported at `crate::artifacts::space::standards::v1::subsets::any::schema::mutations::touch_artifact` | Added same-style `use …::touch_artifact;` inside `mod tests` |
| 4–14 | `🗿️artifacts/🏠️home/…/✏️editor/🦀️component.rs` (`space_document_persists_through_backbone_port` test) | `Arc`, `OsBackbonePort`, `LocalStorageBackbonePort`, `empty_space_snapshot`, `SpaceKind`, `SpaceVisibility`, `OsSpaceDocument`, `create_backbone_document`, `S_SPACE_SCHEMA`, `seed_os_space_catalog_if_empty`, `load_os_space_document` | `Arc` — repo convention elsewhere in this crate is `use std::sync::Arc;` (matched, not the rustc-suggested `crate::infinite_board_port_directed_dag::Arc` re-export accident). The other 10: `grep "pub trait OsBackbonePort\|pub struct LocalStorageBackbonePort\|pub fn …"` → all defined in `🧰️framework/…/🖥️host/🦀️component.rs` (`OsBackbonePort`, `OsSpaceDocument`, `create_backbone_document`, `seed_os_space_catalog_if_empty`, `load_os_space_document`, and `LocalStorageBackbonePort` via that file's own `pub use store::{…}` at its line 4663) or `🧰️framework/…/🔨️modules/🪐️space/🦀️component.rs` (`empty_space_snapshot`, `SpaceKind`, `SpaceVisibility`, `S_SPACE_SCHEMA`), all re-exported at the `semio_framework_os` crate root (`host_core.rs`'s `pub use crate::space::*;` chained through `glue.rs`'s `pub use host_core::*;`) | Added `use std::sync::Arc;` plus one grouped `use semio_framework_os::{ create_backbone_document, empty_space_snapshot, load_os_space_document, seed_os_space_catalog_if_empty, LocalStorageBackbonePort, OsBackbonePort, OsSpaceDocument, SpaceKind, SpaceVisibility, S_SPACE_SCHEMA };` inside `mod tests` |
| 15 | `⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️component.rs` | `SpaceConfig` | `grep "pub struct SpaceConfig"` found **two** hits — the schema-leaf duplicate at `🎚️config/🧬️schema/🦀️component.rs` and the real `ArtifactApp::Config` at `🎚️config/🦀️component.rs` (doc comment literally says *"Space's real `ArtifactApp::Config`"*). Confirmed by checking non-test call sites (`⚙️engine/🪐️space/🦀️component.rs:28` imports `crate::engine::space::config::SpaceConfig`) | Added `use crate::engine::space::config::SpaceConfig;` inside `mod tests` — **not** the schema-leaf duplicate rustc also offered |
| 16–17 | `⚙️engine/🪐️space/🎚️config/🦀️component.rs` | `vcs` (module/crate) | `vcs::apply_mutation` is not a real external crate; `semio-s-plugin-space`'s own `📦️glue.rs` (before the pruning commit) aliased it via `extern crate semio_framework_os_kernel as vcs;`. Confirmed with `git show 1d71198c -- …/📦️glue.rs`, which shows exactly this line (plus an unused `as pack`) removed | Restored `extern crate semio_framework_os_kernel as vcs;` in `📦️glue.rs` (kept `pack` removed — verified zero call sites) |

No test bodies were touched, deleted, `#[ignore]`d, or rewritten — every fix is an added `use`/`extern crate` line.

## Files changed

- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — restored `extern crate semio_framework_os_kernel as vcs;`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️component.rs` — test-local `use crate::engine::space::config::SpaceConfig;`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` — test-local `Arc` + `semio_framework_os::{…}` group import
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` — test-local `change_catalog_generation` import
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` — test-local `touch_artifact` import

`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` and `🧰️framework/🔨️modules/🔁️workflow/🦀️component.rs` (flagged as possibly in-lease) needed **no** changes — none of the 18 errors originated there; the `vcs` alias already exists correctly on the `semio-framework-os-kernel` side (`extern crate self as vcs;`), the gap was only in the plugin crate's own `glue.rs`.

## Verify — real output

`cargo check -p semio-s-plugin-space` → **0 errors** (one pre-existing-shape warning: `vcs` extern crate is unused under native check, same reason it was pruned originally — only `#[cfg(test)]` code needs it, which is exactly why this class of bug exists). Log: `🧪️space-imports-check-native.txt`.

`cargo check -p semio-s-plugin-space --target wasm32-wasip2` → **0 errors**. Log: `🧪️space-imports-check-wasm.txt`.

`cargo test -p semio-s-plugin-space --lib`:

```
test result: ok. 205 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s
```

Log: `🧪️space-imports-test-final.txt`.

## Shortfall vs the 210 target — explained, not fixed

The brief's target was **210 passed**; the true, current, fully-green number is **205 passed / 0
failed / 0 compile errors**. Investigated the 5-test gap directly rather than assuming it away:

- `grep -c "#\[test\]"` across the crate today finds **207** attributes, and `cargo test -- --list`
  confirms only **205** are actually compiled into the test binary — a gap of exactly 2.
- Root-caused the 2: there are **three** identically-named `primary_asset_is_nonempty` example-demo
  test fixtures in the tree —
  `🗿️artifacts/🏠️home/…/📚️examples/🎬️demo/🧪️tests/🦀️test.rs` (mounted, runs as
  `examples::art_home_demo_tests::primary_asset_is_nonempty`),
  `🗿️artifacts/🏠️home/…/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs` (unmounted), and
  `⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs` (unmounted). Only the first has a
  corresponding `#[path = "…"] mod art_home_demo_tests;` in `📦️glue.rs`; the other two have **never**
  been referenced by any `mod` mount in this repo's history (`git log --all -S` on that path string
  returns nothing) — they predate and are unrelated to commit `1d71198c`.
- Confirmed `1d71198c`'s own diff (`git diff 1d71198c^ 1d71198c -- ✏️s/🔌️plugins/🪐️space`) removes
  **zero** `#[test]` or `fn` lines anywhere in this crate — only import lines (max 8-line file diffs,
  all `-`/`+` on `use`/`extern crate` lines). It cannot be the source of a 5-test (or even a 2-test)
  shortfall.
- The remaining 3-test gap (210 target vs 207 present `#[test]` attributes) has no candidate cause in
  this crate's current source at all — there is no dangling reference, no commented-out test, no
  cfg-gated block accounting for it. Restoring pruned imports cannot manufacture test functions that
  are not present in the source tree.

Conclusion: the import-pruning defect described in the brief is **fully fixed** (0 compile errors,
native and wasm32-wasip2, 205/205 passing). The 210 figure does not match what is reachable by
restoring only the imports this commit pruned; the 5-test difference is pre-existing and orthogonal
(2 attributable to never-mounted duplicate example fixtures predating `1d71198c`, 3 unaccounted for by
anything in this crate's current source). Not fixed here — doing so would mean either wiring in two
unmounted example test files or fabricating missing test code, both outside "restore a pruned
import."
