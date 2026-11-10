# mutapply5 (takes over mutapply4)

## Step 1 (edited, compile pending W-mutapply5-1)
- Canonical arm masking removed: pack json receiving returns zero progress when the cursor wrote 0 bytes and returned Progress(default) (no `size_of::<usize>()` receipt).
- Cause of the zero step at turn 175 (hypothesis from reading): `PagedList::release_page` is a no-op (`link.first_mut()` None -> default progress -> Denied) while `next_release_*` quote a release for an empty root Vec with capacity (copy 0, depth 2, release cap*size). A `[DEBUG] mutapply5 release denied` probe prints the list state on the refusing branch only (pack/json/encode/tree). Fix pending the probe.

## Step 2 (edited, compile pending)
- Prepared retirement ported to the reply protocol: `ArtifactStorePreparedRetirement` and `ArcRootRetirement` carry a resident `FactoryTicketClose` for their boxed `active` child; `close_into/recover_close_into/close_original_into/recover_close_original_into`, `native_header_release`, `next_computational_work`, original birth/control forwards. Legacy `close_step` with an active child now refuses ("requires its borrowed producer") like the other shared owners.
- Sync children in the chain got `native_header_release` + synchronous `close_into` (ArtifactStoreDecodedEditRetirement) / delegation (ArtifactStoreOneItemAuthorityRetirement).
- Facade reexports `FactoryTicketClose`.
- prepared_candidate test drives a top-level `FactoryTicketClose` through `clone_source_testing::drive::drive_reply`.

## Open
- Steps 3-5.

STATUS: BLOCKED in progress (W-mutapply5-1 pending)
