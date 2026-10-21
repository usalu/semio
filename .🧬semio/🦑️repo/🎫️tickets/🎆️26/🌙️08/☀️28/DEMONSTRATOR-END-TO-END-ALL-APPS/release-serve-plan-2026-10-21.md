# Release Build and Final Live Serve — 2026-10-21

Source-only verification plan, not an execution result. Current manifest and site/runtime sources were read. No build, serving process, graph dry run or extra E2E invocation was launched. Parent retains the current E2E process and owns scheduling.

## Plain Release Build

From the workspace root use the existing authored route:

```sh
repo --tool=nx run @semio-tech/mit-bestand-demonstrator:build --parallel=1 --workers=off --output=stream
```

Set ticket and distinct generated artifact environment as for other checks. Do not inherit `SEMIO_BUILD_MODE=prototype`; this target declares `metadata.semio.buildMode.fixed=release`, `defaultConfiguration=release` and its script environment explicitly selects release. No configuration or compiler argument is required. The site script refuses compiler arguments.

The target requires prepare-release plus the surface/editor/node-graph/puzzle WASM targets. Prepare-release owns nine release preparation lanes (Aggregator, Aussuchen, Bearbeiten, Energy, FEM3D, Generation3D, Generator, Koordinator, Verfolgen), verifies real component markers/files and checks runtime session variant/plugin identities. Build prefetches authored map tiles and calls the owned Vite artifact builder with release mode, React, build/play artifact inputs, native config loader, owner demonstrator:site and bundled tile serving. The declared build output is the demonstrator .🧾️store/build.json receipt. Cache is disabled for this build target.

Proof requires actual terminal exit 0 of this command and the resulting owned release artifact/receipt; prerequisite admission or successful prototype E2E does not prove it. Preserve the complete output under generated until parent finishes validation.

## Final Live Service

Use a separate authored continuous route:

```sh
repo --tool=nx run @semio-tech/mit-bestand-demonstrator:serve --parallel=1 --workers=off --output=stream
```

For this command set `SEMIO_BUILD_MODE=prototype`, ticket/artifact environment, and optionally `MIT_BESTAND_DEMONSTRATOR_PORT=6029` (the current default). Serve accepts no positional or compiler arguments. Its declared activate-prototype dependency owns prepare-prototype and real lane activation. The runtime reads the completed activation receipt, requests prototype React serve/play inputs and Vite native config. It binds 127.0.0.1 on native hosts, 0.0.0.0 when DEVCONTAINER=true.

Keep this command alive after an actual `Demonstrator ready: <url>` line; use that reported URL for final browser inspection and real pane smoke/console checks. Do not claim a queued or preparing command is a live service. This route serves prototype runtime bytes. It is separate from the plain release build proof; the current source declares no release static-server route, and none should be invented for verification.

## Existing E2E

E2E 98132 is already admitted and retained by runtime_execution, with full stream in generated/e2e-contract.log. Its authored serve-e2e path creates an isolated generation and free local port after activate-prototype and workspace browser prerequisites. Test runner logs its actual URL and executes the existing twenty cases through Chromium. This route must not be duplicated or restarted while live. Case success requires assertions and automatic page/console health teardown; screenshots prove rendered pixels only after actual case completion.

Required outstanding evidence remains actual unit, runtime catalog, architecture, exact infrastructure regressions, all twenty browser cases, plain release build and final live serve. Source inspection here does not assert any of those passed.

## Independent Source Review

Closure audit crosschecked this plan against existing release/serve boundary research and current manifest: release build proof remains terminal0+owned publication receipt; final authored prototype serve remains separate and must report actual readiness URL. The plan does not claim preparation, screenshots or prototype tests prove release. No concrete readiness-claim or command-scope defect found by source review. Live process ids/status are time-bound owner reports, not results guaranteed by this document. No command launched by reviewer.

## Plain Release Configuration Source Proof

The missing raw configurations object does not imply an unsupported Nx configuration. The current call path explicitly normalizes it:

1. build/🧭️invocation/🟦️.ts resolveBuildInvocationV1 reads the selected owner manifest through readSelectedBuildTargetModesV1. It captures build.metadata.semio.buildMode.fixed=release and the release default; resolveBuildModeV1 chooses that fixed mode before backend preparation and refuses a contradictory inherited mode.
2. build/🎚️mode/🟨️.mjs targetBuildModes copies any raw configurations and then synthesizes BOTH prototype and release configurations from buildModesV1. Each contains the target environment plus SEMIO_BUILD_MODE for that mode; it sets defaultConfiguration=fixed mode and records metadata buildMode fixed/modes.
3. build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs uses targetBuildModes before validating the selected overrides, so release exists at admission despite the raw manifest omitting configurations. The actual projectWithDefaults in sibling 🧱️project-assembly/🟨️.mjs assigns every normalized target from loaded.targetBuildModes and returns targets:normalized in the published graph.
4. The private Nx graph reader in caching/bootstrap/nx-plugin/graph-pin consumes the pinned published graph rather than inventing a raw target. The installed Nx create-task-graph.js resolveConfiguration checks metadata.semio.buildMode: it selects policy.fixed or the initiating mode, rejects an unsupported/mismatched mode, then resolves the actual normalized target configuration and default.

Thus the documented plain build argv needs no extra release configuration flag. This is source proof of route/configuration availability; actual release compilation and terminal success remain pending. No build or dry run was executed to establish this finding.

## Current Finite Preflight Audit

Read-only audit repeated against the current demonstrator manifest, root/runtime/site routers, site builder, Vite configuration, activation implementation, E2E runner and service-session identity helpers. No graph job, build, listener or browser was started. Storage and all verification Source remain frozen. Earlier process98132 references above are historical; it was settled as obsolete preparation and is not a current server. No application assertion is claimed by this audit.

The release build remains an authored exact target with fixed/default release mode and release script environment. It requires all nine release preparation lanes plus the declared engine/plugin WASM producers. Site runBuildScript rejects extra compiler arguments, prefetches the bounded Switzerland raster/vector tile plan through zoom10, refuses any failed download or missing raster/vector canary, then calls the managed artifact builder with owner demonstrator:site, release mode, build/play inputs and bundled map tiles. Upstream tile availability is an actual build prerequisite, not evidence already obtained. The release output remains the owned build receipt and managed artifact inventory; terminal0 plus a current authenticated receipt/artifact is required.

Current serve/dev still require activate-prototype and the three-lane merged prototype component receipt. Runtime serve derives Vite root from its module root, uses explicit mode from the issued environment, serves completed React play inputs, and announces the actual URL only through the owned ready callback. Vite command=serve reads prototype activation independently of selected mode. Thus setting release on this dev route cannot prove release artifact HTTP serving. No serve-release target or static managed-artifact viewer has appeared since the earlier audit. Final live prototype service evidence and release build publication evidence remain separate.

The E2E path retains a unique owner/invocation generation, port0 service and readiness handshake before setting PLAYWRIGHT_BASE_URL for its child. Chromium runs with one worker and real Metal on this macOS host; a software-only adapter does not establish Verfolgen GPU/map correctness. Completion requires all twenty assertions plus health teardown, clean actual console/page/resource observations, and retained bounded visual artifacts; a ready listener or screenshot alone does not satisfy it.

One concrete source wiring gap was found: prefetchDemonstratorMapTiles forwards no AbortSignal even though the owned GIS prefetchMapTiles API accepts one and threads it to bounded fetches and job cancellation. Site runBuildScript creates a local abort controller but passes neither that controller nor the issued invocation signal to tile prefetch. This is a cancellation preflight gap for the expensive release prefetch, not evidence that a build has failed. No repair was made during the current Source freeze. Runtime serve also creates local signal controllers but passes the issued invocation signal to serveVite; parent should preserve the issued lifetime rather than infer local controller cancellation is the service lifetime.

No additional confirmed preparation/route blocker was established by source inspection. Actual component markers, managed output bytes, tile network/cache state, port availability, engine compilation, release publication, all browser interactions and final ready listener remain execution checks.

## Map Prefetch Cancellation Repair

Parent authorized the concrete cancellation repair while keeping defining storage modules frozen. Schema and four language-neutral cases were saved first in the existing demonstratormaptiles test owner: live completion, initially cancelled invocation, cancellation during prefetch, and tile failure. The existing unit helper now extracts the actual wrapper and site functions through the TypeScript compiler, checks the actual issued-signal call argument through its independent AST, validates the neutral contract with Ajv, and compares the observed prefetch/build/cancellation outputs to each case. No external network is used by this regression.

Implementation now requires the issued AbortSignal in prefetchDemonstratorMapTiles, checks cancellation before starting and immediately after awaiting the owned GIS prefetch, and forwards that same signal into the existing bounded GIS fetch/job API. Site runBuildScript passes context.invocation.control.signal to prefetch and the artifact builder; its unused independent controller/listeners were removed. The process invocation receiver already owns SIGINT/SIGTERM cancellation of the issued invocation, so no competing cancellation lifetime is created. The real build still performs the same tile plan/canary/failure checks and managed release artifact publication.

Touched files: site/🟦️.ts; site/🗺️map-tiles/🟦️.ts; demonstrator 🧪️tests/🧪️demonstratormaptiles/{🟦️.ts,🧫️cases.json,🧬️schema/🔣️.json}. Native unit execution remains pending the current SourceDemand immutable snapshot admission; no passing result is claimed.

## Planned Vector Canary Repair

A subsequent finite check found that3/1/2.pbf is outside the actual prefetch plan. The owned GIS plan defaults to Switzerland bounds west5.95/south45.82/east10.52/north47.81; its bounded enumerator maps both longitude bounds to x4 at zoom3 and the latitude bounds to y2. A cold cache populated only by those successful jobs cannot contain the old x1 canary. Parent authorized the same-domain repair without changing storage or restarting the still-pre-graph unit.

Schema and neutral canary/cache cases were saved before the one-string runtime fix. The test extracts the actual GIS coordinate/range/list functions through the TypeScript compiler, compares the entire authored zoom0..10 plan with the existing independent d3-geo Mercator oracle, proves3/4/2 inclusion and3/1/2 rejection, and executes the actual wrapper against cold and warm synthetic caches containing only planned jobs. The wrapper's actual bounded options, skipExisting and issued signal are checked. The vector canary is now3/4/2.pbf; raster remains0/0/0.png. No network, raw cache mutation, new dependency, package manifest edit, storage Source edit, or native restart was introduced. Actual unit result remains pending session9640.

## Explicit Oracle Data Observation

Independent review found the computed helper argument holding the GIS TypeScript path is outside commandUrlReferences literal newURL/join recognition. Direct default inputs cover the demonstrator, and the test imports d3/TypeScript/fs rather than executing the GIS runtime module, so there was no exact GIS-JS data observation declaration for that computed source read. The schema path was a literal newURL and discoverable, but both oracle resources are now declared explicitly for a self-contained contract.

Schema-first existing compile-closure fixtures add exactly the GIS tile Source and its plan schema as freshness data. A TypeScript AST projection derives those two actual out-of-project read paths from the map helper and compares them with the neutral declaration; both test and canonical-architecture retain default/^default and their existing inputs, all sixteen executable runtimeSourceEntries, execution version1/source and freshness observation. No GIS runtime import, extra executable entry, producer bypass, package change, or storage Source edit was added. The still-pre-graph unit9640 is retained under normal Source reconciliation.

## Energy Oracle TEXT Resource Reconciliation

Acceptance added two literal TEXT reads to the already configured console-unit entry: the authored Energy simulation policy JSON and framework Rust run-job driver Source. Schema-first closure fixtures now list exactly those DATA resources, and the existing independent TypeScript AST projection derives the actual external newURL reads from the console entry. Both test and canonical-architecture metadata append these paths to closure.resources while preserving other closure fields, GIS freshness inputs, default/^default, canonical environment inputs and all sixteen executable entries. The Rust file remains DATA and is not imported or promoted into a native compiler traversal. Production storage sources remain untouched.

## Receipt Recording Trace — 2026-10-21T06:20:47.514Z

The current site helper publishes the managed release output through buildViteArtifact, withRepositoryArtifactDestinationV1 and stageRepositoryArtifacts. The destination owner records the completed output directory via recordStoreContentV1. The task issuance protocol sets its declared .🧾️store/build.json destination and SEMIO_TASK_PROJECT/TARGET environment; recordStoreContentV1 observes sorted path/size/content hashes and writes the exact version1 project/target receipt. Thus the site helper does not need a separate manual receipt write. Final proof must read the actual build receipt and check current entries/digests, plus the managed .semio-artifact.json owner demonstrator:site and nonempty site files. No release execution or artifact authentication has yet occurred.
