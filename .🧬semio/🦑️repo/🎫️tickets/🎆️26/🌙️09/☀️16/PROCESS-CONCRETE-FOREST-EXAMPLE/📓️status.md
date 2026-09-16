# Status: Process Concrete Forest Example

**Status:** open 2026-09-16
**Goal:** Add the concrete forest example to process and apply steps using all machines available for concrete.
**Bookkeeping:** manual on disk — repo MCP failed to connect (connection closed) at session start.

## Findings so far
- `process3d` ships two example documents (`timber-beam-joinery`, `drilled-plate`) as hand-minted DSL text; the concrete catalog (`🧩️extensions/🧱️concrete`, mirrored in `🧬️schema/🦀️.rs`) carries 7 machines / 7 capabilities but no example uses any of them.
- The kernel replay (`💡️inferences/🦀️.rs::solid_for_spec`) only rebuilds `Box`/`Cylinder`/`Sphere` across sessions; `ImportedSolid` handles are session-ephemeral and `ImportedMesh` has no B-Rep, so neither can carry the concrete forest piece into a persisted example.
- The piece exists as an exact STEP export (`♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.stp`, 78 KB, one solid, 57 planar faces) and the brep kernel already reads STEP (`Brep::import_step_sync`).

## Progress (14:57)
- Built: `WorkingSolid::Reference` + shipped reference solids, the example leaf/asset/registration, three kernel fixes in
  the STEP reader, two in the validator; native laws green (see [the report](./📓️concrete-forest-example-2026-09-16.md)).
- Running: `@semio-tech/process-plugin:component-dev` wasm build for the browser gate on the react lane.
