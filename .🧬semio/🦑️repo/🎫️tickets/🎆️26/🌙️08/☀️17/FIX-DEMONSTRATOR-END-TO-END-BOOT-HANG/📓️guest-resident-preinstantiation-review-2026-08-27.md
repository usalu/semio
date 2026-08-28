# Guest Resident Pre-instantiation Review

The complete runtime proposal `native-resident-initialization-boundary-2026-08-27.md` and the current generated `loadActor` / `createActorApi` paths were read. The proposed one required composition record on the existing canonical paged poll input is compatible with the reserved transport ownership, but it is configuration, not a guest allocation permit. No schema, WIT or generated output was changed by this review.

## Current Executed-Source Gap

The current generated worker calls `bridge.createActorApi(actorId, activationGeneration)`. That bridge imports an activation-specific component URL, whose current auto-instantiating JCO module can allocate Wasm memory during import. Therefore admission must precede that import, not merely the first reactor.poll. The separately demonstrated explicit-instantiation factory has not been mounted into this producer. Neither actors.delete nor an empty map proves the ESM-cached component memory was released.

The actual original worker/activation must retain a privately admitted guest-domain owner before creation and on constructor/import faults. Its declared compiled static/data/stack envelope and dynamic allocation authority must come from the actual artifact/host domain; no verified source of that allocation envelope is mounted here yet. The new 32-MiB JavaScript raw/UI/metadata/scratch ledger is not a guest memory permit, and native per-turn Budget.memoryBytes is not an affine domain witness.

## Canonical Mapping Requirements

Only the existing canonical poll input may acquire the agreed required composition field. The worker captures its exact configuration from the original admitted guest owner before instantiation, freezes that fixed value, and forwards it on original activation cleanup after routing or operation revocation. InputAck, cancel and retiredAck must not resolve a replacement actor or synthesize a new capacity from current defaults. Equal numeric fields across two activations do not establish the same owner.

Initialization must complete through the same captured retained owner before describe, jobs, checkpoint, restore or other producer calls. Current module imports and descriptor initialization cannot bypass that ordering. The owned allocator/whole-JSON bootstrap remains a separate prior-admission gap; adding a later field cannot fix it. Invalid or changed configuration must preserve the original pending roots through the fixed refusal protocol, not return empty success.

## Scope Of Agreement

The single-field direction is accepted for coordinated schema-first work. Actual field naming and canonical vectors remain runtime/Dag-owned. This lane reserves mapping only after that concrete release, and will join explicit factory / pre-instantiation capture in one ABI without a compatibility decoder. Host admission refusal before the actual constructor, two exact activation owners, partial-constructor faults, stale cleanup and unchanged configuration require executed tests. No guest memory, fresh Wasm, browser or all-app readiness is claimed.
