# Terrain Acceptance in the Normal s Shell

## Execution

Run [verify-s-shell-terrain.mjs](./verify-s-shell-terrain.mjs) from the workspace root after the first-party local Hub and regular s Dev row are ready. [Dev setup and Space creation](./workflow-artifact-creation-host.md)

```sh
repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/verify-s-shell-terrain.mjs'
```

Node's syntax check passed. The runner reached its HTTP preflight, which reported `ConnectionRefused` from `http://127.0.0.1:6070/`. Chromium did not launch. Port 6083 served HTTP 200 for the scoped GIS preview. That is a separate entry point from the full s shell.

The default run signs into the local Hub through the regular Dev broker in a fresh Chromium profile. It creates a disposable private Atelier from Home, enters its Studio route, and waits for the real Workflow document to mount. The runner removes that Atelier through Home. A supplied Studio or Index URL reuses its existing Space.

## Registered editor and viewer in Catalogue

The authentic GIS component publishes `s.gis.gisterrain@1/*#editor` and `s.gis.gisterrain@1/*#viewer` at the same `semio / gis / 3d` breadcrumb. The host registers every app and sorts the palette by plugin and app ID. The old Catalogue made each path a single optional app, so the viewer replaced the editor in that tree. The Catalogue now keeps both roles as separate rows under the shared group, using exact plugin and app IDs in each drag payload. Single-app paths still use a draggable breadcrumb row. The neutral fixture proves the collapsed group stays lazy and the expanded group exposes both registered drag payloads.

The runner expands **Catalogue → semio → gis → 3d**, drags **Editor** to Workflow, asserts a fresh graph node with the exact registered GIS app and instance IDs, then double-clicks that node. `spawnApp` adds the node to the workflow. `openInstance` emits `OpenPluginInstance` with the stored plugin and app identity. Flow's double-click and context-menu actions resolve a widget's `params.instanceId`. The Space Workflow producer now writes that field from `node.id`, so those actions can address the specific node without relying on selection. [Flow double-click handler](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🎯️flow-pick-spotlight/🟦️.ts) · [Workflow snapshot producer](../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🔀️workflow/🦀️.rs)

The Rust projection and React resolver tests share a neutral Workflow snapshot with the expected open target. An independent Ajv check rejected the original `nodeId` field and passed against the corrected field. The production React resolver also returned the expected instance ID when run directly with that snapshot. The native test queued for compiler-image space and was cancelled before compilation to give the full s build priority. The separately admitted renderer test was waiting on the shared project graph at the last check. [Neutral opening fixture](../../../../../../../🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🧫️fixtures/🚪️workflow-opening/🔣️.json)

The runner checks the rendered Terrain window's `data-surface-app-id` and `data-surface-role`, then uses the example picker, which follows the focused program. It chooses the Terrain **Demo** example, checks for **Institut de Botanique**, clears overlays, checks the published action and verifies that Undo and Redo restore and clear the same feature list. It undoes the edit before checking Viewer. The detailed elevation and overlay exchange checks live in the separate scoped preview test. [Scoped Terrain browser exchange](./verify-terrain-exchange-browser.mjs)

## Viewer and shared document identity

The Workflow Open Instance effect carries the document opening reference separately from the node ID. The browser attaches that existing document through `openDocument` before it publishes the spawned surface. The mounted window exposes the owning app, role, Space ID and document ID. Browser-actor windows, panels and check-in follow the focused document while the primary session stays in Space. [Open Instance shell effect](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🌱️spawn-and-open-instance-effects/🟦️.ts) · [Spawned window owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📋️panel-hosting/🟠️mode-windows/🟦️.tsx)

The role controls, chip and keyboard shortcuts resolve the focused program. The existing document-surface preparation restores the archive into the target app, and attachment admits only the exact predecessor client of the same document. After attachment the shell replaces the surface inside the same Workflow node and retires the old app. A stale request cannot replace an unrelated mounted owner or take focus from a sibling. The acceptance runner checks the app and document identity across Editor and Viewer and checks that Viewer lacks the raise tool. [Role controls](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔎️command-bar/🔶️scope/🟦️.tsx) · [Spawned role switch](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/👁️switch-spawned-role/🟦️.ts)

The neutral mounted-role fixture covers Editor → Viewer → Editor for a document with an ID distinct from its Workflow node. A direct run of production preparation, panel replacement and DOM identity functions passed the fixture with Ajv. The renderer test also covers stale and cancelled attachment attempts. Full browser acceptance requires a persisted document in a disposable Space with the existing document authority. [Mounted-role fixture](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪟️spawned-program-session/🧫️fixtures/👁️mounted-role/🔣️.json)

## Persisted Terrain acceptance

The browser check reloads the full Studio page after a reversible Terrain edit, finds the same Workflow artifact and node, opens the Terrain window again, and compares document identity, overlay labels, and History. It then switches the same document from Editor to Viewer and back. The default run creates a disposable Space. An explicit Index URL uses the same artifact operations after creating Terrain through the Space Index.

The expanded mounted-role fixture passed the production ownership guard, failed-attachment rollback, sibling focus and window identity checks in a direct Bun run. The scoped TypeScript parse and whitespace checks passed. The full renderer type check reported 33 diagnostics outside the edited shell and fixture test files. Port 6070 still refused connections when the shell changes were checked.
