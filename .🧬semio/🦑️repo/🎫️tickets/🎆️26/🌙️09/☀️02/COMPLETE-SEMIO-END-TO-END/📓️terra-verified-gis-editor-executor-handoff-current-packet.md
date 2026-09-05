# Verified GIS Editor Executor Handoff — Current Packet

## Decision

`renderer-unavailable` is the correct current result, not a small missing UI hook. The authenticated worker has a verified raw component lease, but the only runnable browser actor path accepts a generated JavaScript module URL and that JavaScript requires several relative JavaScript/Wasm assets. Passing the raw component to `loadPluginModule`, or exposing a URL/bytes through the public worker response, would either fail to instantiate or give a caller code-selection authority.

The smallest durable path is a **separately catalogued, closed browser actor ESM** built from the JCO output at materialization time. The browser fetches that single verified bundle through the existing protected target relay, transfers it once over a private broker-to-Shell channel, and gives a fresh Blob URL only to the shard activation. It does not re-use the current physical `bridge.js`, a caller module URL, directory scanning, or the retired backbone ABI.

This is an implementation packet only. No build, browser run, or product edit was performed for this audit.

## Current Proof and Exact Frontier

| Boundary | Present reusable behavior | Blocking fact |
| --- | --- | --- |
| Hub selection | Selection is re-authenticated, revalidated, generation-fenced, and resolves assets without any caller package/path selector. | It owns only `component` and `descriptor` bytes; the component is not a browser module graph. |
| Browser lease | The worker bounds, hashes (SHA-256 + BLAKE3), and descriptor-validates component/descriptor bytes before retaining a private lease. | It concludes with `renderer-unavailable`; no trusted executor handoff exists. |
| Actor runtime | `PluginRuntime` + `ActivationRegistry` + `ShardClient` already implement actor lifecycle, turns, UI patch retention, close/revocation, and private Worker activation. | `loadPluginModule` first fetches a descriptor beside a caller-supplied `moduleUrl`; `ShardClient.activate` likewise accepts a public string URL. |
| GIS command | `patchPositions` is declared as a GIS editor mutation and produces granular artifact changes. | The channel invocation adapter deliberately publishes `mutations: []`; the Shell currently persists none of an action response. |
| Existing persistence transport | A canonical `MutationEnvelope[]` sent as `localMutations` is bounded/retained by the worker and relayed to the authenticated hub socket; remote replay applies those envelopes to the app. | The old plugin backbone path is explicitly undrained after channel v12, so it cannot be used as the outgoing GIS command sink. |

### Source-backed details

- [Trusted catalog asset owner](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:303) exposes only `Arc<[u8]> component` and `descriptor`; `assets_for_current_selection` resolves exact current selection at [352](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:352).
- [Hub target selection](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2428) authenticates, revalidates, reloads the descriptor and checks directory revision before emitting asset bytes; its asset enum and route output are only `Manifest`, `Component`, and `Descriptor` at [2421](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2421) and [2531](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2531).
- [The worker lease](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:532) is non-serializable and wipes component/descriptor buffers on drop at [567](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:567). Its allowed relay path is deliberately only the three assets at [597](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:597), and successful grant currently reports the terminal non-execution status at [867](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:867).
- [The shared plan](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:950) binds raw component digests only. The receipt-free lease projection repeats that raw component identity at [1171](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1171) and validates it against the package at [1254](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1254). Therefore a browser ESM must not be silently substituted into `component`.
- [The current materializer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:367) emits a host shim, JCO component JavaScript, descriptor, and `bridge.js` as separate files. JCO rewrites core-Wasm URLs relative to `import.meta.url` at [745](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:745), and the bridge dynamically imports the component and host shim at [565](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:565). Those facts prove raw component+descriptor cannot execute the bridge closure.
- [The standard loader](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1109) first fetches a descriptor adjacent to a supplied module URL and registers that URL. It creates actor IDs from only plugin ID plus instance ID at [1200](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1200); that is insufficient for scope-bound verified target registration.
- [Shard activation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1434) currently receives `moduleUrl: string` and imports it. It has strong activation-generation/revocation handling, which should be retained, not replaced.
- [Current Shell opening](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4018) installs an already-loaded physical plugin before the verified worker open; it indexes document entries by the full `documentRuntimeKeyV1` at [4036](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4036). That full key is the correct verified executor identity.
- [Retired backbone functionality](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1676) explicitly states that main-thread backbone inbound has no guest drain. `PluginRuntime` leaves `attachBackbone`/`detachBackbone` undefined at [1536](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1536).
- [The actual retained network sink](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:646) accepts only canonical `MutationEnvelope[]` as `localMutations`; the worker bounds and queues it at [2897](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2897). Shell remote replay already uses `applyMutations` at [1863](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1863).
- [The current action adapter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1390) returns empty `mutations` at [1426](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1426); Shell consumes only history/effects at [4424](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4424). A `KernelMutation` is not a causal envelope—it lacks the wire payload hash, causal metadata and document binding required by the hub sink [1200](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1200).
- [GIS `patchPositions`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:225) is a declared command, declared artifact-only publication at [284](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:284), and a mutation action at [901](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:901). It has an existing native granular-diff law at [133](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🦀️.rs:133).

## The Closed Browser Artifact

### 1. Add a distinct catalogued asset; do not overload `component`

Introduce a non-optional browser execution field for a `rendererTarget: "wasm"` selection, for example:

```ts
type DocumentOpenBrowserBundleV1 = Readonly<{
  format: "semio.browser-actor-esm/v1";
  sha256: string;
  blake3: string;
  byteLength: number;
}>;
```

It belongs in the canonical package descriptor/open-plan projection and in `DocumentExecutionTargetLeaseFieldsV1`; it is required exactly for a browser-Wasm selection and rejected for a selection that cannot execute it. Keep `package.component*` as the genuine raw component identity for native codecs. The descriptor must bind both the raw component digests and this new bundle digest. The trusted catalog must load, bound-check and dual-hash the bundle alongside its component/descriptor before it is retained. `VerifiedExecutionTargetAssets` should then own `browser_bundle: Arc<[u8]>`, not reinterpret `component` as JavaScript.

The protected assets become `manifest`, `browser-bundle`, and `descriptor` for the browser. The browser does not need to download the unused raw component. The Hub must apply the existing exact selection/authentication/revalidation path to `browser-bundle`, with `no-store`, exact content length, and the same cancellation/deadline semantics as the other assets. The worker's route regex, enum, stream bounds, progress vocabulary and plan-to-lease equality must add the bundle field in the same atomic schema change. No optional legacy component-as-bundle interpretation is safe.

### 2. Materialize one self-contained ESM at build time

Extend the product web materializer, not the browser:

1. Run the existing JCO transpile through [transpilePluginComponent](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:769) exactly as now.
2. Generate a first-party closed entry from that JCO result. It contains the current bridge contract (`createActorApi`) but transforms each JCO core-Wasm URL/load to an embedded immutable byte table and an internal `WebAssembly` instantiate path. It turns the current host shim into `createHostShim(actorId, activationGeneration)` instead of a module-global shim selected by query-string URL.
3. Bundle that generated entry with the already-used browser ESM `Bun.build` seam ([existing use](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:4991)), no code splitting and no external imports. Emit its bytes as the new package asset, hash them, and place the exact digest/length in descriptor/catalog generation input.
4. Parse the final ESM during materialization and reject a static import declaration, an import expression, or component/core asset `fetch`/URL resolver. A host-mediated application effect may remain only if it is an already permitted effect request through the shard protocol; it is not part of component loading and cannot select an executable module.

`Bun.build` alone is not enough. The existing bridge deliberately uses query-parameter-distinct dynamic imports [565–575](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:565), and the host shim keeps `effectSeq`/`pendingEffects` in module scope [923–929](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:923). A single bundled ESM would otherwise cache and share that state across actors.

The emitted entry's only public export should therefore be:

```ts
export async function createActorApi(actorId: string, activationGeneration: bigint): Promise<ActorApi> {
  const host = createHostShim(actorId, activationGeneration);
  return instantiateFreshActor(host, embeddedCoreBytes);
}
```

Every invocation must allocate fresh component instance state, imports, memories/tables and host pending-effect map. Reusing immutable compiled `WebAssembly.Module` values is permissible only if every call creates a fresh instance; the conservative first implementation compiles/instantiates per actor. The source must not rely on Blob URL query variants for isolation.

### 3. Private, scope-bound handoff and factory activation

Keep verified bundle bytes out of `BackboneWorkerResponse`, `ArtifactActorMsg`, React state, plugin `viewState`, and public lease fields. Add a private `MessagePort` protocol between the broker worker and the Shell:

1. The Shell asks to claim exactly `{ runtimeKey, scope }` only after the worker has admitted the plan, verified the bundle/descriptor, and obtained the socket authority.
2. The worker compares the complete scope and current live lease, transfers the bundle buffer exactly once, keeps only receipt-free immutable selection fields needed for state, and marks the lease claimed. Close/cancellation before claim wipes the bundle; a duplicate/foreign/stale claim fails without a buffer.
3. The Shell immediately wraps that transferred buffer in a module-private `VerifiedBrowserExecutionBundleOwner`; it independently rechecks its exact pre-verified fields, mints no public URL, and owns final zero/revoke/actor close. The worker should emit `active` only after Shell acknowledges actor activation; failure returns to a terminal integrity/activation error and retires the owner.

The current `documentRuntimeKeyV1` uses both space and document identity ([implementation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:608)); derive both the registry key and actor ID from that opaque runtime key plus the verified bundle SHA-256. Do not key a verified actor only by GIS `pluginId`, which allows colliding document/space instances to overwrite a shared registry manifest.

Add a private verified registration source beneath `PluginRuntime`, conceptually:

```ts
type VerifiedActorModuleSource = {
  readonly registrationId: string; // derived internally from scope + bundle digest
  mintForActivation(actorId: string, generation: bigint): PrivateModuleUrlLease;
  close(): Promise<void>;
};
```

`mintForActivation` creates a **new** Blob URL from the owned immutable bundle bytes for each activation/recovery, and the shard-owned activation lease revokes it only after that actor is disposed. This avoids ESM cache/shared-state coupling. `ActivationRegistry` needs a source/factory registration for this sealed path rather than its present `pluginId -> moduleUrl` entry; `ShardClient` may still receive a string internally, but neither caller nor plugin can supply it. Ordinary physical `loadPluginModule(pluginId, moduleUrl)` remains separate.

The verified loader accepts the already checked canonical descriptor manifest—never `fetchDescriptorManifest`—and rejects before `register`/actor allocation unless all of these agree: exact scope/runtime key, selected app/window/surface, editor role/write grant, descriptor digest, raw component digest, browser-bundle dual hash/length, catalog generation, plan revalidation binding, and Bundle format. The plan receipt and socket-grant receipt remain worker-only; neither reaches the shard or plugin.

## Command Publication to the Retained Hub Sink

The first GIS proof should remain deliberately one-document, one artifact lane: `s.gis.gismap@1/*#editor` with `patchPositions`. It must not claim the future three-member Map durable group.

Add a channel-v12 **typed committed-publication frame** emitted only after the guest has accepted a mutation action. It must carry canonical `MutationEnvelope` bytes (or an opaque pack decoded only by the trusted Shell into those exact envelopes), not the loose `KernelMutation` TypeScript object. The protocol-owned producer has the document/actor causal identity and can include the actual payload hash, inverse, dependency and causal/frontier data. The Shell's verified executor validates the returned document/scope/schema/actor/activation/role against its owner, then sends only those envelopes to the existing worker request:

```ts
{ kind: "send", documentId, spaceId, message: { kind: "localMutations", envelopes } }
```

That reuses the existing retained outbox and hub commands/ack/replay implementation. It must not route through `registerPluginBackboneRoute` / `postPluginBackboneInbound`, synthesize an envelope from `KernelMutation`, or write a local snapshot as a substitute for a hub mutation. UI state may optimistically reflect a guest patch, but user-visible durable success is only the normal hub command acknowledgement/rebootstrap observation; rejection restores/refreshes from the last acknowledged document state.

## Minimum Schema-First Laws

Create one neutral fixture family, `verified-gis-editor-executor-v1`, containing canonical plan/lease/bundle descriptor identities, a closed fixture ESM source with embedded tiny Wasm bytes, scope/runtime key, one `patchPositions` action, and expected canonical envelope/snapshot values. Its schema must reject unknown fields and carry both component and bundle digests.

1. **Closed bundle admission and isolation (language-agnostic + browser).** A real browser Worker imports the transferred Blob ESM, creates two actors from the same bundle for two different `{spaceId, documentId}` values, and proves independent host effect sequence/state. Assert the final ESM has no executable static/dynamic imports and no JCO component/core network asset fetch. Tamper one byte, substitute raw component bytes, use a viewer surface, swap scope/runtime key, or re-claim the lease: assert no actor, no plugin-visible byte/receipt, and no socket command.
2. **GIS action publication (native).** Use the selected GIS editor package and its existing `patchPositions` vector. After plan/descriptor/bundle admission, the genuine typed action produces exactly one document-bound canonical envelope accepted by the existing retained worker/hub sink. Assert wrong app/window/surface/action, forged envelope document/schema/actor, duplicate publication, and hub rejection produce no durable GIS edit.
3. **End-to-end close/reopen (browser + selected SQLite Hub native).** Author opens the real selected GIS editor; bootstrap loads state; `patchPositions` is acknowledged; close revokes the actor Blob URL and zeroes the private owner; reopening through a fresh plan/socket/actor returns the edited position from persisted Hub data. The paired same-document/different-space session sees neither the actor nor mutation. A close during bundle fetch/activation/action retains no reusable bytes and leaves no queued mutation; a transport break after an accepted local envelope uses the existing outbox retry and yields one logical edit after re-open.

The existing browser target source law around [4877](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4877) must be replaced, not weakened: it currently proves the intentional no-loader boundary. The new test must prove the actual private broker handoff and no public byte transfer. The GIS native granular-diff law remains reusable but is insufficient alone because it does not exercise the plan, bundle, socket, or persistence authority.

## Ordered Implementation Slice

1. Schema/catalog/Hub: add typed browser-bundle descriptor/catalog/plan/lease asset and protected route; update canonical fixtures and source/native catalog laws.
2. Materializer: make the factory-closed ESM and its materialization structural law; publish no separate browser-required component/core/host/Preview2 files for a closed target.
3. Worker/Shell: add the one-shot private claim/ack lifecycle keyed by `documentRuntimeKeyV1`, then add `VerifiedActorModuleSource` activation under the existing shard lifecycle. Replace the `renderer-unavailable` success outcome only after actor open succeeds.
4. Channel publication: add the typed committed-envelope frame and let the verified Shell executor send it to the existing `localMutations` sink. Delete/avoid the dead backbone outbound dependency for this path.
5. Run the three laws above against the real GIS editor and selected SQLite Hub; only then extend beyond `patchPositions` toward the multi-artifact Map coordinator.

## Safety Nonclaims

- A valid raw component/descriptor today does **not** prove browser execution.
- A Blob URL is an internal activation implementation detail, not a plan field, public response, or caller-supplied authority.
- Bundle hashing validates the served closed artifact; catalog/descriptor validation must bind it to the raw native component identity before execution.
- Loading/rendering a GIS actor does **not** prove persistence until a canonical envelope passes the existing retained outbox and a close/reopen observes the edit.
- This packet intentionally does not claim multi-artifact Map publication, an AI proposal approval flow, or WGPU execution.
