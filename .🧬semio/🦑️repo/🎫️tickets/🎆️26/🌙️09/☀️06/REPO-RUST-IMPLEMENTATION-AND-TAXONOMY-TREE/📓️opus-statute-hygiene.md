# 📓️ Statute hygiene (Opus executor)

Job: three statute/harness fixes that no single domain executor owned, plus real verification.
Resumed after a rate limit killed the first run at "verify the per-owner contract for the five listed
owners".

## 1. `testCaseSlugPattern` — emoji identity on test case directories

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`

```
"testCaseSlugPattern": "^(?:[0-9#*]\\uFE0F?\\u20E3|[\\u{1F1E6}-\\u{1F1FF}]{2}|\\p{Extended_Pictographic}\\p{Emoji_Modifier}?[\\uFE0E\\uFE0F]?(?:\\u200D\\p{Extended_Pictographic}\\p{Emoji_Modifier}?[\\uFE0E\\uFE0F]?)*)[a-z0-9]+(?:-[a-z0-9]+)*$"
```

The old pattern was `^[a-z0-9]+(?:-[a-z0-9]+)*$`, which made *every* emoji-prefixed case in the repo a
`testing/taxonomy case-slug` breach — including the reference example
`🧪️test/🧪️tests/🖥️host-protocol-parity` (coordinator decision, `📋️plan.md` §5). The new pattern requires
exactly one leading emoji grapheme (keycap, flag pair, or pictographic with modifier / VS / ZWJ
sequence) followed by the kebab slug, so the case directory carries the same identity every other
taxonomy node carries while the derived Nx project name (emoji dropped) stays typable.

Compiled once and reused in
`🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` (`caseSlugPattern`, required key list, breach text) and in
`🔨️modules/🧪️test/🟨️.mjs`. The breach message and remedy now name the emoji requirement.
`🔨️modules/🧪️test/README.md` step 2 of "Adding a feature" documents the shape.

Baseline had 250 `Test case directory …` breach lines; every post-fix contract run has **0**.

## 2. Workspace-permitted crates — `serde`/`serde_json` are not oracle leakage

Some owner registered a `serde_json`-based reader as an oracle
(`serde-json-equation-carrier-reader`), which armed `testing/dependency oracle-in-production` against
**143** production Rust files — including six of this ticket's own new repo crates
(`🧾️yaml`, `🧩️providers`, `🔗️graphql`, `📡️events`, `📐️model`, `🏃️test-runner`) — because they legitimately
use `serde` per `📋️plan.md` §3.

Rule (in `🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`):

* `workspacePermittedProductionPackages(repoRoot)` parses the ROOT `Cargo.toml`
  `[workspace.dependencies]` table and returns the external distributions it pins
  (today `serde`, `serde_json`, `wasm-bindgen`, `tokio`).
* `isWorkspacePermittedOraclePackage(ecosystem, name, permitted)` gates `oracle-in-production`,
  `oracle-production-reachable` and `probe-production-reachable` for Rust crates in that set only.
* `📜️script.ts` `dependency` prints each such package as `workspace-permitted … production reach is
  governed there, not by oracle purity` instead of failing, and drops them from the `leaked` set.

The exemption is **derived** from the root manifest, never declared per registry entry, so dropping a
crate from `[workspace.dependencies]` re-arms the purity rule in the same commit. Every other
ecosystem and every Rust crate outside the table is unchanged. Documented under
"Workspace-permitted crates" in `🔨️modules/🧪️test/README.md`.

All 143 `serde-json-equation-carrier-reader` breaches are gone; the 174 remaining
`oracle-in-production` hits are unrelated pre-existing ones (`node-crypto-repo-events` 65,
`typescript-compiler` 58, `three-fem2d-mesh-reader` 17, `brepjs-occt` 8, …) owned by other areas.

## 3. MCP `notifications/initialized` pipelining race — fixed in both implementations

A client is entitled to send `initialize` and `notifications/initialized` back to back. The reader
handed the notification through inline while `initialize` was still on a worker, so the notification
arrived in phase `Connected`, was dropped, and every later request answered `-32002`.

Both implementations now REMEMBER it and the initialize path applies it:

* Go `🔌️mcp/📦️packages/🐹️go/🐹️.go`: `Session.pendingInitialized`; `handleNotification` sets it in
  `phaseConnected` (and promotes directly in `phaseInitialized`), the initialize completion consumes it
  (line ~1108) and lands in `phaseReady`.
* Rust `🔌️mcp/📦️packages/🦀️rust/🦀️.rs`: `SessionState::pending_initialized`, same two sites
  (`handle_notification`, and `std::mem::take(&mut state.pending_initialized)` on the initialize path).

New scenario `@id-initialize-and-initialized-pipelined` (`@level-fundamental`, `@mode-differential`) in
`🔌️mcp/🧪️tests/🤝️jsonrpc-handshake/🥒️.feature`, with Go, Rust and TypeScript-SDK-oracle adapters:
"the client pipelines the initialized notification ahead of the initialize response" → "every
implementation answers a following ping instead of reporting an uninitialized session".

## 4. Collateral fix required to make parity green: the Go MCP package's dead import

The concurrent `go-split` wave removed the `github.com/usalu/semio/repo/client` module but left
`🔌️mcp/📦️packages/🐹️go` importing it, so the Go subject could not even build
(`no required module provides package github.com/usalu/semio/repo/client`) and **all five** Go
handshake/capability scenarios failed — not only the new one. Repointed at the split-out modules that
now own those symbols (verified no module imports `repo/mcp`, so no cycle):

| symbols | new module |
| --- | --- |
| `Tool*` (bundle/codebase/contributor/file/folder/goal*/policy/ticket*), `RunCLI`, `RunMCP` | `github.com/usalu/semio/repo/cli` |
| `McpClient*`, `McpServerName`, `ParseMcpClientKind` | `github.com/usalu/semio/repo/providers` |
| `LatestTicket` | `github.com/usalu/semio/repo/tickets` |
| `ToolExtract`, `ToolIntegrate`, `ToolSectionMove` | `github.com/usalu/semio/repo/move` |
| `ToolResult` | `github.com/usalu/semio/repo/workspace` |

`🔌️mcp/📦️packages/🐹️go/go.mod` gained the five `require` entries plus the sibling `replace` lines;
`🐹️.go` and `🧪️_test.go` use the qualified names. `go build ./...` and `go vet ./...` are clean.

## 5. Verification (real output, this host)

`RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`.

`bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts contract --owner <dir>`:

| owner | `Test case directory` breaches | `registered oracle serde…` breaches |
| --- | --- | --- |
| `📐️model` | 0 | 0 |
| `🧩️providers` | 0 | 0 |
| `🔌️mcp` | 0 | 0 |
| `🔗️graphql` | 0 | 0 |
| `🧾️yaml` | 0 | 0 |
| `🧪️test` | 0 | 0 |

(baseline before the fixes: 250 and 143 respectively). Every run still exits 1 on **unrelated**
pre-existing breaches shared by all six owners — `testing/discovery` executable-test-file baselines
(`temp` 207, `🧰️framework` 66, `.storybook` 11, `✏️s` 6, `♻️mit-bestand` 1), `testing/fixture`
mutation-vector gaps under `✏️s/🔌️plugins/…`, and the 174 non-serde `oracle-in-production` hits above.
None of those are in this job's scope.

`SEMIO_TEST_BUDGET_MS=600000 … parity quick --owner 🔌️mcp`:

```
[test] level=quick cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
exit=0
```

(before the §4 fix: `passed=18 failed=5 parity=9/19`, all five failures the Go subject's build.)

## 6. Left for others

* `📚️library/🔣️taxonomy.json` is in an **unmerged index state** (`git ls-files -u` shows stages 1/2/3)
  from another fleet's operation. The working-tree file is conflict-marker free and parses as JSON, and
  every command above read it fine. Not touched — resolving an index conflict needs a modifying git
  command.
* Case directories that earlier executors created without an emoji prefix (`🔗️graphql`, `🗣️languages`,
  `📡️events`, `📐️model`, …) are the audit wave's rename sweep per `📋️plan.md` §5. The pattern now
  demands the emoji, so those will surface as `case-slug` breaches under their own owners — the six
  owners verified here are clean.
* `@modelcontextprotocol/sdk` is still declared a `dependency` (not `devDependency`) by
  `💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript` while the MCP registry entry declares it `testOnly`
  (`📓️opus-mcp.md` §7.3). The `dependency` phase does not flag the disagreement today; a stricter purity
  pass would. Not a serde-class case — it is not in the root `[workspace.dependencies]` table.
* `bun ./📜️script.ts test` for any Rust package remains broken repo-wide via `loadTaxonomy()` and the
  `wgpu-frame-worker` generator contract (`📓️opus-mcp.md` §7.2). `cargo test -p …` and the harness both
  work; unrelated to this job.
