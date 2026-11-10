# replayop: store/replay/operation redesign onto the 733 contract

## Step 1 (edited, unverified)
- New `native/👁️post`: `PostCustody<P>` (OriginalPostRead + resident `PostProducer` authority + selected action) drives Capture / Prepare(mode) / Drive(mode) / Return as one effect-level producer (receive/recover, nested projection Drive forwarded). The Source turn comes from `effect.work_source_turn()`; Pending keeps `pending=(op,demand,work)` and the same action recovers next turn. Stage `PostCapture` added; CloseEdit and close paths return the candidate (`Prepare(Mutable)` then `Return`).
- New `native/🌅️next`: `NextRootRetirement<P>` (Arc root -> elected payload -> `SharedOriginalStorage::admit_from` -> `FactorySharedRetirement::admit_original_into`), used by native close and derived close (replaces `FactorySharedRetirement::admit_original` over `Arc<P>`; `ArtifactReplayPrepared.next` stays `Arc<P>`). Debt: same election logic as publication's `ArtifactStorePostRootRetirement`; unify into a framework crate.
- Inverse rows: `OriginalVectorCursor::admit_into/take_front_into`; closing hands the cursor to `RetainedCloneClose` (`begin_granted`/`step_granted`), no RetirementCursor use.
- Arity drift: `take_output(&mut out,effect)`, `admission_borrowed(source)`, `close_step(body,effect)`, `step(_controlled)(body,..)`, `close_factory_ticket_controlled(slot,body,..)`.
Rerun: chk.sh W-replayop-<n> 🌎️hub/Cargo.toml native -p semio-os-store-replay-operation

## Step 2
- Native lib W-replayop-5 rc 0 (also added `impl OriginalSourceProjection<T> for CapturedSnapshotSource<T>` in snapshot-read). Wasm W-replayop-6 and tests next.

## Step 3
- Wasm W-replayop-7/8 blocked by job-testing-caller (driver2 owns turn_original/serve flag); my stray edit is gone. W-replayop-9 running; paged-one-item next.

## Step 3 (libs verified)
- native lib W-replayop-5 rc0, wasm lib W-replayop-9 rc0. Tests: tst.sh W-replayop-10 Cargo.toml native -p semio-os-store-replay-operation (SLOT_HANG_SECS=100).
STATUS: FIXED-verified libs W-replayop-5/9; tests pending

## Step 4 (paged-one-item)
- semio-os-store-paged-one-item native lib W-pagedone-2 rc 0. Post via PostCustody, projection via PagedProjection, mutation take via ReplyStash(take_authority_into/transfer_original_into), returned edit/post split in owners (production path, not admit_returned). Wasm + tests pending.
STATUS: FIXED-verified native libs W-replayop-5, W-replayop-9 (wasm), W-pagedone-2; tests pending

## Step 5 (tests, W-replayop-11)
- Test builds do not compile: 28 lib-test errors + operation-unit target. They are stale-API test code (SharedAllocation/Arc roots, 2<->3 arg close_step/step, OriginalVectorCursor/admit_original old calls, ErasedSnapshotRetirement::close_step impls with 2 params), not the lib. Removed duplicate #[global_allocator] from the moved operation-unit target (testing_allocator already provides one). Test port NOT done; open debt for a test lane.
Rerun: SLOT_HANG_SECS=100 tst.sh W-replayop-<n> Cargo.toml native -p semio-os-store-replay-operation
STATUS: FIXED-verified libs native W-replayop-5, wasm W-replayop-9, paged-one-item native W-pagedone-2; tests BLOCKED stale-API test code (W-replayop-11)
