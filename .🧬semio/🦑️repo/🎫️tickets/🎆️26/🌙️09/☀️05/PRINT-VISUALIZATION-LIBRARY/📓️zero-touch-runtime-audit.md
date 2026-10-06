# Zero-Touch Runtime Audit

Read-only source audit on 2026-10-06. No tests, publishers, package edits, Git mutations, or generated cleanup were performed.

## Finding

The current registered print launch has a source-owned automatic stable-runtime path. It does not require a ticket-local Bun environment workaround. This is a source finding; this audit did not execute the launch or establish macOS/Linux runtime success.

- `package.json:28` pins `bun@1.3.14`; `package.json:30` routes `bun nx` through the repository bootstrap script.
- `.vscode/launch.json:42915` invokes `bun nx run @semio-tech/print:test-viz-full` from the workspace root. No ticket runtime path appears in this launch row.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/package.json:4` independently pins `bun@1.3.14`.
- The tools `📜️script.ts:56–62` validates that recipe pin and calls `prepareBun` with it before Nx execution.
- The Bun owner `🛠️tools/🟦️bun/📜️script.ts:42–73` reuses an exact current version or acquires a checksum-verified workspace-owned executable, verifies its version, uses a cancellation signal, and reports acquisition progress. Its authored `🔣️.json:2–13` supplies Windows/macOS/Linux x64 and arm64 distributions, including Linux musl.
- The bootstrap `📜️script.ts:17–23,154` prepends the acquired executable directory to PATH; its Node-hosted Nx children inherit this environment at lines 189–190. Thus the print target's `bun ./📜️script.ts test viz full` (`📋️project.json:259`) selects the acquired Bun.
- `.devcontainer/Dockerfile:4,80–85` pins and verifies Bun 1.3.14; `.devcontainer/README.md:21` documents the pinned Linux x64/arm64 provisioning.

## Progress and Cancellation Seam

The Nx owner registers SIGINT/SIGTERM during provisioning (`📜️script.ts:143–150`) and graph execution (`194–195`), tracks descendants, and performs bounded shutdown (`155–188,235–245`). The full visualization consumer reports per-PDF completion and count (`print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:888,900`). Acquisition emits ten-second progress (`🛠️tools/🟦️bun/📜️script.ts:62–64`). No additional runtime-pin gap is substantiated in the audited launch chain.

## Limits

The outer launch still needs an available Bun and the Nx host invokes `node`; native installation prerequisites and actual launch behavior on all operating systems were not executed here. The source architecture is cross-platform, but that must not be reported as cross-platform runtime validation. The parent owns the live registered full gate and its runtime evidence.
