# Canonical Actor Request Ingress Boundary

## Verified Current Sources

This is a read-only implementation map, not a completed Request/Completed feature or a boundedness certificate.

The canonical Plugin WIT already has `request-event { req, params }`, with origin, capability and pack payload, and `completed-event { req, outcome }`. Its one `reactor.poll` accepts event lists, an optional command page and a budget. The reactor currently ignores `Event::Request`; `Event::Completed` resolves the request registry from a whole payload. The host router's `run_router_effect_job` explicitly returns an unmounted-pump fault after cancellation checking. These are not working generic extension execution paths.

The existing command-page transport has a fixed 4,096-byte block, owner and generation, plus command index/count, instance, sequence, opcode, page index/count, item count and metadata. Its native definitions are in `framework/os/spr/channel/🦀️component.rs` (`FixedCommandPage`, `CommandPageCursor`, `CommandIngressStatus`). The opcode is AppCommand authority; it must not be reused as a general Request/Completed discriminator. The associated retained command owner and ArtifactOwnedToolJob factory carry app/operation authority, not extension evaluation authority.

The existing reactor jobs module provides bounded-step interfaces and raw job factories, but raw whole input/output vectors and a registered job name are not a certificate of bounded payload processing. Reusing those primitives requires an extension-owned factory, exact admitted root, progress/cancellation, bounded input/output movement and exceptional-owner retention.

## Required Integration

The existing poll transport needs a schema-owned general ingress discriminator before a Request or Completed payload can use its fixed page storage. AppCommand semantics remain with the ordinary-command owner. Request and Completed must capture their own owner/generation, exact requester and extension activation, request identity, payload type and checked page accounting; no opcode alias or second invoke ABI is acceptable.

Host capture is now implemented for requester completion and inbound host effects. It validates activation object, slot, worker incarnation and generation, rejects late publication, and cannot settle a replacement request merely because its string ID is equal. This does not validate the extension target or make UTF-8/JSON/pack conversion bounded. Current completion still copies the whole outcome and decodes frame collections; these remain explicit integration work.

All variable fields, including capability/origin and results, must be admitted before copying. Rejection, cancellation, worker replacement and closing must retain or retire the exact admitted roots; neither ordinary Drop nor an unowned array conversion is an acceptable escape. No cap increase is proposed.

## Close Authority Join

The historical instance-only guest close has now been replaced in source by the approved captured lifetime request/receipt protocol described below. The TypeScript codec, ShardClient owner, dedicated renderer scheduler and generated WIT mapping have executed focused/full JavaScript regressions. The actual current generated producer gate is **60/60**, with no fresh component publication. Native guest descendant joining remains with Dag and is not inferred from those tests.

The temporary 34-byte close-only codec and worker side-message branch have been removed, not kept as compatibility. The existing poll path carries the canonical 44-byte lifecycle messages and exact guest-issued receipts. It does not manufacture accepted/retired messages from idle status, quarantine removal, host-effect cancellation or synthetic generations. Actual PluginRuntime create/close and owned renderer mounting remain open.

## Issued UI Patch Authority Join

The runtime owner confirmed a single native patch per turn and approved `ActorUiPatchReceipt { lifetime, patchSequence: bigint }`, encoded as four canonical ULEBs with maximum 35 bytes. Optional `uiPatchReceipt` follows `lifecycleReceipt` in the outer actor frame. PatchAck and PatchRejected must carry the exact producer receipt; surface/revision alone do not distinguish reused guest lifetime/revision. Dag owns native/schema/shared vectors/Kernel/WIT and is preparing their concrete release. This task owns TypeScript codec/mapping and host producers after that release; no patch sequence will be synthesized from revision or array ordinal.

## Preservation And Evidence

All existing active-ticket targets, logs and captures remain preserved; no cleanup or shared generated-output publication was performed here. Executed JavaScript RED/GREEN, exact compiler errors and per-stage limitations are recorded in `📓️browser-proof-2026-08-27.md`. Source capture `🧪️inbound-activation-sources-1.log` includes the actor, materializer and canonical schema paths but is still a selected-source snapshot, not the entire dependency graph.

## Approved Lifecycle Contract For The Next Cutover

The coordinator confirmed this contract after the 59-test producer release. **The TypeScript codec now passes its three focused tests, including all nine neutral vectors; ShardClient and PluginRuntime mounting remain in progress.** Dag owns schema, neutral fixtures, Rust codec, Kernel/WIT records and native mounting. This task owns TypeScript codec, ShardClient, generated mapping, and PluginRuntime create/destroy/close. Ordinary command scheduling and tutorial joins remain separately owned.

`ActorInstanceLifetime` gains positive u64 `guestLifetime` alongside positive u64 `activationGeneration` and u32 `instanceId`. Open carries activation generation, instance and a positive safe-number request sequence. Guest lifetime is minted by the guest immediately after exact native capture; the host must never synthesize it.

The new TypeScript names are approved: `ActorInstanceOpenRequest`, `ActorInstanceCloseRequest`, `ActorInstanceLifecycleReceipt`, `ActorInstanceLifecycleAck`, `ActorInstanceLifecycleWire`, `encodeActorInstanceLifecycle`, `decodeActorInstanceLifecycle`, and `ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = 44`. Remove old codec aliases and update authored consumers together.

| Tag | Message | Body after tag |
| --- | --- | --- |
| 0 | Open | activation, instance, request |
| 1 | Captured | activation, instance, guest lifetime, request |
| 2 | Close | activation, instance, guest lifetime, request |
| 3 | Accepted | activation, instance, guest lifetime, request, close generation |
| 4 | Retired | activation, instance, guest lifetime, request, close generation |
| 5 | ACK Captured | exact Captured body, no nested tag |
| 6 | ACK Accepted | exact Accepted body, no nested tag |
| 7 | ACK Retired | exact Retired body, no nested tag |

ACK JSON is exactly `{kind: "ack", receipt: <Captured | Accepted | Retired>}`, with no separate request ID. All u64 fields remain BigInt in TypeScript; request sequence remains a positive safe number (at most 2^53 - 1).

WIT instance-open adds activation-generation/request-sequence; close becomes the exact captured request; `instance-lifecycle-ack` is added; `TurnResult.lifecycle-receipt` is optional and carries at most one retained receipt per turn. Host exposure waits for Captured. Retired must join native cell/pump, reactor tasks/requests/resumes/timers/metadata, patches, turn handback and render bindings, with host UI retirement still independently required. No idle/map-removal shortcut or side export is authorized.
