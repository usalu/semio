# P9e — Owned Synchronous 2D Kernel

## Outcome

- Removed the `pollster` runtime bridge from `semio-framework-2d`.
- Converted flow's in-process `DrawingKernel` contract, its sole `DrawingStore` implementation, JSON bridges, and tests to direct synchronous calls.
- Removed the direct `pollster` and `thiserror` dependencies from the 2D crate.
- Replaced `DrawingError` and the shared `EngineFault` derive macros with owned `Display` and `Error` implementations.
- Replaced flow's private `DrawingKernelError` derive with owned formatting/conversion.

## Verification

```text
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-2d-sync \
  bun nx run semio-framework-2d:test-quick --skip-nx-cache

21 tests run: 21 passed, 0 skipped
NX Successfully ran target test-quick for project semio-framework-2d
```

The first run exposed the shared `EngineFault` derive as the remaining direct `thiserror` consumer reachable from this crate. After replacing it, the same isolated Nx gate passed. The flow-family product gate remains separate because the current concurrent RNG API repair temporarily blocks that build upstream.
