# Native Bootstrap Audit

The Unix bootstrap now compares numeric versions with POSIX awk rather than GNU sort -V, installs Fedora ICU development headers through libicu-devel, and passes --no-modify-path to rustup-init. These predecessor edits were inspected and validated. Rust toolchain installation now passes --no-self-update to prevent rustup from silently replacing its pinned installer version.

The language-neutral JSON corpus covers six version ordering examples, development packages for apt-get, dnf, pacman and zypper, and required rustup installer and toolchain arguments. Ajv validates version fixture shape and semver independently checks expected ordering. Shell extraction executes the actual implementations with a deliberately unusable sort function; a fake downloaded rustup installer captures the actual installer arguments and a stub rustup captures toolchain arguments. No downloaded installer is run by the tests.

## Verification

- Red-first regression: the new --no-self-update assertion failed with the original toolchain invocation (2 passing tests, 1 failing).
- After correction, the native bootstrap toolchain suite passed all 3 tests.
- bash -n passed for the complete Unix bootstrap.
- The combined native and existing cross-platform bootstrap run completed with 17 passing tests and 5 failures. The failures concern existing over-budget tracked paths, quoted git check-attr paths, obsolete Bun installer pin extraction (two cases), and a missing generated flow browser-package/package.json workspace member. The existing native Unix package-manager checks passed.

## Scope and Limitations

Source changes are confined to the Unix bootstrap and adjacent language-neutral test corpus and TypeScript test runner. No AGENTS.md or git metadata was modified. Actual bootstrap provisioning is being coordinated by the parent agent; the earlier live script process ended after concurrent source edits shifted its read position, so the parent will resume after the finalized syntax check. Native macOS and Windows execution were not available on this Linux host.

## Stale Cross-Platform Test Corrections

The existing cross-platform bootstrap suite now decodes git check-attr output as NUL-separated path, attribute and value triples, so default core.quotepath does not corrupt non-ASCII path assertions. Bun runtime checks now read the launcher-owned registry and verify checksum shape there; the image correctly contains no separate Bun download, and Unix distribution extraction tests cover only the native asset manifest the script owns. All three corrected tests pass, with 216 assertions. Existing path-budget and missing workspace-manifest failures remain documented separately. See path-budget-audit.md for measured path violations.

## Windows Rust Parity

A red-first regression identified that Windows rustup-init also omitted --no-modify-path. Windows now uses that flag and passes --no-self-update when installing the repository toolchain, matching the same language-neutral argument corpus used by the executed Unix tests. The Windows source test independently decodes the PowerShell argument array through JSON and validates its shape with Ajv. Native Windows execution is not claimed. The PowerShell UTF-8 BOM was preserved.

The final targeted native and cross-platform run passed all 9 selected tests and 264 assertions, including portable ordering, all four package-manager inputs, Unix installer execution with stubs, Windows argument parity, PowerShell encoding, shell LF encoding, complete bash parsing, checksum contracts and shell distribution selection. The three stale-test failures were corrected without weakening path or workspace checks. Path-budget measurements are retained in path-budget-audit.md (195 files and 281 directories over the declared stock Windows budget).

## Browser Libraries Across Linux Families

The Unix bootstrap now owns Chromium prerequisite package rows for dnf, pacman and zypper and invokes them through the existing privileged package-manager interface. apt-get retains the pinned Playwright CLI install-deps chromium path, including its existing behavior when Playwright is absent. Darwin exits without installing Linux packages. Supported non-apt managers no longer return a manual-install message. Installation errors propagate from ensure_browser_libraries; the outer bootstrap retains its existing diagnostic behavior.

A schema-first neutral corpus declares exact manager prefixes and package argument sets. The regression failed before implementation because non-apt hosts emitted no installer arguments. The final tests execute the actual extracted shell functions, capture privileged command argument boundaries with NUL separators, compare the command array with the neutral fixture through both native array assertions and Ajv JSON Schema const validation, and force each manager to fail to verify status propagation. apt routing and failure propagation, plus Darwin no-op behavior, are covered. These tests invoke no real package manager or privilege escalation.

Official package evidence supports the capability mapping: [Arch Chromium dependencies](https://archlinux.org/packages/extra/x86_64/chromium/), [Fedora AT-SPI bridge](https://packages.fedoraproject.org/pkgs/at-spi2-core/at-spi2-atk/), [Fedora GBM](https://packages.fedoraproject.org/pkgs/mesa/mesa-libgbm/index.html), and [Fedora Xvfb](https://packages.fedoraproject.org/pkgs/xorg-x11-server/xorg-x11-server-Xvfb/index.html). The openSUSE software endpoint returned HTTP 403. The official [openSUSE x86_64 package index](https://download.opensuse.org/tumbleweed/repo/oss/x86_64/) and [noarch package index](https://download.opensuse.org/tumbleweed/repo/oss/noarch/) were read through the repo node interface instead: this corrected the candidate libnspr4 name to mozilla-nspr and verified xvfb-run in noarch plus xorg-x11-server-Xvfb in x86_64. The remaining candidate openSUSE package names were present in the x86_64 index. Xvfb is listed explicitly as well as its wrapper.

The existing native-toolchain invocation-count assertion was scoped to ensure_native_toolchain, because browser provisioning is a separate owner with its own explicit command and error tests. Its previous condition accidentally counted every package installer invocation anywhere in the script. Native fail-and-report behavior remains asserted; browser failure propagation has independent coverage.

Final focused run: 12 tests passed, 0 failed, 299 assertions across native toolchain and cross-platform bootstrap suites. Complete Bash parseability, shell LF encoding, PowerShell BOM preservation, package-manager mapping, checksum contracts, Rust flags, version ordering and browser routing passed. No Fedora, Arch or openSUSE package installation or browser startup is claimed: these require clean hosts/images with package-manager privileges and the pinned browser executable. The changes provide automatic installation through each supported manager, with that end-to-end validation boundary recorded explicitly.

## Desktop Launch and Vulkan Runtime Prerequisites

Neutral package requirements were expanded first for xdg-utils and each distribution Vulkan loader, and an image capability requirement was added for mesa-vulkan-drivers. Before source correction these tests failed (4 pass,2 fail). All four native package rows now satisfy desktop URL-launch and Vulkan loader prerequisites; the image explicitly installs the software ICD needed without host GPU devices. The native regression suite subsequently passed6 tests and70 assertions. Actual Ubuntu provisioning and preflight evidence are recorded in host-capabilities.md.

The actual retained Ubuntu container now passes repo preflight, compiles and links a C11 Cairo/Pango/ICU/UUID smoke test against system development packages, and loads a functioning Vulkan software ICD (llvmpipe). See host-capabilities.md and retained smoke input sources. Following successful root lock regeneration, the unchanged bootstrap was resumed; its final outcome will be appended after execution.

Final focused bootstrap regression run after desktop/Vulkan declarations: 13 tests passed, 0 failed, 314 assertions. Shell syntax/LF, PowerShell BOM, all native manager inputs, browser manager argv/error propagation, installer/runtime pins and software-ICD declarations passed. Fresh provisioned container rustup remains pinned at1.29.0 after repeated bootstrap probes.

## Final Current-Source Setup Run

Final configured Linux setup is live as session7334, streamed from start to generated/container-setup-final.log. Before running the exact devcontainer onCreateCommand and postCreateCommand, independent host/container hashing checked969 current Go and changed/untracked implementation/configuration files (14898446 bytes), including file modes. Drift was0. The existing frontend was a regular ELF at /home/vscode/.local/bin/repo. This is the same private Podman :O Source view, with no Source copies or worktree.

Native apt prerequisites were already installed. The bootstrap rebuilt its owned launcher and successfully rebuilt/installed the final native repository ELF from current frozen Go Source. It then entered workspace:setup; global graph inference is progressing with3761 projects. The full setup has not yet reached terminal success. No bootstrap .sh file is edited while this process is live. The existing misleading native-ELF shadowing warning is retained in the log for a post-terminal fix; no native ELF was replaced by a launcher symlink.

## Final Installed ELF Native Validation

Actual batch42380 completed1 because four native search/tree commands each exceeded their explicit120000ms budget with no stdout, no stderr and no progress notification. Both cold and identical warm search used --repo=/workspaces/semio --format=json search linux --limit=8. The only-file and explicit search-package scope variants also timed out120s. No successful search result or cachehit is claimed. Native search cancellation sent SIGINT to its exact owned child at40ms and settled in51ms with SIGINT, without a forced timeout. This proves process interruption, not an unobserved cooperative backend checkpoint. Cursor behavior remains API-level Go/SQLite proof; the CLI has no cursor flag.

The same final installed ELF MCP server completed0 in39ms. Actual newline-delimited initialize, initialized notification, tools/list and resources/list produced exactly response IDs1,2,3 with no error, protocol2025-06-18, 6 tools and 8 resources; the notification produced no response. Full raw MCP stdout is retained temporarily under generated/native-mcp.stdout.json. The Go owner is repairing the confirmed pre-search full-tree/cache bottleneck; no competing native rebuild or setup retry is live.

## Setup Terminal Correction

The earlier session7334 is terminal1, not live. Host earlyoom sent SIGTERM to the captured graph parent at4178MiB RSS, after464worker adoptions. Root has no live setup or wholegraph retry. The parent/worker/fixed-overhead reservation correction has focused green evidence and remains under final lifetime review before a fresh run. The Source overlay must be revalidated against the completed patch before that setup retry. Full monorepo setup/build/test remains unproved.
