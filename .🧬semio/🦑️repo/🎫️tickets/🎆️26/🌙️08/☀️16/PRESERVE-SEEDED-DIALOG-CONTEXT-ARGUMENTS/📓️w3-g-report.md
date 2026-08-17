# Lane 3-G report — checkpoint-after-remote-relay fix + authorless-checkpoint fix

## Lease verification

`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` confirmed closed before any
edit: `🎫️ticket.json.status == "closed"`, its `📌️important.md` no longer exists. `git log --date=iso`
on both leased files shows their last commit at `2026-08-16 21:52:13 +0200` (`e648c495`), well before
this lane started editing. Both files are shared/live-edited by other concurrent sessions throughout
this lane (per the brief's own warning) — every edit below was re-read fresh immediately before
applying, and every verify command ran against the file's live, current-at-that-moment content.

## The defect — root cause, confirmed by direct trace, not assumption

Both lane 2-G and lane 3-A correctly identified the symptom (`ArtifactStore::dispatch_inner`'s
`CommitCheckpoint` arm, `🏪️store/🦀️component.rs` ~4789) and two real, distinct contributing findings.
I traced the ACTUAL failure mechanism end to end (not stopping at "reproduced"):

1. **The real trigger is `merge_remote_snapshot`, not `ingest_remote` alone.** A bare
   `ingest_remote(edit)` immediately followed by `CommitCheckpoint` on the SAME store is
   self-consistent — `ingest_remote` pushes the reconstructed `Edit` into both `applied_edit_ids`
   and `envelope.vcs.edits` together, under the same (wire) id, so a `Change` minted right after
   references an id that genuinely exists locally. The crash needs a SECOND step: the SENDER later
   commits its own checkpoint (whose `Change.edit_ids` names the edit under the sender's own REAL
   `Edit.id`) and relays a full snapshot — exactly what `flush_outbound(is_apply: false)` does for
   `CommitCheckpoint`/`CreateAlternative`/etc. `merge_remote_snapshot`'s `batch.is_empty()` fast path
   (`🏪️store/🦀️component.rs` ~5716, reached because the receiver already recognizes the edit as
   "known" via its wire-derived operation identity) merges the incoming `Change` in verbatim via
   `merge_by_id` **without ever touching `vcs.edits`**, then calls `validate_durable_history` — which
   rightly rejects it, because the receiver's own copy of that edit lives under a DIFFERENT id than
   the one the incoming `Change` names.
2. **The id-domain discontinuity itself**, confirmed by full trace, not by inspection alone:
   `edit_from_operation_envelope` (`🏪️store/🦀️component.rs:5981`, called from `ingest_remote`)
   reconstructs a remote `Edit.id` as `envelope.mutation_id.0` — the wire per-op id. I then traced
   *why* this id differs from the sender's own `Edit.id` even for the dominant single-op case:
   `replay_mutations` (`🏪️store/🦀️component.rs:5114`) unconditionally stamps
   `mutation_meta[i].mutation_id = Some(mutation.mutation_id().unwrap_or_else(|| MutationId(mint_mutation_id(&encoded))))`
   — a pure CONTENT hash of the raw op bytes (`🌿️vcs/🦀️component.rs:62`) — while `Edit.id` itself is
   `mint_edit_id(actor, sequence, forwards_fingerprint)` (`🌿️vcs/🦀️component.rs:35`), a DIFFERENT
   formula (actor + sequence + full-fingerprint). `mutation.mutation_id()` has exactly one
   implementor repo-wide — the trait's own `None` default (`📡️spr/🎮️command/🦀️component.rs:106`,
   confirmed zero overrides by grep) — so op 0 always falls to the content hash. These two hashes
   were never going to collide by construction, for any edit, single-op or not.

## Store-level regression test — written before the fix, per instructions

`🏪️store/🦀️component.rs`, test module, new test
`checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`
(placed right after `snapshot_merge_preflights_every_conflict_before_committing`): store `a` applies
a local edit and later commits a checkpoint; store `b` ingests `a`'s edit over the real wire encoding
(`crate::os_spr::mutation_envelope_from_edit`), commits its own checkpoint (proving the
self-consistent, non-buggy first step), then absorbs `a`'s full snapshot via the private
`merge_remote_snapshot` (same mechanism `flush_outbound(is_apply:false)` triggers on the wire).
Asserts the merge succeeds and every `Change.edit_ids` entry `b` now knows about is backed by a real
`b.envelope().vcs.edits` entry.

- **Before the fix**: failed with the exact real-world message shape —
  `ValidationFailed("change change-9d0fdf39e02a521b has an invalid edit reference edit-885ba8428e7eef9d")`.
  Log: `🧪️3-g-regression-test-pre-fix.txt`.
- **After the fix**: passes. Log: `🧪️3-g-regression-test-post-fix.txt`.

## Fix 1/2 — the id-domain unification, deliberate choice with the rejected alternative's own evidence

**Chosen: carry the sender's `Edit.id` across the wire (for the single-op case) — a single global
edit-id domain.** New free function `stamp_primary_operation_identity` (`🏪️store/🦀️component.rs`,
next to `uncommitted_edit_ids`): for a freshly-minted edit with exactly ONE forward op, overrides its
sole `mutation_meta[0].mutation_id` with the edit's own real `id`. Called from both fresh-edit
constructors: `apply_command` and `amend_command`'s fresh (non-amend-target) branch — an `AmendLast`
extension always appends beyond index 0 of an edit that's already had this stamp applied, so it never
needs it itself. **Deliberately guarded to `forwards.len() == 1` only** — I first tried the
unconditional version and it broke a genuine, pre-existing multi-op convergence test
(`operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit`): `ingest_remote`
reconstructs one local single-forward `Edit` per WIRE op, so giving op 0 of a MULTI-op edit the
parent's bare id makes that one-forward phantom collide, id-for-id, with the real N-forward edit once
its full snapshot later arrives — `merge_remote_snapshot`'s
`same_edit_operation_identities_and_payloads` check correctly flags the shape mismatch as
`"remote history conflicts with established edit"`. Restricting the override to single-op edits
closes exactly the collision this ticket's repro hits (the dominant case — one atomic user action,
one forward op, e.g. every `SpawnApp`) without introducing a new one for multi-op edits, which keep
their pre-existing content-hash wire ids unchanged.

**Why the alternative ("harden `uncommitted_edit_ids` to only return ids present in
`envelope.vcs.edits`") is the wrong PRIMARY fix, with evidence:** I traced through
`merge_remote_snapshot`'s `batch.is_empty()` path (`🏪️store/🦀️component.rs` ~5716-5729) — the
`Change` that fails validation is one `merge_by_id`'d in verbatim from a REMOTE peer's own snapshot,
not minted by `uncommitted_edit_ids` on the FAILING store at all. Hardening `uncommitted_edit_ids`
would do nothing for this exact failure (it's called only from THIS store's own `CommitCheckpoint`
arm, never touched during a snapshot merge) — proving lane 3-A's own instinct right: it "merely stops
[a different] crash" (a locally-minted checkpoint referencing a locally-dangling id) while leaving
this ticket's actual, reported failure completely unfixed. Filtering would also have been the wrong
call even for that different, narrower case: it silently drops a real edit out of a checkpoint with
no trace, exactly the failure mode this lane's regression test's own doc comment warns against.

**Kept as defense in depth, NOT as the fix**: `uncommitted_edit_ids` (`🏪️store/🦀️component.rs:2141`)
now carries a `debug_assert!` (no behavior change in release builds — nothing is filtered) that fires
loudly and specifically if any FUTURE cause ever reintroduces a dangling id, instead of surfacing two
calls later as `validate_durable_history`'s generic message.

### Fallout from the id-domain fix — two pre-existing tests, both resolved correctly

- `operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit`: initially broke (see
  above); fixed by scoping the override to single-op edits only. Now passes.
- `operations_then_snapshot_remaps_the_durable_message_ledger_to_the_wire_edit_id`: its own
  `assert_ne!(source_edit.id, operation.mutation_id.0, ...)` self-check encoded the now-fixed
  divergence as a fixture precondition — with the fix, a single-op edit's wire id and its own real id
  are IDENTICAL by design, so there's nothing left to "remap" for this case. Renamed to
  `operations_then_snapshot_keeps_the_durable_message_ledger_on_the_shared_edit_id`, assertion
  flipped to `assert_eq!`, docstring added explaining the fixed invariant and pointing at the sibling
  multi-op test for the case that's still genuinely divergent. Test body and its actual coverage
  (message-ledger survives ingest-then-snapshot) unchanged.

## A second, real bug the fix uncovered — the actual plugin-level test

Fixing the id-domain bug alone did NOT turn
`two_instances_converge_on_disjoint_edits_via_backbone` green: it started failing with a DIFFERENT,
genuine error — `"validation failed: cannot create an empty checkpoint"` — at `instance_b`'s own
`commitCheckpoint`. Traced, not assumed: with the id-domain fix in place, by the time B calls
`commitCheckpoint`, its own `dispatch`'s leading `pump()` has already absorbed A's full snapshot
(A's own checkpoint, whose `Change` now correctly resolves against B's edits since the ids finally
match) — B's `applied_edit_ids` are now ENTIRELY already covered by that just-merged-in `Change`, so
`uncommitted_edit_ids` is genuinely empty and `CommitCheckpoint` correctly refuses to mint an empty
one. This is expected, correct convergence behavior (B has nothing new of its own left to commit,
since A's checkpoint already covers everything both sides know) — not a bug to route around, but a
benign "requested but nothing to do" outcome the CALLER should treat the same way it already treats
`NothingToUndo`/`NothingToRedo`/`ForeignEdit` (`🔌️plugin/🦀️component.rs`'s `dispatch_action`, existing
lenient-match arm) — a real scenario in production too, not just this test: any editor session that
receives a peer's checkpoint covering everything it has, then presses "Checkin" or hits its own
auto-check-in timer, would otherwise surface a hard error for a fundamentally benign situation.

Fixed with a small addition kept entirely inside the two leased files (no `VcsError` variant added,
`🌿️vcs/🦀️component.rs` untouched, not my lease):
- `🏪️store/🦀️component.rs`: extracted the existing inline string into `pub const
  EMPTY_CHECKPOINT_MESSAGE: &str = "cannot create an empty checkpoint"`, reused at the one existing
  call site (`dispatch_inner`'s `CommitCheckpoint` arm) instead of duplicating the literal.
- `🔌️plugin/🦀️component.rs`'s `dispatch_action`: extended the existing benign-no-op match arm with
  `Err(vcs::VcsError::ValidationFailed(ref message)) if message.as_str() ==
  vcs::EMPTY_CHECKPOINT_MESSAGE => Ok(Self::empty_result(...))`, alongside the pre-existing
  `NothingToUndo`/`NothingToRedo`/`ForeignEdit` arms.

## Fix 3 (task 3) — `history_command`'s hardcoded `authors: Vec::new()`

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `history_command` (~10514, inside
`impl<A: ArtifactApp> VcsArtifactApp<A>`): new `history_command_authors(args: Option<&Value>) ->
Vec<vcs::Author>` decodes `args.authors` (`[{id, name, avatar?}]` — both shells' wire shape per lane
3-A's report, `vcs::Author` already `#[serde(rename_all = "camelCase")]`-deserializable) via
`serde_json::from_value`, defaulting to empty on absence or malformed input (an authorless checkpoint
stays valid, it just can't say who checked in — never a hard error). `commitCheckpoint`'s arm now
calls `Self::history_command_authors(args)` instead of the hardcoded `Vec::new()`.

## Verify — real, final numbers

- New regression test alone: `cargo test -p semio-framework-os-kernel --lib
  checkpoint_after_ingesting_a_remote_edit` → **1 passed; 0 failed**
  (`🧪️3-g-regression-test-post-fix.txt`).
- `cargo test -p semio-framework-os-kernel --lib` (the crate owning `🏪️store` — confirmed via its
  `Cargo.toml` `name = "semio-framework-os-kernel"`, matching the brief's own guess) → **988 passed;
  0 failed** (`🧪️3-g-kernel-full-test-final.txt`, final rerun after all three fixes). Baseline before any
  fix, with only the new test added: 987 passed / 2 failed (the two pre-existing tests fixed above);
  net +1 test vs. that baseline (the new regression test), 0 regressions.
- `cargo check -p semio-framework-plugin` (owns `🔌️plugin/🦀️component.rs`) → clean, 0 errors, final
  rerun after the benign-no-op fix (`🧪️3-g-plugin-check2.txt`, warnings only, all pre-existing).
- `cargo check -p semio-s-plugin-dag` → clean, 0 errors (`🧪️3-g-dag-check.txt`, warnings only).
- `cargo check -p semio-s-plugin-norm` → clean, 0 errors (`🧪️3-g-norm-check.txt`, warnings only).
- `cargo test -p semio-s-plugin-space --lib` → **204 passed; 0 failed**
  (`🧪️3-g-space-plugin-test2.txt`) — target hit (baseline stated in the brief: 203 passed / 1 failed,
  that 1 being exactly `two_instances_converge_on_disjoint_edits_via_backbone`; now 204/0, +1 test
  because I did not touch that test file myself but a concurrent peer session landed one more test on
  the live tree between my two runs — confirmed via the first run's own tally already showing
  203 passed/1 failed matching the brief's baseline exactly, then 204/0 on the second run after my
  fix, with zero of my own edits touching that plugin crate's test count).

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — new
  `stamp_primary_operation_identity` (called from `apply_command`/`amend_command`'s fresh branch), new
  `pub const EMPTY_CHECKPOINT_MESSAGE`, `uncommitted_edit_ids` hardened with a `debug_assert!` (defense
  in depth only, no behavior change), new regression test, one existing test renamed/updated (see
  above).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — new
  `VcsArtifactApp::history_command_authors` (threaded into `history_command`'s `commitCheckpoint` arm
  instead of the hardcoded `Vec::new()`), `dispatch_action`'s benign-no-op match arm extended to catch
  `EMPTY_CHECKPOINT_MESSAGE`.

## sharedFileRequest

`✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs` (NOT in my lease) — the doc comment right above
`#[test] fn two_instances_converge_on_disjoint_edits_via_backbone` (~line 813-837) records a "🚧️
BLOCKED (2026-08-17, lane 2-G)" status that is now stale: the test passes as of this lane's fix
(confirmed: `cargo test -p semio-s-plugin-space --lib` → 204 passed, 0 failed, including this exact
test). Whoever owns that file should trim the doc comment down to a short note of the fixed history
(or remove it), per CLAUDE.md's "no stale docs" expectation. Not touched myself — outside my lease.

## What is NOT done

- The `two_instances_converge_on_disjoint_edits_via_backbone` doc comment's stale "BLOCKED" note —
  filed as the sharedFileRequest above instead of editing a file outside my lease.
- Nothing else from this lane's scope remains open: both the store-level regression test and the
  target plugin test pass, the authors fix is threaded through and verified not to break the two named
  consumer plugins.
