# Closed Browser Component Factory

## Current Boundary

The GIS executor audit requires a separately attested, closed browser actor bundle. This first implementation is its build-time component factory substrate, not the actor bundle, descriptor/catalog admission, private Shell handoff or live GIS rendering.

Installed JCO1.27 exposes an `instantiation: async` mode that places component state inside its `instantiate` function. A first in-memory Bun probe18495 produced JCO output but exited1 because a JCO dependency uses Node's `process.binding("tcp_wrap")`. The Node probe27858 exited0 and exposed the exact generator shape. Permanent qualification now runs JCO under Node through the existing bounded captured-process runner; Bun remains the package manager and Nx the task runner.

## Implemented Substrate

`🔌️plugin/🌐️browser-bundle/📜️script.ts` owns the build-time API and neutral test. It accepts a JCO instantiation source plus an exact closed set of core bytes, validates bounds and literal core selectors, removes only the recognized default fetch resolver, and rejects extra declarations, imports, dynamic executable loading, network loader references and missing/duplicate/extra core assets.

The emitted private `instantiateFreshComponent` contains immutable chunked core bytes. Every invocation creates fresh JCO state and Wasm instances. It decodes in bounded chunks, reports progress, checks cancellation before/after compilation, zeroes staging arrays and drains all started core promises before returning or throwing. It exports no URL or code-selection capability itself. This structural gate is not a sandbox proof for arbitrary JavaScript; the intended input is the trusted build-time JCO output.

## Qualification

- Initial registered20313 was infrastructureRED on an incorrect relative import; corrected.
- Registered1442 was the expected test-first RED: real JCO output reached the not-yet-defined factory.
- Registered48948 GREEN: strict AJV fixture, genuine JCO component extraction, two independent instances sharing one generated module, native WebAssembly independent oracle matching `[1,2,1,3,2]`, eight hostile closure denials and precompile cancellation. Evidence: `🗑️generated/browser-component-factory/browser-component-factory-Pir6iv`.
- Stronger rerun90548 is pending: explicit measured compile/instantiate counters, cancellation during compilation, zeroed staging and build cancellation replace the earlier precompile-only observation. No stronger result is claimed until that run finishes.

The permanent target is `@semio-tech/framework-os-dev:closed-browser-component-factory-check`; launch seed411.082 supplies ticket-generated output ownership. Normal registry generation/freshness is assigned to the Home lane.

## Next Integration

Create an actor-local first-party host shim and a fresh-instance WASI import factory, combine them with this component factory and the current bridge API, then materialize and attest one browser bundle. Extend the descriptor/trusted-catalog/open-plan/lease schema and exact protected asset route. Only after private broker-to-Shell activation and canonical typed command publication should `renderer-unavailable` be replaced. The genuine `patchPositions`→Hub acknowledgment→close/reopen law remains required.
