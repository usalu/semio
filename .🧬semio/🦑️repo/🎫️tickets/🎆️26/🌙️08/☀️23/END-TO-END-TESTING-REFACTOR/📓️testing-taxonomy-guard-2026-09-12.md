# Testing Taxonomy Guard — 2026-09-12

## Scope

This lane owns the repository testing-layout schema, neutral vectors, TypeScript guard, canonical testing taxonomy fields, normalization and discovery consumers, focused implementation tests, generated schema catalogs, and the repository testing README. The final follow-up also corrected the assigned framework plug-in and directory test topology after the JCO and scale fixture relocation.

## Classification Contract

The only testing collection directories are:

- `🧪️tests`
- `🧫️fixtures`
- `📚️examples`
- `🔮️oracles`

A test implementation is a direct file-kind leaf at `<semantic-owner>/🧪️tests/<test-name>/<implementation>`. The guard rejects extra implementation directories, testing-suffixed files outside canonical cases, fixtures inside cases, and fixture roots below delivery scopes.

The obsolete-category classifier operates on exact directory segments. It rejects the closed testing vocabulary, including `testkit`, `test-support`, `testing-support`, `test-helper`, and `test-harness`. Ambiguous production words such as `helper`, `harness`, `mock`, and `stub` are rejected only when the segment carries the test emoji. A semantic case name immediately below `🧪️tests` remains a case rather than a category.

Canonical fixtures are opaque because they may intentionally model invalid or foreign layouts. Production assets and generated outputs are opaque for the same ownership reason. Executable examples and oracles remain inspected and must put their assertions in their own canonical test collections. The scanner inventories real directory entries, including empty forbidden categories.

Cargo workspace membership and `[workspace.dependencies]` are ownership declarations, so they may point at synthetic packages under owner fixtures. Package `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, and `lib`, `bin`, and `build` target paths remain dependency edges and are still enforced.

## Schema-First Regression Evidence

Neutral vectors cover populated and empty legacy testkit/support/helper/harness roots; singular and wrong-emoji oracle roots; the four canonical categories; executable example and oracle tests; opaque invalid fixture payloads; production helper/harness/mock/stub domains; authored inline suites; negative `!import.meta.vitest` production boot gates; fixture imports and reads; and Cargo fixture manifests.

The initial obsolete-category run was red at 65 pass and 2 fail. After schema registration isolated the behavior failure, the legacy-category vector still produced no findings instead of the expected 15. The implementation then added exact-segment classification and physical directory traversal.

The negative Vitest gate vector was independently red with one `inline-test-body` finding for a production boot condition. Following TypeScript unary-expression polarity removed that false positive.

The Cargo workspace vectors were independently red: both `fixture-manifest-workspace-fixture` and `fixture-manifest-workspace-dependency` produced `production-fixture-dependency`. After separating ownership declarations from package dependency edges, the two vectors and the independent TOML/Cargo discovery check passed:

```text
3 pass
0 fail
12 expect() calls
```

Final bounded layout result:

```text
74 pass
0 fail
216 expect() calls
Ran 74 tests across 1 file. [5.55s]
```

Independent evidence in that suite comes from Ajv, minimatch, `Intl.Segmenter`, Node filesystem traversal, a TOML parser, Cargo package discovery, esbuild, rustc, the TypeScript checker, and the isolated Nx workspace.

## Canonical Oracle and JCO Ownership

The taxonomy has one `testOraclesDirName: "🔮️oracles"` field. Discovery, dependency classification, normalization, mutation projection lookup, the Nx plug-in, and cache inputs use that canonical collection without aliases or per-owner overrides.

Earlier focused ownership results remained green:

```text
contribution directory ownership: 4 pass, 0 fail, 29 expects
normalization canonical oracle lookup: 1 pass, 0 fail, 9 expects
taxonomy canonical collections: 1 pass, 0 fail, 7 expects
exhaustive cache independent ancestor walk: 1 pass, 0 fail, 11 expects
```

The final JCO contract run was initially red at 0/2. It found ten taxonomy paths containing the removed `🧩️support` segment and a workspace test assumption about a nonexistent fixture-local destination schema. The schema now names the seven current guest coordinates, vendored shims resolve under `🌐️browser-host/🪞️preview2-shim`, generated variants resolve under `🌐️browser-bundles`, declaration and WASM imports resolve relative to each generated bundle, and the workspace contract compiles the OS schema's `JcoProbeDestinationV1` definition with Ajv.

Final result:

```text
2 pass
0 fail
311 expect() calls
```

## Framework Physical Corrections

- Moved `wire_effect_laws` from the reactor turn production source to `⚛️reactor/🔄️turn/🧪️tests/📡️wire-effect-round-trip/🦀️.rs` and retained private access through an external `#[path]` module.
- Moved the two app selection laws and two plug-in runtime laws from production bodies to direct canonical Rust implementations.
- Folded the cross-case command-page authority source into its actual `🔬️plugin-runtime-plugin-builder-contract` consumer and removed the malformed `📄️command-page-authority` case.
- Moved the recursive composition corpus to plug-in owner fixtures and updated its TypeScript and Rust consumers.
- Folded `FakeTransport` and `FakeWs` into the directory client's direct `🦀️.rs` case and removed the support sibling.

The full `semio-framework-plugin` test target compiles after these changes. Focused runtime results:

| Contract | Result |
| --- | ---: |
| Reactor wire effect laws | 2/2 pass |
| App interaction selection laws | 2/2 pass |
| Plug-in runtime diagnostics/yield laws | 2/2 pass |
| Folded command-page authority laws | 4/4 pass |
| Directory client with folded fake transport | 1/1 pass |
| Composition Ajv/Graphlib oracle | pass |

The command-page runtime recorded exact pacing `1→1 2→2 4→4 8→8`; its 320-command stream completed in 321 turns with zero retained ingress occupancy and zero retained bytes per measured command.

The composition Rust group resolved and parsed the moved fixture. With `RUST_MIN_STACK=67108864`, one of three recursive replacement laws passed. Two behavior laws still failed: one did not reach `ValidatingClosure`, and one observed `Fault` instead of `Ready`. No composition behavior code or fixture bytes were changed by this lane; the independent Ajv/Graphlib oracle passed all eight neutral vectors. This is a product-behavior limitation, not a path-resolution or compilation failure.

## Generated Schema Catalog

The taxonomy path-pattern validator is initialized before `implementationLeafPolicy` validation, and concurrent generator output roots were restored to byte-lexical order before final validation.

Final generation:

```text
[schema generate] .../🔣️schema-catalog.json: 3114 scopes, 6213 diagnostics.
[schema docs] .../📓️schema-catalog.md: 3114 scopes.
[schema generate] .../🔣️schema-catalog.json is current (3114 scopes).
```

## Physical Census

The retained lane census is `🗑️generated/taxonomy-guard/physical-census.json`. It completed with 25 findings during the final concurrent cleanup. All 16 stale findings in the assigned plug-in and directory scope disappeared. The three framework rows left in that snapshot were a renderer JSON fixture and two scale path-constant imports; root subsequently corrected all three without a guard exemption and reported a later three-row repository scan confined to concurrent wiring races. Root owns the final acceptance census.

## Exact Edited Files

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔏️path-emoji-statutes/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔏️path-emoji-statutes/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📐️test-layout/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧭️contribution-directory-ownership/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/⚡️exhaustive-cache-inputs/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️testing-taxonomy-guard-2026-09-12.md`

Added:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📡️wire-effect-round-trip/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕹️interaction-selection-laws/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️plugin-runtime-runtime-laws/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️composition/🔣️.json`

Removed:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🧱️transport.rs`

## Acceptance Boundary

JCO and scale synthetic programs remain under owner fixtures. Their executable assertions remain direct canonical test leaves. No legacy category alias, script-wide exemption, include-wide exemption, or production fixture dependency exception was introduced.
