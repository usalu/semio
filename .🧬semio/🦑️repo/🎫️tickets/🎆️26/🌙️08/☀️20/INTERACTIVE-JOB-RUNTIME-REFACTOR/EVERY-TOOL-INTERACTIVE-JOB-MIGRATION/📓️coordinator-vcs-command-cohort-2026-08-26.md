# VCS Command Cohort

## Outcome

All ten `s.vcs.vcs@1/*#editor` command rows are now admitted by explicit app-owned retained factories. The official verifier reports zero remaining VCS rows and zero VCS scan-then-monolith rows.

## Bounded Interactions

Eight constant/small-payload commands use `VcsBoundedCommandJobFactory`: `incrementCounter`, `patchSnapshot`, `setLocale`, `noMutation`, `canvasPointerDown`, `canvasPointerMove`, `canvasPointerUp`, and `canvasWheel`.

- Raw text admission is capped at 8,192 bytes.
- Semantic extent is one work item.
- Oversized action payloads, retained wire input, and checkpoint owners fail closed.
- Operation context and app-instance/document/generation identity flow into the retained payload.

## Resumable Text Editing

`textEdit` and `edit` use `VcsResumableCommandJobFactory` and a dedicated `VcsEditCommandWork` state machine.

- JSON input is capped at 8,192 bytes; current and next tag counts are capped at 4,096; output text ownership is capped at 16,384 bytes.
- Decode, reserve, scalar diff, current-tag indexing, next-tag indexing, additions, removals, and publication are separate scheduler turns.
- No terminal reducer call follows the cursor stages; the workspace itself assembles semantic mutations.
- Checkpoints carry a replay cursor and deterministically rebuild the operation-owned workspace through replay turns.
- Cancellation retirement releases mutations, both ordered tag indices, parsed tags, and the parsed snapshot through bounded close grants.
- The batch reducer was deduplicated and changed from nested `Vec::contains` scans to ordered-set membership; it remains the serde-based output oracle for the resumable implementation.

## Tests and Fixtures

- `🧪️fixtures/🎯️retained-command-limits.json`: bounded IDs and maximum/max+1 contract.
- `🧪️fixtures/🎯️retained-edit-limits.json`: resumable IDs, tag/output/work caps, and close grant.
- Rust laws compare the cursorized output with the serde JSON batch oracle, enforce maximum+1 rejection, prove multi-turn behavior, exercise incremental retirement, and measure each representative work turn below 8 ms.

## Verification State

- `rustfmt` parsing and `git diff --check`: green.
- Both language-neutral JSON fixtures decode through Bun: green.
- Official tool-job verifier: expected repository-wide exit 1, with 217 admitted rows, 719 remaining rows, 32 globals, 53 scan-then-monolith rows, and 35 import routes; VCS contributes zero rows to the four failure ledgers.
- Native VCS runtime tests remain queued behind the shared single-compiler lease. No runtime-green claim is made yet.
