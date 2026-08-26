# Draw Production Repair Resume

## Scope

This checkpoint resumes the Draw production compile repair after the retained isolated diagnostic reported 106 Draw errors. It records only source/static work completed while the coordinator owned the shared warm Cargo target. No Git mutation, worktree, ticket closure, Cargo compile, native runtime test, or Wasm compile was performed in this resume.

## Source Repairs

- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs` now re-exports `plugin::DrawApps` at the crate root. This matches peer plugin glue and closes the five retained `crate::DrawApps` resolution errors without an adapter.
- `✏️s/🔌️plugins/🖍️draw/🗑️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️component.rs` now handles `UiText::try_from_string` as its current `Result` return type with `map_err`, closing the remaining catalogue admission mismatch.
- The current Draw tree contains zero obsolete `ArtifactEnvelope { ... }` initializers. Static reconciliation also found zero surviving sync serializer/deserializer awaits, un-awaited `Self::io()` or `interaction.selection(...)`, plugin-namespace `SurfaceKind`, `tree_item` calls fed a legacy `LabelText`, or obsolete `UiText::try_from_string(...).ok_or_else(...)` uses.

## Verifier Regression Repair

The strengthened production census began passing a production-stripped Draw editor into `toolJobDrawEnvelopeCallerRetainedExact`, while that predicate still required four `#[cfg(test)]` law names and the populated-history fixture string from the stripped test module. The raw predicate was true, but the production census emitted a false Draw `.spr`/`.ops` blocker.

The repair in `📜️script.ts` keeps every production and forbidden-pattern check on the production-stripped editor. A separate raw root-editor input is used only for the four `draw_live_*` law witnesses and populated-history fixture witness.

TDD evidence:

1. A new hostile self-test passes a production editor with all five test-only witnesses removed and supplies the raw law source separately.
2. Before the predicate repair, `bun ./📜️script.ts verify interactivity tool-jobs --self-test` exited 1 with `production-stripped Draw editor with raw cfg(test) law witnesses was falsely rejected`.
3. After the repair, the same command exited 0 with `self-tests=424 clean`.

The self-test count increased from 423 to 424. Existing hostile Draw mutations still use the default same-source path, so their coverage remains intact.

## Fresh Static Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: exit 0, `self-tests=424 clean`.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json --output .../📊️sol-draw-production-static-after-test-law-routing-2026-08-26.json`: the repository-wide command exits 1 on 166 current fleet failures, but the saved report has `selfTests: 424` and `failures.filter(failure => failure.includes("Draw"))` equal to `[]`. No Draw-specific production static blocker remains.
- `git diff --check` on `📜️script.ts` and the two focused Draw repair files exits 0.
- The strengthened Bun self-test was also run after each coherent Draw source batch and reported 423 clean before the added regression case.

## Validation Truth

The last valid isolated native diagnostic remains `🧪️sol-draw-focused-native-check-after-deasync-isolated-2026-08-26.txt`, which reported 106 errors before the current source repairs. The later isolated attempt stopped in externally corrupted target metadata before reaching Draw. This resume deliberately did not touch `target-p0-current` while the coordinator compiled Trinity.

Therefore this checkpoint does **not** claim native green, Draw runtime-law execution, or Wasm green. The next authoritative step is a serial warm-target Draw compile, followed by compiler-driven repairs, native retained-law tests, Wasm validation, and a fresh final static census.
