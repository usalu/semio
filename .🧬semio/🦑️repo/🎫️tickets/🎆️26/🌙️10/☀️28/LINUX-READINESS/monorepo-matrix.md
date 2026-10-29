# Complete Monorepo Readiness Matrix

Static inventory on 2026-10-28 found 1,489 project manifests, 3,978 declared targets and 2,151 distinct target names. These are authored manifest counts, not a successfully resolved runtime build graph. Counts include organizational projects and the workspace root; they do not imply 1,489 independently buildable applications.

## Declared Target Coverage

| Target | Project declarations |
|---|---:|
| build | 251 |
| test | 516 |
| check | 232 |
| lint | 11 |
| verify | 2 |
| test-e2e | 3 |
| test-native | 40 |
| wasm | 11 |
| package | 26 |

The inventory contains many named native, oracle, snapshot SQLite, source, ownership and runtime targets beyond generic test/build. Passing only generic targets cannot establish all named requirements without checking aggregate target closures.

## Domain Inventory

| Top-level domain | Manifests |
|---|---:|
| ♻️mit-bestand | 3 |
| ✏️s | 326 |
| 🌎️hub | 115 |
| 🎓️teaching | 2 |
| 🏢️semio-tech | 1 |
| 👴️leutwiler | 1 |
| 📋️project.json | 1 |
| 🧰️framework | 1040 |

## Language and Runtime Gates

| Family | Manifest mentions | Required closure |
|---|---:|---|
| Rust | 1242 | pinned rustup/toolchain, rust-lld, Cargo locks/offline registry custody, native compiler partitions, wasm32 target, generated packs, native tests |
| JavaScript/TypeScript | 267 | launcher Node/Bun registry, frozen workspace dependencies, generated bootstrap assets, Nx entrypoints, type checks, React/Electron/browser startup |
| Go | 30 | pinned Go, go.work modules, canonical runner compilation and publication, retained physical lease mount |
| Python | 8 | uv pin, Python interpreter, lock consistency, managed environment and fixture/oracle execution |
| .NET | 6 | global.json SDK pin, ICU on Linux, NuGet managed cache and native test execution |
| C/C++ | 1 | compiler, pkg-config, CMake/Ninja, Cairo/Pango/UUID/ICU and root cpp setup/configure/build/test/toolchain targets |

Mentions are overlapping lexical classifications of project path/target declarations; they are not precise package counts. Browser dependencies, SQLite, graphics/headless display, fonts, numerical oracle tools, Docker-backed services and project-specific generated assets add runtime prerequisites beyond language installation.

## Comprehensive Execution Gates

Run through the native repo frontend, with an explicit supported language. These targets exist in the root manifest:

1. repo preflight, then repo run workspace:setup and dependency ownership checks.
2. repo run workspace:test-workspace-build, repo run workspace:verify-zero-touch, repo run workspace:verify-build-owners-sync, repo run workspace:verify-dependencies-locks and repo run workspace:generate-config-check.
3. repo run workspace:schema-verify, repo run workspace:schema-test and repo run workspace:verify-typescript.
4. repo run workspace:build, repo run workspace:test and repo run workspace:lint. Inspect resolved closure to prove all appropriate project build/test/check targets participate.
5. repo run workspace:cpp-test-toolchain and repo run workspace:cpp-test, plus native/wasm/browser product acceptance targets that aggregates do not select.
6. repo run workspace:test-contract, workspace:test-oracle, workspace:test-subject and workspace:test-parity; verify named source/native/oracle ownership target closure instead of assuming generic tests include them.
7. Exercise all three declared test-e2e targets with their prepare/serve dependencies, browser startup and cancellation; exercise Docker-backed hub services on a Docker-capable host.
8. Repeat cold and warm devcontainer creation/start/attach and the corresponding native host gates on Windows/macOS. Source fixtures alone are insufficient platform execution evidence.

These are recommended commands, not completed runs. Do not run publish/release/deploy merely to validate build readiness. Resolve all missing system prerequisites before interpreting native build failures.

## Independent Change Verification

Independently ran current Go lease law plus native bootstrap toolchain laws through repo tools exec bun using explicit test paths and ticket artifact directory: 6 tests passed, 0 failed, 104 assertions. Included browser prerequisites routing every Linux package manager through the owned privileged installer, stable version third-party semver parity, Linux compiler/ICU inputs, Unix/Windows rustup profile safety, and Go compiler/publication physical lease partition settlement. These are focused behavior/source contract tests; they do not certify real package installation across distro images or a full compiled monorepo.

Machine-readable static inventory is temporary generated output and will be removed during final ticket cleanup. No source edits were made by this audit.

## Exact Authored Browser End-to-End Gates

Current physical manifest inventory has exactly three targets named test-e2e. Commands must run through the target graph, preserving preparation and owned service generations. No E2E target was launched by this audit.

| Target command | Authored manifest | Direct dependency route | Coverage/prerequisites |
|---|---|---|---|
| SEMIO_LANGUAGE=en repo run @semio-tech/mit-bestand-demonstrator:test-e2e | ♻️mit-bestand/🧺️demonstrator/📋️project.json | test-e2e→serve-e2e→prepare-e2e→activate-prototype + workspace:deps-browsers | Chromium acceptance: pane roster/boot, overview, preview, undo-redo, energy run/cancel, funding logos. Native/WASM activation and actual GPU-capable map surface required. |
| SEMIO_LANGUAGE=en SEMIO_BUILD_MODE=release repo run @teaching/architecture-course:test-e2e | 🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json | assets:build + ui-styling-tokens:generate + @teaching/proctor:build; course script owns disposable stacks | Both dev and release rehearsal topologies, Chromium desktop/phone/layout/pets plus exclusive presence→shortage→away→faults, boot warmup. Rust proctor, site release/Vite build, browsers, ports and isolated stores. |
| SEMIO_LANGUAGE=en repo run @semio-tech/semio-tech-play:test-e2e | 🏢️semio-tech/🎡️play/📋️project.json | test-e2e→serve-e2e→prepare-e2e→activate-prototype + workspace:deps-browsers | Chromium overview/app boot plus 3D edit/analyze/cancel, language choice and generation workflow locale variants. Plugin/WASM activation and real GPU/map behavior required. |

Demonstrator and Play direct leaves share relative commands: test is repo ./🔨️modules/🧪️e2e/📜️script.ts test, preparation is repo ./🔨️modules/🧩️runtime/📜️script.ts prepare-test, continuous server is the same runtime script serve-test. Their config files are 🔨️modules/🧪️e2e/🎚️config/🟦️.ts beneath each product. Test scripts require build-tool service session/generation and consume PLAYWRIGHT_BASE_URL; Playwright does not start an independent server. Both use one Chromium worker and platform ANGLE Metal onmacOS/GL otherwise; software override DEMONSTRATOR_E2E_GPU=swiftshader or PLAY_E2E_GPU=swiftshader is explicit reduced-capability selection, not proof of map-pane coverage. Demonstrator config records TiledMapHost failing to request tiles on SwiftShader. Preserve that coverage distinction. Activation transitively prepares framework OS variants and Rust WASM surface/editor/NodeGraph/puzzle (Demonstrator), and the full Play variant roster; direct leaf execution would omit these dependencies.

Course direct leaf is repo ./📜️script.ts test-e2e from its TypeScript package. runCourseEndToEnd lives in ../../🎭️e2e/🟦️.ts, with stack owner ../../🧱️stack/🟦️.ts and session schema ../../🎭️e2e/🔒️sessions/🔣️.json. Defaulttopology both; --topology dev/rehearsal deliberately narrows coverage, --serial changes concurrency only, --project/--grep narrow browser cases. Its rehearsal is staticreleaseorigin plus production proctor on anotherorigin; dev uses site proxy+developmentproctor. Both are actual local-process topologies. Docker image/stack checks are separate authored course targets; none of these three test-e2e routes directly requires Docker. Course ci-browser-prepare still calls Playwright install-deps chromium (Debian-specific CI helper); normal cross-platform browser prerequisite setup is workspace:deps-browsers, not invoking that CI helper on every Linux distro.

Existing environment proof, not new query: host-capabilities.md records registered Docker interface backed by rootless Podman6.0.2, checked-in Dockerfile image build and CPU llvmpipe Vulkan VkInstance/device enumeration without GPU mount. Current Dockerfile contains Xvfb/libvulkan1/mesa-vulkan-drivers and devcontainer selects DISPLAY=:99. These establish infrastructure/software ICD availability, not full native UI rendering or browser GPU-map acceptance. Complete headless Linux coverage still needs these actual three owned browser gates, including real map tile/pixel behavior; a successful package install, ICD enumeration or narrowed SwiftShader lane cannot substitute. No new Docker/headless probe was run.


## Current Linux Acceptance Gates (2026-10-29)

Re-read current root and selected project manifests; commands below are a plan, not executed by this audit. Finish the single existing workspace contract gate96614 through its owner; do not duplicate it. Representative canonical harness proof91133 is green, but a whole workspace pass is still required. Native layout candidate is pending final freeze and fresh installed ELF cold/warm search proofs. Nx watcher scoped coverage is a separate daemon-readiness gate; successful daemon fallback alone is insufficient.

After those owner gates, the canonical dependency-bearing sequence is `repo run workspace:setup`, `repo run workspace:build`, and `repo run workspace:test`, each with the existing explicit language/control and admitted concurrency policy. setup includes build tools, JS/Python/Cargo/Go/.NET/C++/browser/WASM/tool dependencies, Git setup, prepare, repo-mcp:install and framework OS MCP Rust build. Calling setup leaf directly does not replace these dependencies. workspace:build aggregates every declared build plus Storybook; workspace:test aggregates project test targets and the repo test-domain contract. Record explicit host-platform skips; Windows-specific tests cannot be claimed as native Linux execution.

The three actual E2E targets remain `repo run @semio-tech/mit-bestand-demonstrator:test-e2e`, `repo run @teaching/architecture-course:test-e2e`, and `repo run @semio-tech/semio-tech-play:test-e2e`. Demonstrator/Play depend on serve-e2e and their authored readiness chain; course depends on assets, generated styling tokens and proctor build. Preserve earlier matrix coverage requirements for real map/GPU adapters, domain shard scenarios and actual server readiness. Container image/adapter/headless driver checks already documented do not prove these E2E tests passed. Repeat acceptance in the configured devcontainer via its owned lifecycle after native Linux success; do not substitute container compilation for host-native success.

Native frontend gates use current `repo-mcp:build`, `repo-mcp:install`, `repo-mcp:test-install` and `repo-mcp:test` ownership. Rebuild/reinstall once the Go owner freezes, then verify actual installed binary search/MCP/progress/cancellation rather than only package unit tests. Recovery names have moved: `repo-cache-retirement:cache-retire-inactive-preview` and retirement, and `repo-cache-reset:cache-reset-{graph,nx,rust,products}`. They are maintenance capabilities, not mandatory destructive acceptance steps; their focused genuine/neutral gates already passed. Old repo: recovery names are stale.
