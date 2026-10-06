# Final Runtime Audit — 2026-10-06

Read-only source audit. No runtime test was run and no passing-test or functioning-runtime claim is made. Authored TypeScript/TSX/Rust census used `rg`, excluding node_modules, .git, dist, ticket output, and test/fixture directories for the initial production candidate set. Concrete surrounding code and callers were inspected before classification. Concurrent edits may change the cited line numbers.

## Confirmed Residual Cases

1. `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs:12` declares ungated `pub const FLOW_EXAMPLE_TEXT = include_str!("../🧫️fixtures/📝️text/🗣️.dsl.semio")`. This puts fixture content in the production schema module. Only observed callers are the text and binary unit tests. Gate the constant with `cfg(test)` or move it into test ownership.

2. `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts:3028-3040` names actual production interaction records `ModelDefinitionInteractionFixture` and `interactionFixtureRow`. `shippedInteractionJsons()` obtains `modelDefinitionInteractionCatalog()` and `shippedSpatialInteractionCatalog()` maps it. These records are real model assets; canonical model terminology is required.

3. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts:890,962-971` duplicates that actual-model `ModelDefinitionInteractionFixture` and `interactionFixtureRow` naming. `modelDefinitionInteractionRegistry()` at 1003-1010 parses and compiles the real catalog into runtime interaction specs; `loadSpatialInteraction()` at 1017-1025 resolves and caches them. Comments at 987 and 1002 describe actual shipped assets as fixtures. This is a runtime-domain misuse, not an incidental testing identifier.

4. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts:14` statically imports `createFreshComponentTests` from `../🧪️tests/🆕️fresh-component/🟦️.ts`; line 397 instantiates it at module evaluation and 398-400 expose test functions. The describe package `📦️packages/🦀️rust/📜️script.ts:6` imports both the normal `DescribeComponentScript` and these tests from that module, so ordinary describe/build loading admits the synthetic test source. Dispatch-local dynamic imports should own test construction.

5. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:6,393-394` statically imports and constructs `createPluginRunnerTests` from `../../🧪️tests/🏃️runner-self-tests/🟦️.ts`. Its normal command router shares this module. The fixture-reading self-test factory should load only when test dispatch runs.

6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:10,28,60` statically imports and constructs `createCachePolicyTests` from `./🧪️tests/⚡️cache-contracts/🟦️.ts`, exporting `testCacheContracts` from the instance. This is the analogous script-load dependency on synthetic self-test source; use dispatch-local construction.

## Concrete Acceptable Cases

- Puzzle artifact retained job manifests for 2d/3d/5d are `include_str!` calls inside `#[cfg(all(test, feature = "component-app-assembly"))]` test catalog functions. They do not enter normal artifact compilation.
- Trinity rewriting editor `NAKAGIN_CHILD` at line 98 and `nakagin_fixture()` at 106 are both now `#[cfg(test)]`. Their embedded committed content is test-only.
- Hub Cargo default features are `sqlite,native-artifact-execution`; `integration-fixtures` is a separate explicit feature. There is no default fixture feature admission in the inspected manifest.
- Browser bundle `📜️script.ts:896-897` dynamically imports and constructs `createBrowserBundleTests` inside its test function. This has the desired dispatch boundary.
- Schema validator ownership proof imports a fixture but its entrypoints are assertion-based tests and test-script dynamic imports; the schema AJV oracle imports validation vectors and vendor vocabulary, with observed test consumers. Those are test/oracle tooling, not product document loaders.
- OS parity journey and scale benchmark modules import fixture data for explicit parity and benchmark execution; this is a testing use. Canvas mounted-input oracle and glTF contract vectors are also test surfaces.
- `DevServeFixture` is a stale region/comment label around actual serve lifecycle helpers, not a fixture-data dependency. It can be renamed for clarity but does not independently establish runtime loading of fixtures.
- Existing Storybook raw parsers and comments describing real bundled DSL examples must not be treated as fixture-runtime dependencies from spelling alone; the actual paths and owned example assets decide their classification.
- Physical sanitary fixtures and pet domain fixtures require domain-sensitive classification. A mechanical ban on the word would misclassify genuine domain nouns.

Known active scope (Puzzle3d scene model names, import/export snapshot API, Trinity before graph terminology, generic test inputs) was not duplicated. No additional source modifications were made by this audit.
