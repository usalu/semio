# G2 — Law Verification (evidence, not edits)

Lease: `📡️spr/🧪️testkit/🦀️component.rs` (full file) + `🏪️store/🦀️component.rs` `🧪️Tests` region
(ADD-only). All commands actually executed; raw logs in `$T/🧪️g2-*.txt`.

## 1. Six required crates — real pass/fail counts

| # | Crate | Result | Log |
|---|---|---|---|
| 1 | `semio-framework-os-kernel --lib` (baseline) | **971 passed, 0 failed** | `🧪️g2-01-os-kernel.txt` |
| 1′ | `semio-framework-os-kernel --lib` (after adding law-wiring tests, §2 below) | **977 passed, 1 failed** | `🧪️g2-07-os-kernel-final.txt` |
| 2 | `semio-framework --lib` | **136 passed, 1 failed** | `🧪️g2-02-framework.txt` |
| 3 | `semio-framework-plugin --lib` | **214 passed, 6 failed** | `🧪️g2-03-plugin.txt` |
| 4 | `semio-framework-plugin-host` | **43 passed, 0 failed** (+0 doc-tests) | `🧪️g2-04-plugin-host.txt` |
| 5 | `semio-framework-os-kernel-db --lib` | **417 passed, 7 failed** | `🧪️g2-05-os-kernel-db.txt` |
| 6 | `semio-framework-os-run --lib` | **15 passed, 0 failed** | `🧪️g2-06-os-run.txt` |

### Failure attribution

**Crate 1′ — `testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch` — OURS, a real law violation.** See §3, "law does NOT hold" — added by this lane, kept failing on purpose (not papered over).

**Crate 2 — `workflow::tests::remove_operations_backwards_restores_cascade_deleted_dependents` — a real bug, out of my lease (🔁️workflow module), same ticket's fan-out work, not attributable to another ticket.**
`🔁️workflow/🦀️component.rs:1471-1479`, `WorkflowMutation::RemoveNode`'s `inverse()` returns `[AddNode, ConnectPorts(cascaded edges), Bind…]` (node-first). Every real caller of `.inverse()` reverses the returned list before applying it (`🏪️store/🦀️component.rs:4566-4568` `let mut back = op.inverse(&state); back.reverse();`, and the test helper `assert_operation_round_trip` at `🏪️store/🦀️component.rs:7699-7701` does the same) — so a node-first inverse list gets applied edge-first, and `ConnectPorts` panics with `mutation.apply.missing-target` because the node it needs isn't back yet. The fix (out of my lease) is to build the list cascade-dependents-first, `AddNode`/`AddParameter`/`DeclareInput` **last**, mirroring the pattern the file's own doc-comment claims but doesn't implement. `git log --date=iso` on the file shows the last commit at `2026-08-16 20:26:15 +0200` (auto-commit sweep, not attributable to a specific author/lane by message) and the docstring at line 1438 explicitly tags this code as "26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS W0" — i.e. this ticket's own still-open fan-out, not a foreign ticket.

**Crate 3 — 6 failures — NOT ours, live peer tickets `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` (open) and `FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` (open).**
All 6 failing test names (`artifact_definition_contract_tests::*`, `testkit::testkit_tests::assert_two_instances_converge_on_disjoint_edits`, `plugin_builder_contract_tests::*`) are independently named in those two open tickets' own logs (`ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w4-final-gates.txt`, `/🧪️w4-tests.txt`; `FULL-STDIO.../📋️wave-0-a-artifact-definition.md`), which is corroborating evidence these are mid-flight elsewhere, not introduced by this lane (I made zero edits to `🔌️plugin/🦀️component.rs`).

**Crate 5 — 7 failures — NOT ours in the sense of my lease, but a LIVE PEER lane inside this same ticket (`MUTATION-OUTCOMES`'s own `🛢️db`/C9 slice), currently mid-edit.**
`git status --porcelain` shows `M  🛢️db/⚔️conflict/🦀️component.rs` and `M  🛢️db/📄️artifact/🦀️component.rs` staged right now; mtimes 22:45 today (< 1h before this run). `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w4-final-gates.txt` independently corroborates: it already found `error[E0433] 'cannot find ConflictRule/MergeStrategyKind in protocol'` at these same 3 db files, attributed to "LIVE PEER (MUTATION-OUTCOMES, os-kernel scope)". The 7 failures I see (serde `Counter`/JSON `"expected value"` errors in `db_artifact`/`db_engine`/`db_facade`/`db_testkit`) are consistent with an in-progress C9 hub/db wire-format change, not something in my `testkit`/`store` lease.

## 2. Testkit-law audit — what actually executes

| Law | Test name(s) | Executed? | Where |
|---|---|---|---|
| `assert_missing_target_is_error` | `os_spr::testkit::tests::missing_target_is_error_holds_for_a_correct_impl` (+panic twin) + 32 real per-facet call sites across plugins (e.g. `mathematical/…/🧬️mutations/🦀️component.rs:281`) | **YES, holds** | crate 1 |
| `assert_fatal_never_applies` | `…fatal_never_applies_holds_for_a_correct_outcome` (+panic twin) + 33 real facet call sites | **YES, holds** | crate 1 |
| `assert_outcome_deterministic` | `…outcome_deterministic_holds_for_add` (+panic twin) + 8 real facet call sites | **YES, holds** | crate 1 |
| `assert_policy_matrix` (3×4) | `…policy_matrix_holds_for_the_real_apis` — calls the REAL `MergePolicy::rejects`/`MutationOutcome::is_applicable`, not a stub (+panic twin) | **YES, holds** — was previously only referenced in a comment (`mathematical/…/🦀️component.rs:277`) claiming no real call site existed; the real call site is the testkit self-test itself | crate 1 |
| `assert_modify_vs_delete(policy)` | **NEW**: `testkit_law_modify_vs_delete_holds_under_normal_and_vigilant`, `testkit_law_modify_vs_delete_holds_under_laissez_faire` (🏪️store/🦀️component.rs:8719,8733) — real `ArtifactStore::ingest_remote` two-peer envelopes, all 3 policies | **YES, holds** (0 real call sites before this lane; only a synthetic self-test existed) | crate 1′ |
| `assert_chronological_determinism` | **NEW**: `testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch` (🏪️store/🦀️component.rs:8745) | **NO — LAW VIOLATION, see §3** | crate 1′ |
| `assert_quarantine_accept_equals_laissez_faire` | **NEW**: `testkit_law_quarantine_accept_equals_laissez_faire_via_real_store` (🏪️store/🦀️component.rs:8761) | **YES, holds** | crate 1′ |
| `assert_quarantine_discard_preserves_state` | **NEW**: `testkit_law_quarantine_discard_preserves_state_via_real_store` (🏪️store/🦀️component.rs:8781) | **YES, holds** | crate 1′ |
| `assert_ledger_matches_replay` | **NEW**: `testkit_law_ledger_matches_replay_via_real_store` (🏪️store/🦀️component.rs:8800) | **YES, holds** | crate 1′ |
| `assert_conflict_spr_round_trip` + `HistoryOpMeta.messages` | **NEW**: `testkit_law_conflict_spr_round_trip_via_real_store` (🏪️store/🦀️component.rs:8824), plus the pre-existing `spr_round_trip_preserves_edit_messages_and_conflicts` (🏪️store/🦀️component.rs:10304) — the latter is the real `HistoryOpMeta.messages` round trip: `print_document_spr` splits `edit_messages` down into each op's own durable `HistoryOpMeta.messages`, `parse_document_spr` reassembles them (`🏪️store/🦀️component.rs:2801`, `:3031`) | **YES, holds** | crate 1 / 1′ |
| `assert_merge_convergence` | only the synthetic self-test — 0 real call sites | **not wired to real code** (not in this lane's required list; noted, not fixed — out of scope/time) | — |
| `assert_channel_frame_corpus` (C8) | only the synthetic self-test — 0 real call sites against `AppCommand::{SetMergePolicy,ResolveConflict,ReadConflicts}`/`AppFrame::{MergeReport,Conflicts}` | **not wired to real code** — lives in `📡️spr/🧵️channel`, outside this lease | — |

Before this lane: the store already had **standalone, hand-written** tests
(`modify_vs_delete_quarantines_under_normal_and_vigilant`, `chronological_determinism_any_arrival_order_converges`,
`quarantine_accept_equals_laissez_faire_result`, `quarantine_discard_preserves_state`,
`ledger_matches_a_fresh_replay_of_the_same_envelopes`) asserting the same behavior **without calling the frozen
testkit helper functions at all** (0 external call sites, confirmed by repo-wide grep). The 7 new tests above close
that gap for 5 of the 6 collaboration laws — and in doing so, found a real bug in the 6th.

## 3. Law that does NOT hold: `assert_chronological_determinism`

**Claim in the frozen contract:** "the same envelope set delivered in ANY arrival order converges to the same
snapshot, the same `applied_edit_ids` order, and the same conflict set."

**Reality:** it does not, for two causally-independent (no declared dependency — the correct shape for a genuine
concurrent modify-vs-delete) envelopes ingested via two separate `ingest_remote` calls.

- Envelopes: `delete` @ HLC(1,100), `modify` (`SetN{n:42}`) @ HLC(2,200), default `MergePolicy::Normal`.
- Arrival order `[delete, modify]`: delete applies (n→`i32::MIN`); `ingest_remote(modify)` hot-appends (k = len),
  replay raises `mutation.target-missing` (Error), Normal rejects → **quarantined, state stays `n = i32::MIN`**.
- Arrival order `[modify, delete]`: modify applies first (n→`42`, no delete on record yet — no error);
  `ingest_remote(delete)` computes `k = 0` (delete's HLC sorts before the already-applied modify — correct
  rewind target) and replays `[delete, modify]` from genesis, which **also** raises `mutation.target-missing` —
  worst = Error, Normal rejects — but the step-8 reject branch (`🏪️store/🦀️component.rs:5160-5174`) only pushes a
  `Quarantined` conflict and returns; it never touches `self.current`/`self.applied_edit_ids`, which are still
  whatever they were **before this call** — i.e. still `n = 42`, the already-committed (and, per this rebase, now
  provably invalid) modify.
- Result: `left: DemoSnapshot { n: 42 }` vs `right: DemoSnapshot { n: -2147483648 }` — different final states for
  the identical two-envelope set, panic at `📡️spr/🧪️testkit/🦀️component.rs:783` ("arrival order must not change
  the final state").

**Root cause:** `ingest_remote`'s reject path (§C6 step 8, `🏪️store/🦀️component.rs:5160-5174`) is a no-op relative
to already-committed local history. The rewind (step 3, `:5132`) correctly identifies that an already-applied
edit needs to be re-validated against a newly-arrived, chronologically-earlier envelope, and the replay correctly
detects the resulting conflict — but "reject" only refuses to admit the *new* envelope; it does not retroactively
un-commit the edit that was accepted earlier under a different (order-dependent) view of history. Whichever of
the two mutually-exclusive edits is *ingested first* wins permanently, regardless of true HLC order — this is not
a arrival-order-independent design; it is asymmetric by construction. **This is a genuine implementation gap in
the C6 algorithm, not a test-harness artifact** — the test uses the exact same fixture shape
(`mutation_envelope_at`, `Vec::new()` dependencies) as the store's own pre-existing, passing
`modify_vs_delete_quarantines_under_normal_and_vigilant` test.

This test (`testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch`,
`🏪️store/🦀️component.rs:8745`) is left **failing on purpose** — not weakened, not deleted — per the coordinator's
explicit instruction to report a real law violation rather than paper over it. Final count for crate 1′
(`semio-framework-os-kernel --lib`) is **977 passed, 1 failed**, and that one failure is this finding.

## Files touched (within lease)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — added 7 tests to the `🧪️Tests` →
  `🔖️MergePolicyTests` → new `🔖️TestkitLawWiring` subregion (lines ~8712-8838). No other edits.
- `📡️spr/🧪️testkit/🦀️component.rs` — read only, no edits needed (all required law helpers already existed with
  correct signatures from 1-D's landed work).
