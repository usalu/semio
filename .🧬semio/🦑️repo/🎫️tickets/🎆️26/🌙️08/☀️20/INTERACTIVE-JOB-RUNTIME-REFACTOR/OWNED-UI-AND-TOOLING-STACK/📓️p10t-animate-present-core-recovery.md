# Animate Present Core Recovery

## Outcome

Restored the canonical owned `@semio-tech/animate-present-core` TypeScript entrypoint at its three existing alias targets. The restored implementation is byte-for-byte equivalent to the last intact repository version at `fa51b5c82f`, except that the obsolete deprecation annotation on `presentationPlayAppDefinition` is now an owned API docstring.

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

Read-only `diff` against `fa51b5c82f` reports exactly one semantic text delta: the `presentationPlayAppDefinition` docstring. The 3,133-line implementation body and public contract are otherwise unchanged.

## Deferred Gates

The full `@semio-tech/animate-js:test` route and the 33. Projektetage consumer build remain intentionally deferred until the exclusive P4 wall-clock gate releases CPU-heavy validation, as required by the recovery assignment.

## Intentional Files

- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts`
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/🧪️index.test.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/📓️p10t-animate-present-core-recovery.md`
