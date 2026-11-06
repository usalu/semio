# Original AppFrame Receiving

## Actual Interface Audit

OpBinaryReceiving already accepts an actual borrowed &[u8] and exact range, so no RetainedCloneRef/SealedShared clone or copied payload view is necessary. AppFrame itself has no OpBinaryReceiving implementation. The existing ordinary async decoder allocates complete fields/rosters and nested causal envelopes; it cannot be used as the new Source-controlled decoder.

Canonical bounded OriginalBinaryReadSource is now implemented by existing PagedCommandReader and allocation-free OriginalBinarySlice. Existing Varint/Field atoms share this interface. Shared field allocation keeps the exact paged command maximum for that backend and the actual borrowed payload extent for the borrowed backend, with the atom's explicit caller field maximum checked independently. A read-only canonical() result permits strict nested local-interaction validation without changing generic atom acceptance.

## Intended Actual Decoder Owner

Domain path: framework OS spr/channel/app-frame/receiving. Schema-first input.json contains all 33 existing tags in their actual binary order plus valid scalar and overflow/truncation/trailing cases. AJV + independent SQLite/TextDecoder portable gate: 1 test / 16 assertions passed.

The actual receiving source now defines NativeAppFrameLimits, NativeAppFrameReceiver::new(limits), advance_from(&[u8],cx), with_frame_step(cx,unit callback), take_into_step(&mut actual Option<AppFrame>,cx), close_step(cx), terminal_is_empty. It qualifies the actual Source and complete immutable byte identity, retains every partial field before receipt posting, and moves the final frame only into a funded caller recipient. It implements actual OriginalNativeRetirement. The native receiving-scope helper captures its original child result before Source posting and keeps body, receipt, context and work refusal roles separately.

All 33 typed final transfers and non-causal continuations are authored: bounded UTF8/byte/scalar reads, child/head/window/byte rosters, recursive archive entries and strict local-interaction replies. Local-interaction pages parse directly from the immutable source into their final owned Page Vec; no ordinary decoder or whole inner-envelope mirror is used. Causal row fields/rosters and exact actor String donation into its shared header are also authored.

This owner is NOT yet registered or exposed to consumers: the canonical CausalPayload page controller is still being composed against the live foundation API. No supported-tag subset, Unsupported stub, ordinary-decoder fallback or alternate runtime path has been mounted. Missing payload module and package registration are explicit implementation work; current source is not described as compile-coherent.

## Canonical Reservation Coordination

Guest execution owns NativeSequenceCollection's new associated Reservation and bounded Vec/PagedList reserve/push implementations. The external PagedBytes adapter and causal field controller remain this lane. The final controller will use genuine cx.with_native_receiving control and actual available work, retain the reservation's opaque NativeReceivingEffect before allocation, and preserve its actual physical receipts. It must use the Source retirement recovery-only port for pending allocation recovery before generic partial-owner teardown; permanent cleanup refusal is not accepted.

Actual PagedListSelection ports are now present: select_reservation(limit,max_work), selection.demand(), reserve_work_demand(), reserve_selected(...actual marker). These are DATA quotes bound to actual storage, not Source authority. Exact Copy selection/kind retirement was added in the existing retirement trait owner using its existing direct paged-list dependency. No competing page implementation or guessed CPU allowance was created.

## Evidence Limits

Borrowed reader existing/new portable batch initially passed 3 tests / 43 assertions; canonical-varint cases were added afterward and require rerun. Its original four source entries parsed before that small extension. Owning command-ingress closure was16files114562bytes1681lines before the latest helper/getter/law additions. All hard limits remain unchanged. The new complete AppFrame owner, recovery path, native heap laws, actual MCP receiver and server journey remain uncompiled/unverified; no native artifact or readiness success is claimed.

Latest borrowed reader rerun: 3 tests / 47 assertions pass, including strict canonical-varint classification. Four actual Rust source entries parse without errors (rustfmt formatting-diff exit1). Current owning cfg(test) Source closures: command ingress16files116514bytes1692activeLines; retirement25files214273bytes2000lines. No limits changed.
