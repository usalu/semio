# 🔗️ Check Binding (wave 2)

No source touched; nothing compiled or run. Verified by reading.

## Finding
The task is already implemented by the check-highlight work (see `check-highlight.md`); the audit's literal matching was the false negative.
- Shared mechanism: `Owners::file` (compliance `✅️check-results`) rebinds `subject.entity_id` to an owning row (subject row, else row id embedded in the check id, else the row that references the subject row). Applied in `evaluate_units` and `EvaluationSession::merge`; scene side `projection_kit::highlight_owners` colours the owner (all 10 `fragment()` wired: en1990..en1999).
- en1998 `.elevationRegularity` (`en1998.1.{bid}...`) and `.analysis` (`en1998.2.{bridge}.analysis...`) embed the building/bridge id, so they bind to that aggregate entity through the embedded-id rule; no per-artifact fix needed.
- Law test: `✏️editor/🧮️incremental/🧪️tests/🧭️ownership/🦀️.rs` exists and is mounted (`mod ownership;`) in all ten artifacts. It files each demo document's report, draws `live_scene_of`, and asserts `ownership_audit(...).is_clean()` (every check resolves to a scene entity or design case, or is a documented global; a failing check lights its entity).
- `OWNERS` const present in the units of all ten artifacts (en1991 `AS_EMITTED`).

## Residual risks (unchanged)
- Documented global prefixes (e.g. en1990 `input.`, `annex-b.`; en1997 `governing.`; en1998 `site.`) are exempt and show only in the `global-checks` label, not on an entity. No aggregate-entity fallback was added: it needs a compiled run to pick the right aggregate per code.
- Nested subjects neither top-level nor embedded in the check id (en1994/en1995), and en1993 `loadCases` have no entity; these will surface as law failures with check ids on first run.
- Python cross-reference: check-outcome fixtures hold inputs only (no emitted check ids) and check ids are composed at runtime, so a static fixture-vs-scene cross-reference is inconclusive; the Rust law test is the authority.
