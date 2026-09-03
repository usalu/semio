# MCP Binary Gate

## Outcome

The TypeScript MCP conformance package no longer treats a missing `semio-os-mcp` process subject as a successful run.

- The Rust MCP package owns a permanent `build` command and Nx target for the exact Cargo package and binary.
- The TypeScript package's default test path invokes that Rust Nx build before Vitest. `SEMIO_OS_MCP_BIN` remains an explicit prebuilt-artifact seam and is validated before Vitest.
- The build and test sides share one platform-aware resolver for the Cargo target directory and executable name.
- All four real-process suites require the executable during module loading. The four `describe.skipIf(!BIN_PRESENT)` branches and absence warnings are gone.
- The resolver handles relative `CARGO_TARGET_DIR` and `SEMIO_OS_MCP_BIN` values relative to the workspace and produces `.exe` paths on Windows.
- The shared language-neutral fixture is consumed by both the Rust build router and TypeScript tests. TypeScript additionally uses the host executable permission check as an independent oracle.

No runtime dependency was added. Existing process helpers, progress/cancellation-aware command runners, and test levels remain in use.

## Test-first evidence

### Baseline reproduction

```sh
SEMIO_OS_MCP_BIN="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/missing-semio-os-mcp" bun nx run @semio-tech/framework-os-mcp:test-quick --skip-nx-cache
```

Before the change this exited `0`: `2` files passed, `3` files skipped, `4` tests passed, and all `30` binary-backed tests skipped.

### RED path contract

```sh
SEMIO_OS_MCP_BIN="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/missing-semio-os-mcp" bun nx run @semio-tech/framework-os-mcp:test-quick --skip-nx-cache --testNamePattern='workspace default|relative cargo'
```

Before implementation: `3` failed. Relative Cargo targets were not workspace-resolved and Windows used POSIX separators/name.

### GREEN path contract and independent executable oracle

```sh
SEMIO_OS_MCP_BIN="$(command -v bun)" bun '🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts' test quick --testNamePattern='workspace default|relative cargo|explicit artifact|missing explicit'
```

Exit `0`: `1` file passed, `5` tests passed. The displayed `4` files/`36` tests skipped are exclusively the deliberate `testNamePattern` filter; they are not binary-absence skips.

### Fail-closed missing artifact

```sh
SEMIO_OS_MCP_BIN="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/missing-semio-os-mcp" bun '🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts' test quick
```

Exit `1` before Vitest discovery with `semio-os-mcp binary gate failed` and `ENOENT`.

Static removal check:

```sh
rg 'BIN_PRESENT|describe\.skipIf|suite SKIPPED|binary not found' '🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️'*.test.ts
```

Exit `1` with no matches. All previous absence-based skip paths are eliminated.

## Rust build and real-process blocker

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/mcp-binary-gate-target" bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache
```

Exit `1` / Cargo status `101`. Cargo reached a shared upstream dependency, then reported exactly `21` errors in `semio-framework-plugin-host`. Representative root diagnostics:

- `protocol::ForeignStep: serde::Deserialize` at the plugin-host `🦀️.rs:6454` and `🦀️.rs:6464`.
- `protocol::MergePolicy: serde::Deserialize` at `🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs:33` and `:38`.
- `semio_framework::io_schema::IoPayload: serde::Serialize` at the plugin-host `🦀️.rs:4133` and `:4137`.

These are outside this packet's MCP scope. No MCP-gate-owned compiler diagnostic was emitted, but Cargo could not produce a current-tree binary. Consequently:

- real-process tests executed: `0`;
- real-process tests skipped because the binary was absent: `0`;
- real-process tests blocked before Vitest by the upstream Rust build: `30`;
- exact binary smoke: blocked because there is no current-tree executable.

The failed target directory `🗑️generated/mcp-binary-gate-target` was deleted. Other agents' ticket output was preserved.

## Configuration validation

```sh
bun nx show project @semio-tech/framework-os-mcp --json
bun nx show project @semio-tech/framework-os-mcp-rs --json
```

Both projects resolve. The Rust project exposes `build`; the TypeScript test router directly invokes that Nx target on the default path and then independently requires the resolved executable. The direct router enforcement is required because the repository-wide `targetDefaults` currently normalize named test-target dependency metadata.

No launch seed change was needed: existing MCP stdio and HTTP developer commands already cover executable development, while `build` is an internal prerequisite rather than a developer-run process.

## Files

Added:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🧱️binary-gate.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-mcp-binary-gate.md`

Updated:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️end-to-end.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️hygiene.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️legacy-conformance.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️modern-era.test.ts`

The descriptor packet's retained oracle inputs were separately restored at `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/`, and `📓️sol-document-descriptor.md` now uses that retained path.

## Scoped diff check

```sh
git diff --check -- '🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp' '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-document-descriptor.md' '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle'
```

Exit `0`, no whitespace errors. The surrounding worktree contains extensive concurrent changes; none were modified or cleaned by this packet.
