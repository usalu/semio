# React Renderer Quick-Gate Repair

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Outcome

The renderer fundamental, quick, and long gates are now honest, bounded, and green. They execute one, two, and 535 tests respectively instead of trying to collect and run the complete renderer corpus inside 15/30 seconds. Three deterministic infrastructure defects and the shard-client import boundary behind 97 exhaustive failures were repaired. The isolated quadratic exhaustive hotspot is also green across every 1,446 authored cancellation prefix; a complete exhaustive-corpus result is still outstanding.

## Failures Reproduced

`bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` initially reported:

- `RendererResidentComposition never replaces a closing composition ledger` failed because the test imported `../../💾️resident/🟦️` as default fixture data; that module exports only `rendererResidentLedger`.
- `RendererResidentComposition shares one exact ledger and preserves both consumers' charges` failed for the same reason.
- The runner then exceeded the 30-second quick budget.

The focused long-level command exposed a second independent configuration error: `@semio-tech/framework-surface-rs` could not resolve because `wasmEngineStub` named the nonexistent `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` file.

The first unfiltered long run then completed collection and reported 646 passing and 97 failing tests. Every failure shared one cause: `UiDocumentStore` dynamically imported `ShardClient` from the actor scheduler package entry, where it is intentionally not exported. The focused rerun after correcting that boundary passed. A subsequent unfiltered long run reached the five-minute wall budget without a final result, establishing that the incremental ownership matrices belong at exhaustive level rather than silently overrunning the long contract.

## Changes

- Both ledger tests now load the existing language-neutral fixture `💾️resident/🧪️fixture/🔣️.json` while continuing to load the ledger implementation from `💾️resident/🟦️.ts`.
- The renderer Vitest config now points WASM aliases at the existing styling adapter `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`.
- Specific `framework-os`, `framework-surface-rs`, and `framework-editor-rs` aliases precede the broad `@semio-tech/framework` alias.
- `UiDocumentStore` and `TaskManager` now import the shard transport from its canonical `🧵️shard-client/🟦️.ts` module instead of the scheduler package entry.
- A small language-neutral resident-composition smoke suite now compares the production ledger against the JSON fixture using Node's independent deep-equality oracle.
- Fundamental runs the capacity contract, quick cumulatively runs capacity plus shared React/WGPU identity, long runs the package corpus and three moderate in-source suites, and the expensive `UiDocumentStore` ownership/cancellation matrices run at exhaustive level.

## Verification

`bun nx run @semio-tech/framework-renderer-react:test-long -- -t RendererResidentComposition`

- Nx exit: 0
- test files: 1 passed, 5 skipped by the explicit name filter
- tests: 2 passed, 741 skipped by the explicit name filter
- duration: 20.69 seconds

`bun nx run @semio-tech/framework-renderer-react:test-long -- -t 'OwnedPagedAdmission roots the actual field builder'`

- Nx exit: 0
- test files: 1 passed, 5 skipped by the explicit name filter
- tests: 1 passed, 742 skipped by the explicit name filter
- duration: 23.32 seconds

`bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache`

- Nx exit: 0
- test files: 1 passed
- tests: 2 passed
- Vitest duration: 4.51 seconds

`bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache`

- Nx exit: 0
- test files: 1 passed
- tests: 1 passed, 1 skipped by the fundamental-level selector
- Vitest duration: 2.97 seconds

The pre-classification unfiltered `test-long` attempt was killed at the explicit 300,000 ms budget and is not represented as a pass.

`bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache`

- Nx exit: 0
- test files: 6 passed
- tests: 535 passed
- Vitest duration: 33.97 seconds

`SEMIO_COVERAGE=0 SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/framework-renderer-react:test-exhaustive -- -t 'OwnedPagedCancel retains every first-fragment cancellation prefix' --reporter=verbose`

- Nx exit: 0
- test files: 1 passed, 6 skipped by the explicit name filter
- tests: 1 passed, 744 skipped by the explicit name filter
- authored cancellation prefixes: 1,446
- focused test duration: 82.72 seconds
- Vitest duration: 93.60 seconds

The first complete coverage-disabled exhaustive attempt was manually interrupted after it stopped producing reporter output; its orphaned worker was then terminated explicitly. It is not represented as a pass. The focused evidence above proves the slow point was finite quadratic test work rather than a lifecycle deadlock, but the complete corpus still requires a clean uninterrupted result.
