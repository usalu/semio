# Report: draw, energy, lowpoly, space, sourcing, writer — `(lib)` target warnings

All six crates verified at **0 warnings, 0 errors** on `cargo check -p <crate>` (lib target). None of
the crates' `(lib test)` targets were touched (out of scope per the parent ticket — the
`Mutation::apply`/`::diff` migration is another session's in-flight work). `semio-s-plugin-sourcing`
happened to already have a clean `(lib test)` build too (verified with `--tests`, no other-session
breakage there).

## Recurring pattern found in 5 of 6 crates: `derived_composition` module boilerplate

Every artifact's `🚪️io/🦀️component.rs` has a hand-written `mod derived_composition { ... impl
ArtifactComposition for X { fn compose(sources: &[ComposeSource]) ... } }` block. Two near-identical
warnings recurred verbatim across draw, energy, lowpoly, space (home io), and sourcing (curate io):

1. `use semio_framework_plugin::ArtifactAnalyzer as _;` — unused. The macro `derive_artifact_facets!`
   (framework/plugin) generates BOTH a trait impl (`impl ArtifactAnalyzer for $analyzer`) AND an
   **inherent** `impl $analyzer { pub fn analyze(...) }` that calls through it via UFCS — so call
   sites like `DrawAnalyzer::analyze(...)` resolve through the inherent impl and never need the
   trait in scope. Fix: delete the `use ... as _;` line. Confirmed by reading the macro definition at
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:18653` before deleting anything.
2. `fn compose(sources: &[ComposeSource])` — hidden-lifetime-parameters deprecation warning. Fix:
   `&[ComposeSource<'_>]`.

Files fixed for this pattern (both warnings, same file each time):
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (only had the unused-import half; already had `<'_>`... actually had the hidden-lifetime warning too, both fixed)

## Per-crate detail

### 1. `semio-s-plugin-draw` — 6 → 0 warnings
- `derived_composition` pattern above (2 warnings).
- `patch_value_json` (dead in `drop-layer-kind` and `move-layer` command files — genuinely unused
  copies; the SAME function name is separately defined and actually used in the sibling
  `patch-layer`/`patch-layers` command files). Deleted the dead copies + the now-unused
  `use serde_json::Value;` import in each.
- `resolve_reorder_target` (dead in `patch-layer` and `patch-layers` — genuinely unused copies; used
  in `drop-layer-kind`/`move-layer` instead). Deleted the dead copies + the now-unused
  `find_draw_layer`/`find_draw_layer_location` import in each.
- Verified crate-wide via grep before deleting each: every "dead" copy had zero call sites anywhere,
  including no `#[cfg(test)]` usage.
- Files touched: `🚪️io/🦀️component.rs`, `✏️editor/🎮️commands/🗂️drop-layer-kind/🦀️component.rs`,
  `🗂️move-layer/🦀️component.rs`, `🗂️patch-layer/🦀️component.rs`, `🗂️patch-layers/🦀️component.rs`.

### 2. `semio-s-plugin-energy` — 2 → 0 warnings
- Only the `derived_composition` pattern above. File: `🚪️io/🦀️component.rs`.

### 3. `semio-s-plugin-lowpoly` — 2 → 0 warnings
- Only the `derived_composition` pattern above. File: `🚪️io/🦀️component.rs`.

### 4. `semio-s-plugin-space` — 39 → 0 warnings (largest, structural root cause, not deletions)
This crate's plugin-root `🦀️component.rs` (~350 lines: `catalog_port`, `resolve_studio_document`,
`home_space_rows`, etc. — all doc-commented as "shared by ≥2 surfaces") looked, on the surface, like
a giant orphaned dead-code cluster: **every** function/const/struct in it except `plugin()` and the
`app_labels!` macro block was flagged `never used`, despite many of them having real call sites
elsewhere in the crate (confirmed via targeted grep, e.g. `demo_space_projection` has 12 external
call sites but was still flagged dead).

Root cause (confirmed by reading `📦️glue.rs`): the file was `#[path]`-mounted **twice** — once as
`mod space_shared;` with `pub use space_shared::*;` (making its `pub` items reachable from
`crate::X`, which is what every real call site elsewhere in the crate actually uses), and a SECOND
time as `mod plugin;` purely to get `plugin::plugin` for the `plugin_exports!` macro call. Because
`#[path]`-mounting the same file twice creates two textually-identical but semantically distinct
compiled modules, the `plugin` module's private duplicate copy of every helper function had zero
*internal* callers (nothing calls `plugin::catalog_port()` — real callers use `crate::catalog_port()`
which resolves through `space_shared`), so rustc correctly flagged that duplicate copy as dead. Every
sibling plugin (checked `draw`'s `glue.rs`) mounts its plugin-root file exactly once; `space` was the
only one double-mounting.

Fix: deleted the redundant second mount entirely —
```rust
// before
#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
// after
semio_framework_plugin::plugin_exports!(space_shared::plugin);
// (cargo fix then simplified this to `plugin` — an unnecessary-qualification lint,
//  since `pub use space_shared::*;` already puts `plugin` in scope at crate root)
```
This alone dropped the warning count from 39 to 2. Remaining two, both real, both fixed:
- `unused doc comment` on the plugin-root file at the point where an outer `///` doc comment preceded
  an `app_labels! { ... }` macro invocation (macros don't propagate a leading outer doc comment to
  their expansion unless it's written inside the macro call). Converted the leading block from `///`
  to `//` (plain comment) since it documents the region/rationale, not a specific generated item.
- The `derived_composition` pattern (home io file, `ArtifactAnalyzer as _` unused import).

Files touched: `📦️glue.rs`, plugin-root `🦀️component.rs`,
`🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`.

**Nothing was deleted from this crate** — the fix was structural (de-duplicating a module mount),
not a dead-code cleanup, and every function/const/struct that looked dead is genuinely live via the
`space_shared` re-export path.

### 5. `semio-s-plugin-sourcing` — 7 → 0 warnings
- `derived_composition` pattern (curate io file).
- `extern crate semio_framework_os_kernel as vcs;` in `glue.rs` — unused under a plain `cargo check`
  but genuinely used inside a `#[cfg(test)] mod tests` block in
  `🧬️schema/🧬️mutations/🦀️component.rs` (the exact "invisible to non-`--tests` check" hazard from
  the ticket's methodology). Fixed by gating the `extern crate` line itself with `#[cfg(test)]`
  rather than deleting it (confirmed it's the ONLY file in the crate referencing `vcs::`).
- `unused doc comment` on the plugin-root `curate/🦀️component.rs`, same shape as space's: an outer
  `///` doc block preceded a `thread_local! { ... }` macro invocation instead of the `static` inside
  it. Fixed by moving the doc comment inside the `thread_local! {}` braces, directly above the
  `static` it actually documents (`thread_local!`'s own macro_rules forwards `$(#[$attr:meta])*` onto
  the generated static, so this is a semantics-preserving move, not just a warning suppression).
- `App` unused import in `✏️editor/🦀️component.rs` — same "only used inside `#[cfg(test)] mod
  testkit` via `use super::*`" hazard. Fixed by removing it from the outer `use` list and adding
  `App` to the inner `#[cfg(test)] mod testkit`'s own explicit `use semio_framework_plugin::{App, ...}`.
- `EMPTY_EXAMPLE_ID` and `crate::artifacts::curate::schema::empty_document` unused imports in
  `set-artifact-json/🦀️component.rs` — same hazard, both only used inside that file's `#[cfg(test)]
  mod tests`. Moved both into the test module's own `use` block.
- Files touched: `📦️glue.rs`, plugin-root `🗂️curate/🦀️component.rs`, `🚪️io/🦀️component.rs`,
  `✏️editor/🦀️component.rs`, `✏️editor/🎮️commands/📄️set-artifact-json/🦀️component.rs`.
- Sanity-checked `cargo check -p semio-s-plugin-sourcing --tests` afterward: also 0 warnings, 0
  errors — this crate isn't touched by the other session's `Mutation::apply`/`::diff` migration.

### 6. `semio-s-plugin-writer` — 4 → 0 warnings
- `derived_composition` pattern (writer io file).
- `private item shadows public glob re-export` in `🧬️schema/🔺️diff/📝️text/🦀️component.rs`: the
  file had both `use crate::artifacts::writer::schema::diff::WriterDiff;` (private, explicit) AND
  `pub use crate::artifacts::writer::schema::diff::*;` (public glob) importing the exact same item —
  the private explicit import always wins local resolution, silently defeating the glob's intent to
  make `WriterDiff` public through this re-export path. Fixed by deleting the redundant private
  import; the local `impl WriterDiff { ... }` block still resolves `WriterDiff` through the glob,
  which is now unambiguously the public one.
- `struct WireWriterIdiom is never constructed` in the artifact's `🧬️schema/🦀️component.rs`. This
  was a genuine judgment call, resolved as a real (small, additive, low-risk) fix rather than a
  deletion or a suppression: `WireWriterIdiom` is a complete, correct `dsl::DslIdiom` impl for the
  `"wire"` embedded language, doc-commented as the sibling of `JackWriterIdiom` ("see
  `JackWriterIdiom`'s doc comment for why it lives here"). `JackWriterIdiom` avoids the same warning
  solely because `pub fn jack_completions_json` calls `<JackWriterIdiom as dsl::DslIdiom>::complete`
  — `WireWriterIdiom` had no such caller anywhere (confirmed by grep: zero references outside its own
  decl/impl, no wasm-cfg-gated usage either). Rather than deleting a fully-implemented trait impl or
  leaving the crate with an unresolved warning, added the obviously-intended, previously-missing
  sibling function `pub fn wire_completions_json(text: &str, cursor: usize) -> Option<String>`
  immediately after `jack_completions_json`, calling `<WireWriterIdiom as dsl::DslIdiom>::complete`
  — identical shape to the existing function, purely additive, doesn't change any existing behavior
  or call site. Did NOT attempt to unify `WireWriterIdiom::classify`/the hand-duplicated `"wire"`
  branch in `tokenize_language` (they return different shapes — `TokenClass`/`TextSpan` vs.
  `GrammarToken`'s `String`/byte-offset shape — and `tokenize_language` deliberately bypasses the
  generic `dsl::idiom()` registry's less-precise fallback for byte-accurate spans; unifying that is a
  larger, riskier refactor outside this ticket's warning-fixing scope).
- Files touched: `🚪️io/🦀️component.rs`, `🧬️schema/🔺️diff/📝️text/🦀️component.rs`,
  `🧬️schema/🦀️component.rs`.

## Nothing left unresolved
All six crates' `(lib)` targets are confirmed 0 warnings / 0 errors as of this report. No
`#[allow(...)]` was used anywhere. No `(lib test)` errors from the other session's
`Mutation::apply`/`::diff` migration were touched.

## Full file list (created/modified only — nothing deleted as a file)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️drop-layer-kind/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️move-layer/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️patch-layer/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️patch-layers/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-artifact-json/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
