# Native Resident Exact-Layout And Consumer Adversarial Plan

## Pinned Allocator Evidence

Read the pinned nightly-2026-07-07 local `alloc/src/raw_vec/mod.rs` regions try_reserve_exact/grow_exact/finish_grow/set_ptr_and_cap and `alloc/src/alloc.rs` Global runtime allocation/deallocation. For the currently selected default Global allocator, grow_exact computes `len+additional`, allocates that Layout, and records that requested capacity. Global returns a slice of exactly the requested Layout size. The source explicitly states its allocate/grow/shrink implementations never return a larger allocation than requested. This means the current native default-Global path cannot honestly be described as having reproduced capacity over-allocation. R2/R3 only reproduced the independent lazy mutex allocation.

The future-permissive Vec API and the candidate's post-allocation extra-debit branch still do not constitute a portable preadmission law. Replacing the page owner with an explicit exact Layout request removes that ambiguity and its after-allocation funding path. A forced larger-capacity test must be labeled an injected allocator/backend contract case, not an observed default-System allocator event. Changing the global allocator to return extra addressable bytes alone does not change Vec's recorded capacity in this pinned implementation; such a test would not exercise the branch it claims to cover.

## Meaningful Native Backing Laws

The next packet should record every actual allocation request in the existing fixed observer. Before the one page request, exact bytes/slots/owners must already be reserved in the original pending root. A zero/short resident capacity or zero/short work grant must produce no allocator event. Allocation failure must retain the original pending consumer and reservation or return the exact original source with no payload destructor. Typed page initialization must remain separate from allocation, and cancellation of an allocated/uninitialized page must release exactly its original Layout once. Alignment and large typed payload cases should be generated from Rust Layout, not TS prices. No successful test may rename caller-owned or global allocator metadata as charged data bytes.

For an adversarial backend that proposes a larger layout, the proposal must be checked before invoking its actual allocation. Refusal records zero allocation calls and leaves the source/pending record unchanged. An after-allocation `Capacity` error plus charged closing state is not equivalent. The existing exact default allocator is not to be replaced by an unbounded runtime dependency merely to manufacture a failure.

## Actual Mutable-Consumer Race

The public `Arc<Mutex<Option<C>>>` has a real source-level race: consumer_empty drops its guard, then root.close_step drops the erased Arc. A foreign thread can fill the Option and drop its own Arc in that interval; the root's Arc then becomes the last one and destroys C. The test should synchronize a real foreign thread at that exact interval, retain a fixed destructor counter, and keep source cleanup free of a second panic. This is distinct from replacing the predicate with a lying callback.

The fix must use the original phase-qualified registered consumer and registered receiver in the actual Opening/RuntimeAppCell parent. Holding the old public mutex guard through Arc drop is not enough: the final mutex cannot be destroyed while its guard is alive, and external mutation authority remains. No bool terminal certificate, Arc count sampling, arbitrary Option receiver or generated numeric identity will be substituted. Actual callback-tail quiescence still precedes the final native worker-cell handoff.

The access-repair12 source hold is released after actual native12 and both Wasm compile checks. No backing or consumer-race production repair is mounted. No native adversarial execution is claimed in this design packet.

## Staged Exact Foreign-Thread RED

The canonical admission fixture now declares `foreignRepopulation`: an empty observation has occurred, replacement is refused, and zero consumers are destroyed by the erased-alias release. The thirteenth native test uses the original public `Arc<Mutex<Option<C>>>`, not a substituted predicate. A cfg(test)-only one-shot channel interlock pauses the existing head-node release immediately after its real empty check. A real foreign thread fills that exact original Option, drops its external Arc, and resumes the original close. The parent has already dropped its external Arc. The current candidate can therefore become the last Arc and destroy the newly inserted consumer and its 32-byte payload in the original close call.

The test records the actual destructor count immediately after that call, joins the thread, and finishes the empty original ledger before checking the unchanged expected zero count and refused replacement. Its probe does not panic during Drop, so an intended first failure cannot be hidden by the earlier strict-probe secondary abort. Channel waits are bounded to one second and are test scheduling only, not runtime latency evidence. The hook and its two channel handles do not exist in non-test builds. The existing twelve tests remain selected; no exclusion or budget change is requested.

The permanent TS gate validates the declaration with strict Ajv and uses Immer's actually frozen object plus `Reflect.set` to independently check the sealed replacement-refusal model. This is an immutable-state oracle only: it does not reproduce Rust mutex scheduling or native Drop. The native test is the concrete race/destructor oracle. Both await the fresh results recorded below; no expected result is a passing-runtime claim.

## Actual R6 Result

The complete compiler-owned report `📓️resident-foreign-consumer-r6-native-red-2026-08-28.md` was read. Canonical exhaustive noargs executed all13:12PASS/1FAIL/0skip,.050s,Nx1. The new test printed `accepted=true consumerDropsDuringRelease=1 originalRootTerminal=true`, then failed the unchanged expected0 count after cleanup. There was no secondary abort. The current predicate/last-Arc release is therefore concretely unsound as a consumer retirement boundary. The next assertion about accepted=false was not reached; the accepted=true observation is actual debug evidence. Native source hold is released. The original12 pass only as reported in the aggregate footer; their passing diagnostics were not captured.

The independent TS gate actually passed Nx0, with complete output in `📓️native-resident-foreign-consumer-neutral-r1-2026-08-28.md`. Its scope remains the strict schema and sealed immutable-state oracle, not native scheduling.
