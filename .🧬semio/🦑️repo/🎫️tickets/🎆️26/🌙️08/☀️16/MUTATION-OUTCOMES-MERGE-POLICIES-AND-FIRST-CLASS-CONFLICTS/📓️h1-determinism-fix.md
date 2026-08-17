# H1 — Chronological Determinism Fix (`ingest_remote`)

## Root cause (two independent bugs, both in `ingest_remote`, `🏪️store/🦀️component.rs`)

1. **Step 8 (reject) was atomic-over-the-whole-suffix and never touched already-committed
   history.** `replay_suffix` computed one `worst` over ALL of `order[k..]` (new batch *and*
   any already-applied edits a rewind pulled back in), but reject only ever discarded the
   *new* batch — an already-committed edit that the rewind proved invalid (e.g. `modify`,
   once `delete`'s earlier HLC forced a rewind) stayed permanently committed. Whichever of two
   mutually-exclusive edits was *ingested first* won, regardless of true HLC order — exactly
   the bug G2 found (`n=42` vs `n=-2147483648` depending on arrival order).
2. **`Conflict.timestamp`/`ConflictId` were hashed from `self.clock`**, which is a genuine
   ticking HLC (`HybridLogicalTimestamp::merge` bumps `logical` on *every* call, not only when
   the incoming timestamp is newer) — so its value depends on how many merges happened and in
   what order, not on the envelope set. Once bug 1 was fixed and state/`applied_edit_ids`
   converged, `ConflictId` still diverged between arrival orders because it hashed this
   arrival-order-dependent clock value.

## Fix

- New `ArtifactStore::replay_suffix_partitioned` (added next to `replay_suffix`, unused by
  `merge_remote_snapshot` — that path is untouched): walks `order[k..]` **one edit at a time**
  instead of computing one atomic accept/reject decision. Each edit's own outcome is checked
  against `policy.rejects(..)` using the *running* state; an edit that fails is left out of
  `committed_ids` (its forward ops/`MutationMeta` are never touched — same "diffs/inverses/
  messages recomputed, forwards never rewritten" contract `Undo` already relies on) and the
  next edit is evaluated against the state as if the rejected one had never applied. This makes
  an out-of-order arrival reproduce exactly what true-HLC-order arrival already produced.
- `ingest_remote` steps 6–9 rewritten around this: `applied_edit_ids` becomes
  `self.applied_edit_ids[..k] ++ committed_ids`; quarantined edits (new or retroactively
  invalidated) go into one `Quarantined` conflict; committed edits with `worst ≥ Warning` go
  into one `Degraded` conflict; `self.dag` only advances when nothing quarantined (preserves
  the pre-fix redelivery-retry behavior for a fully-rejected batch).
- `Conflict.timestamp`/the `hlc` fed to `ConflictId::new` now come from
  `max(conflicting_edits.map(|e| e.mutation_meta[0].timestamp))` — a pure function of the
  edit set — instead of `self.clock`. `self.clock` itself is untouched (still ticks normally
  for stamping future local edits).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `ArtifactStore` region only:
  added `replay_suffix_partitioned`; rewrote `ingest_remote`'s steps 6–9. No test was weakened
  or deleted; no change to `merge_remote_snapshot`, `resolve_conflict`, or `replay_suffix`
  (still used by `merge_remote_snapshot`).

## Verification (real, executed)

1. **Target test** — `testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch`:
   **PASS** (was failing). Log: `🧪️h1-target-test.txt`.
2. **Full crate** — `cargo test -p semio-framework-os-kernel --lib`:
   **978 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out** (was 977 passed / 1 failed —
   same 978 total, the one prior failure now passes, nothing else moved). Log:
   `🧪️h1-full-suite.txt`.
3. **G2's other collaboration-law tests** (`🔖️TestkitLawWiring` subregion), re-run individually:
   `testkit_law_modify_vs_delete_holds_under_normal_and_vigilant` — ok
   `testkit_law_modify_vs_delete_holds_under_laissez_faire` — ok
   `testkit_law_quarantine_accept_equals_laissez_faire_via_real_store` — ok
   `testkit_law_quarantine_discard_preserves_state_via_real_store` — ok
   `testkit_law_ledger_matches_replay_via_real_store` — ok
   (plus the pre-existing hand-written twins `modify_vs_delete_quarantines_under_normal_and_vigilant`,
   `modify_vs_delete_applies_under_laissez_faire_with_a_degraded_conflict`,
   `quarantine_accept_equals_laissez_faire_result`, `quarantine_discard_preserves_state`,
   `ledger_matches_a_fresh_replay_of_the_same_envelopes`, `chronological_determinism_any_arrival_order_converges`
   — all ok). Log: `🧪️h1-target-test.txt` (25-test batch run).
4. **`bun ./📜️script.ts verify mutation-outcome-law`**: `[verify mutation-outcome-law] passed.`
   — 0 breaches. Log: `🧪️h1-verify-mutation-outcome-law.txt`.

No contract amendment needed — §C6 steps 6–8 were under-specified for the "already-committed
edit re-enters the suffix" case (step 7's "worst over the whole replayed suffix... incl. local
edits after k" already implied per-edit re-evaluation was in scope; step 8's literal text just
didn't say what to do with an already-committed edit that fails that re-evaluation). Recommend
the contract text be tightened to say explicitly that quarantine/accept is decided *per edit*
within `order[k..]`, not atomically over the whole suffix.
