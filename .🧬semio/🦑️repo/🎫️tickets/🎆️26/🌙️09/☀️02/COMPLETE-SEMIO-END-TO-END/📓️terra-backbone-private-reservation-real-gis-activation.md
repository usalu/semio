# Private Backbone Reservation to a Real GIS Actor

## Current boundary

The new reservation is a useful lifecycle boundary, but it is deliberately not activation.

- `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:804-841` creates one `DocumentBrowserActorReservation` from the current private `DocumentExecutionTargetLease`, exchanged socket actor grant, a monotonic generation, document/lease abort signals, and a retirement timer.  It rechecks the same state slot, same live lease, same grant, and the reserve deadline after `reserveBrowserActorChild` resolves.
- `:844-869` binds the lease to the current hub `{spaceId, documentId}`, schema, normalized Hub origin and runtime key.  `none`, a missing/expired grant, duplicate reservation, stale state, close/replacement during reservation, and worker construction failure are denied.
- `:794-798` closes the child before wiping the lease's verified component/descriptor bytes.  This ordering is correct and its TDD observes it.
- The TDD at `:5031-5104` is explicitly reservation-only: its fixture
  `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️browser-actor-reservation-v1.json`
  requires `bodyRequests: 0` and `loadedActors: 0`.  The sole production reference to
  `reserveDocumentBrowserActorChild` is currently that test (`:5074`, `:5080`, `:5088`,
  `:5093`, `:5096`).  It must not be described as an editor launch.

The lower child is likewise only a protected execution container:

- `🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:16-29` admits the actor id, generation and actor byte digest/length, reserves fixed capacity, and boots a dedicated module worker.
- `:74-90` transfers a caller buffer to exclusive parent ownership before hashing, rehashes it, and transfers it once to the child.  `:93-155` fences all invocation/result messages by nonce, generation and sequence.
- The child worker at `🧵️child/🧵️worker.ts:20-21` intentionally gives the generated actor a deny-all host port.  It has no actor-body broker, host-effect message, effect-result message, `resolveEffect` path, or WASI port.  Backbone never calls its `load` or `invoke` methods today.

This correctly keeps actor bytes and effects off the current route.  It also means the current `renderer-unavailable` status at Backbone `:964-967` remains truthful.

## Smallest closed activation seam

Add one **private** `DocumentBrowserActorOwner` inside `DocumentBrowserActorReservation`; do not add a public browser bundle URL or a generic child port to `ArtifactState`.

1. After a reservation is live, request one new protected `browser-actor` body only through the existing document/plan broker authority.  Its request must retain the parsed plan intent/receipt internally; it may not select package, digest, generation or path.  The response length and SHA-256 must equal `lease.fields().browserActor` and its `ArrayBuffer` must be owned by the reservation.
2. Immediately before the request, after every awaited body chunk, and immediately before `child.load`, require all of: exact `artifacts.get(runtimeKey)`, `state.browserActorReservation === owner`, exact lease identity/live state, exact grant identity, correct scope/schema/origin, and the applicable expiry.  On any failure, zero the owned body and close the child.  No result body should survive in a detached promise after close.
3. Pass only that owned buffer to the existing `BrowserActorChild.load`; its transfer plus the child rehash are the actor-byte handoff.  The child must receive `{actorId, activationGeneration}` only, never the plan receipt, lease, Hub origin, space id, or a browser broker capability.
4. Represent later host traffic as a private bridge keyed by `(runtimeKey, reservation generation, actorId)`.  Each child-to-parent frame and each completion must match all three before dispatch/resolve.  A stale/duplicate completion is dropped; a malformed frame terminates the child.  The reservation owns the bridge's abort controller and closes the child before wiping bundle/lease bytes.

`DocumentExecutionTargetLease` currently retains component and descriptor bytes only (`backbone-worker.ts:543-595`).  The actor bundle needs a separate linear `DocumentBrowserActorBundleSource` private to the reservation: `take_for_load()` is one-shot and `close()` zeros an untransferred buffer.  Do not make browser-actor bytes another public `installedTarget` field.

The generated closed bundle already expects exactly this shape.  `🔌️plugin/🌐️browser-bundle/📜️script.ts:404-471` exports `activate(identity, port, control)`, creates an activation-local `createBrowserHostActivation`, conditionally creates activation-local WASI, and returns `invoke`, `close`, `resolveEffect`, and `rejectEffect`.  The worker's internal actor shape must therefore be extended privately to retain the latter two methods.  Its current `hostPort` is intentionally insufficient.

## Typed host and Map boundaries

The bridge must implement the existing first-party port, rather than inventing a GIS-specific JavaScript API:

| Boundary | Existing contract | Required owner rule |
|---|---|---|
| Host async | `🌐️host/🟦️.ts:1-177`, especially `BrowserHostFrame`, `BrowserHostPort`, tagged `{tag:"ok"|"err"}` replies, and activation-local cancellation | The bridge validates the identity/generation itself, retains request IDs, and returns only canonical pack/fault bytes.  It must call generated `resolveEffect` only after the exact scoped host operation completes. |
| WASI | `🌐️wasi/🟦️.ts:1-15` and bundle activation `📜️script.ts:419-424` | If the actual materialized GIS import set includes a supported Preview2 interface, provide an activation-local monotonic clock plus bounded copied stdout/stderr sink.  No ambient environment, filesystem, process, network, terminal or shared WASI resource crosses the reservation.  `WebAssembly.Suspending`/`promising` remains a required JSPI admission, not a Promise substitute for a synchronous trampoline. |
| Initial document view | Backbone's authenticated bootstrap owns `state.currentPack`, `state.currentSpr`, and `state.frontier` after `installArtifactBootstrap` at `backbone-worker.ts:1646-1671` | A `document-read` bridge may return a bounded copy of the current canonical pack only for the exact active document/lane contract.  It must not let the guest select another document id, space, frontier, blob, or URL. |
| Mutation | WIT defines `host-async.document-write` at `🔌️plugin/🧬schema/📜️.wit:1213-1214`; Backbone only relays already-created replication `MutationEnvelope`s at `backbone-worker.ts:1446-1470` | A browser actor must never call `relayMutationsToHub` directly.  Convert a recognized Map operation through the per-document owner and the same typed Store/DB authority used for approval; fail closed with a typed capability error until that owner exists. |

This last rule is material.  `relayMutationsToHub` has the right socket/write-role fence, but it has neither a typed GIS operation parser nor a fixed-three publication/witness.  Store's
`DurableOwnedThreeStoreMapAssemblyV1` is real ownership scaffolding (`🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1120-1191`), yet its current laws use demo Stores/fake journal and no GIS per-document actor mounts it.  The Hub remains unavailable at that approval boundary.  Browser execution must therefore not claim `patchPositions` persistence, undo, or durable three-member publication yet.

The universal actor ABI does identify the future bridge surface: `world actor` exports async `reactor`, `jobs`, `checkpoint`, and `describe` (`📜️.wit:1319-1326`); `reactor.poll` is async (`:1154`), and `document-read`/`document-write` use canonical `pack` parameters (`:1203-1214`).  The concrete generated GIS export spelling and a real poll vector still have to be read from the freshly materialized GIS output, not guessed from a synthetic actor fixture.

## First executable law

The next honest law is deliberately smaller than Map persistence:

`authenticated_closed_gis_bundle_loads_describe_and_retires_in_real_chromium`

It should use the fresh, independently verified GIS materializer result selected by the registered target
`⚖️gate🧬️trusted-stdio-gis-bundle-native🌎️hub` (`.vscode/launch.json:6409-6417`), not a fixture component or a caller-provided URL.  A real authenticated plan/manifest/lease must expose the actor metadata; the private actor-body request must return exactly the corresponding closed-bundle bytes.  In Chromium, reserve the child, load those exact bytes, call the observed real `describe.describe` export (the WIT requires no arguments), compare its canonical result with the native materializer oracle, then close the document.  Assert:

- one protected actor-body request and no arbitrary package/path request;
- parent and child SHA/length agreement plus transferred-source detachment;
- no unexpected host effect, no mutation envelope, no outbox write, and no local Store publication;
- close/lease expiry/abort terminates the worker, rejects an in-flight call, clears the runtime-key reservation, wipes any untransferred bytes, and restores child capacity to zero.

This proves exact GIS byte activation only.  It does **not** prove document sync or rendering.  The subsequent bridge law may only be added once the native GIS oracle supplies a concrete `reactor.poll` event/input that causes a known `document-read` or UI effect.  It must use that real export and then prove same-scope pack response plus cross-space rejection.  A hand-written component that calls `documentRead`, or the existing host/child fixtures, would be a false positive for GIS.

## Admission and cleanup gaps to close with activation

1. `DocumentBrowserActorGrant` distinguishes `reserveBeforeMs` from `retireAtMs` (`backbone-worker.ts:800-817`).  The current code checks the former only before/after reserve but timers at the latter.  Make the intended post-reservation authority explicit: either socket-grant expiry retires the child, or it is solely an admission cutoff and plan expiry is the lifetime.  Add a native/Chromium law for idle grant expiry; otherwise a real body could remain runnable in the interval between those times.
2. Keep `DocumentBrowserActorReservation.close()` as the sole cleanup path.  Its current comparison-clearing of `state.browserActorReservation` is correct.  The forthcoming body fetch must not clear `state.executionTargetLease` without `lease.drop()`, and must not leave a body owner after an await loses the state/lease comparison.
3. Do not widen `browserExecutionTargetAssetRequest` (`backbone-worker.ts:603-614`) into a caller-selectable asset endpoint.  Add the body as a closed enum member of that same exact document-scoped policy and require plan intent/revalidation before every returned byte.
4. Do not pass the generic `BrowserHostPort` directly to Shell/plugin UI.  Its effects include storage, blob, HTTP, document write, plugin spawning and UI emissions (`🌐️host/🟦️.ts:156-168`); the reservation must present only individually admitted operations, with denied operations producing a tagged canonical fault.

No product code or build was run for this audit.
