# Result: REPO-TEST-VSCODE-SPLIT (`rtv`)

Executed 2026-10-16 by the Sonnet executor. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`, `R` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`, `T` = `R/🧪️test`, `V` = `R/💻️client/🧩️vscode`.
Tools: `F/rtv-tools/` (`graph.mjs`, `split.mjs`, `entry.mjs`, `repoint.mjs`, `lossless.mjs`, `api.mjs`, `shape-diff.mjs`, configs `cfg-test.mjs` and `cfg-vscode.mjs`, the library hand-off script and the generator-registry entry). Logs and the original-file snapshots: `F/🗑️generated/rtv-*`.

## 1. Verdict

Both splits are applied, compile, and keep behaviour.

| Target | Before | After |
| --- | --- | --- |
| `T/🟦️.ts` (repo test harness) | 7,656 lines, 446 statements, 337 exports | deleted. 32 child modules (18 to 1,116 lines each, 8,105 lines in total including docstrings, import blocks and repeated region markers). The 337 exports are unchanged and are re-listed explicitly by the package entry `T/📦️packages/🟦️typescript/🟦️.ts` (211 lines). |
| `V/🟦️.ts` (VS Code client) | 3,600 lines, of which about 800 are a dead hand-kept GraphQL codegen copy | 153 lines (header, `registerSidebarViews`, `activate`, `deactivate`: the extension entry that is genuinely its own) plus 13 children (35 to 501 lines each) |
| GraphQL types | hand-kept, drifted, mostly swallowed by an unclosed block comment | generated from the served schema into `V/🤖️generated/🔗️graphql-types/🟦️.ts` (982 lines) by a new generator `V/🔗️graphql-types/` |

Verified (details in section 6): `tsc --noEmit` of both packages exits 0; the harness test failure set is identical before and after (14 fail + 4 errors, same 10 named cases and same error lines); the public API of the harness is identical (`getExportsOfModule`, 337 names with kind and type text, empty diff); every statement is accounted for (446 of 446, 0 mismatches; for the VS Code file the only missing statements are the deliberately deleted ones); the generated types agree with graphql-js on every type, field, nullability, argument, union member and enum value; 125 of 125 differences against the old hand-kept copy are explained by the served SDL.

Not verified, and why (section 8): the Nx route `@semio-tech/repo-test:test` (stuck more than 20 minutes in `build:selected-compile-imports`, killed) and `@semio-tech/repo-vscode:generate-graphql-types` (launcher refuses with a foreign error); the VS Code extension-host Mocha tests (need a VS Code host); the `eslint` lint target.

Blocked by the instruction not to touch `📚️library/**`: two library tests and some library data still name the deleted `T/🟦️.ts`. A ready, hash-guarded patch is in section 7.

## 2. Baseline (recorded before the first edit)

Importers of `T/🟦️.ts` (`git grep -F '🧪️test/🟦️.ts'` plus a resolver over every file under `T`, `FILES` below):

- Direct importers of the file itself: `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` (static import), `T/🧪️tests/🧫️case-fixtures/🟦️.ts`, `T/🧪️tests/🧬️mutation-fixtures/🟦️.ts`, `🧰️framework/🔨️modules/🧪️test/🔌️adapter/🧪️tests/🟦️.ts` (dynamic `import()`), `R/📚️library/🧪️tests/🧭️fixture-placement/🟦️.ts`, plus the package entry `T/📦️packages/🟦️typescript/🟦️.ts` (`export * from "../../🟦️.ts"`).
- Source-text readers: `R/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` (parses `mutationVectorRegistryBreaches` out of the file text); `🧰️framework/🔨️modules/🧪️test/🧪️tests/🧫️fixtures/🔌️adapter-ownership/🔣️.json` (`removedExports.source`, read by the adapter test).
- Importers through the package entry (about 41 plugin generators and mutate tests under `✏️s`, ten framework tests, and every orchestrator under `T`). They keep working unchanged because the entry still exports the same 337 names.
- References by path in data or comments: `R/📚️library/📦️packages/🟦️typescript/📋️project.json` (2 nx inputs), two library fixture JSONs, the Rust runner and Python host doc comments, one os comment.
- The root `📜️script.ts` imports neither target, so it was not touched.

Importers of `V/🟦️.ts`: only the package entry (`export * from "../../🟦️.ts"`, the bundle entry), the tsconfig include, and `V/🧪️tests/🧩️extension/🟦️.ts` (through the package entry, 25 names). One library fixture row names it as an owner (unchanged, the file still exists).

Baseline commands and results (logs in `F/🗑️generated/`):

| Command | Result |
| --- | --- |
| `tsc -p tsconfig.json --noEmit` in `T/📦️packages/🟦️typescript` (`rtv-tsc-test-baseline.log`) | exit 0, no output |
| same in `V/📦️packages/🟦️typescript` (`rtv-tsc-vscode-baseline.log`, with `--listFiles`) | exit 0 |
| `bun test` of the 7 files the package `test` script runs (`rtv-test-baseline.log`) | 134 pass, 14 fail, 4 errors, 148 tests, 195 s |
| `bun ./📜️script.ts doctor` (read-only CLI route), from an extracted HEAD copy of `T` | 5 `[doctor]` lines (`rtv-cli-doctor-base.txt`) |
| `bun ./📜️script.ts repo` (prints the discovered cases as JSON), from the same HEAD copy | 635 cases, 264,138 bytes (`rtv-cli-repo-base.json`) |
| `vite build` of the extension and of the extension test bundle, from an extracted HEAD copy of `V` | both build (`rtv-build-vscode-base`, `rtv-build-vscode-test-base`) |
| `bun test` of the norm oracle-ownership suite (importer, run against a HEAD copy of `T`) | 3 pass, 3 fail |
| API of `T/🟦️.ts` (`api.mjs`, TypeScript checker, name + kind + type text) | 337 exports (`rtv-api-test-before.txt`) |

Honest note on the Nx route: `bun repo run @semio-tech/repo-test:test` was started for a baseline and ran for more than 20 minutes in `build:selected-compile-imports` while the machine was loaded; I killed it. That log (`rtv-route-test-baseline.log`) is not a baseline, because my edits began while it was running. The baseline above is the exact `bun test` command line the package `test` script uses (it adds `repoTestArtifactEnvironment`, so env-dependent cases such as "Caller-owned ticket output required" fail in both runs equally).

## 3. Target 1: the repo test harness

### 3.1 How the seams were found

`graph.mjs` parses the file with the TypeScript compiler and resolves every identifier with the checker (property names, parameters and locals do not count as references). It builds the dependency graph of the 446 top-level statements, classifies each edge as eager (evaluated at module load) or lazy/type-only, and groups statements by `//#region`. Findings: zero eager cross-region edges, so any acyclic split is safe at load time; the 36 regions condense into one 17-region cycle, but the statement graph itself is nearly acyclic (one cycle of 3 statements). A region-to-module assignment plus a handful of statement moves gives an acyclic module graph, including type-only edges (`eval.mjs` reports the module cycles and the backward edges for a candidate assignment).

### 3.2 The 32 children (`T/<emoji>️<kebab>/🟦️.ts`)

Low to high in the import order. "Regions" are the original `//#region` names the statements come from.

| Child | Lines | Holds |
| --- | ---: | --- |
| `🗂️taxonomy` | 190 | `🔣️Contract`: `testTaxonomy`, `TestTaxonomy`, file-kind and location helpers, `isExcludedTestPath`, `CORE_COMPARISON_PROFILES` |
| `🔢️digests` | 32 | `#⃣Digest` |
| `🟰️comparison` | 141 | `⚖️Comparison` |
| `🧭️case-discovery` | 137 | `🔍️Discovery`, `🧭️Root` (`repoRootFromHere`), plus `testCollectionIndexes`, `testFixtureCollectionIndex` |
| `⚡️output-cache` | 77 | `⚡️Cache` |
| `🗃️oracle-registry` | 149 | oracle entries, linked packages, adapter registration, no-oracle decisions, host packages (from `📇️Registry`) |
| `🧮️mutation-catalog` | 191 | mutation vector and catalog records, profile slug rules, `mutationCatalogProblems` (from `📇️Registry`) |
| `🪆️subset-policy` | 164 | `🪆️Subset` |
| `📐️schema-contracts` | 1,116 | `🧬️SchemaContracts` |
| `✅️schema-validation` | 322 | `🧬️SchemaValidation`, `🧫️SchemaFixtures` |
| `📑️mutation-manifest` | 297 | `🧬️Manifest` |
| `🔬️probes` | 79 | `🔬️Probe` |
| `📏️tolerances` | 67 | `📏️Tolerance` |
| `📤️results` | 78 | `📤️Results` |
| `🔀️comparison-pipelines` | 170 | `⚖️Pipeline` |
| `🤝️contributions` | 261 | `TestContribution`, `OracleRegistry`, `loadOracleRegistry`, contribution discovery (from `📇️Registry`), plus the registry accessors `manifestsOwning`, `pipelineTable`, `probeTable`, `toleranceProfileTable` |
| `🔐️run-lease` | 126 | `🔐️Lease` |
| `🗝️run-key` | 84 | `🗝️RunKey` |
| `🧷️fixture-evidence` | 143 | `🧫️Fixture` |
| `🗄️blob-storage` | 101 | `🗄️Storage` |
| `📥️fixture-inputs` | 80 | `🧫️Fixtures` |
| `🌿️mutation-leaves` | 560 | `🪪️LeafDescriptor`, `🧬️PayloadSchema`, `🏗️LeafScaffold` |
| `🔒️oracle-dependencies` | 315 | `🔒️Dependencies` |
| `🚨️breaches` | 18 | `TESTING_BREACH_KINDS`, `TestingBreachKind`, `breach` (from `🧾️Contract`) |
| `🗺️test-layout` | 623 | the test-layout scan of `🧾️Contract` (path assessment, source signals, `scanTestLayout`) |
| `🚧️mutation-gates` | 956 | `🧾️ContractV2` |
| `📃️case-contract` | 284 | `validateCaseContract` and the mutation-vector gates (from `🧾️Contract`) |
| `🏛️repository-contract` | 235 | `repositoryContractBreaches`, `oracleImportsInProduction`, migration status (from `🧾️Contract`) |
| `🎬️execution-plan` | 178 | `📋️Plan`, `🏃️Coordinator` |
| `🔌️adapter-context` | 119 | `🧭️Adapters` |
| `🗑️retention` | 317 | `🧹️Clean`, `🧹️GarbageCollection` |
| `📈️reporting` | 495 | `📊️Reports`, `📈️Metrics`, `📈️CoverageV2` |

The census sketch named `taxonomy/`, `discovery/`, `oracle-registry/`, `mutation-catalog/`, `contributions/`, `digests/`; all six exist. The discovery child is `🧭️case-discovery` because a sibling `🔍️discovery` directory already exists.

Cohesion decisions that needed statement moves out of their region (each makes a module cycle disappear; verified by `eval.mjs`, and removing any one of them re-creates a cycle):

- `testCollectionIndexes`, `testFixtureCollectionIndex` (from `🧾️Contract` to `🧭️case-discovery`): discovery used them while layout lives above it.
- `breach` and its kind table (to `🚨️breaches`): used by `🌿️mutation-leaves`, `🚧️mutation-gates`, layout and the contracts.
- `manifestsOwning`, `pipelineTable`, `probeTable`, `toleranceProfileTable` (to `🤝️contributions`): they are accessors of `OracleRegistry`; leaving them beside the record types made `registry <-> manifest/pipeline/tolerance` cycles.
- The `📇️Registry` region is cut three ways, and `🧾️Contract` five ways, along the sketch's concerns.

Resulting module graph: acyclic (type-only edges included), 32 nodes. No module-level `let` is shared (the two that exist, `taxonomyCache` and `coreProfileTableCache`, are written only in their own module).

### 3.3 Mechanics and losslessness

- Each child starts with a docstring whose first character is the folder emoji (unique among siblings), then a `//#region 🔌️Adapters` block holding only the imports that child uses, then the original region blocks, copied byte for byte from their original line ranges.
- Relative import specifiers are recomputed from the child's location. Cross-child imports use `type` for type-only uses.
- 16 declarations that were file-private but are now used across children received an `export ` prefix (the only text change inside moved statements): `MutationCatalogAuthority`, `isPlainObject`, `strictMutationCatalogs`, `walkDirectories`, `breach`, `canonicalTestCaseName`, `testCollectionIndexes`, `testFixtureCollectionIndex`, `SKIP_DIR_NAMES`, `PROFILE_MARKER`, `TEST_INPUT_URI_RE`, `rawTaxonomy`, `profileDirectoryIsCanonical`, `MANIFEST_MUTATION_ID_RE`, `readJson`, `SHA256_RE`. They are not part of the package entry, so the public surface did not grow.
- Region markers: every original marker pair is kept. A region whose statements went to several children keeps its name in each of them, so these names appear more than once now: `🔌️Adapters` (the import block, 32 files), `🧾️Contract` (+4), `📇️Registry` (+2), `🧬️Manifest`, `📏️Tolerance`, `⚖️Pipeline` (+1 each, because the registry accessors moved). The `🧲️Header` license region moved to the package entry.
- `lossless.mjs` (`rtv-lossless-test.log`): 446 of 446 statements present exactly once in the children, text equal after stripping the 16 inserted `export ` prefixes (0 mismatches). Of the 7,129 non-blank original lines, 18 are not found in the children (the 2 license comment lines, which moved to the package entry, and the 16 changed declaration lines); the other differences are the regenerated imports, docstrings and repeated region markers. One note on the original: it contained 14 orphan docstrings (the types they described live in `🔌️adapter` now); they travelled with the next statement unchanged.

### 3.4 Package entry and importers

- `T/📦️packages/🟦️typescript/🟦️.ts` stays the package's public surface (`exports["."]`, 41+ importers rely on it) and now lists the 337 original names explicitly per child (`export { … } from` for values, `export type { … } from` for types), after a docstring and the license header region. It is not a new barrel: it was already `export * from "../../🟦️.ts"`. Writing the types as inline `type X` specifiers made the library classifier read the file as an "implementation" ("domain type declaration is owned inside the package"); separate `export type` statements classify as "declaration" like before (`classifyPackageSource`, checked).
- Re-pointed to the children (all re-read before editing, produced by `repoint.mjs`, which splits the named specifiers by owning child): `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` (1 import into 3), `T/🧪️tests/🧫️case-fixtures/🟦️.ts` (1 into 4), `T/🧪️tests/🧬️mutation-fixtures/🟦️.ts` (1 line), `🧰️framework/🔨️modules/🧪️test/🔌️adapter/🧪️tests/🟦️.ts` (the dynamic import of `makeAdapterContext`, 1 string), `🧰️framework/🔨️modules/🧪️test/🧪️tests/🧫️fixtures/🔌️adapter-ownership/🔣️.json` (`removedExports[0].source` now names the package entry, which the adapter test parses; 9 of 10 cases of that suite pass, the tenth needs a Script process invocation envelope and fails before reaching the changed line).
- Comment-only path updates to the child that holds the named item: `T/🏃️runner/🦀️.rs` (2 doc comments), `T/🖥️host/🐍️.py` (1), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts` (1).
- Ticket scripts under `.🧬semio/**` that import `T/🟦️.ts` by absolute path (about 50, from older tickets) were not touched. They are historical scratch.

## 4. Target 2: the VS Code client

### 4.1 What the "codegen copy" really was

Lines 57 to 70 of the old file declared `DocumentNode` and seven helper types (`Maybe`, `InputMaybe`, `Exact`, `MakeOptional`, `MakeMaybe`, `MakeEmpty`, `Incremental`). Line 67 then opened `/** All built-in ands = {` (a sweep had cut the closing `*/` and the `export type Scalars` line away). That comment swallowed lines 67 to 866 together with every schema type, every `…Args` type, the operation types, `…Document` AST constants and the stray region markers. So the 400 "type" lines the census saw were dead text, and the live parts were the seven helpers, `DocumentNode`, `graphql()` and `DocumentType` (all unused), plus eleven `…Document = graphql(\`…\`)` operations that nothing in the repo imports. Two stray `// #endregion 🧲️Header` lines sat inside it as well.

### 4.2 The generator (`V/🔗️graphql-types/`, wired like `repo-entity-kinds`)

| Piece | Path | Role |
| --- | --- | --- |
| reader | `📖️parse/🟦️.ts` (242 lines) | owned SDL and operation reader (no library): definitions, fields, arguments, lists, non-null, unions, interfaces, enums, inputs, variables, selection sets, inline fragments; unsupported syntax is refused with a position |
| projection | `📽️projection/🟦️.ts` (195 lines) | emits the TypeScript module in the shape of the GraphQL Code Generator `typescript` and `typescript-operations` plugins (sorted types and fields, `Maybe`/`InputMaybe`/`Exact`, `Scalars`, `…Args`, `…Variables`, result types) |
| source | `📥️source/🟦️.ts` | reads the served SDL `🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql` (the committed document "every client reads") and the `…Document` string exports of `⌛️queries` |
| plan | `📋️plan/🟦️.ts` | one target: `V/🤖️generated/🔗️graphql-types/🟦️.ts` |
| commands | `🏃️execution/🟦️.ts` | `GenerateScript`, `CheckScript` (byte compare, never writes), `PreviewGeneratedScript` (generator protocol) |

Wiring: `V/📦️packages/🟦️typescript/📜️script.ts` registers `generate-graphql-types`, `preview-generated`, `check-graphql-types`, `test-graphql-types`, and its `test` command now always runs the graphql proof before the long-only extension-host case. `V/📦️packages/🟦️typescript/📋️project.json` gets the four targets (`generate-graphql-types` is cached, takes the SDL and the generator sources as inputs and declares the generated file as output) and `dev`, `build`, `test`, `compile-extension`, `watch-extension` now depend on `generate-graphql-types` next to the existing `framework-schema:generate`. `tsconfig.json` includes `⌛️queries`, the generator commands and the two tests, so tsc covers what no entry file imports. `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` each got four entries in group `9_gates`, right after the entity-kinds entries and with the same naming scheme (`🦑️Repo 🧩️vscode 🔗️graphql types 🏭️generate | 👁️preview | ✅️check | 🧪️contract`, orders 900.057181 to 900.057184); both files parse (launch.json with JSON, the seed with the TypeScript JSONC parser).

Commands run through the generator code (the Nx route is blocked, section 8): the three command classes were instantiated directly: generate wrote the file, check printed "fresh", appending a line made check report "stale" with the refresh command, preview emitted a one-node protocol plan (31,539 bytes).

`⌛️queries` now types every document: `graphql<RepoStructureQuery, RepoStructureQueryVariables>(…)` with `TypedDocument<TResult, TVariables>` (a branded string), importing the types from the generated module. This makes the generated module part of the compiled closure and fails the generator when an operation stops matching the schema. One operation was removed: `FixDocument` (`mutation Fix`) selects `fix(scope: …)`, which the served schema does not have (graphql-js: `Cannot query field "fix" on type "Mutation"`). It had no importer anywhere. The other ten validate against the SDL.

### 4.3 Is the generated module the same shape as the hand-kept copy?

`shape-diff.mjs` (`rtv-shape-diff-vscode.txt`) reconstructs the dead block (restoring the cut `Scalars` head) and compares declarations with the generated module:

- 111 old declarations, 135 new. 47 are identical in shape (members, optionality, type text, enum values).
- 37 exist only in the old copy (types the served schema no longer has, such as `Autofix`, `BundleMetrics`, `Checkpoint*Contrib`, `Contribution*`, and the operation types of operations that no longer exist: `RepoQuery`, `CodebaseQuery`, `FixMutation`), 61 only in the new one (types added since: `Draft`, `Goal*`, `FilterInput`, `FileKind`, `ContributorBundle`, …, helper types, the ten current operations), 27 differ (field added or removed, nullability, and `Node & {…}` becoming plain objects because `Node` is a union in the served schema).
- All 125 differences are explained by the served SDL, checked automatically with graphql-js (a `-` field really is absent from the SDL type, a `+` field really is present, an `…Args` type matches the field's arguments, `Node` is a `UnionTypeDefinition`): 0 unexplained.
- Intentional shape choices: custom scalars map to `any` (as the plugin does; `DateTime: { input: any; output: any; }`), enums are TypeScript `enum`s with PascalCase members, and `…Args` types follow the plugin's naming.

The hand-kept copy is deleted (regions `🧬️CodegenGraphql` and `🧬️CodegenGql` are gone with it, as are the two stray `#endregion` markers).

### 4.4 The VS Code children (`V/<emoji>️<kebab>/🟦️.ts`)

| Child | Lines | Holds |
| --- | ---: | --- |
| `🌳️tree-providers` | 501 | filter, monorepo and sections tree providers, `buildCliTreeArgs`, `loadAvailableFilterValues`, the two provider boxes |
| `🏬️uri-resolution` | 372 | tree node cache, labels, `navigateToUri`, `parseUri` |
| `⌨️commands` | 358 | `registerCommands` |
| `⌛️queries` | 334 | ten typed GraphQL operations, `graphql`, `TypedDocument` |
| `⚙️data-models` | 261 | `Breach`, `TicketData`, `ContributorData`, `RepoEvent`, … |
| `🎼️utilities` | 221 | logging, binary and workspace resolution, CLI execution, event parsing |
| `🔎️code-lens` | 219 | definition code lenses, gutter decorations |
| `🗺️file-analysis` | 182 | analysis, diagnostics, autofix |
| `🎵️path-helpers` | 88 | scope and ticket path helpers |
| `🎩️globals` | 81 | output channel, diagnostic collections, caches, CLI slot gate |
| `🌪️entity-emoji-registry` | 62 | `ENTITY_EMOJIS`, `ENTITY_ID_REGEX` |
| `🧾️kit-validation` | 55 | kit document validation |
| `🎞️constants` | 35 | constants and UI strings |
| `🟦️.ts` (parent) | 153 | header, `registerSidebarViews`, `activate`, `deactivate` |

Module graph: acyclic. Three statement moves were needed: `buildCliTreeArgs`, `filterProvider`/`monorepoProvider` and `loadAvailableFilterValues` into `🌳️tree-providers` (they belong to the filter provider and break `globals <-> providers` and `uri <-> providers` cycles). 46 file-private declarations became `export`s because other children use them. Importer update: `V/🧪️tests/🧩️extension/🟦️.ts` now imports its 25 names from the owning children (the package entry still re-exports `activate` and `deactivate`).

One behaviour difference you should know: the extension bundle used to export every public name of the old file (the package entry is `export *`); it now exports only `activate` and `deactivate`, which is what VS Code reads (`node` check on the built bundles: before, a long export list; after, `["activate","deactivate"]`). The bundle shrank from 692,620 to 685,820 characters and the test bundle from 775,050 to 739,063 bytes.

## 5. Edits outside the owned set (all re-read just before editing)

| File | Edit |
| --- | --- |
| `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` | 1 import became 3 (importer of the deleted file) |
| `🧰️framework/🔨️modules/🧪️test/🔌️adapter/🧪️tests/🟦️.ts` | 1 dynamic-import string |
| `🧰️framework/🔨️modules/🧪️test/🧪️tests/🧫️fixtures/🔌️adapter-ownership/🔣️.json` | 1 path |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts` | 1 comment path |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | 4 entries each (peers edit both files all the time; my diff is the 48 inserted lines, the rest of `git diff` on them is theirs) |

Nothing under `📚️library/**` was edited. A throw-away copy of the two library tests with the patch applied was created inside `📚️library/🧪️tests/` for a few minutes to prove the hand-off patch and then deleted (`ls` confirms no `rtv` entries remain). The root `📜️script.ts` was not touched.

## 6. Commands run and what they returned

| Command | Result |
| --- | --- |
| `tsc -p tsconfig.json --noEmit` in `T/📦️packages/🟦️typescript` after the split (`--listFiles` shows all 32 children processed) | exit 0, 0 errors (`rtv-tsc-test-after.log`, `rtv-tsc-test-final.log`) |
| `api.mjs` on the package entry vs `rtv-api-test-before.txt` | identical, 337 lines |
| `lossless.mjs` | 446 of 446 statements, 0 mismatches (`rtv-lossless-test.log`) |
| `bun test` of the same 7 files after the split (`rtv-test-after.log`) | 134 pass, 14 fail, 4 errors, 148 tests; the sorted `(fail)` lines and the sorted error lines are identical to the baseline (`diff` empty); the named failures are the 10 below |
| final re-run after the package-entry reformatting (`rtv-test-final.log`, 174 s) | 134 pass, 14 fail, 4 errors; sorted `(fail)` lines and sorted error lines identical to the baseline |
| `bun ./📜️script.ts doctor`, HEAD copy vs working tree | `[doctor]` lines identical (the cache-root hash differs because it includes the invocation directory) |
| `bun ./📜️script.ts repo` (635 discovered cases as JSON), HEAD copy vs working tree | byte-identical (`diff` empty) |
| `tsc -p tsconfig.json --noEmit` in `V/📦️packages/🟦️typescript` after the split | exit 0 (`rtv-tsc-vscode-after.log`, `rtv-tsc-vscode-final.log`) |
| `vite build` extension and extension-test bundle, HEAD copy vs working tree | all four build; bundle exports checked with a stubbed `vscode` module (section 4.4) |
| pure helpers (`slugify`, `extractLeadingEmoji`, `parseUri`, `treeNodeDisplayLabel`, `parseRepoEvents`, `extractRepoResult`, the entity regex and registry) before vs after, with a stubbed `vscode` module | identical JSON for all (`rtv-vscode-diff-*.json`; two cases throw the same errors in both) |
| `bun test ./🧪️tests/🔗️graphql-types/🟦️.ts` in `V` | 6 pass, 0 fail, 1,560 expects (`rtv-test-graphql-types.log`) |
| mutation probe: drop the optional marker in the projection | 2 of 6 tests failed; reverted, 6 pass again |
| norm oracle-ownership suite (importer) | 3 pass, 3 fail, the same 3 as against the HEAD copy |
| `@semio-tech/repo-test` layout scanner (`inspectTestLayoutSources`) over the 51 new untracked files | 0 findings |

The 10 named failures are identical before and after and are not caused by this slice: `HTML source pair manifests retain exact native-reader and parser inputs`, `HTML source pair normalization accepts the primary leaf of a multi-extension kind`, `Nx discovers the same canonical names and semantic owners as the layout vectors`, `Nx test discovery rejects loop, case and feature links and skips generated/opaque trees`, `Nx uses the current supplied candidate inventory with independent no-follow glob membership`, `canonical-fixture-implementation-pair`, `fresh authority links and resident Node and Bun source mutations fail closed`, `ordinary-mutation-unit-test-without-fixture-pair`, `test inference awaits the exact library bootstrap before command helpers`, and `the committed tree is measured by the same checkers > the test platform's own subtree carries no schema-contract finding` (its message lists 10 `schema-catalog-stale` rows in both runs; only the timing differs).

## 7. Hand-offs

1. `📚️library/**` (held). Because `T/🟦️.ts` no longer exists:
   - `R/📚️library/🧪️tests/🧭️fixture-placement/🟦️.ts` cannot load (`Cannot find module '../../../🧪️test/🟦️.ts'`). Fix: import `inspectTestLayoutSources` from `🧪️test/🗺️test-layout/🟦️.ts` and `testTaxonomy` from `🧪️test/🗂️taxonomy/🟦️.ts`.
   - `R/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` (test "mutation vector discovery audits exact grouped owners…") reads the removed file: change `"../../../🧪️test/🟦️.ts"` to `"../../../🧪️test/📃️case-contract/🟦️.ts"` (the function and the helper names its `support` object injects are unchanged).
   - `R/📚️library/📦️packages/🟦️typescript/📋️project.json`: two nx inputs name the file (the `{workspaceRoot}/…/🧪️test/🟦️.ts` rows); replace with the taxonomy, test-layout and case-contract children.
   - Data rows to decide: `R/📚️library/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json` (row with `legacy` = the test package entry, `owner` = the deleted file, `anchor` = `export const TAXONOMY_REL_PATH`; the anchor now lives in `🗂️taxonomy`, and the package entry re-exports 32 owners instead of one) and `R/📚️library/🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` (a row keyed by the file and line 82).
   - Ready patch: `F/rtv-tools/apply-library-repoints.mjs` (dry run lists 1 + 1 + 2 anchor hits; `--write` applies; I did not run it). With the first two edits applied in a temporary copy, the two edited suites no longer fail on the removed file (the "mutation vector discovery…" test passes; the 10 other failures in those two files are unrelated: ENFILE bursts, missing `Cargo.toml` of a foreign crate, a 43 s tree walk). Before my change those files loaded. `repo-source-ownership` itself already fails to open its fixture (wrong relative path, pre-existing).
2. Taxonomy registration (`R/📚️library/🔣️taxonomy.json`, held): the 32 new test folder names and the 13 new VS Code folder names plus `🔗️graphql-types` and its five children and `🤖️generated/🔗️graphql-types` are not registered in `members-of-tests` or `crate-module-children` (see `RESULT-TAXONOMY-SWEEP.md`). Without the rows the whole-repo taxonomy inventory will report them as `directory-kind-unresolved`. The test-layout scanner (which does not use the directory-kind table) reports none.
3. Generator registry (same file): add the entry in `F/rtv-tools/generator-registry-entry.json` (`repo-vscode-graphql-types`, modelled on `repo-entity-kinds`) so `check-generated` knows the new generator.
4. The unused `⌛️queries` operations: nothing in the extension sends them (the CLI-only refactor left them behind). They are kept because they are the generator's operation input; delete them together with the generator input if the extension stays CLI-only.
5. The local `⚙️data-models` types (`Breach`, `TicketData`, …) are still hand-written duplicates of schema shapes; migrating them to the generated types is a follow-up with behaviour risk and was not done.

## 8. Not verified, foreign errors

- `bun repo run @semio-tech/repo-vscode:generate-graphql-types` stops in build-graph inference with `Selected compile source import missing: 🧰️framework/🔨️modules/🛂️manifest/🟦️.ts -> ../🎠️kernel/🟦️.ts` (a peer's in-flight kernel split; not mine). The new `project.json` targets are therefore not exercised through Nx yet. Re-run `bun repo run @semio-tech/repo-vscode:generate-graphql-types` and `…:test-graphql-types` once the launcher loads.
- `bun repo run @semio-tech/repo-test:test`: see section 2. The direct command line was used instead.
- `eslint` (`lint` target) was not run. The VS Code extension-host Mocha suite (`vscode-test`, long level) was not run; its bundle builds and its imports resolve (`tsc`).
- The machine showed `ENFILE: file table overflow` bursts; two Bash calls and one library test run were affected and repeated.
- `rtv-route-test-baseline.log` (killed Nx run) is not a baseline.

## 9. Final state of the working tree

- Deleted: `T/🟦️.ts`. Created: the 32 children under `T` and the 13 children, `🔗️graphql-types/` (5 modules), `🧪️tests/🔗️graphql-types/🟦️.ts` and the untracked generated file under `V`. Rewritten: `T/📦️packages/🟦️typescript/🟦️.ts`, `V/🟦️.ts`.
- Temporary baseline copies of `T` and `V` (extracted with `git archive HEAD` into sibling directories for the before/after comparisons) were removed.
- No ticket tool, no modifying git command, no sub-agent. Debug logs: none added.
