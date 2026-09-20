# 🔄️ WGPU Sync Owner Lifetime Repair

## Scope

This packet replaces the native WGPU renderer's retired `attach_backbone` / `detach_backbone` stubs with the channel-v12 document-backbone protocol already owned by the plugin runtime. It preserves the detached Sync and Task Manager UI from `📓️astra-sol-sync-task-manager.md`; the repair is below that UI, at the retained kernel-event, ProgramBridge, Shell owner, and `ArtifactHost` mailbox seams.

The existing React oracle is `🏛️ShellHost/🟦️.tsx::bindDocumentBackbone`: binding carries the exact plugin instance, monotonic binding generation, and actor URI; an admitted port forwards actor messages into the plugin and guest effects back to that exact port; terminal faults close the port; retirement addresses the same owner. The shared implementation contract is `🔌️plugin/📡️backbone/🔗️binding`, whose neutral schema and fixture already cover accepted, replayed, stale, colliding, and refused bindings.

## Repair

### Shared binding codec

`🔌️plugin/📡️backbone/🔗️binding/🦀️.rs` now exposes the missing host-side operations:

- canonical `DocumentBackboneBindingCommandV1::encode` with the existing URI/control byte limits;
- exact receipt validation for schema, instance, generation, URI, expected operation, refusal code, and canonical bytes.

The plugin remains the sole binding reducer and channel owner. The renderer does not duplicate its state machine.

### Retained kernel event owner

The native WGPU kernel request queue now admits one bounded event of these forms per exchange:

- `SurfaceVisible`;
- `Message { source: Shell, ... }` up to the shared binding-control limit;
- `Message { source: Backbone, ... }` up to the shared hot-backbone limit.

The queued owner retains event kind, endpoint, and payload separately and retires them through the existing bounded shutdown lane. Other event variants and multi-event requests remain refused.

### ProgramBridge

`ProgramBridgeEntry` now provides native async operations that use the retained kernel exchange:

- `bind_document_backbone(instance, generation, uri)`;
- `retire_document_backbone(instance, generation, uri)`;
- `receive_document_backbone(instance, uri, payload)`.

Bind and retire require exactly one Shell receipt addressed to the same instance and validate it against the exact command. The receipt is consumed; any remaining effects stay owned by the caller. Backbone ingress validates the shared hot message contract before dispatch.

### Shell and actor lifetime

The native Shell now owns one monotonic, non-reused binding generation and records it in `ShellSyncChannel`. Its completion identity is the five-part tuple `(plugin id, instance id, binding generation, actor URI, scoped document key)`.

Attach/open now performs:

1. checkpoint and exact retirement of the previous generation;
2. actor open and event subscription;
3. fresh generation mint;
4. plugin binding command and exact receipt;
5. channel publication only after the receipt;
6. routing of any initial guest backbone effects.

An uncertain/failed bind issues one same-generation retirement for cleanup, closes the actor, and returns the original terminal failure. It does not retry the bind.

Detach retires the exact guest generation first, then sends the actor `Detach`, closes the document key, and clears status/presence. Retirement failure still closes local authority and is returned rather than swallowed.

Actor `DocumentBackbone` events are delivered once to the plugin with the exact URI. The Shell rechecks the complete owner after the await before accepting effects. Guest `Mutations` effects enter the active actor mailbox; guest `Ack` effects are consumed because the actor mailbox deliberately admits mutation batches only; genesis and malformed hot traffic fail terminally. Mailbox pressure, stale URI, missing plugin, receipt refusal, and guest delivery faults retire the channel in the existing per-frame sync phase without retries.

The same binding and retirement protocol now covers the short-lived background space-index instance. Both its binding turn and `touchArtifact` command effects are routed before retirement. The live-session space-index path now also feeds command effects through the Shell's canonical effect funnel.

## Hub overlay reachability

Hub remains reachable after removal from the default dock:

- footer control `framework.hub.signIn` routes to `/hub`;
- OS command `os.openHub` routes to `/hub`;
- `/hub` selects `ShellChromeFramePhase::HubWorkspace`;
- that phase publishes the retained overlay under `FRAMEWORK_HUB_PANEL_ID`.

The Hub is therefore route-owned, rather than an orphaned or substitute dock leaf.

## Laws and validation

Added/extended Rust laws:

- shared binding command/receipt round trip accepts the exact owner, rejects a stale generation, and surfaces a normalized refusal;
- Shell owner equality rejects late generation, foreign URI, and foreign instance completions;
- source closure requires both channel-v12 message kinds and forbids retired Shell calls;
- Hub closure requires the footer opener, OS command opener, `/hub` route, and retained overlay phase.

The existing language-neutral binding schema/fixture and existing React/third-party `📡️actor-backbone` suite remain the cross-language oracle. No duplicate fixture or runtime dependency was added.

Static parse verification:

- `rustfmt --edition 2021 --check` parsed the shared binding, ProgramBridge, Shell, and renderer kernel files without syntax errors. Formatting differences from concurrent shared work were reported; the command was check-only.

Focused Bun/Nx command:

`SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run <absolute os-command-shortcuts test> <absolute actor-backbone test> --config <absolute React config> --silent=false --reporter=verbose`

Result: **2 files, 12 tests passed**. This includes all eight actor-backbone ownership/capacity laws and all four neutral shortcut/React matcher/WGPU routing laws. An earlier `nx exec` attempt without the workspace project scope stopped on the repository's unrelated project-graph cycle, and a second attempt used repo-relative filters from Vitest's package root and selected no files; neither produced a test verdict.

Root owns native/wasm compilation and runtime activation, so this report does not claim a compiler, native test, browser, or end-to-end Sync result.

## Concrete limitation

The repaired owner is the native WGPU `ArtifactHost` path. The WGPU wasm branch still records the selected URI and delegates browser persistence to its host shim; it does not construct an in-wasm `ArtifactHost` document port here. React continues to own the real browser document port. Browser WGPU Sync parity requires a host-published document-port seam before it can be claimed, but the native retired-channel failure and native owner lifetime are no longer represented by compatibility commands or no-op adapters.
