# Play Vitest and native TypeScript stripping

After the catalog refresh, Node v24.15.0 and Vitest 4.1.12 loaded five Play suites with 43 passing tests. The assets, activation, and pages suites failed during module loading with `TypeScript parameter property is not supported in strip-only mode`.

Play's Vitest configuration externalizes the repo library, so Node loads the repo library and its transitive native TypeScript dependencies. The ticket-local [loader trace](trace-play-strip-types.mjs) invokes Node's strip-only parser on the source returned by the native loader to identify the exact file. The Play assets suite reproduced the failure first at the graph registry error and then at the WASM package identity error.

- [Graph registry follow](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧭️registry-follow/🟦️.ts) now declares and assigns the original `ClosedOwnerRegistryInvalidationV1.fault` field explicitly.
- [WASM package producer](../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts) now declares and assigns `WasmPackageIdentityErrorV1.timestampsOnly` explicitly. The package observation, physical identity checks and retry conditions retain their existing behavior.

The focused assets suite passed with the diagnostic hook. The full Play suite then passed with the normal native loader, covering **81 tests in eight files** in 12.61 seconds. The command ran from the Play root.

```sh
../../node_modules/.bin/vitest run --config '🧪️tests/🎚️config/🟦️.ts' --maxWorkers=1
```

An independent Node assertion checked that the registry custody wrapper recognizes the closed invalidation and keeps the original error object in both `fault` and `cause`. The successful custody branch returned its original value. The Play Vitest configuration and Bolt fallback remain in place.

## November 21 regression and rerun

Node again refused three test suites while loading retained Cargo source provenance. `CargoProvenanceSessionV1` and `CargoSourceWitnessV1` used constructor parameter properties. Both now declare and assign the same instance fields explicitly. The source witness and retention logic use the original argument values. The Play landing grid assertion was separately updated for the existing logical navbar padding. [Landing grid regression](play-landing-grid-test.md)

After the two repairs all 81 Play Vitest tests passed in eight files. All 13 release routing tests passed, including the independent-client test for published status, MIME, body, cache and CORS. The CDN cohort and browser boot checks remained pending.
