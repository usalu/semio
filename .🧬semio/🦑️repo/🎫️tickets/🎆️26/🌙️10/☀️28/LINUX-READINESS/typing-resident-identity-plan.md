# Typing Resident Identity And Paid Ownership Plan

Read-only current Rust Source review, 2026-10-29. No Source edit, test, compiler, native, process or lease operation occurred. Job52482 remains owner-reported queued; no runtime failure is inferred from this review.

Paths below are relative to 🧰️framework/🔨️modules/.

## Slot Identity Survives Container Movement

value/🧬️retained-clone/📦️original-slot/🦀️.rs:71 defines RetainedOriginalSlotKey as issuance+generation, not memory address or vector index. Slot fields at73 store those values; vacant at78 obtains unique checked atomic issuance; key/admit_key at90–91 compare exact values. A Rust move of the existing slot preserves both. Therefore Vec reallocation and sorted insertion do not inherently invalidate slot keys, provided the exact existing slot is moved rather than reconstructed/replaced.

A slot with immutable readers owns a Box<dyn RetainedOriginalReadOwner<T>>, so relocating the outer row moves the Box pointer, not its reader owner allocation. After birth, RetainedCloneSource seals the actual authority first and then computes its projected NonNull pointer (value/🧬️retained-clone/🔗️source/🦀️.rs:66–80). Its pointer addresses sealed owner storage, not the enclosing Vec row. Before that birth the staged authority remains inside the boxed reader owner. Existing retained reader backing therefore remains stable when another window causes Vec relocation.

Unborrowed TypingState movement copies native String/Vec headers while their backing allocations remain owned by those same headers. A Vec<M> net handoff through take/replace preserves the allocation pointer; cloning/rebuilding its elements would not satisfy the same-backing law. Rust borrowed references to row fields cannot legally survive a simultaneous mutable insertion; callers that persist identity must retain window+slot key or the existing immutable projection, not a raw row pointer/index.

## Real Current Integration Gap

tool-machine/⌨️typing/🦀️.rs:331–338 still declares windows as BTreeMap and has no writing field. Its APIs at355–451 use map get/insert/remove/keys/iteration; retained readers at465–476 also use map/range. The separate resident/🦀️.rs is Vec-shaped: binary search, insert, capacity, last_mut/pop, plus self.writing. It is not sufficient to attach that file as a module without integrating the parent representation and all these operations.

This is a concrete static shape mismatch, not an observed52482 compiler result. The long-term integration must choose one representation consistently, route cold/read/write/retirement APIs through it, preserve sorted lexical window order and retain the existing reader fences. Do not add a second shadow ledger or compatibility adapter that duplicates original ownership.

## Prospective Vec Path Invariants

Resident position/original_slot methods re-search by window, so insertion before an existing reader's row does not misaddress it. prepare_original_window returns the moved slot's issuance/generation key, never its numeric index. Capacity growth and row insertion must move all existing TypingWindow entries intact; no RetainedOriginalSlot::new/vacant reconstruction for an existing window. Existing cold_install's assignment creates a new issuance, so it is only valid after the old slot is known unissued/reader-free and the public source revision/write policy has admitted replacement.

Concrete counterexample to reject: retain key K for window z, insert window a before z, then use an old vector index0 to mutate/return z. Index0 now belongs to a; exact key admission must refuse rather than falling back to the occupant. Correct operation re-finds z and validates K. Reconstructing z's slot during reallocation produces another issuance and also correctly refuses K; never weaken key validation to make such reconstruction work.

Read fences remain local to the target slot: original-slot transform_original at99 refuses read ownership; take/install/replace enforce their current generation/reader rules; return_read at112 returns the exact original plus captured factory after child custody empties. Other-window readers may remain live during row movement because their boxed/sealed backing remains owned. close_original_window must stop on live readers, withdraw payload under its key, retire resulting owners, then pop only vacant reader-free rows and finally release vector allocation.

## Original Binding And Paid Work

OriginalTypingEdit in tool-machine/⌨️typing/✍️edit/🦀️.rs stores OriginalSourceBinding, native receiving decoder identity, exact TypingLedger object address and source_revision. validate/validate_source require all of them. Moving the internal windows buffer is compatible with the ledger object address; moving/replacing the ledger itself mid-edit is not. prepare-original mutations must update the edit's expected revision only after that exact admitted mutation, as current prospective advance does. Other mutation must remain fenced by writing; do not ignore revision drift or mint a substitute binding/decoder.

The edit's admission, text staging, key SharedUtf8 admission/clone, vector reserve/insertion, metadata creation, transaction mint, slot transform and residue closure all quote RetirementDemand, check every retained currency/should_yield, and consume actual progress. Resident growth quotes existing backing movement plus capacity/release; sorted insertion quotes shifted row bytes. Keep those costs proportional to native moved row headers, not cloned payload bytes. Any allocator capacity beyond the pre-admitted quote must still be covered by actual admitted grant and progress; do not treat try_reserve_exact as authority to undercharge physical allocation.

The result transform takes previous state and exchanges net Vec ownership with mem::replace/take; old net/request residue remains in ControlledRetirement owners. A stalled/refused grant must leave source/request/output ownership recoverable. Writer flag cannot clear until paid source closure; dropping an unclosed edit or reader remains an invariant refusal, including cancellation.

## Required Neutral And Physical Verification

Existing edit tests at ✍️edit/🧪️tests/🦀️.rs:10 compare observed requested/released heap bytes with retained receipts; test20 checks neutral outputs and net Vec pointers from incoming/fold backing. Keep those assertions and actual Original fixture caller/decoder.

Add to that same neutral/physical ownership envelope: reader in z across insertion of a; reader in z across forced backing growth; sorted multi-window iteration; stale key after actual target transform/replacement refuses; insufficient copy/capacity/release/depth/items grants cause no unauthorized mutation; same net pointer on all accepted fold choices; wrong Original/decoder/ledger/revision refuses; cancellation completes paid residues without mutating reader-held state. Validate equivalent neutral expected semantics through the existing independent oracle pattern. Require actual remaining slot/key/factory closure and zero unaccounted heap events. No success for these new cases is claimed here.
