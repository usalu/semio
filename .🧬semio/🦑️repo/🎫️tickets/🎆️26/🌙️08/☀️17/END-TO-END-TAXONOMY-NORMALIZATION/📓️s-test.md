# S-TEST — Taxonomy Normalization Verification

## Scope

S-TEST added permanent, isolated coverage for the frozen normalization API in `🧹️normalization/🟦️.ts` and reconciled the existing discovery-schema tests with the incompatible taxonomy v7 contract. Production normalization, discovery, schema, root scripts, project configuration, launch configuration, AGENTS instructions, and the shared Git repository were not edited by this lane.

Touched paths:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` — test-only `fast-glob@3.3.3`
- This report.

## Fixture and safety model

Every engine test creates a repository beneath this ticket with `mkdtemp`, copies taxonomy v7 into that disposable repository, creates a fixture-only `compose/`, and runs `git init`, local user configuration, `commit.gpgsign=false`, `git add`, and one commit only inside that repository. Each test scopes inventory to `🧪️tests/🧪️fixture`, supplies the fixture's baseline commit and opaque digest, and removes successful-run fixtures in `finally`.

The shared repository and its real `compose/` were never targets of fixture commands. Present opaque trees are checked byte-for-byte and by Merkle digest across rollback. Symlink targets are never traversed. An absent `compose/` remains lexically excluded without producing `opaque-digest-missing`.

## Permanent coverage

The 15 normalization cases cover:

- Canonical JSON, deterministic inventory/plan bytes, stored/self-omitting plan digests, worker/progress inputs, and a language-agnostic file census cross-checked against `fast-glob`.
- Longest compound extensions, generic semantic-stem removal, parent-owned semantic kinds, exact fixed/configurable contracts, generated outputs, PNG/BMP assets, NFC preservation, VS16 policy, same-kind collisions, Windows reserved names, and path-byte limits.
- Exact D3D12 extraction from `.../🪟️d3d12/📦️packages/🦀️rust/🦀️backend.rs` to `.../🪟️d3d12/⚙️backend/🦀️.rs`.
- Exact PDF test-case normalization from `🧪️tests/mutate-pdf-1-7/{🦀️component.rs,component.feature}` to the registered case directory with `{🦀️.rs,🥒️.feature}`.
- Exact asset normalization from `🖼️rathaus-ahlen-grundriss.bmp` to `🖼️rathaus-ahlen-grundriss/🖼️.bmp`.
- Structured Rust, TypeScript/JavaScript, Go, Python, .NET, native C/C++, JSON, JSONC, TOML, YAML, XML, and Markdown reference edits, including locations, old/new values, preimage hashes, and move-to-edit links.
- Thin Rust/TypeScript package declarations accepted; substantive domain implementations rejected fail-closed.
- Stale preimage rejection, cancellation, all four injected failure stages (`after-staging`, `after-moves`, `after-edits`, `before-verify`), byte-for-byte rollback, successful apply/verify, and an empty second plan.

Existing taxonomy tests were also migrated away from all removed v6 filename, area-state, plugin-state, snapshot/diff, and ecosystem filename fields. They now assert v7 file-kind IDs, semantic directory registries, clean-area enforcement, and v7 structural relationships. A legacy-key census over the test file returns no matches.

## Acceptance evidence

Final focused Bun run:

```text
$ bun test './🧪️index.test.ts' -t 'taxonomy normalization'
15 pass
196 filtered out
0 fail
180 expect() calls
Ran 15 tests across 1 file. [7.36s]
```

Final v7 discovery-schema reconciliation run:

```text
$ bun test './🧪️index.test.ts' -t 'loadTaxonomy|validateTaxonomy'
38 pass
173 filtered out
0 fail
163 expect() calls
Ran 38 tests across 1 file. [4.00s]
```

Final focused Nx repository-library run used a single-token selector so Nx could not split the test name:

```text
$ bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=taxonomy.normalization
15 pass
196 filtered out
0 fail
180 expect() calls
Ran 15 tests across 1 file. [6.52s]
NX Successfully ran target test for project @semio-tech/repo-lib
```

An earlier `bun ./📜️script.ts lint` run exited 1 on pre-existing package-wide issues outside S-TEST: `import.meta.env`, `import.meta.glob`, and TS6059 cross-product `rootDir` diagnostics. It reported no diagnostic attributable to the S-TEST normalization imports or assertions.

## Retained ticket-local fixture repositories

The first focused run produced 14 ticket-local repositories before `commit.gpgsign=false` was added. The host's global SSH commit signing made fixture construction throw before the test received the fixture and entered its cleanup `try/finally`. Per ticket evidence policy and the coordinator's explicit instruction, these repositories are retained rather than removed:

- `🧪️s-test-absent-opaque-meNWNh/`
- `🧪️s-test-cancel-GCloky/`
- `🧪️s-test-collisions-0gXMTl/`
- `🧪️s-test-failure-after-edits-1EZB6j/`
- `🧪️s-test-failure-after-moves-nQlZKz/`
- `🧪️s-test-failure-after-staging-34FSkg/`
- `🧪️s-test-failure-before-verify-QXI0BJ/`
- `🧪️s-test-inventory-VT74oy/`
- `🧪️s-test-kinds-VgCsaZ/`
- `🧪️s-test-named-pilot-8oSIhR/`
- `🧪️s-test-opaque-wu5YF5/`
- `🧪️s-test-package-glue-ZROImb/`
- `🧪️s-test-references-b80dw1/`
- `🧪️s-test-stale-nP3N9r/`

The originating run was `1 pass / 14 fail`; all failures were fixture commit-signing failures rather than normalization assertions. Subsequent fixture commits explicitly disable signing locally, and the final Bun and Nx runs clean up all newly created fixtures.

## Result

All S-TEST acceptance checks pass. There are no residual focused normalization or v7 schema-test failures.

## S-TEST-DOMAIN-V7

The domain test package was refactored off the removed v6 test filename/path fields without adding compatibility behavior. `🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts` now resolves feature, adapter, contribution, registry, and synthetic-case paths through the production v7 `testFilenameForKind` and `testLocationPath` helpers using `testFeatureFileKindId`, `testAdapterFileKinds`, `testContributionFileKindId`, and `testOracleRegistryLocation`. Its removed-key census for `testFeatureFilename`, `testAdapterFilenames`, `testContributionFilename`, `testOutputMarkerFilename`, `testExcludedPathPrefixes`, `testOracleRegistryPath`, and `testSchemaPath` returns no matches.

The focused v7 helper and synthetic-fixture cases pass:

```text
$ bun test './🧪️index.test.ts' --test-name-pattern='taxonomy.exposes|differential.scenario|ecosystem.s.import.syntax.is.only|mutation.completeness.gate|real-world.artifact.fixtures'
13 pass
56 filtered out
0 fail
25 expect() calls
Ran 13 tests across 1 file. [34.14s]
```

The broader affected selector was also run to expose integration state:

```text
$ bun test './🧪️index.test.ts' --test-name-pattern='taxonomy.exposes|differential.scenario|host.package.carrying|ecosystem.s.import.syntax.is.only|framework.s.own.registry|mutation.completeness.gate|real-world.artifact.fixtures'
13 pass
54 filtered out
2 fail
28 expect() calls
Ran 15 tests across 1 file. [72.40s]
```

The two residual failures are production migration gaps, not v6 expectations in the test:

- `testOracleRegistryLocation` canonically resolves `🧪️test/📇️registry/🔣️.json`, but the current tree still contains only `📇️registry/🔣️component.json`; the open/closed test therefore fails with `ENOENT` at the v7 path.
- The cross-language host-package test exceeded its existing 30-second timeout while repository contribution discovery ran against the same not-yet-normalized contribution layout.

The schema/policy lane confirmed that its scope defines and validates the v7 location contract but does not perform these production path moves. S-TEST-DOMAIN-V7 deliberately did not add a present-file fallback, raise the timeout to mask the migration gap, edit production/package metadata, modify Git, or access Compose.

## S-TEST-PHYSICAL-LEAVES

The repository-library taxonomy tests now follow the frozen physical-format leaf invariant. Test semantics live in registered `🧪️…` parent directories; the leaf emoji is exclusively the physical file kind:

- Rust: `🦀️.rs`
- TypeScript: `🟦️.ts`, `🟦️.tsx`, and the genuine compound form `🟦️.d.ts`
- JSON: `🔣️.json`
- Markdown: `📝️.md`

The former `🧪️component.test.ts → 🧪️.test.ts` fixture and expectation were removed. The permanent physical-leaf case now proves generic `component` stem removal into its owning test case, explicit semantic parents for React, declaration, JSON, and Markdown members, and physical leaves beneath each parent. A normalization-section census for `.test.{rs,ts,tsx}`, `/🧪️.{rs,ts,tsx,json,md}`, and `🧪️.test` returns zero matches.

The named pilots remain covered in valid frozen contexts:

- D3D12 still resolves `.../🪟️d3d12/📦️packages/🦀️rust/🦀️backend.rs` exactly to `.../🪟️d3d12/⚙️backend/🦀️.rs`.
- The PDF pair now starts inside the registered `🧪️tests/🧪️mutate-pdf-1-7` case and resolves to sibling `{🦀️.rs,🥒️.feature}` physical leaves.
- The Rathaus BMP still resolves to the registered semantic asset directory `🖼️rathaus-ahlen-grundriss/🖼️.bmp`.

The loader expectations were reconciled to the same frozen contract: `windowEmptyFacetFileKindId` is `markdown`, and Rust/TypeScript example tests use the ordinary `rust-source`/`typescript-source` file-kind IDs rather than removed role-specific test kinds. The focused loader gate is green:

```text
$ bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=loadTaxonomy
9 pass
202 filtered out
0 fail
91 expect() calls
Ran 9 tests across 1 file. [2.44s]
NX Successfully ran target test for project @semio-tech/repo-lib
```

Final focused normalization evidence uses the no-space/equal-form selector:

```text
$ bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=taxonomy.normalization
15 pass
196 filtered out
0 fail
182 expect() calls
Ran 15 tests across 1 file. [4.96s]
NX Successfully ran target test for project @semio-tech/repo-lib
```

All prior pilot, fixed/configurable contract, collision/platform/path-budget, structured-adapter, package-purity, stale-apply, four-stage failure injection, rollback, cancellation, opaque-prefix, and empty-second-plan cases remain in that passing 15-test packet.

The package lint target was also attempted and remains blocked by six production diagnostics outside the exclusive S-TEST write scope:

```text
$ bun nx run @semio-tech/repo-lib:lint
exit 1
TS2339: import.meta.env and import.meta.glob in UI styling
TS6059: four cross-product files outside the repo-library rootDir
NX Running target lint for project @semio-tech/repo-lib failed
```

No diagnostic names the touched test file. S-TEST did not mask or modify these production defects.
