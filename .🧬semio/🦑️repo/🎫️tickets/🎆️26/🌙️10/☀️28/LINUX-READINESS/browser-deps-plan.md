# Browser Prerequisite Wiring Implementation Plan

Read-only Source plan, 2026-10-29. No Source changes, tests, package installers, native probes or runtime gates were executed.

## Decision

Extend the existing `prepareDependencies("browsers", ...)` branch so it awaits Linux system prerequisites **before** its existing Playwright Chromium download. Keep `workspace:deps-browsers` as the sole graph interface; root setup and Demonstrator/Play already depend on that target. No new root target or shell subcommand is needed.

Move Chromium-only distro package mappings from Bash into schema-validated data owned beside native dependency orchestration. Remove the Bash post-setup browser installer and its ignored failure. Keep native toolchain package installation in Bash: it is required before a launcher exists, whereas browser dependencies run after the launcher and locked JS packages exist. This avoids parallel browser installers with divergent semantics.

## Existing Exact Owners and Authority API

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/`.

| Source | Current responsibility / required change |
|---|---|
| Root `📋️project.json` (workspace root, not modules) | setup already depends on deps-browsers; deps-browsers invokes native dependency script sync browsers. No graph change required for this fix. |
| `📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts` | `prepareDependencies` browsers branch currently only downloads. Add awaited system-prerequisite planner/executor before the existing runtimeExecutableV1 + playwright/cli.js install chromium command. |
| `📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` | Reuse `NativeDependencyToolRunnerV1` / `runNativeDependencyToolV1`; no new raw spawn runner. Signature already carries command, argv, cwd, exact AbortSignal, capture, environment, CargoPreparationStorageV1 and NativeCallerPhaseV1. |
| `📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts` | Preserve phase.authority.original, phase.authority.network.offline, phase.leaseControl.signal; phase.advance()/budget()/options() retain original bounded caller, progress, cancellation, transport and deadline. Do not synthesize replacement grants. |
| `🔩️native/🥾️bootstrap/🐚️.sh` | Remove linux_browser_packages, ensure_browser_libraries, and ignored post-setup call at current line474. Keep run_privileged/install_linux_packages for native compiler/toolchain installation. |
| `🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain/🟦️.ts` | Existing browser test extracts removed Bash functions; replace that browser-specific law with the new portable planner/executor test. Keep Rust/version/compiler/container laws. |
| `🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain/🔣️.json` | Existing neutral browserPackages corpus has exact dnf/pacman/zypper argv and package rows; move browser rows to new owner fixture and preserve exact data. |
| `📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts` | Existing direct apt command audit at line279 validates all Bash apt install/update calls use owned run_privileged env DEBIAN_FRONTEND=noninteractive; retain it for compiler install. Add assertion that native bootstrap no longer has a browser posthook. |

The existing `systemNativeToolPortV1` in `📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts` is for pinned Go/uv provisioning, uses runTool and has no passed NativeCallerPhaseV1. Do **not** route this new prerequisite through it: the dependency branch already owns the stronger exact phase-aware runner. The package-manager seam under dependencies/🔌️managers covers lock/sync/provision/install-pinned ecosystem generations, not privileged Linux OS packages; no reusable cross-distro OS-package interface was found in scoped library inspection. LifecycleHost's apt-only optional F3D installer is also unsuitable.

## Schema First, Small New Domain

Suggested short adjacent owner: `📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🌐️system/`.

Create a closed schema `🧬️schema/🔣️.json` before implementation. Define manager enum apt-get/dnf/pacman/zypper; ordered manager selection; noninteractive privilege policy (root direct, otherwise sudo -n); closed argv step shape (program/arguments/environment); platform skip values win32/darwin; and Linux Chromium policy data. Use data `🔣️.json` and neutral fixture `🧪️tests/🔣️.json`; implementation `🟦️.ts`, test `🧪️tests/🟦️.ts`. All full paths remain below 256 Unicode characters; verify actual repository path metric before creation.

An injected repository-owned system-package port should expose platform, uid/root determination, command availability, and the phase-aware command runner. Its real execution adapter must call the existing runner closure supplied by prepareDependencies, so all actual children retain phase.leaseControl.signal and phase.options(environment). Keep selection/planning pure and platform-independent; do not read process.platform/getuid inside the pure planner. Export no external-library type.

Linux:
1. Select first available apt-get/dnf/pacman/zypper in existing order. Missing supported manager is a required failure.
2. Root executes direct. Nonroot uses **sudo -n**, never a password prompt; missing/nonworking privilege fails the gate.
3. apt uses pinned existing Playwright CLI `install-deps chromium`, explicitly privileged and with DEBIAN_FRONTEND=noninteractive, before browser download. Use exact existing runtime executable and cli.js path; explicit sudo -n + env avoids Playwright itself attempting an interactive sudo. Missing CLI must fail, not succeed with “skipping”.
4. dnf: install -y; pacman: -Sy --needed --noconfirm; zypper: --non-interactive install --no-recommends, preserving current fixture packages exactly. In particular current zypper uses **mozilla-nspr**, and both xvfb-run and xorg-x11-server-Xvfb; do not resurrect old audit candidate libnspr4.
5. Propagate installer failure/cancellation unchanged; do not use the probe catch around required install. Check phase budget/progress around each child as current dependency branch does.
6. Respect original offline authority: a system-install operation requiring network cannot silently run online. Reject such provisioning when caller is offline unless a separately proven installed-prerequisite plan exists; do not infer installed state merely from package-manager presence.
7. Only after prerequisites succeed call existing Playwright install chromium.

Windows/macOS skip only Linux package provisioning and retain existing Chromium download behavior. No shell, PowerShell or Bash dependency is added to those platforms.

## TDD With Existing Neutral Evidence

Use the existing browserPackages rows as initial exact expected command/package outputs; they already assert install status7 propagation and Darwin no-op in the Bash test. Promote the inline Ajv schema to the new owned schema and use own validateJsonSchemaSubset plus existing third-party Ajv on the **same** input/output cases. Add portable recording-port cases:

- all four Linux managers, manager precedence, root direct and nonroot sudo -n;
- no manager/no privilege/missing CLI/installer status7 → failure, **zero browser-download calls**;
- noninteractive apt environment and exact CLI path before download;
- successful package child before download, Win/mac only download;
- cancellation before/while provisioning retains the original signal and prevents download;
- offline refusal without external calls;
- invalid schema/extra keys rejected by own validator and Ajv consistently.

Call the real prepareDependencies browsers branch with injected phase-aware runner/port using existing admitted test harness, to prove orchestration ordering rather than only testing a standalone planner. Existing dependency suite owner is `📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts`; follow its neutral corpus and third-party test conventions, but keep this feature's cases adjacent to the new domain. Existing native caller schema defines NativeDependencyKind browsers, so no new kind/schema enum is necessary.

Register new test/source entries through the current test contribution/manifest convention if discovery requires it; inspect current owner contribution before editing. Do not assert runtime Linux readiness from mocked commands: real clean-distro installer and Chromium startup acceptance remain later owner execution gates.
