# Derived Browser Actor Worker Containment Frontier

## Outcome

The next authenticated derived-actor execution boundary must be a per-document child `Worker` owned by `🧵️backbone-worker.ts`, not a module import in React/ShellHost and not the shared plugin `ShardClient` pool. The backbone worker is already the document's authority owner; its independent event loop can enforce a real wall-clock deadline by terminating a non-cooperative child. The Shell continues to receive only scope-bound status/projection messages.

This is an execution-containment design, not a current production-execution claim. The current verified lease holds only raw component and descriptor bytes, then deliberately publishes `renderer-unavailable`; it does not have a derived ESM field, child worker, invocation protocol, physical memory admission, or persistent GIS command path.

This was a read-only source audit. No browser, build, or native test was run here.

## Current Boundary and the Exact Gap

1. [`🧵️backbone-worker.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts) owns one private `DocumentExecutionTargetLease` per `ArtifactState` (lines 221–238, 533–579). It streams the exact Hub manifest/component/descriptor through the broker-only three-route allowlist, bounds and hashes those bodies, and parses the descriptor before minting the lease (lines 594–770). The plan exchange keeps that lease private, but emits `renderer-unavailable` at lines 805–877.
2. The authoritative lease schema has only raw component/descriptor fields (`DocumentExecutionTargetLeaseFieldsV1`, directory schema lines 1163–1313). The protected Hub target route set likewise contains only `manifest`, `component`, and `descriptor`. There is no selected derived-browser-actor byte identity to fetch or verify.
3. The closed bundle is a genuine single ESM factory, but currently exists only in the build/fixture path. `closedBrowserActorBundle` has a maximum of 32 active invokes and rejects unknown path/arity (browser-bundle script lines 188–275), while its `invoke(path, args)` accepts arbitrary structured values and has no per-turn deadline, aggregate argument/result bound, static core-memory proof, or worker termination owner.
4. The first-party host/WASI layers already provide useful inner bounds: host effects/streams cap at 128 and an aggregate 1 MiB buffered stream (`🌐️host/🟦️.ts` lines 13–189); WASI bounds resources and output (`🌐️wasi/🟦️.ts` lines 14–229). These are not a Worker heap cap and cannot stop a CPU-bound guest.
5. `ShellHost` currently owns only the outer backbone worker at `new Worker(new URL(...backbone-worker.ts...))` (ShellHost lines 1712–1741), maps it by `documentRuntimeKeyV1`, and removes the route/status state on close (lines 1765–1783 and 4103–4134). It must not receive derived bundle bytes, a plan receipt, a broker proof, a child module URL, or a child `MessagePort`.

The existing execution-target browser gate is an appropriate registration seam: `os-hub:execution-target-lease-browser-check` runs the neutral/source validation and the `@semio-tech/framework-os:test-long` worker tests (Hub script lines 3811–3817). Its current fixture explicitly asserts all renderer claims are false (backbone test lines 4913–4952), so it must be extended rather than reinterpreted as execution proof.

## Smallest Coherent Owner Graph

```text
Hub selected catalog record
  -> private backbone DocumentExecutionTargetLease
      -> private DerivedActorOwner (one runtimeKey + lease generation)
          -> first-party dedicated execution Worker (one actor only)
              -> closed ESM Blob import / one per-activation host + WASI
          <- private MessagePort, typed effects/results
      -> scope-only status/projection to ShellHost
```

### 1. Add a derived payload to the selected lease before any worker work

Extend the catalog-owned derived actor record described in `📓️terra-closed-browser-actor-attestation-current-packet.md`, then project it into the *same* selected-plan/manifest comparison relation. The new receipt-free field must contain only:

```text
browserActor: {
  mediaType: "text/javascript";
  sha256; blake3; byteLength;
  codegenPolicySha256;
  coreLimitsSha256;
}
```

The Hub must serve that fourth body only through the authenticated exact-selection route, after the same session/membership/revision/catalog revalidation used for each current body. The backbone lease fetches it with the existing streamed-body helper, verifies both hashes/declared length, and keeps the `Uint8Array` private. Neither a package URL nor a client-selected artifact/digest is an alternative input. Do not overload raw `PackageHashes`: source component/descriptor identity remains there; this is a catalog-owned derived execution payload.

`coreLimitsSha256` is necessary because browser `Worker` construction has no portable heap-limit option and the JS WebAssembly reflection API does not reveal core memory limits. At build admission, parse every generated core's memory/table sections with a first-party bounded parser, reject shared memory and memory64, require an explicit maximum, enforce the selected policy ceiling, and hash the sorted `{coreSha256, initialPages, maximumPages, tableLimits}` rows. The worker verifies the record before instantiation. Until this exists, the path must not claim a `memoryBytes` sandbox merely because the legacy `ShardBudget` has a `memoryBytes` field.

### 2. Make the backbone worker the only supervisor

Add a module-local `DerivedActorOwner` to `ArtifactState` beside `executionTargetLease` (backbone worker lines 221–238, initialized at 2831–2870 and retired in `closeArtifactRuntime` at 2892–2908). It owns:

```text
runtimeKey, exact lease generation + derived payload identity,
child Worker, one transferred private MessagePort,
AbortController, activation/invocation epoch, deadline timer,
one active invocation, bounded byte reservations, pending effect controllers.
```

Create the child with a fixed first-party module URL such as `new Worker(new URL("./🔌️derived-actor-worker.ts", import.meta.url), { type: "module" })` from the backbone worker. It is not supplied by the document, plan, descriptor, or UI. Pass the verified ESM bytes and a new `MessageChannel` port directly to that child; transfer the payload buffer only after parent-side hash/length admission. The child re-hashes it before decoding it as strict UTF-8, creates a Blob URL only locally, `await import(blobUrl)`, immediately revokes the Blob URL, and never posts bytes/URL/receipt/broker proof to its parent or ShellHost.

The child receives an opaque activation epoch and public scope identity only. It never receives the browser-broker `MessagePort`, session bearer, socket grant, open-plan receipt, hub base URL, descriptor, or raw component. Every inbound/outbound frame is checked against the parent-held `runtimeKey`, scope, lease generation, and activation epoch; a stale/foreign frame is dropped before any effect or UI projection.

On close, plan invalidation, route change, or `docAbort`, backbone aborts all pending child effect controllers, clears the deadline, closes the private port, calls `worker.terminate()`, wipes any still-owned byte reservation, then drops the lease. It must not wait indefinitely for guest `close()` acknowledgement. A voluntary close may report terminal acknowledgement first; termination is the terminal backstop for a guest that never yields.

### 3. Define a bounded protocol instead of passing arbitrary `invoke` values

The shell must not obtain a generic actor `invoke`. Add a canonical Pack-encoded `DerivedActorInvocationV1` that is decoded by the backbone parent before forwarding:

```text
{ schema, version, runtimeKey, activationEpoch, invocationId,
  exportPath: [one or two declared names], argumentsPack: bytes }
```

`argumentsPack` is canonical and strictly bounded before decode; its decoded form has finite depth, item count, scalar/text/byte totals, and a total allocation reservation. The worker repeats the same canonical/bound check. The catalog's derived record contains one fixed execution-limits record (argument bytes, decoded aggregate bytes/items/depth, output bytes, concurrent invocations=1, timeout, and core-memory/table limits); the plan does not choose any of them. A pending invoke holds its reservation until exactly one result, deadline termination, child error, or close. Result/effect data crosses the port as bounded Pack bytes, not arbitrary structured-cloned guest objects.

The first effect policy should be intentionally narrow. `createBrowserHostActivation` exposes many effect names (host source lines 153–169), but the backbone parent should initially accept only descriptor-declared document reads and a typed, parent-validated document command proposal. It rejects network, registry, blob, cache, window/dialog, plugin-instance, job, and generic storage effects. A write is admitted only after the parent observes the exact editor lease grant and routes the typed proposal through the Map durable three-member/journal owner; a viewer always receives a typed denial. No child frame writes a socket, a WAL, the shell store, or the DOM directly.

Use a separate `execution-actor-status` worker response rather than widening `execution-target-status`: the latter means byte admission and is currently schema-tested as only `verifying`, `integrity-failed`, `stale`, `cancelled`, and `renderer-unavailable` (Hub script lines 3743–3752). The new response is scope-bound and contains only a bounded phase (`activating`, `ready`, `deadline-exceeded`, `capacity-exceeded`, `cancelled`, `faulted`) plus opaque invocation id/progress, never errors, bytes, URLs, grants, or receipts. Extend the central `BackboneWorkerRequest`/`Response` codec at OS `🟦️.ts` lines 706–775 and 812–918 with an exact parser, then make ShellHost match the existing `documentRuntimeKeyV1` entry before rendering a localized status.

### 4. The deadline is parent-enforced, not cooperative

Start the parent deadline before posting the admitted invoke. The dedicated child can be executing an infinite synchronous Wasm/JS turn, but the backbone worker event loop remains runnable; on expiry it terminates the child, settles the one invocation as `deadline-exceeded`, aborts every owned host effect, and releases all reservations. No `AbortSignal`, guest timer, `Promise.race`, or `close()` call is sufficient by itself for this case.

This is containment of UI responsiveness and application-managed inputs/outputs—not a browser security sandbox or a physical-memory quota. A dedicated Worker still has normal worker globals. The codegen policy must therefore retain its closed-module/global-capability fence; no route should advertise "sandboxed" until an explicit generated-source rule denies ambient network/storage/worker entry points except the small first-party runtime allowlist.

## Minimum Executable Corpus

Add a schema-first neutral fixture such as `derived-browser-actor-worker-v1` with one positive closed actor and these required vectors:

1. Exact selected scope/generation/derived-payload/policy/core-limits identity activates once, produces a bounded result, and emits no payload fields to ShellHost.
2. One-field changes to derived SHA/Blake3/length, policy digest, core-limits digest, scope, catalog generation, editor/viewer grant, activation epoch, or `runtimeKey` yield no worker/effect/publication.
3. Argument at limit succeeds; limit+one byte, noncanonical Pack, excess depth/items, second concurrent invoke, stale reply, and excess result/effect frame all fail before child execution and leave no reservation.
4. A deliberately non-cooperative infinite invocation is physically terminated by the parent deadline; a new document/actor can then activate, while the original child emits no late effect/result.
5. Close/revocation during activation, during a pending host effect, and after a cooperative result terminates/aborts exactly once, rejects late frames, and leaves no live child or byte reservation.
6. Viewer-originated write and a forged child `document-write` both remain uncommitted; an editor proposal is only visible after the actual durable Map group receipt (that later bridge needs its own acceptance).

The browser process law belongs in the existing `os-hub:execution-target-lease-browser-check` group, replacing the old `renderer-unavailable` positive only when the derived record and worker exist. Run a real Chromium page with the installed static child-worker module and a real Hub selected target: observe child `ready`; force the loop/deadline; verify `Worker` termination from the supervisor, a responsive page, no pending effect post-deadline, and successful fresh activation. The existing `@semio-tech/framework-os:test-long` tests are useful unit/worker support, but they cannot prove a child worker's physical termination without this process law.

## Implementation Order

1. Catalog/Hub: derived-record schema, selected exact body/manifest projection, revalidation and neutral route cases.
2. Browser bundle: builder emits and verifies core-limit metadata; no worker launch until policy/record checks are present.
3. Backbone: private `DerivedActorOwner`, static child worker, single-invoke Pack protocol, parent deadline/termination, status codec; ShellHost only maps status by exact runtime key.
4. Add the process law, then separately wire a typed GIS editor export to the durable Map approval/journal owner. Do not connect the general plugin backbone route or `loadPluginModule` to this path.
