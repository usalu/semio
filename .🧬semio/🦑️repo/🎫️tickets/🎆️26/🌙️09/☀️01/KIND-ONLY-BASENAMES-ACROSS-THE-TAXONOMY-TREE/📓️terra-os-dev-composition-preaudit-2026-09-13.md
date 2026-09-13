# OS Dev Composition Script Extraction — Independent Pre-Audit

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
