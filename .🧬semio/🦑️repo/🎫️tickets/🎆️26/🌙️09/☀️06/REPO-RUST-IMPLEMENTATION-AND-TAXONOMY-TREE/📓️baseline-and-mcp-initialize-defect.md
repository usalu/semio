# 📓️ Baseline and MCP Initialize Defect

## Baseline (2026-09-06)

| Implementation | Build | Tests |
| --- | --- | --- |
| Go client `🔨️modules/💻️client/⌨️cli` | `go build ./...` ok | `go test ./...` FAILS: package `repo/client` hits the 600 s default timeout (`TestExhaustiveFoldersNonEmpty` and siblings query the live monorepo through the GraphQL executor); `internal/command` and `internal/eventstore` pass |
| Rust CLI `semio-framework-repo-cli` | `cargo build -p semio-framework-repo-cli` ok (needs `RUSTC_WRAPPER=""`; `.cargo/config.toml` pins `sccache`, which is not installed on this host; `devToolingEnv()` already clears it for nx-driven runs) | 23 unit tests pass |

## Repo MCP fails to connect in Claude Code

`.mcp.json` starts `bun ./📜️script.ts dev mcp stdio client`, which builds and runs the Go MCP binary from `🔨️modules/💻️client/🔌️mcp`.
Claude Code reports `(-32602): "invalid initialize params"`.

Root cause: `🔌️mcp/🖥️server.go` `route()` decodes `initialize` params with `DecodeParams` → `decodeExact`, which disallows unknown JSON fields. Modern MCP clients send additional fields in `initialize` (`clientInfo.title`, extra capability objects such as `roots.listChanged`, `elicitation`, `tasks`, …). Any unknown field makes the whole initialize fail with `-32602`, so the session is closed before `tools/list`.

A manual probe with the minimal spec payload returns `{"error":{"code":-32004,"message":"session closed"}}` because stdin closes right after the request (expected). The defect is the strict decoder, not the transport.

Consequence for this ticket: the MCP protocol schema (🧬️schema) must define `initialize` as open for extension (unknown members ignored) and the language-agnostic MCP handshake test must include a fixture with extra client fields so both the Go and the Rust implementation are forced to accept them.

## Taxonomy JSON conflict (2026-09-06, resolved)

`🔨️modules/📚️library/🔣️taxonomy.json` carried four unresolved `<<<<<<< Updated upstream` / `>>>>>>> Stashed changes` hunks left by another session's `git stash pop`. Every harness phase parses this file first, so all test phases failed with a JSON parse error. Semantic comparison of both sides (`$TICKET/🗑️generated` python diff) showed the only difference was the plugin name `🪐️space` (upstream) versus the stale `🪐️s` (stash); the on-disk plugin is `✏️s/🔌️plugins/🪐️space`. The file was rewritten to the upstream side (JSON validated); no git commands were used.

## Harness reference run (coordinator, 2026-09-06 04:35)

`bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts subject fundamental --case "🖥️host-protocol-parity" --implementation rust|go` → `cases=1 executed=2 passed=2`. `parity fundamental --case "🖥️host-protocol-parity"` → `executed=8 passed=8 parity=12/12` (rust, go, typescript, python); the dotnet host fails to build because `🧪️test/📦️packages/🔷️dotnet/🔷️host.cs` is missing (pre-existing, out of scope; parity exit code 1 is caused by that alone). The `--case` selector wants the directory slug including its emoji.

## Closing check (coordinator, 2026-09-06 17:05)

`bun ./📜️script.ts dev mcp stdio client` (Rust default) and the same with `SEMIO_REPO_IMPLEMENTATION=go` both answer `initialize` (with `clientInfo.title` and `roots.listChanged`) and `tools/list`; the nine tool objects are identical across implementations (`file_integrate, goal_close, goal_open, goal_reopen, section_extract, section_move, ticket_close, ticket_open, ticket_reopen`). All 27 `go.work` members build, vet and test; `cargo build --release -p semio-framework-repo-cli` succeeds; parity for all 25 owners: 0 failed, 0 errored.
