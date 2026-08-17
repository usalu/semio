# J1 — Security/Robustness Audit Findings, Closed

Lease: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (regions `🔖️ArtifactStore` +
`🧪️Tests` only) and `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⚔️conflict/**`. No file outside the
lease was touched — `📡️spr/⚔️conflict/🦀️component.rs` needed no change (its content was already
correct); everything landed in `🏪️store/🦀️component.rs`.

## HIGH-1 — silent skip on a ghost edit id: REAL, fixed

Confirmed at `replay_suffix_partitioned` (bare `let Some(edit) = edits.get(edit_id) else { continue
};`). Also found the **identical** defect in the sibling `replay_suffix` (used by
`merge_remote_snapshot`) — same file/region, same corruption risk, so fixed both rather than leaving
one half-hardened.

Fix: both now do `edits.get(edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?`.
`VcsError::UnknownEdit` already exists and is already this crate's convention for "id referenced but
missing" structural failures (`materialize_document_snapshot`, `validate_history_lanes`, etc.) — no
new `VcsError` variant needed, so nothing outside the lease (`🌿️vcs/🦀️component.rs`, which owns the
`VcsError` enum) had to be touched.

Tests (fail without the fix — verified by reverting to `continue` and re-running, see
`🧪️j1-verify-high1-fails-without-fix.txt`): `replay_suffix_partitioned_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it`,
`replay_suffix_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it`.

## HIGH-2 — `ConflictId` minted from an empty mutation-id list: REAL as a structural-invariant gap, not independently exploitable today

Traced both filter_maps (quarantine mint, degraded mint). Given HIGH-1's fix, `quarantined_ids`/
`committed_ids` coming out of `replay_suffix_partitioned` are *already* guaranteed (by construction)
to only contain ids that were found in `edits_by_id` — so as the code stood, the `filter_map` could
never actually produce an empty edit list while its id-list guard was non-empty. HIGH-2 is therefore
coupled to HIGH-1, not an independently triggerable bug via the public API today.

Still hardened it per the instruction, as defense-in-depth against a future refactor breaking that
invariant: added `ArtifactStore::edits_for_ids` (strict `HashMap` lookup, `VcsError::UnknownEdit` on
a miss) and replaced both filter_maps (`ingest_remote`'s quarantine mint and degraded mint) with it.

Test (isolates the helper directly): `edits_for_ids_errors_loudly_on_a_ghost_edit_id_instead_of_silently_filtering_it`.

## MEDIUM-3 — unbounded conflict growth (DoS): REAL, fixed

Confirmed: `ingest_remote`'s dag never advances while anything in a batch is quarantined, so the SAME
rejected envelope is eligible for redelivery forever — each redelivery pushes another `Quarantined`
conflict. Added two named consts on `ArtifactStore` (`OPEN_CONFLICT_CAP = 256`, `RESOLVED_CONFLICT_CAP
= 512`) with different failure modes per the instruction:

- **Resolved** (`Accepted`/`Discarded`) conflicts are prunable — `prune_resolved_conflicts` evicts
  oldest-first (first non-`Open` entry scanning from the front) once the total exceeds
  `RESOLVED_CONFLICT_CAP`. Called after every conflict push and after every `resolve_conflict` status
  flip.
- **Open** conflicts are never silently dropped — `ensure_open_conflict_capacity` is checked in every
  call site that might mint one (`ingest_remote`'s quarantine+degraded mints, `merge_remote_snapshot`'s
  two branches), *before* any store-field mutation for that call, so a refusal
  (`VcsError::ValidationFailed("open conflict backlog is at capacity...")`) is fully atomic — same
  "nothing applied" guarantee a policy rejection already gives. This required hoisting
  `ingest_remote`'s `degraded_ids` computation earlier (it only needs `committed_ids`/`replayed`,
  both already available pre-mutation) so both potential mints for one call can be capacity-checked
  together before either touches `self`.
- `edit_messages` ledger reviewed and deliberately **not** capped: unlike `conflicts`, it only grows
  once per edit that reaches `committed_ids` (a quarantined id's entry is explicitly cleared, never
  written), so its size tracks genuine applied-history growth, not redelivery-of-rejected-content
  amplification — capping it would mean silently losing a still-applied edit's own diagnostic record.
  Reasoning is in the `OPEN_CONFLICT_CAP`/`RESOLVED_CONFLICT_CAP` docstring.

Tests (fail without the fix, verified by reverting — see `🧪️j1-verify-medium3-open-cap-fails-without-fix.txt`,
`🧪️j1-verify-medium3-prune-fails-without-fix.txt`): `ingest_remote_refuses_a_new_open_conflict_once_the_backlog_is_at_capacity`
(proves atomicity too), `resolved_conflicts_are_pruned_oldest_first_once_the_ledger_exceeds_its_cap_while_open_ones_survive`.

## MEDIUM-4 — message-lifecycle ordering: verified correct, proved with a real mixed-batch test

Traced `ingest_remote`: quarantined ids are computed (both newly-arrived and retroactively-invalidated
kinds share one list, mutually exclusive with `committed_ids`), the quarantine block clears their
ledger entries via `replace_edit_messages(id, Vec::new())`, and only THEN does the commit step iterate
`replayed` filtered to `committed_ids` — quarantined ids are structurally excluded from that second
loop, so there is no double-write/clobber path. Analysis said this was already correct; added
`quarantine_message_clearing_is_correct_for_a_mixed_new_and_retroactive_batch`, a real
`ingest_remote` test (not a synthetic unit test) that, in ONE batch/call:

- retroactively invalidates a previously-committed edit that carried a REAL non-empty
  `mutation.cascade` ledger entry from its first commit, and
- newly quarantines a brand-new edit that never committed at all,

using the dag's dependency-buffering (`MutationEnvelope.dependencies`) to force both into the same
`drain_applied_envelopes()` batch. Verified by commenting out the clear loop and re-running: the test
fails (`🧪️j1-verify-medium4-fails-without-clear.txt`) — the stale non-empty ledger entry survives —
confirming the test is non-vacuous and the ordering genuinely matters.

Added a `BumpN { delta }` variant to the `DemoMutation` test fixture (state-dependent, always emits an
Info `mutation.cascade`) since neither existing fixture (`DemoMutation`'s `SetN`/`DeleteN`,
`SeverityMutation`'s fixed-severity ops) could produce a state-dependent non-empty message needed for
this test.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `🔖️ArtifactStore` region:
  `replay_suffix`, `replay_suffix_partitioned` (HIGH-1); new `edits_for_ids`,
  `ensure_open_conflict_capacity`, `prune_resolved_conflicts`, `OPEN_CONFLICT_CAP`,
  `RESOLVED_CONFLICT_CAP` (HIGH-2/MEDIUM-3); `ingest_remote`, `merge_remote_snapshot`,
  `resolve_conflict` (MEDIUM-3 call sites). `🧪️Tests` region: `DemoMutation` gained `BumpN`; new
  `🔖️RobustnessTests` subregion (6 tests, all verified to fail without their fix).
- `📡️spr/⚔️conflict/🦀️component.rs` — read only, no change needed.
- No file outside the lease was touched. No existing test was weakened, deleted, or had its
  assertions loosened.

## Verification (real, executed)

1. `cargo test -p semio-framework-os-kernel --lib`: **987 passed; 0 failed; 0 ignored; 0 measured; 0
   filtered out** (was 981/0 before this lane's 6 new tests — 981+6=987, nothing else moved). Log:
   `🧪️j1-full-suite.txt`.
2. `🔖️TestkitLawWiring` subregion (`testkit_law_*`, the determinism/collaboration-law tests): **7
   passed; 0 failed**. Log: `🧪️j1-testkit-law-wiring.txt`.
3. `bun ./📜️script.ts verify mutation-outcome-law`: `[verify mutation-outcome-law] passed.` — 0
   breaches. Log: `🧪️j1-verify-mutation-outcome-law.txt`.
4. Every new test individually confirmed to FAIL without its fix, by temporarily reverting the fix,
   re-running just that test, observing the failure, then restoring (never left reverted; confirmed
   `git diff` shows only the intended final state). Logs: `🧪️j1-verify-high1-fails-without-fix.txt`,
   `🧪️j1-verify-medium3-open-cap-fails-without-fix.txt`,
   `🧪️j1-verify-medium3-prune-fails-without-fix.txt`, `🧪️j1-verify-medium4-fails-without-clear.txt`.
