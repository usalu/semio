# Guest Lifecycle Wire Contract

## Concrete Shared Types

The following is the agreed implementation target, not a claim that guest execution is mounted. Existing actor names for `ActorInstanceLifetime` and `ActorInstanceCloseRequest` remain the domain names. `ActorInstanceLifecycleWire` replaces the earlier close-only wire without an alias or old-format decoder.

```rust
struct ActorInstanceLifetime {
    activation_generation: u64,
    instance_id: u32,
    guest_lifetime: u64,
}
struct ActorInstanceOpenRequest {
    activation_generation: u64,
    instance_id: u32,
    request_sequence: u64,
}
struct ActorInstanceCloseRequest {
    lifetime: ActorInstanceLifetime,
    request_sequence: u64,
}
enum ActorInstanceLifecycleReceipt {
    Captured { lifetime: ActorInstanceLifetime, request_sequence: u64 },
    Accepted { lifetime: ActorInstanceLifetime, request_sequence: u64, close_generation: u64 },
    Retired { lifetime: ActorInstanceLifetime, request_sequence: u64, close_generation: u64 },
}
struct ActorInstanceLifecycleAck { receipt: ActorInstanceLifecycleReceipt }
enum ActorInstanceLifecycleWire {
    Open(ActorInstanceOpenRequest),
    Close(ActorInstanceCloseRequest),
    Receipt(ActorInstanceLifecycleReceipt),
    Ack(ActorInstanceLifecycleAck),
}
```

All fields are public. Rust derives Copy/Eq and schema-aligned serde. JS uses camelCase fields, `bigint` for activation/guest/close generations, and safe positive `number` for requestSequence. Instance IDs are unsigned 32-bit including zero. Generations are nonzero unsigned 64-bit. Request sequence is `1..9007199254740991`. JSON fixtures encode generation fields as canonical decimal strings. A Captured receipt's sequence correlates the open request; Accepted/Retired correlate the close request. No nullable or zero substitute for a not-yet-created generation.

JS types mirror the tagged values: open has `{kind:"open", activationGeneration, instanceId, requestSequence}`; close has `{kind:"close", lifetime, requestSequence}`; the three receipt variants are flat `{kind:"captured"|"accepted"|"retired", ...}`. `ActorInstanceLifecycleAck` is `{kind:"ack", receipt}`. `ActorInstanceLifecycleReceipt` is their receipt union. The previous two-variant receipt type is replaced, not aliased. TS functions are `encodeActorInstanceLifecycle` and `decodeActorInstanceLifecycle`; the shared constant is `ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES` (44).

## Fixed Binary Layout

One unsigned-byte tag followed by canonical unsigned LEB128 fields. Maximum envelope: 44 bytes, including the largest safe request sequence.

| Tag | Value | Ordered Fields |
| --- | --- | --- |
| 0 | Open | activationGeneration, instanceId, requestSequence |
| 1 | Captured | activationGeneration, instanceId, guestLifetime, requestSequence |
| 2 | Close | activationGeneration, instanceId, guestLifetime, requestSequence |
| 3 | Accepted | activationGeneration, instanceId, guestLifetime, requestSequence, closeGeneration |
| 4 | Retired | activationGeneration, instanceId, guestLifetime, requestSequence, closeGeneration |
| 5 | ACK Captured | activationGeneration, instanceId, guestLifetime, requestSequence |
| 6 | ACK Accepted | activationGeneration, instanceId, guestLifetime, requestSequence, closeGeneration |
| 7 | ACK Retired | activationGeneration, instanceId, guestLifetime, requestSequence, closeGeneration |

Encoding rejects invalid authority before changing output. Decoding rejects noncanonical, overflow, truncated, trailing, and unknown-tag input. Receipt identity includes the exact tag, full lifetime, request sequence, and close generation where present. A Retired receipt cannot establish a missing Accepted receipt. Same-activation replacement changes guestLifetime, not just numeric instance.

## Existing Poll ABI Join

WIT owns a new `instance-lifetime` interface with `lifetime`, `open-request`, `close-request`, `captured-receipt`, `close-receipt`, and `receipt { captured(...), accepted(...), retired(...) }`. These are fixed records only.

- `events.instance-open-event` adds `activation-generation: u64` and `request-sequence: u64` beside its existing instance ID and app initialization fields.
- `events.instance-close-event` is replaced by `instance-close(close-request)`; no numeric-only close path remains in the canonical event.
- `events.event` adds `instance-lifecycle-ack(receipt)`.
- `reactor.turn-result` adds `lifecycle-receipt: option<receipt>`.
- Kernel `Event` and `TurnResult` use the canonical actor Rust types, not a duplicated authority record. Native and WIT reducers share the same aggregate.

One pending receipt per aggregate remains retained until exact ACK. One selected receipt per turn is returned; no delivery bit releases it. Closing before the host receives Captured must wait for the exact open result, not recapture by numeric ID. The host must not expose a ready app handle before accepting and acknowledging Captured. Host worker/activation identity is retained independently and checked before any receipt enters the client.

## Guest Ownership

The runtime pre-admits lifecycle/outbox capacity, issues its checked guestLifetime, and captures the exact native cell immediately after open. Native close accepts only the stored lease. Aggregate descendant admission must succeed before detach. Captured ACK enables the live slot; Accepted ACK permits terminal delivery; Retired ACK releases the completed aggregate. Missing/foreign ACK does not release any owner. No general Idle/quarantine-empty result is terminal evidence.

Implementation ownership: this lane owns shared JSON schema/fixture, Rust actor codec, Kernel/WIT records and Rust reactor aggregate. Demonstrator owns TS codec and generated JS/ShardClient/PluginRuntime forwarding. Neither side should mount a partial producer based on an invented callback.

## Test-First Checkpoint

The shared schema and nine vectors were authored before the replacement codec. The three native `actor_instance_lifecycle_wire_` laws produced the expected compile RED (32 missing-type/field errors, no tests executed) in `🧪️member-actor-lifecycle-red-r1-native-2026-08-27.txt`. The replacement codec is now mounted. Canonical native R2 passed **3/3, 97 skipped, 0.065s** (`🧪️member-actor-lifecycle-green-r2-native-2026-08-27.txt`, also preserved in `📓️actor-lifecycle-green-r2-native-2026-08-27.md`). This includes strict decimal serde roundtrip and same-activation guest-lifetime identity. The coordinator separately reports the TS owner's three independent LEB128/Ajv laws passing; that run was not launched by this lane.

This is codec evidence only. Kernel/WIT records, outer actor TurnResult forwarding, native aggregate and browser host ownership are not yet mounted by this checkpoint. The unrelated production Interaction leaf gate remains compile-blocked (R4: 89 upstream/test errors, no selected test executed); it is not a prerequisite pass or guest lifecycle acceptance.

## Outer Turn and WIT Checkpoint

The preceding paragraph is the earlier codec-only boundary. Kernel and WIT now carry the canonical open request, direct close request, direct receipt ACK and optional typed turn receipt. Native/WIT conversions preserve all fields. The independent numeric `close_instances` argument is being removed; guest admission must derive closing state only from the captured exact lease.

Actor R4 executed all five lifecycle tests: 5 passed, 97 skipped, 0.050 seconds, canonical exit 0. Evidence: `📓️actor-lifecycle-all-green-r4-native-2026-08-27.md` and `🧪️member-actor-lifecycle-all-green-r4-native-2026-08-27.txt`. This adds two actual outer-turn codec laws to the three fixed-body laws. It does not prove reactor descendant retirement or browser completion.

The newly authored outer vectors initially miscounted the existing usage fields. Direct source inspection corrected them to three fixed little-endian u64 counters (24 bytes), without changing the existing outer ABI. Receipt length remains canonical one-byte ULEB for 0..44, with the body capped at 44 bytes. Empty outer fixture: 30 bytes; maximum-receipt outer fixture: 74 bytes. Invalid authority leaves the caller's encoder output untouched; decoder refuses nonreceipt tags, oversized bodies and noncanonical multibyte receipt lengths.

Current remaining ownership work: pre-open structural retention, exact native app plus reactor/UI participant aggregation, preadmission before detach, staged ACK commit after a valid strict-under-8000us verdict, fault/unwind recovery and real consumer mount. Receipt absence or Idle is not a terminal witness.
