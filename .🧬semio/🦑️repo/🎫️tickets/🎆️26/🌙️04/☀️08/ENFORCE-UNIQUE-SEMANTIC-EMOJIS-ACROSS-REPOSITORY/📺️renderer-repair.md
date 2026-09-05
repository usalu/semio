# Renderer Emoji Repair

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`. Manual naming repairs and browser/generator verification completed. The sibling OS agent owns everything else in OS; shared taxonomy/root changes below were explicitly delegated by the parent. Applicable root, products, and OS instructions were read; no nested renderer `AGENTS.md` exists.

The baseline covers 112 non-reserved files and 109 directories, including generated browser JavaScript. Dependency `node_modules`, literal tool-reserved files, and `.DS_Store` are not renamed. An old misplaced `.🦑️repo` ticket mount under `🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` contains historical Cargo output; it was reported to the parent and is not silently treated as authored renderer code or removed during other agents' builds.

## Handpicked Decisions

All choices are explicit after inspecting purpose and siblings. Unique existing format leaves remain meaningful and unchanged.

Element paths below are relative to `🧑️‍🎨️engine/🧱️elements`.

| Original | Handpicked | Meaning |
| --- | --- | --- |
| `AgentBridge` | `🔗️AgentBridge` | Agent gateway connection |
| `AgentPresence` | `🚦️AgentPresence` | Connected, working, idle, and disconnected status |
| `Canvas2dHost` | `📐️Canvas2dHost` | Two-dimensional geometry canvas |
| `IconRenderHost` | `🖼️IconRenderHost` | Rendered icon shot preview |
| `InkCanvasHost` | `🖋️InkCanvasHost` | Freeform ink and note strokes |
| `NodeGraph` | `🕸️NodeGraph` | Connected node graph |
| `Paint2dHost` | `🖌️Paint2dHost` | Raster brush painting |
| `PluginRuntime` | `🔌️PluginRuntime` | Plugin actor-channel runtime |
| `ShellHelpers` | `🛠️ShellHelpers` | Shared shell utility functions |
| `ShellHost` | `🏛️ShellHost` | Shell orchestration and window host |
| `ShellSync` | `🔄️ShellSync` | Document synchronization attachment |
| `Table` | `📊️Table` | Tabular data rendering |
| `TiledMapHost` | `🧭️TiledMapHost` | Navigable tiled map viewport |
| `UtilityTree` | `🎛️UtilityTree` | Grouped utility ribbon controls |
| `World3dHost` | `🌐️World3dHost` | Three-dimensional world viewport |
| `🟦️Interpreter` | `🗣️Interpreter` | Semantic UI interpretation, not one implementation language |
| `ShellHelpers/🧪️fixtures` | `🛠️ShellHelpers/🧫️fixtures` | Samples distinct from the sibling executable component test |
| `ShellHelpers/🧪️fixtures/🔣️.schema.json` | `🛠️ShellHelpers/🧫️fixtures/🧬️.schema.json` | Shape distinct from sibling JSON sample |
| `ShellHelpers/🧪️fixtures/📂️open-artifact` | `🛠️ShellHelpers/🧫️fixtures/🚪️open-artifact` | Artifact-opening relay rather than a generic folder |
| `ShellHost/🧪️fixtures/🔣️.schema.json` | `🏛️ShellHost/🧪️fixtures/🧬️.schema.json` | Extension invocation shape |
| `NodeGraph/🧪️fixtures/🔣️.schema.json` | `🕸️NodeGraph/🧪️fixtures/🧬️.schema.json` | Pick-target shape |

Inside `🔌️PluginRuntime/🧪️fixtures`: `🔣️channel-close.json` → `🔒️channel-close.json` (closed channel), `🔣️channel-close.schema.json` → `🛡️channel-close.schema.json` (closure safety contract), `🔣️lifecycle-scheduler.json` → `⏱️lifecycle-scheduler.json` (scheduled lifetime traces), `🔣️lifecycle-scheduler.schema.json` → `📐️lifecycle-scheduler.schema.json` (trace shape), and `🔣️surface-refresh.json` → `🔄️surface-refresh.json` (refresh coalescing).

WGPU paths below are relative to `🧑️‍🎨️engine/🎯️targets/🧊️wgpu`. `🧵️frame-job` is retained for its thread-coordinating purpose.

| Original | Handpicked | Meaning |
| --- | --- | --- |
| `🧵️browser-boot` | `🚀️browser-boot` | Browser launch |
| `🧵️frame-worker` | `🎞️frame-worker` | Dedicated frame production |
| `🧵️browser-interactive-job-port` | `🔌️browser-interactive-job-port` | Interactive job protocol port |
| `🧵️browser-worker` | `🌐️browser-worker` | Browser-owned worker runtime |
| `🧵️interactive-job-registry` | `📇️interactive-job-registry` | Registered interactive jobs |
| `🧵️browser-frame-transport` | `🚚️browser-frame-transport` | Frame message transport |
| `🧊️renderer-boot` | `🎬️renderer-boot` | Starting the renderer |
| `🧪️tests/🟦️browser-frame-transport.ts` | `🧪️tests/📨️browser-frame-transport.ts` | Message transport checks |
| `🧪️tests/🟦️browser-interactive-job-port.ts` | `🧪️tests/🎮️browser-interactive-job-port.ts` | Interactive work protocol checks |
| `🧪️tests/🟦️package-integration.ts` | `🧪️tests/🧩️package-integration.ts` | Package integration |
| `📦️packages/🦀️rust/🟦️typescript/🟨️boot.js` | `📦️packages/🦀️rust/🟦️typescript/🚀️boot.js` | Generated browser launch bundle |
| `📦️packages/🦀️rust/🟦️typescript/🟨️frame-worker.js` | `📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` | Generated frame-worker bundle |

In the React package root `🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`: `🧪️index.test.ts` → `🔬️index.test.ts` (integration checks), `🧪️quick.test.ts` → `⚡️quick.test.ts` (bounded quick checks), and `🧪️opening.test.ts` → `🚪️opening.test.ts` (artifact opening relay). The `🧪️tests` directory owns the test configuration and remains distinct.

## Verification

All 41 explicit moves above are applied. All incoming TypeScript imports, renderer tests and configuration, WGPU source producers, direct generated bundles, and native source mounts were repaired precisely. No modifying Git operations, global replacement scripts, migration tooling, or old-name aliases were used.

The stacked `../🖥️🛸️ShellHost/...` imports in `🔌️PluginRuntime` now resolve to `../🏛️ShellHost/...`. The three non-renderer incoming references were coordinated with the OS agent and confirmed repaired. The renderer also consumes the OS agent's `🧵️backbone-worker.ts`, `🔗️directory-share-link`, `🎚️slider-overlay.json`, and `📐️.schema.json` names.

The native renderer contained 19 stale literal mounts left by the earlier package/layout corruption. Exact `#[path]` and `include_str!` references now select the existing renderer sibling owners and current component directories; seven element roots and the plugin registry mount had incorrect relative depths. A read-only resolution audit now finds all 49 literal Rust references present. This is path validation, not a claim of successful Rust compilation.

The parent authorized seven exact renderer references in the root script and two keys in `🔒️layering.json`; these now select the actual names. The WGPU generator's current `outputRoots` and current-layout `inputPatterns` were repaired and byte ordered. Historical hashed package source-preimage fixtures were preserved.

`Trunk.toml` remains literal. Running `trunk --offline --skip-version-check config show` in the WGPU Cargo package without `--config` loaded this exact configuration. Its schema registration is `renderer-trunk-config`, scoped to that single path; an unrelated `Trunk.toml` receives no exemption. The command required removing the inherited `NO_COLOR=1` from its environment because the installed Trunk parses that setting as a boolean. No environment files were changed.

### Completed checks

- React quick: 4 tests passed.
- React full long: 537 tests passed across 6 files before the final artifact-opening/infinite fixture-reference changes.
- Focused tutorial document, action semantics, and artifact opening checks: 4 passed, 533 skipped.
- WGPU browser protocols: 32 tests passed across 2 files.
- WGPU generator integration: 9 tests passed, including independent Node crypto/WebCrypto and JSON decoding oracles.
- Direct WGPU frame-worker generation and browser-worker freshness checks passed with `🎞️frame-worker.js` and `🚀️boot.js` as the only direct generated browser bundle names.
- All 224 internal TypeScript import references resolve; the source parse audit reported no syntax diagnostics.
- Central taxonomy loads successfully after generator-contract and Trunk changes.
- Scoped `git diff --check` passed.

### Canonical generator authority repair

The separate canonical generator initially failed because production depended on frozen source-layout snapshots. The parent explicitly authorized a new current WGPU catalog, preserving the historical evidence. `🪪️package-catalog.json` identifies the current package and four small adapter/registration artifacts; sibling `🧬️package-catalog.schema.json` owns its shape. Both basenames were selected individually and remain sibling-unique. The central contract binds the exact catalog path and SHA-256 digest. The browser profile now declares both current entry sources and their exact outputs: `🎞️frame-worker` and `🚀️browser-boot`; no emoji is inferred from the entry kind.

The catalog parser validates identity, boundaries, exact artifact roles, and literal adapter targets. The production generator verifies current Cargo/Node/Nx identities, reads the 66 explicitly reviewed compiler inputs, checks that all inputs were actually used, rejects undeclared imports, and hashes the input bytes again before returning to reject concurrent changes. Its 80 actual inputs are declared in `inputPatterns`. The historical package catalog, source digests, frozen bytes, and source-preimage verification functions were not weakened or rewritten.

Normalization's WGPU activation now verifies only the current no-follow catalog and current Cargo manifest. The obsolete WGPU projection activation and 32 old source-layout input declarations were removed from its live generator contract, along with the frozen-catalog input dependency. Other package projection authority is preserved.

The new language-neutral `🧪️tests/🔣️browser-entry-authority.json` supplies eight entry-identity cases, three digest-integrity cases, and five current-activation cases. The entry and activation regressions were observed failing before their implementation changes. Ajv, emoji-regex, WebCrypto, and both independent Bun/TypeScript compilers validate the matching outputs. The full generator integration suite now passes 12 tests.

Canonical `generate-wgpu` succeeded and changed exactly two browser bundles; all four declared adapter/registration files remained byte-identical. Canonical `check-wgpu` then passed: six exact artifacts, zero changed. Direct browser package generation and freshness checks also passed after the neighboring actor rename. The 32 worker-protocol tests passed again.

### Final audit and limitations

The final central statute audit covers 232 physical entries, with 222 governed entries and zero missing, multiple, generic, duplicate, spacing, presentation, or reserved-name findings. It includes the newly generated canonical boot output and new catalog/test data. The explicitly omitted dependency/cache paths are recorded in `statute-audit-final.json`; no authored paths were blanket-exempted.

Final React long rerun passed all 537 tests after the neighboring actor, Flow, and UI import moves completed. Earlier transient collection failures are retained as logs, not reported as successes. Scoped `git diff --check` passes.

Native WGPU nextest hit its configured 20-minute compilation budget while building the stdio plugin dependency graph. No native test success is claimed. The 49 literal Rust path mounts were independently verified to resolve, but native compiler/test completion remains a broader verification limitation.

After UI contract fixture case repairs and the asset-owner icon builder update, renderer exhaustive passed all 747 tests across seven files. Canonical WGPU generation refreshed one of six artifacts and the check reports all six fresh; direct frame-worker generation and its freshness check also pass. The current catalog bytes and digest remain unchanged, while the exact typed-scene catalog input path now points to `📇️catalog.json`. Frozen historical witnesses remain untouched.

Repo-library lint was run and failed on existing cross-project `rootDir`, typed-array, missing directory export, and `ImportMeta` diagnostics. It emitted no diagnostics in the changed WGPU catalog parser or generator sections. No unrelated type/API changes were made to force that broad lint target green.

Generated evidence is retained under `🗑️generated/renderer` until the parent finishes the ticket. The misplaced historical ticket mount mentioned above remains explicitly out of authored renderer scope.
