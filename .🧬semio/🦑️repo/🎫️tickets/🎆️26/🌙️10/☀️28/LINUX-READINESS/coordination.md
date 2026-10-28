# Linux Readiness

The native launcher and repo command were built on Linux using the owned bootstrap and repo-mcp:install target. The local MCP stdio server initialized successfully and repo://goals was read before this ticket opened. No previous ticket specifically covered Linux readiness.

Native bootstrap remains live (session 62846), provisioning pinned Rust after an unsuccessful password-protected pacman installation. Native repo installation completed successfully (session 44638). Local MCP server is live in session 36246.

Work covers native Linux and devcontainer setup with Windows/macOS preservation, launcher/path integration, repository build/test gate discovery, implementation fixes and final verification. Completion remains unproven.

## Verified Gates

- Native repo-mcp:install: passed on Linux, installs /home/deck/.local/bin/repo.
- MCP initialize and repo://goals read: passed through repo mcp stdio.
- repo preflight: failed with missing cc and pkg-config. Package-manager installation requires unavailable interactive sudo credentials. This is a verified environment prerequisite failure; it does not prove repository readiness.
- Bootstrap process remains live: PID 61145, child rustup toolchain install PID 64308 observed through /proc.

- Linux preflight contract suite: 3 tests passed, 51 assertions, including language-neutral corpus and independent schema admission.
- Launcher installer suite: agent reports 8 tests passed, 122 assertions.
- workspace:test-workspace-build and tools verify both fail before execution with ENOENT for .cache/b/l/fixed.dir. This clean-cache native-build lifecycle defect is assigned for regression test and source repair.
- Bootstrap installed pinned Rust nightly-2026-07-20 and .NET SDK 8.0.421 successfully; uv provisioning still live.

## Bootstrap Follow-through

Original bootstrap exited2 when concurrent source edits shifted the running Bash script read position. After source finalization and syntax verification, the bootstrap was restarted through repo tools exec; it installed/verified all pinned language runtimes and rebuilt native repo, then exited1 at workspace:setup on the already diagnosed fixed.dir lifecycle defect. No bootstrap process remains live.

Replaced pre-permission-change agents with fresh agents to eliminate stale escalation prompts. Corrected .devcontainer/README.md storage request to160GB and described actual image-installed Node separately from launcher-provisioned runtime.

## Current Fix Verification

Bootstrap portability fixes now cover POSIX numeric version comparison, Fedora ICU development dependency, rustup profile preservation and disabled rustup self-update. Native syntax check and neutral bootstrap contract suite pass (3 tests31 assertions independently rerun by auditor). Cross-platform assertion corrections separately pass3 tests216 assertions. Windows parity and fixed-partition lifecycle repairs remain in progress.

## Full Monorepo Execution

Static inventory covers1489 project manifests and3978 declared targets. Native lease corruption retired only after no open /proc descriptors and12 exclusive SQLite database locks, inventory/inode verification and atomic quarantine. Workspace build now advances beyond fixed.dir failure and refuses stale JavaScript locks in7 scopes. Regeneration exposed excessive global ownership frontier replay (>2.6million operations); operation was explicitly canceled (terminal130, no source bindings changed) and targeted performance repair assigned.

Exact devcontainer Dockerfile built successfully through registered Docker interface backed by rootless Podman (image8f71f06d6a99f056b69d57e5065522a35ccd92aaf35eebb90c7b6afbc5c1b494). Provisioned compiler/browser libraries and pinned Node verification passed; container native bootstrap and full source build remain in progress.

## Provisioned Linux Host Evidence

Rootless Ubuntu container boots unprivileged uid1000 with isolated cache/node_modules volumes. C13.3+pkg-config compile/link/run Cairo/Pango/ICU/UUID smoke passes. All pinned language runtime bootstrap steps completed; native repo install then stopped on stale nonroot dependency scope locks. No duplicate bootstrap launched.

Native distro prerequisites now include xdg-utils and Vulkan loader; image includes Mesa software Vulkan ICD for rootless headless validation. New package capability corpus passes6 tests70 assertions.

Local native-repo-path is generated machine-specific output; exact .gitignore rule added and independently checked. Tracked artifact retirement remains pending final installed-frontend verification.

## Current Gates After Lock Repair

All6 authored JavaScript lock scopes independently check with0 drift after successful all-scope regeneration. Workspace build gate proceeds but requires32320MiB dependency image capacity; old declared8GiB image refuses before source bindings change. Dependency budget/schema and derived host requirements are under repair.

Provisioned Ubuntu container repo preflight passes and real Vulkan VkInstance creation/physical device enumeration reports llvmpipe software ICD. Native Go integration is rerunning after raw tools were archived, byte-compared and retired with no /proc executable/file-descriptor references.

Full workspace build, tests and full-monorepo readiness remain unproven.
