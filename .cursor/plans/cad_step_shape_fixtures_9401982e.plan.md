---
name: cad step shape fixtures
overview: Add a raw-STEP topology importer to the brepjs kernel, use it to generate two new `spatial.shape` model-space fixtures from the hexagonal-cut-concrete-forest `.stp` files, and wire them into the CAD play harness end-to-end.
todos:
 - id: ticket
   content: Read repo://goals and open a ticket for the STEP-shape fixture import
   status: completed
 - id: importer
   content: Add raw-STEP topology importer (importStepBrepToModelSpace) to brepjs kernel index.ts
   status: completed
 - id: gen-validate
   content: Extend kernel in-file Vitest to import both .stp, validate (volume/faces/round-trip), and emit fixtures behind env guard
   status: completed
 - id: script-launch
   content: Add 'fixture' command to kernel script.ts and register it in launch.json
   status: completed
 - id: play-wiring
   content: Add two imports and two SHAPE_ASSETS entries in play index.tsx
   status: completed
 - id: e2e
   content: Run generation+validation, then dev:cad to confirm both shapes render; fix kernel/import errors
   status: completed
 - id: close
   content: Close ticket with summary and touched files
   status: completed
isProject: false
---

# CAD: Import Two STEP Shapes as Play Fixtures

## Goal

Create two new CAD fixtures by importing the `spatial.shape` geometry from:

- `compose/fixture/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left.stp`
- `compose/fixture/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-right.stp`

and make them load and render end-to-end in the CAD play harness, fixing kernel/import errors along the way.

## Key facts established

- Each `.stp` is one `MANIFOLD_SOLID_BREP` / `CLOSED_SHELL` with 57 `PLANE` faces and degree-1 `B_SPLINE_CURVE_WITH_KNOTS` edges (straight lines) -> exact polyhedron, so topology extraction is lossless.
- CAD fixtures are `spatial.modelspace/v1` JSON under [cad/asset/play/](cad/asset/play/), model id `spatial.shape`, with per-object inline primitives (`vertex` -> `edge` -> `wire` -> `face` -> `shell` -> `solid`). On load, `Model.fromJSON` + `materializeInlineObjectPrimitives` lift them into the kernel graph; `syncSolidsFromModel` -> `sewShells`/`solidFromShell` rebuilds the brep; `mesh()` tessellates for display.
- The brepjs kernel imports `getFaces`, `getEdges`, `getVertices`, `vertexPosition`, `getSurfaceType`, `getCurveType`, `normalAt`, `importSTEP` from `brepjs` (see [cad/js/kernel/brepjs/index.ts](cad/js/kernel/brepjs/index.ts) lines 10-62) - everything needed for topology extraction.
- Existing `importStepToModelSpace` (line ~2596) only handles spatial-UDA STEP; raw Rhino BREP falls through with empty topology. This is the core gap.
- OpenCascade WASM only initializes in the Vite/Vitest env (the top-level `?url` wasm import breaks under plain `bun`). Generation + validation must run through the kernel's in-file Vitest tests.

## Approach (chosen)

Build a real, reusable raw-STEP -> `spatial.shape` topology importer in the kernel, generate the two JSON fixtures with it, validate via the kernel test, and add them to the play navbar. No brep-blob schema changes are needed because the solids are planar polyhedra that round-trip exactly through the existing topology path.

## Steps

### 1. Ticket setup

- Read `repo://goals`, then open a ticket via repo MCP (`ticket_open`) titled like "Import STEP Shapes as CAD Fixtures". Keep any temp logs inside the ticket folder.

### 2. Kernel: raw-STEP topology importer

In [cad/js/kernel/brepjs/index.ts](cad/js/kernel/brepjs/index.ts), add a new region (e.g. `//#region StepBrepImport`) with `importStepBrepToModelSpace(stepText): Promise<ModelSpace>`:

- `await ensureInit()`, `importSTEP(new Blob([stepText]))`, unwrap the solid(s).
- For each solid, walk `getFaces` -> per face collect its `getEdges`; per edge read endpoint vertices via `getVertices`/`vertexPosition` (`curveStartPoint`/`curveEndPoint` as fallback); dedupe vertices by quantized position into `VertexRecord`s; build `line` `EdgeRecord`s, order edges into a closed `WireRef` loop (connect by shared vertex), build planar `FaceRecord` (optionally store `plane` surface from `normalAt`), one `ShellRecord` over all faces, one `SolidRecord` with `shellIds`.
- Create one `spatial.shape.primitive` object referencing the solid; assemble a `Model`, link into a `ModelSpace` under `spatial.shape`.
- Make raw import the fallback inside `importStepToModelSpace` when `parseSpatialUdaPayloads` yields no `spatial.modelspace` (single clean entry point), or expose as a sibling public function next to `importStepToModelSpace` (line ~2895).
- Verify rebuilt solid is valid (`syncSolidsFromModel` -> `sewShells` succeeds, positive volume); add healing fallback if sew fails.

### 3. Generate + validate fixtures (Vitest, no new files)

Extend the existing in-file kernel test block in [cad/js/kernel/brepjs/index.ts](cad/js/kernel/brepjs/index.ts):

- Read both `.stp` via Node `fs`, run `importStepBrepToModelSpace`, assert: one solid, 57 faces, `measureVolume > 0`, and a successful tessellation.
- Behind an env guard (e.g. `CAD_GENERATE_STEP_FIXTURES`), serialize each imported model space to `spatial.modelspace/v1` JSON and write:
  - `cad/asset/play/hexagonal-cut-concrete-forest-left.model.json`
  - `cad/asset/play/hexagonal-cut-concrete-forest-right.model.json`

### 4. Script + launch wiring

- Add a `fixture` command to [cad/js/kernel/brepjs/script.ts](cad/js/kernel/brepjs/script.ts) that runs the kernel Vitest with `CAD_GENERATE_STEP_FIXTURES` set (reuses the proven WASM env; obeys "only script.ts" rule).
- Register the new command in [.vscode/launch.json](.vscode/launch.json) following the existing cad grouping/order.

### 5. Play harness wiring

In [cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx):

- Add two imports next to lines 879-884.
- Add two `SHAPE_ASSETS` entries (lines 919-926) with labels "Concrete forest (left/right)" and unused hotkeys.

### 6. End-to-end validation + fixes

- Run the kernel `fixture` command to generate + validate (must pass: volume>0, 57 faces, round-trip).
- Launch `dev:cad` play, select both new fixtures, confirm they render as the hexagonal-cut shapes (add `[DEBUG]` logs if needed, remove after). Fix any kernel/import/sew errors at the root.

### 7. Close ticket

- `ticket_close` with summary and all touched files: kernel `index.ts`, kernel `script.ts`, `launch.json`, play `index.tsx`, the two new `cad/asset/play/*.model.json`.

## Notes / decisions

- Fixtures use the standard `spatial.modelspace/v1` JSON format and appear in the play navbar, consistent with existing shape fixtures (no schema or brep-blob changes).
- The `.stp` files (compose fixtures) are only read as importer input via explicit paths; generated CAD fixtures contain pure geometry and no compose coupling, so technologies are not mixed.
