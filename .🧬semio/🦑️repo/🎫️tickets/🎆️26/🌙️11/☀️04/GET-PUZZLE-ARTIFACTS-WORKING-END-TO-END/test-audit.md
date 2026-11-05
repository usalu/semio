# Puzzle Test Audit

Read-only audit on 2026-11-04. Source files were not modified.

## Confirmed Defect

`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts` navigates obsolete story IDs `puzzle-2d--overview-select`, `puzzle-2d--lasso-select`, `puzzle-2d--brush-utility`, `puzzle-2d--brush-placement`, and `puzzle-2d--forced-lod-pane` (first occurrence line 64). Their owner `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️2d-board/🧪️.story.tsx:351` now declares title `🧩️puzzle🩻️2d`, without an explicit metadata ID. The browser test therefore cannot reach its intended story. Prefer resolving IDs from the served Storybook `index.json` by title/export, or use Storybook CSF ID generation behind the test interface; avoid creating another independent title sanitizer. The spatial suite already uses the emoji-title convention.

## Discovery and Coverage

Both Playwright suites are explicitly registered at `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts:37` and line 38. Their declared owner is `repo run workspace:test-storybook`. The first broad search missed hidden `.storybook`; there is no confirmed Playwright discovery defect.

`@semio-tech/puzzle-js:test` is a separate Vitest target. Its config `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️config/🟦️.ts` includes transform gesture, renderer coordinate ownership, committed example definitions, and diff parsers. It does not execute the Playwright suites. The renderer boot variants fixture has four aliases/coordinates, but its test only inspects catalog plugin/app ownership and parsed app role; it does not boot renderers.

The 2d browser suite asserts renderer interaction through `Board2dHost`, but its story uses the local `reduceStoryPuzzle2dAction` reducer (story lines 327–335) for mutation/selection state. This is valid renderer coverage, but it cannot establish native Puzzle plugin dispatch, persisted mutation, or Rust implementation end-to-end behavior. The existing test header describing real WASM BoardSession checks is broader than the actual plugin coverage.

## Validation Limits

Inspected fixtures, target/config wiring, browser suites, story metadata and controlled reducer wiring through `repo tools exec --light python`. No browser or test target was launched by this audit agent; the parent owns runtime verification.

## Follow-up: Production React/TypeScript Imports and Host Props

Inspected all Puzzle production TSX entrypoints and the four development stories. Puzzle itself contains only the `🎯️targets/⚛️5d-react/🟦️.tsx` compose/flatten implementation and its thin package re-export; it has no production React window component to audit. Window UI is owned by native plugin scene emission and generic framework renderer hosts.

Checked 324 literal relative references across Puzzle and its development integration. Generated TypeScript `.js` imports are intentional extension substitution; example DSL imports carry the Vite `?raw` query. Neither category is evidence of a missing file. The canonical mutation test constructs its fixture URL relative to an explicit canonical root, so a text-only import path scan initially over-reported it; its URL base must be retained when checking resolution.

All four story call sites consistently pass `node` and `onAction` to the corresponding generic `Board2dHost`, `World3dHost`, and `GraphTimelineHost`. No additional concrete production import or prop-call defect was established from these reads. No compiler or browser process was launched, respecting the constrained build capacity. This does not claim type-check completeness.

## Follow-up: Browser Selectors and Interaction Assumptions

Confirmed missing interaction dispatch: the 2d browser cases press Delete and Ctrl/Cmd+A against a standalone Board2dHost. The story reducer has matching handlers, but the generic host keyboard hook only registers Escape and Tab; no event routes these two keys to the story reducer. Story-local focused keyboard dispatch is appropriate. The native canvas pointerdown handler stops propagation, so a wrapper that focuses on pointerdown must use capture. The select-all browser case currently only moves the mouse; it must establish focus for a focused keyboard contract.

Confirmed missing timeline selector: the spatial browser case looks up `data-testid=graph-timeline-table`. GraphTimelineHost renders HistoryTable with a generated DOM `id`, and HistoryTable renders that id without any data-testid. Scope the browser lookup to `.semio-graph-timeline-host` and its actual row structure instead.

Verified `.semio-board-2d-host canvas` exists. Debug fields and authored two-node/one-edge counts match the tests. World-to-client coordinates match the authored camera (140,60,zoom=1), with alpha at (0,0) and beta at (280,120). Brush hovering alpha center is supported: brush_slot_pointer_hit_distance permits the sole eligible free handle via node-body hit; alpha v0 has an edge and v1 is free. Alt-leave is explicitly handled by native pointer_leave_screen and brush_finish_slot.

Readiness gap: the debug text and canvas mount synchronously, while the generic host asynchronously creates its session and attaches the canvas. The current helper only checks visibility before input. Awaiting host snapshotParsed status and node vitals would establish the actual session fixture is ready; a visible blank canvas alone cannot establish that.

## Independent Review of Keyboard Repair

Reviewed the integration agent keyboard changes after editing: canvas pointerdown capture focuses the containing tab-indexed story pane; the pane handles keys locally, excludes editable targets, composition and modified deletion, and skips noninteractive stories. This correctly accounts for native canvas stopPropagation. The select-all browser case now clicks before issuing its shortcut. Node deletion now resolves endpoint node IDs before filtering incident edges. No actionable defect found in this focused read-only review. Browser execution remains with the parent.

## Follow-up: Playwright Listing Hang and Discovery Scope

Installed implementation confirms full-tree traversal before filtering. `node_modules/playwright/lib/runner/index.js:2234` calls cachedCollectFiles(testDir, respectGitIgnore), then applies extension/testMatch/testIgnore filtering at lines 2235–2244. collectFiles recursively visits directories at lines 2279–2314 and only unconditionally skips node_modules. Therefore explicit testMatch and testIgnore do not constrain directory traversal.

`node_modules/playwright/lib/common/index.js:612` computes respectGitIgnore with fallback `!projectConfig.testDir && !config.testDir`. The Storybook config explicitly sets workspaceRoot, so git ignores are disabled by default. This exposes .cache, repository cache images, .venv, build outputs, ticket generated output and .git to enumeration. Setting respectGitIgnore true is the smallest mitigation and preserves authored hidden .storybook tests, but .git itself is a real directory and root .gitignore does not ignore it, so this is not a strictly bounded solution.

Recommended bounded declaration: use a single authored browser collector under `.storybook/🧪️tests`, importing the registered suites with static imports; set testDir to that small collector folder and match only its entry. This preserves the existing single chromium project and shared browser options while avoiding any repository-wide discovery. A project per exact suite directory also bounds traversal but changes project naming/filter contracts. Do not rely on testIgnore to prune caches: it runs after collection.

Checked all 13 registered file paths. One independent stale registration exists: `✏️s/🔌️plugins/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts` does not exist; the actual authored file is `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts`. Preserve that intended coverage using its actual path when centralizing the collector.

No source edits or test process launches performed during this investigation.

## Follow-up: Playwright Runtime Singleton Port

The bounded collector imports 13 authored browser suites. All 13 directly import runtime test and expect from @playwright/test. Their only external type requirements are Page and Locator. The browser config additionally imports runtime defineConfig and devices. An owned port under .storybook/🧪️tests should explicitly reexport those four runtime symbols and two types from its single root-resolved @playwright/test import. Route the config and every suite below to that exact source module via relative paths, avoiding workspace package resolution selecting another generation.

- `.storybook/🧪️tests/🧪️catalog-browser/🟦️.ts`
- `.storybook/🧪️tests/🧪️visual-layout/🟦️.ts`
- `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎞️storybook-deck/🟦️.ts`
- `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-no-wasm/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-wasm/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts`
- `✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎨️storybook/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts`

A relative source import walk visited 18 modules including these suites and their helpers. No helper additionally imported Playwright; there was no alternate playwright runtime import among these modules. No node_modules or dependency generation changes are required for this source-level singleton repair.

## Follow-up: Legitimate Root Storybook Capability Birth

Reviewed rootTestBuildArgumentsV1/rootTestFreshCapabilitiesV1 and the bootstrap capability factory. Current schema only recognizes exact declared cache infrastructure requests and selected schema routes. Standalone root Storybook therefore receives workspaceRoot alone, explaining missing native caller resources at the owned finite store boundary.

Recommended schema-first extension: declare closed route prefixes for test storybook (bounded caller tail accepted by the existing test runner), build storybook, and dev storybook-static. Keep help requests outside resource issuance. Preserve original argv exactly in the returned selection rather than fabricating an unrelated run request. Unknown dev/test routes, incomplete prefixes and extra operands on exact routes should remain workspace-only.

createBuildBootstrapCapabilitiesV1 accepts root command argv: its unknown native operation check applies only when original arguments start with repo. It creates the native policy grant from canonical architecture environment, exact arguments/selection/launches, build mode, finite artifact directory, child transport and offline network declaration, then expands owned descendants. Passing real Storybook argv at legitimate initial root birth is consistent with this existing mechanism.

Extend the existing language-neutral fixture plus independent SQLite selection oracle and birth spies with fresh positive, help/unknown/incomplete negative, and received positive/negative Storybook vectors. Assert providers precede bootstrap issuance, bootstrap receives unchanged argv, workspace-only routes call neither port, and received Originals preserve identity without invoking freshCapabilities.

Also reviewed the new root dev time-travel delegator: repository provider designation is issued through the owned provider interface, and the existing this.invocation is passed intact into the harness which calls checkScriptInvocation. Relative imports resolve existing owners. No separate invocation or envelope is manufactured by that route.

## Independent Review of Storybook Birth Repair

Read the patched Protocol, classifier, neutral fixtures and SQLite oracle. No actionable defect found in the requested routes: test storybook accepts its caller tail; build storybook and dev storybook-static require exactly two operands; explicit -h/--help anywhere disables Storybook native issuance. Fresh issuance still designates providers before forwarding unchanged argv to the bootstrap capability factory. The received Original path in runWorkspaceScriptMain returns through receiveScriptProcessInvocation before evaluating freshCapabilities; identity/no-calls assertions remain present. The independent SQLite oracle expresses these route conditions separately rather than calling the classifier.

Read-only process observation during the actual parent run showed the owned root build child had launched the installed Storybook dispatcher with build -c .storybook and a temporary output path under the finite .cache/b/p/m/storybook/prototype store. The silent precompiler stage had progressed into the real compiler. No helper capability patch or process mutation was performed.

## Follow-up: Puzzle Story WASM Delivery

All three real-fixture stories import @semio-tech/puzzle-wasm: 2d-fixtures requires puzzle2dParseDslJson, 3d-world requires puzzle3dParseDslJson, and 5d-timeline requires puzzle5dParseDslJson; each also initializes the default wasm-bindgen export. The declared owner is `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust`, produced by `@semio-tech/puzzle-plugin:wasm`. Its actual generated declaration currently exports all three parsers plus BoardSession and default init. The Rust parser exports remain present in their dimension-owned bridge leaves. No parser export rename was established.

The current Storybook puzzle/2d, puzzle/3d and puzzle/5d scopes have no Puzzle wasm alias. The reusable canonical ownership declaration already exists in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json`: packageName @semio-tech/puzzle-wasm, sourceOwner as above, wasmBaseName semio_puzzle, entrypoint generated. Its frameworkOsEngineDeliveriesV1 port projects managed artifact requests and exact generated JS entries from productsRoot and mode; play/distribution facets include Puzzle, renderer excludes it. Storybook should bind the actual Puzzle generated delivery through this owner mechanism, retaining receipt/prerequisite validation. Externalizing the unresolved import or replacing parsers with mocks would hide the missing delivery rather than repair it.

Communicated exact owner, target, exports and delivery interface to the integration agent. No source edits performed.

## Independent Review of Managed Storybook WASM Wiring

Reviewed actual .storybook/main.ts patch: frameworkOsEngineDeliveriesV1 receives the canonical repository products partition and build mode; puzzle scope activation chooses play facet, including every puzzle child scope. managedWasmPackagesVitePluginV1 receives exact declared package name, managed root, generated entry and finite files. The resolver validates closed data, distinct package identities, canonical absolute paths, path/query limits and allowlisted subpaths. There is no checkout pkg alias, parser mock, fallback or unresolved-import externalization.

Compared all three story module types with the actual generated declarations: default initialization returns a Promise and may be invoked without arguments, and every dimension parser takes a string and returns JSON text. All counterpart story expectations match. No actionable defect found in this focused source review. Integration agent is adding its independent regression; runtime build remains owned by the parent.

## Follow-up: Standalone 3D/5D Required Contexts

Audited World3dHost and GraphTimelineHost for an analogous missing external session factory after discovery of the 2d BoardSessionFactoryContext omission. No analogous mandatory provider was found. World3dHost owns its real R3F WorldCanvas and mounts WorldOrbitViewSnapGateProvider and WorldLodBridge itself; the required LOD context is supplied internally. World instance chrome context is also provided by its own WorldInstancesLayer. World3dWindowViewStoreContext uses optional read/write, shell scope is explicitly optional, and window identity/title/icon callbacks have null-safe usage. GraphTimelineHost directly renders HistoryTable with an optional window identity and has no WASM/session constructor requirement.

Current standalone 3d/5d scenes supply assembled camera, mesh, instance, selection and vortex JSON lanes consumed by the host; 5d supplies the timeline columns lane. Their DSL loaders still require the true managed Puzzle WASM module and actual exported parsers. No source parser replacement, render mock or externalization is needed. This source audit establishes no additional required-provider repair; runtime canvas readiness and actual browser rendering remain with the parent run.

## Follow-up: Managed WASM Specifiers and Board Provider Review

Current preview editor loader now uses the canonical bare @semio-tech/framework-editor-rs entry. No remaining /pkg specifier for the four managed engine package identities was found across authored TS/TSX. The closed managed delivery lists generated filenames relative to its managed root, so old pkg/framework_editor.js must be corrected at the caller; widening the allowlist would recreate obsolete packaging assumptions.

Four other preview loader package names remain stale: framework-surface-node-graph-rs, framework-surface-paint-rs, framework-surface-tiled-map-rs and framework-surface-terrain-rs. None has an authored framework package.json. The actual modern renderer binding owner is `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🪝️host-ports/🕸️wasm-bindings/🟦️.ts`: surface uses bare framework-surface-rs, editor bare framework-editor-rs, nodeGraph framework-os-node-graph-rs/framework_os_node_graph.js, and flow the core plus browser module. These four old names are outside the managed resolver package roster, so they do not explain a closed-file-list rejection for the editor package. Communicated canonical entries to the integration agent.

Independently reviewed both 2d story provider repairs: public WasmSessionLoader SDK resolves the exact Puzzle2d editor identity against actual PUZZLE_BOARD_SESSION_FACTORIES. Its constructor initializes the real module, installs page runtime assets and returns an actual BoardSession. useMemo stabilizes the isolated scoped factory. No mock, missing mandatory counterpart context or additional actionable issue found.

## Independent Review of Async Board Attachment Ownership

Reviewed the actual lifecycle repair and two attachment fixtures. Session raw/handle refs, peer registration and ResizeObserver are now published after attach_canvas resolves, so scene sync, pointer dispatch and resize cannot borrow the session while async attachment owns it. Cleanup marks disposed and defers release while boot is pending. owner is nulled before free, preventing double session free if disposed attachment rejection runs both the finally release and fail path. A factory resolving after disposal frees its returned session directly without starting attachment. Both async creation and boot chains terminate with catch(fail), so attachment rejection is observed.

No concrete disposal ownership defect was found in this read-only review. The current two regression vectors cover unmount during successful pending attachment, including scene update/resize. They do not yet cover rejection during attachment or normal ready publication/unmount. Recommended live/disposed rejection and normal ready unmount vectors to the integration agent for direct free-once/error-path evidence.

Corrected an investigated hypothesis: extra browser data attributes do not prove stale source because publishVitals sets transform, interaction and handle-position attributes imperatively. Keep the actual snapshot readiness assertion; weakening it would conceal a failed session boot.

## Follow-up: Attach Promise Return Propagation

Audited the current authored factory, SDK facade, session proxy, generated source package JS and Rust bridge. No dropped Promise return was found. resolveAppSurfaceSessionFactory preserves registration.create directly; the public WasmSessionLoader facade explicitly reexports the source; createPuzzleBoardSession returns the actual BoardSession; frameDemandingSessionV1 returns method.apply(target,args). Generated semio_puzzle.js attach_canvas returns wasm.boardsession_attach_canvas result and its declaration says Promise. The current Rust bridge is a synchronous pub fn returning Promise from future_to_promise, clones its Rc state and only takes its mutable state borrow after awaiting GPU surface creation.

The name resolveBoardSessionFactory does not exist in authored TS/TSX/JS searched; the actual current name is resolveAppSurfaceSessionFactory. Any browser stack using the former name needs exact loaded module/generation comparison. Current source evidence does not justify changing wrappers to repair a discarded return. Communicated this distinction to parent and integration agent. No files beyond this audit changed.

## Follow-up: Scope-Owned WASM Prerequisite Audit

For the authored Puzzle scopes, every dimension requires @semio-tech/puzzle-wasm: each real-fixture story parses DSL with its dimension export, and 2d additionally creates actual BoardSession. Shared renderer host source consumes constructors through installed ports rather than importing all native generated packages, so this alone does not justify adding surface/editor/graph prerequisites to Puzzle.

The infinite scope stories exercise mock WasmCanvas, real R3F WorldCanvas and ReferenceMedia without native engine modules. No managed native package requirement was found there. UI-only likewise needs no host native engine package.

Framework/hosts and framework scopes include actual TextEditor, NodeGraph, Flow, Paint2d, TiledMap and Terrain stories. Their modern native requirements are framework-editor-rs, framework-surface-rs (shared paint/map/terrain), and framework-os-node-graph-rs (source facade plus managed generated module). Flow is separately delivered by its existing flowBrowserDeliveryV1 contract: build selects the published browser package, serve selects explicit source browser/host entries. Do not fold Flow into an unrelated engine roster.

Preview currently activates the entire framework wasm loader registry and host binding module for framework scopes. Build-time requirements must account for its import graph even if a particular runtime case does not invoke a loader. framework/os additionally hosts actual plugin-module deliveries, whose producer declarations belong to the existing plugin artifact owner rather than the generic shared engine requirement field.

Sent scope/package findings and import-graph caveats directly to puzzle_birth_launch. No edits or builds performed.

## Confirmed First-Fault: Repeated Mutable RefCell Borrow in Snapshot Load

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌉️bridge/🦀️.rs:231` calls load_board_snapshot_json with four separate self.state.borrow_mut() expressions for host.scene, host.focus, host.style and host.outbox in the same call. The first RefMut temporary remains alive through the statement; evaluating the second borrow therefore panics before snapshot decoding begins. This is a deterministic current source defect in the initial loadBoardSnapshotJson path, independent of async attach timing.

Repair requires one state borrow, then disjoint mutable references from its host. The pattern already used elsewhere in that bridge (`match &mut self.state.borrow_mut().host { __host => ... }`) also expresses one owner. A search found this is the sole same-line repeated RefCell mutable borrow in Puzzle Rust source.

Later wasm-bindgen recursive-use errors in defersDescriptorSyncFromJs/free are consistent with being secondary errors after a first WASM trap while the mutable exported-class borrow was active. This secondary explanation is an inference; the repeated RefCell borrow itself is directly established from source. Sent exact source and repair to parent and integration agent immediately.

The renderFrame export takes one mutable state borrow and builds/renders the vector scene. The explicit JS progress callback in this bridge belongs to standalone board_redraw_layout_snapshot_json rather than BoardSession state. No additional export holding a BoardSession state borrow across an authored JS callback was established in this inspection.

## Follow-up: Compiler Source Closure Responsibility Boundaries

The authorizer captures literal import/reexport/require sites and declared computed roots; a runtime branch or conversion from static import to literal dynamic import does not remove a required source edge. Closure reductions must change actual dependency responsibility, retaining the source admission guarantees.

Concrete broad edges from the Puzzle hub package script: it imports the public repo-lib/cargo barrel only for runRepositoryCargoTests, but that barrel reexports native artifact builders, exact Cargo laws, workspace command control, preparation, profile and compiler execution. The same dispatcher imports CanonicalArchitectureScript and FixturesScript from Rust test command code, whose canonical handler imports broad repo-lib/testing and repo-lib/laws and Puzzle test modules. Those are genuine test/fixture program dependencies, not dependencies of producing the browser WASM package. A separately declared WASM task entry, with its actual script owner metadata, can exclude test dispatcher responsibilities; a direct owned admission leaf also avoids barrel-only edges while keeping the used function admitted.

The WASM build policy imports the large repository process module solely for resolveWorkspaceBin. That process module owns native Cargo source/compilation graph admission, workspace execution, compiler storage and tool routing. A focused workspace binary lookup owner is a genuine responsibility separation; the WASM generator can consume lookup without depending on unrelated execution APIs. Sent these concrete seams to parent and birth agent. No bound increase or source exclusion workaround is recommended.

## Independent Review of Snapshot and Render-Failure Repairs

Actual Rust snapshot fix takes one RefMut state and derives disjoint host field references, removing the identified repeated-borrow panic. The actual browser first case still awaits successful native snapshot parsing and counts before comparing the portable expected fixture. Actual renderFrame scheduler now sends its first exception to fail/error boundary instead of swallowing it, and primary plus release failures are retained in AggregateError. The expanded seven lifecycle fixture paths cover successful publication, early close, attachment rejection, rendering fault and combined attachment/release failure. No actionable defect found in the reviewed edits. Runtime rebuilding and browser confirmation remain with the parent.

## Final Bounded Browser Acceptance Audit

Concrete remaining false positive in the noninteractive 2d test: it clicks canvas center, which the authored camera maps to world (140,60). Both nodes are elsewhere, at (0,0) radius44 and (280,120) radius40. That click leaves selection empty even when interaction is enabled, so the test does not establish its claimed refusal. Use the existing worldToClientPoint helper to click alpha, whose selection is already positively established by the interactive test.

The lasso boot case checks only empty selection and two nodes. Changing its initial method to rectangle leaves those assertions unchanged; a lasso-specific gesture or method assertion is needed to establish the named behavior. The 2d helper also checks accumulated errors before returning to each interaction case, so faults emitted by later pointer/key actions are never asserted. Return a final error assertion callback, as the spatial helper now does, and invoke it after the actual interaction assertions.

Spatial 3d counts come from the story fixture/debug state and canvas visibility alone does not establish that those objects reached the world scene. The 5d case changes only its revealCount debug assertion. Narrow strengthening is to compare the actual host data-instances-json IDs/count before rendering checks and after timeline scrubbing; those observations establish the current host scene contains the intended projection. No GPU pixel claim should be inferred from visibility alone.

## Native Hub Test Acceptance Semantics

The hub package has seven explicit surface tests: viewer-never-mutates for three dimensions, editor/viewer dialect equality for three dimensions, and one codec parity test covering all three artifact kinds. The viewer helper invokes only the default command and checks empty artifact/draft emissions when the handler returns Ok; a handler Err is accepted by that law. Dialect tests compare constants. Codec test compares plugin runtime/native pack schema hashes, genesis decode/mirror op logs, and nonempty DSL, then asserts exactly three compared schemas. These are useful composition and codec laws, but do not execute native editor mutation, persistence/reload or browser dispatch flows. Actual time-travel/editor acceptance is still necessary for those criteria. Sent exact scope to parent; no builds or source edits performed.

## Acceptance Assertion Repairs

Root strengthened the 2d lasso journey with a portable polygon whose bounding rectangle includes beta while the polygon only includes alpha; a rectangle fallback cannot pass. The noninteractive story now clicks the known alpha position and permits an actual animation frame to settle. All 2d cases assess captured errors after their interactions. Spatial cases require the renderer host instance inventory, including the reduction from 180 to one after timeline scrubbing, alongside visible canvas and parsed fixture counts.

## Compliance and Invocation-Leaf Consistency Audit

Checked 87 currently relevant changed/new source, fixture, report and ticket-output files for absolute path length in Unicode characters and UTF-16 code units; none exceeded 256. Also checked all new non-ticket paths reported by Git, with no UTF-16 overlength paths found. No AGENTS.md changes or package.json dependency changes were reported. The explicit Playwright port reexports existing installed test APIs; it is a singleton test boundary, not a runtime dependency addition or compatibility implementation.

New Storybook discovery/MDX/workspace-export cases are explicitly included in the Storybook unit runner; Puzzle story-index test is explicitly included in puzzle-js test config; renderer attachment regression is explicitly included by elementSuite in the owning React renderer test config. Browser fixtures are read by the owned registered Playwright suites. Ticket scripts/reports remain under the ticket.

Actionable consistency issue found: dashboard-control test extracts runOwnedBuildInvocationV1 from its new invocation leaf, but its wrapperPorts still exposes executeCapturedBuildSourceV1. The actual leaf imports sealed/execution and calls executeImplementationV1; the local test load dispatcher lacks that exact import and would reject with Undeclared dashboard help port before the expected captures/events. Other inspected phase/portable/bootstrap tests now locate the function at its current leaf. Sent exact mismatch to parent for correction; no test run or source edit performed by this audit.

## Independent Review of Native Replay Test Mounting

Reviewed actual 3d/5d native changes. Existing replay preview, cancellation, blocking outcome, overwrite, supersession and TransactionRef laws remain intact; new pack and DSL codec roundtrip checks strengthen nonblocked replay assertions against the same authored expected snapshot. Fixture content is unchanged. The 3d genesis producer argument updates the actual current API, and the 5d mutable loop permits the existing explicitly opt-in fixture regeneration branch.

Found an actionable newly activated 5d fixture reference error: fixture_text resolves relative CARGO_MANIFEST_DIR, so the previous six-parent path does not resolve from the newly mounting 5d artifact crate. Integration corrected the reference to the existing authored any-subset fixture and verified it exists.

Found a second 3d reference mismatch: its shared selection-time-travel module is mounted by both the mutations crate and the host-owned crate. The current four-parent CORPUS reference resolves correctly only from the mutations manifest; from host-owned/packages/rust it targets nonexistent 3d/modules/tests data. Both mount locations and the exact resolution evidence were sent to integration for a canonical fixture-owner reference independent of the compiling crate. This concerns test discoverability/fixture loading, not weakened laws.

Global same-line repeated-borrow search across Puzzle and hub Rust now finds no remaining borrow_mut/borrow_mut overlap. A broader statement scanner over wasm bridges produced only cross-function false positives, with no additional confirmed simultaneous-borrow site.

## Native Fixture-Owner Repair Follow-through

Reviewed final repository-owned repo_text references in 3d/5d. They name the same existing authored fixture files independent of the compiling crate manifest. The 3d helper additions are dev-dependencies on the existing repository fixture-reader crate, with correct relative owner paths; they do not create runtime external dependencies. The redundant newly proposed 5d root mount was removed after discovering its existing mirror-test owner, preserving intended ownership. All existing replay laws and fixture contents remain intact; new pack/DSL checks add assertions.

One follow-up noted to integration: 3d CORPUS changed from LazyLock<String> to LazyLock<&str>, while its parse call retained CORPUS.as_str(). Direct *CORPUS avoids resolving the old String-only method against str and keeps the current stable string API surface. Native compiler confirmation remains with the owner agent.

## Browser Provisioning and Cross-Platform Route Audit

Confirmed missing zero-touch wiring: root TestScript.runStorybookPlaywright sets PLAYWRIGHT_BROWSERS_PATH and invokes the browser runner, but does not provision a missing matching browser. Time-travel runProbe calls ensureParityPlaywrightBrowsersPath, whose body only assigns that same cache environment variable, then launches Chromium. Neither cache-path helper installs or verifies browser executables. The generic loadPlaywrightV1 adapter also only selects the cache and loads the module.

The existing public owner command is repo tools setup chromium, implemented by runToolchainsSetupV1. Its Chromium revision is read from the installed matching playwright-core browsers.json through Playwright package resolution. The row delegates to the owned native-dependencies script sync browsers, whose browser system interface installs Chromium with that installed CLI and handles Linux prerequisites through declared host/system policy. Browser readiness/provisioning should happen through that owner under the existing actual Original control before expensive build/server startup and launch. A child build request requires an exact launch declaration before fresh issuance; direct owner invocation must retain the original native network/offline authority rather than manufacture online permission.

Also found time-travel hardcodes --use-angle=metal plus Vulkan flags for every host, unlike existing parity and Storybook launch policy using portable SwiftShader flags. Use the common owned launch policy for native Windows/Linux/macOS rather than a macOS backend literal.

Existing tool verification hook only checks the chromium-revision directory, although default headless launches may require a distinct chromium_headless_shell executable; a partial browser install can therefore appear provisioned. Actual launch readiness should use the matching installed Playwright executable contract. Sent exact owner/control findings directly to integration agent; no browser installs/builds/source edits were performed by the audit.

## Independent Review of Browser Preparation Repair

Reviewed the actual schema-owned launch policy, executable probe, preparation owner and call sites. The probe asks the installed matching Playwright registry for both Chromium and headless-shell platform-specific executable paths and checks executable access; it no longer treats the browser directory as readiness. Registry resolution correctly takes dirname of the resolved sibling package.json. No hardcoded Chrome binary or user-cache fallback was introduced.

prepareRepositoryBrowserV1 checks the real ScriptInvocation and exact repository identity, creates an issued control, and calls the existing runToolchainsSetupV1 owner with the Original network.offline authority. It verifies exactly one successful Chromium report. Root Storybook and time-travel call it before expensive build/server activation. Storybook, parity and time-travel now share the software WebGPU launch options. No actionable code defect found in this source review.

The new pure fixtures exercise launch policy and executable readiness under Node VM rather than invoking the installer. Recommended additional narrow preparation ownership tests for unchanged Original/control/offline propagation and failed/root-mismatch refusal, and explicit owned inventory registration of the new browser test entry. Integration owns those verification details; this audit performed no installation, compiler or browser launch.


## Final Spatial Pick Interaction Audit

Positive object IDs agree with the real renderer: World3dHost instance-layers maps the original instances array with its numeric index (line 494), passes index unchanged into WorldInstanceNode (line 510), and active-utility handleInstancePointerDown dispatches worldPick id:index. Both authored stories build instances in exactly fixture object/part order, so their positive numeric index lookup is valid. The 5d revealed slice preserves that order. No mesh-ID/string-ID mismatch was found.

Concrete defect: 3d-world story worldPick lines 158–163 and 5d-timeline lines 165–170 use Number(args?.id). Real handle-pointer-up line 112 dispatches id:null for an empty pick; active-utility also dispatches null for disabled instances. Number(null) becomes zero, so an empty click selects the first authored object instead of clearing the selection. Sent directly to integration owner; positive pick followed by empty clear is necessary acceptance coverage.

Both concrete-forest authored DSL documents have first ID seed-left-001 at origin [0,0,0]. The 3d story includes visible locked ref-masterarbeit at [7,0,0.01], width 50; thus an arbitrary canvas center/corner cannot be assumed to target object/empty world before checking the live frame. DOM-only targeting can read data-instances-json and data-viewport-camera-json, and use the actual canvas rectangle plus CPU look-at/projection math. The DOM camera includes position/target/up/zoom/fov/projection (scene-schema lines 414–424); renderer defaults up to [0,0,1] and perspective fov to 45. If the live target is the known object origin, exact canvas center is an especially simple valid target. This uses actual pointer input and public DOM diagnostics, without GPU or React object introspection.

No build or browser execution performed in this read-only audit.

## Actual Spatial Pointer Acceptance

Root extended the existing 3d Concrete Forest journey to pick the authored seed-left-001 object and clear it on background input. Its target comes from the renderer instance position and current viewport-camera DOM values, projected by the existing independent Three camera math. The 5d scrub journey also asserts background input preserves the empty selection instead of coercing null to index zero. Portable world-session expectations are stored beside the other browser corpora. Actual browser execution remains pending fresh WASM and scope build.


### Spatial Pick Follow-up Review

Root browser helper clickFirstWorldInstance reconstructs the actual perspective camera correctly: position, fallback Z-up, lookAt target, fov, zoom, projection update and world-matrix update precede projecting the DOM instance position. Current instance-node fallback lines 461–470 is a real unit box centered at this exact origin, with renderer raycast and pointerdown dispatch. Positive semantic selection assertion makes reference-plane interception an actual failure rather than false success. No source defect in the projection helper identified.

Canonical empty-selection oracle is the current generic app port, not the removed worldPick app command: plugin/instance/render/staging/interaction Rust lines 309–323 clears empty targets only for Replace; wire/interaction next_selection lines 620–623 otherwise returns current unchanged before Range handling. Therefore a null Range pick must preserve selection, unlike the first shared story helper draft which clears Replace or Range. Integration notified to correct helper and neutral fixture. Existing actual Puzzle3d world-selection-laws Rust lines 77–90 explicitly verifies empty-target Replace clears. Strict number/safe-integer/nonnegative index validation is correct and introduces no string coercion compatibility.

Integration follow-through independently read: null Range now preserves current, only null Replace clears; neutral fixture Range expected selection is beta. Catalog-integrity consumes all sixteen authored vectors through the actual shared helper. Numeric string, boolean, empty string, negative, fractional, out-of-range and omitted IDs preserve current. No remaining actionable issue in this bounded helper review.

Final browser adapter follow-through: installed Playwright1.64.1 coreBundle.js exports registry namespace; both Chromium/headless-shell executable records expose _validateHostRequirements(sdkLanguage), matching the new exact registry.registry probe. executablePathOrDie plus X_OK and awaited host dependency validation fail the probe on missing actual binaries or Linux libraries. Real Original/root/offline/control and repository cache ownership remain in preparation. No remaining actionable defect found in this read-only check; no queued execution.


## Bounded 2D Brush Acceptance Audit

The current BrushPlacement fixture alpha node is centered at(0,0), radius44; alpha:v0 angle0 is occupied by link-1, alpha:v1 angleπ/2 is its only free handle. Its direct circle-rim position is(0,44), from graph drawing handle_position_on_circle. The live anchor may instead be an indirect ring at Overview/Compact/Normal LOD: normal scene grid-snap-and-brush-candidates Rust lines121–127 deliberately resolves that ring before direct geometry. Do not assume direct rim equals the painted/hit anchor across LODs.

Current browser hover at alpha center is nevertheless valid, not an obsolete API assumption. normal gestures brush-slot-preview Rust lines24–27 accepts any point inside a node body when that node has a sole eligible free indirect handle; nearest-source lines35–37 excludes linked handles. scene-and-style-state-impls lines62–74 establishes sole-free eligibility. Therefore alpha center targets alpha:v1 and is stable across LODs; tests should describe this as sole-free-node body targeting instead of literally hovering the handle. Existing native law brush_slot_accepts_pointer_on_node_body_at_overview_lod exercises that behavior.

Alt-leave is current production semantics. Renderer Board2dHost board-pointer-and-keyboard lines142–150 forwards PointerEvent.altKey into pointerLeaveScreen and drains resulting events. normal host cancel-and-transform-gumball lines12–15 updates the Alt state then finishes the active brush slot. gestures brush-slot-preview lines151–155 commits preview only when brush_alt_pressed, otherwise only clears. The browser holds Alt while moving outside then releases, matching native brush-slot-laws lines61–69 and the without-Alt no-commit law. No concrete target/chord mismatch found; no source edits/builds performed. The no-commit browser case still only checks unchanged counts; without a preview/source diagnostic it cannot prove that a slot opened, although the adjacent commit case proves the same target can actually place.


## Bounded Keyboard Focus Accessibility Audit

Concrete introduced issue: 2d-board story line353 makes a plain div keyboard-focusable and moves canvas pointer focus onto it, but supplies no accessible role/name. The Board2dHost child canvas also has no role or accessible name, and no lifecycle code supplies one. The new focus stop is consequently anonymous. Sent integration owner a narrow fix: localized named group/region using the current UI translation/locale owner, with existing scoped key behavior preserved. Avoid hardcoded default-language label.

Editable input/textarea/select/role=textbox/contenteditable, composition, modifiers and default-prevented guards remain present. No other introduced keyboard editing/focus defect found. Existing Storybook withLocale sets selected UI locale and has an EN fallback/default before this change; that preexisting configuration is outside this introduced-defect audit. No builds or source edits performed.

## Localized Keyboard Focus Acceptance

The focused board group now has the existing translated diagram name. Root runs its real keyboard/text-input journey in explicit English and German Storybook globals and checks the actual accessible group name and focus before commands. Portable labels are in the keyboard corpus. Discovery minimum is now143tests, with20Puzzle/Infinite scope cases (17Puzzle). Actual browser execution is still pending fresh managed WASM.


### Localized Keyboard Acceptance and Empty World Target Review

Normal labels in the neutral keyboard corpus exactly match current UI content-host bundles: Node graph (EN) and Knotengraph (DE). Top-level en/de iteration registers ordinary supported Playwright callbacks. Optional helper globals=locale:en/de targets the preview declared locale globals and withLocale decorator; other existing helper callsites omit the optional parameter.

Concrete correction to earlier audit recommendation: raw useTranslation().t(ui.diagram.label) returns the structured label object, because owned i18n translate preserves objects. React aria-label would stringify that as [object Object]. Existing useLabel(ui.diagram.label), exported by the same UI port, is the proper current locale/tier string resolver. Sent integration/root; localized named-group acceptance correctly catches this before keyboard checks.

WorldOrbitProjectionSwitchPane initializes bottom-right, folded, inside a pointer-events-none fallback container; it does not default top-left. Bottom-left inset plus document.elementFromPoint(x,y)===actual canvas is a valid DOM-only chrome-avoidance guard; no semantic state injection needed. Locked reference planes disable mesh raycast through worldEntitySelectable gating (reference item lines261/323). Keep actual selection-clear assertion, which confirms the chosen world point misses selectable content.

Final localized keyboard review: current wrapper uses useLabel(ui.diagram.label) string resolver, role group, existing scoped focus/key behavior. Preview default UI driver explicitly labelTier normal, matching neutral EN Node graph and DE Knotengraph expectations. Each locale test asserts real named group visible/focused, native input editing preserved, then board select-all/delete; optional locale URL globals and ordinary callback registration remain valid. No further concrete defect found. No browser/test request queued.
