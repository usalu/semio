# Lane mounted result

## Port: publication-mounted grant -> effect (EDITED, native lib compiles W-mounted-2 rc0)
- Leaves effect-shaped: `payload_close(bytes, effect)` / `MountedReceiptPayload::close_step(effect)` (release_empty_page through the effect, Denied -> Progress(default), no auto-copied-item), `MountedPreparedOperationBytes/OperationsBytes::{advance,close_step}(.., effect)`, `MountedKernelMutationReceipt::{advance,close_step}(.., effect)` (author `SharedUtf8::admit(.., effect.grant(), effect.actual())` + work recorded on the effect; precheck of 2 work units so a denied turn charges nothing; refused admission hands the original bytes back into `strings[5]`).
- Threaded up: `MountedMemberReceipt::{advance,close_step}`, `MountedGroupReceipt::close_step` (now `Result`), `MountedGroupMemberReceipts::{advance_receipt,advance_row,close_step}`; nested steps use `effect.nested(narrowed grant)` (same work sink, same actual marker).
- Pure grant steps (byte copy, reserve, append, take, handle issuer, metadata, final assembly) reach no effect leaf and stay `grant` (the caller passes `effect.grant()`).
- Callers: `MountedPrivateChildGroup::close_step` (owner) runs `receipts.close_step` through `plugin_effect_close(cx, grant, ..)`; instance typed-ops `child-group` runs `advance_receipt` through `plugin_effect_close(cx, grant, ..)`.

## arc root 16-byte gap (FIXED-verified native W-mounted-t1, value-arc-root 5/5)
- Cause: the root keeps its captured issuer (`RootIssuer`, an `Arc<dyn ..Factory>`) until the shell `Drop`; the child only receives a clone. With a unique factory (the law's ZST `OwnedValueRetirementFactory`) the shell drop freed the last strong reference, i.e. the 16-byte `ArcInner` header, with no receipt: the allocator freed 992 while the shell release reported 976.
- Fix (production, `arc-custody/factory/root`): after payload/storage/active/ticket are gone the root hands the issuer to a `FactoryAuthority` (new field `closing`), drives it through the same funded reply protocol as `FactoryOwnedRetirement` (so the Arc is elected with `Arc::into_inner` semantics and its header release is receipted, or 0 when another holder remains), then drops the authority in a last unit step; only then is the root `terminal()` and `native_header_release` quotes the plain shell. Touched: `terminal`, `issuer_closing`, `demands`, `work_demand`, `stage`/`release_issuer`, `close_reply`, `requires_original_birth/control`, `close_original_birth_step`, `Drop`.
- Law restored: `arc_root_closes_through_the_factory_ticket` loops until the slot is empty (final shell release through the ticket) and asserts the payload alias is gone.
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-mounted-tN "🧰️framework/🔨️modules/🌱️value/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-value-tests --test value-arc-root`

## Verification
- native lib publication-mounted: W-mounted-2 rc0.
- wasm32-wasip2 via `🌎️hub/Cargo.toml`: W-mounted-w1 rc0 (semio-framework-value-factory + semio-plugin-publication-mounted).
- value-arc-root: W-mounted-t1 rc0, 5 passed.


## Rerun after disk freed (W-mounted-t3 / W-mounted-t4, native)
- lib (`--lib`, W-mounted-t3) rc1: 7 passed, 6 failed (production compiles; failures are test fixtures). 5 of 6 (`mounted_group_receipt_*`, `mounted_member_receipt_*`, `mounted_group_member_receipts_*`) unwrap `OriginalRetirementFailure { primary: ValueError OwnershipLimit "shared allocation birth is not funded" }`: fixtures hand a plain grant that no longer funds shared allocation birth under the effect API. 6th (`..refuses_missing_original_metadata_before_acknowledgment`) asserts `(0,0)` but gets `(64,0)` at retained-clone testing `🦀️.rs:212` (expected work/allocation accounting now charges 64).
- publication-mounted-tests companion (W-mounted-t4) rc1: does not compile; E0609 `no field default_*` on `*NativeReceivingState<I>` (RetainedCloneProgress/Grant, NativeEncode/DecodeAllocationOutcome, NativeAllocationRequest): the 84 errors are located in `value/🛬️decode/🧳️owned` (+ `🌳️value/⚠️refusal`), i.e. the derived Default-style fields were removed upstream; not mounted code, needs a port in that crate.
- Fix path: give the 5 receipt tests an effect/funded-birth fixture (effect with allocation grant for shared birth) and update the 64-unit expectation; port companion tests off `default_*` fields.

STATUS: PRODUCTION FIXED (native lib compiles, W-mounted-2); TEST DEBT: lib 6/13 red (fixture funding), tests companion uncompilable (default_* fields).
