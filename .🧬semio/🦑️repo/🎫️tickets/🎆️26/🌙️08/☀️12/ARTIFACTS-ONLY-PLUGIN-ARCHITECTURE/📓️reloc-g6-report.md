# g6 declaration()/pilot_languages() relocation — 🧩️puzzle, 🪐️space, 🔋️energy

Scope: for every `pub fn declaration()` under `🗿️artifacts/<a>/…/⚙️engine/🦀️component.rs` in the three
assigned plugins, move `declaration()` and its private helper `pilot_languages()` to that artifact's
root `🗿️artifacts/<a>/🦀️component.rs`, keeping `pilot_languages` **private** (no `pub`). Widen nothing.

## Enumeration (`grep -rln "fn declaration"`)

- `🧩️puzzle`: 3 hits — `🧊️3d/🦀️component.rs` (already at root, see deviation below), `🖐️5d/…/⚙️engine/🦀️component.rs`, `◻2d/…/⚙️engine/🦀️component.rs`
- `🪐️space`: 1 hit — `🏠️home/…/⚙️engine/🦀️component.rs`
- `🔋️energy`: 1 hit — `🔋️model/…/⚙️engine/🦀️component.rs`

## 🧩️puzzle — `semio-s-plugin-puzzle`

### `puzzle3d` (🧊️3d) — DEVIATION: reverted an already-applied `pub` widening
Found in the state I inherited: `declaration()` had *already* been moved to the artifact root
(`🗿️artifacts/🧊️3d/🦀️component.rs:548`), but its helper had been left behind in `⚙️engine` and made
**`pub fn pilot_languages()`** (`🧊️3d/…/⚙️engine/🦀️component.rs:574`) so the root could reach it by a
qualified path (`crate::artifacts::puzzle3d::standards::v1::engine::pilot_languages()`). This is
exactly the site the dispatch warned about (earlier pass told agents to make it `pub`). Fixed: moved
`pilot_languages()` body into the root file as a **private** `fn`, deleted it from `⚙️engine`, and
changed `declaration()`'s call back to the unqualified `pilot_languages()`.

- Before: `🧊️3d/…/⚙️engine/🦀️component.rs:574` `pub fn pilot_languages()`; root `declaration()` called it via full qualified path.
- After: `🧊️3d/🦀️component.rs` — `declaration()` at line 548, private `fn pilot_languages()` immediately below it (same `//#region 🔖️Declaration`); zero `pilot_languages` left in `⚙️engine`.

### `puzzle5d` (🖐️5d) — move-both, clean
- Before: `declaration()` + private `fn pilot_languages()` at `🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:296` and `:210` (region `🔖️IoFacet` / bare fn above `🔖️ArtifactEngine`).
- After: both moved verbatim (still private) to `🗿️artifacts/🖐️5d/🦀️component.rs`, new `//#region 🔖️Declaration` right after `//#endregion 🔖️ArtifactKind`. `register_mesh_io()` stayed in `⚙️engine` untouched.

### `puzzle2d` (◻2d) — move-both, clean
- Before: `declaration()` + private `fn pilot_languages()` at `◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:59` and `:91` (region `🔖️Register`, shared with `register_media_io()`/`register_app_schemas()`).
- After: both moved to `🗿️artifacts/◻2d/🦀️component.rs`, new `//#region 🔖️Declaration`. `register_media_io()`/`register_app_schemas()` stayed in `⚙️engine`, region left non-empty.

### Plugin-root call site — `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`
```
- .artifact(crate::artifacts::puzzle2d::standards::v1::engine::declaration())
- .artifact(crate::artifacts::puzzle3d::standards::v1::engine::declaration())
- .artifact(crate::artifacts::puzzle5d::standards::v1::engine::declaration())
+ .artifact(crate::artifacts::puzzle2d::declaration())
+ .artifact(crate::artifacts::puzzle3d::declaration())
+ .artifact(crate::artifacts::puzzle5d::declaration())
```
`setup()`'s `register_app_schemas`/`register_media_io`/`register_mesh_io` calls untouched (still `standards::v1::engine::…`, correct — those functions stayed in `⚙️engine`).

## 🪐️space — `semio-s-plugin-space`

### `home` (🏠️home) — move-both, DEVIATION: unqualified `OnceLock` fixed
- Before: `declaration()` + private `fn pilot_languages()` at `🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:32` and `:45` (region `🔖️Register`).
- Deviation: this `pilot_languages()` body used bare `OnceLock` (`static LANGUAGES: OnceLock<…> = OnceLock::new();`), resolved only via that file's own top-level `use std::sync::{Mutex, OnceLock};`. That `use` does not travel with a two-function move, and every sibling artifact's own `pilot_languages()` already spells this fully qualified — so instead of adding a new top-level `use` to the root file, I qualified it as `std::sync::OnceLock` (matching convention), and left the deviation documented inline. The `⚙️engine` file's own `use std::sync::{Mutex, OnceLock};` is left as-is (both names were already effectively unused elsewhere in that file before my edit; not in scope to touch).
- After: both moved to `🗿️artifacts/🏠️home/🦀️component.rs`, new `//#region 🔖️Declaration` right after `//#endregion 🔖️ArtifactKind`. The now-empty `//#region 🔖️Register` / `//#endregion 🔖️Register` pair in `⚙️engine` was removed (it held only these two functions).

### Plugin-root call site — `✏️s/🔌️plugins/🪐️space/🦀️component.rs`
```
- .artifact(crate::artifacts::home::engine::declaration())
+ .artifact(crate::artifacts::home::declaration())
```
(Stale doc-comment cross-references to `crate::artifacts::home::engine::declaration()` remain in
`📦️packages/🦀️rust/📦️glue.rs:480` and three `🎛️apps/*` files for `puzzle` — prose only, not call
sites; out of this pass's scope, noted here for whichever pass next touches those files.)

## 🔋️energy — `semio-s-plugin-energy`

### `model` (🔋️model) — move-both, DEVIATION: unqualified `io_registry::entries()` fixed
- Before: `declaration()` + private `fn pilot_languages()` at `🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:45` and `:67` (region `🔖️Register`, alongside `register_document_codec()` which stayed).
- Deviation: `declaration()`'s body called `.composers(io_registry::entries())` — an **unqualified same-file sibling reference** to that `⚙️engine` file's own `pub mod io_registry { … }` (the module with the real `ComposerEntry` rows, distinct from the root file's own thin `io_registry` wrapper of the same name). This does not survive a plain move, so per instruction 3 I qualified rather than moved: `.composers(crate::artifacts::model::standards::v1::engine::io_registry::entries())`.
- After: `declaration()` + private `pilot_languages()` moved to `🗿️artifacts/🔋️model/🦀️component.rs`, new `//#region 🔖️Declaration` right after `//#endregion 🔖️ArtifactKind`. `register_document_codec()` stayed in `⚙️engine`.

### Plugin-root call site — `✏️s/🔌️plugins/🔋️energy/🦀️component.rs`
```
- .artifact(crate::artifacts::model::standards::v1::engine::declaration())
+ .artifact(crate::artifacts::model::declaration())
```
`.setup(crate::artifacts::model::standards::v1::engine::register_document_codec)` untouched (function stayed put).

## VERIFY — four greps, per plugin

```
$ grep -rn "fn declaration" <plugin>   # exists at artifact root, gone from ⚙️engine
🧩️puzzle: 🧊️3d/🦀️component.rs:548, 🖐️5d/🦀️component.rs:544, ◻2d/🦀️component.rs:449
🪐️space:  🏠️home/🦀️component.rs:47
🔋️energy: 🔋️model/🦀️component.rs:60

$ grep -rn "engine::declaration" <plugin>   # zero real call sites (doc-comment prose only, listed above)
🧩️puzzle: 3 hits, all `///`/`//` doc comments in 🎛️apps/*
🪐️space:  1 hit, doc comment in 📦️glue.rs
🔋️energy: 0 hits

$ grep -rn "pub fn pilot_languages" <plugin>   # zero hits everywhere
🧩️puzzle: 0   🪐️space: 0   🔋️energy: 0

$ [#[path] resolution] — every #[path] in each plugin's 📦️glue.rs resolves on disk (no files moved, only edited in place) — verified by script, 0 missing for all three glue.rs (🧩️puzzle: 📦️packages/🦀️rust/📦️glue.rs; 🪐️space & 🔋️energy: 📦️packages/🦀️rust/📦️glue.rs)
```

## CARGO — one `cargo check -p <crate> --all-targets` each, with the mandated override

Command template used for all three:
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target" cargo check -p <crate> --all-targets
```

### `semio-s-plugin-puzzle` — **GREEN**
`Finished `dev` profile [unoptimized] target(s) in 5m 32s` — 0 errors. 67 lib warnings / 79 lib-test
warnings, all pre-existing and unrelated (unused imports, unused variables, dead fields/fns in files
this pass never touched).

### `semio-s-plugin-energy` — **GREEN**
`Finished `dev` profile [unoptimized] target(s) in 5m 57s` — 0 errors. 9 lib warnings / 10 lib-test
warnings, same pre-existing/unrelated character.

### `semio-s-plugin-space` — **code-complete, UNVERIFIED green (pre-existing upstream errors, none mine)**
Build surfaces 8 lib errors + 12 lib-test errors (`error[E0432]`, `E0609`, `E0560`, `E0308`, `E0599`),
all in files this pass never touched:
- `use dsl::ArtifactEngine;` (`🏠️home/…/⚙️engine/🦀️component.rs:38`, inside the pre-existing `#[cfg(test)] mod tests` block) — `E0432: no ArtifactEngine in the root`. `dsl::ArtifactEngine` trait appears to have been renamed/removed elsewhere in the tree by another in-flight session. This cascades into `E0599` "no method `apply`/`snapshot`/`artifact` found for `SHomeEngine`" three times, in the same untouched test.
- `CsvSnapshot` no longer has `headers`/`rows` fields (`E0609`/`E0560`) in `🏠️home/…/🚪️io/📥️import/…/📊️csv/…` and `…/📤️export/…/📊️csv/…` — an upstream schema change, not touched by this pass.
- `serde_json::Value` vs `JsonValue` mismatch (`E0308` ×3) in `🏠️home/…/🚪️io/{import,export}/…/🔣️json/…` — likewise untouched.
- `OsAppRegistration` has no `document` field (`E0609`) in `🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs` — app-panel file, untouched.

Confirmed pre-existing/unrelated, not introduced by this relocation: `git diff --stat` against HEAD
for every one of those files (`🏠️home/…/🚪️io/**`, `🎛️apps/🪐️space/📌️panels/🛍️catalogue/**`) returns
**empty** — they are byte-identical to HEAD, so today's errors there cannot be a side effect of any
edit in this pass. My own edited regions (`🗿️artifacts/🏠️home/🦀️component.rs`'s new
`//#region 🔖️Declaration`, the `⚙️engine` file's now-empty-then-removed `//#region 🔖️Register`, and
the plugin-root `.artifact(...)` call) produce **zero** errors or warnings of their own — every error
line in the log traces to a file/region outside this pass's diff. Per the ticket's stdio precedent I
am not reporting this green (real output pasted above, not fabricated), but I am also not attempting
to fix upstream CsvSnapshot/JsonValue/OsAppRegistration/dsl::ArtifactEngine breakage — out of scope
and explicitly forbidden ("leave everything else alone" / don't work around upstream breakage).

## apa-status

**complete but UNVERIFIED for `semio-s-plugin-space`** (real cargo output pasted above; every error
traces to files this pass never touched, confirmed via empty `git diff --stat` on those exact paths) —
**GREEN for `semio-s-plugin-puzzle` and `semio-s-plugin-energy`** (0 errors, full `--all-targets`
including `#[cfg(test)]`, mandated `sccache` override applied).

Files touched (created/updated), full list:
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️component.rs` (moved `pilot_languages` in from `⚙️engine`, reverted its `pub`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed `pub fn pilot_languages`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs` (added `declaration()`+`pilot_languages()`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed same)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs` (added `declaration()`+`pilot_languages()`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed same)
- `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs` (3 call-site path updates)
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️component.rs` (added `declaration()`+`pilot_languages()`)
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed same, dropped now-empty region)
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs` (call-site path update)
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️component.rs` (added `declaration()`+`pilot_languages()`, qualified `io_registry::entries()`)
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed same)
- `✏️s/🔌️plugins/🔋️energy/🦀️component.rs` (call-site path update)

`🎯️target` scratch build dir and `scratch-*-check.txt` logs left in this ticket folder per the ticket
folder rule (not deleted).
