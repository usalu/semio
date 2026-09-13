# Repo-Test Command Composition Acceptance Audit — 2026-09-13

## Status

**Accepted.** The current ordinary package route and isolated registered Nx target are both green at 10 passing cases and 219 assertions. The 16-owner semantic split, router boundary, ownership graph, host/scenario separation, source-data closure, and ordinary route registration are accepted. The earlier 8/173 direct result and 7/8 pre-assertion Nx interruption are retained below as history.

## Ownership and router boundary

The exact 16 anonymous owners are declared by `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json`, with 29 context records in its paired schema. They separate command contracts, selection, host materialization, scenario execution, parity/contracts/reporting/dependency/inventory/provenance/coverage/retention/schema/environment orchestration, and domain policy.

I independently parsed the current owners: all 16 files exist, with 17 owner-to-owner direct edges, no owner import to `📜️script.ts`, and no cycle. The command module is 75 lines and contains routing plus two small command selectors. It directly imports the orchestration owners, while the lower-level contract, selection, host-materialization, and scenario owners are consumed downstream. It does not re-export command classes or recreate a command API facade.

The `policy` re-export is required script metadata rather than a compatibility facade. The library and Nx plugin parse `policyFile` from the mandatory script to make the policy leaf an attributed lint and cache input; the router points it directly to `⚖️policy/🧹️domain/🟦️.ts`.

## Host and scenario responsibilities

`🖥️host/🏗️materialization/🟦️.ts` reads each owner's declared oracle packages through the registry. For every local Python contribution it projects an explicit, ordered `--local-source <directory>` argument to the committed host; the host prioritizes those declared roots before its own script directory before it loads an adapter. `PYTHONPATH` remains a descendant import-path behavior, but it is not the selecting boundary. The materializer creates cache-owned Rust, Go, Python, TypeScript, and .NET launch recipes and marks generated host roots.

`🏃️execution/🎬️scenario/🟦️.ts` first removes the prior result stream, requests a host recipe, runs native host preparation under `buildBudgetMs()`, then executes the host under `testLevelBudgetMs(level)`. It reports an absent compiled executable rather than silently proceeding and marks work/output roots only after the run.

The source-only command-composition test checks the Cargo machine-output parser and the build-versus-scenario budget/order. It imports host/scenario source only; it does not materialize a host, invoke Cargo, provision Python, install a package, or execute an oracle. That is the appropriate safe audit boundary. General host execution remains capable of creating an isolated Python environment and attempting `pip install` for a pathless declared external package, so no runtime-host result is claimed here.

## Source-data and route closure

The package router exposes `bun ./📜️script.ts test command-composition-source`. The project registers cached target `test-command-composition-source` with that exact command, and both seed and derived launch files register `🧪️test🧭️command-composition-source` for the exact Nx target.

The target's `commandCompositionSources` correctly lists the router, 16 owners, portable schema/fixture/test, test-platform consumer, library entry, Nx plugin, and taxonomy data. It also preserves the unrelated `htmlSourcePairs` list for `test-fixture-verify`; the source test explicitly checks its 17 inputs.

However, the cached test reads all of the following as source data:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` for the target command and HTML-input assertions;
- `.vscode/🧩️launch.seed.jsonc` for the launch seed;
- `.vscode/launch.json` for the generated launch entry.

Historical finding: the initial `commandCompositionSources` omitted all three. The executor added them with the two directly consumed TypeScript library owners. A first isolated Nx rerun reached 7/8 only because concurrent Norm taxonomy work left the unrelated `oracle-source-ownership` contract pointing at missing `testdata` before composition assertions. This is non-acceptance history. The later isolated target passed 10/219 and closes the repair. The repair does not change source ownership or justify a router facade.

## Current evidence and limits

Historical baseline: the executor's earlier ordinary package route was 8/8 with 173 assertions in 21.48 seconds. The current green evidence is 10/219 direct and isolated Nx, recorded below. I did not duplicate the route. No production test, host provision, Python install, Cargo build, or destructive cleanup was run by this audit.

## Generic Python local-source repair delta

The repaired materialization owner has an explicit `pythonHostArguments` projection. It emits each manifest-declared local source as a repeated `--local-source` argument before plan/output/adapter arguments. The Python host validates that each source is a directory, preserves the declared order while removing duplicates, and replaces `sys.path` so those declared roots precede the host script directory before the adapter is loaded.

The command-composition target now includes the host `🐍️.py` and its same-basename fixture `🐍️.py` as cache inputs. The focused native control proves resolution of the declared fixture marker after manually introducing the host directory, but calls the host prioritization helper through a spec-loaded module; it is not an end-to-end host CLI run. The selected Norm CLI control is required for that boundary.

## Final registered evidence

The executor’s current ordinary package route is green: 10 passing cases and 219 assertions in 4.07 seconds. The actual isolated registered Nx target `@semio-tech/repo-test-domain:test-command-composition-source` is also green with 10/219, target 5.6 seconds, cache skipped. This confirms the expanded `commandCompositionSources` cache closure, including the project and launch source-data readers, the host source, and the same-basename hostile fixture.

The safe existing provenance command `fixture verify --artifact s.stdio.html --standard 5 --subset any` is green with eight fixtures and zero file problems in 15.96 seconds. The existing source-as-data compiler is green 1/6. Neither result claims a browser or native Rust execution.

**Acceptance:** accepted for the bounded 16-owner command-composition extraction. The native same-basename helper control remains a component proof; the separate selected Norm CLI establishes the full Python host command boundary.
