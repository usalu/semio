# 📋️ Executor Brief (read fully before touching code)

T = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES`

1. Read `T/📋️design.md` (laws L1–L5, violation codes, Rulings, the fixed Framework API, build rules) and your scope's
   audit report `T/🔍️audit-*.md`. Audit rows are heuristic candidates — confirm each by reading the code.
2. Read `/Users/ueli/Documents/semio/AGENTS.md`. Highlights: concise code, no comments inside definitions, docstrings start
   with a unique emoji, no legacy/compat layers/deprecations/adapters, schema-first, handcraft fixtures, test-driven.
3. Concurrency: other agents (and humans) edit the same files. Never run modifying git commands (`commit`, `stash`,
   `checkout`, `restore`, `reset`), never use worktrees. Re-read a file right before editing it; keep edits compile-atomic
   (a file is never left half-converted). Never open/close/reopen tickets. Never delete `T/🗑️generated` or other agents'
   files. Scratch files go ONLY under `T/🗑️generated/<your-label>/`.
4. Per mutation kind, the target state:
   - `🔺️diff/🦀️.rs`: builds a sparse typed diff declaratively from payload + reads of `base` (no base clone written into,
     no whole list/record/snapshot unless the kind semantically replaces exactly that entity, no `between(`, no `.apply(`).
   - `↩️inverse/🦀️.rs`: builds concrete mutations from payload + reads of `base` directly — never by calling the forward
     diff or walking its output, never a whole-snapshot/collection restore, never `Vec::new()` when the kind changes state.
     Inverse rows restore exact base values (absolute setters) so the inverse diffs sum exactly to the negative diff.
   - The diff type: per-id/index keyed collections (added / removed / modified rows), `absorb` coalesces same-key entries
     (patch∘patch → one patch, create∘delete → nothing, delete∘create → replace), `DiffAlgebra::inverse` implemented
     concretely, `apply(&self, base, capability: protocol::ApplyCapability)`.
   - Shared generic seams named in the Rulings are DELETED, not wrapped. Each kind owns its concrete code in its leaf
     directory; small shared pure geometry/value helpers (e.g. a rotation matrix) are fine.
   - Hand-written `impl Mutation<P>` with whole-state diffs (config/window/presence/transient) become sparse per-field
     diffs with concrete inverse mutations (set-field back to the base value).
   - Every caller that applied a diff itself uses `protocol::apply_diff(diff, &base)`; `MutationOutcome::apply_to` is gone.
5. Tests: every leaf's `🧪️tests` calls `assert_mutation_inverse_sum_law(&mutation, &before)` (framework law helper; rg for
   it — it is being added concurrently by the framework executor; if it does not exist yet, write the call anyway and
   report it as pending). Update the committed `🧫️fixtures/**/🔺️diff/🔣️.json` (and `➡️after`) by hand for every kind whose
   diff shape changes; update diff JSON schemas (`🔣️.json`) schema-first; update the independent oracle script named in the
   test docstring if it exists (Python, in its ticket folder) so it reproduces the new diff.
6. Build rules: every cargo call through `"$T/🚦️gate.sh" <your-label> -- cargo ...`, foreground only, never
   `run_in_background`/`Monitor`, never `CARGO_TARGET_DIR`; plugins: `cargo check -p <crate> --target wasm32-wasip2
   --message-format=short`, then `cargo test -p <crate> <filter>` for the touched leaves when the check is green. Errors in
   crates you do not own (host, peers) are noted, not fixed. "WRITTEN BUT UNVERIFIED" with a precise list is acceptable.
   Find the crate name in the nearest `📦️packages/🦀️rust/Cargo.toml`.
7. Report: write `T/📓️exec-<your-label>.md` — per kind: before → after classification, files touched, test status
   (run / not run, with the exact command), open issues. Final message: ≤ 12 lines + report path.
