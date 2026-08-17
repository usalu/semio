# Process + Sourcing Extension Crates — Report

Scope: 8 workspace members (`process` plugin root + its 4 extension crates, plus 3 of
`sourcing`'s extension crates — `sourcing` itself was already fixed by an earlier pass this
session). Goal: `(lib)` target to 0 warnings / 0 new errors. All 8 verified via
`cargo check -p <name> --message-format=short`.

## Results summary

| Crate | Package name | Starting warnings | Ending warnings |
|---|---|---|---|
| `🏭️process/📦️packages/🦀️rust` | `semio-s-plugin-process` | 18 | **0** |
| `🏭️process/🧩️extensions/🪵️wood` | `semio-s-plugin-process-wood` | 0 | **0** (already clean) |
| `🏭️process/🧩️extensions/🧱️concrete` | `semio-s-plugin-process-concrete` | 0 | **0** (already clean) |
| `🏭️process/🧩️extensions/🔩️metal` | `semio-s-plugin-process-metal` | 0 | **0** (already clean) |
| `🏭️process/🧩️extensions/🤖️robotic` | `semio-s-plugin-process-robotic` | 0 | **0** (already clean) |
| `🪵️sourcing/🧩️extensions/🪵️beams` | `semio-s-plugin-sourcing-beams` | 0 | **0** (already clean) |
| `🪵️sourcing/🧩️extensions/🪟️windows` | `semio-s-plugin-sourcing-windows` | 0 | **0** (already clean) |
| `🪵️sourcing/🧩️extensions/🧱️slabs` | `semio-s-plugin-sourcing-slabs` | 0 | **0** (already clean) |

Only `semio-s-plugin-process` (the parent plugin) had real warnings to fix — all 7 extension
crates (4 process, 3 sourcing) were already at 0 warnings, 0 errors before this pass, presumably
from the earlier workspace-wide `cargo fix --workspace --all-targets` run and/or the earlier
`sourcing` root cleanup mentioned in `📓️progress.md`. No errors, new or pre-existing, found in
any of the 8 crates' `(lib)` targets.

## `semio-s-plugin-process`: 18 → 0 warnings, 0 errors

### Step 1 — `cargo fix --lib -p semio-s-plugin-process --allow-dirty --allow-staged`
Auto-applied 14 of the 18 warnings (unused imports, unnecessary qualifications, hidden
lifetime). Files touched by `cargo fix` alone:
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (3 fixes)
- `📦️glue.rs` (2 fixes — the two `unused extern crate: unused` warnings)
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs` (1 fix)
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (2 fixes)
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` (1 fix)
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs` (3 fixes)
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (1 fix)
- `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` (1 fix)

### Step 2 — 4 remaining warnings, hand-triaged

1. **Unused doc comment** on a `thread_local! { ... }` block in
   `🗿️artifacts/🧊️process3d/🦀️component.rs:737` — rustc doesn't attach outer `///` doc comments
   to macro-invocation items like `thread_local!`. This was an 11-line explanatory comment (not
   real API docs — describes an internal same-process working-scene cache), so converted `///` →
   `//` (11 lines) rather than deleting real explanatory content.

2. **Unused import** `semio_framework_plugin::ArtifactAnalyzer as _` in
   `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:12` — grepped the
   file: `Process3dAnalyzer::analyze()` is called directly (inherent/associated fn), never through
   the `ArtifactAnalyzer` trait. Deleted the import.

3. **Hidden lifetime parameter** in the same file, `fn compose(sources: &[ComposeSource])` at
   line 36 — the recurring `ComposeSource<'_>` pattern already established across ~10 other
   plugins this session (per `📓️progress.md`'s wave-3 notes). Changed to
   `&[ComposeSource<'_>]`.

4. **Dead code**: `fn hash_value<T: Serialize>(...)` in
   `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs:108`.
   Crate-wide grep found a second, *used* `hash_value` with the identical signature in a sibling
   file (`🧬️schema/💡️inferences/🦀️component.rs`) — this one, in the `//#region 🔖️PreviewCache`
   block, had zero callers anywhere (not even in `#[cfg(test)]`). The region's only other function,
   `preview_payload_cached`, doesn't actually cache anything (calls straight through to
   `evaluated_preview_payload`) — this looks like an abandoned/never-wired memoization attempt, not
   a "one function short of working" case like `writer`'s `WireWriterIdiom` precedent in
   `📓️progress.md`. Implementing a real cache would be a scope-expanding design decision (cache
   key strategy, invalidation, etc.), not a warning fix, so left the surrounding
   `preview_payload_cached`/region structure untouched and only deleted the genuinely-dead
   `hash_value` fn plus its now-unused imports (`serde::Serialize`,
   `std::collections::hash_map::DefaultHasher`, `std::hash::{Hash, Hasher}`).

### Verification
Re-ran `cargo check -p semio-s-plugin-process --message-format=short` after all fixes: 0
warnings, 0 errors, build finished clean.

## Files touched (semio-s-plugin-process only — all other 7 crates untouched, already clean)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (cargo fix)
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs` (cargo fix)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (cargo fix)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` (cargo fix)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs` (cargo fix)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` (cargo fix)

## Notes on other 7 crates
No changes made — each verified clean via isolated `cargo check -p <name>`. One transient
`semio-framework-os-kernel` E0308/E0631/E0599 error was observed on the first attempt at
`semio-s-plugin-process-wood` (another concurrent session actively editing
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` at that moment) — resolved on retry, matches the
known "Concurrent Cargo Workspace Churn" hazard pattern from this session's memory notes. Not a
real bug, not touched.

## Out of scope / not touched
- `(lib test)` targets not checked (per ticket instructions — the cross-cutting
  `Mutation::apply`/`::diff` migration and other known in-flight migrations are out of scope).
- `semio-framework-os-kernel`, `semio-framework-plugin`, `semio-s-plugin-stdio` — all upstream
  dependencies with pre-existing warnings unrelated to this ticket's assigned crates (visible in
  every `cargo check` transcript above but not touched, not in scope).
