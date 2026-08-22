# Animate Present Core Recovery

## Outcome

Restored the canonical owned `@semio-tech/animate-present-core` TypeScript entrypoint at its three existing alias targets. The restored implementation matches the last intact repository version at `fa51b5c82f`, with two current-contract corrections: the obsolete deprecation annotation on `presentationPlayAppDefinition` is now an owned API docstring, and the internal morph-participant accumulator clones its readonly input into a mutable owned `Set` before adding morph slots.

No runtime dependency, compatibility adapter, consumer source, Rust artifact, or package manifest was added or changed.

## Focused Regression

Added one focused regression that exercises the required recovery seam end to end:

- assembles an out-of-order slide glob into chapter, sequence, and thought hierarchy;
- verifies `collectPresentationSlides` ordering and bookmark coordinates;
- resolves the first arrangement through the accumulated presentation scope.

The Animate Vitest config includes this focused test alongside its existing in-source renderer tests.

## Evidence

Executed while the P4 wall-clock gate owned the CPU-heavy validation slot:

```text
bun x vitest run '../../🎛️apps/🎬️present/⚡️implementations/🟦️typescript/🧪️index.test.ts' --config './🧪️vitest.config.ts' --maxWorkers=1
Test Files  1 passed (1)
Tests       1 passed (1)
Duration    435ms
```

Focused TypeScript validation of the recovered entrypoint and regression test is clean:

```text
bun x tsc --noEmit --skipLibCheck --target ES2022 --module ESNext --moduleResolution Bundler --lib ES2022,DOM --types vitest/importMeta,vitest/globals <core> <regression>
exit 0
```

The full Animate gate is green against the final source:

```text
bun nx run @semio-tech/animate-js:test
Test Files  3 passed (3)
Tests       141 passed (141)
Errors      0
Duration    7.12s
```

The uncached 33. Projektetage consumer production build is green against the final source:

```text
bun ./📜️script.ts build
Modules     1,844 transformed
Duration    11.92s
Result      production build succeeded
```

Existing non-blocking Vite diagnostics remain for a malformed scrollbar selector, runtime-resolved assets, browser-external Node modules, a circular manual chunk, and large chunks.

## Integration Reconciliation

- Corrected the Animate test alias for the Projektetage spec to its current canonical package entrypoint.
- Routed PDF DOM tests through the owned `PdfCanvasPort` and awaited async canvas readiness rather than depending on the deleted synchronous `react-pdf` mock behavior.
- Made relative `.json` fetch fixtures valid JSON, eliminating six unhandled parse rejections.

## Intentional Files

- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts`
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/🧪️index.test.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️vitest.setup.ts`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️component.tsx`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/📓️p10t-animate-present-core-recovery.md`
