# WGPU Input Root Identity Review

## Exact Current Sources

The current UI-host UiThreadToken is a Copy zero-sized marker, not an exact queue identity. EventQueue has only per-instance input generation and no stable owner key. The WGPU render_frame_operation_id is process-static and shared by every event/metrics wrapper; it cannot distinguish two candidate roots. NativeHost has no canonical root identity field, while CanvasHost has canvas/generation transport identity, which is not a general native EventQueue identity. UiResidentPermit has a genuine occupied-slot/epoch authority, but its key is private to the document domain and its current public admission is the unchanged surface/document ledger. It must not be silently repurposed into an independent host allowance.

Sources read: UI-host 🦀️enqueue.rs capabilities/EventQueue; 🦀️window.rs NativeHost and CanvasHost fields; WGPU 🦀️winit_app.rs:40; UI-contract 🎟️resident/🦀️component.rs.

## Constraint for the Pending Candidate Packet

A key consisting only of per-queue counter plus frame operation/generation is insufficient: two live queues can share those values. The narrow writer facade fixes whole-root swapping, but does not by itself prove that an admission obtained from queue A is rejected by queue B. A raw address plus local epoch also fails across owner moves and allocator/address reuse.

The native tests must include two live queues with equal numeric counters and later owner-address reuse, besides same-queue stale epoch and actual callback-verdict replay. No public mutable root, caller-selected nonce or copied successful CallbackVerdict may authorize the replacement.

The concrete physical root identity remains a prerequisite before mounting production. A stable allocation identity with retained weak ownership would avoid address ABA but introduces real backing and a final-key release obligation; it cannot be inserted as an unmetered Arc/Box. Reusing an admitted canonical resident owner could avoid a second authority, but needs the actual host-composition binding rather than borrowing the document quota by assertion. This review does not choose or implement a parallel budget/registry. The private affine InputCommitTurn continues to own the real Watchdog directly and does not require any global callback nonce.

## Status

Read-only. No production changes, no native identity claim. The five current baseline RED laws and neutral 22-case oracle remain queued behind canonical UI-host metadata registration; their scope does not yet include this new-key adversarial test.

## Checked Sequence Alternative After Reuse Census

The nearest existing mint is trace::allocate_operation_id at ⏱️trace/🦀️component.rs:641. It uses unchecked fetch_add and returns a publicly constructible OperationId(pub u64), so its current API does not provide permanent exhaustion or private candidate-root authority. async::CapabilityTokenId is only a caller-owned correlation handle, with no mint. UiDocumentRootIdentity is deliberately bound to an admitted document handle, not a window/input root. Reusing any of these unchanged would either inherit wraparound or incorrectly borrow a different domain's live owner.

Proposed smallest local alternative: one crate-private InputRootSequence wrapping exactly one AtomicU64 initialized to zero; one process/static instance for the linked UI-host crate. try_mint performs one load, checked_add(1), and one strong compare_exchange. No retry loop, mutex, allocation, map or registry. A competing mint returns Busy without mutating the caller's queue; exhaustion returns Exhausted permanently. The transition MAX-1→MAX is valid, and every later mint refuses. Values are never returned to the sequence, including cancelled or failed queue constructions.

EventQueue retains an optional NonZeroU64 root field, initially absent. Only actual queue admission may install it, after preflighting the physical fixed-metadata grant and empty structural target, before payload allocation/normalization. A refused zero/short grant cannot consume a root number. Successful mint installs the root immediately in the private queue; later backing failure retains that admitted root and any actual backing, never remints or returns a loose root owner. New() remains an unadmitted empty shell, not proof of physical admission. No public constructor, setter, serde conversion, raw root access, mutable candidate reference or external nonce is added.

The candidate key contains that private root identity plus a checked, non-reused per-root admission epoch. It is not a live ownership container and carries no String. Writer lookup validates both against the structurally installed candidate before any source mutation. Moving EventQueue preserves its root identity. Dropping and reconstructing at the same address obtains a later root, so address reuse cannot revive an old key. Equal counters in distinct live queues still have unequal roots.

### Collision and Accounting Scope

Uniqueness is within one linked UI-host crate instance/native process or one Wasm memory instance. Keys are private in-memory types and must never cross a serialized browser/component transport; canvas/actor wire identities remain separate. Separate processes/Wasm memories can start at root1 without a collision because their private keys and roots cannot be mixed through this API. This is not a global distributed identifier and not callback authorization.

Declare size_of::<InputRootSequence>() once as fixed static physical storage; declare size_of::<Option<NonZeroU64>>() in every queue's actual metadata, plus the entire concrete admission/candidate key fields and padding in the candidate size census. The expected atomic size is eight bytes on the supported targets, but native/wasm layout checks must measure/compile that assertion rather than hide it in a guessed total. This identity counter has no independent byte/slot allowance and does not authorize payload allocation: existing physical admission still must cover queue/candidate/source backing. A spent id after cancellation is intentional, not a leaked capacity permit.

### Required Native and Neutral Laws

- Zero/short metadata grant leaves both exact queue shell and sequence unchanged.
- Two live equal-counter queues reject each other's keys; same-address reconstruction rejects the former key.
- MAX-1→MAX succeeds once; the next and repeated calls refuse with no queue/allocation changes.
- A controlled competing single-CAS failure preserves the unadmitted receiver; successful concurrent mints are all distinct.
- Failure after root installation preserves that exact installed root and actual backing across retry/cancel/unwind.
- Native fixed metadata sizes and browser wasm32 compilation; independent BigInt fixtures for MAX boundary and non-reuse.
- Callback replay remains separately tested through the private affine InputCommitTurn; this root sequence never replaces or validates a Watchdog verdict.

Status: parent reviewed and approved this exact schema-first private-sequence direction. No production or baseline fixture changes yet; baseline5 must execute first. No new clock, quota, ownership ledger or callback nonce. The checkpoint compile hold has now released; only separate unmounted schema/oracle staging follows.

## Schema-First Staging

Separate unmounted root fixture, schema and oracle are now authored under admission/🪪️root. They reuse the parent schema's exact u64 definition instead of duplicating its range pattern. Six arithmetic vectors cover zero/short admission, first/last identity, permanent exhaustion and one-CAS contention; exact little-endian bytes use Node Buffer, and cross-root equal epochs/non-reuse use independent BigInt arithmetic. Six schema hostiles are authored. This is not native concurrency, allocation, queue identity or Watchdog proof.

The child oracle is not imported by the canonical source target yet and has not executed. No native root test is mounted and no production identity exists. The original baseline5 plus22-case source fixture/script/Cargo remained unchanged: canonical metadata released, sourceR1 passed22cases/7hostiles/3frontiers, and baselineR3 actually failed all5 intended semantic assertions after the separate test-only browser cfg join. The missing-API root test mount may now follow the required parent review; it must not erase those baseline failures.
