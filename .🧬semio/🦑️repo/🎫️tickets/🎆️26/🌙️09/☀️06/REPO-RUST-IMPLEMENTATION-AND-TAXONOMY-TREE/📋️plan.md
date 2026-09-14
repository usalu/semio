# 📋️ Plan: Repo Rust Implementation and Taxonomy Tree

Coordinator: Claude Fable 5.1 (this session). Executors: Opus 5 agents. Explorers: Sonnet 5 agents.
Ticket folder: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE` (abbreviated `$TICKET` below).
Stable read-only copy of the Go sources as they were at ticket start: `$TICKET/🗑️generated/go-snapshot/{client,mcp,coordinator,library}`.
Exploration reports: `$TICKET/📓️explore-*.md`.

## 1. Outcome

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/` is a domain-driven, implementation-neutral tree: every module is `<emoji><domain-slug>/{🧬️schema, 🧪️tests, 🧫️fixtures?, 📦️packages/{🐹️go,🦀️rust,🟦️typescript?}}`. No `internal/`, `cmd/`, OS-suffixed files, language-named domain folders, or committed binaries.
2. Every Go domain has a Rust twin crate with the same behaviour. Both are exercised by the same language-agnostic tests (Protocol v2 harness in `🔨️modules/🧪️test`: `🥒️.feature` + `🧫️fixtures` + one adapter per language), with a third-party oracle where one exists.
3. The `semio` binary (Rust) carries the whole repo CLI surface and the MCP server. Every dev entry point (`.mcp.json` and the IDE `mcp.json` files, `launch.json`, nx targets, hooks, TypeScript library binary resolution, VS Code extension) uses the Rust implementation by default; `SEMIO_REPO_IMPLEMENTATION=go` selects the Go implementation.
4. The terminal dashboard shows repo-domain commands (tickets, goals, analyze, tree, statutes, …) provided in-process by the Rust domain crates.

## 2. Target tree (`🧰️framework/🛍️products/🦑️repo/🔨️modules/`)

Legend: [MOVE] existing content relocated, [NEW] net new, [KEEP] unchanged. Go module path prefix: `github.com/usalu/semio/repo/`. Rust crate name prefix: `semio-framework-repo-`.

| Module | Go package (module path suffix) | Rust crate suffix | Content | Source (Go snapshot) |
| --- | --- | --- | --- | --- |
| `🏠️workspace` | `workspace` | `workspace` | repo-root discovery (`.🧬semio`), layout constants (`🦑️repo`, `🎫️tickets`, `🎯️goals`, `🧑️‍💻️devs`, `📁️files.json`), `RepoConfig` (`📋️config.toml`), ignore matching, glob matching | Types→Utils/RepoConfig 16751–17777, `internal/ignore`, `internal/glob` [MOVE] |
| `🪪️identity` | `identity` | `identity` | entity emojis, semantic ids, URIs (`repo://`), artifact refs, compose-id codecs, humanize, id generation, leading-grapheme rules | Entity Rendering/Artifact ID 41084–43194, `internal/id`, `internal/humanize`, `AllEntityEmojis` [MOVE] |
| `🧾️yaml` | `yaml` | `yaml` | hand-rolled YAML codec | `internal/yaml` [MOVE] |
| `🔎️search` | `search` | `search` | content search | `internal/search` [MOVE] |
| `📐️model` | `model` | `model` | domain model types (former GraphQL Types: Node, Repo, Technology, Bundle, Folder, File, Section, Definition, Ticket*, Goal*, Breach*, Contributor*, Checkpoint*, Draft*, Todo*, inputs), slug normalisation (`NormalizeLLMSlug`…), Codebase Types, Tree/Goal/Ticket node models | GraphQL Types 9881–12346, Models 5315–5555, Codebase Types [MOVE] |
| `🗣️languages` | `languages` | `languages` | `LanguagePlugin` per language (TypeScript, Go, C#, JSON, Markdown, Rust, Ruby, Shell, TOML, YAML, SQL, GraphQL), section/definition parsing, region markers, headers, comment formats | Types→Languages 13335–16749, Types→Sections 17779–18420, parsing primitives of `library/🐹️.go` [MOVE] — schema-first: language table lives in `🧬️schema/🔣️.json`, both implementations load it |
| `📡️events` | `events` | `events` | event kinds, payloads, envelope, emit to coordinator, append-only JSONL event store, event export | `library/🐹️.go` (event part), `internal/eventstore`, `📤️event_export.go` [MOVE] |
| `🗂️codebase` | `codebase` | `codebase` | bundle/folder/file walk, `CodebaseContext`, loaders, ID/URI builders for folders/files/sections | Codebase 21615–22700, ID builders from Cli region [MOVE] |
| `🎫️tickets` | `tickets` | `tickets` | ticket lifecycle (open/close/reopen/change/search/purge), important document transaction, GitHub issue/milestone/label DTOs, ticket file resolution, `CanCloseTicket`, `ComputeTicketFiles` | Tickets 22702–27552, ticket parts of Cli region [MOVE] |
| `🎯️goals` | `goals` | `goals` | goal list/read/open/close/reopen/change, goal tree | Goals 39633–40528 (goal part), goal tree from Tree Logic [MOVE] |
| `🧑️contributors` | `contributors` | `contributors` | contributors/devs, sessions, checkpoints, interactions | Contributor + Sessions + Checkpoints regions [MOVE] |
| `📝️todos` | `todos` | `todos` | todos and drafts | Todos 40753–41082, Drafts [MOVE] |
| `📜️statutes` | `statutes` | `statutes` | policies/statutes/breaches: analyze, fix, ignore directives, breach cache (gzip+sha256 envelope) | Policies 18420–21613, BreachCache [MOVE] |
| `🌳️tree` | `tree` | `tree` | monorepo tree, statute tree, territory tree, filters, sorting, Mermaid rendering | Tree Logic 5557–7712, Mermaid [MOVE] |
| `📊️metrics` | `metrics` | `metrics` | LOC history (git log numstat), benchmark | LOC 7714–9877 part, Benchmark [MOVE] |
| `🧩️providers` | `providers` | `providers` | management (GitHub via `gh`), version control (git), sandbox (devcontainer), editors (Copilot, Cursor, Windsurf, Claude, Droid, Codex, Antigravity, Kiro), provider registry; all process execution behind an interface | Providers 12348–13190 [MOVE] |
| `🔗️graphql` | `graphql` | `graphql` | query parser, executor, schema builder, resolvers, `RepoContext` port + default context | `internal/graphql`, Executor 30605–33474, Resolvers 33476–34659, Context 27554–30603 [MOVE] — the 2763-line `buildSchema` is split into per-aggregate schema fragments |
| `🚚️move` | `move` | `move` | move/copy files, integrate, extract, rename with casings, section move | Rename 5099–5291, File Utilities, integrate/extract/section-move command logic [MOVE] |
| `🪝️hooks` | `hooks` | `hooks` | agent hook events (`HookResult*`), per-IDE hook resolution, tool blocking, plan/spec source resolution, micro-commit reset | Hooks 37586–39067, Missing Hook Functions 43340–44483 + 45906–46896 (hook part), Cli hook types [MOVE] |
| `🏃️test-runner` | `testrunner` | `test-runner` | detection and execution of go/bun/uv-pytest/cargo/dotnet/rspec tests per scope | Test Command 1513–5099, test-file resolution [MOVE] |
| `🔌️mcp` | `mcp` | `mcp` | JSON-RPC/MCP protocol types, session, stdio transport, server, hash-chained event log, per-IDE profiles, tools/resources/prompts, descriptions | `💻️client/🔌️mcp/*.go`, `internal/mcp`, `internal/mcpserver`, Mcp region 34661–36087, `CreateMcpServer` [MOVE] — `initialize` MUST ignore unknown members (defect recorded in `$TICKET/📓️baseline-and-mcp-initialize-defect.md`) |
| `⌨️cli` | `cli` | `cli` (bin `semio`) | command framework (former `internal/command`), engine + event stream, renderers (NDJSON/human/markdown), templates, `RunCLI` command tree, usage; Rust crate is the existing `semio` orchestrator crate extended with every repo verb and `mcp` verb | `internal/command`, `internal/templatefunc`, Engine 195–602, Renderers 7714–9877, command factories, `cmd/repo`, existing `⌨️cli/📦️packages/🦀️rust` [MOVE+MERGE] |
| `🎛️dashboard` | — | `dashboard` | terminal dashboard, daemon, IPC, command-tree discovery, workflow, plugin registry, playground session/catalog, root-script delegation, usage presentation | `🎮️commands/*` [MOVE] — Rust only (TUI), consumes the Rust domain crates in-process |
| `🧩️vscode` | — | — (TypeScript) | VS Code extension, resolves the `semio` binary | `💻️client/🧩️vscode` [MOVE] |
| `🪶️sqlite` | — | — | client-local SQLite schema | `💻️client/🪶️sqlite` [MOVE] |
| `🖥️server/🎛️coordinator` | `coordinator` | `coordinator` | event store (canonical JSONL, sha256 checksum), durability (`🛡️durability/` with per-OS targets), HTTP API, projection, webhooks; deployment files under `🚀️deploy/` | `🖥️server/🎛️coordinator/*.go` [MOVE]; the Next.js TypeScript package stays as `📦️packages/🟦️typescript` |
| `📚️library` | — | — | TypeScript build/lint/test tooling, discovery, normalization, nx plugin | [KEEP]; the Go `🐹️.go` leaves for `📡️events`/`🗣️languages` |
| `🧪️test` | `testhost` | `test-host` | Protocol v2 harness | [KEEP] |
| `🔩️native` | — | — | bootstrap | [KEEP], build lines updated |

Removed at the end: `💻️client/`, `🎮️commands/`, top-level `🔨️modules/⌨️cli` duplicate (merged), root `./repo/` legacy tree, tracked binaries (`client.exe`, `mcp.exe`, `server.exe`, `client-win-test.exe`).

### Dependency DAG (lower may not import higher)

```
L0  🏠️workspace  🪪️identity  🧾️yaml  🔎️search  📐️model
L1  🗣️languages  📡️events  🧩️providers
L2  🗂️codebase  📜️statutes  📊️metrics
L3  🎫️tickets  🎯️goals  🧑️contributors  📝️todos  🌳️tree  🚚️move  🏃️test-runner  🪝️hooks
L4  🔗️graphql
L5  🔌️mcp  ⌨️cli
L6  🎛️dashboard
```
Validated against the symbol graph in `$TICKET/📓️go-region-dependency-graph.md`; any edge that violates the DAG is broken by moving the shared symbol down or by a port interface in the lower module.

## 3. Package conventions (binding for every executor)

- Go package: `📦️packages/🐹️go/{go.mod, 🐹️.go, 🧪️_test.go (tests; Go only compiles `*_test.go`), 📋️project.json, 📜️script.ts}`. `go.mod`: `module github.com/usalu/semio/repo/<suffix>`, `go 1.25`, no external deps. Root `go.work` lists every module. Test adapters for the harness live under the module's `🧪️tests/<case>/🐹️.go`.
- Rust crate: `📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}`; `name = "semio-framework-repo-<suffix>"`, `[lib] path = "🦀️.rs"`, `[lints] workspace = true`, `[package.metadata.semio] role = "library"` (`"tool"` for the cli), dependencies only `serde`/`serde_json` (`.workspace = true`) plus sibling path crates. Root `Cargo.toml` lists every crate. No other external runtime crates; SHA-256, gzip, regex-like matching are hand-rolled inside the owning module and validated by tests against a third-party oracle (test-only dependency, allowed under `[dev-dependencies]`).
- Godfile sections: `// #region 🔖️Name` … `// #endregion 🔖️Name`, one emoji-prefixed docstring per item, no comments inside definitions, `[DEBUG] ` prefix on temporary logs.
- `📋️project.json` calls only `bun ./📜️script.ts <cmd> …`; `📜️script.ts` extends `BundleScript`/`ScriptRouter` from `🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`. Rust test target uses `runCargoTestBudgeted`; Go test target uses the shared Go runner in the library.
- Schema: `🧬️schema/🔣️.json` (JSON Schema draft 2020-12) per module describing its fixtures and wire shapes; implementations load data tables (languages, event kinds, mcp descriptions, entity emojis) from schema-adjacent `🔣️.json` collections at build time (`include_str!` / `//go:embed`) so both languages share one source of truth.
- Language-agnostic tests: `🧪️tests/<case-slug>/{🥒️.feature, 🧫️fixtures/, 🐹️.go, 🦀️.rs}` following `🔨️modules/🧪️test/README.md` and the `🖥️host-protocol-parity` example; the owner's contribution manifest (`🔮️oracle/🔣️.json`, the harness's real convention — see `📓️opus-graphql-parser.md`) names the third-party oracle (TypeScript oracle adapters `🟦️.ts` using a library such as `yaml`, `micromatch`, `graphql`, `@modelcontextprotocol/sdk`, Node `crypto`/`zlib`, `simple-git`) or a `noOracleDecisions` entry. Every scenario carries `@id-`, one `@level-`, one `@mode-`.
- launch.json: edit `.vscode/🧩️launch.seed.jsonc` only, then regenerate with `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`; follow existing grouping (`3_dev`, test groups) and naming (`🛠️dev🧰️repo…`, `🧪️test🧰️repo…`).
- Do not run `git commit`, `git stash`, `git checkout`, worktrees. Other fleets edit the repo concurrently; ignore unrelated changes.
- Temporary files, logs and command output go to `$TICKET/🗑️generated/<agent-slug>/`. Every executor writes `$TICKET/📓️opus-<slug>.md` with what was done, how it was verified (real command output), and what is left.
- Cargo on this host: set `RUSTC_WRAPPER=""` (sccache is configured but not installed); `devToolingEnv()` does this for nx-driven runs.

## 4. Waves

| Wave | Agent | Scope |
| --- | --- | --- |
| 1 | Sonnet `go-region-dependency-graph` | Go AST tool in `$TICKET/🔨️go-symbol-graph/` mapping every top-level decl of `component.go` to a region and the target module of §2; region×region reference graph; SCC/cycle report with the symbols causing them |
| 1 | Sonnet `harness-verification` | Run the Protocol v2 harness for `host-protocol-parity` with Go and Rust; document exact working commands, breakages, how to add a case with Go+Rust adapters and a TypeScript oracle |
| 1 | Opus `mcp` | `🔨️modules/🔌️mcp`: move Go MCP module, build Rust MCP crate (protocol, session, stdio transport, server, event log, profiles, descriptions), `Repository` trait for domain handlers, language-agnostic tests from g2 fixtures + lenient initialize + MCP SDK oracle |
| 1 | Opus `foundation` | `🏠️workspace`, `🪪️identity`, `🧾️yaml`, `🔎️search`: move Go internals, Rust crates, tests from g1 fixtures + oracles |
| 1 | Opus `events` | `📡️events`: Go library + eventstore + export moved, Rust crate, tests (event kinds/payload schema, JSONL store, checksum) |
| 1 | Opus `graphql-parser` | `🔗️graphql` parser/lexer/validator (from `internal/graphql`) in Go (moved) and Rust, tests with `graphql` npm oracle |
| 2 | Opus `go-split` | Scripted AST split of `component.go` + tests into the §2 Go packages (single agent, sequential) |
| 2 | Opus per domain (`model`, `languages`, `codebase`, `statutes`, `tree`, `tickets`, `goals+contributors+todos`, `providers`, `move+test-runner+hooks`, `metrics`) | Rust crates from the Go snapshot + language-agnostic tests + Go adapters |
| 3 | Opus `graphql-executor`, `cli`, `dashboard`, `coordinator` | Rust executor/context, `semio` verbs + `mcp` verb, dashboard integration, coordinator Rust |
| 4 | Opus `wiring` | `.mcp.json` + IDE configs, launch seed + regenerate, nx, hooks, TS binary resolution, VS Code extension, bootstrap, delete legacy trees |
| 5 | Sonnet `audit` | Independent verification: build both, run every harness case for both implementations, parity, statutes (`analyze`) on the new tree |

## 5. Coordinator decisions (running log)

- 2026-09-06 06:10 — Test case directories under `🧪️tests/` MUST carry a leading emoji identity like every other taxonomy node (reference: `🧪️test/🧪️tests/🖥️host-protocol-parity`). The `testCaseSlugPattern` in `🔣️taxonomy.json` is being changed by the `statute-hygiene` executor to require it. Executors that created plain kebab-case case directories (`🔗️graphql`, `🗣️languages`, `📡️events`, `📐️model`, …) are NOT to be re-run; the audit wave renames those directories and their references in one sweep and re-runs parity.
- 2026-09-06 06:10 — Oracle manifests live at `<owner>/🔮️oracle/🔣️.json`; oracle ids are owner-scoped (`<tool>-repo-<owner>`), because the registry resolves an id to its first match.
- Harness runs on this Windows host need `SEMIO_TEST_BUDGET_MS=600000` for the first (cold `go run`) execution.

- 2026-09-06 — `🧪️test/📦️packages/🐹️go` IS listed in `go.work` (plan §3 holds, no deviation). Measured both ways with `discover` + `parity fundamental --owner 🪪️identity` and `--owner 🔌️mcp`: identical results (identity 5/5 parity 4/4; mcp 12/12 parity 6/6, with the same pre-existing Rust `semio-framework-repo-mcp` E0308 in `repository_server` failing two rust subject hosts in both runs). The generated hosts cannot be affected: `goWorkspaceModules` and `goSutModule` in `🧪️test/📜️script.ts` both skip the module named `semio.tech/repo/test`, so it never reaches a generated `go.mod`, and every host runs with `GOWORK=off`. The host `go.mod` was raised from `go 1.23` to `go 1.25` to match the workspace. — `schema-cleanup` executor
