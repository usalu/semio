# WGPU Event and Metrics Admission Declaration

## Canonical Fixture Location

Taxonomy-requested manual moves preserve exact bytes: `📥️input/🎟️admission/🧪️tests/🔣️.json` (SHA-256 `a33d1397b7a9b9da06dca7b3a83353592ed2e9e2b0b0f3785e8ed515fa643ecc`) and `📥️input/🎟️admission/🧬️schema/🔣️.json` (`3749abdc0206f9a5b7d7ec7c90d9f0fcf3dde6e00f8d46810b750e82e6f0c6bd`). The older filenames below identify the initial declaration only. Domain script and native includes now use the canonical locations. Cargo gained only the existing workspace serde_json dev dependency; no runtime dependency was added.

## Review Boundary

Schema and source-plan only; no EventQueue, WindowDelegate, WGPU, mailbox, surface-lane, scheduler or trace production edit. No WGPU Cargo run or preimage release. The permanent neutral fixture/schema are at `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔣️.json` and `🧬️schema/🔣️.json`. Their current 22 cases declare expected behavior, not executed native proof; the initial 20-case declaration is historical. Redraw, offscreen presentation, GPU submission and snapshot-sink retirement remain outside this packet.

## Exact Current Callers

| Owner | Current source and gap | Required ownership change |
| --- | --- | --- |
| EventQueue | UI host `📦️packages/🦀️rust/🦀️enqueue.rs:263–305`: generation changes before capacity refusal; consumed String is dropped on Overflow. | Checked candidate generation and admitted receiver before source transfer; exact source remains retained on every refusal. Remove the public consuming enqueue/Overflow route when callers migrate. |
| OsHost callbacks | WGPU `🦀️winit_app.rs:54–110`: event/metrics guards never finish; metrics continues into both other queues. | One typed callback result, minted by the existing real Watchdog, gates the entire event or three-sink metrics transition. |
| Shared native host | UI host `🦀️window.rs:369/372,562/566/589`: void delegate consumes normalized input. | Caller-owned pending event/metrics and typed acceptance. Refusal must not destroy its exact normalized owner. Reserve before normalization, not after creating an arbitrary String. |
| Shared browser host | UI host `🦀️window.rs:955–1004`: takes pending event/pointer/metrics and acknowledges despite delegate refusal being unobservable. Clipboard :1071 clones text then consumes it. | Keep original transport event and normalized owner in structural pending fields through callback/refusal/unwind. ACK only after accepted transfer; clipboard must use an admitted retained source, not another full clone. |
| WGPU native loop | `🦀️winit_app.rs:775/781/796` calls the same void methods. | Same receiver-before-normalization discipline, no ignored typed result. |
| WGPU browser worker | `🦀️browser_worker.rs:425/499` calls metrics/event directly. | Preserve exact pending source and propagate typed refusal through the actual browser control reply, not success followed by local Drop. |
| Test delegate | UI host `🦀️window.rs:1682/1686`; WGPU callback latency fixtures :865–901. | Migrate signatures and assert actual ownership, never provide a compatibility default accepting everything. |

The two writer helpers above are the production EventQueue entry points found by the current scoped census. WindowDelegate is the shared canonical boundary, so a queue-only fix would be incomplete.

## Candidate and Receiver Contract

1. Reuse the existing EventQueue capacity of 256 discrete events, 4096 logical bytes per event and 1048576 logical queued bytes. Reservation counts against that existing capacity, including any pending candidate. Physical backing remains actual capacity × size plus each owned String capacity, distinct from logical bytes. Do not label a small String length as its allocation charge. No implicit reserve/grow in the callback.
2. A pending source is structurally owned before entering the producer/callback. Candidate methods take mutable source/receiver borrows rather than accepting and returning a temporary owned event through an unwind boundary. Full, zero grant, cancellation, generation exhaustion, missing/backward clock and watchdog fault leave the exact String pointer/capacity/content and original generations unchanged.
3. Stage checked frame and input generations; reject u64 exhaustion before mutation. Only an accepted candidate advances each once. Stage coalesced pointer/scroll values in fixed Copy metadata; preserve scroll accumulation and original ordering.
4. Metrics needs simultaneous preflight of EventQueue, existing surface-lane token/generation and the existing RuntimeCompletionQueue. The mailbox guard must use try_lock and distinguish busy from poison. Its checked revision and actual fixed receiver must be reserved before any sink mutation. Do not call current RuntimeMailboxInner::enqueue: it increments revision early, blocks twice, can drop a replaced RuntimeApply and invokes an arbitrary waker.
5. Reuse the existing surface lane; prepare its scalar request without beginning close on an active child. A successful metrics transition only installs that scalar owner. Child cancellation/retirement advances later through the existing bounded lane driver. A refused candidate cannot cancel the prior child.
6. Mailbox capacity remains 128, including existing in-flight reservations. Replacing a prior window-metrics Resize may only replace that exact scalar variant; do not evict/drop an unrelated generic RuntimeApply merely because a string key matches. Any displaced owner requiring retirement must already have an exact retained receiver.
7. The existing Watchdog covers validation, receiver acquisition, actual candidate construction and prepared writes. Finish returns the sole callback-owned verdict; no second clock/watchdog or global identity lookup. Expensive initialization, owner release, callbacks and wake work cannot move after finish. Final visibility consists only of previously admitted, fixed scalar metadata under the still-exclusive receivers. Tests must state that exact linearization scope: a finish-gated publication proves fault refusal, not a mathematical timing guarantee about unmeasured post-finish instructions or irreversible GPU effects. If the concrete receiver cannot provide this bounded final transition, it remains a blocking implementation prerequisite, not an assumed constant-time commit.
8. Keep the exact source and every reserved receiver outside callback/unwind closures. Failure releases only empty reservations; owned payload retirement remains explicit and byte-granted. Existing EventQueue::close_step currently pops/drops a whole String, so it cannot be cited as the retained rejected-payload close proof.

## Platform Admission Limit

The browser already has an acknowledged transport owner and can stop polling while the candidate is retained. Native winit delivers an external stream without that backpressure API. A fixed pending field cannot honestly promise lossless buffering of arbitrarily many new native events while a queue is full. The native bridge must reserve before producing an owned normalized event, retain any already-produced refused event, and expose terminal/backpressure failure rather than silently report acceptance or allocate an unbounded spill queue. This declaration does not claim an implementation of lossless external-device buffering. The exact bridge admission/fault behavior is part of the required caller review before production mounting.

## Schema and Independent Oracle

The fixture pins real trace boundaries 7999 / 8000 / 8001 microseconds, missing/backward clock, zero/cancel/full receivers, poison, closed surface, and u64 generation exhaustion. Event refusal commits zero sinks; accepted event commits EventQueue only. Metrics refusal commits zero of three sinks; acceptance commits all three. UTF-8 payload bytes are independently derived with Node Buffer; strict Ajv and Node BigInt provide schema/range/arithmetic oracles. Native laws must obtain CallbackVerdict from actual Watchdog::finish with the existing controlled test clock; a fabricated boolean verdict or a duplicated timing predicate is not authority proof.

## Required Native REDs and Follow-Up

- Actual EventQueue: `input_admission_refusal_preserves_exact_event_and_generation`, `input_admission_checked_generation_never_wraps`, `input_admission_cancel_and_zero_preserve_reserved_source`, `input_admission_physical_capacity_is_not_logical_length`.
- Actual WindowDelegate bridge: `browser_input_refusal_preserves_transport_owner_without_ack`, `native_input_refusal_retains_the_original_normalized_source`.
- Actual WGPU mounted helper: `mounted_event_verdict_7999_8000_8001_gates_publication`, `mounted_metrics_verdict_gates_all_three_receivers`.
- Actual RuntimeCompletionQueue/surface lane: `metrics_receiver_busy_full_poison_preserves_all_sources`, `metrics_refusal_does_not_cancel_active_surface_child`, and a held-real-mutex law.
- Cancellation/unwind after each candidate phase, including inside candidate construction, must demonstrate actual exact owners and empty-reservation release.

These are proposed selectors, not mounted or executed tests. UI-host currently has Cargo metadata but no canonical task script/registration in that directory; request the taxonomy-owned test/source/check target and launch registration before execution. Existing WGPU router is `@semio-tech/framework-renderer-wgpu` at its actual `📜️script.ts`; its heavy graph remains held until source review/coordination. No raw Cargo bypass, new cache or quota/timeout/thread override is proposed.

## Concrete Signature Revision

The companion `📓️wgpu-retained-input-signature-proposal-2026-08-28.md` supersedes any mutable-root escape: public producers receive only a narrow borrowed writer facade with no DerefMut/raw mutable owner or String access. Persistent reservations are exact slot/epoch identities, never live guards; actual guards are reacquired/revalidated only during the bounded prepared-commit turn. That private affine turn owns and finishes its actual Watchdog; no publication API accepts a supplied/replayed CallbackVerdict.

## Oracle Source Prepared, Not Executed

The same domain's `📜️script.ts` now exports `testInputAdmissionFixture()` for the forthcoming canonical source target. Its expanded packet checks 22 expected rows, UTF-8 bytes, exact u64 increments, seven schema hostiles and zero/one/seven logical close frontiers over retained 64-byte backing using strict Ajv/Node Buffer/BigInt. No command registration or execution is claimed yet. The schema's decimal-string pattern rejects u64 maximum-plus-one rather than merely checking string length.

## Open / Excluded

Queue backing construction and any new pending metadata need explicit physical admission; cold preallocation is not callback proof. Native upstream overflow behavior and bounded rejected-String retirement are required parts of this packet, not silently deferred compatibility behavior. Redraw's already-issued GPU/platform effects, snapshot Arc/Mutex publication, and the two host-shard guards remain separate open obligations.
