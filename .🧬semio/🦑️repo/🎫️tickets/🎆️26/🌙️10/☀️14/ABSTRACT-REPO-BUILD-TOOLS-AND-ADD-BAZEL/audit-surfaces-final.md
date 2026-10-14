# Public Surface Audit

Read-only audit performed 2026-10-14. Source and configuration edits are owned by the integrating agents. No git mutation was performed. Findings reflect concurrently changing files.

## High Impact Findings

### Launch normalization misses executable forms currently published

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌳️command-tree/🚀️launch/🟦️.ts:15` only recognizes `bun nx` and `bun run nx`; it misses `bun x nx` and standalone forwarded `-- nx run`. Array normalization also misses `["bun","x","nx",...]`.

Runtime reproduction executed with Bun importing the normalizer and printing `[DEBUG]` output: both `{command:"bun x nx run p:test --skip-cache"}` and a serialized `SEMIO_PROCESS_INVOCATION` containing `launches:[["bun","x","nx","run","p:test"]]` remain unchanged.

Real `bun x nx` launch commands occur at `.vscode/launch.json` lines 31135, 31146, 31157, 31168, 31179, 31190, 31201, 31322, 48248, 48262, 48273; seed equivalents are `.vscode/🧩️launch.seed.jsonc` lines 28606, 28617, 28628, 28639, 28650, 28661, 28672, 28793, 45735, 45749, 45760. They launch value-resident, UI host, framework-plugin, framework-process budget, framework-test adapter ownership and identity grapheme tests.

Forwarded `-- nx run` remains in launch lines 51097 and 51114; seed line 48473. These are active launch controls for ticket-owned verification, although ticket implementation history itself was excluded from this audit.

Existing precise normalizer regression suite: `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🚀️launch/🟦️.ts` (normalization assertion at line 66). This audit did not execute that suite.

### Runtime workspace discovery still depends on Nx configuration

These are runtime code paths, not Nx adapter tests. Replacing/removing `nx.json` would fail root discovery or weaken guards:

| File | Lines / Exact Behavior |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | 86,92 accept only `candidate.join("nx.json").is_file()`; 96 errors if absent; 4320 repeats the marker in another helper. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs` | 66,72 accept only Nx marker for override/ancestor discovery; 76 errors if absent. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` | 255 reads `ancestor.join("nx.json")`; 259-270 validate exact regular Nx/project pair and reject source authority without it. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` | 5629 uses `self.folder.join("nx.json").is_file()` to reject event-log writes to repository root. A neutral root must preserve this guard. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs` | 185 uses paired Nx/package markers then silently falls back to current directory. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py` | 374 requires Nx/package markers. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🔷️.cs` | 239 requires Nx/package markers. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐹️go/🐹️.go` | 319 requires Nx/package markers then falls back to current directory. |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` | 104 copies root `nx.json` into its fixture/bootstrap workspace. |

Canonical TypeScript `findWorkspaceRoot` now validates paired `📋️project.json` and `package.json` with a declared `workspaces` array/object (`🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts:91-105`). It checks up to 32 ancestors, honors explicit boundary and neutral environment hints. That implementation was inspected, not exhaustively runtime tested here.

## Other Observations

No Nx schema references were found in current authored project manifests. Current generator preview execution (`📚️library/🔍️discovery/🟦️.ts:6322`) correctly requires `repo:script` and validates literal owner command arguments. The neutral dashboard graph reader (`🎛️dashboard/📚️inventory/🦀️.rs:183-217`) reads version 1 `projects` records with `name`, `root`, `targets`, `metadata`; it no longer expects native Nx `nodes[name].data`.

No live direct Nx invocations were found in `.github` or `.devcontainer` in the scoped scan. Root package retains pinned Nx dependencies, appropriately needed by its backend adapter. Root `📜️script.ts` still contains comments naming Nx (6679,8354), which do not themselves affect execution.

Several tests outside adapter oracles still assert authored `executor === "nx:run-commands"`, including schema neutrality line 229, WFC snapshot ownership tests, and OS production-browser-artifacts completion line 21. These source-contract expectations need updating to `repo:script` where they inspect authored manifests. They were not executed by this audit.

Numerous Rust and TypeScript tests throughout hub, DSL, OS and plugins independently walk to `nx.json`. They are latent test portability coupling; runtime paths above have priority. Adapter-specific caching fixtures and native Nx oracle calls were excluded intentionally.
