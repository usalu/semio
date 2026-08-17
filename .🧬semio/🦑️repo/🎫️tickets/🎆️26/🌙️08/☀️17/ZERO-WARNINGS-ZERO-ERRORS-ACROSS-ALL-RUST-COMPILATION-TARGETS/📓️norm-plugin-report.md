# `semio-s-plugin-norm` — Warning Triage Report

## Scope
Crate: `semio-s-plugin-norm` (package name), `(lib)` target only, per the parent ticket's
instruction. `(lib test)` target is out of scope (pre-existing cross-cutting `Mutation::apply`/
`::diff` trait migration from another session — untouched, see verification below).

## Result
- **Starting warnings**: 32 (`(lib)` target, `cargo check -p semio-s-plugin-norm`)
- **Ending warnings**: 0
- **New errors introduced**: 0 (confirmed via `cargo check -p semio-s-plugin-norm`, exit code 0,
  no warning/error lines emitted for this crate)

All 32 warnings fell into exactly 3 repeated, mechanical patterns across the per-standard `io`
modules (`iso16757`, `vdi3805`, `din4108`, `din16798`, `en1990`–`en1999`, `din18599` — 15
standard families total) plus 2 top-level component files. Nothing was dead code; nothing was
deleted. No `#[allow(...)]` used.

### 1. Unused `ArtifactAnalyzer` trait import (15 occurrences, 1 per standard family)
Each `🚪️io/🦀️component.rs` had `use semio_framework_plugin::ArtifactAnalyzer as _;` inside
`pub mod derived_composition`. Verified via the `derive_artifact_facets!` macro definition
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:18665`) that the macro-generated
`$analyzer` type (e.g. `Iso16757Analyzer`) gets **both** a trait impl (`impl ArtifactAnalyzer for
$analyzer`) **and** a separate inherent `impl $analyzer { pub fn analyze(...) }`. The call site
(`Iso16757Analyzer::analyze(&[native])`) resolves through the inherent impl, so the trait import
was genuinely unused, not a case of test-only or cfg-gated usage. Deleted the import line.

### 2. Missing elided lifetime on `ComposeSource` (15 occurrences, 1 per standard family)
Same 15 files: `fn compose(sources: &[ComposeSource]) -> ...` inside the `impl ArtifactComposition
for <Standard>ComposerComposition` block. `ComposeSource<'a>` (defined in
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs:861`) carries a lifetime parameter, and the trait
this implements (`ArtifactComposition::compose`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:872`)
already declares the parameter explicitly as `&[ComposeSource<'_>]`. Changed each impl to match:
`&[ComposeSource<'_>]`. Purely a lint (`elided_lifetimes_in_paths`), no behavior change — confirmed
against the trait signature it implements.

### 3. Doc comment on a macro invocation, not an item (2 occurrences)
`📘️en1990/🦀️component.rs` (was line 67) and `📙️din18599/🦀️component.rs` (was line 95): a long
`///` doc block sat directly above a bare `thread_local! { static ...; }` macro invocation. Rustc
attaches outer doc comments to the following item, but a macro invocation statement isn't a doc-
comment-bearing item, so the comment was flagged unused. Fix: moved the doc comment inside the
`thread_local!` block, directly above the `static` declaration it documents — `thread_local!`
does support per-static doc attributes, so this is the idiomatic placement, not a suppression.
Content of both doc comments preserved verbatim.

## Files touched (17 total)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🦀️component.rs`

Each of the 15 `io/component.rs` files got both fix #1 and fix #2 (2 diffs each, 30 total). The
2 top-level files each got fix #3 (1 diff each).

## Left alone / out of scope
- `(lib test)` target: 37 errors (`E0433` x7, `E0609` x30) as of this pass — up from the ~30 the
  parent ticket's progress notes recorded earlier, consistent with the other session's in-flight
  `Mutation::apply`/`::diff` → `MutationApplyResult<T>` migration continuing to move. Verified via
  `git diff` that none of these errors land in any file this pass touched (the errors are outside
  the `io` modules and the two `thread_local!` files). Not attempted, per parent ticket
  instructions — this is squarely the other session's live migration.
- No `dead_code` warnings existed in this crate's `(lib)` target at all — every one of the 32
  warnings was import/lint-shaped, not dead-code triage. So none of the dead-code hazards from the
  parent ticket's brief (test-only helpers, stale `#[cfg(feature)]` gates, ambiguous norm
  scaffolding) applied here; nothing was deleted, nothing needed `#[cfg(test)]` gating.
- No `#[allow(...)]` used anywhere.

## Verification
- `cargo check -p semio-s-plugin-norm --message-format=short`: `semio-s-plugin-norm` (lib) — no
  warning/error lines emitted (down from `generated 32 warnings`), exit code 0.
- `cargo check -p semio-s-plugin-norm --all-targets --message-format=short`: confirms the `(lib
  test)` failures are unrelated to and unaffected by this pass's edits (grepped the error output
  against the touched files — zero matches).
