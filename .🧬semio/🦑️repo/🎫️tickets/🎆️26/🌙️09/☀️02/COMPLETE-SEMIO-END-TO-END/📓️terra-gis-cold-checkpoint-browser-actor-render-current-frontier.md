# GIS Cold Checkpoint to Closed Actor Renderer Frontier

## Verdict

The current private Session/reservation path can authenticate, load, and `describe` a closed GIS actor, but it cannot yet render a cold GIS checkpoint. The failure is not a missing React/canvas implementation: the real viewer produces a `tiled-map` component tree and the renderer already selects its real `TiledMapHost`. The missing production bridge is a scoped, retained checkpoint-pair hydration turn plus a typed actor-turn/patch bridge into `UiDocumentStore`.

This is source review only. No build or browser run was performed by this audit.

## Existing Real Endpoints

| Concern | Existing authoritative seam | Finding |
| --- | --- | --- |
| Cold document source | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:361-368,1887-1898` | Authenticated replication retains `currentPack`, `currentSpr`, and the frontier in the document `ArtifactState`. It is the only viable browser-side source for a first cold render. |
| GIS viewer output | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:35-75` | `GisMapViewer` renders the selected map body from `doc.snapshot`; it is intentionally read-only. |
| Actual map component | `.../👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🦀️.rs:51-53` | The viewer creates `TiledMapScene` and emits `scene_surface(..., ContractSurfaceKind::TiledMap, ...)`, not a fixture canvas. |
| Actor invocation ABI | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:652-662,1079-1155` | The selected component enters through async `reactor.poll(instance-open, …)`, which returns revisioned `ui-patches` and a lifetime receipt. It has no public `render` export. |
| Patch application | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:448-458` | `UiDocumentStore.applyPatch` is the existing transactional renderer authority. Its `Interpreter` already maps `tiled-map` to `TiledMapHost`; the closed child does not currently feed it. |
| Dedicated child | `.../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:16-120` and `🧵️worker.ts:22-35` | The child authenticates its bundle and has bounded local WASI diagnostics, but its host port denies all stateful effects and it exposes only generic `invoke`. |

## Immediate Hydration Blocker

`instance-open` carries `config` and `assets` in WIT, and conversion faithfully preserves them (`plugin/⚛️reactor/🦀️.rs:2320-2325`). The live reactor handler then destructures `Event::InstanceOpen { request, app_id, actor, quotas, .. }` at `plugin/⚛️reactor/🦀️.rs:1377-1403`: both values are ignored. It opens lifecycle metadata and sets the actor, but never calls the real document loader.

The real loader is `plugin_runtime::plugin_load_document_pack`; checkpoint restore uses it at `plugin/⚛️reactor/📸️checkpoint/🦀️.rs:95-100`. The legacy shell calls an equivalent direct runtime path, but that is not an export of `world actor`. Thus passing the pair as generic `assets`, or enabling the current generic `document-read`, will merely leave the GIS viewer at its default snapshot. It cannot produce a cold checkpoint render.

## Smallest Coherent Production Slice

Add a first-class, host-owned cold-checkpoint event/record to the actor lifecycle rather than making `assets` ambiguously mean document state:

1. The Backbone reservation retains a private `DocumentCheckpointPairV1` only after the existing authenticated bootstrap completes. It binds `runtimeKey`, scope, selected actor id, reservation generation, pack/Spr byte caps and digests, plus the exact baseline frontier/revision. It must not be serializable from plugin input or caller-supplied JS.
2. After `instance-open` has a valid lifecycle receipt, the parent supplies exactly one typed hydrate event to `reactor.poll`. The Rust reactor validates the lifetime/generation then performs the existing `plugin_load_document_pack` path. A replay, stale pair, cross-scope pair, mismatched frontier, or partially present pair is a fault and closes the reservation.
3. The same parent-owned turn driver decodes the returned WIT `ui-patches`, requires the exact runtime key, actor generation, selected surface and lifecycle/patch receipt, and applies each through the existing `UiDocumentStore`. Only a successful transactional patch may make the surface visible; its acknowledgement is carried in the next actor turn.
4. The reservation owns the pair, poll driver, child, and patch acknowledgement until one terminal close. Session replacement, document abort, descriptor mismatch, pair fault, or child fault must wipe/drop the pair, stop pending turns, terminate the child, and discard late patches.

This requires no general child authority. The cold viewer first slice should retain only the pure clock/log/trace import, lifecycle polling, the private hydrate record, and patch delivery. Keep `document-write`, storage, HTTP, blobs, cache, registry, actions, windows, media, extension, jobs, and generic `document-read` denied. If future component code genuinely reads its snapshot through `document-read`, bind it to one host-minted opaque document handle and one fixed read lane; do not expose the WIT `doc`/`lane` arguments as document selection authority.

## First Executable Laws

Use a materialized GIS component/descriptor/bundle and an authenticated GIS pack/Spr pair; an ESM fixture or synthetic canvas cannot qualify this seam.

1. Exact Session + selected actor + canonical cold pair produces a `tiled-map` UI patch accepted by `UiDocumentStore`, resulting in the real `TiledMapHost`; the next poll carries the matching patch acknowledgement.
2. A pair from another space/runtime key, stale bootstrap frontier, mismatched pack/Spr digest, or a truncated pair produces no UI mutation, closes the child, and leaves child capacity at zero.
3. Wrong actor generation/surface, invalid descriptor/describe result, malformed patch, or invalid patch receipt preserves the pre-turn UI document and produces no renderer fallback.
4. Socket replacement, rebootstrap, close, or document abort while hydration/poll is pending retires the pair and child; late turn results cannot mutate the UI or invoke a host effect.
5. Viewer attempts at `document-write`, storage, HTTP, blob, cache, registry, action, extension, or job imports receive a typed denied fault with zero durable/remote effect.

The final process law should run the real Hub bootstrap and real Chromium Session path, then assert a real `TiledMapHost` receives the GIS checkpoint. A prior native materializer-to-Chromium law can establish the actor/pair/renderer contract without waiting for the Hub binary, but it must still use genuine GIS output.

## Nonclaims

The current child `describe` acceptance, private Session reservation, `renderer-unavailable` status, and synthetic closed-actor fixtures do not demonstrate cold GIS rendering, render-patch authority, map persistence, or editing. The later editor write path must remain behind the fixed three-Store durable journal/DB witness owner; it is not a capability to add to the viewer host port.
