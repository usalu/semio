STATUS: GREEN (native lib check) for semio-os-store-one-item-publication (store4-104 rc 0) and semio-os-store-snapshot-clone (store4-110 rc 0). Tests, wasm and downstream callers UNVERIFIED.

## Ported (EDITED, awaiting compile)
- authority/🦀️.rs: RetireOwned seed form + close_step(effect) via erased_cursor_turn.
- canonical-edit/🧳️source: ArtifactStoreCanonicalOwner advance/take(effect); pay/polled/paid_progress helpers; AuthorityLease seed.
- lifted/canonical-edit sealer: advance via NativeDecodeControl::with_allocated_effect; close_step(effect)->OriginalRetirementPoll (close_ladder + settle).
- publication 🦀️.rs: ArtifactStoreOneItemPreparation::close_step/close_original_step(effect), retire_input(&mut Option, effect)->Poll, ArtifactStoreBatchSource::close_step(effect), admit_actor_lease(effect).
- lifted/owned: BatchRetirement::step(effect)->Poll; BatchAdmission/begin now (&mut Option<T>, effect).
- diagnostics stage, batch digest, prepared retirement, operation-wire receiving + binary (advance(..., maximum_work_units, grant)->Poll, take_owned_into/close_step effect).
- value: ADDED factory_retire_snapshot (additive).

## Downstream callers NOT ported (other crates): snapshot-clone/assembly, paged-one-item, composition emission owned (BatchOwner, admit_batch_retirement), snapshot-clone metadata (admit_actor_lease), operation-wire/hydration callers of OriginalOperationReceiving advance/close_step.

## snapshot-clone (coordinator add-on)
EDITED (UNVERIFIED): metadata RetireOwned seed form; RetainedClonePreparation close_step(effect)+close_ladder; pending::close and assembly::close_step effect ladders; four receiving-recovery trait stubs (no pending recovery). NOT done: ⚖️law driver (uses removed NativeDecodeConstructionRecipient; needs rewrite against OriginalConstructionRecipient + job receive API); advance_original ladder (grant-based, children now effect-shaped) will surface next.
store4-105 stopped upstream at semio-framework-replication-history-index (peer mid-edit of fold/index/🦀️.rs, missing deferred_birth_bytes_for import); rerun queued as store4-106.
Store crate store/🧵️operation-wire and space-history untouched by store4 (free for store5/store3).

## Round 2 status (store4-107, log 🗑️generated/1105b/store4-107.txt): snapshot-clone 63 errors, UNVERIFIED fixes after that run
Imports fixed since: original_vector now `semio_framework_value_original_vector::OriginalVectorCursor` (Cargo.toml dep added, LOCK NEEDED logged); NativeDecodeConstructionRecipient is gone, law driver rewritten onto OriginalJobFixtureCaller + cx.with_native_receiving (snapshot-clone Cargo: semio-framework-job features=["testkit"], also manifest change).
Remaining in snapshot-clone (per store4-107): 🦀️.rs 29 (advance_original ladder: last_work u64 vs u128 work, source.take_authority/close_step now (effect) Poll, admission/cursor grants), ✏️config-apply 13 (RetainedCloneEditCursor::begin_close removed from cursor use, grants), 📦️assembly 7 (OriginalVectorCursor::begin_close gone: its step/close is now seed-based; drop the call), 🪪️metadata 5, 🚚️handoff 3 (cursor.begin_close), ♻️custody 3. Pattern for all: pay grant-shaped work via native_source::pay, effect.nested(child) for effect-shaped children, RetirementPollExit::resume for Polls.


## Round 3 (coordinator corrections)
- Manifest: `law` feature (off by default) on snapshot-clone enables job/testkit; law driver compiled only for `cfg(any(test, feature="law"))`; testkit is a dev-dependency; production deps carry no testkit. plugin window config replace crate gets a dev-dependency with feature `law`. LOCK NEEDED logged (snapshot-clone, replace Cargo.toml).
- Receiving-recovery methods: not stubs. The clone preparation opens every native effect atomically (with_effect/with_allocated_effect), so no receiving frame can stay pending; has/pending queries answer false/None and finish_receiving_recovery is a typed InvariantViolated refusal with that reason.
- snapshot-clone: advance_local runs under receiving.with_effect; seal under its own funding (seal_turn); close_ladder; cursor begin_close_paid/close_started; domain RetainedCloneEditCursor::{advance,close_step} now take the effect (last_turn_work removed); RetainedEditCustody::step(effect); metadata/assembly/config-apply/handoff ported. Verified only by store4-110 lib check.
- Ported callers (EDITED, UNVERIFIED): plugin window-config replace cursor, camera-owner test cursor, test-app-mutations-config selection cursor (advance wrapped, close via custody), testing doubles retained-edit, composition emission owned (TypedBatch/MemberStoreOwnedBatch::close_granted(effect)) and emission receiving (MemberOriginalBatchReceiving advance(maximum_work_units)->Poll, settle/close_step(effect)).
- Not mine / not touched: paged-one-item (lane paged claimed the crate; store4-112 shows 4 errors there incl. SemanticEdit trait mismatch) and replay (store3, 5 errors). Plugin owners still calling MemberStoreOwnedBatch::close_granted(grant): plugin composition group/owner:708, group/mounted/owner:99, group/🦀️:176, publication/🦀️:208-209, decl emission preparation:75 -> need effect (their lifecycle lanes).
- Stale tests in snapshot-clone/composition emission tests still use grant forms (not compiled).