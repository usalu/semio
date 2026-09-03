# Native ProgramBridge Attachment Implementation Audit

## Scope and result

This is a current-tree, source-only audit of the native WGPU document mount
path. It rechecked the previous attachment audit, document-open-plan,
SocketGrant, bootstrap, local-bootstrap, authoritative-open-plan reports, and
the umbrella acceptance matrix. No production or test source was changed; no
build, test, executable, socket, or UI runtime probe was run.

`DocumentOpenPlanV1`, `SocketGrantV1`, `OpenableDocumentCatalog`,
`NativeDocumentMountGrantV1`, and `NativeDocumentMountSupervisor` do not occur
in the current Rust/TypeScript/JSON sources. The remote authority dependencies
are absent, not merely unconnected.

## Decision

The normal native open remains deterministically blocked at the retired
`ProgramBridgeEntry::attach_backbone`. It must not be repaired by reviving
`ChannelBackbone`, `HostBackboneChannel`, global URI registration,
empty-command polling, a generic blob route, or a caller-provided
actor/token/schema path.

The smallest safe Sol packet now is **N1: a local, fail-closed structural mount
supervisor contract**. It defines the mount identity, ownership state machine,
bounded Event/Effect framing, cancellation/progress/status projection, fixture,
and independent oracle. It exposes an internal sealed admission boundary but
does not obtain an admission, does not start a hub actor, and does not alter an
open request into a remotely mounted program. Until plan, grant, and catalog
dependencies exist, the ordinary remote route remains `mount-unavailable`.

This lets native code acquire correct ownership and stale-generation laws
without accepting the current relay's document/schema/app/surface data or
`PersistenceBinding::Hub.token` as authority. The later packet connects a
verified open plan and socket grant to this supervisor.

## Current source evidence

| Boundary | Current source evidence | Finding |
| --- | --- | --- |
| WGPU relay/open | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:421-475,3510-3583,3918-3943` | `os.open-artifact` accepts caller document/schema/app/surface facts, derives `actor://{documentId}`, opens an actor, then calls the retired attach stub. The normal error path leaves the newly opened actor unowned. |
| Legacy native binding | Same Shell source `:353-365,3025-3059,3561-3583` | The default hub binding still puts a raw session token in `PersistenceBinding::Hub`; `current_shell_actor` is client-derived. Detach remains based on `ShellSyncChannel` and a bare document id. |
| Retired bridge | `…/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:274-295,417-431,516-572` | `attach_backbone`, `detach_backbone`, and `ephemeral_snapshot` intentionally return channel-v12 retirement errors. `destroy_app` starts a kernel close but does not expose/retain completion to a mount owner. |
| Kernel transport seam | `…/ProgramBridge/…/🦀️.rs:95-102,308-335`; `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:5197-5365,7118-7185,7600-7645`; `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:538-544,987-1000,1036-1124` | `KernelClient::exchange_events` can submit `Event`s and outcomes retain non-shell `Effect`s. `Event::Message`/`Effect::SendMessage` with `MessageEndpoint::Backbone` are the correct async vocabulary. Kernel closes have a retained asynchronous handle, but bridge/shell do not own it. |
| Guest handling | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1496-1503,1979-2056,2348-2386,2529-2577` | WIT conversions preserve message endpoints, but the guest recognizes only a shell typed-result acknowledgement. Generic backbone messages are ignored; there is no guest `EffectBackboneV1`. |
| Effects prototype | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs:631-705,1122-1154` | `BackboneRegistry` is a process-global URI map whose registration overwrites. It is not window/plan scoped and cannot be the native mount authority. |
| Actor lifecycle | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:90-145,470-535,1003-1187,1450-1520,2307-2444` | `ArtifactHost` starts a bounded actor and retains runner close state by generation, but its active map key is only `String document_id`; `open` replaces that id. Its config takes caller actor and arbitrary bindings. Mailbox bounds are 64 items / 1 MiB. |
| P2-C actor | Same store source `:1720-2105` | It owns bootstrap verification, cancellation, requeue, outbox, reconnect, progress, rebootstrap failure, and atomic store replacement. The shell must not reproduce this state. Its public event still carries whole `pack`/`spr` vectors, unsuitable for an unbounded guest message. |
| Shell event pump | Shell source `:3067-3253` | `pump_sync_events` directly calls guest `ApplyEnvelopes`/`LoadDocument`, ignores broadcast `Lagged`, filters roster presentation by `presence_surface`, and drops `CommandOutcome`. `queue_host_effects` drops unmatched effects, including backbone messages. |
| Authority prerequisites | `📓️terra-document-open-plan-implementation-audit.md`; `📓️terra-authenticated-socket-grant-implementation-audit.md` | No server-issued plan/grant/catalog exists. Current Hello still carries client actor/token/schema/hash; native persistence still carries its raw token. |

The existing `touchArtifact` helper closes the actor and destroys its temporary
instance when attach fails (`Shell …/🦀️.rs:3448-3458`). The normal
`attach_sync_backbone` and `open_document` paths do neither after their
fallible attachment call. That is direct source evidence for the ordinary-open
rollback gap; it is not a runtime claim.

## N1 contract and identity

Define a repository-owned language-neutral contract before Rust or TypeScript
bindings. It is a local supervision contract, not a remote grant and not a
replacement bearer capability:

```text
semio.native.document-mount/v1

NativeDocumentMountKeyV1 {
  scope: { spaceId, documentId },
  planId, planGeneration,
  componentInstanceId,
  localGeneration
}

NativeDocumentMountFrameV1 {
  version, key, sequence,
  direction: hostToGuest | guestToHost,
  kind: attached | bootstrapProgress | snapshotBegin | snapshotChunk |
        snapshotDone | mutations | presence | disposition |
        rebootstrapRequired | closing | closed | failed,
  payload
}

NativeDocumentMountStatusV1 {
  key, phase, statusKey, progress?, canCancel, canRetry, disposition?
}
```

`planId` is opaque and never a receipt or bearer. N1 provides no public
constructor that turns a URI, relay payload, plugin id, app id, actor, hub URL,
or token into this key. A private sealed `VerifiedMountAdmission` is the only
input allowed to enter `Opening`; the real implementation arrives later from a
verified `DocumentOpenPlanV1` plus consumed `SocketGrantV1`. Fixture-only
constructors are test-only.

Bounds: each scope component and opaque id is at most 256 UTF-8 bytes; a frame
is at most 16 KiB; the supervisor admits at most 64 queued frames and 1 MiB
total queued frame bytes per mount; sequences are nonzero `u64` and strictly
contiguous per direction; snapshot frames contain at most 4 KiB. A snapshot is
`snapshotBegin`/ordered chunks/`snapshotDone` with existing P2-C public
identity, lengths, and hashes—not an unbounded
`SnapshotReplaced { Vec<u8>, Vec<u8> }` message. P2-C/P4-B retain ownership of
the 64 MiB pair and 15-second bootstrap limits.

## Ownership state machine

One `NativeDocumentMountSupervisor` is owned by one native shell window. It
owns the only active mount route, child cancellation root, bounded guest-effect
ingress, lower actor event subscription, kernel close handle, actor terminal
receipt, and chrome projection. It is not static and is not a second store.

```text
Vacant
  -> Admitting -> Opening -> Bootstrapping -> Live
  -> Retiring -> Closed
  -> Failed
```

`Admitting` is reachable only from the sealed verified admission. N1 has no
such production admission and reaches `Failed(mount-unavailable)` without
opening an actor. `Opening` allocates the local generation and cancellation/
close resources before any guest event. `Bootstrapping` means the existing
`ArtifactHost` P2-C actor owns pair verification and tail catch-up; the guest
is read-only until it receives a committed snapshot and the actor reaches the
required frontier. `Live` is the only write-admissible state.

Every local action, actor event, guest effect, kernel outcome, close receipt,
and status observation carries the full key. A mismatched or stale key is
rejected before touching actor/store/guest state. The actor map must ultimately
be keyed by structural `DocumentScope + planId + planGeneration`, never bare
document id. `componentInstanceId` and `localGeneration` distinguish re-created
guest instances without widening document authority.

Replacement `A -> B`, window close, expiry, revocation, and cancellation follow
one law:

1. Mark A unroutable and increment its local generation before starting B;
   immediately reject A effects/events.
2. Cancel A's child scope; abort P2-C transfer/reconnect work; request a
   bounded close/checkpoint only if the verified plan authorizes it.
3. Retain lower-runner and kernel-close handles until terminal receipts. A close
   timeout may leave them retained for cleanup but can never regain B's route.
4. Clear A status/progress/presence projection, then admit B only after the
   hand-off commits. Failure after actor/app creation runs the same close path;
   no detached actor or kernel instance is abandoned.

The existing `ArtifactHost` retained close runner is useful evidence for step
3. The current `ProgramBridgeEntry::destroy_app()` is insufficient because it
hides the `KernelCloseHandle`; N1 must expose a retained, pollable bridge close
owner rather than call fire-and-forget destroy.

## Event/effect transport and no-legacy boundary

The supervisor is the sole translator:

```text
actor -> supervisor -> Event::Message {
  source: Backbone { uri: private mount address }, payload: MountFrameV1
} -> exact guest instance

guest -> Effect::SendMessage {
  target: Backbone { uri: same private mount address }, payload: MountFrameV1
} -> supervisor -> exact actor
```

The address is only a local routing label. Authorization is the verified key on
every frame, never possession of a globally registered URI. `queue_host_effects`
must hand a matching backbone effect to the supervisor and return a typed
accepted/rejected disposition; it may not silently discard it. The supervisor
must never use the host-effects `BackboneRegistry`, generic topic fanout,
`ChannelBackbone`, `exchange(instance, [])`, or shell-direct `ApplyEnvelopes` /
`LoadDocument` delivery.

Extend the guest reactor to recognize only a frame from the matching backbone
endpoint and emit bounded `Effect::SendMessage` frames. Generic message data
remains denied. Incoming mutations, presence, dispositions, rebootstrap, and
snapshot stream apply through that guest effect backbone; the shell projects
chrome only. The guest cannot originate document authority, actor identity, raw
token, checkpoint selection, or snapshot integrity facts.

N1 removes all new callers of `attach_backbone`, `detach_backbone`, and
`ephemeral_snapshot`; it adds no adapter around them. The final remote cutover
deletes `ShellSyncChannel`, direct `pump_sync_events` document writes, and the
retired bridge methods together. Existing remote/manual routes remain
fail-closed until then rather than becoming a legacy transport.

## N1 source ownership

1. Add neutral schema and trace fixture under:

   ```text
   🧰️framework/🛍️products/💻️os/🧫️fixtures/🖥️native-document-mount/🧬️schema/🔣️.json
   🧰️framework/🛍️products/💻️os/🧫️fixtures/🖥️native-document-mount/🔣️.json
   ```

2. Add supervisor types, state-transition reducer, cancellation root, and close
   receipts in a new module beside
   `…/Shell/🎯️targets/🧊️wgpu/🦀️.rs`, then wire it privately from that shell.
   Do not retain independent `sync_channel`/status/presence fields in
   `ShellState` for the mounted path.
3. Add event injection, effect extraction, and a retained close-handle API in
   `…/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` and
   `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`. Reuse
   `KernelClient::exchange_events`; do not add a synchronous guest call.
4. Add guest frame parsing and per-instance `EffectBackboneV1` semantics in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs`.
5. Defer live actor adaptation: change the `ArtifactHost` active map/config to
   use the structural mount key and convert actor events to bounded frames only
   in the later authority-backed packet in
   `…/🏪️store/🔄️sync/🦀️.rs`.

## Focused laws, progress, cancellation, and accessible chrome

The supervisor cancels before admission work, before every actor/guest turn,
on every bootstrap chunk, and during closing. It reports monotonic stages
`admitting`, `opening`, `recovering(bytes/chunks)`, `live`, `closing`, and a
bounded terminal code. Lag from the actor broadcast is an integrity gap:
disable writes, invalidate the generation, re-enter P2-C rebootstrap, and do
not retain the current `TryRecvError::Lagged(_) => continue` behavior.

Presence is structural-document scoped. Same document ids in distinct spaces
never share a mount, actor, subscriber, queue, or roster. A selected surface is
non-authoritative peer telemetry only; it must not filter the document-wide
roster or confer access. `CommandOutcome` must surface a bounded disposition
instead of disappearing.

`NativeDocumentMountStatusV1` needs semantic localized label keys for opening,
catalog check, recovering with bytes/chunks, live, read-only, pending,
reconnecting, reauthorization required, conflict, cancelled, and failed. EN
and DE translations are both required, with no implicit/default language. A
valid-phase Cancel/Retry control is keyboard-operable; screen-reader status
announcements coalesce progress rather than announce every chunk. Current WGPU
plain-English `sync_status_label`/`sync_pill_text` is not adequate evidence for
this requirement.

## Neutral fixture and independent oracle

The neutral fixture contains a valid local-supervisor trace plus stale
generation, same-document/different-space, replacement while bootstrapping,
window close, admission failure before actor start, failure after actor/app
start, cancel, mailbox max-plus-one, malformed/oversized/reordered frame,
broadcast lag, rebootstrap, expiry/revoke, read-only mutation denial,
document-wide presence, terminal close, and EN/DE status/control keys.

Rust unit tests in the supervisor/bridge/guest reactor consume the fixture and
assert ownership, bounded queue, stale rejection, rollback, and terminal-close
laws. An independent TypeScript state-machine oracle—proposed beside
`…/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️opening.test.ts`—validates the
same JSON Schema and trace without importing Rust codecs or supervisor code.
It independently checks frame order/limits and status localization presence.
A separately compiled minimal WASI test component that implements
`EffectBackboneV1` is the later guest/host byte-trace oracle; a Rust
`ChannelBackbone` actor test is not a native-component transport proof.

No command was run by this audit. After tests and launch registrations exist,
the registered native kernel/WGPU launch targets—not an ad hoc script—are the
appropriate execution route.

## Later authoritative integration

N2 begins only after SocketGrant S1/S2 derives server actor/scope and removes
bearer/actor material from WebSocket Hello; N3 requires a published immutable
verified `OpenableDocumentCatalog`; N4 requires `DocumentOpenPlanV1` plan issue
and one-use plan-to-socket-grant exchange. The native adapter then:

1. Sends only structural `{ scope, requestedSurfaceId?, clientInstanceId }` to
   the protected open-plan command.
2. Validates the returned exact plan package/component/surface/catalog/
   descriptor/checkpoint facts locally.
3. Exchanges the opaque receipt once for a plan-bound socket grant and sends a
   credential-free `HelloVNext`.
4. Builds `VerifiedMountAdmission` solely from server-selected plan/grant
   identity, opens an `ArtifactHost` with a structural mount key, and starts the
   supervisor.
5. Maps P2-C control, public verified pair, and required tail to bounded mount
   frames, with a fresh plan/grant after rebootstrap.
6. Connects session/share revoke, membership/delete, catalog restart, plan
   expiry, and checkpoint release to the same cancellation/terminal-close path.

At cutover delete rather than support the legacy channel: caller-selected
`OpenArtifactRelayTarget` document/schema/plugin/app/actor authority, raw
`PersistenceBinding::Hub.token`, `current_shell_actor` transport identity,
token/actor-bearing Hello, `ShellSyncChannel`, direct shell snapshot/mutation
pump, query/polling attachment, and `BackboneRegistry` authority. Explicit
local folder/file opening may later use a separately labelled trusted local
plan; it is not a fallback for remote plans.

## Blocker order

1. **N0:** retired attachment makes every ordinary native remote open fail;
   normal attach failure also leaks its just-opened actor.
2. **N1:** land the local sealed supervisor, message/effect framing, retained
   close ownership, fixture, Rust laws, and independent TypeScript oracle—no
   remotely usable admission yet.
3. **N2:** SocketGrant server-derived subject/actor and all-client removal of
   bearer/actor carriers.
4. **N3:** immutable verified openable catalog and server-issued one-use open
   plan; existing document creation must also become server-derived.
5. **N4:** plan/grant-backed mount adapter, structural actor keys, P2-C
   rebootstrap/tail composition, EN/DE accessible chrome, and no-legacy
   cutover.
6. **N5:** real two-user native/window proof including replacement, outage,
   lag/rebootstrap, revoke, close, document-wide presence, and teardown.

The current acceptance state remains `BLOCKED` for Native OS. This audit
defines a safe landing order only; it makes no build, test, mount, socket, or
end-to-end success claim.
