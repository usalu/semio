# Development Graph Refactor

Nx 21.6.11's daemon/watch mechanism is now qualified on this macOS workspace (see daemon verification). The existing OS development router still owns an independent catalog scheduler, compiler/materializer semaphore, leader/follower startup lease, and Rust file watcher; these remain task-scope violations until replaced together.

## Current Ownership

`buildPluginCargo` compiles one Cargo package for `wasm32-wasip2` using `wasm-dev` or `wasm-release`, a fixed 8 MiB stack, and optional symbols. Its artifact currently remains in mutable Cargo state. `materializePlugin` transpiles that component, writes a source-owner descriptor and JSON, publishes a browser bridge and host shim, installs extensions, and writes a global wall-clock hot-swap marker. `preparePluginBuildTargets` additionally installs the Rust target, regenerates the registry/session, populates vendor shims, runs a font-dump compiler on an existence shortcut, and synchronizes unrelated outputs.

`buildEngineWasm` builds surface, editor and flow-core unconditionally for React, then merges Cargo playground `engines` with package `semio.browserSessionFactories`. The Cargo declarations are source metadata and can supply variant edges before the generated catalog exists. The current DevScript combines all of the above with Vite/trunk and a custom watcher. BuildScript recursively executes the same pipeline before Vite or trunk.

## Required Task Boundaries

1. Infer one component producer per Cargo plugin package and profile, with immutable staged component output separate from Cargo's compiler store.
2. Infer one materializer per package and profile, consuming that staged component and declaring every descriptor/bridge/core-WASM output. Move compiled descriptors out of source owners and update consumers at the same time.
3. Give vendor shims, shard worker, and guest-slim font bytes separate producers; replace existence checks and compiler invocation in consumers with graph prerequisites.
4. Infer each variant's session and preparation target from Cargo playground declarations plus composition session factories. Include only its real plugin/engine closure.
5. Keep server targets continuous and uncached. Nx watch selects the preparation/build graph; it does not implement another project scheduler. Ensure cancellation owns watch callback descendants as well as the server.
6. Separate cached materialization from runtime installation/notification. A global Date.now hot-swap marker cannot be a cached deliverable. Profile-specific staged outputs avoid two profile tasks owning the same mutable browser module directory.

The observer must work when outputs are missing after checkout, restored from Nx, or rebuilt following dependency changes. A warm preparation must execute zero compilers. The browser must consume a restored module and a source-change rebuild through the real development server. Existing callers in demonstrator, test/bench harnesses, and Hub must move with their producer contracts.

## Producer Progress and Next Session Boundary

The component and materialization producers now exist for all 59 Cargo components and both profiles. Browser support outputs also have explicit producers and passed deletion/restoration plus a Node import. The output tree is the plugin TypeScript package’s `dist/<dev|release>/🔌️plugin-modules`, with disjoint ownership for each module, Preview2 vendor directory, and shared shard directory. See [materialization](📓️browser-materialization.md). Materializer cancellation and bounded diagnostics passed the public repository contract suite, including an actual subprocess ignoring SIGTERM.

The next narrow boundary is one cached playground session per Cargo-declared variant, owned by the registry at `dist/sessions/<variant>/🟦️session.ts`. Its prerequisite is registry generation; the generated dependency bytes are inputs. The existing `writePlaygroundSession` emits a standalone module and can be reused without a compiler. Vite currently imports one globally rewritten `🧑‍💻dev/🤖️generated/🟦️session.ts`; change that consumer to the variant artifact when introducing the preparation graph.

Variant preparation can then use authored Cargo `metadata.semio.playground` rows and `metadata.semio.host` (host means all catalog components), plus `metadata.semio.depends-on` and the composition’s `semio.browserSessionFactories` engine declarations. React’s current baseline surface/editor/flow-core and the composition’s Puzzle WASM producer must be explicit edges. All server/build consumers currently still use the original catalog scheduler.

## Session and Selection Verification

All 60 variant session targets now have editor launch entries. The Note session was deleted and restored from Nx with identical bytes; the Studio session was unchanged. Node independently imported both and checked variant identity, host mode and plugin membership. See [session restoration](📓️playground-sessions.md).

Root development selection now reads authored Cargo rows before Nx starts and has a passing independent TOML oracle, including same-mtime invalidation and missing/stale generated data. See [selection](📓️playground-selection.md). The font tool/data boundary and the actual preparation/server integration remain in progress.

## Authored Preparation Graph

The Nx plugin now infers 120 React preparation targets from Cargo component/playground declarations and the composition’s linked engine declarations. For each variant it closes declared runtime dependencies (including extension hosts) and topic providers; host variants include all components. Every engine must resolve to an authored project with a WASM target. The Flow playground incorrectly named generated bindings as its engine; its Cargo declaration now names the real producer. Surface’s output contract was corrected from `pkg` to its actual `🕸️bindings` output, with a TypeScript compiler oracle. Flow’s two actual output directories were retained.

The public repository contract suite passed all variants/profiles and generated editor entries. Nx’s real task graph for Note contains 13 tasks: input guard, registry, Note session, browser support, font tool/data, four browser engines, Note WASI component, Note materializer, and preparation validation. Root selection sets the build mode before graph creation. The uncached preparation leaf only validates completed session/module/support/font outputs. Actual preparation is running; the existing dev/server and custom watcher paths still need replacement.

## Consumer Integration Constraints

Vite still hardcodes `dev/🔌️plugin-modules`, imports the global rewritten `dev/🤖️generated/🟦️session.ts`, and resolves every catalog engine by opening generated package metadata during config evaluation. The prepared profile tree and per-variant session must replace these consumers together with the public React dev route. The font route currently expects `🪞️vendor/🔤️guestslim-typst-fonts.bin`; serve it from Infinite’s staged font output. The existing extension publisher copies completed modules into the runtime installation store; preserve this as an uncached runtime operation, driven by descriptor hashes, rather than as a materializer side effect.

The existing SSE observer reads a global hot-swap marker and never closes its filesystem watcher. Replace it with an observer of complete atomically staged module directories, comparing descriptor digests and emitting runtime notifications only when content changes. Nx must own source watching/rebuild scheduling; the Vite observer only announces completed outputs. Server close must close the output watcher and SSE keepalive timers. The old scheduler, leader/follower lease, plugin build semaphore and Rust source watcher remain to be removed with this integration.

## Completion Notification Boundary

A filesystem observer of cached output directories alone is insufficient: Nx restoration may copy descriptor/marker files before a large core WASM file finishes, so directory existence or a short debounce cannot prove a complete restore. Use an explicit uncached activation target after preparation to install source-owned extensions and publish a per-variant/profile runtime receipt. The server’s SSE observer reads that receipt; it never infers compilation or restore completion from mtimes. Compare descriptor digests with the previous receipt so unchanged warm preparation does not trigger plugin reloads. This receipt is ephemeral runtime state, separate from cached component/materializer outputs. Nx watch callbacks select this activation graph; they do not compile or schedule individual components themselves.

## Activation Receipt Contract

The authored activation schema and language-neutral cases now define a content-based receipt, sorted by plugin identity. An unchanged completed artifact retains its activation timestamp even when the clock advances; changed content receives a strictly increasing timestamp even if the clock moves backwards. This timestamp is a runtime cache-busting identity, not a compiler mtime. Invalid identities, duplicate plugins and unexpected receipt fields must fail validation. The independent reference uses Lodash ordering/indexing and Ajv schema validation; runtime code has no third-party dependency.

Consumer integration will use a dedicated development installation namespace per variant/profile, separate from users’ persistent extension installations. The receipt is outside every cached deliverable. The activation step must hash all relevant module/support bytes, complete extension publication, and only then publish the receipt. The observer must close its filesystem watcher and active SSE connections on server shutdown. These changes are in progress; no live-server completion claim is made yet.

## Receipt Verification Evidence

`repo:test` passed the new activation model, including the independent Lodash/Ajv oracle (`activation-green.log`). The first native filesystem observer test failed: an atomically replaced Unicode receipt did not produce the expected content notification under Bun. The comparison fixture also uses Chokidar and retains raw watcher events for diagnosis; no observer correctness claim is made until that failure is resolved.

The corrected observer passed under Bun with an independent Chokidar receipt observer (`activation-observer-corrected.log`). Bun emitted the source temporary filename for atomic replacement rather than the destination filename. Observing events in the dedicated receipt directory and reading only the published receipt fixes that behavior; partial module writes and unchanged receipts still emit no activation.

The first actual Note preparation did not complete. Its Nx executors reported `readCachedProjectGraph: No cached ProjectGraph is available`, and the parent exited 130. This is a shared workspace graph-state failure, not a successful engine preparation or restoration. Existing separately qualified component/support producers remain recorded in their own reports; the complete preparation requires a fresh runtime run.

All 120 inferred activation targets and matching editor entries passed `repo:test` (`activation-graph-green.log`). Each activation is uncached and depends only on its matching preparation. The implementation hashes completed component and shared support/font bytes, installs source extensions into the variant/profile runtime namespace, and atomically publishes a content-based receipt. Live activation and the Vite consumer are being exercised separately before connecting the public development route.

## Runtime Activation and Adapter Checks

The Note activation leaf completed against the real staged component, session, browser support and font assets (`activation-leaf-ready.log`). This invocation deliberately excluded task dependencies to isolate activation behavior; it is not evidence that the full four-engine preparation passed. A prior isolated attempt correctly refused to activate while the Note materializer directory was temporarily absent during a restoration probe.

The actual Vite adapter passed snapshot delivery, exactly one changed-content event, unchanged receipt suppression, HTTP-close teardown, and bundle-close idempotence (`activation-sse-green.log`). The test used the production adapter and production receipt publisher with an actual filesystem watcher. Chokidar independently observed the same completed receipts.

The earlier component, font and WGPU restoration probes were cancelled after source changes invalidated their warm hashes and caused fresh compilation. Their finally handlers restored all missing backups. Their successful warm producers are useful execution evidence, but these attempts do not establish cache restoration. Run restoration qualification after the relevant input definitions and implementation bytes are stable.

## Actual HTTP and Browser Consumer

A real Nx `serve-note-react-dev` leaf started Vite on an isolated port. HTTP returned byte-identical Note bridge, descriptor, extracted core WASM and staged font bytes, and the live SSE endpoint returned the Note activation snapshot. Chromium independently fetched and decoded the descriptor, and the page title/body identified the Note editor. This used existing staged prerequisites to isolate the new consumer path (`activation-server-http.log`). The app also reported `module.vcs: snapshot read retirement factory is not installed`; full app behavior is not passing, and the full preparation graph must be rebuilt before attributing that runtime mismatch to current source. Browser logs and screenshot remain in the ticket’s generated evidence.

The root coordinator now owns Nx source watching as a sibling of the server task graph. It waits for the pinned Nx watch readiness notification before launching the server and routes changes through the activation target. Public graph inspection does not start watchers. The existing isolated daemon/callback cancellation regression passed with the new coordinator class (`activation-coordinator-regression.log`); a new combined watcher/server fixture is still being debugged.

## Combined Coordinator Result

The combined real-Nx fixture passed (`development-coordinator-source-watch.log`; see `📓️development-coordinator.md`). It used the actual root coordinator class, established watcher readiness before starting the server graph, rebuilt an emoji-path dependency, delivered changed content over HTTP, ignored generated reports, and on SIGTERM closed the server, an ignoring child, and the occupied port with parent exit 143. Initial fixture failures were authored test setup issues (escaped newline and missing nested-workspace ignore exceptions), corrected before this passing run.

The actual Note server was stopped through its public root runner; it exited 143 and port 6279 was released. The latest complete repository contract run passed after the React dev leaf stopped invoking its custom build scheduler (`react-dev-contracts.log`). The root public React and multi routes now select explicit variant targets. Existing WGPU, plugin, build and E2E commands still retain parts of the old scheduler and must be refactored before declaring the monorepo complete.

## Nx 23 Restoration and Startup Cancellation

The font tool and packed asset, browser support directories, and variant session restoration probes all completed with exit 0 under Nx 23.2.0. Each deleted only its owned output, restored identical bytes/modes from Nx, and exercised an independent consumer. OpenType parsed all 17 restored fonts; Node imported the Preview2 IO module; Node imported both Note and Studio sessions and verified their independent membership. Evidence: `nx-23-font-restoration.log`, `nx-23-support-restoration.log`, `nx-23-session-restoration.log`; retained details are in `📓️font-artifacts.md`, `📓️browser-support-artifacts.md`, and `📓️playground-sessions.md`.

Early watcher startup cancellation also passed with exit 143 and no surviving launched child (`nx-23-startup-cancellation.log`). Full Note preparation continues through its real WASI component prerequisite; the complete warm graph and product runtime are still unqualified.

## Full Preparation Failure After Compilation

The first Nx 23 complete preparation exited 130 after 19m43s. All four compiler invocations completed, and Note component/materialization completed. Flow failed during its browser wrapper bundle: the authored rewrite pointed from the wrapper source to a nonexistent `../../🕸️bindings/flow_core.js`, while its resolver only externalized the intended sibling `./flow_core.js`. The graph remained valid at 307 projects with no persisted errors. The wrapper rewrite and its existing browser assertion now name the packaged sibling. This failure is recorded in `nx-23-note-preparation.log`; the corrected browser test and full preparation still require execution.

The corrected Flow browser package test passed against its real compiled module and bundled wrapper (`nx-23-flow-browser-sibling.log`). It covered two sessions, exact retirement acknowledgements, cancelled/rejected opens, bounded close polling, initializer ownership and sibling module references. Complete preparation is being rerun after the native input boundary changes; no full warm graph or app success is claimed yet.

## Next Build Consumer Boundary

The current Vite module evaluates a runtime activation receipt before exporting its configuration. That prevents a build from consuming prepared artifacts without prior development activation, and makes runtime receipt membership influence cached build selection. The prepared variant session already owns the same complete component membership and should supply pure build/config selection. The activation observer starts only in configureServer, so it can remain specific to serving while build selection uses session data.

A further ownership split is needed for source extensions. The Vite static extension copy still consumes runtime/extensions, whose JavaScript shims and installation metadata are currently rewritten during activation. A cached distribution must consume an independently staged extension deliverable with deterministic metadata and correct vendor routes. Runtime installation/timestamps should remain uncached. Do not cache an activation receipt or a users' mutable installation directory to bypass this boundary.

Existing distribution generation has a schema-owned compiler partition (`🚚️distribution/📇️layout.json`, `🔗️inputs.json`), captures actual Vite/Rollup input witnesses and publishes an owned manifest. Its renderDistributionBundle currently imports the same Vite config, sets Studio/React in process environment, and assumes a configuration object. Any factory/config split must update this caller together. Its current DistributionBundleScript also uses inline Bun evaluation and tempdir fallback, which remain script/output-ownership violations to resolve.

WGPU TrunkBuildScript/TrunkServeScript still call tool/target setup, boot bundling and worker checks themselves. Trunk serve additionally owns compiler watching; OS DevScript calls buildPlugins/buildEngineWasm and has port-based process takeover. These must become explicit compiler/asset preparation targets plus a server leaf and the root Nx watch coordinator. The current native WGPU build/runner boundary does not finish the browser Trunk path.

## Native Input Qualification Follow-Up — 2026-09-08

The full Note React development preparation completed successfully in 67m13s with 13 tasks and no cache hits. This run overlapped source and contract edits, so it establishes successful preparation only; it does not establish warm performance. The follow-up uses the new explicit native generator prerequisites and is running in `native-inputs-note-repeat.log`. Real browser execution, source watch behavior, and cancellation remain to be qualified against its completed outputs.

## Repeat Note preparation, 2026-09-08 12:15 CEST

The repeat run finished with failure (outer exit 130), after 57m21s, 14 executed tasks and no cache hits. The causal failure was `note-plugin:component-dev`: ten Rust errors in DWG snapshot/I/O sources compiled through Stdio (slice-versus-fixed-array callbacks and TryInto on fixed-array references). `prepare-note-react-dev` and Note materialization did not run. This is not a successful warm-cache qualification. Evidence: `🗑️generated/native-inputs-note-repeat.log`.
