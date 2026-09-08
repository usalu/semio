# Final Current Test Layout Audit

Read-only final recheck on 2026-09-08 after the TypeScript moves, Rust proxy cleanup, and final scanner run. No production source, Git state, ticket state, or shared generated directory was changed by this audit.

## Current Layout Evidence

The ticket-private [script](./🧑‍💻final-layout-audit/📜️script.ts) performed a no-ignore JavaScript, TypeScript, Python, and Go source-body audit using the exported layout inspector. Its completed focused inventory inspected 3,487 sources: 657 canonical implementations and 2,962 executable candidates. It returned zero inline-test-body, inline-self-test-declaration, test-owner-delivery-scope, or other layout findings. Its temporary JSON capture has been transcribed here and removed during ticket closure.

A subsequent coordination-owned full scanner inspected 29,224 current sources and returned:

~~~
{ "total": 0, "counts": {} }
~~~

That fresh full-tree result supersedes the narrower inventory above. The parent also verified the final caller/runtime suite at 45 passing tests and 1,443 assertions. The earlier focused layout suite passed 23 tests and 75 assertions; it established the Rust compiler oracle, literal masking, semantic-owner/delivery-scope, external-wiring, domain-support escape, and Nx discovery/hash checks.

## Non-Feature Runner Samples

These direct source checks cover runner paths that ordinary test-file globbing does not explain. They are samples, not a claim that all non-feature cases were executed in this audit.

- Guarded registration is explicit in [typescript owner](/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/🟦️.ts:411): import.meta.vitest dynamically imports the canonical 🧪️tests/🧪️docklayoutstore/🟦️.ts and calls registerTests1. The guarded owner file contains registration only; the test body remains in the canonical implementation.
- The actor-import fixture proxy [script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts:7) imports testCanonicalActorAsyncImport from its canonical test leaf. Its import.meta.main block at line 19 dispatches that function for its two local runtime targets, runtime-check and pending-host-close-check. It has no test declaration or assertion body, so it is a controlled runtime adapter rather than a second suite.
- Storybook Playwright collection uses the two canonical engine leaves directly in [playwright config](/Users/ueli/Documents/semio/.storybook/playwright.config.ts:29). The React Vitest config excludes those exact paths at [Vitest config](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts:66), while retaining its own engine suites. This prevents duplicate collection.
- The canonical WebGPU surface case is invoked by the owner Rust package script at [script](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts:26), which runs Bun against ../../🧪️tests/🖼️webgpu-surface/🟨️.js. The exact command completed with a2Composition true, surfaceTrace create-resize-frame-drop, and all declared resource ceilings satisfied.
- The root [Vitest config](/Users/ueli/Documents/semio/vitest.config.ts:36) deliberately has an empty include list and passWithNoTests false, because it owns no repository test collection. It therefore cannot silently double-collect default test-file names.

The generic Nx test-case plugin discovers files named 🥒️.feature; discoverTestCases requires that file. At this audit point, 122 of 636 canonical JavaScript, TypeScript, Python, and Go case directories had a feature file, while 514 did not. This only establishes that the generic feature plugin does not create semantic case targets for those 514 directories. Package-native configuration and explicit scripts remain their execution paths.

## Result

No concrete current JavaScript, TypeScript, Python, or Go placement issue, hidden executable fixture/proxy, duplicate runner, unwired sampled runner, or wrong-framework sampled runner was found. The independent Rust compiler and traversal evidence is retained in [rust compiler oracle](./📓️rust-inline-module-compiler-oracle-2026-09-08.md).
