# audit-verify-1019c: verification of result-bzl-sandbox, result-plugin-cache, result-owners-gate, result-fixups, result-deps3

Read-only auditor `aud-verify3`. No source edits, no builds, no `bun repo`, no cargo or Go compile. Tests run only as explicit single files, one at a time. Scratch files are under `C/🗑️generated/verify3-art/`.

Paths: `R` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `C` = this ticket.

## 1. result-deps3 (dependencies routing): CONFIRMED, one undisclosed change

- Routing, `R/🏗️build/📜️script.ts`: the `dependencies` branch returns at about line 121, before the `"dependencies"` reader at line 131. The reader is now opened only for the diagnostics partition. `cargo` also returns early (about line 125), so it no longer runs under the dependencies reader.
- Commands now outside the dependencies reader: `dependencies` (intended), `cargo` (previously inside it), `run @semio-tech/repo-dashboard-rs:*` (previously inside it).
- Need check: `cargo` and the dashboard do not require the dependencies reader. The dependency-store custody functions (`requireJavascriptDependencyStorageV1`, `requireRepositoryCacheStoreReadCustodyV1`) are called only from the dependencies storage module and its own openers. `R/🏃️process/🦀️cargo/` uses the compiler store and its own registry readers. `R/🎛️`-equivalent `🔨️modules/🎛️dashboard/📦️installation/🟦️.ts` opens no cache store. `ensureBuildEnvironmentV1` opens its own dependency store. No other command loses a needed reader.
- ISSUE (undisclosed, minor): the dashboard condition changed from `nativeArguments && native.dashboardInstalled(root,mode)` to `nativeArguments !== undefined`. Dashboard `run` now always launches the native binary, even when it is not installed. Fix: restore the `dashboardInstalled` check or state the change and its reason in the report.
- Storage test `R/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts -t "displaced lock"`: 1 test ran, 0 fail. Confirmed.

## 2. result-bzl-sandbox: CONFIRMED

- `bun test` of `B/💾️storage/🧪️tests/🟦️.ts` (`B` = `R/🏗️build/🔧️tools/🏗️bazel`): 7 pass, 2 skip, 0 fail. Matches the report.
- The guard is darwin-only. `B/💾️storage/🟦️.ts:16` has `if(!rule)return;`, and only `platforms.darwin` exists in `HostPaths`. Linux and Windows are not refused.
- Behaviour to note (not a defect): on darwin, a checkout under a non-ASCII user home (for example `/Users/Jürgen/...`) now fails at layout time for `install`, `outputs` and `server`. The fixture rows at lines 56-57 pin this. This is the intended fail-closed behaviour, but report it to the user as a hard refusal.
- Not verified: Linux and Windows layouts (the report says the same).

## 3. result-plugin-cache: ISSUES (report is stale against the working tree)

- Test `P/🧪️tests/💾️cache/🟦️.ts` (`P` = `R/🏗️build/🕸️graph/📥️inference`), run through `bzl-act-tools/wrap.ts`: 8 pass, 0 fail, 699 expects. Matches the report.
- ISSUE: the report's claims that `build` (79 TS artifacts) and the wasm store-slot producers stay `cache:false` (`managed-output`) no longer hold. `P/🧱️project-assembly/🟨️.mjs:49-50` defines `receiptedStoreTargetV1`, which keeps `policy.cache` and emits `deterministic` with output `storeReceiptOutputV1(name)` when the manifest says `cache:true`. Lines 221 and 226 apply it to `build` and to `wasmPackageSourceV1` producers. The corpus test at `P/🧪️tests/💾️cache/🟦️.ts:112` now asserts a "receipted store-linked build" replay.
- The replay depends on `R/🏗️build/🧾️store-receipt/` (`??` untracked, not in the report). The report's precondition ("the products store must provide a restore protocol") appears to be met by this untracked module, which was not part of the report's change list.
- Fix: either update result-plugin-cache sections 0, 2.1, 2.2 and 3 to the receipt design, or keep `cache:false` for `build` and wasm producers until `store-receipt` is tracked and audited. Track the `store-receipt` directory in the same change.
- Risk to check (not verified): `test` for artifact packages now has `outputs: []` with cache from the manifest (`project-assembly` `if (name === "test") policy.outputs = []`). A replayed test that also writes to `dist` or the store would skip those writes. Check the `test` target's `dependsOn` and inputs before flipping.
- Confirmed: `session-*` and `activate-*-react` were already `cache:true` at HEAD (`git show HEAD:…/🎮️playground-targets/🟨️.mjs`), so they are not newly cacheable.

## 4. result-owners-gate: CONFIRMED with ISSUES

- Tests: registry-consistency 11 pass, crate-tree registration 7 pass. Admission 2 tests: with the default 5 s timeout, the test "the canonical owner registry names actual ordinary Source manifests" fails at 9.9 s (ISSUE: timing flake under load). With `--timeout 300000`, 2 pass.
- Real registry, read-only `checkBuildOwnerRegistry` (script in `C/🗑️generated/verify3-art/owners-check.ts`): 3679 owners, 0 findings. The report's 3674 is stale, not wrong.
- ISSUE, silent data loss: `R/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts`, `syncOwnerRegistry`, the `if (!target)` removal branch (about line 120). It splices out the whole row, including `nativeRoots`, and records only `{name, root}` in `removed`. The report says native roots "stay authored (reported, not synced)". Fix: do not remove rows that carry `nativeRoots` (report them as authored findings), or add their `nativeRoots` to `removed`.
- ISSUE, destructive on transient absence: `gone` is driven by `existsSync` on the working tree (`findOwnerFindings`, `missing-root`/`missing-manifest`). A tracked manifest that a peer has temporarily moved or deleted causes its row to be dropped by `verify build-owners sync`. Fix: drop a row only when its path is also absent from the index or HEAD (`git ls-files`), or require an explicit flag for removal.
- ISSUE (AGENTS): no launch registration for `verify-build-owners` and `verify-build-owners-sync` (report open item 2). AGENTS requires every executable command in `launch.json`. The report's reason (`.vscode` retired) is not a waiver under AGENTS. Register them in the current replacement.
- Root `📋️project.json` targets (lines 3066-3082): both follow the `bun ./📜️script.ts verify build-owners [sync]` form. Compliant.
- Writers: `registerBuildOwnerRows` (crate-tree), scaffold and mutation-tree call `reconcileBuildOwnerRegistry` with a scope. Old helpers are removed, no compat layer. Confirmed.
- Not run: the CLI route (`bun ./📜️script.ts verify build-owners`), blocked per the report by a fem project.json declaration. Not re-run under the no-`bun repo` rule.

## 5. result-fixups: CONFIRMED

- MCP structured content is the same rule in all three implementations. Go: `💻️client/🔌️mcp/📜️protocol/🐹️.go` `CallToolResult.MarshalJSON`. Rust repo: `🔨️mcp/📦️packages/🦀️rust/📨️protocol/🦀️.rs` `wire_structured_content`. Rust os: `💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️.rs` `wire_structured_content`. Null is omitted, object is kept, array or scalar is wrapped when declared, otherwise omitted.
- The `outputSchema` text is identical in the fixture, Go (`ToolResultOutputSchema`) and Rust (`result_output_schema`). Schema `$defs.ToolOutput` and `Tool.outputSchema` are present in `🔨️mcp/🧬️schema/🔣️.json`.
- Fixture `🔨️mcp/🧪️tests/🧫️fixtures/🧱️tool-result-shape.json`: `semio.mcp.tool-result-shape/2`, 18 vectors, consistent with the rule.
- `bun test` of `🔨️mcp/🧪️tests/🔬️unit/🟦️.ts` (explicit file): 22 pass, 0 fail. The SDK `@modelcontextprotocol/sdk` is a devDependency in the root `package.json`, so it is test-only.
- Stale specifier: `git grep -F 'framework/config/schema/mutations"'` over ts, tsx, mjs and json (excluding tickets): 0 hits.
- Microsecond binding law: `toolJobMicrosecondWorkerBindingSelfTests()` returns 32 (direct call from the factory test module). Matches the report. `toolJobMicrosecondBudgetSelfTests` is not exported from `🏭️factory/🟦️.ts` under that name; the report's count of 30 was not re-checked.
- Not run (no-build rule, same as the report): Go tests, Rust unit tests and the os cargo tests. The Rust os `tool_result_wrapper_shape` and widened tool schema are therefore unverified by compile.

## Open items for the coordinator

1. plugin-cache: reconcile the report with the receipt design, or keep `build` and wasm off until `store-receipt` is tracked (section 3).
2. owners-gate: protect `nativeRoots` and index-tracked rows in the sync removal branch (section 4).
3. deps3: restore or disclose the dashboard `dashboardInstalled` change (section 1).
4. Launch registration for the two `verify-build-owners` targets (AGENTS).
