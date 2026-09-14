# Explore: Go MCP server + library (for Rust port)

All paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/` unless absolute.

## 1. Go modules / packages

| Module (go.mod) | Path | Package | Lines (non-test) | Notes |
|---|---|---|---|---|
| `github.com/usalu/semio/repo/go` | `📚️library/` | `repo` | `🐹️.go` 805 | Shared: `EventKind` consts, `Event` envelope, ~20 typed payload structs (Ticket/Goal/Contributor/Todo/Draft/File/Folder/Section/Integrate/Extract), `Emit()` HTTP POST helper, source-parsing primitives (region-marker/markdown-heading section+definition extraction, `BuildScopeID`, `BuildScopesForFile`, emoji-rune detection). stdlib only (`net/http`, `encoding/json`, `regexp`, `path/filepath`). `go.mod`: 3 lines, `go 1.25`, no deps. |
| `github.com/usalu/semio/repo/client` | `💻️client/⌨️cli/` | `client` | `🧩️component.go` **46,896** + `📤️event_export.go` 169 | THE monorepo CLI: ticket/goal/contributor/todo/draft/file/folder/section/integrate/extract commands, `McpClientKind` enum (generic/cursor/kiro/copilot/claude/codex) + `ParseMcpClientKind`/`McpServerName`/`HookClientForMcpKind`/`ResolvePlanSource`, `Tool*` functions (`ToolTicketOpen`, `ToolTicketClose`, `ToolTicketReopen`, `ToolSectionMove`, `ToolIntegrate`, `ToolExtract`, `ToolCodebase`, `ToolBundleList`, `ToolFolderList`, `ToolFileList`, `ToolTicketList`, `ToolGoalList`, `ToolPolicyList`, `ToolContributorList`), `RunCLI()`. Own `internal/` subpackages: `command`, `eventstore` (CLI-local event log + tests), `glob`, `graphql`, `humanize`, `id`, `ignore`, `mcp` (210 lines, JSON-RPC types shared with `internal/mcpserver` 149 lines — an older/parallel MCP scaffold used only inside `internal/`, separate from the top-level `💻️client/🔌️mcp` binary), `search`, `templatefunc`, `yaml`. Depends only on `repo/go`. `cmd/repo/🐹️.go` is the CLI binary entrypoint (`client.RunCLI()`). |
| `github.com/usalu/semio/repo/mcp` | `💻️client/🔌️mcp/` | `main` | `📜️protocol.go` 368, `📡️event.go` 193, `🖥️server.go` 860, `🗄️repository.go` 368, `🚚️transport.go` 140, `🧩️component.go` 72 | The **from-scratch** MCP server binary (no third-party MCP SDK). Depends on `repo/client` (the giant package above) and transitively `repo/go`. `🧩️component.go` = process entrypoint (`main`, `resolveMCPProfile`, `runMCP*`, `serveMCP`). `📦️packages/🐹️go/` is an nx-only wrapper (`project.json`+`script.ts`, no real source) used by launch configs as `go run ./…/🔌️mcp/📦️packages/🐹️go`. |
| `github.com/usalu/semio/repo/server` | `🖥️server/🎛️coordinator/` | `main` | `🗄️event_store.go` 950, `🧩️component.go` 1580, `📚️repository.go` 814, `🛡️durability.go` 96, `🐚️durability_unix.go` 26, `🪟️durability_windows.go` 52 | Coordinator (compose) server. Depends only on `repo/go`. |

`go.work` (repo root) lists exactly these 4 modules: `⌨️cli`, `🔌️mcp`, `📚️library`, `🖥️server/🎛️coordinator`.

## 2. Wire protocol

### JSON-RPC / MCP (client/🔌️mcp)
- JSON-RPC 2.0, protocol version `2025-11-25`; `SupportedProtocolVersions` also accepts `2025-06-18`, `2025-03-26`, `2024-11-05`, `2024-10-07` (negotiated down).
- Error codes: standard JSON-RPC (-32700..-32603) plus custom -32001 payload-too-large, -32002 not-initialized, -32003 duplicate-request, -32004 stale-session, -32005 server-busy, -32800 cancelled. Domain tool errors use -32010.
- Transport: line-delimited JSON over stdio (`bufio.Scanner`), one goroutine pool (`MaxHandlers`) per session, request-id-keyed dedup/replay (`commitExchange`), progress notifications (`ProgressReporter.Report`), cancellation.
- **Tools (exactly 6, registered in `🗄️repository.go` `NewRepositoryServerWithLimitsFor`):**
  | name | required args | notes |
  |---|---|---|
  | `ticket_open` | emoji, title, prompt, goal | optional: client, llm, effort, draft, parent, issue, no_issue, no_management; `plan_id` added for cursor/copilot/claude/codex profiles, `spec_id` for kiro |
  | `ticket_close` | summary | optional: path, files[], title, no_management |
  | `ticket_reopen` | (none required) | path, prompt, llm, effort, client, draft, title, goal, parent, no_management, plan_id/spec_id per profile |
  | `section_move` | file, old_name, new_name | |
  | `file_integrate` | source, target_section, target_file | optional target_parent_section |
  | `section_extract` | source_file, source_section, target_file | |

  No `goal_*` tools are exposed over MCP — goal open/close/reopen exist only as CLI commands (`client.OpenTicket`-style functions and equivalents for goals live in the 46.9k-line `component.go`) and are not wired into the MCP tool table.
- **Resources (8, all `text/plain`):** `repo://`, `repo://bundles`, `repo://folders`, `repo://files`, `repo://tickets`, `repo://goals`, `repo://policies`, `repo://contributors` — each dispatches to a `Tool*List`/`ToolCodebase` call in `ClientRepository.Read`.
- **Prompts (4):** `enhance`, `refactor`, `test`, `comply` — each takes one required `prompt` argument and returns a fixed instruction string prefixed to it.
- **Per-IDE profile (`SEMIO_REPO_MCP_CLIENT`, `McpClientKind`):** `generic` (also matches `""`/`client`), `cursor`, `kiro`, `copilot`, `claude`, `codex`. Differences: (a) server name reported in `initialize` (`repo`, `repo-cursor`, `repo-kiro`, `repo-copilot`, `repo-claude`, `repo-codex`); (b) extra tool-schema field `plan_id` (cursor/copilot/claude/codex) or `spec_id` (kiro) on `ticket_open`/`ticket_reopen`; (c) `ResolvePlanSource` resolves that id to a different on-disk plan/spec file per IDE (`.cursor/plans/*_<id>.plan.md`, `.kiro/specs/<id>/`, `~/.copilot/projects/<repoBase>/memory/<id>.md`, `~/.claude/plans/<id>.md`, `~/.codex/memory/<repoBase>/<id>.md`).

### Event envelope (client/🔌️mcp, internal event log)
`Event{schema="semio.mcp.event/1", sequence, kind, peer, generation, requestId, payload, previous, hash}` — SHA-256 hash-chained JSONL, `EventLog.Commit`/`Snapshot`/`Events`, `ReplayEvents` validates chain integrity on load. This is separate from the CLI→coordinator event (`repo/go` `Event{kind, source, payload}`).

### Coordinator (🖥️server/🎛️coordinator)
- `EventEnvelope{stream, sequence, id, generation, type, payload, checksum}` — append-only, single flat file at `COMPOSE_SERVER_DB` (default `compose-server.db`, despite the name **not** SQLite — plain JSONL with SHA-256 checksums). Durability via stage/next/backup files + OS-level file lock (`🐚️durability_unix.go` / `🪟️durability_windows.go`) and fsync; `recoverLocked`/`rollbackLocked` handle crash recovery.
- HTTP (stdlib `net/http`, `ListenAndServe(config.Address)`, default `127.0.0.1:8787` via `COMPOSE_SERVER_ADDR`): `/healthz`, `/ticket/open`, `/ticket/close`, `/ticket/reopen`, `/tickets`, `/ticket/{id}`, `/diff/ingest`, `/repo/reindex`, `/repo/index-file`, `/warnings`, `/breachs`, `/scopes`, `/events`, `/webhooks/github`. No WebSocket in the Go server.
- `repo/go`'s `Emit()` is the CLI-side client for this: no-op unless `COMPOSE_SERVER_ADDR` set, POSTs to `<addr>/api/v1/events` with optional `Authorization: Bearer $COMPOSE_SERVER_TOKEN`.
- In-memory projection (`newCoordinatorProjection`/`.apply`) is rebuilt by replaying the event log — no external DB engine despite `.env.example` describing Postgres (`DATABASE_URL`, `POSTGRES_*`) — that env file documents the **separate TypeScript** coordinator, not the Go one.

## 3. Coordinator TypeScript package — real duplicate, not a twin
`🖥️server/🎛️coordinator/📦️packages/🟦️typescript/` is a **Next.js app** with its own backend, not a client of the Go coordinator:
- Routes under `app/api/v1/`: `auth`, `breach`, `diff`, `event`, `health`, `repo`, `scope`, `ticket`, `ticket/[id]`, `warning`, plus `app/api/webhooks/github` — this is a 1:1 route mirror of the Go `mux.HandleFunc` table above, reimplemented.
- `🖥️server-implementations.ts` uses real Postgres (`pg` `Pool`) and `pg-boss` (durable job queue) — genuinely different storage backend from the Go file-based JSONL store.
- `✅️validation.ts` (90 lines) + test — likely shared-shape validation, not protocol logic.
- **Conclusion for point 5:** the TS coordinator is a second, non-trivial implementation of the same HTTP surface with a different storage engine (Postgres vs. flat-file event log). Porting to Rust should pick one canonical implementation (recommend the Go file-store semantics, since `.env.example`'s Postgres config appears aspirational/unused by the Go binary) rather than porting both.
- `🖥️server/📚️library/📦️packages/🟦️typescript/🟦️.ts` (914 lines) and `👷️worker/🟦️.ts` (11 lines) are additional TS-side server library code — not yet inspected line-by-line; flag for follow-up before porting.

## 4. `📚️library/📦️packages/🟦️typescript/🟦️.ts` is NOT a twin of the Go event library
This file is 6,436 lines but it is the **monorepo build/lint/test-orchestration library** (nx/bun/cargo/vitest/pytest/dotnet runners, git metrics, coverage/lcov, cargo-law process probing, policy linters `BaseLinter`/`TechnologyLinter`/`BundleLinter`/etc., `resolveCliBin`/`resolveMcpBin`). A repo-wide grep for `EventKind` and event-kind string literals (`ticket.open.starting`, etc.) across all `*.ts` in `🔨️modules/` returned **zero matches** — the Go `EventKind`/payload types have no TypeScript counterpart to reconcile; they can be ported to Rust without cross-checking a TS twin. `resolveCliBin`/`resolveMcpBin` (lines 90–105) do however encode the expected binary names/paths (`💻️client/mcp[.exe]`, and a CLI bin) that a Rust build must keep resolving to.

## 5. Existing Rust / other-language surfaces that must not be triplicated
- **`⌨️cli/📦️packages/🦀️rust/`** (different path from `💻️client/⌨️cli/`!) — crate `semio-framework-repo-cli`, binary `semio`, 1,142 lines (`🦀️.rs`) + `📦️main.rs`. This is already "the monorepo orchestrator CLI + TUI dashboard, replacing `script.ts`'s dev/build front door" (its own doc-comment). It currently has **zero** ticket/goal/MCP logic (grep for `mcp`/`ticket_open` under `🎮️commands/` returned nothing) — it only handles workflow/plugin-registry/terminal-dashboard/playground-session/cli-usage/root-script-delegation/command-tree-discovery. This is the natural landing spot for a ported Rust dashboard, but the MCP/ticket/goal domain has no Rust code yet — a straight port, not a merge.
- **`🧪️test/` module** has `📡️protocol/🦀️.rs`, `📦️packages/🦀️rust/📦️lib.rs`, and a `🧪️tests/🖥️host-protocol-parity/` case with `.rs/.go/.ts/.py/.cs/.feature` adapters — this is a generic multi-language "host protocol" parity harness (per its own README, "Protocol v2" test platform), unrelated to the repo MCP/ticket protocol; do not conflate.
- **`💻️os` product** already has a working Rust MCP gateway: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` (+ `🚚️transport/`, `🧵️bridge/`), run via `cargo run -p semio-framework-os-mcp --bin semio-os-mcp -- stdio|http`. This is the `"semio"` server in every `mcp.json`/`.mcp.json`/`.cursor/mcp.json`/etc. It is a working reference for how this repo structures a hand-rolled Rust MCP stdio/HTTP server and is a good structural template for porting the `repo`/Go MCP server.
- **SQL schemas**: `💻️client/🪶️sqlite/📐️schema.sql` (244 lines, client-local SQLite — contributor table etc., separate from anything above) and `🖥️server/🧬️schema/🐘️postgres/🗄️.sql` (887 lines, aspirational Postgres schema referenced by the TS coordinator's `.env.example`, not by the Go coordinator). Neither is used by the Go event-store coordinator described in §3.

## 6. Build / run / test entry points

| Entry | Command | What it does |
|---|---|---|
| Root `📜️script.ts dev mcp stdio <profile>` | `bun ./📜️script.ts dev mcp stdio {client\|cursor\|kiro\|copilot\|claude\|codex}` | `runMcpStdioRepo` → `buildRepoMcpClient(root)` = `go build -o <resolveMcpBin> ./🧰️.../💻️client/🔌️mcp` (env `GOWORK=<root>/go.work`), then runs the built binary with `SEMIO_REPO_MCP_CLIENT=<profile>`. `resolveMcpBin` defaults to `💻️client/mcp` (`mcp.exe` on Windows) — **note:** every dev-focused launch.json entry (`🛠️dev🧰️repo⌨️client`, native bootstrap scripts) instead does `go build -o …/💻️client/client[.exe] ./…/🔌️mcp/📦️packages/🐹️go` — a **binary-name mismatch** (`mcp[.exe]` vs `client[.exe]`) between the TS resolver and the ad-hoc build commands; a tracked `💻️client/mcp.exe` artifact exists in the tree from a prior `resolveMcpBin`-style build. |
| Root `📜️script.ts dev mcp stdio os` | → `runMcpOs("stdio", extra)` | `cargo run --quiet -p semio-framework-os-mcp --bin semio-os-mcp -- stdio` — the unrelated Rust "semio" OS gateway, launched by the same dispatcher. |
| `.mcp.json` (Claude Code, this session) | `bun ./📜️script.ts dev mcp stdio client` (repo) / `... stdio os` (semio) | Root project MCP config — matches the two connection failures reported at session start. |
| `.vscode/mcp.json` | `... stdio copilot` / `... stdio os` | |
| `.cursor/mcp.json` | `... stdio cursor` / `... stdio os` | |
| `.windsurf/mcp.json` | `... stdio client` / `... stdio os` | |
| `.codex/config.toml` | `... stdio codex` / `... stdio os` | |
| `.kiro/settings/mcp.json` | `... stdio kiro` / `... stdio os` | |
| `.vscode/launch.json` `"🛠️dev🧰️repo🤖️mcp"` | `bun run dev -- mcp repo` + MCP Inspector via `serverReadyAction` running `go run ./…/🔌️mcp/📦️packages/🐹️go` | Dev-time MCP Inspector wiring. |
| `.vscode/launch.json` `"🛠️dev🧰️repo🤖️mcp⌨️cursor"` | `bun ./📜️script.ts dev mcp stdio cursor` | |
| `.vscode/launch.json` `"🛠️dev🧰️repo⌨️client"` | `go build -o …/💻️client/client ./…/🔌️mcp/📦️packages/🐹️go && ./…/💻️client/client --help` (env `GOWORK`) | Ad-hoc smoke build, outputs `client[.exe]` (see mismatch note above). |
| `🔩️native/🥾️bootstrap/🐚️.sh` (line 580) / `🔵️.ps1` (line 867) | `go build -o …/💻️client/client[.exe] ./…/🔌️mcp` | Zero-touch bootstrap also builds the MCP binary as `client[.exe]`, called from `setup`/`start` dispatch. |
| nx target `repo-mcp` / `test-quick`/`test-long`/`test-exhaustive` (`💻️client/🔌️mcp/📋️project.json`) | `bun ./📜️script.ts test [level] repo-mcp` | Routes into the `🧪️test`-level Go test runner (`runRepoGoTest` in root `📜️script.ts`, filters `-run "Mcp|MCP|mcp"` against `./🔨️modules/💻️client/⌨️cli` — i.e. this target actually tests the CLI package's MCP-related tests, not `🔌️mcp/🧪️contract_test.go` directly by path). |
| nx target `repo-go-lib` (`📚️library/📦️packages/🐹️go/📋️project.json`) | `bun ./📜️script.ts test [level]` (cwd = that package dir) | Tests `📚️library/🐹️.go`. |
| nx target `@semio-tech/repo-client` (`💻️client/⌨️cli/📦️packages/🟦️typescript/📋️project.json`) | `dev`/`build`/`test*` → `bun ./📜️script.ts <cmd>` | This is a **TypeScript** package target name living alongside the Go CLI; scope not fully inspected — worth checking whether it wraps/tests the Go binary or is independent JS tooling. |
| nx target `@semio-tech/repo-coordinator` (`🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📋️project.json`) | `dev`/`start` → **`go run .`** (cwd = the TypeScript package dir, which has no `go.mod`) | **Bug/inconsistency**: `dev`/`start` targets for the TS coordinator invoke `go run .` inside the TS folder; there is no Go source there, so these targets are almost certainly broken/copy-paste leftovers from the Go coordinator's own project config. `test*` targets correctly call `bun ./📜️script.ts test [level]`. |
| `go.work` | n/a | Declares exactly 4 modules: `💻️client/⌨️cli`, `💻️client/🔌️mcp`, `📚️library`, `🖥️server/🎛️coordinator`. |
| `📚️library/🔌️nx-plugin/🟨️.mjs` | nx `createNodesV2` plugin | Auto-generates `breach-<slug>` lint targets for any `📜️script.ts` exporting `policy`; unrelated to Go/MCP building. |

## 7. Summary for the Rust-port ticket
- Clean layering to replicate in Rust: `repo-go` (types/events/parsing, stdlib-only) ← `repo-client` (CLI/domain logic, currently monolithic 46.9k-line file — a prime target to split into a taxonomy tree during the port) ← `repo-mcp` (protocol/transport/server, no 3rd-party MCP SDK) and, independently, `repo-server` (coordinator, own event store) ← `repo-go`.
- No TypeScript or existing Rust code implements the Go event kinds/payloads, the MCP tool/resource/prompt table, or the CLI's ticket/goal domain logic — porting is additive, not a merge, **except** the coordinator HTTP surface, which is duplicated in TS/Postgres and should be resolved to one implementation before/while porting to Rust.
- `⌨️cli/📦️packages/🦀️rust` (binary `semio`) is the intended home for "the dashboard" per its own doc-comment and has an established pattern (`🎮️commands/<slug>/🦀️.rs` modules included via `#[path]`) to extend with new command modules for ticket/goal/mcp.
- `💻️os/🔨️modules/🌉️mcp/🦀️.rs` is a working example of this repo's from-scratch Rust MCP stdio/HTTP implementation style to mirror for the repo MCP server.
- Fix-or-confirm before/at port time: (a) `client[.exe]` vs `mcp[.exe]` binary name mismatch across `resolveMcpBin`, ad-hoc `go build` launch entries, and native bootstrap scripts; (b) the `@semio-tech/repo-coordinator` TS project's `dev`/`start` targets calling `go run .` in a directory with no Go sources.
