---
name: Core Kernel Decoupling
overview: Strip all model-definition assets and geometry/transformation logic out of cad/js/core code so the core only defines interfaces, calls the kernel, and consumes pure-data assets through a registry. The brepjs kernel implements every geometry primitive; transformations become declarative data.
todos:
  - id: ticket
    content: Read repo://goals and open a ticket associated with the best goal before editing.
    status: pending
  - id: registry
    content: Replace ModelDefinitionAssets import.meta.glob in core/index.ts with a registry + registerModelDefinitionAssets API; accessors read the registry; clear owner/folder caches on register.
    status: pending
  - id: assets-file
    content: Create cad/js/core/assets.ts with the import.meta.glob blocks calling registerModelDefinitionAssets; add to core tsconfig include; side-effect import from kernel, query, play apps, renderer, and the core Tests region.
    status: pending
  - id: geometry-kernel
    content: Add geometry primitives (faceNormal/Centroid/areaEstimate, solidFaceIds, fuseSolidsToExternalFaces, projectPointOnScalarAxis, scalarTopOnAxis, clampPointAlongDirection) to SpatialPreviewKernel; move implementations into brepjs kernel; replace core call sites with preview.*.
    status: pending
  - id: transformation-schema
    content: Extend spatial.transformation/v1 with a declarative derive block (fuse/hull/rules/opening/ensure) in schema + core parser; encode energy from_geometry rules in its transformation.json.
    status: pending
  - id: transformation-engine
    content: Replace applyEnergyFromGeometryTransformation + classify/role helpers + applyTransformation special-casing with one generic runTransformation engine that calls kernel primitives; remove all energy/shape literals from core code.
    status: pending
  - id: thread-kernel
    content: Thread preview kernel through applyTransformation, ModelSpace.transform, and query runTransformationCall.
    status: pending
  - id: dehardcode-ids
    content: Derive default model-definition from a manifest flag; move primitiveKinds/kernel-typology mapping into assets; remove energy/structure substring heuristics and SHAPE_MODEL_DEFINITION_ID/defaultGeometryKernelTypologyIds literals.
    status: pending
  - id: tests
    content: Extend core and kernel Tests regions for registry loading, kernel geometry primitives, and data-driven energy transformation; run vitest for core/kernel/query and confirm green.
    status: pending
  - id: close-ticket
    content: Close the ticket with a summary and all touched files.
    status: pending
isProject: false
---

## Goal

`cad/js/core` must contain only interfaces + orchestration. No asset imports, no geometry math, no model-definition-specific (`spatial.shape` / `aec.building.energy` / `energy.energy.*`) code. Geometry lives only in the kernel; transformations/actions/interactions are pure data.

## Current leakage (in `cad/js/core/index.ts`)

- `import.meta.glob("../../assets/modelDefinition/**")` baked into the `ModelDefinitionAssets` region (lines 5-89).
- `TransformationGeometry` region (lines 2589-2935): real geometry (`transformationFaceNormal`/`Centroid`/`AreaEstimate`, `fuseShapeSolidsToExternalFaces`, `facesAreContactPair`, `solidFaceIds`) + domain code (`EnergySurfaceRole`, `classifyEnergySurfaceRole`, `energyTypologyForRole`, `applyEnergyFromGeometryTransformation`) + `applyTransformation` hardcoding `"aec.building.energy.from_geometry"`/`"spatial.shape"`.
- Scattered vector math: `projectPointOnScalarAxis`, `scalarTopOnAxis`, `clampPointAlongDirection` (lines ~4425-4527).
- Hardcoded ids: `SHAPE_MODEL_DEFINITION_ID = "spatial.shape"` (1668), `defaultGeometryKernelTypologyIds` `spatial.shape.kernel.*` (~2011), `inferTypologyPrimitiveKinds` substrings `energy.energy.`/`structure.structure.` (1723).
- `applyTransformation` is called synchronously without a kernel from `ModelSpace.transform` (1373) and `cad/js/query/index.ts:1225`.

## Architecture target

```mermaid
flowchart LR
  assets["assets.ts (import.meta.glob)"] -->|registerModelDefinitionAssets| registry["core registry (data only)"]
  registry --> coreEngine["core: interfaces + generic engines"]
  coreEngine -->|SpatialKernel / SpatialPreviewKernel| kernel["brepjs kernel: ALL geometry"]
```

## Workstream A — Detach assets (register-entry)

- In `cad/js/core/index.ts` `ModelDefinitionAssets` region: delete all `import.meta.glob` calls. Replace with a registry holding the raw catalogs (typology/action/interaction/manifest/attribute/property/transformation/extension) plus `registerModelDefinitionAssets(catalogs)` and a reset. Keep the existing `modelDefinition*Catalog()` accessors but have them read the registry. Clear the `*OwnerByIdCache` and `modelDefinitionFolderIdMapCache` on register. Zero `assets/` references remain in `index.ts`.
- New file `cad/js/core/assets.ts`: holds the `import.meta.glob(...)` blocks (paths relative to this file) and calls `registerModelDefinitionAssets(...)` from `@cad/js/core` at import time. Add `assets.ts` to `cad/js/core/tsconfig.json` `include`.
- Side-effect import `@cad/js/core/assets` from every consumer: `cad/js/kernel/brepjs/index.ts`, `cad/js/query/index.ts`, the play apps' entry files, `cad/js/renderer`, and the core `Tests` region (`import "./assets"`). Confirm bun resolves the `@cad/js/core/assets` subpath; if not, add a subpath to core `package.json`.

## Workstream B — Geometry primitives to the kernel

- Extend `SpatialPreviewKernel` (in `SpatialKernelInterface` region) with the primitives core currently computes itself: `faceNormal(model, face)`, `faceCentroid(model, face)`, `faceAreaEstimate(model, face)`, `solidFaceIds(model, solidId)`, `fuseSolidsToExternalFaces(model, solidRefs)`, `projectPointOnScalarAxis`, `scalarTopOnAxis`, `clampPointAlongDirection`.
- Move those implementations from `cad/js/core/index.ts` into `cad/js/kernel/brepjs/index.ts` (`SpatialKernelMath`/`kernelGeometry` regions). Delete the core copies; replace in-core call sites with `preview.*`.

## Workstream C — Transformations as pure data

- Extend `spatial.transformation/v1` (`cad/schema/json/transformation.json` + `TransformationSpec` parser in core) with an optional declarative `derive` block (surface-classification): `fuse` source primitive kind, `hull` typology, ordered `rules` (axis dominance, min-dot, z-band max/min) → typology, `opening` attribute→typology, `ensure` typologies.
- Encode the energy rules in `cad/assets/modelDefinition/aec.building.energy/transformation/from_geometry/transformation.json` (roof/baseplate/slab/externalwall/window thresholds currently hardcoded in `classifyEnergySurfaceRole`).
- Replace `applyEnergyFromGeometryTransformation`, `EnergySurfaceRole`, `classifyEnergySurfaceRole`, `energyTypologyForRole`, `applyTransformationFallback`, and the special-case in `applyTransformation` with one generic `runTransformation(spec, source, preview)` engine that interprets `derive` and calls kernel geometry primitives. No `energy.energy.*`/`spatial.shape`/`from_geometry` literals left in core code.
- Thread the preview kernel through: `applyTransformation(spec, source, preview)`, `ModelSpace.transform(...)`, and `query`'s `runTransformationCall` (pass `ctx.preview`/`ctx.kernel`).
- De-hardcode remaining ids: derive the default model-definition from a manifest flag (e.g. `default: true`) instead of `SHAPE_MODEL_DEFINITION_ID`; require `primitiveKinds`/kernel-typology mapping to come from assets, removing the `energy.energy.`/`structure.structure.` substring heuristics in `inferTypologyPrimitiveKinds` and the `defaultGeometryKernelTypologyIds` literals (move to the `spatial.shape` manifest/typology data).

## Workstream D — Tests & validation

- Extend the core `Tests` region and kernel `Tests` region (no new test files): registry-driven asset loading; new kernel geometry primitives; data-driven energy `from_geometry` produces identical objects (`energy.energy.hull/roof/baseplate/externalwall/windows`).
- Run vitest for `@cad/js/core`, `@cad/js/kernel/brepjs`, `@cad/js/query` via nx and confirm green with runtime output.

## Ticket workflow (per repo rules)

- Read `repo://goals`, open a ticket (`ticket_open`) associated with the best goal before editing; keep temp artifacts in the ticket folder; close with `ticket_close` listing all touched files.

## Notes / decisions

- One new file (`cad/js/core/assets.ts`) is intentional per the chosen register-entry design; `index.ts` stays asset-free.
- Transformation execution becomes kernel-injected but stays synchronous (all needed primitives are sync `SpatialPreviewKernel` math).