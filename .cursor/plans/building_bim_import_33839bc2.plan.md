---
name: Building BIM Import
overview: Add canonical BIM class typologies to the aec.building model definition, teach the BREP importer to classify imported STEP solids by their presentation-layer name, and regenerate the concrete-forest-left play fixture so its Building pane shows the layer-classified BIM model.
todos:
  - id: typologies
    content: Add canonical BIM class typologies (Column, Beam, Slab, Wall, Roof, Foundation, Stair, Ceiling, Railing, Door, Window) under cad/asset/modelDefinition/aec.building/typology/ with building.building.<name> ids and styles
    status: completed
  - id: importer
    content: Add STEP presentation-layer parser, BIM_LAYER_TYPOLOGY map, typology option on modelFromImportedBrepSolid, and importStepBimToModelSpace in cad/js/kernel/brepjs/index.ts
    status: completed
  - id: fixture-gen
    content: Extend the gated fixture generator to emit a combined shape+building ModelSpace (shape-first) and regenerate hexagonal-cut-concrete-forest-left.model.json
    status: completed
  - id: tests
    content: Extend brepjs and play tests to assert BIM solid counts, per-typology classification, and Building-pane object listing
    status: completed
  - id: verify
    content: Run the generator and verify in CAD Play that the Building pane renders the classified BIM model and document
    status: completed
isProject: false
---

# Building BIM Import

## Context

- New asset: [`hexagonal-cut-concrete-forest-left-bim.stp`](compose/fixture/kit/folder/abbau-aufbau/hexagonal-cut-concrete-forest-left-bim.stp). It contains 12 `MANIFOLD_SOLID_BREP` solids grouped by `PRESENTATION_LAYER_ASSIGNMENT` into 3 layers: `Slab` (1), `Beams` (8), `Column` (3).
- [`aec.building/modelDefinition.json`](cad/asset/modelDefinition/aec.building/modelDefinition.json) currently declares `kinds: ["action","typology"]` but defines **no** typologies, so the Building pane is empty.
- The current importer [`importStepBrepToModelSpace`](cad/js/kernel/brepjs/index.ts) ignores STEP layers and tags every solid `spatial.shape.kernel.solid`; [`modelFromImportedBrepSolid`](cad/js/kernel/brepjs/index.ts) hardcodes that typology.
- Objects render/list in a pane only when their typology is owned by the pane's model definition (`listModelObjectsForModelDefinition` -> `listTypologiesForModelDefinition`).
- Fixtures are plain `ModelSpace.toJSON()` (`spatial.modelspace/v1`); selecting a shape asset runs `modelsFromCadJson` which loads **every** model in the space, so co-locating shape + building models in one fixture is the cleanest "keyed" wiring (no play code change).

Confirmed decisions: canonical BIM set; building model auto-loads into the Building pane when "Concrete forest (left)" is selected.

## 1. Add canonical BIM typologies to `aec.building`

Create `cad/asset/modelDefinition/aec.building/typology/<Name>/typology.json` for the canonical set, each `schema: "spatial.typology"`, `primitiveKinds: ["solid"]`, a `style` block (color/edgeColor/opacity, mirroring [energy Roof](cad/asset/modelDefinition/aec.building.energy/typology/Roof/typology.json)), and id `building.building.<name>` (matches the `energy.energy.*` / `structure.structure.*` base-definition convention):

- `Column` -> `building.building.column`
- `Beam` -> `building.building.beam`
- `Slab` -> `building.building.slab`
- `Wall` -> `building.building.wall`
- `Roof` -> `building.building.roof`
- `Foundation` -> `building.building.foundation`
- `Stair` -> `building.building.stair`
- `Ceiling` -> `building.building.ceiling`
- `Railing` -> `building.building.railing`
- `Door` -> `building.building.door`
- `Window` -> `building.building.window`

Typologies are import-only (no construction action/interaction needed), so `actions`/`interactions` are omitted/empty. `kinds` already includes `"typology"`.

## 2. Layer-aware BIM import in [`cad/js/kernel/brepjs/index.ts`](cad/js/kernel/brepjs/index.ts)

In the `StepBrepImport` region:

- Add a STEP layer parser `stepPresentationLayers(stepText)` returning the global declared order of `MANIFOLD_SOLID_BREP` entities (from the `ADVANCED_BREP_SHAPE_REPRESENTATION` items list, e.g. `#26`) and each solid's layer name from `PRESENTATION_LAYER_ASSIGNMENT('<Layer>',...,(#refs))`.
- Add `BIM_LAYER_TYPOLOGY` map normalizing layer names to typology ids (case/plural-insensitive): `slab->building.building.slab`, `beam(s)->building.building.beam`, `column(s)->...`, plus wall/roof/etc.
- Give `modelFromImportedBrepSolid` a `typology` option (default keeps `spatial.shape.kernel.solid`).
- Add `importStepBimToModelSpace(stepText, options)` that imports solids (`validSolidsFromImportedShape`), maps each solid (in declared order) to its layer -> typology, builds one `Model` with per-object typologies, applies `scaleModelPositions`, and links it under `aec.building`.

Solid<->layer matching uses the declared `MANIFOLD_SOLID_BREP` order, which equals the OCCT `iterTopo("solid")` order; validated by the count assertions below. (If runtime shows a mismatch, fall back to centroid matching via `getBounds`.)

## 3. Multi-model fixture serialization + regeneration

- The combined space serializes via existing `ModelSpace.toJSON()` (round-trips through `ModelSpace.fromJSON`); `inlineModelSpaceFixtureJson` (single-object) is not used here.
- Extend the gated generator test (`CAD_GENERATE_STEP_FIXTURES=1`, ~line 3842) in [`cad/js/kernel/brepjs/index.ts`](cad/js/kernel/brepjs/index.ts): for concrete-forest-left, build a space with the **shape** model (from `.stp`, under `spatial.shape`) and the **building** model (from `-bim.stp`, under `aec.building`), emit with the `models` array ordered **shape-first** so the Shape pane stays active, and write [`cad/asset/play/hexagonal-cut-concrete-forest-left.model.json`](cad/asset/play/hexagonal-cut-concrete-forest-left.model.json). Right keeps shape-only.
- Run the generator to regenerate the fixture.

## 4. Wiring (no play code change)

Because [`modelsFromCadJson`](cad/js/renderer/play/index.tsx) loads all models from the space and `ensurePlayQuadModelSlots` keeps the `aec.building` slot, selecting "Concrete forest (left)" auto-populates the Building pane with the classified BIM model; other assets leave the Building pane empty. No change to `SHAPE_ASSETS` or `handleShapeAssetChange`.

## 5. Tests (extend existing files only)

- In [`cad/js/kernel/brepjs/index.ts`](cad/js/kernel/brepjs/index.ts) tests: import the bim file via `importStepBimToModelSpace`; assert 12 solids, per-typology object counts (`building.building.slab`=1, `beam`=8, `column`=3), scaled coordinates (`<100`), and positive volumes after `syncSolidsFromModel`.
- In [`cad/js/renderer/play/index.tsx`](cad/js/renderer/play/index.tsx) tests: assert the regenerated concrete-forest-left fixture yields a non-empty `aec.building` model whose objects are listed by `listModelObjectsForModelDefinition(model, "aec.building")`.

## 6. Verify at runtime

Reload CAD Play, select "Concrete forest (left)", confirm the Building pane renders the BIM model with distinct per-class colors and that Slab/Beam/Column nodes appear in the document tree (confirm via console/log per repo rules).