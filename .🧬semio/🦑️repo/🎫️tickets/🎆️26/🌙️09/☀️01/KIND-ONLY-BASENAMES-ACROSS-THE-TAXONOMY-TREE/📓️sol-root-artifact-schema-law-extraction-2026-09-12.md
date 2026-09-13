# Root Artifact Schema Law Ownership Extraction

Date: 2026-09-13  
Status: complete; focused direct and registered checks are green.

The root artifact-schema policy is split into eleven semantic owners with anonymous `🟦️.ts` leaves. The directory tree owns source admission, artifact owner discovery, facet representation, declared identity, each independent law, and aggregation. Root `📜️script.ts` remains command orchestration and consumes these owners directly.

## Owner inventory

All paths are relative to the repository root.

| Concern                            | Kind                                              | Anonymous owner                                                                                               | Declarations                                                                                                                                                                                                                                                                                           |
| ---------------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| no-follow source admission         | `repo-discovery-source-access`                    | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`                         | `PolicySourceEntry`, `PolicySourceOperations`, `PolicySourceDirectoryResult`, `PolicySourceTextResult`, `POLICY_SKIP_DIRS`, `POLICY_SOURCE_OPERATIONS`, `policySourceUnavailableState`, `policySourceAncestry`, `policySourceDirectory`, `policySourceText`, `policyReaddirSafe`, `policyReadFileSafe` |
| artifact standard/subset discovery | `repo-schema-artifact-owner-discovery`            | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/🔍️owner-discovery/🟦️.ts`               | `PolicyArtifactSchemaOwnerDiscoveryIssue`, `PolicyArtifactSchemaOwnerDiscovery`, `policyDiscoverArtifactSchemaOwners`                                                                                                                                                                                  |
| normative declared identity        | `repo-schema-artifact-export-identity`            | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/🏷️export-identity/🟦️.ts`               | `policyDeclaredSchemaExportName`                                                                                                                                                                                                                                                                       |
| configured facet leaves            | `repo-schema-artifact-facet-leaves`               | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/📚️facet-leaves/🟦️.ts`                  | `POLICY_SCHEMA_FACET_RELS`, `PolicyArtifactSchemaLeaf`, `policyLoadSchemaFacetLeaves`                                                                                                                                                                                                                  |
| facet completeness                 | `repo-schema-artifact-facet-completeness-law`     | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🧩️facet-completeness/🟦️.ts`     | `policyArtifactSchemaFacetCompletenessBreaches`                                                                                                                                                                                                                                                        |
| representation field parity        | `repo-schema-artifact-field-parity-law`           | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/📏️field-parity/🟦️.ts`           | `policyArtifactSchemaFieldParityBreaches`                                                                                                                                                                                                                                                              |
| snapshot state parity              | `repo-schema-artifact-state-parity-law`           | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/💾️state-parity/🟦️.ts`           | `policyArtifactSchemaStateParityBreaches`                                                                                                                                                                                                                                                              |
| diff coverage                      | `repo-schema-artifact-diff-coverage-law`          | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🔺️diff-coverage/🟦️.ts`          | `policyArtifactSchemaDiffCoverageBreaches`                                                                                                                                                                                                                                                             |
| type-name parity                   | `repo-schema-artifact-type-name-parity-law`       | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🏷️type-name-parity/🟦️.ts`       | `policyArtifactSchemaTypeNameParityBreaches`                                                                                                                                                                                                                                                           |
| fixture/source ownership parity    | `repo-schema-artifact-ownership-field-parity-law` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts` | `policyArtifactOwnershipFieldParity`                                                                                                                                                                                                                                                                   |
| artifact law aggregate             | `repo-schema-artifact-law-aggregate`              | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/📋️aggregate/🟦️.ts`              | `policyArtifactSchemaBreaches`                                                                                                                                                                                                                                                                         |

The taxonomy adds the `repo-schema-artifact` and `repo-schema-artifact-laws` parents plus the eleven exact leaf contexts, for thirteen registered contexts overall. The owner graph has eleven nodes, twenty-three owner-to-owner imports, zero cycles, and zero imports back to root `📜️script.ts`.

## Behavior and evidence boundaries

Source admission uses `lstat` for the repository root and every admitted ancestor before inspecting the final source. It rejects a linked ancestor without reading its target. Typed reads retain `missing`, `unreadable`, `symlink`, and wrong-kind outcomes. The two existing safe wrappers return an empty value only for `ENOENT`; unreadable, linked, `ENOTDIR`, and wrong-kind sources throw. Artifact owner discovery returns `{ owners, issues }`: intentional missing discovery roots remain absent, while unreadable, linked, and wrong-kind roots or subtrees enter the aggregate as `artifact-schema/source-unreadable` evidence. Completeness applies the same separation to facet directories and leaves.

The schema laws reuse the accepted field-discovery owners. TypeScript facet loading still resolves imported and re-exported aliases from the source file's directory. JSON Schema remains normative. The established Protobuf map optionality and fixed-list representation exceptions remain unchanged. No parser or runtime dependency was added.

The current live diagnostic inventory remains an observed result, not a permanent assertion:

| Finding                           | Count |
| --------------------------------- | ----: |
| discovered standard/subset owners |   192 |
| discovery admission issues        |     0 |
| diff coverage                     |    74 |
| facet completeness                |   377 |
| field parity                      | 1,363 |
| normative leaf                    |     2 |
| state parity                      |    83 |
| type-name parity                  |   101 |
| all artifact-schema diagnostics   | 2,000 |

## Consumer closure

Root `📜️script.ts` directly imports shared source access, ownership-field parity, and the artifact aggregate. It owns none of the moved declarations and exposes no compatibility facade. Its lint and verify/report/enforce orchestration call the real owners.

`📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts` now imports owner discovery, ownership-field parity, and aggregation from their anonymous semantic owners. It consumes the typed discovery result and requires an empty issue set before comparing its 192 owners with independent fast-glob discovery.

The root-script compiler compiles the actual complete root through Bun and esbuild after extraction. The focused ownership contract parses every owner and the root as source data, proves exact declaration ownership and root removal, and follows the real consumer imports. The focused Nx target declares root, taxonomy, source access, test-output environment ownership, artifact laws, field discovery, portable inputs, and the field-parity consumer as inputs.

## Portable contract and registration

The language-neutral contract is:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-artifact-schema-law-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-artifact-schema-law-source/🔣️.json`

The anonymous test leaf is `🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts`. It validates the JSON contract with Ajv, independently parses it with TypeScript JSON syntax, checks all owner declarations and contexts, typechecks all eleven owners with installed TypeScript, proves the graph and consumer boundaries, compares discovery with fast-glob, and executes injected plus real-filesystem source admission cases.

The executable route is `@semio-tech/repo-lib:test-root-artifact-schema-law-source`, implemented by `bun ./📜️script.ts test root-artifact-schema-law-source`. It is registered in package scripts, the Nx project, launch seed, and derived launch catalog as `🧬schema🗿artifact🧪root-artifact-schema-law-source`. The package router automatically supplies a ticket-owned artifact directory for its native filesystem fixture while preserving explicit `SEMIO_TEST_ARTIFACT_DIR` overrides through the existing environment owner.

## Test-driven findings and repairs

The schema-first red phase passed one case and failed seven, with nine assertions. It established that the new contexts and owners were absent, the root retained the artifact declarations, the consumer graph still targeted root, and no Bun/Nx/launch route existed.

After the split, independent hostile testing found two real false-clean paths. Safe wrappers collapsed chmod-denied inputs to empty values, and final-leaf-only `lstat` followed a linked parent. Full ancestry admission and non-missing wrapper failures repaired both. The portable operation fakes were then updated to model `/repo` and each real ancestor as directories, rather than bypassing the new boundary.

Independent discovery testing then found that an `EACCES` plugin root returned zero owners and zero findings. Typed discovery issues and aggregate conversion repaired that result. The portable control now requires an `artifact-schema/source-unreadable` breach for the exact injected failure. Native ticket-private controls verify chmod-denied files and directories, linked-parent files and directories, missing-source empties, and wrapper exceptions.

One post-classification run reached 10/11 passing cases because semantic context resolution took 5.16 seconds under concurrent CPU load and exceeded Bun's five-second default. That case now has the same explicit 30-second budget as the other expensive installed-compiler/live-inventory cases. The corrected ordinary route passes within the package's 45-second budget.

## Verification

| Check                                                                        | Current result                                                                                                               |
| ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| taxonomy load and validation                                                 | green, zero problems                                                                                                         |
| direct ordinary package source route                                         | 11/11, 2,147 assertions, 22.24 seconds                                                                                       |
| isolated registered Nx source target, cache skipped                          | 11/11, 2,147 assertions, Bun 40.00 seconds, Nx 41.0 seconds                                                                  |
| direct artifact field-parity oracle                                          | green; 192 owners, 16 alias/module graphs, 3 GraphQL metadata cases, TypeScript AST and Ajv parity                           |
| isolated registered Nx workspace artifact-field-parity target, cache skipped | green with the full oracle; 25.2 seconds                                                                                     |
| root-script compiler                                                         | 6/6, 86 assertions; complete root accepted by Bun and esbuild                                                                |
| root runtime import                                                          | green                                                                                                                        |
| owner graph                                                                  | 11 nodes, 23 internal edges, zero cycles and root back imports                                                               |
| live aggregate                                                               | 192 owners, zero admission issues, 2,000 observed diagnostics with the unchanged distribution above                          |
| package/Nx/seed/derived launch registration                                  | green                                                                                                                        |
| Prettier for owners, controls, package route/manifests and report            | green                                                                                                                        |
| strict JSON and JSONC parsing                                                | green for taxonomy, portable inputs, package/project manifests and both launch catalogs                                      |
| scoped `git diff --check`                                                    | green                                                                                                                        |
| independent Terra audit                                                      | repaired boundary independently green at 11/11 and 2,146 assertions; injected EACCES and native linked-ancestor probes green |

No live policy enforce, cleanup, scaffold, taxonomy apply, publication, Git mutation, worktree operation, AGENTS edit, or ticket lifecycle action ran. The 192 owners and 2,000 diagnostics are current debt evidence rather than a claimed clean policy result. Native permission assertions run on POSIX; Windows executes the portable injected error controls and the native junction no-follow control without assuming POSIX chmod semantics.

Full-file Prettier checks still flag root `📜️script.ts` and both shared launch catalogs because of concurrent formatting outside this slice. The three owned root imports and the one launch entry in each catalog were inspected in place and match their surrounding generated format; this slice did not reformat unrelated shared content.

## Exact mutation attribution

This slice owns:

- root `📜️script.ts`: direct source-access, ownership-field-parity, and aggregate imports plus removal of the artifact-schema declarations only;
- the eleven anonymous owners in the inventory;
- library `🔣️taxonomy.json`: the thirteen artifact/source-access contexts only;
- the portable schema, fixture, and test above;
- `📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts`: its three artifact-owner imports and typed discovery assertion only;
- library TypeScript package `📜️script.ts`, `📋️project.json`, and `package.json`: the artifact source route, its exact inputs, and native-fixture environment call only;
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`: the single artifact source launch only;
- this report.

The package router, project manifest, taxonomy, root, launch catalogs, and field-parity test contain concurrent changes. The coordinator owns the kind-only route budget. Cleanup/scaffold owns the shared test-output environment owner. Print and app-verification lanes own their adjacent route, taxonomy, normalization, and launch changes. This report does not attribute those changes.

### Complete created or updated path list

1. `📜️script.ts`
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/🔍️owner-discovery/🟦️.ts`
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/🏷️export-identity/🟦️.ts`
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/📚️facet-leaves/🟦️.ts`
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🧩️facet-completeness/🟦️.ts`
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/📏️field-parity/🟦️.ts`
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/💾️state-parity/🟦️.ts`
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🔺️diff-coverage/🟦️.ts`
11. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🏷️type-name-parity/🟦️.ts`
12. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts`
13. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/⚖️laws/📋️aggregate/🟦️.ts`
14. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-artifact-schema-law-source/🔣️.json`
15. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-artifact-schema-law-source/🔣️.json`
16. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts`
17. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts`
18. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
19. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
20. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
21. `.vscode/🧩️launch.seed.jsonc`
22. `.vscode/launch.json`
23. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-root-artifact-schema-law-extraction-2026-09-12.md`
