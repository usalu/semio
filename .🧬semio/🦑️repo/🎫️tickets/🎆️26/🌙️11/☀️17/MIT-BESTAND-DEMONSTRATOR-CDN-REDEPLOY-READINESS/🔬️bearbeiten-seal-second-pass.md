# Bearbeiten `seal canonical second pass length changed`

## Root cause
`ArtifactCanonicalEditSealCursor::advance` (seal/🦀️.rs) hashes the canonical bytes page-wise in the second (Hash) pass: the tree writes into `page`, `hash_pending` flushes it. The final check `hashed != length` ran when the tree reported terminal, but BEFORE the residual page tail (fewer than SEAL_FLUSH_MARGIN_BYTES = 512 bytes, never flagged `hash_pending`) was hashed. Whenever the edit is not fully buffered in the first page (canonical length above about 512 bytes, i.e. any real contribution edit with 10 observed units) the tail was unhashed, so `hashed < length` and the seal refused with "second pass length changed". Not nondeterminism and not a resumption bug: the two passes are byte-identical; the check compared against an incomplete counter. Small fixtures (fully buffered path) never reach this check, so they passed.

## State of the source (concurrent edits)
Another agent had already moved the flush after the loop (post-loop: `phase!=Count && page_length!=0 && tree terminal` sets `hash_pending`, and the in-loop check was removed). I added the remaining hole: the turn-start branch (`tree.terminal_is_empty()` seen at the start of a turn) still ran the `hashed != length` check and dropped the tree without flushing a residual page. It now sets `hash_pending` first and finishes the tree on the following turn.

## Files changed
- `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs` (turn-start terminal branch flushes the residual page before the length check). A foreign `[DEBUG] phase ... window` eprintln (line ~131) from another agent is still in that file; I left it.

## Tests
Covering law (already in tree, exact-vs-paged grant, payload sizes 0..3000, independent framed SHA256 oracle): `paged_seal_digest::paged_seal_digests_equal_the_one_event_seal_and_the_independent_framed_oracle`, `a_wide_work_turn_seals_the_same_digest_as_a_one_step_turn`, and in the edit crate `canonical::seal::tests::canonical_native_edit_seal_settles_every_exact_quote_within_its_own_grant_for_large_edits`.
NOT VERIFIED: all of these currently fail before reaching the seal logic in the shared test harness (`semio_framework_value_clone_source_testing::drive`, line 214: heap observed (392,0) or (666,98) vs reported progress (0,0)): the first turn's tree/alias allocation is not reported in the turn's `retained_capacity_bytes`. This is an allocation-accounting regression outside the seal law (CountStart/first tree turn), reproducible with count=1, and is under concurrent edit by someone else. The seal fix is therefore reasoned, not test-confirmed.

## Rebuild
Everything linking `semio-framework-replication-edit` (seal): `demonstrator` module root and any plugin/module root that includes the replication/edit crate (bearbeiten host, reactor/os-store modules); browser release must be rebuilt afterwards.
