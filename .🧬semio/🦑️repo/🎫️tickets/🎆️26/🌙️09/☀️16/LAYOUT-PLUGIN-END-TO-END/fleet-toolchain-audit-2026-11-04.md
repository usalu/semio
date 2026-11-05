# Independent Wasm Tool Routing Audit — 2026-11-04

Read-only source audit; no tools provisioned, executables run, tests/builds launched, or source edited.

## Routing Answer

An absolute owned wasm-bindgen executable is honored even after a stale bare wasm-bindgen route has been cached. In 🧰️framework/🔨️modules/🏃️process/🧰️tools/🔌️funnel/🟦️.ts:15, commands containing either path separator are excluded from tool lookup, except exact process.execPath. Line16 returns the original command, arguments and environment before line17 cache access. wasm-bindgen cannot normally equal the running runtime executable. No stale cached bare resolution rewrites this explicit path.

Conversely the bare-name cache at lines17-19 is keyed only by root, tool id and name. PATH and tool override values are absent from that key. Changing PATH after the first bare resolution does not guarantee a new bare resolution. The registry wasm-bindgen entry supports SEMIO_TOOL_WASM_BINDGEN then PATH and ~/.cargo/bin; this is separate from the policy SEMIO_WASM_BINDGEN_BIN override. A narrow caller fix can avoid the cached route by passing the explicit executable already selected and version-checked by the Wasm owner. Global routing cache redesign is not required to address this observed failure.

## Existing Owner Contract

The policy resolveWasmBindgenBin at repo library 🏃️process/🕸️wasm/🔣️policy/🟦️.ts:30-42 reads the root Cargo.lock version, rejects conflicting WASM_BINDGEN_VERSION, selects explicit SEMIO_WASM_BINDGEN_BIN or canonical provisioned partition cargo/home/bin when its probe matches, then falls back to PATH and verifies exact version/status. wasmPackEnvironment at lines51-59 prepends selected bindgen directory and selected optimizer directory, preserving child lookup. repositoryWasmBuildPolicyV1 line69 passes the selected absolute executable as bindgen.command and searchPath. Framework wasm-build line105 uses policy.searchPath and line127 invokes policy.bindgen.command for the direct threaded path.

Use this existing interface in the failing caller, with the same effective compiler environment used by actual execution. A selected PATH alone cannot prove direct owned execution when the registry has a prior bare cache. Keep exact version verification before the expensive compiler path.

## Minimal Regression Expectations

- With stale ambient PATH tool and correct owned provisioned tool, selection returns the owned absolute path and exact Cargo.lock version.
- After first resolving bare wasm-bindgen, routing the owned absolute path preserves command, arguments and supplied environment. This specifically distinguishes the observed cache scenario. Existing funnel tests cover explicit /usr/bin/git passthrough but not this prior-cache sequence.
- Actual direct invocation receives the selected executable; wasm-pack child environment resolves the selected dirname first. Assert both direct and child routes, since they use different mechanisms.
- A wrong explicit override or version disagreement refuses before compile; Windows selected filename/path uses .exe and platform PATH delimiter.

No runtime success inferred. Shared the routing answer with coordinator and layout_execution implementation owner.
