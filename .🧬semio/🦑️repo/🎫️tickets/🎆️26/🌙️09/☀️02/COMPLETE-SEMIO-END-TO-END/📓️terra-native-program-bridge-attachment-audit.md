# Native `ProgramBridge` attachment audit

**Scope.** Read-only source audit on 2026-09-03 for the native WGPU document-open path. No source, test, configuration, or build output was changed; no build or test was run. This is deliberately an implementation packet, not evidence that the proposed path works.

## Decision and first deterministic blocker

**Blocker N0 — critical, deterministic.** Every native `ShellState::open_document` that reaches a loaded WASM `ProgramBridgeEntry` fails before it can adopt its sync channel. It opens an `ArtifactHost` actor, then calls `plugin.attach_backbone`; that method unconditionally returns the channel-v12 retirement error. The live `os.open-artifact` relay reaches exactly this method. The equivalent manual `attach_sync_backbone` path fails in the same way.

| Link in the actual path | Current result | Evidence |
| --- | --- | --- |
| Native shell opening relay | accepts caller `documentId`, `schema`, optional space/app identity, computes local bindings, calls `open_document` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:417-475,3918-3943` |
| Native open | checkpoints/detaches, derives `actor://{documentId}` and a client-side actor string, opens/subscribes `ArtifactHost`, then invokes retired attach | same file `3561-3583` |
| Direct failure | `attach_backbone` returns `Err("retired in channel v12 … no EffectBackbone replacement")` | `…/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:274-295,516-535` |
| Visible consequence | `sync_channel`, status, history refresh, and UI adoption occur only after that `?`, hence are unreachable | Shell `3573-3583` |
| Manual sync-card path | repeats open/subscribe/retired attach | Shell `3504-3542` |

There is a second high-severity lifecycle fault beneath N0: both opening paths have started the actor before their fallible attach, but do not close it on the error path. The one-off `touchArtifact` helper does close and destroy on precisely that failure (`3448-3458`), demonstrating the missing rollback in the normal path. Therefore a failed normal open can leave an unowned actor until another same bare document id replaces it or process teardown occurs. Do not patch this by making the stub a no-op.

## Why the old attachment was correctly retired

The old design depended on a synchronous, process-global `HostBackboneChannel`/`AppCommand::AttachBackbone` protocol. Channel v12 instead has an async kernel turn: host-to-guest data is `Event::Message` and guest-to-host data is `Effect::SendMessage`. The native WASM guest now executes through `KernelClient` on its kernel thread, so the former in-process registry cannot safely identify a pooled, multi-instance guest. The comments in the native bridge and plugin SDK say exactly this; the plugin initializer deliberately leaves backbone-dependent guest code with a real “no host backbone linked” failure rather than silently operating without sync.

* `MessageEndpoint::Backbone { uri }` is the stable kernel vocabulary, not a ready transport: `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:982-1000`.
* The guest reactor only reacts to a shell acknowledgement; generic `Event::Message` is currently ignored: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1496-1503`.
* The store's `BackboneChannelPorts` is intentionally uninhabited; no cross-WASI port exists: `…/🏪️store/🦀️.rs:16897-16936`. `ChannelBackbone::pair` is a same-process crossed queue for Rust stores and an actor, not a guest transport: `16994-17065`.
* The plugin-host effects module has a useful prototype `BackboneRegistry`, capability check, and message vocabulary, but it is a process-global URI map whose `register` overwrites and it lacks per-mount lifecycle integration. It cannot be reused as the native authority: `…/🔌️plugin/🖥️host/⚡️effects/🦀️.rs:630-705,1123-1154`.

So the correct repair is a replacement, per-open-plan effect/event mount protocol. It must not restore `attach_backbone`, a global URI registry, polling `exchange(id, [])`, or an adapter that pretends the old `ChannelBackbone` crosses the guest boundary.

## Existing production seams and limits

| Concern | Reusable production seam | Gap / implication |
| --- | --- | --- |
| Plugin UI and renderer | `ProgramBridgeEntry` owns manifest/package path and uses `KernelClient::create_app`; normal actions, commands, `LoadDocument`, mutation application and rendering are async turn calls | It can host an event-driven mount, but `attach_backbone`, detach and ephemeral polling are hard error stubs. `ProgramBridge` `371-550` |
| Lower actor, outbox, reconnect | `ArtifactHost` starts a bounded-mailbox actor, gives an event receiver, retains closing runners by generation | Current key is **bare `document_id`**, and `open` closes an existing same id. It cannot distinguish same id in two spaces/plans. `…/🏪️store/🔄️sync/🦀️.rs:1013-1187` |
| P2-C payload safety | Native actor validates registered codec/schema/frontier, bounds and reports artifact bootstrap, persists it, inserts a snapshot into its store, and rejects private DB snapshot frames; rebootstrap/control causes failure/requeue | Reuse this actor/payload path; do not create a shell copy of its P2-C state. `sync/🦀️.rs:525-542,1743-1754,1872-1960,2000-2090` |
| Current native event fan-out | `pump_sync_events` forwards mutations and snapshot bytes directly into guest app commands; status/progress/presence become shell fields | It duplicates the eventual mount data route and silently ignores broadcast `Lagged`; replace it with generation-tagged mount delivery where loss forces rebootstrap. Shell `3067-3164` |
| Effect routing | `queue_host_effects` handles UI/navigation/load/document and drops every unmatched effect | It drops `Effect::SendMessage { Backbone }`, the exact guest-to-actor route required. Shell `2504-2555` |
| Presence | Actor emits roster; shell stores it locally and filters display by `presence_surface`; heartbeat derives a client actor and relies on retired ephemeral poll | The required invariant is document-wide roster for the structural document scope. `surface` is peer telemetry only, never a visibility or authority filter. Current behaviour conflicts with that invariant. Shell `478-490,3214-3253,11655-11662` |
| Status/accessibility | There is a progress/status field and WGPU footer text | Labels explicitly are English-only plain strings; no established semantic/native accessibility, localized cancel/retry state, or CommandOutcome UI. Shell `1649-1660,3168-3212` |
| React comparison | React opens a per-document worker route before its old `plugin.attachBackbone`, then closes/unregisters it | This shows desirable scoped ownership, but is not a valid native adapter: its post-flip inbound queue has no guest consumer either. `ShellHost/🟦️.tsx:3360-3410`; `🎠️kernel/🟦️.ts:1608-1629` |

Synthetic Rust actor tests attach a local `ArtifactStore` to `channels.channel_backbone`; that validates the in-process queue, not a WASI program or `ProgramBridge` mount (`sync/🦀️.rs:5050-5060`). It must not be presented as native end-to-end proof.

## Proposed schema-first replacement

Define the following repository-owned, language-neutral schema before Rust/TS bindings. Names are proposed; the contract is the required part.

```text
semio.native.document-mount/v1
NativeDocumentMountGrantV1 {
  mountId, planId, planGeneration, authorizationGeneration, expiresAt,
  scope: { spaceId, documentId },
  descriptor: { digest, artifactKind, artifactSchema, packSchemaHash,
                checkpointId, requiredTailFrontier, catalogGeneration },
  component: { pluginId, packageSha256, appId, surfaceId },
  capabilities: { read, write, inference, approvedMutation }
}

BackboneMountFrameV1 {
  version, mountId, planGeneration, direction, sequence,
  payload, frontier?, disposition?
}
```

The hub-issued open plan (not a shell action, URL, client actor, plugin id, or mutable local manifest) supplies the grant. The native client verifies its authenticated secure-carrier binding and expiry, then verifies the exact component against the trusted native-codec/openable catalog before creating the app. `mountId` is opaque. A private `mount://{mountId}` endpoint is an address only; every ingress also checks mount id, plan generation, component instance, structural `(spaceId, documentId)`, plan capabilities and current authorization generation.

The server remains the source of actor, session and document scope. The native process must not manufacture `current_shell_actor`, `actor://{documentId}`, default hub persistence, `S_USER`, or a hub token for a remotely shared plan. The existing relay stays local-only until it is replaced by the authenticated open-plan request; its caller-supplied document/schema/app fields are not authorization.

### One authoritative live backbone per mounted plan

Create a shell-window-owned `NativeDocumentMountSupervisor`, not a static registry and not a second `ArtifactStore`:

1. It accepts exactly one verified `NativeDocumentMountGrantV1` at a time per shell window, creates a monotonic local mount generation, creates/reuses the plan-scoped lower actor, and owns its cancellation root, bounded effect ingress, event subscription and close receipt.
2. The lower `ArtifactHost` is changed to key its active map by a structural `DocumentScope + planId`/mount key, never bare `documentId`. Its actor config receives the verified grant facts rather than caller actor and free-form bindings. It exposes a mount handle with terminal-close observation, not raw `ChannelBackbone` to the shell.
3. The guest SDK receives a first `Event::Message` from `MessageEndpoint::Backbone { uri: mountUri }` describing the verified mount and implements per-instance `EffectBackboneV1`. Guest mutations/snapshots become bounded `Effect::SendMessage` frames; the supervisor serially submits them to that actor. The actor returns delivery, authoritative snapshot, remote mutation, P2-C bootstrap/rebootstrap, command disposition and presence frames as `Event::Message` to that same instance.
4. The actor remains the one owner of persistence, outbox, hub reconnect, P2-C verifier and frontier. The guest store is the one document materializer. The shell only projects host chrome (status, localized progress and roster); it never separately calls `ApplyEnvelopes`/`LoadDocument` for the same data and does not synthesize mutations.
5. `queue_host_effects` becomes a bounded asynchronous dispatch hand-off owned by the supervisor. It must admit/reject a backbone effect with a typed disposition, never drop it. A `BackboneMountFrameV1` monotonic sequence plus retained mutation identifiers makes retry safe. The guest receives rejection/receipt as an event; the lower actor decides durable acceptance.

This uses the kernel's existing `Event::Message`/`Effect::SendMessage` shape but does **not** reuse the host-effects `BackboneRegistry` as an authority. Its URI capability idea can inform a component-specific capability check inside the supervisor.

### Replacement, cancellation and loss rules

Use `Opening → Bootstrapping → Live → Closing → Closed | Failed` for each `{mountId, planGeneration, localGeneration}`. All worker output, effects and status events carry that tuple.

* To replace A with B, first make A unroutable and cancel its scope; reject A ingress immediately. Request a bounded checkpoint only where the grant still permits it, then request the actor close and wait for its terminal receipt up to a fixed deadline. Start B only after the route hand-off is committed. If A misses the deadline, retain its close owner for cleanup but never let it address B.
* Close/revoke/expiry detaches the guest route first, aborts the P2-C assembler and reconnect work, drains its fixed mailbox, and clears host projections. `ArtifactHost` already has a generation-keyed retained close runner and aborts a bootstrap on close (`sync/🦀️.rs:1156-1187,1476-1504`); expose that terminal state rather than assuming synchronous close.
* A lagged native event receiver is not harmless. Treat it as an integrity gap: disable writes, request/re-enter P2-C bootstrap, and only resume after a descriptor/frontier-valid snapshot and required tail. Do not retain the current `TryRecvError::Lagged(_) => continue` rule.
* Progress is bounded by the existing P2-C received bytes/chunks contract; cancellation applies during plan verification, catalog activation, pair transfer and replacement. Return typed, redacted failure categories (expired/revoked, plan mismatch, catalog mismatch, corrupt pair, bounded queue, bootstrap deadline) rather than URL/token/error text.

### Privacy, presence and native UX

Structural `(spaceId, documentId)` plus plan/session generation is the isolation boundary. The roster is document-wide inside that boundary: every authorized peer on that document is shown even if their `surface` differs. `surface` may be displayed as non-authoritative telemetry and must never filter the roster or grant access. Same document id in a different space must not route, deduplicate live actor state, or leak presence.

Define a `NativeDocumentMountStatusV1` host projection with localized `Label`/semantic nodes for at least opening, checking catalog, recovering with byte/chunk progress, live, read-only, pending, reconnecting, reauthorization required, conflict, cancelled and failed. Provide keyboard-operable cancel/retry only for valid phases, and screen-reader status announcements that coalesce progress. EN and DE translations are both required; there must be no implicit default locale. `CommandOutcome` and recovery must be observable, not silently discarded as today.

## Ordered Sol-sized implementation packet

| Packet | Scope and exact files to change | Gate / dependency |
| --- | --- | --- |
| N1 — mount schema and SDK semantics | Add neutral `native-document-mount/v1` schema/fixtures and generated repository bindings; implement per-instance guest `EffectBackboneV1` consuming/sending `Event::Message`/`Effect::SendMessage` in `…/🔌️plugin/🦀️.rs` and `…/🔌️plugin/⚛️reactor/🦀️.rs`. Delete rather than wrap `ProgramBridgeEntry::{attach,detach,ephemeral_snapshot}`. | Can land with deterministic fixture tests; needs no live hub, but depends on the secure ABI decision. |
| N2 — plan-scoped actor port | In `…/🏪️store/🔄️sync/🦀️.rs`, introduce verified mount config/handle, scope+plan key, bounded guest ingress/egress and terminal close receipt. Adapt P2-C output to mount frames and make event loss trigger rebootstrap. Preserve its existing codec/frontier/assembler/outbox authority. | Requires the authenticated first-frame carrier and P2-C semantics; interface may be stubbed only with a schema fixture, not free-form identities. |
| N3 — native bridge supervisor | In `…/ProgramBridge/…/🦀️.rs`, add asynchronous event injection and effect extraction for a concrete `{instance,mount,generation}`. In `…/Shell/…/🦀️.rs`, replace `ShellSyncChannel`, direct `pump_sync_events` mutation/snapshot calls and dropped backbone effects with `NativeDocumentMountSupervisor`. Add rollback around every partial open. | N1+N2. No global `HostBackboneChannel`, no raw `ChannelBackbone` exposure. |
| N4 — authoritative opener and chrome | Replace `open_document`, `attach_sync_backbone`, and `handle_open_artifact_relay` remote path with a typed hub open-plan client. Retain only explicitly local folder/file open as a separately labelled local plan. Replace client actor/default hub binding, surface filtering and English-only status with status/a11y/EN+DE projection. | Requires secure carrier, authenticated WebSocket migration, open-plan CQRS endpoint and trusted native-codec/openable catalog. |
| N5 — live proof | Add two isolated native processes/windows against the secure local hub: distinct users, same plan/doc, document-wide presence, edits, checkpoint, reconnect, forced lag/P2-C recovery, revoke and close. | Requires N4 plus P2-D artifact storage/pair access and real runtime backends. |

## Neutral fixtures, independent oracle, and focused commands

Add a language-neutral fixture containing a signed/open-plan-shaped grant, catalog/descriptor identities, baseline pair/frontier, normal frames, replacement race, wrong-space same-document id, stale generation, lag/rebootstrap, expiry/revoke, write denial and expected localized status keys. Rust schema tests validate bounded parsing and the native state machine; a TypeScript test validates the same fixture with the repository's schema validator. For an independent behavioural oracle, use a minimal separately compiled WASI test component that implements `EffectBackboneV1`; capture its message trace and compare it byte-for-byte to the fixture trace. This is a genuine guest/host boundary oracle, unlike a Rust `ChannelBackbone` test. Add a real-socket test only after the prerequisite services exist.

Focused commands to run **after** the associated tests/targets are registered (none were run for this audit):

```sh
bun x nx run @semio-tech/framework-os-kernel:test-native --skip-nx-cache -- native_document_mount
bun x nx run @semio-tech/framework-renderer-wgpu:test-native --skip-nx-cache -- native_document_mount
bun x nx run os-hub:secure-local-smoke --skip-nx-cache
```

The first two targets exist today (`…/💻️os/📦️packages/🦀️rust/📋️project.json:36-42`; `…/wgpu/📦️packages/🦀️rust/📋️project.json:24-30`) but do not yet contain this named coverage. Use the existing launch registrations rather than inventing a CLI profile: `🧪️test🗄️os-hub🔐️local-bootstrap`, `⚖️gate💻️os🎠️kernel🦀️native`, `⚖️gate📺️renderer🧊️wgpu🦀️native`, and `🛠️dev🖥️s🧊️wgpu🖥️native` in `.vscode/launch.json:2598-2606,2873-2881,2972-2979,4342-4365`.

## Exit criteria and blocker order

1. **N0 critical:** the native ordinary open cannot complete because attachment always fails.
2. **N0a high:** ordinary failure leaves an opened actor unadopted; bare document-id keys also cross space identity boundaries.
3. **N1 high:** no guest-side `EffectBackboneV1` consumes generic backbone messages, and shell currently drops outbound backbone effects.
4. **N2 high:** native must wait for secure carrier + server-issued open plan + trusted native-codec catalog; otherwise its current caller-controlled document/app/actor path is not an authority boundary.
5. **N3 high:** direct shell snapshot/mutation delivery and ignored receiver lag would duplicate/lossily bypass the authoritative P2-C actor.
6. **N4 medium:** surface-filtered roster contradicts document-wide presence; ephemeral polling is retired; status/command outcome/a11y/localization are incomplete.

Completion means a verified grant creates exactly one plan-scoped live actor and one guest store, all stale frames/effects are rejected, document-wide presence never crosses structural scope, P2-C rebootstrap is observable and cancellable, replacement reaches a terminal close boundary, and the independent guest-component plus real two-user native oracle both pass. It does not mean that a local `target/` binary or the existing synthetic channel tests are sufficient proof.
