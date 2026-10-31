# Linux Setup and Acceptance Gate Map

Static revalidation on 2026-10-29 of current root/project manifests, native bootstrap and devcontainer lifecycle implementation. This report supersedes obsolete configuration observations in linux-audit.md; it does not establish runtime success. No build, test, native, browser, container, service or lease action was launched. Product Source was not edited.

## Requirement to Canonical Gate

Every command below starts with `SEMIO_LANGUAGE=en /home/deck/.local/bin/repo` (installed native frontend). Preserve the coordinating owner's issued control and concurrency policy. Do not execute target script leaves as replacements for graph targets.

| Requirement | Canonical command suffix | Authored prerequisite/acceptance boundary |
|---|---|---|
| Host admission | `preflight` | Real platform, storage and toolchain readiness; neither source fixtures nor image package declarations substitute. |
| Complete dependency/bootstrap closure | `run workspace:setup` | Depends on deps-build-tools, deps-js-all, deps-python, deps-cargo, deps-go, deps-dotnet, deps-cpp, deps-browsers, deps-wasm, deps-tools, setup-git, prepare, repo-mcp:install, @semio-tech/framework-os-mcp-rs:build. Leaf only reports graph completion and stages MCP. |
| Build | `run workspace:build` | repo:noop aggregate: all other projects' build targets plus build-storybook. Does not implicitly cover targets with other names. |
| Workspace test | `run workspace:test` | Other projects' test targets except workspace and @semio-tech/repo-test-domain, plus @semio-tech/repo-test-domain:test-contract; root leaf selects canonical test platform. |
| TypeScript | `run workspace:verify-typescript` | Explicit leaf routes verify typescript check; not a declared build/test prerequisite. |
| Contract and reference implementations | `run workspace:test-contract`, `run workspace:test-oracle`, `run workspace:test-subject`, `run workspace:test-parity` | Explicit canonical test selections; current root declarations have empty dependsOn. Preserve existing single owner execution instead of duplicating it. |
| Source/schema/setup laws | `run workspace:test-workspace-build`, `run workspace:verify-zero-touch`, `run workspace:verify-build-owners-sync`, `run workspace:verify-dependencies-locks`, `run workspace:generate-config-check`, `run workspace:schema-verify`, `run workspace:schema-test` | Separately authored gates, not implied by setup. Source-law success is distinct from actual cold installation. |
| Lint | `run workspace:lint` | Other projects' lint targets plus root leaf. |
| C++ named acceptance | `run workspace:cpp-test-toolchain`, `run workspace:cpp-test` | Explicit named gates; generic test does not establish arbitrary named-target coverage. |
| Demonstrator browser | `run @semio-tech/mit-bestand-demonstrator:test-e2e` | serve-e2e continuous owner → prepare-e2e → activate-prototype + workspace:deps-browsers. Preserve actual service readiness and GPU/map coverage described in monorepo-matrix.md. |
| Play browser | `run @semio-tech/semio-tech-play:test-e2e` | Same graph shape as Demonstrator; owned prepare/server, actual 3D/map and cancellation behavior. |
| Course browser | `run @teaching/architecture-course:test-e2e` with `SEMIO_BUILD_MODE=release` | Current dependencies: assets:build, ui-styling-tokens:generate, @teaching/proctor:build, plus **build-parametrization-en and build-parametrization-de**. Keep both dev/rehearsal topologies and all browser projects. Course real browser branch is peer-owned. |

prepare depends on framework-schema:generate, ui-styling-tokens:generate, assets:build, framework-graph:generate and plugin-registry:generate. Native dependencies additionally declare Python project deps, .NET test deps, cpp-setup, and wasm-opt/trunk. Browser sync installs pinned Playwright Chromium. Native Go synchronization uses pinned environment and go.work; .NET validates exact configured SDK. These facts describe source wiring, not completed installations.

## Devcontainer Cold and Warm Lifecycle

The configured entry is .devcontainer/devcontainer.json with Compose service semio and workspace /workspaces/semio. Image: Ubuntu 24.04, pinned checksum-verified Node x64/arm64, native/browser/graphics packages; features provide Go/Python/uv/ruff/.NET/Git and Docker. Current host requirement is **200gb**, matching README; prior 128/160gb drift is obsolete.

| Phase | Exact configured argv / target | Required observation |
|---|---|---|
| Cold create | onCreateCommand: sudo chown vscode:vscode /workspaces/semio/.🧬semio/🦑️repo/⚡️cache /workspaces/semio/.cache | Both checkout-private named cache volumes writable before installation. |
| Cold post-create | bash 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh setup | Bootstrap must install launcher before repo calls, install repo-mcp, execute full workspace:setup and debugger setup. Record real exit and missing prerequisites. |
| Every start | repo run workspace:setup-devcontainer-start | Lifecycle owns persisted home/submodule ownership, emoji fonts, Claude auth, git safe.directory, optional signing agent. Successful optional-step log is not proof of every external command's success. |
| Every attach | repo run workspace:setup-devcontainer-attach | Optional F3D installation unless SEMIO_POST_ATTACH_SKIP_TOOL_INSTALL; caught failures are intentional optional behavior. |
| Warm restart/attach | Same authored start/attach hooks after stopping and restarting the existing container | Preserve volumes/dependency generations; no duplicate signing shell blocks/safe.directory entries; verify actual read/write ownership and available frontend. |
| Container acceptance | Repeat native canonical setup/build/test/typecheck/contract/browser gates in configured container | Existing image/ICD/headless infrastructure evidence does not establish these gates passed. DISPLAY=:99 is configured; actual headed rendering remains an execution acceptance condition. |

Compose uses sleep infinity and no fixed project/image/container identity. Separate named cache volumes use devcontainerId; auth/preferences use checkout basename. No new container invocation was made.

## Concrete Remaining Source Configuration Blocker

**Linux browser system prerequisites are outside the canonical workspace:setup graph.** Root deps-browsers invokes native dependency sync browsers; its implementation runs only Playwright `install chromium`. The Linux system-library installer is the Bash bootstrap's ensure_browser_libraries, called only *after* workspace:setup and debugger setup. Its failure is converted to a log at bootstrap line 474, so native cold bootstrap may report success without the required libraries, and direct workspace:setup never requests them. Route system-library installation/probing through the owned browser dependency interface, with required prerequisite failures observable by the setup gate. This is a concrete missing graph prerequisite; actual host failure has not been tested here.

Old linux-audit.md's non-apt manual-instructions blocker is resolved in current Source: linux_browser_packages supplies dnf/pacman/zypper package maps and ensure_browser_libraries uses install_linux_packages. The apt path still delegates to Playwright install-deps chromium. README capacity/Node drift is also resolved. No additional blocker was proven in this scoped static review. Existing native linker/client and Course browser failures remain separately owned and are not re-audited here.
