# W1-D — Testkit Laws

Lease: `📡️spr/🧪️testkit/🦀️component.rs` region `🔖️Laws` (+ `benches/protocol.rs:5` prose fix, done).
Added subregions `🔖️Outcome`/`🔖️Policy`/`🔖️Merge`/`🔖️Conflict`/`🔖️Channel`. All helpers are pure/generic
or take the not-yet-landed store/history/channel operation as an injected closure, so they **compile
today** against only already-landed C2/C3/C5 types (`MutationOutcome`, `MergePolicy`, `Conflict`,
`MergeReport`, `MutationDag`) — no `os_store`/`os_spr::history` new-symbol dependency.

## Signatures
1. `assert_missing_target_is_error<P, Op: Mutation<P>>(base: &P, mutation: &Op)`
2. `assert_fatal_never_applies<D>(outcome: &MutationOutcome<D>)`
3. `assert_outcome_deterministic<P, Op: Mutation<P>>(base: &P, mutation: &Op)`
4. `assert_policy_matrix(rejects: impl Fn(MergePolicy, Severity) -> bool, is_applicable: impl Fn(MergePolicy, Severity) -> bool)`
5. `assert_merge_convergence<P>(seed: u64, peer_count: usize, envelopes: &[MutationEnvelope], fold: impl Fn(&[MutationEnvelope]) -> P)`
6. `assert_modify_vs_delete<P>(policy: MergePolicy, pre_state: &P, post_state: &P, report: &MergeReport, conflicts: &[Conflict], part_present: impl Fn(&P) -> bool)`
7. `assert_chronological_determinism<P>(envelope_count: usize, seed: u64, permutation_count: usize, run: impl FnMut(&[usize]) -> (P, Vec<String>, Vec<ConflictId>))`
8. `assert_quarantine_accept_equals_laissez_faire<P>(state_after_accept: &P, state_under_laissez_faire: &P)`
9. `assert_quarantine_discard_preserves_state<P>(pre_state: &P, post_state: &P, discarded_edit_ids: &[String], relayed: &[String])`
10. `assert_ledger_matches_replay(ledger: &HashMap<String, Vec<MutationMessage>>, replayed: &HashMap<String, Vec<MutationMessage>>)`
11. `assert_conflict_spr_round_trip(conflict: &Conflict, encode: impl Fn(&Conflict) -> Vec<u8>, decode: impl Fn(&[u8]) -> Conflict)`
12. `assert_channel_frame_corpus<T>(corpus: &[T], encode: impl Fn(&T) -> Vec<u8>, decode: impl Fn(&[u8]) -> T)`
13. `assert_mutation_inverse_law<P, Op: Mutation<P>>` — unchanged signature, now additionally asserts the forward `outcome.messages()` has no Error/Fatal before checking the inverse restores `base`.

## Self-tests proving failure-on-violation (all present, all `#[should_panic]` paired with a passing twin)
missing_target_is_error(Buggy…), fatal_never_applies(non-empty diff), outcome_deterministic(Nondeterministic…),
policy_matrix(wrong closures), merge_convergence(order-dependent string-concat fold vs commutative sum fold),
modify_vs_delete(×2: wrongly-accepted Normal, part-still-present LaissezFaire), chronological_determinism(raw
vs sorted order), quarantine_accept_equals_laissez_faire(unequal states), quarantine_discard_preserves_state
(×2: relayed, state changed), ledger_matches_replay(unequal maps), conflict_spr_round_trip(lossy decode),
frame_corpus_round_trip(lossy codec), mutation_inverse_law(RejectedForwardOp carrying Fatal).

## Pending on 1-A/1-B/1-C
Helpers 6/8/9/10/11 take the store/history operation as an injected closure precisely because
`ArtifactStore::{ingest_remote→MergeReport, resolve_conflict, merge_policy, set_merge_policy}` (1-A) and
`encode_conflicts`/`decode_conflicts`/`HistoryConflict` (1-B) don't exist yet — a W3 facet wires the real
methods in once landed; nothing here needs editing when that happens. Helper 12 is ready to sweep
`AppCommand::{SetMergePolicy,ResolveConflict,ReadConflicts}` / `AppFrame::{MergeReport,Conflicts}` (1-C) the
moment those variants land — today's self-test exercises it against the real `encode_app_command`/
`decode_app_command` on existing variants only.

## Acceptance (`🧪️w1-d-cargo.txt`)
Crate does **not** currently compile — but every failing site is outside this lease. Distinct locations at
last run: 7× `E0425 reconcile_with_last` gone (🏪️store, 1-A mid-deletion), 5× `E0063 HistoryLog.conflicts`
+ 2× `E0063 HistoryOpMeta.messages` (🏪️store call sites, 1-A/1-B), 2× `E0004 ArtifactCommand::{SetMergePolicy,
ResolveConflict}` non-exhaustive match (🏪️store, 1-A mid-add), 1× `HistoryLog.conflicts` at
`📡️spr/🧪️testkit/🦀️component.rs:216` — **inside this file but region `🔖️Gen`, not my `🔖️Laws` lease**, so
left untouched per the no-foreign-edits rule. One in-lease breakage was found and fixed: the pre-existing
`channel_frame_round_trip_holds_for_command_and_frame_samples` test's `AppFrame::Error{..}` literal was
missing the new trailing `report` field 1-C landed — added `report: Vec::new()`. No error was ever reported
inside `🔖️Laws` itself across 4 successive `cargo check` runs. Real pass/fail counts for
`os_spr::testkit::tests` are unavailable until 1-A's store edits land; will re-run and update this file
once the barrier clears.
