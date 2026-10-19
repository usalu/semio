# Result vtax: `validateTaxonomy` decomposition (taxonomy-authority)

Executor `vtax`, TypeScript only (no crate). `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery`. Tools: `C/vtax-tools/` (`C` = this ticket folder).
Pre-split snapshot (differential oracle): `C/vtax-tools/orig/taxonomy-authority.ts`. Nothing was committed, staged or reverted.

## 0. Verdict

| file | before | after | note |
| --- | ---: | ---: | --- |
| `D/🔐️taxonomy-authority/🟦️.ts` | 1,805 | 505 | the mutual-recursion group stays here, plus the thin `validateTaxonomy` shell (31 lines) |
| 20 rule group validators | (1,281-line function) | 20 to 193 each | one function in one new sibling folder each |
| `D/🪣️taxonomy-audit-context/🟦️.ts` | n/a | 109 | `TaxonomyAudit`, `TaxonomyAuditPorts`, `createTaxonomyAudit` |
| `D/🪜️audit-schema-scope-vocabulary/🟦️.ts` | (inside the file) | 63 | former private `validateSchemaScopeVocabulary` |

Largest new file 193 lines, largest file overall 505 (target 600, limit 1,000). 22 new folders, all resolve to the same engine kind as the authority folder (`crate-module-children`), unique RGI emoji among the 71 siblings, docstring emoji = folder emoji, longest path 148 bytes.

## 1. Design

The cycle was `loadCatalogTaxonomy` to `validateTaxonomy` to helpers whose default parameter is `loadTaxonomy()`. Rule groups need 8 of those helpers, so any naive split puts a module cycle between the groups and the authority. Solved by dependency inversion, with no API change:

- `🔐️taxonomy-authority` keeps all 23 units of the recursion group (`loadCatalogTaxonomy`, `loadTaxonomy`, the 14 helper functions with `= loadTaxonomy()` defaults, `validateTaxonomy`, `semanticPackageAdapterPreview`, `semanticPackageSourceOutputPhase`, `nestedCargoGeneratedPrestate`, `validateGeneratorContractsAgainstWorkspace`, `pathIsExcluded`, `registryCatalogInputView`). Its exports and signatures are unchanged, so none of the ~160 importers changed.
- `validateTaxonomy(taxonomy)` builds one `TaxonomyAudit` through `createTaxonomyAudit(taxonomy, ports)` and calls the 20 rule groups in the original statement order. `ports` are the 8 authority functions the rules call (`semanticDirectoryKindId`, `fileKindIdForSourcePath`, `canonicalFilenameForKind`, `canonicalFilenamesForKind`, `taxonomyCliAttemptPreparationsProblems`, `taxonomyCliLeaseDirectoryProblems`, `artifactPathProjectionCatalogRoots`, `semanticDescendantNodeRelativePath`). Rule groups import only types from the context module, never the authority.
- The shared closures of the old function body (`problems`, `pathMatcher`, `canonicalEmoji`, `document`, `record`, `ids`, `pattern`, `fullPattern`, `pathPattern`, `workspacePath`, `extensionChain`, `exactKeys`, `kebabId`) moved verbatim into `createTaxonomyAudit`; groups destructure what they use under the same names, so even thrown `TypeError` texts stay identical.
- Statements are verbatim and in order; the only edits are the destructuring header per group, `validateSchemaScopeVocabulary(taxonomy)` to `(audit)` plus its signature, and the removal of the two `//#region 🪞️SemanticPathProjection` / `//#endregion` folding markers (the region spanned four groups; nothing reads them).

Rule groups (function, folder under `D`): `auditSchemaVersionAndEvidence` 📆️audit-schema-version-and-evidence, `auditLeafAndReferencePolicies` 📎️, `auditFileKinds` 📔️, `auditScopedAndDirectoryKinds` 📗️, `auditTransactionArtifacts` 🚥️, `auditMemberAndDescendantContracts` 🛤️, `auditProjectionAndPackageContracts` 🚏️, `auditOwnedFileProjection` 🔑️, `auditReferenceConsumersAndMutationCatalog` 🗞️, `auditFixedContracts` 🔩️, `auditPackageBoundaryContracts` 🚛️, `auditGeneratorContracts` ⚗️, `auditPathPoliciesAndAreas` 🛑️, `auditDependencyDirections` 🚇️, `auditEcosystemsAndTargets` 🌄️, `auditTestVocabulary` 🎲️, `auditManifestAndRootContracts` 📙️, `auditSchemaFormatsAndFacets` 📘️, `auditMutationVocabulary` 💉️, `auditRepresentationAndChildDirectories` 🛖️ (folder slugs are `audit-*`, see `ls D`).

Generator: `C/vtax-tools/split.cjs` (+ `spec.cjs`, `context-header.ts`, `context-imports.txt`), TypeScript-checker based, refuses to write if the authority changed since the snapshot. Backup of the replaced file: `/Users/ueli/.cache/vtax/backup/authority.ts`.

## 2. Verification (all run here)

| check | result |
| --- | --- |
| Lossless (`verify-text.cjs`) | 172 original statements (validator body + scope vocabulary) equal the 172 new ones, 0 missing/extra: LOSSLESS OK |
| Type check (`tscheck.cjs`, repo strict options, 285-file program, negative control with an injected typo) | 0 diagnostics for the old file and for the 23 new files |
| Import cycles | own Tarjan over static imports reachable from the authority: 97 modules, 0 cyclic SCCs; dependency-cruiser `no-circular` over `D`: 596 modules, the 47 existing cycles are all outside `D`, none involves a discovery module |
| Live taxonomy engine (`kinds.ts`) | all 22 new folders resolve to `crate-module-children`, 0 shape problems |
| Differential old vs new (`diff.test.ts`, frozen taxonomy snapshot sha256 `c46631b6…a4b` at `/Users/ueli/.cache/vtax/taxonomy.base.json`) | real taxonomy plus 5,803 deterministic mutated variants (delete, null, wrong type, append, flip, reverse, extra key, bad regex/glob, targeted projection-activation/scope/owner-cycle/descendant-contract cases), each run on separate deep clones: **0 mismatches** (3 shards: 1,935 + 1,935 + 1,934). Baseline: 0 problems in both. 3,395 variants produce diagnostics (identical arrays), 646 variants throw (identical `Name: message`) |
| Negative controls | changing one message in a new group file: 4 of 5 targeted variants mismatch, and the new golden test fails; restored afterwards, 0 mismatches |
| Coverage of the 22 new modules by the variants (bun lcov, merged) | 100% of lines in 21 files; `auditMemberAndDescendantContracts` 168/180 (the `draw-editor-command-bundle-v1` block was not reached). Push-site coverage lower bound (shards 1+2 only): 295 of 444 literal `problems.push` sites triggered, so the differential is strong but not exhaustive for the rarest messages |
| Existing tests, pre-split vs post-split (`runtests.ts`, 79 files that import the authority) | 40 files fail in both states with identical pass/fail/error counts (swapped the three edited files back for the baseline run, restored afterwards, hashes verified): all pre-existing/unrelated (e.g. self-contradicting `artifactFacetPathIsDeclared` assertion, missing packages, stale source-ownership vectors). 39 files fully green in both. `taxonomy-pattern-compiler-reuse`: 93 pass |
| `bun repo --help` | boots after the split, after the config edits and at the end |

Run tests with `bun test --tsconfig-override /Users/ueli/.cache/vtax/tsconfig.paths.json ./<path>` (the `./` prefix matters: without it bun treats the path as a filter and scans 7.5M files before crashing; the override maps packages that exist only in `node_modules/.bun` such as `jsonc-parser` without touching `node_modules`).

## 3. Text readers, configs and tests changed

- `🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts`: the pin on `validateTaxonomy` (1 `createTaxonomyPathMatcher`, 2 `pathMatcher.matches`, no one-off matcher calls) now resolves the validator and the 21 audit owners (context factory + 20 groups) from the authority's imports; the matcher type law also covers those files. The old assertion called `getText(source)` with nodes of another source file; the new one uses each node's own file.
- `🧪️tests/🔬️workspace-contract/📖️taxonomy-loading/🟦️.ts` (new tests, 740 lines): (1) 20 golden corruption vectors, one per rule group, whose expected problem arrays were produced by the pre-split validator; (2) no audit module reaches the authority module and each stays at most 600 lines.
- `🧪️tests/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json` (`copies`) and `🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json` (`deny`): the 22 new modules added in sorted position (explicit module closure lists).
- Unchanged and still valid because the declarations stayed in the authority: `root-script-compiler`, `artifact-empty-facet-authority`, `registry-catalog-gitlink-boundary`, `cargo-discovery-exclusions`, `📋️project.json` inputs (`discovery/**/*.ts`), transaction provenance paths.

## 4. Open risks

- The new tests live in an existing registered suite, no new test name or `launch.json` entry was needed.
- `taxonomy-loading` has 4 pre-existing failures (identical before/after); `workspace-contract/🟦️.ts` entry could not finish within 200 s under load, its groups were run individually.
- `readme-reviewed-fixture-inputs` and `config-graph` lists must be kept in sync by whoever adds another discovery module.
- Peers edit `🎁️package-projection` and `📖️source-access` concurrently; untouched by me.
- `/Users/ueli/.cache/vtax/` holds the taxonomy snapshot, backup and tsconfig override; regenerate `orig/taxonomy-authority.abs.ts` with `node mkold.cjs` before re-running `bun test diff.test.ts` (shard with `VTAX_SHARD=i/n`, filter with `VTAX_ONLY=<regex>`).

## 5. Crates

None (TypeScript only).
