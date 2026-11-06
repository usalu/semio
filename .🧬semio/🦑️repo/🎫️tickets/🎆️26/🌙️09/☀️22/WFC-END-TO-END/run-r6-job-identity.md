# Original Job Construction Loan Lifetime

Current production StepContext::with_native_receiving in job/⏱️context creates OriginalJobConstructionRecipient on its stack for each call, borrowing the real retained and receiving ledgers. It passes that ephemeral backend to OriginalJobDecodeOwner::receive. The decoder owner in job/🛬️receiving persists across turns, carrying detached native continuation and retirement recipient.

The new NativeConstructionRecipientLoan must persist with the actual original backend across pending callback resolution. Minting a new token around the stack recipient on every call would change authority and cannot resolve a frame retained after a callback panic. A fake default ID, pointer address or fresh wrapper per rebind is rejected.

The production API must receive a caller-held long-lived loan/backend, or redesign ownership so the original Job caller retains the construction backend and loan across turns while lending current grant/ledger state. This must preserve the actual Job ledger identity and receipts; it is not sufficient to add a field solely to satisfy a compiler.

OriginalJobDecodeOwner::receive currently takes its continuation out of self, calls decode and only detaches/restores after the call returns. It also needs explicit unwind restoration: catch before the continuation is dropped, detach the same native owner, restore self.receipt, then resume the original panic. The actual token must stay alive across that operation.

Required owning tests: real Job receiving with partial retained frame, original callback unwind, the same continuation and recipient after unwind, a new turn using the same loan, wrong-loan rejection with zero callbacks, original callback resolution, and exact paid closure. Existing neutral7/headless tests do not establish these new semantics.

Artifact verifier owns the generic typed NativeReceivingEffect and decoder construction integration. VCS owner supplies stable allocation loans and typed original-frame reborrow. Root owns this Job/UI production integration once the actual long-lived loan contract is coherent. No Job source edit or production pass is claimed yet.

## Production Change Authored

After the generic construction API adopted explicit durable source delegation, root added NativeConstructionRecipientIssuer to OriginalJobDecodeOwner. Each receive turn issues a short loan to that turn's real existing backend; the source issuer remains owned by the original decoder across turns. This supports changing borrowed turn closures without inventing a new source identity.

Root also wrapped receive and retire continuation calls in catch_unwind, detaching and restoring the exact continuation before resuming an operation panic. Concurrent NativeDecodeCallFailure<T,E> result custody changes were preserved rather than mapped down.

A portable constructor fixture and owning Job-receiving law were authored first. The law verifies native ledger/retirement identity after operation unwind, quotes/effects in one receive scope, and posts/takes the same construction frame in a second scope. It also closes the original decoder. It has not executed; current lower owner contracts are still compiling.

Root owns this specific production change and its actual test. Actual same-source recovery of a retained callback remains dependent on the generic frame/loan protocol and complete Job caller integration; a new temporary source issuer is never created by resume.
