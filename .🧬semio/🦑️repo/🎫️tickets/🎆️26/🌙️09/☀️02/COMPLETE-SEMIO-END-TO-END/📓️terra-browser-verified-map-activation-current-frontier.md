# Terra Browser Verified Map Activation — Current Frontier

## Verdict

**RED — no authenticated browser Map open/render/persisted-edit journey currently exists.** Hub can issue a tightly bound `DocumentOpenPlanV1`, and the browser broker has a strict verifier for three target bodies. The two halves do not meet in production, and the verified raw component is not an executable browser module graph.

The shortest honest end-to-end slice is therefore: **Hub-selected Map plan → authenticated exact target/activation-closure bodies → browser-owned verification → private renderer handoff → existing Map actor/backbone attachment → one persisted `patchPositions` edit.** A change limited to adding the existing three target routes is valuable verification plumbing, but must not be reported as component activation or Map rendering.

This was source inspection only; no browser or native build was run.

## Concrete Map target and reusable seams

The frozen trusted profile already singles out the required editor target: `gis`, artifact `s.gis.gismap`, surface/app `s.gis.gismap@1/*#editor`, window `gis2d-main`, `wasm`, writable. The assertion is at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4031-4034`; the frozen fixture says the same at `🌎️hub/🧪️fixtures/🗺️gis-map-frozen-binding-v1/🔣️.json:13-15`.

| Journey segment | Reusable current seam | Current state |
| --- | --- | --- |
| Authenticated Map selection | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2174-2268` authenticates, revalidates, resolves from `openable_catalog`, issues a TTL-bound plan | Works at plan issuance. `requestedSurfaceId` is already optional in `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:928-934,1071-1080`; the Hub resolver accepts its absence at `bin.rs:2203-2207`. |
| Verified package bytes | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:183-235,390-525` retains and validates the selected component and descriptor bytes | Available server-side; no route exposes them under the selected plan. |
| Browser verification | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:564-575,656-735` allows only document-scoped `manifest`, `component`, and `descriptor`, checks the plan projection, SHA-256/BLAKE3, byte lengths and descriptor relations | Implemented client-side, but unreachable against today’s Hub. |
| Socket and persistent document loop | `…/backbone-worker.ts:824-838`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3607-3639` waits for the socket actor, opens directory scope, and attaches the plugin backbone | Reusable *after* a verified actor exists. The current branch deliberately emits `renderer-unavailable`. |
| Map rendering and one durable edit | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:820-830` renders retained bodies; `…/🎮️commands/🗺️features/🦀️.rs:42-58` maps `patchPositions` to artifact mutations | Real native component behaviour, but no verified browser execution reaches it. `patchPositions` is a single-document proof edit, avoiding the separate parent-plus-child CreateRegion durability frontier. |

## Present breaks, in execution order

1. **The production Hub has no target-body routes.** Its router has only status/checkpoint, `POST /open-plan`, `POST /socket-grants`, and socket routes at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6162-6167`. The browser verifier issues exact `POST /spaces/{space}/documents/{document}/execution-target/{manifest|component|descriptor}` calls (`…/backbone-worker.ts:564-575`). Each must derive the selected package again from the authenticated scope/intent and revalidation state, then provide only the exact asset—never accept package id, digest, generation, path, or arbitrary URL from the browser.

2. **The local Home relay rejects those calls before Hub.** `localRelayUpstreamPath` permits `open-plan` and `socket-grants`, but not the three target paths: `🌎️hub/📦️packages/🦀️rust/📜️script.ts:332-343`. Add three exact, query-free `POST` patterns; do not add an `execution-target/*` or directory-wide escape hatch.

3. **The shell cannot initiate the current verifier contract.** `PersistenceBinding` has `installedTarget?: DocumentExecutionTargetLeaseFieldsV1` at `🧰️framework/🛍️products/💻️os/🟦️.ts:583-585`; the broker rejects absent `installedTarget` before it requests a plan (`…/backbone-worker.ts:774-787`). Yet `ShellHost.openDocument` supplies `{ kind: "hub", baseUrl, spaceId, surface }` at `…/ShellHost/🟦️.tsx:3591-3601`. `surface` is not the current binding field and it is a caller-derived selector. For this known unique Map profile, initiate with scope/client id only and omit `requestedSurfaceId`; let Hub select the Map editor, then construct the receipt-free fields only from the returned plan and verified bodies.

4. **Three verified bodies cannot activate the current browser Map runtime.** `DocumentOpenPlanV1` binds raw component + descriptor identities only (`…/directory/🧬️schema/🟦️.ts:943-950,992-1007`). The actual static GIS JCO bridge imports its sibling wrapper (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌍️gis/🌉️bridge.js:124-134`), which in turn fetches/compiles WASM (`…/semio_s_plugin_gis_component.js:4819-4825`) and fetches `semio_s_plugin_gis_component.core.wasm` (`…:11920-11924`). The descriptor records a core-WASM hash (`…/🌍️gis/🔣️.json:161-166`), but that core byte, bridge, wrapper, and their module dependencies are absent from the plan/package identities and the trusted catalog’s retained executable payload.

   The static `loadPluginModule(pluginId, moduleUrl)` route is therefore not an admissible shortcut: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1061-1064` accepts a caller-provided URL, and `fetchDescriptorManifest` derives a sibling JSON URL and checks only manifest owner/apps (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:108-131`). Neither verifies the plan’s descriptor bytes nor the component/closure hashes.

5. **The verified lease has no execution handoff.** It privately owns and zeroes the verified buffers at `…/backbone-worker.ts:503-548`; its only public projection has no bytes. After the socket grant it is retained internally and emits `renderer-unavailable` at `…:834-838`. There is no bridge to a `ShardClient` actor, no renderer start, and no `loadAppDocumentPack` receiver associated with such an actor.

## Smallest runnable design

Use the existing plan as the authority, not an installer-provided target, a static directory scan, or a caller module URL.

1. Shell sends the normal scope/client request to the Backbone worker without `installedTarget` and without a requested surface. Hub’s existing authenticated resolver selects the sole writable Map surface and returns the verified `DocumentOpenPlanV1` bytes.
2. Factor the Hub’s current post-authentication resolution into an exact target-body helper. The three existing client request shapes can return the lease manifest, raw component, and raw descriptor from `VerifiedTrustedCatalog`; every response must re-authenticate/revalidate and must match the eventual plan fields. The browser retains the present cross-check before requesting a socket grant.
3. Add a **trusted browser activation-closure record** to the package/catalog and plan projection: entry bridge plus every bridge/wrapper/core-WASM dependency, each named by a closed logical name, byte length, and hash. Hub serves only this selected closure. The browser-owned executor verifies every member, resolves imports privately, and starts the Map actor. A generic Component Model executor that consumes only the already verified raw component would also satisfy this requirement, but no such executor exists in current production source.
4. Transfer the resulting sealed execution capability directly from the verification owner to the renderer/shard owner through a private internal channel. Do not serialize it into `BackboneWorkerResponse`, `PersistenceBinding`, a UI action, or a `moduleUrl`. The public surface gets only actor/app state. This is the missing replacement for the present `loadPluginModule(pluginId, moduleUrl)` entry point.
5. Once the actor is live, reuse the existing socket/backbone sequence and invoke Map `patchPositions` with one valid positions array. Its command emits artifact mutations (`…/🎮️commands/🗺️features/🦀️.rs:52-58`), so a close/reopen proof can assert the changed document pack rather than merely transient camera/config state. It also avoids claiming the not-yet-durable typed CreateRegion group path is atomic.

The Home process owner reports the current physical Space component descriptor is stale and the plugin worker’s shard-worker URL needs a harness alias. Those are independent run-harness concerns; neither substitutes for the Map closure admission or private execution handoff above.

## First five executable laws (proposed; not run)

1. `hub_document_open_serves_only_selected_map_target_bytes` — authenticated Map intent with no requested surface receives a plan plus exact manifest/component/descriptor bodies; their identities equal the plan. A stale catalog, changed descriptor, unauthorized member, query, or package/digest selector is rejected.
2. `home_relay_allows_only_document_execution_target_posts` — local relay forwards the three exact query-free asset paths and rejects a fourth name, a query, traversal, and a non-POST request.
3. `browser_map_open_derives_target_from_verified_plan_not_binding_or_url` — opening with scope only emits `open-plan`, verifies the selected bodies, and rejects a forged `installedTarget`, mismatched Map surface, or a caller `moduleUrl` before actor creation.
4. `browser_map_activation_rejects_unbound_bridge_closure_before_import` — alter any bridge/wrapper/core-WASM byte or dependency identity; the private executor must fail before import/instantiation. The success case proves every loaded member is in the plan-selected closure, not merely that the raw WIT component and descriptor verified.
5. `verified_map_editor_renders_then_patch_positions_survives_reopen` — the authenticated Map editor renders `gis2d-main`, `patchPositions` produces and receives an acknowledged artifact mutation, and a fresh open restores the changed position. It must assert no `renderer-unavailable` status and no public module URL/closure bytes.

## Ownership boundary

The next Home executor implementation should own the browser-only private handoff and composed test (laws 3–5), in coordination with Hub’s exact protected body routes/closure admission (laws 1–2). It must not represent the current static plugin-module directory, descriptor fetch, or a development harness alias as catalog authority.
