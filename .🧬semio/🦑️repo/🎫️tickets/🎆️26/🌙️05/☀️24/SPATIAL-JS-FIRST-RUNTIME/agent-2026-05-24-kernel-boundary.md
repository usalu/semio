# Agent session (kernel boundary)

## Summary

- Removed `InMemoryKernel` from `@spatial/js-core`; geometry stays behind `KernelAdapter` only.
- Core factory specs load from `spatial/fixtures/*.json`; added `buildExtrudeFactorySpec` / `buildOffsetSurfaceFactorySpec`.
- Core tests use an inline `RecordingStubKernel` (records `createBoxFromCorners` args; no volume/mesh math).
- `evalExpr` handles non-object sub-expressions (e.g. literal `0` in JSON guards).
- Implemented `BrepjsKernel` in `@spatial/js-kernel-brepjs` (brepjs `init`, `box`, `measureVolume`, `mesh`; optional `query` / `extrudeWire` / `offsetFaces` stubs).
- Kernel package: Vitest + `tsconfig` `paths` alias for `@spatial/js-core` → `../core/index.ts`.

## Commands run

- `bun nx run @spatial/js-core:test` — pass
- `bun nx run @spatial/js-kernel-brepjs:test` — pass

## Files touched

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/kernel-brepjs/vitest.config.ts`
- `spatial/js/kernel-brepjs/tsconfig.json`
