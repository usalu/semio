# Native Operation Paging Boundary

## Current Producer

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`, `pack_patch_field` around 2250 and `kernel_patch_op_to_wit` around 2268, encode each operation field independently. The helper converts the typed field to a DSL value and passes it through `store::pack_rt::encode_wire_value`. It does not encode the entire UiPatch as one pack document.

| Operation | Exact current WIT payload |
| --- | --- |
| upsert | `val.node`: packed complete UiNodeRecord |
| set-component | `val.node`: u64 id; `val.component`: packed Component |
| set-layout/style/accessibility/bindings/menu | u64 `node` plus independent packed named field |
| set-activity | u64 `node`; packed `activity` is the `{activity, disabled}` wrapper |
| set-children | u64 `node`; direct list of u64 children, not pack |
| remove/set-root | scalar u64 id |

## Current Owned Decoder

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts` uses `nativeOperation` and `OwnedUiWireOperationCursor` to select the exact profile. Each packed field enters `RetainedUiTypedCursor`, whose retained wire decoder reads the general wire-pack document with its symbol table and field-1 arbitrary value. Direct children use their own typed u64-list cursor. Node ids above the exact JavaScript safe-integer domain are rejected, not rounded; native u64 parity remains an explicit unresolved domain boundary.

The Surface component's nested `doc.bytes` is a separate native scene-serde pack dialect; it must not be confused with the outer field wire pack. A nested scene `*Json` string may in turn carry JSON or `pk:` generic DSL pack. These are three distinct existing boundaries, not permission to introduce JSON as a new outer transport codec.

## Paging Join

Dag owns the upcoming canonical semantic return sections; Demonstrator owns TS return/source transport. Sequential operation framing should preserve the above field dialects while replacing the whole packed payload ownership with exact paged source authority. The existing input-retirement receipt remains distinct from semantic UI publication ACK. This note is source inspection, not native paging runtime evidence; no producer or wire API was modified here.

## Concrete Independently Owned Input Proposal

UI will own `OwnedUiOperationPayloadBuilder` and its privately minted `OwnedUiOperationPayload` result. The builder is bound once to the peer's exact operation/field source authority and declared byte length, not an entire return or a caller-supplied digest. It stores fixed 256-byte destination pages in a bounded-step linked ownership chain, with one current write page and exact total count. No contiguous full-operation buffer or whole directory allocation is required.

The callable shape is `offer(fragment): boolean`, `advance(grant)`, `peekCopiedFragment()`, `acceptFragmentRelease(receipt)`, `finishInput()`, `takeResult()`, `beginClose()`, `closeStep(grant)`, and `terminalIsEmpty()`. `fragment` must be the peer's private exact captured-content fragment, exposing selected byte reads and immutable operation/field offset/length; it cannot be a structural callback or arbitrary byte-reader. `offer` only reserves one current fragment. `advance(1,4096)` performs a page allocation, one byte copy, or one fixed transition. A fragment can span an operation boundary only after the peer's content framing has selected the exact field range.

The private copied-fragment token binds the exact fragment identity, operation authority, source range and builder. It is minted only after those bytes exist in independently owned destination pages. Peer release must verify this exact token and retain the raw page/fragment on refusal; UI retains the pending token until the exact peer receipt. The next raw page is never required before the current fragment has been copied and released. Incomplete multi-page payloads retain only their destination pages across the next raw-page ACK, not the previous raw backing. Copy completion is not typed decode, UI publication, or instance-retirement evidence.

`finishInput` requires the exact declared byte count and no pending fragment/release obligation. The private payload owner then supplies a retained sequential byte reader to the existing wire grammar. The grammar's bounded 512-byte text and eight-byte numeric scratch may cross destination pages but never requires concatenation; previous key/symbol comparison retains only bounded key bytes. Existing `RetainedUiWireValueCursor` currently uses one Uint8Array and subarray reads, so its input seam must be changed to this concrete owned reader rather than an interface callback that could conceal a monolithic read. The native typed component/node domain remains unchanged.

Cancellation closes the current fragment through the peer's exact ownership protocol, then retires one destination page at a time; unoffered raw remainder stays with the outer framing owner. The peer must supply the actual private fragment and release-receipt types before the UI API is compiled against them. Names above describe the UI-owned proposal only; they are not fabricated canonical actor types or an already implemented paging claim. Demonstrator owns outer tag/length and section-order decoding; UI will not duplicate that grammar.

## Required Shared Admission Hook

Fresh inspection of `OwnedUiInstance` finds immutable document limits and one pending lookup, but no byte-reservation ledger. The kernel activation registry has an actor-count/memory heuristic, not a callable retained byte lease. Neither `maxPatchBytes`, declared field length, nor fixed page size reserves shared resident ownership. The proposed builder must therefore remain non-admitting until this missing host-owned resource is supplied; no fulfilled budget is inferred.

The minimal hook is a concrete private-branded `OwnedUiResidentPool`, shared by every UI aggregate under the owning host composition, not one pool recreated per operation. The composition supplies explicit payload-byte and page-owner capacities. An instance receives an exact activation/lifetime-bound child owner from that pool; a payload builder receives a child of that owner. `tryReservePage(builder, byteLength)` atomically reserves both actual destination payload bytes and one fixed page-owner slot, returning a private `OwnedUiResidentPageLease` or refusal before allocation/copy. Page-owner slots separately bound metadata rather than pretending a JavaScript object's physical heap size is known.

The page allocation phase consumes only that exact lease. Failed allocation keeps or explicitly releases its unused lease under the same owner. A successful page stores its lease alongside its private byte array. Transferring a completed payload transfers the page leases, never releases them. Cancellation and final-reader retirement detach one page's data and bookkeeping before releasing its lease; raw-page ACK or semantic publication cannot release resident credit. A final instance witness requires all child leases returned, while closing remains authorized after operation revocation. Replay, wrong-builder/instance leases, exhausted shared capacity and concurrent operations must reject without counter changes.

This hook needs schema-first concurrency/cancellation/overflow laws and a real composition join before it can certify admitted input. It bounds retained logical resources; JavaScript garbage-collector timing and platform allocation latency remain separate measured obligations. No new pool, default byte cap, allocator claim or runtime admission implementation was added by this design note.
