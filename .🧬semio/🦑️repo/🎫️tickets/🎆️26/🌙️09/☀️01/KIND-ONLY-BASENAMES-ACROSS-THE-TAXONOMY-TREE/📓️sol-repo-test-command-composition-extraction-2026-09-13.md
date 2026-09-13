# Repository Test Command Composition Extraction

## Scope and result

This slice extracts the substantive repository-test command implementation from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` into sixteen anonymous TypeScript leaves under their semantic concerns. The mandatory command file is now a 75-line router. It retains the explicit DSL forwarding route and test subcommand selection, imports the real orchestration owners, and exposes the domain policy through its required `policyFile` metadata and a direct re-export. It does not retain an extracted semantic declaration or command-class facade.

The extraction registers 29 semantic contexts, a schema-first portable fixture and test, a cached Bun/Nx source target, and one seed/derived launch identity. The owner graph has 17 internal edges, is acyclic, and has no owner-to-command-module back edge.

## Anonymous owners

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧱️contract/🟦️.ts`
   - repository-test location constants and the neutral `HostPreparation` and `MaterializedHost` contracts
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🔍️discovery/🎛️selection/🟦️.ts`
   - selectors, case/implementation selection, and target/fixture/result matching
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts`
   - five-language host materialization, declared oracle packages, interpreter provisioning, Cargo executable parsing, and the pure Python host-argument projection
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts`
   - one planned scenario execution, result cleanup/readback, preparation and scenario budgets, and completion marking
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️parity/📋️orchestration/🟦️.ts`
   - oracle decisions, plan-mode selection, phase execution, and oracle/subject/parity commands
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts`
   - discovery, contract, and default run commands
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️reporting/📋️orchestration/🟦️.ts`
   - report locations, implementation coverage, JUnit reporting, and metrics
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts`
   - classified-baseline loading and dependency command orchestration
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts`
   - mutation bridge resolution and source inventory orchestration
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts`
    - fixture verification, audit, private reproduction, and explicit generation/publication routing
11. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts`
    - probe and matrix orchestration
12. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts`
    - payload-schema and scaffold command projection plus gap/manifest authoring commands
13. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧹️retention/📋️orchestration/🟦️.ts`
    - clean-plan and cache-retention command orchestration
14. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
    - schema validation command orchestration
15. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🩺️environment/📋️inspection/🟦️.ts`
    - Nx projection and environment/toolchain inspection commands
16. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️policy/🧹️domain/🟦️.ts`
    - the domain-root child and nested-cache policy

## Semantic contexts

The taxonomy registers the following exact contexts:

- `repo-test-command-contract`
- `repo-test-discovery`
- `repo-test-selection`
- `repo-test-host`
- `repo-test-host-materialization`
- `repo-test-execution`
- `repo-test-scenario-execution`
- `repo-test-parity`
- `repo-test-parity-orchestration`
- `repo-test-contracts`
- `repo-test-contract-orchestration`
- `repo-test-reporting`
- `repo-test-reporting-orchestration`
- `repo-test-dependencies`
- `repo-test-dependency-orchestration`
- `repo-test-inventory`
- `repo-test-inventory-orchestration`
- `repo-test-provenance`
- `repo-test-provenance-orchestration`
- `repo-test-provenance-authoring`
- `repo-test-coverage`
- `repo-test-coverage-orchestration`
- `repo-test-retention`
- `repo-test-retention-orchestration`
- `repo-test-schema-orchestration`
- `repo-test-environment`
- `repo-test-environment-inspection`
- `repo-test-policy`
- `repo-test-domain-policy`

Every implementation owner ends in anonymous `🟦️.ts`. The native host and hostile fixture remain anonymous `🐍️.py` leaves.

## Consumer, routing, and cache closure

The command router directly imports only the orchestration owners it registers. Lower owners are reached through those direct semantic dependencies. The required policy metadata names `⚖️policy/🧹️domain/🟦️.ts`; the generic library and Nx policy discovery accept that direct re-export and resolve the real policy owner.

The existing source-as-data compiler in `🧪️tests/🧪️test-platform/🟦️.ts` now reads `rustHostExecutableFromCargo` from the host-materialization owner and `executeOne` from the scenario-execution owner. It retains the installed TypeScript compiler oracle and proves that Cargo's machine-output executable is resolved under the build budget before the finite scenario budget begins.

The ordinary route is:

- Bun: `bun ./📜️script.ts test command-composition-source`
- Nx: `@semio-tech/repo-test-domain:test-command-composition-source`
- launch identity: `🧪️test🧭️command-composition-source`

The target uses the exact `commandCompositionSources` named input. It includes the project manifest, router, all sixteen owners, portable schema/fixture/test, the existing source-data consumer, both direct TypeScript library owners, generic policy discovery sources, taxonomy, Python host, hostile Python fixture, and both launch catalogs. Root's concurrent `normOracleSources` and `htmlSourcePairs` inputs remain intact; the source test confirms the 17 HTML inputs. Root subsequently ran the actual launch generator, which rebuilt the derived catalog with the composition identity still a singleton.

## Python local-source precedence

The selected Norm native run exposed an actual anonymous-leaf collision: CPython placed the directory containing the host `🐍️.py` at `sys.path[0]`, ahead of a manifest-declared local oracle also named `🐍️.py`, despite `PYTHONPATH`.

The repair is a generic, explicit host protocol:

- `pythonHostArguments` emits every manifest-declared local root as an ordered repeated `--local-source <directory>` argument.
- The Python host validates each directory, removes duplicate path identities with Windows-aware normalization, and places the declared roots in manifest order ahead of the script directory before loading the adapter.
- The existing `PYTHONPATH` projection remains for descendant processes and established behavior.
- A schema-declared anonymous `🐍️.py` hostile fixture proves that the declared module wins when the host directory contains the same basename.

Root's separate ticket-local selected Norm control invoked the real host CLI through `argparse` and `run_main` using the production `pythonHostArguments` output. After a separate Norm fixture-coordinate correction, all 65 EN 1991 scenarios passed. This closes the actual CLI import and dispatch boundary rather than relying only on the component fixture.

## Test-driven evidence

The schema-first portable red phase produced one pass and seven failures because no owner/context/consumer/route implementation existed. The first owner phase reached two passes and six failures, exposing incorrect import depth, compiler-export handling, protected root access, and stale source-data consumers. Those failures were corrected at their actual owners without restoring a command-module facade.

The Python hostile control was added before its implementation. Its red run observed the missing `pythonHostArguments`; an unrelated concurrent taxonomy edit caused the other failure in that run. Root's pre-repair selected native invocation independently observed the production collision as an adapter `AttributeError`.

Final current evidence:

- Direct ordinary router: 10 tests, 219 assertions, zero failures in 4.07 seconds.
- Actual isolated Nx target: 10 tests, 219 assertions, zero failures; selected target 5.6 seconds, cache explicitly skipped. Nx workspace data and cache were isolated under this lane's ticket scratch.
- Existing TypeScript source-data compiler: one selected test, six assertions, zero failures. Whole-file top-level registry loading made that process take 64.77 seconds; the selected assertion itself took 0.58 seconds.
- Safe extracted provenance command: `fixture verify --artifact s.stdio.html --standard 5 --subset any` verified eight fixtures with zero file problems in 15.96 seconds.
- Safe extracted discovery command: `discover --json` returned 251 rows, 251 unique project identities, and 250 rows with at least one adapter in 9.10 seconds.
- Root's actual selected Norm host CLI: one test and nine assertions passed in 9.86 seconds, with all 65 planned EN 1991 scenarios passing through the production repeated-source argument and real host `argparse`/`run_main`. Evidence is retained in `🗑️generated/coordinator/norm-oracle-ownership/native-selected.log`.
- Independent Terra audit accepted the sixteen owners, 29 contexts, 17-edge acyclic graph, exact cache closure, direct and Nx 10/219 results, safe provenance result, and the separate actual CLI confirmation. Its retained report is `📓️terra-repo-test-command-composition-audit-2026-09-13.md`.

## Limits

The ordinary `test-oracle`, `test-parity`, and `test` commands still place scenario work and results under the repository test cache; `SEMIO_TEST_OUTPUT_SCOPE` selects a task subdirectory but is not a ticket-generated-root override. This lane therefore did not invoke those broad output-producing commands. The dedicated Norm control built the real plan and kept its native plan/results under ticket-local temporary storage.

No Cargo build, package installation, live cleanup, fixture generation/publication, mutation operation, service operation, shared Nx reset, or shared-cache deletion belongs to this slice. The separate Norm DIN 16798 duplicate expanded feature identity remains a Norm source-contract concern and is not treated as a host or composition failure.

## Exact attribution

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧱️contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🔍️discovery/🎛️selection/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🏗️materialization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️parity/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️reporting/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧹️retention/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🩺️environment/📋️inspection/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️policy/🧹️domain/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🧱️command-composition-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧱️command-composition-source/🐍️.py`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-repo-test-command-composition-extraction-2026-09-13.md`

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The project-manifest attribution is limited to `test-command-composition-source` and `commandCompositionSources`; the concurrent Norm and HTML named inputs belong to their respective lanes. The taxonomy attribution is limited to the 29 contexts listed above, the launch attribution to the singleton composition entry, and the generic policy-discovery attribution to direct policy re-export recognition.

No Git lifecycle, AGENTS, runtime dependency, live cleanup, fixture publication, shared cache, or service mutation belongs to this slice.
