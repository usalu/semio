# Go repo-client inventory (component.go + internal/* + event_export.go + cmd/repo)

Target: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component.go` (46,896 lines, single `package client`), its `internal/*` helper packages, `📤️event_export.go` (169 lines), `cmd/repo/🐹️.go` (19-line `main`), tests `🔬️component_test.go` (23,901 lines, 568 `Test*` funcs, Go-only) and `🤝️g1_contract_test.go` (641 lines, JSON-fixture-driven contract test — the one language-agnostic pattern already in place, fixture `🧫️fixtures/1️⃣g1-contract.json`).

All imports are Go stdlib plus the file's own `internal/*` packages and `repo/go` (sibling product) — **zero external runtime deps**, consistent with the "no external runtime dependencies" rule.

## 0. Structural finding: region nesting is broken

`#region`/`#endregion` markers are unbalanced at the top level. A single depth-1 region **`🌧️Cli Adapter` spans lines 604–40751 (40,147 lines — 86% of the file)** and contains almost everything (commands, GraphQL types/resolvers, providers, tickets, MCP, hooks, CLI rendering). Two more depth-1 siblings follow it: `📰️Todos` (40753–45904) and `🔖️Missing Hook Functions` (45906–46896, itself containing a second, differently-scoped `🤸️Preamble`/`McpClientKind`/`ResolvePlanSource`/`MoveTicketPlan`/`TicketPlanComment`/`McpDescriptions`/`McpServerFactory`/`TicketMcpHandlers` sub-tree — this looks like a later, bolted-on addition, not organically nested under `Cli Adapter`). There is also an orphaned `// #endregion 🧊️Policies` at line 21613 with no matching open marker (the policy-engine content 18420–21613 is unlabeled). **Practical consequence for planning:** ignore depth-1 as a grouping signal; the depth-2 children of `Cli Adapter` (and the depth-1 `Todos`/`Missing Hook Functions` siblings) are the real domain boundaries and are used below.

## 1. Domains (from depth-2/3 regions), with line ranges, size, key symbols, deps

| Domain (region) | Lines | Size | Key exported types/funcs | External deps | Depends on other domains |
|---|---|---|---|---|---|
| Header/Preamble/Templates | 1–193 | S (193) | `templateFuncMap`, `renderTemplate`, `init` | `text/template` (only use in file) | — |
| Engine Events/Errors/Requests/Engine | 195–602 | S (408) | `Event`,`Progress`,`Artifact`,`ErrPayload`,`DonePayload`,`Command`,`Request`,`Engine`,`NewEngine`,`(*Engine).Run` (returns `<-chan Event`) | goroutines/channels | GraphQL Executor |
| Auth/Analyze/EntityEmojis commands | 759–1111 | S | `authCommand`,`analyzeCommand`,`AllEntityEmojis`,`entityEmojisCommand` | os/exec (auth), regexp | command pkg, Engine |
| Test Command (top) | 1513–5099 | L (3586) | `resolveTestScopes`,`runAllTests`,`runTechnologyTests`,`runBundleTests`,`detectJSTestRunner`,`collectGoTestsInSection` | **os/exec** (go/bun/uv/pytest/rspec/dotnet/cargo test runners), regexp | Types/Languages, Sections |
| Rename | 5099–5291 | S (192) | `ToolRename`,`renameCommand`,`applyRenameCasings` | regexp | Sections/Definitions |
| Utilities (misc) | 5293–5313 | S | `parseFlexibleTime` | time | — |
| Models (Tree/Goal/Ticket nodes) | 5315–5555 | S (240) | `TreeNode`,`TreeFilter`,`GoalNode`,`TicketNode`,`TreeNodeKind` | — | Tickets, Goals |
| Tree Logic | 5557–7712 | XL (2155) | `BuildMonorepoTree`,`PropagateParentIDs`,`buildGoalTree`,`renderGoalTree`,`buildStatuteTree`,`buildTerritoryTree`, query/tree caches (`cacheMeta`) | sync (mutex/cache) | Codebase, Tickets, Goals, GraphQL Types |
| CLI Renderers (ANSI, LOC, Mermaid) | 7714–9877 | XL (2163) | `StreamRenderer`,`NDJSONRenderer`,`HumanRenderer`,`MarkdownRenderer`,`LocReport`,`renderStream`,`formatResult`,`toolResultFromEvents`,`toolResultFromTreeRender` (LOC command alone is 1108 lines: git-log-based line-count history) | os/exec (`git log`), regexp, ANSI escapes | Engine, Tree Logic |
| GraphQL Types | 9881–12346 | L (2465) | 66 types incl. `Node`,`Repo`,`Bundle`,`Technology`,`Ticket*`,`Breach*`,`Definition*`; `DeriveDefinitionKind`, `NormalizeLLMSlug/EffortSlug/ClientSlug`, `ResolveAllowedLLM/Effort/Client` (Drafts, GraphQL Input Types nested) | — | consumed by nearly everything below |
| Providers (mgmt/vcs/sandbox/editor) | 12348–13190 | L (842) | `GitHubManagementProvider` (11 exec.Command-based GitHub CLI calls), `GitVersionControlProvider`, `DevcontainerSandboxProvider`, 7 `*EditorProvider` structs (Copilot/Cursor/Windsurf/Claude/Droid/Codex/Antigravity/Kiro), `ProviderRegistry` | **os/exec** (`gh`, `git`), **net/http** (2 call sites, likely `gh api`/webhooks) | GraphQL Types |
| Types → Languages | 13335–16749 | XL (3414) | `LanguagePlugin` interface + `BaseLanguage` + 12 language impls (TypeScript/Go/C#/JSON/Markdown/Rust/Ruby/Shell/TOML/YAML/SQL/GraphQL) each with own `sectionStart/End` **regexp.MustCompile** pairs, comment/header formatting, `Codebase Types` (⭐️, 303 lines, 36 types, 0 funcs — pure schema) | 92 total `regexp.MustCompile` call sites in file, most inside this region | Sections |
| Types → Utils / RepoConfig | 16751–17777 | L (1026) | `GetRootDir/SetRootDir/GetSemioRootDir/GetRepoMetaDir`, `RepoConfig`, `LoadRepoConfig`, `LoggingConfig` | filesystem walk to find `.🧬semio` | — (foundational; almost everything depends on this) |
| Types → Sections | 17779–18420 | M (641) | `ParseCodeSections`,`ParseMarkdownSectionsInternal`,`ParseJSONSections(Detailed)`,`ParseSections`,`ParseDefinitions`,`HydrateSectionsWithDefinitions`,`NormalizeSectionPath` | — | Languages |
| Types → Policies (unlabeled 18420–21613) | 18420–21613 | XL (3193) | `PolicyFunc`,`PolicyContext`,`FindPolicy`,`GetPolicies`,`StreamPolicies`,`ParseIgnoreDirectives`,`CreateBreach`,`FilterIgnored`, path/emoji-taxonomy enforcement (`pathEmojiTaxonomy`, `Godfile`) — this is the **analyze/fix statute engine** | — | Codebase, Sections |
| Codebase | 21615–22700 | L (1085) | `CodebaseContext`, `LoadBundles/Files/Breachs/Tickets/Policies`, `BuildCodebaseBundles/Folders`, ID/URI builders | filesystem walk, `ignore` pkg | RepoConfig, Languages |
| Tickets | 22702–27552 | XL (4850, largest single domain) | `GetTicketsDir/Path`, ticket "important document" transactional create/rollback, `ghIssue/Milestone/Label` DTOs, `FindTicketBySlug`, `LatestTicket`, `StreamOptions`, ticket file resolution sub-region | os/exec (gh), sha256 (content hashing likely) | Providers(GitHub), Codebase, GraphQL Types |
| GraphQL Context Port/Resolver | 27554–27635 | S | `RepoContext` interface (the port), `Resolver` | — | everything (interface boundary) |
| Default Context | 27637–30603 | XL (2966) | `defaultContext`/`repoContext` impl of `RepoContext` (100+ methods: Get* for every entity), `BreachCache` sub-region (envelope-based cache) | sha256/gzip (2 each — cache envelope compression+checksum) | Codebase, Tickets, Goals, Providers |
| GraphQL Executor/Schema Builder | 30605–33474 | XL (2763, mostly one 2763-line `buildSchema` func) | `Executor`,`NewExecutor`,`Execute/ExecuteJSON/ValidateQuery` — **hand-rolled GraphQL engine**, no external graphql-go dep | — | Resolvers, GraphQL Types |
| Query/Mutation/Entity Resolvers + interfaces | 33476–34659 | L (1177) | `queryResolver`,`mutationResolver` (Ticket/Goal/Todo/Draft/Bundle CRUD-ish mutations, `Fix`, `SyncManagement`), `repoResolver` field resolvers | — | RepoContext |
| Mcp (args/paths/graphql/handlers) | 34661–36087 | L (1426) | tool-arg helpers (`getStringArg`,`requireFileTargetPath`…), 4 **prompt** handlers (enhance/refactor/test/comply), 26 **resource** handlers (`handle*Resource` for bundles/folders/files/sections/definitions/tickets/goals/policies/statutes/contributors/checkpoints, singular+plural) | — | GraphQL Executor, internal/mcp, internal/mcpserver |
| Cli (hooks types, diff/breach id, benchmark) | 36089–39631 | XL (3542) | `GitDiffStatus`,`BenchmarkResult`, ~20 `HookResult*` event types (agent lifecycle: Started/Ended/ToolStarting/ToolEnded/Compacting/PlanUpdating/SearchStarting…), `ScopeToFiles`,`ComputeTicketFiles`,`GetGitDiffLines`,`CanCloseTicket`, ID builders (`buildFolderID`,`buildFileID`,`buildSectionID`), plus nested `hookCommand`,`configureCommand`,`microCommitCommand` | os/exec (git diff, bun install) | Tickets, GraphQL Types, Codebase |
| Hooks | 37586–39067 | L (1481) | agent-hook dispatch (resolves native IDE hook events → `HookEvent`) | — | Cli (HookResult types), Providers(Editor) |
| File Utilities / Server Client / Goals(+Sessions) | 39633–40528 | L (891) | `MoveFile/CopyFile`; `serverRequest/serverWhoami` (**net/http** client to a `COMPOSE_SERVER_ADDR`); `ListGoals/ReadGoal/StreamGoals/StreamStatutes/StreamCheckpoints`, `Session`,`StreamSessions`,`SessionKindEmoji`,`DeriveSessionKind` | net/http | Codebase |
| Todos | 40753–41082 (+ resolver method at top) | S (330) | `ScanTodos`,`ParseTodoMarkdown`,`ParseTodoComments`,`TodoCreate/Change/Delete`,`TodoToTicket` | — | Sections/Definitions |
| Entity Rendering / Artifact ID | 41084–43194 | XL (2110) | `SemanticId`,`ArtifactRef`, emoji↔entity mapping (`emojiText`,`extractEntityEmoji`,`resolveTechnologyEmoji`, per-kind emoji funcs), goal/contributor compose-ID codecs | unicode/utf8 (rune-level emoji scanning) | GraphQL Types, Types/Utils |
| Missing Hook/Test/Utility Functions | 43340–44483 | L (1139) | per-IDE hook-event resolvers (Copilot/Cursor/Windsurf/Claude/Kiro), test-file resolution per test framework (go/cargo/dotnet/pytest/rspec/JS), tool-block policy (`IsToolBlocked`,`containsBlockedGitInCode`) | regexp | Hooks, Test Command |
| Missing Hook Functions (top, bolted-on tail) | 45906–46896 | L (990) | `McpClientKind` enum + `ParseMcpClientKind`,`McpServerName`; `ResolvePlanSource`,`MoveTicketPlan`,`TicketPlanComment` (+ `formatPlanComment`,`postTicketPlanComment`); `mcpDesc`/`McpDescriptions` (i18n-ish description table); **`CreateMcpServer`** (the actual MCP wiring: 4 prompts, ~13 resources, only 6 tools) | — | Mcp, Providers(Management), Tickets |

## `internal/*` packages (all small, all stdlib-only)

| Package | Lines | Purpose |
|---|---|---|
| `internal/id` | 44 | ID/emoji helpers |
| `internal/humanize` | 48 | human-readable formatting |
| `internal/templatefunc` | 69 | shared `text/template` funcs |
| `internal/ignore` | 83 | `.gitignore`-style matching |
| `internal/glob` | 168 | glob matching (own impl, no external lib) |
| `internal/yaml` | 485 | **hand-rolled YAML encode/decode** (no external YAML lib) |
| `internal/search` | 589 | text/content search |
| `internal/graphql` | 695 | GraphQL query parsing support for the hand-rolled executor |
| `internal/command` | 676 + 151 test | own **Cobra-like** command/flag framework (`command.Command`) — not using spf13/cobra |
| `internal/mcp` | 210 | hand-rolled MCP wire types (`Tool`,`Prompt`,`CallToolRequest/Result`, `NewTool/NewResource/NewPrompt` builders) |
| `internal/mcpserver` | 149 | hand-rolled MCP JSON-RPC **stdio** server loop (`ServeStdio`, `tools/list`, `tools/call`, `initialize` — no SDK) |
| `internal/eventstore` | 454 + 102 test | append-only JSONL event log (used by `event_export.go`) |

`📤️event_export.go` (169 lines): `ExportToEventLog(Context)` walks a `RepoContext` and writes technologies/bundles/folders/files/sections/definitions as `eventstore.Input` records into `repo.events.jsonl` — sha256 used for content addressing.

`cmd/repo/🐹️.go`: 19-line `main()` that just calls `client.RunCLI()`.

## 2. CLI command surface (`RunCLI`, 36 top-level commands registered on `root`)

Top-level: `auth` (login/logout/whoami-ish), `sync` (→ `github`,`management`), `mcp`, `graphql`, `test`, `ticket` (→ create/change/delete/search + open/close/reopen/purge-artifacts), `todo` (→ create/delete), `goal` (→ change/open/close/reopen), `contributor` (list), `folder`/`file`/`section`/`definition` (→ list/tree, create/move/delete, + `section` also integrate/extract), `move`, `integrate`, `extract`, `rename`, `search`, `list`, `query`, `export`, `hook`, `mermaid`, `loc`, `technology` (→ list/tree, add/remove), `bundle` (→ list/tree), `analyze`, `entity-emojis`, `configure`, `micro-commit`, `checkpoint` (list/tree), `statute` (list/tree), `interaction`, `draft` (create/delete), `benchmark`(top-level, not factory-built), `update`(top-level).

Output formats: `NDJSONRenderer` (streamed JSON events), `HumanRenderer` (ANSI/TTY colorized), `MarkdownRenderer` — selected in `renderStream`; plus `formatResult`/`formatMarkdownResult` for one-shot GraphQL/query output (JSON vs Markdown), and a raw `ToolResult{}` shape reused for MCP tool responses (`toolResultFromEvents`, `toolErrorResult`).

## 3. MCP tool/resource/prompt surface

- **Prompts (4):** `enhance`, `refactor`, `test`, `comply` — each takes one required `prompt` string argument, description resolved via `mcpDesc(kind, key)` (per-IDE text variants keyed by `McpClientKind`: generic/copilot/cursor/windsurf/claude/droid/codex/antigravity/kiro).
- **Resources:** collection + `{id}`-templated pairs for `repo://` (root), `bundles`/`bundle/{id}`, `folders`/`folder/{id}`, `files`/`file/{id}`, `sections`/`section/{id}`, `definitions`/`definition/{id}`, `tickets`/`ticket/{id}`, `goals`/`goal/{id}`, `policies`/`policy/{id}`, `statutes`/`statute/{id}`, `contributors`/`contributor/{id}`, `checkpoints`/`checkpoint/{id}` — 12 collections × ~2 = ~26 URIs, all `text/plain`.
- **Tools: only 6** — `ticket_open`, `ticket_close`, `ticket_reopen`, `section_move`, `file_integrate`, `section_extract`, each wrapped with `wrapMcpToolHandler` (timeout via goroutine+channel race). **Gap:** the CLI exposes 36 top-level commands (goal/todo/technology/bundle/analyze/query/graphql/…) but MCP only exposes 6 mutating tools plus read-only resources — no MCP tool for `goal_*`, `todo_*`, `technology_*`, `analyze`, `query`, `rename`, `integrate` (file-level), etc. This asymmetry is a real gap for the planner to size, not just a porting artifact.
- MCP transport is a **custom stdio JSON-RPC loop** (`internal/mcpserver.ServeStdio`) implementing only `initialize`, `tools/list`, `tools/call` — no `resources/read`/`prompts/get` dispatch visible in that loop (resources/prompts are registered but the stdio loop shown doesn't route to them — worth re-checking at implementation time, since `AddResource`/`AddPrompt` exist but `ServeStdio`'s `switch` only handles the three methods above).
- Sibling module `🔨️modules/💻️client/🔌️mcp` (separate Go module, own `go.mod`, files `📜️protocol.go`,`📡️event.go`,`🖥️server.go`,`🗄️repository.go`,`🚚️transport.go`, `🧩️component.go` only 72 lines) imports `github.com/usalu/semio/repo/client` — i.e. it's a **second, thinner MCP-server front end** wrapping the same client package; treat as part of the taxonomy migration too, not a duplicate to delete.

## 4. Hardcoded repo paths / config keys

- `.🧬semio` root marker: found/walked via `GetRootDir`/`GetSemioRootDir` (Types→Utils region), then joined with literals `"🦑️repo"`, `"🎫️tickets"`, `"🎯️goals"`, `"🧑️‍💻️devs"`, `"📁️files.json"` at ~18 call sites (e.g. `.🧬semio/🦑️repo/🎫️tickets/`, `.🧬semio/🦑️repo/🧑️‍💻️devs/`, `.🧬semio/🦑️repo/📁️files.json`).
- `GetTicketsDir`/`GetRepoGoalsDir` compose these under a resolved root rather than re-hardcoding — good; the raw literals above are the few remaining hardcode points to externalize during the Rust port (should become one shared "layout" constant module).
- Env/config keys read via `os.Getenv`: `NO_COLOR`, `COMPOSE_BUN`, `BUN_INSTALL`, `COMPOSE_SERVER_ADDR`, `COMPOSE_SERVER_TOKEN`, `COLUMNS`. `RepoConfig`/`LoadRepoConfig` (16832–16937) parses a repo-level config file (own tiny parser, `parseRepoConfigBool`/`unquoteRepoConfigValue` — not using any TOML/YAML lib even though `internal/yaml` exists, inconsistency to flag).

## 5. Go-specific mechanisms needing deliberate Rust design

| Mechanism | Count in component.go | Rust design note |
|---|---|---|
| `text/template` | 1 use site (`initTemplates`/`renderTemplate`) | small — replace with a stdlib-only template mechanism or explicit string building; avoid pulling in `askama`/`tera` per no-external-runtime-dep rule |
| goroutines (`go func`) | 51 | mostly one-shot "race against timeout/cancel" patterns (MCP tool timeout, Engine.Run) — map to `std::thread::spawn` + `mpsc`/`crossbeam` channel or async tasks; decide sync-vs-async CLI model up front |
| channels (`make(chan`/`<-chan`) | 44 / 6 | `Engine.Run` returns `<-chan Event` — this is the core streaming-event architecture (CQRS event stream out of the engine); Rust equivalent likely `std::sync::mpsc::Receiver<Event>` or an iterator |
| `os/exec` | 11 direct + more via Providers (`git`, `gh`, `bun`, `uv`, test runners) | needs a Rust process-exec abstraction behind an interface (per CLAUDE.md "external tools behind an interface") — this is already effectively how Providers are structured (interfaces `ManagementProvider`/`VersionControlProvider`/`SandboxProvider`/`EditorProvider`), good precedent to keep |
| `net/http` | 2 (Providers, presumably GitHub API) + `serverRequest`/`serverWhoami` (Server Client region) | Rust: stdlib doesn't have HTTP client — this is one of the few places an external crate (or a hand-rolled minimal HTTP/1.1 client, consistent with "no external runtime deps") is unavoidable; flag for architecture decision |
| `regexp.MustCompile` | 92 | heavy in Languages (per-language section/definition patterns) and Policies; Rust stdlib has no regex — either hand-roll matching (preferred per no-external-dep rule) or isolate behind an interface if `regex` crate is used as an "existing library to test our implementation against" per rule |
| `compress/gzip` | 2 | BreachCache envelope compression — Rust: `flate2` behind interface, or hand-rolled |
| `crypto/sha256` | 2 (+event_export.go) | content-addressing/cache checksums — Rust stdlib has no SHA-256; needs own impl or interface-wrapped crate, consistent with "test against a third-party lib" rule |
| `encoding/csv` | 1 | minor, easy to hand-roll |
| `sync.Mutex/RWMutex/WaitGroup` | 6 | Tree/Query caches — straightforward `Mutex`/`RwLock` mapping |
| reflection | 0 in component.go (1 `reflect` import only in `g1_contract_test.go` for test comparison) | no reflection-driven runtime behavior to replicate — good, simplifies the port |
| Hand-rolled subsystems already avoiding external deps (good precedent to preserve in Rust) | GraphQL engine (parser+executor+schema builder, ~6.6k lines total), YAML codec (485 lines), glob matcher (168 lines), MCP protocol+stdio server (359 lines), Cobra-like command framework (676 lines) | — |

## 6. Porting effort estimate per domain + dead/unused flags

Sizing: S <300 lines, M 300–1000, L 1000–2500, XL >2500. "Dead?" checked via `grep` across the repo for cross-references outside `component.go`/its own tests (TypeScript packages, `.vscode/launch.json`, nx plugins) — none of `component.go`'s Go symbols are consumed outside this Go package/module (it's an internal implementation, only `cmd/repo/🐹️.go` and the sibling `🔌️mcp` Go module import it), so "dead" below means "no runtime path calls it currently" within the file itself, not "unused by the rest of the repo."

| Domain | Size | Effort | Notes |
|---|---|---|---|
| Header/Preamble/Templates | S | S | trivial |
| Engine (events/errors/requests/core) | S | S | core event-streaming contract — port first, everything else depends on its shape |
| Auth/Analyze/EntityEmojis commands | S | S | thin CLI wrappers |
| Test Command | L | L | runner-detection logic for 6+ ecosystems (go/js/py/rust/dotnet/ruby) — lots of exec.Command shelling, high effort mainly due to breadth not complexity |
| Rename | S | S | regex-based rename, needs Rust regex-equivalent strategy first |
| Models (Tree/Goal/Ticket nodes) | S | S | plain data types |
| Tree Logic (+caches) | XL | XL | biggest algorithmic piece: monorepo tree build, caching, propagation — no external deps, straightforward but long |
| CLI Renderers (ANSI/LOC/Mermaid) | XL | L | LOC command alone needs `git log --numstat` parsing/history; Mermaid diagram text generation |
| GraphQL Types | L | M | mostly data + string-enum validation, mechanical port |
| Providers | L | L | 7 editor providers are mostly thin/near-duplicate (flag: `DevcontainerSandboxProvider` is only 12 lines — likely a stub, check if genuinely unused before porting 1:1) |
| Types/Languages | XL | XL | 12 language plugins × comment/section/definition regex — the crux of the "language-agnostic taxonomy" effort; should become schema-first config rather than per-language Go structs |
| Types/Utils+RepoConfig | L | M | foundational path-resolution, port early |
| Types/Sections | M | M | parser logic, needed by many domains |
| Types/Policies (unlabeled region) | XL | L | the analyze/fix "statute" engine — depends on Languages+Sections being ported first |
| Codebase | L | M | orchestrates Languages/Sections/ignore |
| Tickets | XL (largest, 4850 lines) | XL | ticket lifecycle + GitHub sync + "important document" transactional create/rollback — highest risk area, most business logic |
| GraphQL Context Port/Resolver/DefaultContext | XL | XL | the `RepoContext` interface + its one big implementation (100+ methods) — consider whether Rust needs the same one-God-object shape or should split per aggregate |
| GraphQL Executor/Schema Builder | XL (2763-line single func) | XL | hand-rolled GraphQL — highest structural-refactor candidate (one 2763-line function is itself a CLAUDE.md violation: "clean long term solution", should not be ported verbatim) |
| Resolvers (Query/Mutation/Entity) | L | M | mechanical once Executor+Context exist |
| Mcp (args/paths/handlers) | L | M | 26 resource handlers are repetitive/generatable |
| Cli (hook types, diff/breach, benchmark) | XL | L | ~20 `HookResult*` event types = good candidate for schema-first codegen across languages |
| Hooks | L | M | per-IDE event-name mapping tables, mostly data |
| File Utilities/Server Client/Goals+Sessions | L | M | Server Client talks to an external "compose server" over http — needs the same external-dep decision as Providers |
| Todos | S | S | small |
| Entity Rendering/Artifact ID | XL | L | emoji/ID codec logic is repo-wide convention, must match exactly (many string round-trip functions) |
| Missing Hook/Test/Utility Functions | L | M | grab-bag, mostly per-tool test-file resolution — same shelling pattern as Test Command, dedupe candidate |
| Missing Hook Functions (tail: McpClientKind, plan resolution, McpServerFactory) | L | L | this is where **actual MCP tool registration** lives (only 6 tools) — likely the newest/least-finished code (name "Missing Hook Functions" reused twice, region hygiene broken) — treat as actively in-progress, verify with the ticket author before porting |

**Overall dead-code flag:** `DevcontainerSandboxProvider` (12 lines) and possibly individual `*EditorProvider` structs beyond the ones with real handler logic (Copilot/Cursor/Windsurf/Claude have method bodies; verify Droid/Codex/Antigravity/Kiro aren't pure stubs) are worth a follow-up "which providers actually have codepaths reachable from a CLI command" grep pass before committing Rust-port effort to all 7 identically.

**Structural/process flags for the planner (not code, but relevant to the "clean long-term solution" mandate):**
1. `#region` nesting is broken (see §0) — the source itself needs re-sectioning before/while extracting per-domain files.
2. 568 Go-only `Test*` functions in `🔬️component_test.go` violate the "language-agnostic test per feature" rule; only `🤝️g1_contract_test.go` (JSON-fixture-driven) matches that rule today — the taxonomy-tree migration should convert (or replace) the bulk of `component_test.go` into fixture-driven, language-agnostic tests under each new domain's `🧪️tests` folder.
3. `LoadRepoConfig`/`parseRepoConfigBool` hand-parses config without using the file's own `internal/yaml` package — an existing inconsistency to fix during the move, not carry forward.
4. The unrelated `🔨️modules/⌨️cli/📦️packages/🦀️rust` (a "semio monorepo orchestrator CLI/TUI", commands under `🎮️commands/...`) is a **different product** from this repo-client CLI — do not confuse it with the target of this port, though its taxonomy layout (`📦️packages/🦀️rust` sibling folder) is the pattern to replicate for repo-client.
