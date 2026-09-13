# Flow Browser Source, Projection and Publication Owner Plan

## Scope and current authority

This bounded lane owns the six current Flow browser/host/declaration coordinates, their ABI source and browser-types fixture, and the command/package/consumer edges that produce or select them. It does not rename the wasm-bindgen `flow_core*` companions, JCO outputs, hashed OS distribution outputs, or unrelated Flow Rust sources.

The current authored browser and host bodies live under a manifestless `📦️packages/🟨️javascript` implementation directory. The declaration projection and its writer live in an executable test module that production imports. The core package router also contains the browser bundler, host copier, declaration publisher and package-manifest mutation. These are the owner defects to remove.

The public package subpaths `@semio-tech/flow-core/🌐️flow-browser.js` and `@semio-tech/flow-core/🖥️flow-host.js` remain the stable runtime URLs. Their physical package targets become anonymous leaves under semantic directories. The wasm-bindgen root export remains `flow_core.js` and is outside this move.

## Exact old-to-new map

| Role | Current path | Planned path |
| --- | --- | --- |
| Authored browser runtime | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.js` |
| Authored host runtime | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js` |
| ABI source | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi/🔣️.json` |
| Declaration generator/writer extracted from test | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/📤️projection/🟦️.ts` |
| Authored declaration projection | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📝️flow-browser.d.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts` |
| Browser-types fixture | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/📝️browser-types.json` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/📝️browser-types/🔣️.json` |
| Published browser module | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️flow-browser.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️browser/🟨️.js` |
| Published host module | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️host/🟨️.js` |
| Published declaration | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/📝️flow-browser.d.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🌐️browser/🟦️.d.ts` |

The production bundling/copy/manifest behavior moves from the package router to `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts`. The router keeps only command composition and wasm-pack/Cargo invocation.

## Schema-first ownership contract

Before moving product bodies, add:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🟦️.ts`

The contract will bind four executable owners (browser runtime, host runtime, declaration projection, publication), the ABI and browser-types data owners, three source/publication projections, all predecessor removals, exact taxonomy ancestry, the package export map, consumer source tokens, core target inputs, command routes, package scripts and seed/derived launch entries. It will compare target input sets exactly rather than by inclusion only.

The first red must occur before the move because the declared anonymous owners and outputs are absent. The green gate will parse its own schema with Ajv, inspect TypeScript/JavaScript imports, validate the production/test direction, run the pure declaration byte oracle, invoke Bun's `write:false` browser bundle, and check native TypeScript resolution for both public package subpaths.

## Production relationships

The browser runtime imports the semantic host source and contains exactly one native initializer coordinate. The publisher replaces that source-only initializer with `../flow_core.js`, externalizes only `../flow_core.js` and `../🖥️host/🟨️.js`, and emits `🌐️browser/🟨️.js`. The host source is copied byte-for-byte to `🖥️host/🟨️.js`. Declaration bytes are generated only from `🧬️schema/📡️abi/🔣️.json`, written to the authored projection and copied to `🌐️browser/🟦️.d.ts`.

The binding package keeps its three public exports:

- `.` -> `flow_core.js` with `flow_core.d.ts`
- `./🌐️flow-browser.js` -> `🌐️browser/🟨️.js` with `🌐️browser/🟦️.d.ts`
- `./🖥️flow-host.js` -> `🖥️host/🟨️.js`

## Consumer and cache closure

The move will rebind these current physical/source-as-data consumers:

- Flow host, draw-list, clock, open-ownership and schema-oracle JavaScript tests.
- Flow mock bridge TypeScript test and Rust protocol `include_str!` ABI reader.
- Flow component-domain Rust law source and published `include_str!` readers.
- Flow browser declaration Ajv/TypeScript/runtime oracle, which will import the generator and no longer export production behavior.
- Renderer React Vitest alias, rebased to the semantic authored browser runtime while public imports remain package subpaths.
- Renderer engine-contract ABI JSON import.
- Infinite draw-list fixture source coordinate.
- OS distribution layout facade source, rebased to the anonymous published browser module while retaining `🌊️flow/🌐️browser-[hash].js` output.
- Flow core package manifest, command router, project inputs, package route, and both launch seed and generated launch.

The existing 2D Vitest root alias already resolves the real `🫀️core/🕸️bindings/flow_core.js`; it is recorded as a current correct wasm-bindgen consumer and remains unchanged. The historical missing-alias prerequisite is not treated as current evidence.

## Verification plan and limits

1. Run the schema/fixture/ownership gate directly and through a new cached `test-browser-ownership` Nx target with ticket-private Nx directories.
2. Run `declarations` and compare generated source/published bytes to the pure projection.
3. Run direct and registered `test-browser` to execute the host/runtime suites and the actual Bun bundle with `write:false`.
4. Run the selected Rust `production_reachability_fixture_and_hostile_source_census_reject_the_old_route` law with a ticket-private `CARGO_TARGET_DIR` to prove `include_str!`, source/publication equality and public loader reachability.
5. Run taxonomy/discovery and exact launch-seed freshness checks.

No full wasm-pack rebuild, app build, server launch, installation, JCO generation or hashed distribution publication is part of this lane. Existing wasm-bindgen companions remain exact native outputs under their current contracts.
