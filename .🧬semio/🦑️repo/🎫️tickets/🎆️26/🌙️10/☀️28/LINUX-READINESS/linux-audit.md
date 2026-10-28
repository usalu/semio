# Linux Readiness Verification Matrix

Audit date: 2026-10-28. This report distinguishes observed execution, source inspection, and remaining verification. Other ticket agents are changing bootstrap and lifecycle code concurrently; their final test results supersede interim observations here.

## Observed Native Linux Evidence

The installed native repo launcher runs on this Linux workstation. Tool registry execution through repo tools exec node works, with capacity admission and light-query modes. MCP initialization and ticket infrastructure were reported working by the parent audit. Independent preflight tests passed: 3 tests and 51 assertions. Launcher installer tests passed: 8 tests and 122 assertions. These counts were supplied by the coordinating agent, not rerun by this read-only audit.

The workstation initially lacks cc and pkg-config. The coordinating agent reports that sudo/pacman provisioning is unavailable in this session. Consequently a successful launcher installation is insufficient evidence that native builds, complete setup, or the workspace build work here. Rust and .NET installation were reported complete; the bootstrap session was still installing uv at handoff. The broad workspace-build suite encountered a fixed.dir lifecycle fixture bug under separate repair. No successful full workspace build is claimed.

## Platform Matrix

| Area | Native Linux | Devcontainer | macOS Preservation | Windows Preservation |
|---|---|---|---|---|
| Cold entry | Checked-in Bash bootstrap and installed native launcher; launcher observed working | postCreateCommand invokes checked-in Bash bootstrap setup before relying on repo | Same Bash bootstrap selects Darwin; Homebrew and macos CMake preset branches present | Separate PowerShell bootstrap retained; host execution unverified |
| System toolchain | apt-get, dnf, pacman, zypper package branches include compiler, pkg-config, Cairo/Pango, UUID and ICU | Ubuntu 24.04 Dockerfile includes compiler, pkg-config, UUID, Cairo/Pango, browser libraries and fonts | Homebrew pkg-config/Cairo/Pango/font installation and macos preset remain | Needs native Windows fixture and actual host verification; Linux execution cannot establish this |
| Architectures | Bootstrap recognizes Linux; devcontainer Node archives explicitly support x64 and arm64 | Dockerfile rejects architectures other than amd64/arm64 and checksum-verifies Node archives | Actual Apple Silicon/Intel bootstrap not executed here | Actual x64/arm64 bootstrap not executed here |
| Language tools | Rust/.NET installation observed by coordinator; remaining registry/setup closure not complete | Features pin Go, Python, uv, ruff, .NET; Rust feature defers toolchain provisioning | Preserve platform-specific tool registry rows and toolchain pins | Preserve PowerShell provisioning and Windows executable/environment discovery |
| Browser tests | Chromium system libraries automatically installed only with apt-get; other supported distro families emit manual instructions | Browser libraries, xvfb, fonts and shared Playwright cache declared | Browser runtime execution unverified | Browser runtime execution unverified |
| Cache filesystem | Preflight checks available bytes, atomic/journaled swap, case sensitivity and executables; actual host refuses missing compiler/pkg-config | Separate named .cache volume avoids host bind-mount filesystem limitations; node_modules and shared cache also named volumes | Host case-insensitive filesystems require supported cache placement; preflight must remain authoritative | Directory-swap implementation and native filesystem behavior require Windows host verification |
| Lifecycle | Full setup pending package prerequisites and current bootstrap repair | Explicit argv hooks; initial volume ownership; start/attach registered targets | No Linux-only lifecycle assumptions should enter shared code | Shared lifecycle fixtures must remain platform neutral |
| Concurrent checkouts | Source control modifications prohibited for agents; no audit source edits | Compose omits fixed project/image/container names; cache volumes keyed by devcontainerId | Docker Desktop file permissions documented | Docker/host path and volume behavior unexecuted |
| Tests/build | Focused preflight/installer passes reported; broad suite initially blocked | Runtime-bootstrap/lifecycle laws exist; container image not built in this audit | Source preservation only; no macOS execution evidence | Source preservation only; no Windows execution evidence |

## Concrete Gaps

1. Native non-apt Linux is not zero-touch for browser tests: ensure_browser_libraries explicitly returns after a manual-install message for dnf, pacman and zypper. Either provision those dependencies through a distro-neutral owned interface or explicitly scope browser-test readiness to supported provisioned hosts.
2. This workstation cannot yet establish complete native Linux readiness because required compiler/pkg-config dependencies are missing and package-manager elevation is unavailable. Record preflight as a prerequisite failure, not a test pass or application defect.
3. Container readiness currently has static configuration evidence only. Validate a cold build/create, start, attach, lifecycle idempotency and browser smoke test on a Docker-capable host before claiming end-to-end devcontainer readiness.
4. Cross-platform preservation requires passing shared platform fixtures plus actual Windows/macOS execution. Linux source inspection cannot validate executable permissions, case behavior, PowerShell, package managers, debugger discovery or native graphics on those hosts.
5. Documentation contains drift: devcontainer.json requests 160gb storage while .devcontainer/README.md describes 128 GB. Its Dockerfile paragraph says Node is not installed in the image while that Dockerfile installs a pinned Node binary. Reconcile documentation with the actual lifecycle contract.
6. The Docker-in-Docker feature can add its own metadata privileges. The Compose file alone does not prove the complete container unprivileged; its README correctly makes this limitation explicit.

## Verification Sequence

Use the native repo frontend for all operations. Complete and record bootstrap first, run repo preflight, then the focused cross-platform bootstrap, launcher installer, preflight matrix/toolchain/remedies, runtime-bootstrap and lifecycle laws. Run the repaired workspace-build suite before broader build targets. On a provisioned host, run native language smoke builds and Chromium startup. Separately exercise cold container creation and a second warm start, then repeat the same native checks on Windows and macOS. Report package prerequisites, fixture failures and runtime failures separately.

## Inspected Sources

- .devcontainer/devcontainer.json
- .devcontainer/Dockerfile
- .devcontainer/docker-compose.yml
- .devcontainer/README.md
- Native bootstrap Bash script under repo/native/bootstrap
- Build preflight implementation and its declared platform policy interface

The audit created only this Markdown report. It did not change source, AGENTS.md, or source-control state.

## Browser Prerequisite Follow-up

The exact owner is the native bootstrap function ensure_browser_libraries at lines 412–424 of 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh. It delegates apt hosts to Playwright install-deps chromium and returns success with manual instructions on every other supported manager. The function is called during the bootstrap setup sequence. linux_toolchain_packages at lines 266–273 already owns distro-specific compiler/graphics-development/font packages, and install_linux_packages at 275–285 owns privileged manager invocation. A browser capability package table belongs beside those package mappings, behind the same owned installer interface.

The checked-in Ubuntu Dockerfile is the existing complete browser baseline: NSS/NSPR, ATK/AT-SPI, CUPS, DRM/GBM, XKB/X11 compositor libraries, ALSA, Pango/Cairo, fonts and Xvfb. It also contains product/Electron dependencies, so copying its entire list would not establish a minimal Chromium set. Playwright dependencies are browser-version-specific; a static list must match the pinned Playwright Chromium binary and pass actual startup. The local node_modules/playwright-core runtime dependency table was absent at the conventional package path during inspection.

| Manager | Candidate direct capability package set, in addition to existing bootstrap Cairo/Pango/fonts | Validation boundary |
|---|---|---|
| apt-get | Existing delegation to pinned Playwright install-deps chromium | Already version-owned; validate with actual browser startup |
| dnf | nss nspr atk at-spi2-atk cups-libs libdrm libxkbcommon libXcomposite libXdamage libXfixes libXrandr mesa-libgbm alsa-lib libX11 libxcb libXext dbus-libs expat glib2 xorg-x11-server-Xvfb | Candidate translation; Fedora release package/SONAME resolution and startup required |
| pacman | nss nspr at-spi2-core libcups libdrm libxkbcommon libxcomposite libxdamage libxfixes libxrandr mesa alsa-lib libx11 libxcb libxext dbus expat glib2 xorg-server-xvfb | Official Arch Chromium dependency list corroborates the main capabilities; exact pinned Playwright binary closure remains unproven |
| zypper | mozilla-nss libnspr4 libatk-1_0-0 libatk-bridge-2_0-0 libcups2 libdrm2 libxkbcommon0 libXcomposite1 libXdamage1 libXfixes3 libXrandr2 libgbm1 libasound2 libX11-6 libxcb1 libXext6 libdbus-1-3 libexpat1 libglib-2_0-0 xvfb-run | Candidate translation; distribution/release naming and startup required |

These are capability translations, not a claim of certified minimal package sets. Dependencies resolved transitively by each manager can reduce explicit rows. Installing the distribution Chromium package would bring a broad dependency closure but add an unneeded browser and still would not prove compatibility with the pinned Playwright binary. A correct minimal set requires checking the pinned Chromium ELF closure and performing startup in clean images for each supported distro family; this audit did neither and makes no false readiness claim.

Primary package references consulted: [Arch Chromium dependency metadata](https://archlinux.org/packages/extra/x86_64/chromium/), [Arch AT-SPI dependency metadata](https://archlinux.org/packages/extra/x86_64/at-spi2-core/), [Fedora Chromium package ownership](https://packages.fedoraproject.org/pkgs/chromium/chromium/). The Arch references support capability names, not the complete Fedora/openSUSE translations.

## Devcontainer Documentation Drift Owners

.devcontainer/README.md, Docs/devcontainer.json paragraph describes 128 GB while .devcontainer/devcontainer.json hostRequirements.storage declares 160gb. Its Dockerfile paragraph says the JavaScript runtime is not installed in the image, immediately followed by language describing pinned Node archives; .devcontainer/Dockerfile actually downloads, verifies and installs Node 24.15.0 under /usr/local/bin. Correct the README to describe the image Node used for bootstrap and the launcher-managed pinned runtime separately, and synchronize its capacity statement with hostRequirements. No source changes were made by this audit.

## Bootstrap Regression Review

Reviewed current POSIX awk version_at_least implementation: numeric component comparison avoids GNU sort -V and handles stable versions including minor rollover. The language-neutral JSON corpus covers six stable version comparisons; its test independently checks expected results using the existing third-party semver.gte implementation and validates row shape using Ajv. The current comparator does not implement prerelease ordering: e.g. 1.0.0-beta would numerically equal 1.0.0. Current uv pins are stable; broaden the declared domain/corpus before accepting prerelease input.

Rustup installer uses --no-modify-path --profile minimal --default-toolchain none. Toolchain installation then runs from the repository with --no-self-update. Confirmed against installed rustup toolchain install --help through the repo tool interface: the toolchain operand is optional and defaults to the active toolchain; --no-self-update is an accepted installation flag. This preserves repository rust-toolchain.toml selection without shell profile changes.

Executed the focused bootstrap toolchain suite through repo tools exec bun with an explicit ./ path and ticket generated artifact directory: 3 passed, 0 failed, 31 assertions. Version, Linux package inputs and recorded rustup argument checks passed. An initial invocation without ./ was refused by the repository operand guard before test execution; corrected invocation passed. Test artifacts are under the shared ticket generated directory for final ticket cleanup.

Root reports README capacity/Node documentation corrections completed; earlier drift observations describe the pre-fix state. No fixed_birth literal appeared in current TypeScript source during this review; the lifecycle repair remains delegated and is not certified here.

## Cross-platform Law Diff Review

Reviewed the current cross-platform-bootstrap test diff against the tracked baseline using read-only git diff through repo. Switching git check-attr to NUL output preserves the LF assertion and improves correctness for Unicode or quoted paths: each triple must still exactly equal path/eol/lf. The byte-level CR rejection remains; bash syntax validation remains on POSIX hosts. Direct byte inspection confirms Bash has no BOM or CR and ends in LF; PowerShell retains its UTF-8 BOM, has no CR and ends in LF. PowerShell does not require CRLF.

Bun expectations now point to the launcher registry and assert bootstrap delegates to the launcher installer. This matches current ownership: checking removed bun_release_row shell helpers and demanding image Bun archives would assert an obsolete architecture. The replacement retains Bun packageManager version equality, hash shape, archive shape and version-bearing download base. Node image hash parity and complete native-bootstrap pin platform/architecture coverage remain asserted. Removing direct shell Bun selection checks is justified by moving selection to the launcher, provided launcher registry/distribution tests retain actual selection coverage. One residual coverage boundary: this cross-platform test alone does not assert a nonempty Bun distribution list or full platform/architecture coverage; installer tests must cover those invariants. This is a coverage boundary, not evidence that checks were weakened merely to hide a failure.

Neutral toolchain corpus and third-party semver oracle checks were verified and executed in the preceding review. Bootstrap-owned generated assets remain validated with Ajv against the bootstrap-source schema, asserted ignored by git patterns, published, checked present, and checked for system-only static closure. No disabled assertions or unconditional expected-success substitution was found in the reviewed diff. No audit source changes.

## Canonical Lease Path Producer Audit

Scanned source TS/Go/Rust across all workspace domains for the exact .cache/b/l/m/data literal, excluding generated caches, dependency trees and fixture/test trees, then narrowed explicit leaseDirectory producers. Hard-coded paths are widespread and do not alone prove an unmounted ingress: most consumers enter repository cache/tooling/compiler stores before acquisition. No additional compiler/bootstrap bypass was proven beyond the separately diagnosed canonical Go-plan path.

Concrete reviewed producers relative to repo library:

- 🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts:40 computes leaseDirectory before admission, but acquireCargoBuildLease at47, shared-write48 and capacity49 execute inside withRepositoryCompilerLaneV1 at42.
- 🏃️process/🦀️cargo/🎯️exact/🟦️.ts:76 passes canonical leaseDirectory into runExactCargoLaws inside compiler lane48.
- ⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts:165,189,202,221 use canonical directories with retained tooling stores; setup planning226 enters withRepositoryToolingStoreV1.
- ⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts:192,199,216 retain tools/compiler publication ownership; these are downstream leases rather than demonstrated cold ingress.
- 🏗️build/🔒️leases/⌨️command/🟦️.ts:31 is a separate suspicious CLI ingress: runLeasesCliV1 passes join(root, REPOSITORY_LEASE_DIRECTORY_V1) straight to runLeasesCommandV1 with no mount wrapper in this module. This is status tooling, not a compiler bootstrap, and needs call-site/lifetime review before classifying as a defect.

Other direct consumers include wasm compiler query policy at OS renderer wgpu wasm script76 and browser catalog staging compiler capacity183; these take admitted compiler/storage context and were not shown to run outside its lifetime. Canonical root strings should not be replaced wholesale: verify the actual retained lease partition scope and caller authority. The coordinating agent received these findings for the repairing agent.
