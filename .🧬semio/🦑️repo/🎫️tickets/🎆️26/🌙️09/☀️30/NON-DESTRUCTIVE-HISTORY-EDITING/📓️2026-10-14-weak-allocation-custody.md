# Original Weak Allocation Custody

## Current Evidence

Read-only current source identifies existing first-party allocator custody in framework OS Neural engine registry/🦀️.rs. ShellAllocator uses actual std::alloc::Allocator::allocate Layout; RegistryAllocator records the actual pointer/Layout in the original retained final-root slot at deallocation. RegistryRetirement releases that recorded original shell only through a funded turn. The package already enables nightly allocator_api under the repo's pinned Rust toolchain. No new runtime library is needed to investigate this mechanism.

Current Runtime cleanup jobs retain std::sync::Weak<RuntimeAppCell/RuntimeCloseState>. Their final Weak may free the shared allocation after the strong payload has retired. A zero-release Drop or a guessed private Arc header layout cannot qualify that cleanup. No canonical generic first-party weak ownership API was found in the framework module scan. No Root Weak producer has been changed, and no native evidence is inferred from Neural source.

## Candidate Canonical Mechanism

Generalize an explicitly admitted shared allocation with a first-party allocator and retained original retirement authority. The allocator receives the real Layout from the system framework; final deallocation transfers the actual original pointer/Layout into the retained authority rather than immediately releasing it. Each strong/weak reader also retains that authority so the original shell handoff cannot outlive its recipient. The authority must remain live through the last reader handoff, and quoted paid close must separately retire payload and original allocation/header. Do not use allocation tracing as runtime infrastructure, infer private std Arc layout, let a passive callback allocate an unpriced queue, or introduce a lifetime/cancellation source replacement.

Before implementation: strict neutral schema/fixture, independent allocation/reference oracle, genuine native RED with strong/weak final-owner orders and thread interleavings, original item/copy/capacity/release/depth denials, exact callback Layout capture, partial cancellation, original recipient/identity binding, actual complete born/released conservation, and terminal Drop0. Constructor allocation must precede moving the original payload so refusal preserves it; Arc::new_in(value) alone is insufficient because allocation refusal can drop the input. Required original issuer/grant and allocation ceiling must be captured before constructing either authority or payload allocation. A fallible uninitialized Arc allocation can preserve source custody only after validating actual standard API and receiving its observed receipt under the first-party caller control.

Strong aliases can race into final payload ownership; a count snapshot is insufficient permission to drop. Original retained root/witness or a genuine last-reader handoff is required. A scoped source borrow cannot be placed into moved Send+'static Worker factories. Domain-neutral allocation ownership must expose first-party types/interfaces, while Runtime-specific payload closure uses its existing genuine app/Store/weak retirement sources. Cross-platform Native/Wasm callers and pinned allocator_api feature propagation remain unqualified.

## Qualification

This is a design/research candidate, not implemented or tested. The source/native/editor acceptance gap remains explicit. UI currently retains unsupported Weak sources rather than declaring a zero-byte terminal drop. No first-party weaker fake owner or foreign producer overwrite is authorized by this report.


## Existing Strong-Only Framework Source

Further read-only actual source found Value retirement/shared/sealed/🦀️.rs. SealedShared explicitly forbids raw Arc, Weak, mutable or consuming escape hatches; it owns original first-party issuer/issuance and fixed alias return slots. It therefore provides no generic weak allocation ownership today. Adding Weak directly would invalidate its existing strong-only header-release and final-reader invariants. An explicit original strong lease for Runtime cleanup could be explored with genuine parent close ordering and paid source handback, but replacing Weak with a strong lease can create the Cell→pump→session→Cell ownership cycle; explicit paid cancellation/cleanup must break that cycle before final body/header retirement. No such Runtime bridge or cycle proof is implemented, and no Root source edit has been made. The new candidate cannot be claimed viable merely because its APIs parse.
