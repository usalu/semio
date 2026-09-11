# Contributions scoped from published examples

Date: 2026-09-11. Serve: `http://127.0.0.1:6018/?plugin=generation3d` (not restarted).

## What is proven

`ReadDocument` is still genesis (`pack 873 / spr 280 / ops 92`, `applied=[]`). The host now scopes from `manifest.examples[].artifactJson` (8 generation3d graphs).

Live `[DEBUG] contributions scoped pack`:

- **248 635 chars**, one pack, page 0 of 1
- `hasManifestJson: true`
- `hasPolygon: true`
- kinds include `brep.curve.polygon`, `math.vector`, `brep.solid.extrude`, `brep.surf.planarFaceWire`, …
- `setContributions` settled (2 frames, 1 effect = `flowEvalTick`)

That is under the 262 144-byte / 64-page ceiling. The 13-plugin 397 921-char dump was not sent.

After the push, preview is **no longer** `flow.extension-not-contributed`. At 40 s:

- numbers evaluated (`height=6`, `radius=0.5`, `sides=6`)
- `evalLen=779`
- widgetErrors: `unknown kind: brep.curve.polygon`, `unknown kind: math.vector`, `unknown kind: brep.solid.extrude`
- `meshes=0`

Flow window shows the 7-node Hexagonal Mushroom Column graph (hover chrome present). Official probe still settles at ~10 s with windows+canvases and `meshes=0` because it does not wait for eval.

## Host changes this turn

- `exampleArtifactSources` + viewer `#` stem fallback
- camelCase kind segments (`brep.surf.planarFaceWire`)
- unresolved ReadDocument → published examples
- removed the 250 ms ReadDocument stall that raced refresh generation
- refresh `requestedEffects` (early `flowEvalTick`) run **after** the contributions push
- setContributions `flowEvalTick` re-arm deferred via `queueMicrotask`
- invoke dispatch uses `loadedPluginsRef`

## Tests

- `@semio-tech/framework:test` example/scope subset: **6 passed**
- `@semio-tech/framework-renderer-react:test` window-fault subset: **7 passed**

## Next blocker (not host scoping)

Guest `FlowHost::evaluate` answers `unknown kind` unless tests call `flow_operators::installed()`. Runtime `setContributions` folds `manifestJson` into the contributed registry; eval still does not treat those operators as known kinds / `PendingExtension`. That is guest catalog vs contributed registry — rust restage required. Do not raise ingress.

## Not done (goal stays open)

- meshes > 0, hover/select/orbit on preview
- all 9 examples live
- generate-mode quality
- viewer 8 commands
- assembly app mount
- wgpu windows + geometry
