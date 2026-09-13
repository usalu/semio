# Hub Credential Source-Proof Intake

At 2026-09-13T02:50:22.647Z, root selected the two current function declarations from `🌎️hub/📦️packages/🦀️rust/📜️script.ts` using installed TypeScript 5.9.3, transpiled their unchanged bodies, and invoked them through the node:vm API under Bun with a bounded read-only filesystem. The only injected dependencies were native path/read/existence functions, the current repository root, and the actual literal MCP probe schema constant. This did not import or execute the whole Hub command, spawn children, open listeners, deliver credentials, run a native Rust binary, or mutate product files.

The two additional diagnostic cases change only source text in memory: one substitutes the current WGPU runner coordinate; the other reduces the duplicated identical MCP selector array to its single current authority. They are diagnostic proposals, not product repairs or compatibility fallbacks.

| Helper | Case | Transpile diagnostics | Result | Exact diagnostic |
| --- | --- | ---: | --- | --- |
| `proveNativeCredentialSourceOrder` | current-source | 0 | failed | `ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts'` |
| `proveNativeCredentialSourceOrder` | in-memory-current-runner-coordinate-only | 0 | failed | `WGPU native runner is not a direct binary supervisor` |
| `proveMcpCredentialSourceOrder` | current-source | 0 | failed | `MCP runner path must resolve to exactly one physical source` |
| `proveMcpCredentialSourceOrder` | in-memory-single-current-runner-authority-only | 0 | failed | `MCP credential marker/claim no longer rejects non-fd3 values before argv parsing and workspace activation` |

## Concrete Repair Boundaries

The WGPU helper still selects the absent Rust package command. Its current runtime command is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts`. A path-only substitution reaches the next stale source predicate: the helper demands the old token runCmdStatus(nativeBinaryPath(ship), whereas current native execution flows through runNativeSession → runNativeBinary → runTool. Rebind the semantic source proof and complete owner/input chain; a pathname-only fix is insufficient. This is a stale proof predicate, not evidence that the current runner executes Cargo or mishandles credentials.

The MCP selector array contains the same current command path twice and requires its filtered length to be one. The current helper therefore fails before reading that runner. Reducing only that array in memory exposes another proof defect: entrypoint.indexOf("parse_args()") matches the function declaration at line93, which precedes the actual credential claim at line166. The actual main-body parse_args invocation is at line188, after the claim. The source assertion currently compares a call against a definition, not two execution points. This is not evidence that the MCP executable parses before claiming a credential.

Resolve call/definition ownership and relevant function-body boundaries against the actual Rust/native syntax oracle, with language-neutral positive and hostile order cases. Keep one current source authority rather than a legacy candidate list. Move these proof helpers to the appropriate verification concerns during Hub ownership extraction; preserve actual native child/pipe tests and report separately whether those ran. The source proof must follow the actual process owner if its command implementation moves again.

## Exact Read Boundaries

### proveNativeCredentialSourceOrder — current-source

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`

### proveNativeCredentialSourceOrder — in-memory-current-runner-coordinate-only

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts`
- `.vscode/🧩️launch.seed.jsonc`

### proveMcpCredentialSourceOrder — current-source

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`

### proveMcpCredentialSourceOrder — in-memory-single-current-runner-authority-only

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
- `.vscode/🧩️launch.seed.jsonc`

## Attribution And Limits

Root created only this retained report and updated Hub intake, work queue, index and coordinator ledger. No product repair is attributed. Raw structured results are disposable at generated/coordinator/hub-source-proof-intake/results.json; all results and read boundaries are retained above. The complete Hub route, native security behavior, actual child environment/fd handling and native Rust tests remain unrun for this intake.
