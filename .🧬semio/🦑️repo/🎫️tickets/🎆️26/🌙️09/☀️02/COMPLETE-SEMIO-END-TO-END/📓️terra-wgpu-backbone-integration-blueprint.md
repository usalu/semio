# WGPU Native Backbone Integration Blueprint

## Current verdict

**RED — do not describe native WGPU document opening as end-to-end.** This read-only audit found a sound-looking source-level SocketGrant actor behind a retired WGPU backbone boundary, an empty trusted native catalog, and a local identity comparison that discards the hashes the open plan was designed to carry. No build or runtime command was run for this report.

The smallest honest first vertical slice is one statically linked, descriptor-verified codec/package whose plan is selected before any document actor or WebSocket is started. It must commit a generation-qualified event bridge before enabling the actor. It must not attempt broad generated-marketplace activation.

## Current execution map

1. The hub obtains its `NativeCodecBinding` vector from `linked_native_codec_bindings()`, but that function returns `Vec::new()` at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:393-395`. Startup therefore has no real native codec binding; `open_plan_ready` is true only when a configured catalog has a target at `:5248-5283`.
2. A WGPU plugin loads a descriptor JSON next to its component, but `read_descriptor_manifest` retains only `manifest` and `package_id` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:779-807`. `ProgramBridgeEntry` has no component or descriptor hash at `:372-376`.
3. The `os.open-artifact` relay pins a raw `spaceId`, obtains the **currently mounted** session's default binding, and calls `open_document` at `…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:4058-4075`. It does not use an open plan to choose the app/window/package.
4. `open_document` derives plugin/app/window from that already-current session, prebinds the host surface, opens the `ArtifactHost` actor, subscribes, and only then calls `plugin.attach_backbone` at `…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:3686-3715`.
5. That call is terminal today: `wasm_program_exchange::attach_backbone` and `detach_backbone` return explicit retired-v12 errors at `…/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:274-295`. The manual `attach_sync_backbone` path has the same order at `…/Shell/…/🦀️.rs:3622-3667`.
6. Were activation able to commit, `pump_sync_events` does have a native receiver that sends snapshots and remote mutations into the plugin and then refreshes the renderer at `…/Shell/…/🦀️.rs:3172-3270`. It is not reachable from a successful document activation while the retired attach call remains in the transaction.
7. The actor itself can obtain an open plan and exchange its receipt through a typed source, put only the grant in `Sec-WebSocket-Protocol`, reject a late/cancelled dial, and validate the returned protocol at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1977-2043`. It waits for a matching `ServerFrame::Session` before releasing mutations at `:2429-2437` and `:2488-2515`.

Thus the actor transport is useful source evidence, not evidence of a WGPU user journey.

## Material blockers

| Boundary | Current evidence | Classification | Required invariant |
| --- | --- | --- | --- |
| Linked catalog → plan readiness | The sole hub linkage vector is empty (`📦️bin.rs:393-395`). | RED | No D0 readiness or plan issue unless an atomic, nonempty trusted native binding set has been built from exact local inputs. |
| Plan → local package/preselection | The plan schema carries `component_sha256`, `component_blake3`, and `descriptor_byte_sha256` (`📇️directory/🧬️schema/🦀️.rs:531-538`), but `DocumentSocketSurfaceExpectationV1` and `matches_surface` compare only kind, plugin/package/version, and surface names (`📇️directory/🔌️client/🦀️.rs:304-349`). The client repeats the same incomplete comparison at `:811-827`. | RED | A local executable is eligible only if *all* plan package hashes, descriptor digest/bytes, schema hash, app/window/role/renderer, and artifact kind match its verified identity. |
| WGPU descriptor → loaded component | WGPU parses descriptor bytes yet never hashes them or the selected Wasm bytes (`ProgramBridge/…/🦀️.rs:779-807`). | RED | One immutable verified package identity must bind the exact component bytes, descriptor bytes, manifest, app definition, and codec factory. No independently discovered file may be paired by package id/version alone. |
| Plan → session selection | The relay selects the active session first (`Shell/…/🦀️.rs:3687-3694`, `:4058-4075`), so a valid plan only post-validates a preselected app. | RED | Resolve a plan to an eligible local package/app/window before creating the session/actor; reject zero or multiple exact matches. |
| App event bridge → actor activation | `attach_backbone` always errors (`ProgramBridge/…/🦀️.rs:281-286`). `open_document` has already opened the actor when the error returns (`Shell/…/🦀️.rs:3704-3707`) and does not close its key. | RED, concrete resource/authority escape | Failed app activation must atomically retire the actor, clear its prebound surface, cancel/reap its dial, and leave no live connection, reconnect timer, outbox, or selected session. |
| Session identity / stale events | `ShellSyncChannel` carries document key, instance, and plugin id (`Shell/…/🦀️.rs:1186-1195`), while host keys are correctly scope-qualified as `Hub { space_id, document_id }` (`🏪️store/🔄️sync/🦀️.rs:113-125`). It does not carry the full verified package/plan identity or an activation generation. | RED | Every bridge message must be accepted only for one `DocumentRuntimeKeyV1`—scope, local verified package identity, app/window, plugin instance, shell session, activation generation, and socket actor epoch. |
| Disconnect/cancellation | The actor clears socket actor, confirmation, authority, deadline and colour together (`sync/🦀️.rs:1922-1938`), closes late invalid sockets (`:2045-2064`), and queues mutations until matching Session. `ArtifactHost::close_key` cancels and requests runner close (`:1266-1289`). | Source-only positive | Preserve these fences, but bind the shell's commit/teardown transaction to them; cancellation before commit must close exactly once and cannot render or reconnect. |
| Rendering/presence | Event draining can render snapshots/mutations, but Session is ignored, Preview has no WGPU UI, and heartbeats call the retired `ephemeral_snapshot` (`Shell/…/🦀️.rs:3210-3268`, `:3319-3350`; `ProgramBridge/…/🦀️.rs:289-295`). | RED for full collaboration rendering | Initial slice may render durable snapshot/mutations and sync state only; it must explicitly exclude live preview/presence UI until pushed-event equivalents exist. |

The comments at `Shell/…/🦀️.rs:3530-3536` still call host keys bare document ids, but the production `ArtifactDocumentKey` is already space-scoped. Treat that comment as stale documentation, not a current cross-space-key defect.

## Smallest implementation packet

### 1. Establish one real local catalog target first

Extend `linked_native_codec_bindings()` only with a first statically linked codec factory that already exists in the binary's dependency graph. Feed `TrustedCatalogLoader` exact descriptor bytes, component bytes, SHA-256, BLAKE3, descriptor-byte SHA-256, manifest, and codec factory identity—not a generated marketplace row. Publish the resulting generation only after every input validates; leave readiness false and the catalog absent on any missing, duplicate, extra, zero-hash, factory mismatch, or schema mismatch.

The first target should be the smallest descriptor-backed artifact with both an actual native codec factory and one WGPU editor surface. The current tree does not establish that any broad catalog row meets those conditions, so this packet must select its target only after an inventory proves it. It does **not** activate all plugins.

### 2. Make plan selection precede receipt exchange and actor creation

Add a private `VerifiedNativePackageIdentityV1` to `ProgramBridgeEntry`: package/plugin/version, component SHA-256, component BLAKE3, descriptor-byte SHA-256, descriptor digest, artifact schema/kind, pack-schema hash, and a descriptor-owned app/window table. Calculate it from the exact descriptor/component bytes at load time with bounded reads and wipe transient byte buffers.

Split native client admission into: (a) protected, bounded plan fetch and validation; (b) exact local identity resolution, requiring exactly one `ProgramBridgeEntry`/app/window; and only then (c) one-use receipt exchange. Extend the local expectation and `DocumentSocketAuthorityV1::matches_surface` to compare the complete package identity rather than names alone. A failed local selection wipes the unexchanged receipt, issues no SocketGrant and creates no socket. The server remains authoritative for scope/grant/catalog; the client proves it can execute precisely that authorized target.

### 3. Replace the retired attach call with a committed event bridge

Do not restore `AttachBackbone` or poll APIs. Introduce an event-driven bridge owned by the native shell, with a `DocumentRuntimeKeyV1` and two bounded sides:

- plugin durable mutations enter the host mailbox only after the matching actor Session event confirms the socket actor;
- host `SnapshotReplaced`, `RemoteMutations`, status, and Session events are delivered to the matching plugin instance, then `refresh_ui`; stale key/generation events are dropped;
- the bridge first registers its event receiver and plugin delivery capability, then opens `ArtifactHost`; only after all checks succeeds does it publish `sync_channel` and render the selected app;
- any post-open failure calls `close_key`, clears the prebinding and local selection, awaits bounded runner retirement, and never leaves a reconnecting actor behind.

`ArtifactHost` already supplies a scope-qualified key, cancellation and runner retirement (`sync/🦀️.rs:1126-1169`, `:1266-1318`); reuse them rather than introduce a parallel ownership registry. Explicitly remove both retired WGPU attach/detach call sites only once their event replacement carries the same cancellation ownership.

### 4. Make open/switch/exit a single identity transaction

Open uses a candidate key derived from the verified local target and plan. Switch first cancels the old key, rejects all old generation events, and retires its runner; only then commits the new key. Exit/drop applies the same close path. A late plan result, grant, WebSocket connection, bootstrap frame, Session, or renderer update whose key no longer equals the active key is closed/wiped/discarded. The existing actor's socket-epoch clearing and Session confirmation remain required, not optional.

## Hostile laws and neutral oracle

Place a language-neutral JSON corpus beside the document-open fixtures. Its oracle must use an independent JSON/hash/WebSocket implementation and a minimal protected HTTP test hub—not Rust helper functions from the subject.

1. Valid vector: descriptor bytes/component bytes/hash fields, exact WGPU editor app/window, plan, receipt exchange, Session, snapshot, one local mutation and peer mutation. Assert the rendered digest changes only after the matching event and the outgoing mutation is actor-stamped after Session.
2. Flip each component SHA-256, BLAKE3, descriptor-byte SHA-256, descriptor digest, package id/version, app, window, role, renderer target, artifact schema, pack hash, or catalog generation. Assert no exchange, no grant header, no actor registration, and no Wasm app creation.
3. Give two local entries the same advertised name/version but different component bytes. Assert ambiguous selection fails closed before exchange.
4. Force the event-bridge registration failure after `ArtifactHost::open`. Assert `close_key` occurs once, runner reaches terminal, pending dial closes once, zero reconnects/outbox writes occur, and no sync channel/render state remains.
5. Cancel after plan, after exchange, and after a successful WebSocket dial but before Session. Assert receipt/grant redaction/wipe, one close for a late dial, no Hello/Commands after cancellation, and no retry.
6. Deliver an old scope, old session, old activation generation, and wrong Session actor after a document switch. Assert none reaches the plugin/renderer or changes the new document.
7. Use same `document_id` in two spaces. Assert two scope keys can coexist and events/close of one cannot affect the other.
8. Disconnect after a delivered Session, then reconnect with a new grant/actor. Assert no operation uses the old actor, exactly one reissued receipt exchange, and only the new matching Session releases queued mutation(s).

## Gates and acceptance

Current registered evidence is insufficient for WGPU integration:

- `bun nx run os-hub:open-plan-check --skip-nx-cache` and `bun nx run os-hub:native-document-open-check --skip-nx-cache` are registered at `🌎️hub/📦️packages/🦀️rust/📋️project.json:111-141` and launch entries ` .vscode/launch.json:4433-4469`; the latter advertises native D1 scope/package/surface, receipt, Session, reconnect and redaction checks, not WGPU plugin activation.
- `bun nx run @semio-tech/framework-renderer-wgpu:test-native --skip-nx-cache` and `…:native-environment-check --skip-nx-cache` are registered at `…/wgpu/📦️packages/🦀️rust/📋️project.json:24-30,122-127`. The environment check proves protected process launch ordering, not document rendering.
- `bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker --skip-nx-cache` proves generated browser-carrier freshness only (`project.json:67-72`). It is not native backbone evidence.

After the packet lands, register an uncached `@semio-tech/framework-renderer-wgpu:document-backbone-check` in that package's existing `📜️script.ts`, with exact-one preflight for the eight laws above. Add a launch entry following the existing document-open gates. Its terminal must compose with, not replace:

```text
bun nx run os-hub:open-plan-check --skip-nx-cache
bun nx run os-hub:native-document-open-check --skip-nx-cache
bun nx run @semio-tech/framework-renderer-wgpu:document-backbone-check --skip-nx-cache
bun nx run @semio-tech/framework-renderer-wgpu:test-native --skip-nx-cache
```

The native application launch may then use the existing generated WGPU native entries (for example `🛠️dev🌐️gis📍️2d🧊️wgpu🖥️native` at `.vscode/launch.json:697-704`) only after the composed gate is green. `os-hub:secure-local-smoke` may be an additional process/FD3 transport check, but cannot substitute for the WGPU identity-to-renderer journey.

## Explicit nonclaims

This packet does not claim broad plugin marketplace activation, a browser WGPU path, MCP rendering, preview rendering, or a complete presence UI. It accepts only one statically linked, full-hash-verified native WGPU editor from trusted catalog selection through D1, SocketGrant, matching Session, durable event delivery, renderer update, cancellation, and reconnect.
