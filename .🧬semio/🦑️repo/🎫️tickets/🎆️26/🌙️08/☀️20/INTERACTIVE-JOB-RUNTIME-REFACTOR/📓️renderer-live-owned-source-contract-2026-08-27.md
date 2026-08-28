# Live Owned Renderer Source Contract

## Decision

The live window/panel result must be the exact `OwnedUiInstanceSurface` facade of the lifecycle-bound `OwnedUiInstance`, not a `BuiltNode`, a copied snapshot, or another `UiDocumentStore`. The existing nine instance tests establish the private input/publication/retirement handshake, but the live application still uses the old maps. Binding an empty instance participant beside those maps would be incorrect and is not proposed.

This is the concrete integration contract for the renderer lane and Demonstrator lifecycle lane. Existing source stays stable during the coordinator's full React gate; the signatures marked additions below are not yet implemented.

## Observed Live Boundaries

`PluginRuntime/🟦️component.tsx` currently decodes and applies patches in `applyRetainedWindowPatches`, stores content in process-global `retainedWindowByActor`, and projects it through `retainedSurfaceToBuiltNode` in `retainedUiRefreshResponse`. `refreshUi` returns those copied bodies. `settlePluginTurn` combines returned turns, so native patch authority must be captured from each original turn before this combination, never from a synthesized combined result.

`ShellHost/🟦️component.tsx` caches the returned bodies, resolves external slots over them, dispatches `SET_WINDOW_UI_BY_WINDOW_ID`, and converts bodies back to flat snapshots in `builtNodeStoreFor`. Its window, panel, spawned-window, and contributor paths must change together. The old external-slot traversal cannot be retained as a hidden tree-copy stage; its node-level behavior belongs at the owned Interpreter boundary.

`Interpreter/🟦️component.tsx` reads the old store in `InterpretedUiNode`, `UiNodeView`, container and tree children. Surface hosts additionally decode complete scene bytes and nested JSON. The owned node/read/scene facade is ready as a foundation, but these consumers are not migrated yet.

## Renderer-Facing API

The host-local `PluginWasmHandle.refreshUi` result will become `PluginUiSurfaceRefreshResponse`, with window/panel entries `{ readonly key: string; readonly source: OwnedUiInstanceSurface }` and the existing typed `requestedEffects`. This is a host reference contract, not a native wire record and not a compatibility union with `BuiltNode`. The current request identifies authored body keys and may remain unchanged. Each returned `source` is obtained by advancing the exact aggregate's existing `beginSurfaceLookup(activation, lifetime, bodyKey)`; repeated lookups return the same facade.

The source's existing stable `view` is `{ revision, root, hash }`. Its identity changes only at atomic paired node/scene publication. Add `subscribeView(notify): OwnedUiReadSubscription` to the facade, forwarding the Surface owner's existing bounded view subscription, and retire it through the existing exact `unsubscribeNode`. A root hook uses this subscription plus `source.view` in `useSyncExternalStore`; no node owner is captured by a metadata read. Before publication the old view remains visible; cancellation cannot publish a new view. Old immutable metadata can remain a scalar React value without retaining an obsolete content tree.

The live Interpreter entry becomes `InterpretedUiNode({ source: OwnedUiInstanceSurface, onAction, onIntent, requestContextMenu })`. It observes that root metadata and renders per-ID children using the existing `useOwnedUiNode(source, id)` and managed `useOwnedUiScene`. Node subscriptions issue exact paired node/scene snapshots, with stable repeated reads, two pending issued snapshots per consumer, exact layout acknowledgements and child-reader-first retirement. A root change causes a root subscription replacement; React's external-store precommit recheck prevents committing a stale root metadata selection. Two windows showing one body use independent subscriptions over the same source, not duplicate content roots.

Shell caches retain only facade references and window/panel routing metadata. They do not own content copies. Replacing a session or closing a tab unmounts its subscriptions; the original lifecycle aggregate retains every detached subscription/read/scene close queue until terminal. A stale refresh result is ignored as routing metadata only: it cannot cancel or discharge a newer aggregate and must not substitute a new actor/instance lookup for its captured source.

## Original Turn Intake and ACK

### Concrete Callable Now Source-Present

Renderer-owned `UiDocumentStore/📥️intake/🟦️component.ts` exports `OwnedUiPatchIntake`. Its constructor is `(owner: OwnedUiInstance, source: OwnedNativeUiPatchAuthority)` and rejects a foreign source/host binding before beginning lookup or transferring any input. It stores the exact source and aggregate, not an actor name lookup. One intake owns one native patch; the lifecycle output owner schedules original patches sequentially and retains any unoffered output. Its five current canonical tests and red/green history are recorded in `📓️renderer-owned-intake-2026-08-27.md`; it is not yet mounted in the live lifecycle scheduler.

The API is `advance(grant): RetainedUiWireStep`, `failure: string | null`, `peekAcknowledgement(): OwnedUiPatchAcknowledgement | null`, `acceptAcknowledgement(receipt: OwnedNativeUiPatchSubmissionReceipt): boolean`, `takeSurface(): OwnedUiInstanceSurface | null`, `beginClose()`, `closeStep(grant): RetainedUiWireStep`, and `terminalIsEmpty(): boolean`. The caller already owns `source`; it pairs that exact object with `peekAcknowledgement()` when calling its dedicated lifecycle submission. Failed submission leaves the intake's token unchanged. `takeSurface` is available only after successful native publication receipt and bounded patch close; it returns the same facade, not a captured content tree.

Each admitted `advance` performs one existing bounded operation or one fixed bookkeeping transition: acquire the single lookup slot, advance one lookup cell, close the lookup, construct the patch, offer one native operation, advance the byte/typed operation cursor, release its exact completed input token, seal input, advance publication, wait for the exact ACK receipt, or close the patch. No loop over all operations, no local timer and no fresh raw operation arrays. The scheduler supplies repeated `1/4096` opportunities and must separately check its actual outer deadline. The intake does not own or close the whole instance; it only closes its own lookup/patch/read-facade alias.

`beginClose` does not drop a committed ACK. `closeStep` finishes cancellation/input retirement, preserves any late committed publication token for the same dedicated submission, and only clears its small source/aggregate aliases once its local descendants are terminal. Parent lifecycle close may concurrently request the aggregate's close; retired lookup/patch handles are recognized by their exact terminal state rather than used again. Raw original-turn/patch wrapper retirement remains with Demonstrator's source owner. A local rejected diagnostic is not native `PatchRejected` evidence: the new producer receipt/rejection handshake must be joined explicitly before ordinary live failure recovery is credited.

The canonical `ActorUiPatchReceipt` is now retained through the actual native authority and privately captured by the UI token. The exact frozen source receipt is required; no UI revision or operation ordinal is used to manufacture `patchSequence`. The independent instance/intake gates for this join are recorded in `📓️renderer-producer-receipt-adoption-2026-08-27.md` and the coordinator's issued-receipt report.

Demonstrator captures one lifecycle owner before open. The native Captured receipt supplies the actual three-field lifetime; only then is the single concrete UI aggregate created and bound with `bindHostRetirement`. Any open-turn UI output arriving earlier remains retained by that same lifecycle owner until it can be offered; no guest lifetime is synthesized.

For every original returned turn, Demonstrator calls `captureUiPatchAuthority(originalTurn, patchIndex)` and supplies the resulting exact authority to the renderer intake. The renderer advances one pending aggregate lookup and calls the existing `owner.beginPatch(sourceAuthority, facade)`. It offers only the current ordinal. Successful `offer` privately installs native accepted-input ownership; completed decode/typed input retirement yields the exact private input-retirement token. The native source retains raw wrapper/turn owners separately.

The intake continuation retains the aggregate, lookup/patch, original source authority and ACK token across yields, cancellation and send refusal. `peekAcknowledgement` is reached only after paired publication, required notifications and prior-root retirement. Demonstrator submits that pair using its dedicated lifecycle-authorized `submitUiAcknowledgement`, including after ordinary operation revocation. Only its exact successful `OwnedNativeUiPatchSubmissionReceipt` is accepted by `patch.acceptAcknowledgement`. No raw `patchAckEvents` is emitted for this path.

All UI-producing command, open, refresh, continuation and ACK-returned turns use this same intake; there is no command-side old-map escape. Newly returned ACK turns are new original sources under the same captured lifecycle owner, not merged arrays whose identity has been lost.

## Close and Ownership

The lifecycle close job first revokes new operation admission and unmounts renderer subscriptions, then continues the original aggregate's `beginClose`/`closeStep(1,4096)` and outstanding native-source/ACK jobs. Close remains authorized after activation revocation. The aggregate drains child queues before waiting for issued-root counters and closes wire streams before surfaces. It cannot become terminal while an accepted input or required publication ACK is outstanding.

Only `takeRetirementWitness()` after actual aggregate terminal yields the private final host witness. Native raw-turn retirement and native Retired are separate obligations. Final lifecycle ACK requires their exact conjunction and the original UI witness. Deleting a routing cache entry, transferring an ACK token or checking a global empty map is not closure evidence.

## Edit Ownership and Order

| Owner | Exact work |
| --- | --- |
| Renderer lane | Facade root subscription/hook; host-local source response type and response projection helper; retained per-turn UI intake owner; ShellHost/ShellHelpers body routing and Interpreter node/scene consumers; prepared JSON/pack projections; WGPU retained-source consumer. |
| Demonstrator | Lifecycle capture/create/destroy/mount; original-turn authority capture and scheduling; dedicated issued-ACK submissions/receipts; raw returned-output retirement; binding the single real UI aggregate and final witness. Renderer does not edit scheduler/create/destroy regions concurrently. |
| Dag | Native lifecycle/reactor descendants and seven tutorial/local-interaction producer joins. |

First implement the root facade/React source contract with actual native-operation DOM tests. Then complete typed host preparation before switching live windows/panels and removing their old content cache/conversion path. The coordinated cutover must atomically route every returned turn and every consumer to the same aggregate; an intermediate empty-host witness is forbidden. WGPU consumes the same exact facade/intake authority with `(activation, guest lifetime, instance, surface)` identity and its own captured readers; its old actor-only retained map and synchronous wire-turn helper must be removed at its corresponding cutover.

## Required Gates and Remaining Boundaries

Native-source-to-DOM tests must cover unchanged refresh identity, node replacement, root replacement, two windows/consumers on one surface, same numeric instance reused with a new native guest lifetime, an old returned turn after reuse, cancellation at each input/publication prefix, failed ACK retry, and close while old React/scene readers are held. Observable labels, actions, context menus, selection and all fifteen supported scene hosts must remain functional.

The fourteen nested-JSON host paths and TextEditor's generic-pack/editor admission still need genuine bounded prepared consumers. Exact text-byte reading, provisional JSON lexing, privately minted base64 pages and generic-pack token documents are implemented and tested. Indexed JSON documents and bounded scalar projection are the next ownership join; none of the parser foundations alone is live host adoption. Full native u64 node/revision identity remains explicitly fail-closed above JavaScript's safe integer domain. Native finite geometry/default parity and actual platform allocation/host-call timing remain separately stated obligations, not inferred from one-item source tests.

## Current Verification

Own instance R18: nine passing, 607 skipped, 616 discovered. Coordinator independently repeated nine passing with stable source hashes and strict TypeScript returned exactly the seven known tutorial joins. No live renderer adoption or full host behavior is claimed by these counts. All ticket artifacts remain preserved; no cleanup, Rust launch, modifying git operation or ticket closure was performed.
