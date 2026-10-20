# Source Route Input Roles

This is a read-only audit of the 26 exact commands in the retained `e2e-source-single-plan.json`. No manifest, execution route or source-observation policy was edited. The authored ownership registry and every selected target declaration were read directly, followed by their actual script dispatch branches. Other typecheck or native routes in the same script are separate selections.

The selected branches run Bun tests, direct portable oracles, or the root zero-touch verifier. None of the selected branches launches a TypeScript type-check phase. This does not mean their tests cannot use the TypeScript parser or compiler API as an oracle; those imports remain genuine runtime dependencies. Every existing runtime import and data read must remain in the derived closure. No prerequisite is bypassed.

The review input separates actual selected runtime entries from authored data inputs. A literal TypeScript input can be freshness data for a generator and must not become a runtime entry merely because it is a literal path. The runtime entries below are proved by the actual selected command or direct oracle/verifier call; the separate data rows remain data. Existing environment inputs and derived runtime closure must be preserved. This is not approval for a blanket freshness metadata edit. The parent and native execution owner are defining the explicit runtime-entry distinction; the process native-image row is their current concrete case.

| Index | Exact target | Actual selected action | Existing entry coverage |
|---|---|---|---|
| 0 | workspace:test-workspace-build | Bun test, five literal test modules | Build directory glob and root manifest; five actual test entries need explicit classification |
| 1 | workspace:verify-zero-touch | Direct synchronous taxonomy verifier, separate from the typescript branch | Actual zero-touch closure query is a runtime source entry; no Bun test leaf |
| 2 | @semio-tech/repo-lib:test-build-caller-birth | Bun test build/caller/tests/birth | Script literal and broad build glob; actual test literal absent |
| 3 | @semio-tech/repo-lib:test-cargo-workspace-ownership | Bun test Cargo workspace ownership | Script literal and Cargo workspace glob; actual test literal absent |
| 4 | @semio-tech/repo-lib:test-process-budgets | Bun test process-budgets | Named default/reads/testEnv inputs; actual test literal absent |
| 5 | @semio-tech/value-rs:test-borrowed-observations-source | Dynamic import of observation oracle, then direct call | Named defaults and cargoToolchain; oracle entry literal absent |
| 6 | @semio-tech/ui-rs:test-crate-boundaries | Bun test crate-boundaries | Actual test module already literal, plus broad data inputs |
| 7 | @semio-tech/ui-rs:test-text-editor-source | Bun test Textarea cases | Named defaults; actual test literal absent |
| 8 | @semio-tech/repo-dashboard-rs:test-source-freshness | Bun test execution module with source-digest name filter | Named defaults/reads/testEnv; actual test literal absent |
| 9 | repo-mcp:test-install | Bun test installation and Go-layout modules | Named defaults; both actual test literals absent |
| 10 | @semio-tech/repo-cli-go:test-entrypoint-source | Bun test CLI entrypoint | Named defaults; actual test literal absent |
| 11 | @semio-tech/framework-os-mcp:test-native-staging | Bun test stdio staging | Named defaults/reads/testEnv; actual test literal absent |
| 12 | @semio-tech/framework-source-workspace:test-source | Bun test workspace module | Project-wide glob and named defaults; actual test literal absent |
| 13 | @semio-tech/value-rs:test-owned-projection-source | Inline Ajv/SQLite oracle over schema and fixture data | Canonical script is the executed entry; schema/fixture are data, not typed compiler roots |
| 14 | @semio-tech/value-rs:test-immutable-projection | Inline Ajv/SQLite oracle; native branch requires explicit native argument | Canonical script is the executed entry; schema/fixture are data, not typed compiler roots |
| 15 | @semio-tech/repo-dashboard-rs:test-dispatch | Bun test dispatch module | Named defaults and cargoToolchain; actual test literal absent |
| 16 | @semio-tech/repo-lib:test-dashboard-control | Bun test dashboard-control, 330s test / 360s Original bound | Named defaults/reads/testEnv; actual test literal absent |
| 17 | repo:test-workspace-ownership | Bun workspace ownership test, then actual ownership inspection | Script literal and Bun workspace glob; actual test literal absent |
| 18 | @semio-tech/framework-process:test-runtime-environment | Bun test environment suite | Environment/TypeScript directory globs and script literal |
| 19 | @semio-tech/framework-process:test-native-image-observation | Bun test silence suite with retained name filter | Parent already authored literal entry and freshness role |
| 20 | @semio-tech/framework-process:test-cargo-source-admission | Bun test source-admission suite | Source/units/leases globs and script literal |
| 21 | @semio-tech/framework-process:test-process-progress | Bun test progress suite | Routing glob and named defaults; actual test literal absent |
| 22 | @semio-tech/framework-process:test-execution-budget | Bun test execution-budget suite | Project-wide glob; actual test literal absent |
| 23 | @semio-tech/framework-process:test-native-cache-descriptors | Bun test cache descriptors with exact neutral-case filter | Cache glob and script literal; actual test literal absent |
| 24 | @semio-tech/framework-process:test-descriptor-lifetime | Bun test physical descriptor lifetime | Lease glob and named defaults; actual test literal absent |
| 25 | repo:test-cargo-retirement | Bun test pruning/budget inference suite | Named default/reads/testEnv; actual test literal absent |

The process router invokes TSC only for routing, command, capture, compiler-state, cargo-driver, cargo-profile, exact-cargo-laws and exact-cargo-storage. None of these is one of the seven selected process rows. The library routes return after their selected Bun test. The Value source selections do not enter their explicit native branches. The Go CLI's entrypoint-source selection does not enter Go build/test.

The independent reviewer verified both final checks directly: each of the seven selected process suites runs exactly its suite.source, with no appended source entries; the root zero-touch route invokes the synchronous inherited taxonomy helper, which calls policyZeroTouchClosureBreaches. There is no Bun test leaf for that route. The read-only Node tool transport stopped returning during these reads, so the reviewer supplied the exact source evidence; only the agent's own waiting orchestration cells were terminated. No running repository process was signalled. Runtime source verification remains owned by the parent.

The closed hand-authored input is retained in `source-route-inputs.json`, with its schema in `source-route-inputs.schema.json`. Independent Python jsonschema validation accepted the input. All 26 distinct targets match the original roster exactly; all 32 runtime-entry paths and four explicit inline-oracle data paths exist. The agent's Node filesystem session recovered after resetting only its own read-only kernel. These checks validate the review input and its physical references, not execution of the 26 suites or completeness of every transitive closure. No source suite or producer was started for this audit.

## Explicit Runtime Entry Implementation

After the authoritative RuntimeSourceEntries schema was authored, the assigned 25 targets across 11 manifests now declare sourceInputObservation freshness and their exact reviewed runtimeSourceEntries. Index 19 remains parent-owned. Existing input data was retained; the two inline Value oracles also retain their four schema/fixture JSON data inputs. Each narrow mutation checked all unrelated manifest semantics and existing input membership.

The mounted source-roster regression covers all 26 target declarations with shared neutral schema/fixtures, authoritative schema equality, physical paths, independent SQLite extraction, and actual dispatch-owner literal coherence. The seven process cases additionally bind the source literal to the selected suite property. Other dispatch cases retain the manual route audit above; this is not a claim of actual per-route execution. The regression and all selected suites remain unrun here; the parent owns the serial Source batch.

A subsequent read-only live-manifest comparison reconfirmed all26 target runtime entry arrays (normalizing workspace/project-root spelling) and freshness roles still exactly match the shared roster after concurrent work. Zero declaration drifts were observed. This is static metadata coherence, not Source-suite execution.

## Added Freshness DATA Observations

The Source roster still has26targets and32runtime entries. Its exact DATA assertions now retain9paths: prior4projection JSON plus2bounded source-worker poll JSON and3image/cleanup TypeScript source files inspected as DATA. The closed literal schema supports projectRoot/workspaceRoot .json/.ts; actual physical expansion and256-character bound are checked. Comparing expanded paths prevents token-alias runtime promotion. Low accepted the three-file guard update; no runtime pass is implied.
