# OS Development Composition Semantic-Owner Plan

## Scope and current authority

This plan covers the 5,585-line command source at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`. The source currently combines plugin build and installation, browser-host staging, engine preparation, activation and serving, four static verification domains, three browser scenarios, parity, distribution compilation and publication, scale-fixture rendering, three benchmark modes, contract validation, canonical bootstrap mirror verification, and command routing.

The final package command will retain its shebang, argument validation, `ScriptRouter` registration and `runBundleScriptMain`. It will import executable command classes or narrow functions from semantic owners. It will not export behavior as a compatibility facade, and it will not retain `import.meta.vitest` or behavior-bearing class bodies.

The extraction extends these existing authorities instead of copying them:

- activation contracts: `dev/♻️activation/🟦️.ts` and `dev/♻️activation/🌐️browser-host/🟦️.ts`;
- distribution contract and input data: `dev/🚚️distribution/🟦️.ts`, `⚙️inputs`, layout, schemas and input graph;
- plugin registry discovery/catalog/session, browser-bundle materialization/descriptor/distribution, deployment and installation owners;
- repository process/caching utilities and the framework hash owner.

## Schema-first ownership contract

Before behavior moves, add a repository-library portable ownership contract:

- schema: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧑‍💻os-dev-composition-ownership/🔣️.json`;
- fixture: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json`;
- test: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts`.

The fixture will enumerate semantic owners, package-router imports, dynamic consumers, source-as-data consumers, project inputs and the existing project/package/launch command routes. The first red requires all owners and direct consumer bindings, rejects command-source behavior declarations and `import.meta.vitest`, rejects the two hash facades, and keeps generated outputs and fixtures classified as data rather than behavior owners.

## Semantic owner map

All implementation leaves are anonymous. Parent concerns carry the domain and role. The map has exactly 49 anonymous semantic implementation owners plus the existing package router, for 50 rows total.

| Concern | Semantic owner | Current behavior |
| --- | --- | --- |
| Playground session command | `dev/🎮️playground-session/🏃️execution/🟦️.ts` | deterministic render, preview, check and output-root selection using the existing registry session renderer |
| Plugin catalog refresh | `plugin/📇️registry/🔄️refresh/🟦️.ts` | `ensurePluginRegistry`, filter resolution, complete-catalog enforcement and generated catalog refresh |
| Plugin build planning | `plugin/🏗️build/📋️plan/🟦️.ts` | profile/target selection, Cargo arguments, concurrency and stale-output preflight |
| Plugin build materialization | `plugin/🏗️build/📦️materialization/🟦️.ts` | component build, JCO materialization, bridge/shim rewriting and shard-worker publication through existing lower owners |
| Plugin descriptor publication | `plugin/🏗️build/🛂️descriptor/🟦️.ts` | descriptor staging/freshness and built-plugin description |
| Extension publication | `plugin/🏗️build/📥️installation/🟦️.ts` | built extension publication, install-root sync and freshness |
| Plugin build execution | `plugin/🏗️build/🏃️execution/🟦️.ts` | ordinary and streaming build command composition |
| Plugin watch execution | `plugin/🏗️build/👁️watch/🟦️.ts` | source-root watching and bounded rebuild routing |
| Plugin size inspection | `plugin/📊️size/🟦️.ts` | WASM section decoding, plugin/engine rows and report command |
| Browser-host staging | `dev/♻️activation/🌐️browser-host/🏗️staging/🟦️.ts` | owned input admission, exact Space descriptor arguments, selected GIS byte identity, private materialization and receipt closure |
| Engine selection | `dev/⚙️engine/🧭️selection/🟦️.ts` | linked-session engine projection and composition selection |
| Engine publication | `dev/⚙️engine/📤️publication/🟦️.ts` | injected/native engine build routing and WGPU prerequisite publication |
| Build lease | `dev/♻️activation/🔐️lease/🟦️.ts` | finite holder/follower lease, takeover, readiness marking and release |
| Readiness | `dev/♻️activation/🩺️readiness/🟦️.ts` | bounded TCP/HTTP/child-exit polling with cancellation/deadline behavior |
| Activated extension publication | `dev/♻️activation/📥️installation/🟦️.ts` | activation digest and atomic activated-extension publication |
| Staged freshness | `dev/♻️activation/🔍️freshness/🟦️.ts` | staged module fact collection and verdict/report projection |
| Preparation | `dev/♻️activation/🧰️preparation/🟦️.ts` | build prerequisites and holder/follower preparation orchestration |
| Activation | `dev/♻️activation/🏃️execution/🟦️.ts` | activation command orchestration with progress/cancellation |
| Serve | `dev/♻️activation/🌐️serve/🟦️.ts` | serve command and runtime activation orchestration |
| Capability policy | `dev/🧪️tests/🧹️capability-policy/🟦️.ts` | source walk, alias freshness and capability lint |
| Layering policy | `dev/🧪️tests/🧹️layering-policy/🟦️.ts` | manifest role parsing and Cargo layering lint |
| Export path policy | `dev/🧪️tests/🧹️export-path-policy/🟦️.ts` | plugin barrel relative-export resolution |
| Host handle policy | `dev/🧪️tests/🧹️host-handle-policy/🟦️.ts` | Rust host-handle scanner and lint command |
| Package tests | `dev/🧪️tests/🏃️execution/🟦️.ts` | explicit Vitest level/config selection only |
| Studio journey | `dev/🧪️tests/🎬️studio/🟦️.ts` | Playwright Studio workflow journey |
| Catalog smoke | `dev/🧪️tests/🔬️catalog-smoke/🟦️.ts` | catalog probe, report projection and browser execution |
| Collaboration journey | `dev/🧪️tests/🤝️collaboration/🟦️.ts` | private ports, services, browser actions, restart and cancellation |
| Verify routing | `dev/🧪️tests/✅️verification/🟦️.ts` | selection among the three distinct verification owners |
| Structural parity | `dev/🧪️tests/⚖️parity/🏗️structure/🟦️.ts` | React/WGPU structure acquisition and pure comparison |
| Pixel parity | `dev/🧪️tests/⚖️parity/🖼️pixels/🟦️.ts` | owned pixel diff algorithm, crop/decode/encode and region result |
| Parity probes | `dev/🧪️tests/⚖️parity/🔬️probe/🟦️.ts` | state probes and changed-path projection |
| Parity server pool | `dev/🧪️tests/⚖️parity/🌐️server-pool/🟦️.ts` | finite browser/server startup, reuse and teardown |
| Parity report | `dev/🧪️tests/⚖️parity/📊️report/🟦️.ts` | report data and artifact publication |
| Parity execution | `dev/🧪️tests/⚖️parity/🏃️execution/🟦️.ts` | smoke, triage, probe, verify and sweep command classes |
| Distribution source admission | `dev/🚚️distribution/📥️source/🟦️.ts` | component-wise no-follow input discovery and source witnesses |
| Distribution plan | `dev/🚚️distribution/📋️plan/🟦️.ts` | exact input/output plan and path ordering |
| Distribution compiler | `dev/🚚️distribution/🏗️compiler/🟦️.ts` | detached Bun/Vite compilation against the semantic owner URL |
| Distribution publication | `dev/🚚️distribution/📤️publication/🟦️.ts` | private preview, atomic publication/recovery and manifest output |
| Distribution freshness | `dev/🚚️distribution/🔍️freshness/🟦️.ts` | byte/source-witness comparison and check result |
| Distribution execution | `dev/🚚️distribution/🏃️execution/🟦️.ts` | generate/preview/check command composition |
| Scale fixture projection | `os/🧫️fixtures/⚖️scale/📽️projection/🟦️.ts` | seeded artifact projection and byte plan |
| Scale fixture publication | `os/🧫️fixtures/⚖️scale/📤️publication/🟦️.ts` | preview/generate/check publication commands |
| Benchmark plan | `dev/📊️benchmarks/🔌️plugins/📋️plan/🟦️.ts` | budgets, row schema and selected mode plan |
| Native benchmark | `dev/📊️benchmarks/🔌️plugins/🖥️host/🟦️.ts` | actual native scale benchmark process and report rows |
| Browser benchmark | `dev/📊️benchmarks/🔌️plugins/🌐️browser/🟦️.ts` | Chromium React/WGPU rows using the existing harness |
| Stub benchmark | `dev/📊️benchmarks/🔌️plugins/🧪️stub/🟦️.ts` | explicitly labelled controlled stub rows only |
| Benchmark execution | `dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts` | mode dispatch and report publication |
| Development contract validation | `dev/🧬️schema/🛂️validation/🟦️.ts` | schema URL and strict named predicate compilation |
| Canonical bootstrap mirror verification | `dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts` | source and private process phases, Ajv and independent SHA-256 oracle |
| Command routing | `dev/📦️packages/🟦️typescript/📜️script.ts` | shebang, imported route registration and default command only |

The map may consolidate a pair only when the live dependency closure proves it is one semantic operation. It may not consolidate unrelated rows into a generic replacement monolith.

## Consumer and source-data closure

The extraction will rebind these high-risk consumers directly:

- Hub browser-host process proof and its source contract import `stageTestBrowserHostV1` from browser-host staging.
- Registry playground-session test reads the catalog-refresh owner when it constrains `ensurePluginRegistry` behavior; it no longer slices the command file.
- The current 1,900-line browser-host test is split by concern or imports the relevant semantic owners directly. No injected command-module facade remains.
- Distribution detached evaluation imports the distribution compiler owner URL, preserving the Vite-cycle break.
- All command-module `blake3Hex`/`Blake3Hasher` imports move to `🧰️framework/🔨️modules/🔏️hash/🟦️.ts`, including OS root/host/replication commands, store identity, descriptor verification and remaining Hub dynamic sites found by a fresh exact scan.
- `DEV_SCHEMA_URL` and `devContract` consumers import the schema validation owner.
- Existing generated-session, distribution and scale outputs remain data; their freshness owners and generator contracts list every source owner as an input.

## Test sequence

1. Run the new portable ownership fixture before source movement and retain the owner-absence/router-behavior first red.
2. Extract owner groups incrementally and run focused direct imports after each coherent group.
3. Run private browser-host staging controls for GIS byte identity, regular-file and ancestor/symlink admission, Space descriptor arguments and receipt closure.
4. Run engine selection vectors and injected build success/failure; label these as portable process-boundary evidence, not a native Wasm build.
5. Run distribution source graph, detached compiler, hostile publication/recovery and preview/check controls only under ticket-private output roots.
6. Run activation lease/readiness/freshness controls with explicit cancellation and deadlines.
7. Run contract Ajv predicates, pure parity algorithms and the existing focused canonical mirror source/process routes with ticket-private artifacts.
8. Run the existing registered focused routes whose behavior changed: browser-host staging, generated-session preview/check, distribution preview/check, scale preview/check and canonical mirror source/process. Do not use live `generate`, server, broad collaboration, parity sweep or benchmark targets as generic acceptance.
9. Run one isolated registered ownership target through Bun/Nx with private Nx state/cache/tmp. Add exact semantic owners to the OS dev project named inputs and generator-contract input patterns before accepting cached routes.

## Runtime limits

The extraction does not claim a full dev server, all-plugin build, browser journey, collaboration scenario, parity sweep, benchmark suite or live distribution publication. Native or browser proof is required only where a moved process boundary changes; injected command stubs remain explicitly labelled. Every spawned process keeps the existing progress, signal, cancellation and finite deadline behavior.

## Attribution and scratch

Owned scratch is `🗑️generated/sol-os-dev-composition`. Retain authored schemas, fixtures and Markdown reports; remove only disposable compiler/test outputs after final evidence is recorded. Shared taxonomy, project, package and launch files will be edited narrowly against their live concurrent state. No Git, lifecycle or AGENTS file mutation is in scope.
