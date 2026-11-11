# mutapply5 (takes over mutapply4)

## Step 1 (edited, compile pending W-mutapply5-1)
- Canonical arm masking removed: pack json receiving returns zero progress when the cursor wrote 0 bytes and returned Progress(default) (no `size_of::<usize>()` receipt).
- Cause of the zero step at turn 175 (hypothesis from reading): `PagedList::release_page` is a no-op (`link.first_mut()` None -> default progress -> Denied) while `next_release_*` quote a release for an empty root Vec with capacity (copy 0, depth 2, release cap*size). A `[DEBUG] mutapply5 release denied` probe prints the list state on the refusing branch only (pack/json/encode/tree). Fix pending the probe.

## Step 2 (edited, compile pending)
- Prepared retirement ported to the reply protocol: `ArtifactStorePreparedRetirement` and `ArcRootRetirement` carry a resident `FactoryTicketClose` for their boxed `active` child; `close_into/recover_close_into/close_original_into/recover_close_original_into`, `native_header_release`, `next_computational_work`, original birth/control forwards. Legacy `close_step` with an active child now refuses ("requires its borrowed producer") like the other shared owners.
- Sync children in the chain got `native_header_release` + synchronous `close_into` (ArtifactStoreDecodedEditRetirement) / delegation (ArtifactStoreOneItemAuthorityRetirement).
- Facade reexports `FactoryTicketClose`.
- prepared_candidate test drives a top-level `FactoryTicketClose` through `clone_source_testing::drive::drive_reply`.

## Update 11/10
- Compiled + run: prepared_candidate passes (W-mutapply5-4/5), with exact work quotes (DecodedEdit and ArtifactStoreEditRetirement `next_computational_work` per phase; prepared strings stage = 1 exact, other stages 0).
- Protocol: prepared + ArcRoot `close_step` is only a refusal; sync stages live in `stage`, driven from `close_into`. mutation_apply prepared loops (lines ~324, ~432) use `close_into_step` (drive_reply). Not yet compiled/run. Other `close_step` callers in that test file (186, 347, 704, 739, 777, 796) are not prepared owners; unreviewed.
- Receiving stall (turn 175, member wire batch): NOT in the canonical cursor (no cursor refusal, no idle-admit, no poll-mismatch, no canonical-pending probe fired in W-mutapply5-5). W-mutapply5-6/7 (zero-phase probe) blocked by cargo lock refusal (job-recorded-clock, browserclock fixing, lock-145).
- [DEBUG] mutapply5 probes still present in: pack json encode tree (2), pack json decode receiving (4), decode/owned/protocol NativeReceivingChild::poll (1). Remove before finishing.
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-mutapply5-8 Cargo.toml native -p semio-os-store-one-item-batch --lib -- --test-threads=1 --nocapture original_member_wire_batch`

STATUS: BLOCKED cargo lock refused (job-recorded-clock, lock-145 pending); receiving stall turn 175 root cause not found; mutation_apply laws and PostFixture LOCK not started
