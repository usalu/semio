# Output Custody Audit

Read-only source inspection on 2026-10-21. No native compilation or execution.

## First-Party Queue API

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🦀️.rs`.

```rust
pub fn push(&self, chunk: Vec<u8>) -> Result<usize, Fault>
pub fn seal(&self) -> Result<usize, Fault>
pub fn take_chunk_original(&self, receiver: &mut Option<Vec<u8>>, terminal: &mut bool, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>
pub fn original_birth_admit(maximum: usize, receiving: &mut Option<Self>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>
pub fn original_birth_demands(maximum: usize) -> Result<RetirementDemand, ValueError>
pub fn original_identity(&self) -> usize
```

The last birth-demand method is implemented in the sibling construction module. There is no ownership-preserving original producer push method. `take_chunk_original` is a consumer method. `ArtifactFixedQueue<T>::push(&mut self, value:T)->Result<(),T>` preserves its rejected value, but the public output push maps that error to Fault and drops the Vec. `can_push(&self)->bool` exists on the fixed queue.

## Concrete Integration Concerns

- Current package commit removes the original Vec from its rope before calling media credit and public push. Credit rejection drops that local Vec; push rejection drops it too. Successful credit followed by push failure also advances credit without publishing bytes. The first-party producer seam must preserve a caller-owned Option until lock, seal, exact byte bound, slot and allocation admission succeed. Returning a fault with the Vec implicitly dropped is insufficient. A pending-credit flag or combined checked queue/credit commit is required if credit and push are separate phases.
- Public push rejects busy or poisoned ownership gates, sealed output, empty or >4096-byte chunks, full slots, byte overflow and exceeded total. An implementation cannot assume errors are impossible solely because output was admitted at construction.
- Current base64 take_prefix copies to a new Vec and pops exhausted raw chunks immediately. The source allocation is released before base64 output append can fail. Retain raw PagedList bytes through base64 using a read cursor; close them later under their own release authority. Current local input and encoded Vec temporaries also leave unquoted allocation/release work. A fixed byte scratch buffer and separately admitted destination growth remove those temporaries.
- Rope len and digest are historical generated length/digest; take_prefix and take_chunk deliberately do not decrement len. Preserve these semantics in PagedList replacement: base64 remaining uses historical len minus cursor, and checkpoint replay observes historical active-output length/digest after queue transfer.
- One encoder call can append multiple fragments. Original OutputWork must cover all fragments of that call before cursor/stage progress becomes durable; a late admission yield cannot replay already appended fragments. Retain exact source identity and append offset across turns, or reserve the whole bounded call demand before running the encoder.
- Queue handoff must retain the contiguous transfer Vec as job-owned pending custody across yields and errors. Copying PagedList bytes into that Vec needs its own capacity/copy receipt; popping/destroying source pages in that same turn additionally needs release admission. Leave source pages owned until explicitly retired if page-consumption cannot meet the same grant.
- Existing close stages Output and Encoded free each chunk and then the VecDeque allocation; terminal checks require their capacity to be zero. Replace both demand and stage dispatch with PagedList close methods, add pending transfer/OutputWork/raw source owners, and include them in terminal emptiness. Cancellation during encoding, base64, and package handoff must enter these same owners, including a fully copied but unpublished transfer Vec.

These findings were sent directly to the implementation owner.

## New Paged Output Review

Reviewed the newly authored unmounted `export/📤️output/🦀️.rs`. `transfer_one` retains a partially filled contiguous Vec between turns, and source bytes remain in PagedList through transfer. Concrete gap: after `try_reserve_exact(length)`, compare actual `transfer.capacity()` to `grant.maximum_capacity_bytes`; on allocator excess return a funded error receipt while retaining the Vec. The existing branch reports actual capacity but currently proceeds successfully even when capacity exceeds the grant. The general OutputWork partial-append concern above applies to the caller integration; it was not established as a bug in this isolated module.

The media credit public signatures are `credit(&self, bytes:usize)->Result<usize,Fault>`, `bytes(&self)->usize`, `validate_terminal(&self)->Result<usize,Fault>`, and the standard original birth helper. No reversal or coupled credit/queue transaction exists. Credit refusal sets a permanent exceeded flag, so retrying rejected credit is not a recovery path.
