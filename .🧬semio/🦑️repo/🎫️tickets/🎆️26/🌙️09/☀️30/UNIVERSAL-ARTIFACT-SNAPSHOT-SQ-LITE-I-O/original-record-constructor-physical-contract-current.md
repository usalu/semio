# Original Record Constructor Physical Contract

Read-only observed current mounted source. No tests run.

The generic decoder now retains `(Option<RecordSpec>, Option<RecordValue>, Option<T>)` in the original receiving frame (`🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs:36–55`). The callback receives the actual third Option by mutable borrow. It can install genuine empty T before field births and leave actual partial T on refusal. Successful output.take transfers only T; spec and record remain in the frame and must retire before output publication. Generated spec.decode and record parsers still return complete objects before assignment, so their own internal partial construction is not newly qualified.

## Tuple and Frame

Value `♻️retirement/🦀️.rs:372–378` has real three-tuple RetireOwned: a Sequence with three Deferred fields. controlled_retirement_supported requires all three real field types; Option328 delegates its inner support, even when None. Tuple birth measurement is exact Sequence scaffold plus three deferred field owners (45–47). It does not use a fake blanket owner.

IO `⏱️control/🛫️snapshot/🦀️.rs:106–139` measures size_of NativeSnapshotReceiving<T,O> with the actual tuple type, rejects unsupported factories/item0/depth0/insufficient frame capacity before invoking the callback, charges the original allocation port for frame, and retains the Box on any error. The tuple increases inline frame size and deferred retirement depth versus the former pair. Existing grants must be explicitly authored to cover actual new scaffold and measured close demands; do not assume old pair extent or fund from SQL ceiling. NativeSnapshotReceiving.close_step29 adds one parent depth and decrements before delegating; each tuple/deferred/option child may require further depth. Admission depth1 alone permits operation but cannot guarantee terminal retirement, by design.

Success retires the spec/record/empty output slot with original remaining grant, charges next retirement capacity through the same native port, accumulates each receipt, and publishes output only after actual terminal-empty plus funded frame release. On failure, frame custody remains in with_retirement_owner. Native owned-byte charge is cumulative admission, not live bytes; retirement receipt release must not decrement it or recharge existing backing.

## Reserved Vec Constructor Law

Use the actual registered T.buffer (or actual nested Vec placed into T) before fallible checkpoint. Original allocate_vec reserves under charge; immediately move its empty backing into that field. Fill only its reserved capacity in bounded spans under original scoped_stage/begin_stage/advance; append before advance so a canceled post-span checkpoint leaves the physical prefix in T. Scoped stage restores parent workload on error while retaining original cumulative owned bytes. Do not invoke copy_bytes, which hides a local partial Vec. Do not reserve again without another authentic original admission.

Observe original allocator callback sequence and native owned_bytes before/after, actual as_ptr/capacity after reservation and after refusal, actual len/prefix contents at retained recipient, and actual terminal retirement. Pointer equality plus unchanged capacity demonstrates same physical buffer; counter alone does not. For Vec<Vec<u8>>, outer and every inner backing must be installed before a fallible copy checkpoint.

Five zero-axis tests must target the live retained frontier whose measured demand for that axis is positive. Item0/depth0 block any nonterminal frame; copy0/capacity0/release0 are not universally inert because some legitimate turns demand zero on those currencies. Inspect next_copy_byte_demand, next_capacity_byte_demand, next_release_byte_demand and select the genuine relevant frontier, then assert denial preserves pointer/capacity/len and actual custody. Never assert every zero currency necessarily refuses every turn. A full explicit original grant should retire to terminal-empty, with receipts fitting each supplied turn and final frame release separately observed.
