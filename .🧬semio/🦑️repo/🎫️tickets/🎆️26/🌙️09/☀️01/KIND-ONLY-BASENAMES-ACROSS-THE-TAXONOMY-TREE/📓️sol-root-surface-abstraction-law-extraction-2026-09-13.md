# Root Surface and Abstraction Law Extraction

Date: 2026-09-13  
Status: complete and independently accepted.

## Result

The root surface-schema and abstraction-ownership implementation moved from root `📜️script.ts` into 17 anonymous TypeScript leaves under three distinct domains. Root keeps command orchestration and directly imports the actual owners. It declares none of the 32 moved top-level bindings and exposes no compatibility facade.

The final owner graph contains 17 nodes and 35 internal direct-import edges, with zero cycles and zero owner-to-root back imports. Twenty-one exact semantic directory contexts resolve through the library taxonomy. The extraction keeps plugin artifact roots distinct from standard/subset dialect rows: the abstraction law combines `policyListPluginArtifactDirs` with each dialect's `subsetRel` and does not substitute artifact-schema owner discovery.

## Exact owners

Discovery:

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗺️surface/🟦️.ts` — `policySurfaceRoots`.
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts` — `policyListPluginArtifactDirs`.
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts` — `POLICY_STANDARDS_DIR`, `POLICY_SUBSETS_DIR`, `PolicyArtifactDialect`, and `policyListArtifactDialectDirs`.

Surface schema:

4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/🧱️contract/🟦️.ts` — the five surface path constants, `PolicyAppSchemaOwner`, `policyAppPresenceTypeName`, and `policyAppSchemaFacetRole`.
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/🔍️owner-discovery/🟦️.ts` — `policyDiscoverAppSchemaOwners`.
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/📚️facet-leaves/🟦️.ts` — `PolicyAppSchemaFacetLeaf` and `policyLoadAppSchemaFacetLeaves`.
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🧩️facet-completeness/🟦️.ts` — `policyAppSchemaFacetCompletenessBreaches`.
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/📏️field-parity/🟦️.ts` — `policyAppSchemaFieldParityBreaches`.
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🪞️config-fidelity/🟦️.ts` — `policyAppSchemaConfigFidelityBreaches`.
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/💧️state-purity/🟦️.ts` — `policyAppSchemaStatePurityBreaches`.
11. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🏷️type-name-parity/🟦️.ts` — `policyAppSchemaTypeNameParityBreaches`.
12. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🚚️config-relocation/🟦️.ts` — `policyAppSchemaConfigRelocationBreaches`.
13. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/📋️aggregate/🟦️.ts` — `policyAppSchemaBreaches`.

Abstraction ownership:

14. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/🧱️contract/🟦️.ts` — `AbstractionOwnership`, `AbstractionOwnershipSchema`, `abstractionOwnershipSchema`, and `abstractionOwnershipViolations`.
15. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/🔍️source-projection/🟦️.ts` — `abstractionOwnershipSchemaFields` and `abstractionOwnershipRustCommands`.
16. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts` — `abstractionOwnershipChecks`.
17. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/⚖️law/🟦️.ts` — `policyAbstractionOwnershipBreaches`.

## Behavior and source boundaries

Surface discovery still derives `XConfig` from real surface `type Config = XConfig;` bindings and derives `XPresence`. Canonical config, legacy config as diagnostic evidence, plugin-level config, missing config, and missing presence retain their prior meanings. The facet loader delegates Rust, TypeScript with real module resolution, GraphQL, JSON Schema, and Protobuf fields to the accepted field-discovery owners. Configured formats without an implemented parser retain an empty projection and therefore remain visible as diagnostics; this extraction does not invent a new runtime parser.

Every new filesystem reader consumes the shared typed source-access owner. Missing inputs retain intentional absence where the law previously treated absence as meaningful. Unreadable, symbolic-link, and wrong-kind roots are thrown or converted to `app-schema/source-unreadable` evidence instead of producing a false clean result. Ancestor inspection remains no-follow. The portable source control injects `EACCES` at the plugin root through the real artifact-root and surface-owner APIs and requires a source-unreadable relocation result.

The normative abstraction schema and portable abstraction fixture remain source data at their existing ownership paths. The verifier continues to use installed Ajv as the independent schema disposition oracle, Bun's TypeScript transpiler, the accepted five field parsers, and real TypeScript module resolution. The field-parity oracle remains a separate consumer of the abstraction fixture.

Root directly imports the surface-root, plugin-artifact-root, dialect, surface aggregate, abstraction verifier, and abstraction law owners. Existing `verify abstraction-ownership` test/report/enforce and artifact-contract-ownership dispatch remain intact. The library package owns only focused test routing.

## Portable contract and registration

The language-neutral contract and vector are:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-surface-abstraction-law-source/🔣️.json`;
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-surface-abstraction-law-source/🔣️.json`.

The anonymous test leaf is `🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts`. It validates the JSON contract with Ajv and TypeScript JSON parsing, resolves all contexts, checks exact declarations, typechecks the 17 owners, audits the graph and root boundary, exercises source admission, executes the existing abstraction proof, observes live law behavior without freezing diagnostic totals, and checks all registrations.

The route is `@semio-tech/repo-lib:test-root-surface-abstraction-law-source`, implemented as `bun ./📜️script.ts test root-surface-abstraction-law-source`. Package, Nx project, launch seed, and derived launch register it once as `🧬schema🗺️surface🏛️abstraction🧪source-ownership`.

## Verification

| Check                                                      | Result                                                                                                                       |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| post-format direct source gate                             | 9/9 tests, 201 assertions, 27.28 s                                                                                           |
| final isolated registered source Nx target                 | 9/9 tests, 201 assertions; Bun 40.00 s, Nx 40.6 s, cache skipped, private workspace/cache roots                              |
| existing direct abstraction proof                          | 11 ownership, four nested-schema, and three Rust-command vectors agree with Ajv; 104 artifact contracts across five formats  |
| existing registered `workspace:test-abstraction-ownership` | green; Nx 1.8 s, 942 ms critical path, cache skipped, private workspace/cache roots                                          |
| root-script compiler                                       | 6/6 tests, 86 assertions; Bun and esbuild accept the complete actual root                                                    |
| root runtime import                                        | green                                                                                                                        |
| live app discovery                                         | 278 owners                                                                                                                   |
| live app aggregate                                         | 600 observed diagnostics: 544 facet-completeness, 24 field-parity, 14 config-fidelity, two state-purity, 16 type-name-parity |
| graph and contexts                                         | 17 owners, 35 internal edges, zero cycles/back imports; 21 contexts                                                          |
| formatting and JSON                                        | owned owner/control files green; package/project/portable JSON strict parsing green                                          |
| independent audit                                          | accepted in `📓️terra-root-surface-abstraction-audit-2026-09-13.md`                                                           |

One contention-tainted duplicate package run exceeded the 60-second child budget while two identical live scans ran concurrently; a clean direct run and two clean registered runs supersede it. One earlier private workspace Nx attempt failed before target startup when unrelated Nx plugin workers timed out; the later successful workspace target supersedes it. Neither is product evidence.

The existing Rust physical-reference native oracle does not parse command enums. Rust command ownership remains first-party `inspectRustStructure` projection compared with language-independent Ajv vectors, so no independent `syn` command-enum result is claimed. The 600 live findings are current diagnostic debt and not a clean-law claim or a frozen acceptance count.

No live policy enforce, taxonomy apply, cleanup, scaffold, publication, Git mutation, worktree operation, AGENTS edit, or ticket lifecycle action ran.

## Exact mutation attribution

This slice owns root `📜️script.ts` only for six direct imports, removal of the 31 root implementation statements carrying the 32 moved bindings, and removal of their empty editorial region. It owns the 17 anonymous owners above, the 21 taxonomy context entries, the portable contract/vector/test, one package route and target, one package script, one seed/derived launch entry, and this report. Shared files contain concurrent root, print, app-verification, cleanup, and other agent changes that this report does not attribute.

Complete created or updated path list:

1. `📜️script.ts`
2. `.vscode/🧩️launch.seed.jsonc`
3. `.vscode/launch.json`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗺️surface/🟦️.ts`
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts`
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts`
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/🧱️contract/🟦️.ts`
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/🔍️owner-discovery/🟦️.ts`
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/📚️facet-leaves/🟦️.ts`
11. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🧩️facet-completeness/🟦️.ts`
12. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/📏️field-parity/🟦️.ts`
13. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🪞️config-fidelity/🟦️.ts`
14. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/💧️state-purity/🟦️.ts`
15. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🏷️type-name-parity/🟦️.ts`
16. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/🚚️config-relocation/🟦️.ts`
17. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️surface/⚖️laws/📋️aggregate/🟦️.ts`
18. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/🧱️contract/🟦️.ts`
19. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/🔍️source-projection/🟦️.ts`
20. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts`
21. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/⚖️law/🟦️.ts`
22. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-surface-abstraction-law-source/🔣️.json`
23. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-surface-abstraction-law-source/🔣️.json`
24. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts`
25. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
26. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
27. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
28. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-root-surface-abstraction-law-extraction-2026-09-13.md`
