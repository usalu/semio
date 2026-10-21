# Demonstrator Unit Source Scope — 2026-10-21

Source-only audit requested while existing jobs remain active. No product, manifest, test source or build guard was edited, and no command/dry run was launched.

## Current Cause

The test target has dependsOn=[], outputs=[], command bun ./📜️script.ts test, forwards user test arguments, and declares neither execution.kind nor sourceInputObservation/runtimeSourceEntries. Its named default input includes `{projectRoot}/**/*`, explicit Vite/E2E/unit configs, exclusions only for dist/test-results, and sharedGlobals.

In build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs, a selected task without an observation role defaults to compile. The actual source scan builds compileByTarget, promotes every matched JS/TS source into compileModules, then follows the full typed import closure. Therefore the unit target currently promotes app source, site/runtime scripts, E2E source and configs matched by the default glob into compiler consumers even though they are not unit test entry points. A narrower launcher alone cannot remove this declared whole-project compilation.

The router is not the same direct site-builder mistake repaired in S dev. Its top-level runtime/🟦️.ts import supplies catalog/generated plugin helpers and inventory/deployment utilities, not runtime/📜️script.ts or site build/serve. TestScript imports the broad public repo-lib testing barrel, which reexports Go/inference/coverage helpers as well as Vitest; the sibling runtime-catalog-test branch imports the broad execution entry. Command-import parsing follows literal imports in sibling functions but explicitly omits `if(import.meta.vitest)` bodies. It does not promote the brand/site test-only imports inside that guard through the command role.

The reported 1039 owners are not by themselves proof that every owner is compiled; distinguishing the registry roster from selected dependency closure remains necessary. The source-level cause above proves promotion of all matched Demonstrator source and the closures it imports, without assigning an unsupported per-owner count.

## Actual Normal Unit Work

Current unit config includes the console test file and includes source tests in router, brand, activation module and pane branding. Its long/exhaustive-only shell-brand architecture entry is separate. Actual normal callback bodies were inspected:

| Entry | Actual callback operations |
| --- | --- |
| router runtime-build variants | Pure catalog selection; filesystem reads of authored runtime JSON/schema/pipeline; Ajv validation. |
| router descriptions | String paragraph splitting and pane descriptions. |
| router map tiles | Pure serve-mode/zoom constants and temporary environment override/restore; no tile prefetch/build. |
| router compile closure | Reads authored Cargo.toml and checks dependency strings; no Cargo invocation. |
| brand pane boot variants | Pure variant identity mapping. |
| brand examples | Reads Rust example ID declarations and brand source, parses TypeScript AST, validates neutral fixture with Ajv and compares actual branded IDs; no native build. |
| activation union receipt | Pure lane/component closure and in-memory synthetic receipt merge/validation; never calls publishDemonstratorUnionReceipt, readDemonstratorActivation, a lane producer or a server. |
| pane branding | Reads authored component manifests and applies real breadcrumb helpers. |
| console/interaction contract tests | Pure console classification and filesystem reads of neutral cases/schema validated with Ajv; no browser/session launch. |

No callback invokes Cargo, a native producer, Vite builder, preparation/activation producer, native process launcher or child repo task. Imports still require their genuine JS/TS source closure; these tests are not free of normal compiler authority.

## Supported Domain-Correct Declaration Proposal

A source-only unit execution role is accurate for this test target: it launches Vitest and inspects sources/pure helpers, not native artifacts. sourceInputObservation=freshness would retain complete default-file freshness while ceasing to interpret every default-matched file as an executable compiler root. It must be paired with explicit runtimeSourceEntries for the actual unit config, four configured includeSource roots and normal console test:

- {projectRoot}/🧪️tests/🎚️config/🟦️.ts
- {projectRoot}/📜️script.ts
- {projectRoot}/🪧️brand.ts
- {projectRoot}/🔨️modules/🧩️runtime/♻️activation/🟦️.ts
- {projectRoot}/🧪️tests/🧪️demonstratorpanebranding/🟦️.ts
- {projectRoot}/🧪️tests/🎭️acceptance/🧬️console/🧪️.ts

Correction from independent review: runtimeSourceEntries follow executable import closure, whose command parser skips import.meta.vitest guards. Merely listing the configured root modules does not capture their guarded registrations. The authored finite roster must include the long shell-brand entry plus every guarded literal helper and production import; the saved fixture below enumerates sixteen roots. Named source-contract declared compile entries have separate semantics. All listed runtime roots and their actual imports retain physical source and producer authority.

The existing public @semio-tech/repo-lib/testing/vitest entry explicitly reexports only the owned runVitest leaf. The TestScript can select that existing boundary instead of the broad testing barrel. Moving demonstratorRuntimeBuildVariants import into its existing import.meta.vitest guard makes the command launcher reflect that this value is a test dependency. These are proposed source-boundary improvements, not an alternate launcher executable or invented CLI flag. The sibling runtime-catalog route should retain its own required owner semantics.

Build, prepare, activation, E2E and serving targets must retain native/production authority; no global inference or guard changes follow from this proposal. Any implementation needs neutral fixture assertions for the unit declaration/runtime roots and existing third-party schema/source oracles, then actual native execution. Nothing in this report claims that the proposal has been implemented or tested.

## Test-First Save Boundary

Saved before parent metadata/root-router edits: Demonstrator `🧪️tests/🧪️demonstratorcompileclosure/🧫️cases.json`, sibling `🧬️schema/🔣️.json`, and extended `🟦️.ts`. The existing normal unit registrar now validates a closed neutral sixteen-entry contract with Ajv, derives the actual config include/includeSource union and recursively follows guarded relative literal imports using TypeScript AST, and compares that independently derived set with fixture and both actual target declarations. Test and canonical-architecture must use source/freshness, identical finite entries, their existing command strings and argument forwarding. Build and test-e2e raw execution/observation controls remain unchanged.

The sixteen roots are seven base config/root/normal/long entries plus nine guarded runtime-helper, registrar and tile-mode entries, as independently audited. Exact expected metadata is in the fixture rather than duplicated in this report. At save time test.metadata was still absent and the runtime-build helper remained outside its Vitest guard; actual native red/green execution has not been claimed. Parent owns the following manifest/router implementation. No duplicate native request or restart occurred.

## Parent Implementation Save

After High saved neutral16-entry fixture, closed schema and Ajv+TypeScript-AST scope regression, parent set only test and canonical-architecture execution.kind=source, sourceInputObservation=freshness and exact16 runtimeSourceEntries. Existing broad/default freshness inputs, forwardAllArgs, testlevels, commandstrings, build and E2E metadata unchanged. Parent replaced broad public testing barrel import by its exact existing owned Vitest wrapper leaf and moved test-only runtimeBuildVariants import into existing Vitestguard; explicit runtimeentries retain that actual implementation plus all seven guardedhelpers/tilemode. Low review pending. Current nativeprocesses retained; no actual test assertion has run yet.

## Parent Implementation Save Boundary

Parent saved exactly the test and canonical-architecture target metadata: execution.kind=source, sourceInputObservation=freshness and the sixteen explicit entries from the neutral fixture. The root router now calls the existing owned Vitest leaf instead of its broad testing barrel, and imports runtimeBuildVariants only inside its existing Vitest guard. Readback confirms both declarations exactly match fixture entry order and the former top-level runtime helper import is absent. Product build, runtime catalog, E2E, preparation and serving declarations were not part of this change.

The five source/contract paths are deduplicated in files-2026-10-21.md. Independent review is pending; no new native invocation was launched and the existing Source-demand 46731 remains retained. This is a saved source boundary, not a passing test assertion result.

The forwarded long shell-brand architecture entry was also inspected after the save: its test imports and callbacks use React static markup, real footer/brand helpers, filesystem existence checks and authored manifest fixture/schema validation. No native process, builder, producer or server call appears. Keeping this entry in the finite roster preserves the long architecture test’s real source closure without requesting native execution.
