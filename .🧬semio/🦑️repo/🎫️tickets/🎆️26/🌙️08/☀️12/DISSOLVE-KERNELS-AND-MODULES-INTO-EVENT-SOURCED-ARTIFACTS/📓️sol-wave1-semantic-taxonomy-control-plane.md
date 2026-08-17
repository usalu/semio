# Wave 1 Semantic Taxonomy Control Plane

## Outcome

Wave 1 established the schema-first semantic collection API and permanent report-mode control plane without touching the quarantined repo-library TypeScript barrel. The active tree now has a deterministic census, structured consumer graph, duplicate evidence report, manifest/tree validation, module consumer threshold validation, and lowest-common-owner validation.

Enforcement remains intentionally outside `workspace:verify-gate` while the active graph contains findings. No baseline, allowlist, compatibility adapter, path exception, or legacy suppression was added.

## Schema and API

- `🔣️taxonomy.json` declares the canonical `🔣️component.json` manifest, `x-semio` extension, semantic collection kinds, two-consumer module minimum, and legal owner levels.
- `SemanticCollectionSpec`, `SemanticMember`, `SemanticConsumerGraph`, `SemanticCensusRecord`, `SemanticProblem`, and duplicate-evidence types are repository-owned public contracts.
- Active scope is derived only from taxonomy areas in `clean` or `mixed` state. `legacy` and `exempt` roots are not scanned.
- Collection manifests require exact direct-child bijection, exact IDs/directories, kind-specific contracts, and generated provenance.
- Collection language leaves are checked for mechanical assembly; authored behavior is reported.
- Modules validate declared consumers against the resolved production graph, ignore test/example/generated consumers, require two independent component IDs, and validate the computed lowest common semantic owner.
- Candidate duplicates use normalized SHA-256 evidence only and never trigger a semantic disposition automatically.
- Resolvers cover relative imports, TypeScript path/package exports, Go module paths, Python project roots, C# project references, Rust cumulative `#[path]`, and dynamic register/mount evidence.

## Permanent Commands

The following root commands are registered in `📜️script.ts`, matching Nx targets, and launch configuration:

```text
bun ./📜️script.ts generate taxonomy census --ticket <ticket-id>
bun ./📜️script.ts generate taxonomy duplicates --ticket <ticket-id>
bun ./📜️script.ts verify taxonomy report [--scope <semantic-id>]
bun ./📜️script.ts verify taxonomy enforce [--scope <semantic-id>]
```

Launch configuration also gained the previously missing `workspace:verify-gate` entry. `.vscode/launch.json` was regenerated through `@semio-tech/plugin-registry:generate`; it was not hand-edited.

## Deterministic Artifacts

- `📊️semantic-census.json`: 4,253 component records, 21,097 resolved consumer edges, 9,092 report-mode findings, 115 duplicate-evidence clusters.
- `📓️semantic-census.md`: human-readable census companion.
- `📊️semantic-duplicates.json`: machine-readable duplicate evidence.
- `📓️semantic-duplicates.md`: human-readable duplicate evidence with an explicit non-conclusion rule.

Final hashes:

```text
fa3764d7f700ecb37c3eb2848576e9bec098e25d79f4bbbc5e3b4fe19d2478b3  📊️semantic-census.json
3de15c12e3c3707965de9ce540fc361a7d616ef66c4e59064a7ab279650db621  📓️semantic-census.md
680eed0da5539a01b6dfa21f1582e325d889dac18c91bb50deaf5e08bcae79b5  📊️semantic-duplicates.json
efb208fe2e6af74412af5838c49fe504bec479a6a3d48e5a435061af4722c2aa  📓️semantic-duplicates.md
```

An unchanged full census rerun reproduced the same JSON and Markdown hashes. The focused fixture also asserts byte-identical rendering across repeated scans.

## Validation

Passed:

```text
bun --check 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts
bun --check 📜️script.ts
bun nx run @semio-tech/repo-lib:test-quick -- --test-name-pattern "semantic collection census"
  5 pass, 0 fail
bun nx run workspace:generate-taxonomy-census -- --ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
bun nx run workspace:generate-taxonomy-duplicates -- --ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
bun nx run workspace:verify-taxonomy-report -- --scope gltf
  report mode passed; 80 scoped components, 155 scoped findings at that run
bun nx run @semio-tech/plugin-registry:generate
bun nx run @semio-tech/plugin-registry:check
  generated registry and .vscode/launch.json fresh
```

The focused fixtures cover a valid two-production-component module, one-consumer failure, test call sites not increasing consumer count, lowest-common-owner computation, list-root authored behavior, missing/extra manifest children, generic member stems, deterministic output, and cumulative Rust path resolution.

## Existing Blockers

`bun nx run @semio-tech/repo-lib:lint` reaches TypeScript but is red on concurrent framework work outside this lease. The output includes missing `PluginRegistryEntry`, `PluginSourceEvent`, `UiPresence`, `UiStatus`, `UiMenuRef`, and generated manifest types; `StatechartEvent.eventCount` incompatibilities; styling readonly/import-meta errors; and cross-package `rootDir` errors. Filtering the same run produced no diagnostics in the changed discovery component or test file.

The existing `validateTaxonomy` test named `reports a completeness dir missing from the structural set` is stale: it removes `📡️spr`, which is no longer in `artifactChildDirs`, so that unrelated assertion remains red. The shipped taxonomy itself validates with zero problems, and the new semantic taxonomy tests pass.

## Changed Paths

- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json` (generated through Nx)
- `📋️project.json`
- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- The four deterministic ticket artifacts listed above.

The protected `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` was not modified.
