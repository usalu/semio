# Flow Browser Ownership Preaudit

## Status

**Current status — accepted for the bounded source/projection/publication extraction, with an explicit missing native-reachability limit.** This report began as a preaudit; its final acceptance and superseding direct/Nx evidence appear below. I did not build Wasm, run a browser, execute native code, or modify product files.

The original production-to-test declaration edge has been removed by the semantic split described below.

## Historical Pre-Extraction Authority And Defect — Resolved

Before the semantic move, the core router was:

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts`.

Its former `publishBrowserDeclarations` imported `flowBrowserDeclaration` from:

`🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts`.

That test leaf then owned:

- `flowWasmContract`, the strict Ajv contract reader;
- `flowBrowserDeclaration` and `writeFlowBrowserDeclaration`, the ABI-to-TypeScript projection and filesystem writer;
- `testFlowBrowserDeclaration`, the package, TypeScript-resolution, runtime-prototype, hostile-fixture, and freshness oracle.

That is a production-to-test implementation edge. Move the ABI declaration projection and publication behavior to a neutral declaration/projection concern. Keep the Ajv/TypeScript/runtime/hostile assertions in a test-oracle owner that consumes the neutral projection. The package router may import the semantic production owners directly, but must not restore a test-owner implementation import.

The test currently uses a synthetic URL anchored at absent `🕸️wasm/📦️packages/🟨️javascript/📜️script.ts`. It is only a path anchor today; extraction must rebase paths from the semantic owner rather than recreate that nonexistent command source.

## Historical Pre-Extraction Source And Generated-Binding Closure — Moved

The authored browser side is under:

- `🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js` — runtime constructor and one authored dynamic initializer import;
- `🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` — host implementation;
- `🌊️flow/🕸️wasm/🧬️schema/📡️abi.json` — ABI operations and argument fields;
- `🌊️flow/🕸️wasm/🧫️fixtures/📝️browser-types.json` — 105-method declaration/package-export oracle.

The generated/published package is under:

- `🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm`;
- `🌊️flow/🫀️core/🕸️bindings/flow_core.js`;
- `🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts`;
- `🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm.d.ts`;
- `🌊️flow/🫀️core/🕸️bindings/🌐️flow-browser.js`;
- `🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js`;
- `🌊️flow/🫀️core/🕸️bindings/📝️flow-browser.d.ts`;
- `🌊️flow/🫀️core/🕸️bindings/package.json`;
- `🌊️flow/🫀️core/🧪️bindings.json`.

The core router currently owns the following behavior, which should become semantic owners while preserving the router as dispatch only:

1. Browser bundle projection: bundle the authored browser entry with write/no-write control, replace exactly one authored `flow_core.js` source-tree initializer import with the published sibling, and externalize only `./flow_core.js` and `./🖥️flow-host.js`.
2. Browser declaration publication: write the ABI projection to the binding package and add the browser runtime, host, and declaration entries to package files/exports.
3. Wasm package publication: invoke the existing Wasm producer, copy its four `flow_core` outputs, copy the authored host, run the browser projection, and preserve snippets if produced.
4. Browser package check: assert one output, the two sibling references, and absence of the source-tree binding coordinate.

The current package manifest has three public entry groups: root `flow_core.js`, `./🌐️flow-browser.js` with `📝️flow-browser.d.ts`, and `./🖥️flow-host.js`. The fixture validates the same seven files and all three exports; retain it as source-as-data for package publication rather than snapshotting implementation bodies.

## Historical Pre-Extraction Consumers And Registration — Rebound

Production/browser consumers currently include:

- `🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js` lazily imports the generated `flow_core.js` only when an embedding initializer was not supplied.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx` dynamically imports `@semio-tech/flow-core` and its `🌐️flow-browser.js` subpath, then passes the core wrapper as bindings.
- The renderer React test config aliases the Flow browser subpath to the authored browser runtime while it aliases bare `@semio-tech/flow-core` to its Wasm stub. This is a test-host consumer boundary, not a reason to publish a second implementation.
- `🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js` and `🌊️flow/🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js` consume the authored browser/host and the generated Wasm byte output.
- `🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs` reads the authored and published browser sources and the package contract as native source/data inputs.

The existing core project routes `declarations`, `wasm`, `test-browser`, `test-browser-clock`, and `test-source` through its only `📜️script.ts`. Launch seed and derived launch register the corresponding `semio-framework-os-flow-core` targets. Extraction must rebind all declared owner inputs and both launch representations to the new semantic owners.

The current 2D Vitest config has already corrected the historical alias baseline: it now resolves `@semio-tech/flow-core` to the real `🌊️flow/🫀️core/🕸️bindings/flow_core.js`. Do not treat the packet's formerly nonexistent alias coordinate as a current producer or recreate it.

## JCO Boundary

A bounded current search found no JCO generator, manifest, or runtime consumer in the Flow core or Flow Wasm source tree. The Flow browser publication is a wasm-bindgen `flow_core` package boundary. The renderer contains JCO discussion and generic plugin-runtime code, but that text does not make it a Flow generated-manifest consumer.

The acceptance fixture should therefore enumerate the actual `flow_core` Wasm/package manifest and Flow browser projection consumers above. It should not add JCO production authority or infer a JCO runtime law from unrelated renderer commentary.

## Pre-Acceptance Controls

Before acceptance, the source gate should demonstrate:

1. Every moved owner has a valid semantic ancestry/context chain and an anonymous `🟦️.ts` leaf.
2. The core router has no direct import of a test implementation; declaration projection and writer are imported only from their neutral owner.
3. The declaration oracle still applies strict Ajv to `📡️abi.json`, parses the declaration with TypeScript, compares the 105 runtime methods and samples, validates the package's three exports and seven files, and rejects the five hostile fixture mutations.
4. The browser projection accepts exactly the intended initializer reference, emits only the published sibling imports, and rejects a source-tree binding coordinate in bundle output.
5. Package manifest files/exports, authored/published host relationship, and generated output names remain source/data checks rather than body-hash expectations.
6. Project named inputs and launch seed/derived registrations include every moved source owner, ABI document, browser-types fixture, authored runtime/host, and generated-output producer boundary.
7. Existing safe browser-bundle and declaration/package routes are rerun by the executor under isolated artifact/cache roots. A full Wasm rebuild, WebGPU browser session, or JCO execution is outside this preaudit and must not be implied by those focused checks.

## Limits

The prerequisite packet records earlier direct and registered browser/declaration evidence. I did not rerun it. Those retained results establish current behavior before extraction, not fresh proof that future semantic owners are registered or source-fresh.


## Plan Review Update

The proposed owner plan in `📓️sol-flow-browser-owner-plan-2026-09-13.md` fits the current responsibilities:

- browser runtime under `🌐️browser/🏃️runtime`;
- host runtime under `🖥️host/🏃️runtime`;
- ABI document under `🧬️schema/📡️abi`;
- declaration projection under `🌐️browser/📝️declaration/📤️projection`;
- package publication under `🌐️browser/📦️publication`;
- authored/published declarations and browser/host files as kind-only leaves under their semantic roles.

It preserves public package URLs while divorcing physical implementation paths from those URLs. The split also keeps the generated `flow_core*` wasm-bindgen companions outside this browser-specific move.

One additional current source error must be corrected during the move: `🌊️flow/🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js` imports `flowWasmContract` from nonexistent `🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📜️script.ts`. Rebind it to an owned test-contract/oracle source that reads the Flow schema. It should not consume the production declaration projection or recreate a package command facade. Include this path in the source-data, project-input, and hostile-resolution coverage.

The related host and host-retirement test sources also consume `flowWasmContract`; their test-only dependency may remain test-to-test after the production declaration emitter is removed from that owner. The new ownership fixture must distinguish that permitted oracle dependency from the prohibited production-router-to-test edge.


## Current Static Reinspection

The planned source relocation is now materialized. The core package router directly imports `bundleFlowBrowserModule`, `publishFlowBrowserDeclarations`, and `publishFlowBrowserPackage` from `🌐️browser/📦️publication/🟦️.ts`; its former `BROWSER_BRIDGE_DIR`, inline browser bundle, inline declaration writer, package-manifest mutation, and production import of `flowBrowserDeclaration` from the declaration test owner are gone. The router's remaining declaration-test dynamic import invokes an oracle after the semantic publication action; it does not provide production implementation.

The current ownership contract tracks the four executable semantic owners, two neutral data owners, three generated/published projections, predecessor absence, source/data consumers, exact target inputs, package route, and both launch representations. The consumed-browser-clock source now obtains `flowWasmContract` from `🧪️tests/🧬️schema-oracle/🛂️admission/🟦️.ts`, rather than the nonexistent package-script coordinate. This restores the intended test-oracle boundary.

This is static/source evidence only. Direct, registered, and selected native-law results remain required before final acceptance.

## Executor Evidence Update — Final Native and Registered Checks Pending

The current direct ownership route is green: **exit 0 in 0.53 s**, with **4 semantic owners, 17 consumers, 39 exact inputs, and three public exports**. It exercised native TypeScript resolution and Bun `write:false` bundle parity. Focused current routes are also green: declarations cover **105 methods, three exports, two resolutions, and five hostile vectors**; preview reports **seven files**, **6,229 browser bytes**, and **23,497 declaration bytes**; browser and consumed-clock checks are green, with the clock route observing **7,733 `performance.now` samples** and terminal-empty behavior.

The production `test-source` import rebind itself is fixed. Its broader route now reaches an unrelated retained-scene assertion expecting `assert!(turns > 1_600)` that is absent from the live board source. This is separate repository source/test data, not a Flow browser ownership failure; it cannot support or weaken this lane's acceptance.

Selected Rust reachability and isolated registered Nx evidence remain pending. No final native or registered acceptance is claimed by this update.

## Final Independent Acceptance

**Accepted for the bounded Flow browser source/projection/publication ownership extraction, with the native-reachability limit below.** Static reinspection confirmed that the four anonymous semantic owners are present, all predecessor coordinates are absent, the core router delegates publication directly, and the schema oracle remains a test-only consumer. The ownership fixture binds 17 current consumers, 39 exact target inputs, three package exports, package/route and seed/derived launch registration; it includes the repaired consumed-clock test-oracle import.

Current direct evidence passed the ownership route, declaration/package oracle, browser bundle, and consumed-clock behavior. The current isolated registered target `semio-framework-os-flow-core:test-browser-ownership --skip-nx-cache` passed **exit 0**, one task, **608 ms critical / 638 ms total**, cache skipped. This establishes the live target/registration result for the source-projection/cache closure.

The selected Flow Rust reachability law did **not** run. Its ticket-private Cargo build stopped first on an unrelated current store compiler error, `E0277: P: ArtifactPack` unsatisfied at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:466` (the relevant implementation bound is at line 85), exit 101 with no Flow test count. This is an external compiler blocker, not Flow law evidence. Consequently this acceptance does not claim native Rust `include_str!` reachability, a Wasm rebuild, JCO execution, or browser-session behavior. The unrelated retained-scene source assertion in the broader `test-source` route also remains separate from the Flow ownership result.

## Current Flow Publication Delta

The final publication closure adds private successful-publication and pre-promotion missing-companion controls, and retires only the transient family-compiler `package.json` after successful publication. The source now has one public `@semio-tech/flow-core` identity under `🫀️core/🕸️bindings`; the family native companions remain as producer output. Final direct ownership is green in **0.64 s**. The superseding isolated target is green, **exit 0**, one task, **585 ms critical / 615 ms total**, cache skipped. Workspace taxonomy is green at **625 kinds, 545 fixed, 23 generators**.

The selected Flow Rust law remains unexecuted. After the ArtifactPack bound repair it is stopped before law execution because `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs` declares `#[path = "📥️retained/🦀️.rs"] mod retained;`, but that retained source and its six `WindowConfigPackLoad*` reexports are absent. This remains an external compiler/source closure limit; this report does not claim native Flow reachability.


## Publication Transaction Limit

The private missing-companion control proves the failure path **before** promotion preserves its preceding public-package sentinel. It does not prove rollback after promotion. Current publication makes two rename transitions and only then retires the transient family `package.json`; if that final `rmSync` fails, the new public package can already be visible. This audit therefore does not claim a blanket atomic/rollback publication transaction. It accepts the tested owner, identity, and pre-promotion failure boundary only.
