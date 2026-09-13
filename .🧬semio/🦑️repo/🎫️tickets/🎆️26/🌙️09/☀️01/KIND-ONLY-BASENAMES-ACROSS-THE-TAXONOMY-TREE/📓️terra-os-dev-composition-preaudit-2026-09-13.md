# OS Dev Composition Script Extraction — Independent Pre-Audit

> **Current status — accepted for the bounded composition extraction.** The final source map has 49 anonymous owners, path-bound contexts, direct/source-data/detached consumer closure, exact 79 target inputs, registration, public-export checks, an isolated registered Nx pass, and focused source-import/source-data evidence. Historical red findings below are retained as repair provenance. Dev/build/service/native behavior remains outside this acceptance.

## Scope

This is a read-only pre-audit of the current 5,585-line executable router at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`. I ran no dev server, build, provisioning, service, or native command. The detailed current source and consumer inventory is retained in [inventory.json](🗑️generated/terra-os-dev-composition-audit/inventory.json).

The router currently contains implemented behavior in `BundleScript` class methods as well as local functions. Those bodies must leave the mandatory package command; keeping a class under `📜️script.ts` is not an extraction. The final router should perform argument validation, select a semantic owner, and delegate through the existing `ScriptRouter`/`runBundleScriptMain` primitives.

## Existing boundaries to preserve

The extraction must extend, rather than duplicate, the present lower owners:

- `♻️activation/🟦️.ts` owns activation receipt and staged-module vocabulary. `♻️activation/🌐️browser-host/🟦️.ts` owns browser-host roots and receipts. `stageTestBrowserHostV1` and its local materializer belong in a staging child of that existing browser-host concern.
- Registry discovery, catalog projection, generated session rendering, browser component materialization, descriptors, deployment paths, extension installation, and BLAKE3 already have distinct owners. The command must import them directly; it must not become a facade for them.
- `🚚️distribution/🟦️.ts` owns the distribution contract. Compiler input discovery, plan construction, witness collection, preview, publication/recovery, and stale comparison should be separate children under distribution. The local `DistributionBundlePlan` region is a composition layer, not a reason to move every distribution concern into one replacement file.
- `🧬schema/🔣️.json` is the contract data. `DEV_SCHEMA_URL` and `devContract` are validation behavior and should move to a contract-validation owner, with tests importing that owner directly.

The next incremental cuts should therefore separate: plugin catalog build scheduling; descriptor/extension publication; browser-host staging; engine selection and publishing; activation/lease/readiness; each lint or scenario verifier; parity comparison; distribution compiler and publication; scale-fixture generation; contract validation; and benchmark orchestration. Benchmarks have their own native, Chromium, and explicitly stubbed rows and must not be merged with ordinary verification.

## Concrete consumer closure

The highest-risk live consumers are source-as-data or dynamically loaded:

- Hub dynamically imports the package command and calls `stageTestBrowserHostV1` around lines 12063–12066. Its composition law also requires that call and its receipt relationship near line 12915. It must directly import the browser-host staging owner after the move.
- The plugin registry playground-session test reads `producerScript`, finds `ensurePluginRegistry`, and slices until `resolvePluginBuildTargets`. That assertion constrains behavior, so it must read the catalog-refresh owner instead of a command body.
- The browser-host test owner receives a large dependency object from `import.meta.vitest` and currently slices the local stage materializer between textual anchors. It also owns the linked-session-engine, engine-publication, distribution, parity, and contract controls. Split its injection by its actual verification concern and update every source-body check to the moved owner; do not retain a command-module test facade.
- Distribution currently uses a detached Bun evaluation with `import.meta.url` to break a Vite import cycle. On extraction this must import the actual neutral compiler owner URL, preserving the detached import-cycle control and the exact preview/check/generate partitions.
- The script reexports `blake3Hex` and `Blake3Hasher` from the framework hash owner. Consumers outside this command must import `🧰️framework/🔨️modules/🔏️hash/🟦️.ts` directly. Existing Hub imports are already direct, while retained OS host, OS Rust command, store test, descriptor, and replication command paths require a focused closure scan before removal.

The project manifest, package scripts, launch seed, and generated launch route all remain command registrations. Their commands can remain stable only if their targets declare every newly imported semantic owner as an input. Generated session and distribution routes have explicit generation/check/preview semantics, so their output paths and freshness consumers must move as source data rather than be inferred from the old command source.

## Critical acceptance controls

A later execution lane needs the following bounded evidence:

1. A portable owner/context/consumer map that proves behavior is absent from the router, detects root facades, and enumerates direct imports, dynamic imports, source-body readers, targets, inputs, and launch routes.
2. The browser-host staging control with private artifact roots: selected GIS byte identity, regular-file and ancestor/symlink admission, exact Space descriptor arguments, receipt closure, and the Hub direct owner import. It must not claim a dev-server journey.
3. Engine selection vectors through `linkedSessionEngines` and injected `buildEngineWasm` success/failure controls, plus a separately scoped selected Rust/Wasm command proof if a native invocation path changes. The injected build function is not native build evidence.
4. Distribution compiler source-graph and detached import-cycle controls; private hostile publication/recovery fixture; preview/check output proof. Never use a live-output `generate` route as the sole acceptance test.
5. Activation lease and cancellation, source digest, stale-reporting, and extension publication controls under isolated roots.
6. Schema/fixture/Ajv controls for contract predicates, and separately labelled native or third-party proof where a moved owner actually invokes one.

Existing routes such as `browser-host-staging-check`, `canonical-bootstrap-folder-mirror-check`, `preview-distribution`, `check-distribution`, generated-session preview/check, and scale-fixture preview/check are potential focused routes. `dev`, `build`, `plugin`, the activation/serve matrix, collaboration E2E, catalog smoke, parity sweeps, and benchmarks are not safe generic acceptance routes for this extraction.

## Limits

This report establishes live source topology and required closure only. It does not claim current build, Cargo, Vite, Chromium, Playwright, dev-server, collaboration, parity, distribution publication, or benchmark success. It also does not assign the unresolved browser-host staging test as a new owner: its existing activation/browser-host contract remains authoritative.

## Exact command and hash residues

The generated inventory retains 25 current text-level command-source candidates and 25 BLAKE3 name candidates for the executor to classify, instead of treating a broad text search as an import graph. The verified command-module BLAKE3 facades that must rebind are Hub at lines 7054 and 9197; the OS Rust package command at 1495; OS host Rust command at 340 and 629; the store initial-child-identity test at 7; fresh-component descriptor verification at 131; and replication Rust command at 96 and 205. Hub already has a direct top-level hash import for its other uses, so the two retained dynamic imports should use that owner too.

The command-source candidate set also includes source-data fixtures and topology controls: the Demonstrator runtime pipeline, plugin-registry playground-session fixture, OS builder staging/config fixtures, library package/project and ownership fixture data, taxonomy controls, and the launch seed/derived launch pair. Each must be classified by its actual relationship during extraction; only executable or behavior-bearing command-module references need rebind, while a stable command registration may remain a router route after its owner inputs are declared.

## Owner-plan review

I reviewed `sol-os-dev-composition-owner-plan-2026-09-13.md` against the current command and retained preaudit. Its semantic boundaries are appropriate: browser-host staging remains beneath the existing activation/browser-host concern; registry refresh is separated from plugin build planning/materialization; the detached distribution compiler is distinct from publication and freshness; and native/browser/stub benchmark modes remain distinct.

The plan preserves every high-risk closure identified in the preaudit: Hub must replace its dynamic package-command import with browser-host staging; the playground-session test must read catalog refresh rather than the router; the browser-host source-body test must split by real owner; distribution must retain detached semantic-owner URL loading; hash users must import the framework hash owner; and project/launch inputs must follow moved owners. Its proposed focused verification routes and explicit exclusion of live dev/build/server/collaboration/parity-sweep/benchmark work are correct.

One accounting defect requires correction before the schema-first fixture is authored. The plan states 48 anonymous leaves, but its semantic-owner table contains 50 rows: rows 1-49 are anonymous implementation leaves and row 50 is the package router. The fixture must declare 49 owners, or the plan must explicitly remove/consolidate one row with a demonstrated semantic reason. The command router must remain outside that anonymous-owner cardinality.

Status: preparation accepted. The executor corrected the plan to 49 anonymous semantic implementation owners plus the existing package router, for 50 table rows. The current plan digest is `6d187890…`; it also correctly locates the native benchmark under the neutral host concern `📊️benchmarks/🔌️plugins/🖥️host/🟦️.ts`, rather than a language-named native directory. The schema-first fixture enumerates all 49 owners.

The first-red route is behaving as intended: its portable schema/fixture test accepts the map while the materialization check fails for the absent `dev/🎮️playground-session/🏃️execution/🟦️.ts` and the routing check finds `PlaygroundSessionGenerateScript` still defined in the package command. This is a current extraction precondition, not acceptance evidence. The plan still requires the Hub direct import, registry source-as-data rebinding, compiler-owner detached import, and complete project/launch inputs before a current route can be accepted. No OS-dev product behavior or route was executed by this audit.

## Current first-red closure finding

The present `os-dev-composition-ownership` schema, fixture, and test prove only the 49 owner rows, forbidden router declarations, and one target command. They contain no fields or assertions for taxonomy contexts, direct/dynamic/source-as-data consumer edges, project named inputs, or seed/derived launch registration. Consequently the current first-red cannot detect a missing Hub browser-host staging import, registry source-body rebind, detached distribution compiler URL, direct hash consumer, or cached-route input.

This is a schema-first closure defect, separate from the expected absent-owner red. Add those fields and exact assertions before implementation movement turns the portable route green; then preserve the planned fixture distinction between executable router links and data/source readers.

## Expanded closure checkpoint — pending repair

The expanded portable contract now has 49 owners, 49 declared context chains, 12 consumer/source-data rows, eight project-input records, an owner-importer test, package/project registration, and seed/derived launch records. This corrects the prior schema-first omission of these categories, but the executor’s second-red result is still current and is not acceptance evidence.

Two precise controls need to close before the portable map can demonstrate the intended topology:

- The context test validates each declared `members` sequence with `semanticDirectoryKindId`, but does not connect that sequence to the `owners` entry bearing the same id. It must derive and compare the exact directory ancestry of each owner path. Otherwise an unrelated, taxonomy-valid chain can satisfy the full 49-row map.
- Registry playground-session source data now identifies `🔌️plugin/📇️registry/🔄️refresh/🟦️.ts` as the producer, while the registry project has no matching external cache input. The refreshed producer must become an input. The package command input should remain only when a target actually executes the command.

The Hub package command still launches the OS dev router as the UI subprocess for its secure-suite route at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:12720-12726`. That executable command edge is legitimate. The consumer controls for moved browser-host staging and hash behavior must test the new direct bindings and reject only the replaced API bindings, rather than forbid the old router pathname across the entire Hub source file.

I delivered all three observations to the executor and coordinator. No OS dev server, build, provisioning, service, or native route ran in this audit.

## Expanded closure re-audit — source gate pending public-symbol repair

I independently inspected the repaired schema, fixture, source test, consumer files, project inputs, and generator contracts. The former closure findings are repaired:

- Each of the 49 context chains is now required to be the actual owner path’s directory suffix before its members are resolved through the taxonomy.
- Hub’s moved browser-host and hash APIs are direct owner imports. The controls reject only the old dynamic bindings, preserving its separate secure-suite subprocess invocation of the OS dev command.
- Registry keeps `producerScript` as the executable command route and adds `producerSource` for catalog-refresh source inspection. Its project cache inputs now include the refresh owner as well as the still-executed session/router paths.
- The target enumerates schema, fixture, test, taxonomy, router, owner-importer, all owner/consumer paths, every external project manifest, and both launch authorities. It also checks the three generator contracts and the staging producer’s exact generated boundary names.

The executor’s current portable result is 10 passing tests, 535 assertions, and no failures in 1.69 seconds. It is a strong structural checkpoint, but not final runtime acceptance: the registered route and focused native/private seams have not yet been reported.

One public-contract control remains before final acceptance. The fixture lists expected `exports` for all 49 owners, but the source test never checks them. Its all-owner verification imports each module as a namespace, which establishes module resolution but allows an empty or wrong public namespace to pass. The test should verify every declared export against the actual owner’s exported declarations and, for command-facing symbols, their direct router binding. I sent this bounded repair to the executor; acceptance remains pending it and the registered/focused evidence.

## Public-owner role finding — blocks acceptance

An independent AST export inventory exposed a material gap in the otherwise green 10/535 portable control: the fixture’s `exports` field is unused by the test, and five current declarations do not match their files.

- `plugin-build-plan` claims `resolvePluginBuildTargets`, but that function is defined and exported by plugin build execution. Because it resolves selection, it should either move to the plan owner with explicit consumers rebased, or the owner table must deliberately assign the execution role.
- `parity-server-pool` claims nonexistent `ParityServerPool`; its actual public API is the port/pair/start/stop helper set.
- `distribution-plan` claims `distributionPathOrder`, which belongs to and is exported by distribution source.
- `benchmark-host` claims nonexistent `benchNativeRows`; the present host owner exports `SCALE_COMPONENT_ARTIFACT`, while native command execution is in the execution owner.
- `benchmark-stub` claims nonexistent `benchStubRows`; it exports `BENCH_WEB_STUB_STATUS` and `benchWebMeasuredRow`.

Namespace imports in the verification concern resolve the five modules but do not validate any named public binding, so the current test cannot expose these mismatches. The fixture must be corrected according to the intended semantic boundaries and the control must assert every declared export from its owner (with package-router bindings where applicable). This is a taxonomy/role-contract defect, not a runtime claim. It blocks acceptance pending repair and refreshed evidence.

The registered ownership target currently has the intended exact cache union: its 77 inputs equal `sharedGlobals` plus the schema, fixture, test, taxonomy, router, importer, 49 owners, 12 consumers, eight external project manifests, and both launch authorities. This was independently recomputed from the fixture with zero missing and zero extra paths. The in-repository test presently checks inclusion only, so it should compare the full union to retain that exactness; this is a control hardening request, not a current input-data defect.

## Current structural re-audit — export and input repairs closed

The current source test now verifies each fixture export with the TypeScript AST and checks every named router import against its declared owner API. My independent AST scan finds all 49 owner export sets present. The five prior stale rows are repaired: selection now belongs to plugin-build plan; parity-server-pool and benchmark-stub list their actual helper APIs; distribution ordering remains with source; and benchmark-host owns the native benchmark row API.

The registered target now tests exact input-set equality. Its current 79 inputs equal `sharedGlobals` plus the full contract union, including the two generated-boundary authority sources (`browser-bundle` materialization and registry deployment). My independent recomputation finds zero missing and zero extra inputs. The previous 77-input observation predates those two necessary authorities and is historical only.

Current structural evidence is 11/565 in the executor’s ordinary portable route. I did not repeat that route or any native/build/dev/server command. The remaining acceptance evidence is executor-owned: the registered isolated route and the already scoped browser-host/distribution/native controls must be reported with their actual provenance and limits.

## Final bounded acceptance

I re-audited the current 49-owner source, schema, fixture, ownership test, package router, external consumers, project manifests, generator contracts, and both launch authorities after the export and exact-input repairs. The owner map is semantically partitioned: plugin selection is in the plan owner; browser-host staging remains under activation/browser-host; distribution ordering is a source concern; benchmark host/browser/stub/execution are distinct; and the package command routes to owners without restoring a domain facade.

My independent TypeScript AST scan confirms every declared owner export exists in the declared anonymous file. The ownership target’s 79 inputs exactly equal its complete contract union, including both generated-boundary authority files; it has zero missing and zero extra inputs. All 49 recorded ancestry chains match their owner-path suffixes and resolve through the current taxonomy. The Hub controls retain its legitimate OS-dev subprocess command route while requiring the moved staging/hash API bindings directly. The registry contract intentionally retains `producerScript` for command execution and separately uses `producerSource` for catalog-refresh inspection.

Current executor evidence is:

- ordinary source ownership route: 11 passing tests, 565 assertions, 715 ms;
- isolated, cache-skipped `@semio-tech/repo-lib:test-os-dev-composition-ownership`: 11/565, 781 ms target time, exit 0;
- focused existing owner-import Vitest boundary: 17 passing and 72 skipped cases in 2.31 s;
- focused registry producer source-data boundary: one passing and one skipped case in 1.45 s.

The first focused registry control initially found a stale fixture `shellPluginId` (`space` versus obsolete markdown `s`). The fixture was corrected and the same focused control passed; this is retained as historical source-data repair evidence.

I accept the bounded taxonomy, public-owner, consumer, generator-authority, input, and registration closure. This acceptance does not claim a dev server, distribution publication, Cargo/Rust/Wasm build, Chromium/Playwright run, collaboration E2E, parity sweep, benchmark, or full OS product lifecycle. The focused owner-import and source-data checks are not substitutes for those routes.

## Outstanding integration work

This accepted extraction has no whole-lane runtime claim. Private browser-host staging, distribution publication, activation lease/cancellation, and the selected native product seams remain unexecuted integration work.

The separate quick-route fixture still reads the absent historical WGPU package command at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` (with historical sibling `🌐️.html` and `Trunk.toml`). It fails with `ENOENT` before the staged-root assertion. This coordinate is neither an accepted OS-dev owner nor repaired by the 49-owner extraction; it needs its own current WGPU fixture/command integration repair.
