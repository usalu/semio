# Demonstrator Preparation

Read-only inspection on 2026-09-08 confirmed that Demonstrator build/dev call OS Dev plugin builds, registry generation and six pane engine preparations inside one target. Its build has no fixed output contract. Its E2E command starts the raw development script and permits reuse of the fixed port. Vite imports the task router for the runtime layout, and copies ambient global plugin/extension roots in addition to its computed union. These paths remain unqualified for task caching.

The six panes need Demonstrator and Procedural runtime plugins. The existing runtime layout recursively follows component dependencies and consumed extension contributions. Nx playground preparation currently expands contributions only for the root before traversing ordinary dependencies. A fixture and comparison against actual Cargo metadata are needed to determine and fix any omitted preparation edges.

Planned implementation: schema-owned runtime selection; pure component closure and consumer configuration without task-router imports; explicit preparation and output ownership; one Nx-owned server lifecycle; independent cache restoration and runtime consumers. No completed Demonstrator runtime validation is claimed by this inspection.

## Runtime Closure Comparison

Independent fixed-point traversal over the current generated registry found these root selections missing recursively consumed contributors under the previous root-only contribution scan:

- flow-extension-bim: flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive, flow-extension-text
- flow-extension-brep: flow-extension-bim, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive, flow-extension-text
- flow-extension-dictionary: flow-extension-bim, flow-extension-brep, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive, flow-extension-text
- flow-extension-draw: flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive, flow-extension-text
- flow-extension-list: flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-logic, flow-extension-math, flow-extension-primitive, flow-extension-text
- flow-extension-logic: flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-math, flow-extension-primitive, flow-extension-text
- flow-extension-math: flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-primitive, flow-extension-text
- flow-extension-primitive: flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-text
- flow-extension-text: flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive
- process-extension-concrete: process-extension-metal, process-extension-robotic, process-extension-wood
- process-extension-metal: process-extension-concrete, process-extension-robotic, process-extension-wood
- process-extension-robotic: process-extension-concrete, process-extension-metal, process-extension-wood
- process-extension-wood: process-extension-concrete, process-extension-metal, process-extension-robotic

Reference: [Nx language support](https://nx.dev/docs/kb/add-language-support) assigns target and dependency inference to project graph plugins; [project configuration](https://nx.dev/docs/reference/project-configuration) defines task prerequisites and continuous targets.

## Shared Closure and Configuration Boundary

A pure JavaScript component resolver now supplies Nx preparation and registry session filtering. It closes ordinary runtime dependencies and consumed contributions recursively, validates duplicate/unknown component identities, deduplicates union roots and supports host selection. Five language-neutral positive cases and three invalid cases pass against graphlib traversal. A separate comparison of actual Cargo metadata and graphlib agrees with all 120 inferred playground preparation targets (60 variants, two profiles). The initial red test failed on the absent resolver.

Demonstrator now has an authored runtime catalog and schema for its six pane variants, host and assets. Vite imports the pure runtime description instead of its task router or brand implementation. The independent esbuild probe reports 11 files and no task or brand imports; catalog, pane remapping and module directory assertions pass. The old in-source layout expectations were stale: the current authored deployment directories use emoji vendor/shard names, and the actual metadata closure does not declare Stdio. The new expectations follow those verified source contracts rather than the stale assertions. This configuration extraction does not complete Demonstrator preparation/build/E2E refactoring.

Repository and product Nx tests are pending; their clients currently wait for shared graph construction while other source changes continue. The shared daemon has not been reset.

The isolated daemon-backed Nx dependency fixture passed all eleven actual task/cache scenarios with the extracted helper present. Demonstrator quick tests passed all four tests in two files. Registry quick tests initially failed because its script forwarded the level word `quick` as a Vitest filename filter; the router now resolves the test level before passing remaining arguments and a retry is running. Registry projection tests now also consume the transitive runtime fixture.

The full repository suite passed its command/input/component/activation checks, then failed the editor coverage gate on newly discovered `semio-s-artifact-stdio-contract:build`. The crate exists in the current workspace but had no build/check/test launch entries. Those three canonical Nx entries have been added to the existing seed order; registry regeneration and a final suite retry remain pending. No unrelated crate changes were reverted.

Registry generation passed in 52.1s and emitted the missing native editor entries. The next registry run passed 24 of 25 tests, including the new session closure test; the remaining preview-order assertion lacked the already implemented Report actor-network contract. Its expected order and the seed are now aligned in the existing preview group. Registry tests explicitly depend on their catalog generator and remain uncached while their dynamic whole-catalog/source reads await a complete input audit. A final test run is pending.

## Final Integration Checks

The final `repo:test` passed in 35.9s, including the runtime component fixtures, 120 real preparation targets, 11-file Demonstrator import boundary, source/input contracts, editor coverage and cancellation checks. Registry `test-quick` passed all 25 tests across four files in 23.01s (52.5s Nx run including its two generation prerequisites). These results qualify the pure runtime extraction and registry fixes; the Demonstrator build/dev/E2E pipeline remains to be refactored.

## Next Pipeline Boundary

The planned Demonstrator graph exposes profile preparation as an outer dependency union over the seven authored pane/runtime variants, allowing Nx to deduplicate shared component, engine, support and session producers. Development consumes a completed generator activation receipt only after that union has finished. Production should copy only the declared component/extension/support directories from profile-specific materialized outputs; it must not copy the global legacy module tree or installation timestamps. The application build remains unqualified until its complete Vite source, assets, runtime-output and environment inputs are proven. The E2E runner must consume an Nx-owned continuous server with explicit readiness and lifetime ownership rather than spawn its own dev pipeline or reuse an ambient port.

The installed Nx task environment supplies NX_INVOCATION_ROOT_PID to related tasks. This is useful for tracing ownership but does not alone prove a process lifetime when a PID is reused. Readiness and cleanup need a concrete runtime proof before adopting it as a resource identity. Nx’s [run-commands executor reference](https://nx.dev/docs/reference/nx/executors) and installed 23.2.0 executor/task-orchestrator source are being used to resolve continuous task behavior. No new Demonstrator target graph or server implementation has been applied yet.

## Authored Preparation Targets

`prepare-dev` and `prepare-release` now declare seven OS development preparation prerequisites in the outer Nx graph: aggregator, aussuchen, bearbeiten, generation3d, generator, koordinator and verfolgen. Their thin command reads completed component ownership markers and generated sessions; it never invokes compilers or another repository task. Both aggregate validations are uncached and own no outputs. Their public launch selection sets the selected native build mode before Nx schedules prerequisite engines, and rejects extra compiler arguments. Two editor launch-seed entries expose these preparations. Existing dev/build/E2E consumers have not yet been connected and still contain their old hidden pipeline.

The language-neutral pipeline fixture initially failed with missing dev preparation and now passes. Profile invocation vectors also pass; the attempted red invocation process was still running when the implementation was added, so it observed the updated code and cannot be reported as a reproduced red result. The final root retry passed all ten graph coalescing scenarios, then failed on the intentionally not-yet-authored Demonstrator preparation target; it must be rerun after this implementation. Actual `prepare-dev` runtime validation and launch regeneration are running.
