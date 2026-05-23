# Spatial STEP AP242 Roundtrip

Implemented six-pillar STEP export/import for the spatial framework:

- **ModelSpace / Model / Object** via `PRODUCT_DEFINITION` + `NEXT_ASSEMBLY_USAGE_OCCURRENCE`
- **Primitives** via brepjs `exportSTEP` chunks merged into the file (hash-deduped across linked models)
- **Attributes** via `spatial.attribute.*` UDA + `ModelJson.metadata`
- **Properties** derived on export as `spatial.property.*` UDA; re-derived on import

## API

- `@spatial/js-core`: `StepEntityWriter`, `parseStepEntityMap`, `parseSpatialUdaPayloads`, `assembleStepFile`, …
- `@spatial/js-kernel-brepjs`: `exportModelSpaceToStep`, `exportModelToStep`, `importStepToModelSpace` on `BrepjsKernel`

## Tests

- `@spatial/js-kernel-brepjs`: 22/22 passing (including AP242 roundtrip, dedupe, property re-derive)
- `@spatial/js-core`: step helper + metadata tests passing

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
