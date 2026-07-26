# Aggregator end-to-end verification (2026-07-26)

## Result

React mit-bestand Aggregator on `http://127.0.0.1:6023/` boots and renders the seeded Abbau Aufbau object mesh.

## Evidence

- Title: `Entwerfen mit Bestand · Aggregator`
- Scene: `instanceCount: 1`, `seed-left-001`, `revealIndex` **omitted** (not `null`)
- Reveal cutoff: `puzzle3d-fill: 0`
- Instance root: `visible: true`, `rootVisible: true`
- GLB: `/mesh/hexagonal-cut-concrete-forest-left.glb` → 200, 86112 bytes, `model/gltf-binary`
- `GlbInstanceMesh`: 57 meshes, `sceneVisible: true`, `rootVisible: true`
- Screenshot: `aggregator-viewport.png` (Perspective pane shows the 3D object on the floorplan)
- Full dump: `verify-aggregator-e2e.json`

## Regression tests

- `vitest … -t isRevealCutoffHidden`: 2 passed
- `cargo test -p puzzle-plugin seeded_objects_omit_reveal_index…`: 1 passed

## Root cause (already fixed in this ticket)

JSON `revealIndex: null` + boot cutoff `0` hid every ordinary instance (`null < 0` → false). Plugin now omits the key; host uses nullish checks.
