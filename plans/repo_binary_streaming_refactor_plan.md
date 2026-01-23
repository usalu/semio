# Repo Binary Streaming Refactor Plan (Single-File: `go/repo/main.go`)

> Constraint: **Everything stays inside `go/repo/main.go`**. No new files or folders.  
> Target architecture: **core commands are concurrent + streaming**, with **three adapters**: **CLI**, **MCP (stdio)**, **VS Code**.  
> Non-goals: legacy API and backwards compatibility (unless required by existing consumers).  
> Existing baseline: Cobra root wiring, JSONL rendering, and `Engine.Run(ctx, Request) <-chan Event` already exist but much of the CLI still funnels into a GraphQL request/response style (`runGraphQL`). fileciteturn3file12 fileciteturn3file16

---

## 0) Why this refactor and what “streaming core” means

### Problem statement (current behavior)
- Many commands are implemented as **GraphQL calls** that return a single `KindResult` event with a big JSON blob (request/response in disguise). fileciteturn3file16  
- MCP server path is currently stubbed and does not run the engine pipeline (`serveMcp` → `runMcpServer(nil,nil)`). fileciteturn3file12  
- Some commands (e.g. `analyzeCmd`) still exist in a “print GraphQL then exit” mode rather than emitting machine-consumable JSONL events consistently. fileciteturn3file17  
- Renderers already support JSONL / compact and recognize `KindDone`/`KindError`/`KindLog`/`KindResult`. fileciteturn3file10

### Desired behavior (streaming all the way down)
For *every* command group (bundles, folders, files, tickets, violations, sections, definitions, policies, export, tool, operational):
- Start emitting events immediately.
- Emit items as they’re discovered (e.g., files stream file-by-file, analyze streams violations as they are found).
- Use concurrency with bounded fanout and cancelation via context.
- Always end with a terminal `KindDone` event and close the stream. fileciteturn3file10

---

## 1) Hard constraints & invariants

### Single-file constraint
- All new types, helpers, and “subpackages” must be expressed as **regions** inside `go/repo/main.go`.

### Output contract
- CLI MUST continue to support `--format jsonl|json|compact`, where JSONL is the canonical machine stream.
- JSONL stream MUST contain a terminal **done payload** with exit code (already supported by `RenderJSONL`). fileciteturn3file10

### Adapter separation (within one file)
Even though everything is in one file, enforce conceptual boundaries:
- **Core**: commands + stores + streaming primitives (no Cobra, no MCP protocol, no VS Code specifics).
- **Adapters**:
  - CLI adapter: Cobra → core invocation → render stream.
  - MCP adapter: tool calls → core invocation → stream results as MCP responses/notifications.
  - VS Code adapter: JSON-RPC/LSP-like stream protocol over stdio (or via the existing `repo` binary execution model).

---

## 2) Proposed event model: “Envelope” events with typed item streams

### Keep existing `Event` but evolve its semantics
You already have:
- `Event.Kind` with `KindDone`, `KindError`, `KindLog`, `KindResult`, `KindProgress` (implied by render logic). fileciteturn3file10
- `RenderCompact` pretty-prints `KindResult` payloads. fileciteturn3file10

Refactor plan:
1. **Stop using `KindResult` for “one huge response blob.”**
2. Introduce **item events** by convention (still using `KindResult` initially to avoid invasive schema changes), where:
   - `event.Meta = { "stream": "items", "kind": "file" }`
   - `event.Data = JSON(file)` for each item.
3. Add an explicit `KindItem` once you’re ready (optional step). If you add it, update renderers to treat it like `KindResult`.

### Standard event shapes (recommended)
- `KindLog`: human-readable messages (stderr in compact mode). fileciteturn3file10
- `KindProgress`: `{ "current": n, "total": m, "unit": "files" }`
- `KindError`: `{ "message": "...", "detail": "..." }`
- `KindItem`: `{ "itemKind": "file|bundle|ticket|violation|...", "item": {...} }`
- `KindDone`: `{ "exitCode": 0, "summary": {...}, "counters": {...} }`

---

## 3) Core primitives to add (in-file “packages” via regions)

Create these regions (names are suggestions; align with existing region style you already use, e.g. `// #region Sections`). fileciteturn3file3

### 3.1 `// #region Stream` — fanout, emit, cancellation, backpressure
Add:
- `type Emitter struct { out chan Event; ctx context.Context; ... }`
- `func NewEmitter(ctx context.Context, buffer int) *Emitter`
- `func (e *Emitter) Log(msg string)`
- `func (e *Emitter) Progress(current,total int, unit string)`
- `func (e *Emitter) Error(err error, msg string, detail string)` (supports non-fatal + fatal)
- `func (e *Emitter) Item(kind string, v any)` (encodes v to JSON bytes into `Event.Data`)
- `func (e *Emitter) Done(exitCode int, summary any)` (emits done, then closes)
- `func (e *Emitter) CloseWithPanicRecovery()` (ensures stream closes with error)

Backpressure rules:
- Output channel is bounded (e.g., 256).
- Producer goroutines block if consumer is slow.
- All goroutines select on `ctx.Done()` to stop quickly.

### 3.2 `// #region Concurrency` — bounded worker pools
Add:
- `type Semaphore` or use `chan struct{}` with `Acquire/Release`.
- `func ForEachConcurrent[T any](ctx, items, limit, fn)` helper.
- `errgroup.WithContext` usage pattern.

### 3.3 `// #region Command Registry`
Replace GraphQL-as-core with:
- `type CommandID string`
- `type Command struct { ID, Title string; Run func(ctx context.Context, deps *Deps, input json.RawMessage, emit *Emitter) int }`
- `type Registry struct { cmds map[CommandID]Command }`
- `func (r *Registry) Register(cmd Command)`
- `func (r *Registry) Run(ctx, id, input) <-chan Event`

Key rule: `Run` writes all events via emitter and returns an exit code; emitter will emit `Done`.

### 3.4 `// #region Deps` — dependency injection
Add:
- `type Deps struct { RootDir string; RepoCtx *RepoContext; CodebaseCtx *CodebaseContext; ... }`
- `func NewDeps(config Config) (*Deps, error)` as the single initializer.
- Avoid passing `Config` everywhere; compute derived values once.

---

## 4) Data access refactor: turn “load everything” into streamable iterators

### 4.1 Bundles
Currently bundles are loaded as a slice and often used for scope resolution. fileciteturn3file17

Plan:
- Create `func StreamBundles(ctx context.Context, deps *Deps, emit *Emitter) (<-chan Bundle, <-chan error)` or simpler:
  - `func ListBundles(ctx, deps, emit)` that emits `Item("bundle", Bundle)` for each bundle.

### 4.2 Files/Folders
There are existing glob helpers, ignore handling, and scope-to-files mapping. fileciteturn3file17

Plan:
- Replace `ScopeToFiles` returning `[]string` with a streaming version:
  - `func StreamScopeFiles(ctx context.Context, scope Scope, bundles []Bundle, emit *Emitter) <-chan string`
- Emit file items progressively:
  - `emit.Item("file", File{Path:..., Size:..., ...})`

### 4.3 Tickets
Tickets already have structure + storage rules in dev docs and code. fileciteturn3file11

Plan:
- Implement ticket operations in core to emit:
  - `Item("ticket", TicketSummary)` per ticket in list
  - `Item("ticketIteration", TicketIteration)` when progressing
  - `Log` for side effects (GitHub, filesystem)
- Ensure ticket list reads from `.semio-repo/tickets` and falls back to legacy `tickets/` when needed (as documented). fileciteturn3file0

### 4.4 Violations / Analyze
The existing `analyzeCmd` special-cases “no scope” to emit codebase snapshot. fileciteturn3file17

Plan:
- Convert analyze to:
  - Stream violations as soon as each file is analyzed.
  - Periodically emit progress counts.
  - When no scope: still write codebase snapshot to `.semio-repo/reports/codebase.json` (existing behavior), but do it as part of the stream with `Log` + `Done summary` payload. fileciteturn3file17

---

## 5) Remove GraphQL as the core execution mechanism (but keep GraphQL as an adapter if needed)

### Current
`runGraphQL` packages `{query, variables}` → `Request{Command: CmdGraphQL}` → `engine.Run` → renderer. fileciteturn3file16

### Target
- **Core** runs command IDs directly, not GraphQL queries.
- GraphQL becomes:
  - Either a compatibility adapter that translates GraphQL requests into core commands, **or**
  - A thin layer that directly calls core stores and emits items.

Given your “no legacy/back-compat required” direction, the cleanest plan is:
1. Keep the existing `graphql` cobra command temporarily for VS Code extension compatibility.
2. Reimplement GraphQL resolvers to call the core streaming stores and **materialize only what GraphQL must return**.
3. Move CLI subcommands away from GraphQL entirely.

This aligns with: “repo CLI MUST emit JSONL event stream with terminal done payload.” fileciteturn3file0

---

## 6) CLI adapter refactor (Cobra → core registry)

### 6.1 Keep Cobra command tree, but change implementations
You already register many cobra command factories. fileciteturn3file12  
Refactor each `RunE` to:
1. Parse args/flags into a small JSON input object (no new structs needed; can build a map and marshal).
2. Call `registry.Run(ctx, "files.list", inputJSON)` (or similar).
3. `renderStream(cmd, config, stream)` remains unchanged. fileciteturn3file10

### 6.2 Define canonical Command IDs
Adopt stable IDs matching your command tree:
- `bundle.list`, `bundle.tree`
- `folder.list`, `folder.tree`, `folder.create`, `folder.move`, `folder.delete`
- `file.list`, `file.tree`, `file.create`, `file.move`, `file.delete`
- `section.list`, `section.tree`, `section.create`, `section.move`, `section.delete`, `section.integrate`
- `definition.list`
- `policy.list`, `policy.check`
- `ticket.open`, `ticket.list`, `ticket.read`, `ticket.progress`, `ticket.close`, `ticket.reopen`
- `analyze`, `fix`, `export`, `tool.run`
- `benchmark`, `preflight`, `update`

### 6.3 Render contract changes
- Ensure all commands emit:
  - Zero or more `KindItem`/`KindResult` item events.
  - Optional `Log`/`Progress`.
  - One `Done`.
- CLI `--format compact` continues to print logs/errors to stderr and results to stdout. fileciteturn3file10

---

## 7) MCP adapter refactor (stdio MCP tools → core registry)

### 7.1 Current status
`mcpCommand` exists and calls `serveMcp(ctx, engine)` but `serveMcp` is stubbed and calls `runMcpServer(nil,nil)`. fileciteturn3file12

### 7.2 Target behavior
- MCP tool calls invoke core commands, not GraphQL.
- MCP returns:
  - A final tool response summarizing what happened (counts + last N items or a cursor).
  - Optional progress notifications if your MCP library supports them.

### 7.3 Streaming bridging patterns
Because many MCP tool protocols require a single response per tool call, implement:
- **Cursor paging**: tool call returns `{ cursor, items }`, next call uses cursor.
- Internally, you still stream items as they’re produced and buffer until page is full.

### 7.4 Tool input validation
Your requirements say MCP tool calls MUST validate argument presence and types. fileciteturn3file0  
Implement this at the adapter boundary:
- parse tool args → validate required fields → build command input JSON → run core command → collect/paginate.

---

## 8) VS Code adapter refactor (stream protocol designed for extension needs)

### 8.1 Current integration reality
VS Code extension runs GraphQL through the repo CLI and consumes final GraphQL JSON. fileciteturn3file1

### 8.2 Two viable approaches (both stay single-file)
**Approach A (minimal extension churn):** keep `repo graphql` stable but implement its internals against core stores.
- VS Code stays unchanged.
- You still get streaming internally (useful for performance), but GraphQL output remains a single JSON response.

**Approach B (true streaming to VS Code):** add a new `repo vscode-stdio` (or `repo rpc`) command:
- It speaks JSON-RPC-like messages over stdio.
- Each request returns a stream of envelope events that VS Code can read incrementally.
- VS Code can cancel by sending `{id, cancel:true}`.

Given your stated direction (“three adapters: cli, mcp, vscode”), the clean plan is:
- Keep GraphQL as transitional.
- Add VS Code streaming command and migrate the extension later.

### 8.3 Suggested VS Code stream message format
One JSON object per line:
- Request:
  - `{"id":"123","command":"analyze","input":{...}}`
- Response stream:
  - `{"id":"123","event":{...}}` (where `event` matches `Event` JSON)
- Cancel:
  - `{"id":"123","cancel":true}`

This reuses your existing JSONL event model and keeps parsing simple.

---

## 9) Step-by-step execution plan (safe, incremental, always runnable)

> This is deliberately broken into small, mechanically verifiable steps to avoid a “big bang” rewrite inside a single file.

### Phase 1 — Establish streaming core primitives (no behavior change yet)
1. Add `// #region Stream` and implement `Emitter`.
2. Add `// #region Command Registry` and `Registry` with `Register` + `Run`.
3. Add `// #region Deps` with `NewDeps(config)` and reuse your current root-dir resolution logic (currently in `defaultEngineFactory`). fileciteturn3file12
4. Add a single “demo” command `internal.ping` that emits:
   - `Log("ping")`
   - `Item("ping", {"ok":true})`
   - `Done(0, {"ok":true})`

**Exit criteria**
- `repo internal ping --format jsonl` returns valid JSONL and a `done`.

### Phase 2 — Refactor CLI to use registry for one command group
Pick the easiest: `bundle list`.
1. Implement core `bundle.list` by calling the existing bundle loader and emitting one item per bundle.
2. Change the cobra `bundle list` handler to call registry instead of GraphQL. (Keep flags the same.)
3. Ensure JSONL output is item-per-line and ends with `done`.

**Exit criteria**
- `repo bundle list --format jsonl` streams bundles and ends with done.

### Phase 3 — Stream folders and files (core data-plane)
1. Implement `StreamScopeFiles` for repo/bundle/folder/file scopes.
2. Implement `file.list` and `folder.list` in core:
   - `folder.list` streams child folders
   - `file.list` streams files
3. Move cobra `file` and `folder` subcommands to registry.

**Exit criteria**
- `repo file list <scope>` prints items progressively.

### Phase 4 — Streaming analyze/violations (highest value)
1. Implement `analyze` core command to:
   - resolve files from scope
   - concurrently analyze each file
   - emit violation items as found
   - emit periodic progress
2. Preserve existing “no scope writes codebase snapshot” behavior in-stream. fileciteturn3file17
3. Adjust `fix` similarly (stream fixed items + remaining violations).

**Exit criteria**
- VS Code can still run `repo analyze <path>` and parse JSONL (even before extension changes, this is valuable for CLI/MCP).

### Phase 5 — Tickets streamification
1. Implement `ticket.list` as item stream (one ticket per item).
2. Implement `ticket.read` streaming:
   - emit ticket header info
   - emit entries/events/iterations as separate items
3. Implement write operations (`open/progress/close/reopen`) with:
   - log events for filesystem/GitHub side effects
   - final done summary with created paths / identifiers

**Exit criteria**
- `repo ticket list` streams ticket summaries quickly.

### Phase 6 — MCP adapter fully wired to core
1. Replace `serveMcp` stub with real MCP server wiring that maps tools → command IDs. fileciteturn3file12
2. Implement strict arg validation.
3. Add cursor paging for item-heavy tools (files, violations).

**Exit criteria**
- `repo mcp` serves tools that operate on streaming core.

### Phase 7 — VS Code adapter command
1. Add new cobra command `vscode-stdio` (or `rpc`) that:
   - reads line-delimited JSON requests
   - spawns command stream per request
   - multiplexes events back with request id
   - supports cancellation
2. Keep `repo graphql` for transitional compatibility.

**Exit criteria**
- A prototype VS Code client can consume events incrementally.

---

## 10) Mechanical mapping: how to replace `Engine.Run` usage without breaking everything at once

### Option 1: Keep `Engine` but change its internals
Currently CLI uses `engine.Run(ctx, request)` and renderers read the event channel. fileciteturn3file16  
You can preserve this external shape by:
- making `CmdGraphQL` just one of many commands
- adding new `CmdInvoke` with `{ "id": "bundle.list", "input": {...} }`
- `engine.Run` routes to the registry

### Option 2: Bypass `Engine` for CLI and keep it for GraphQL
- CLI calls registry directly.
- GraphQL keeps using `Engine` until later.

Given the single-file constraints, Option 1 is cleaner: one execution path for all adapters.

---

## 11) Specific refactor targets in the current file

### Replace `printGQL` (legacy path) with a stream renderer call
`printGQL` prints final string output. fileciteturn3file17  
Plan:
- Keep `printGQL` only for debugging (or remove once not used).
- Ensure cobra commands route to registry so output is uniform.

### Replace `runGraphQL` use sites gradually
Many cobra commands call `runGraphQL(...)`. fileciteturn3file19  
Plan:
- For each command group, migrate subcommands to core registry.
- Leave GraphQL-only commands until core coverage is complete.

### MCP stub removal
`serveMcp` currently returns `runMcpServer(nil,nil)`. fileciteturn3file12  
Plan:
- Wire it to registry, not engine-graphql.

---

## 12) Risks, mitigations, and “gotchas”

### Risk: Output consumers expecting “one JSON object”
Mitigation:
- Keep `--format json` which collects all events into an array (already implemented). fileciteturn3file10
- Maintain `--format compact` for humans.

### Risk: Memory blow-ups from buffering
Mitigation:
- Never accumulate full lists in core (only adapters may page/collect if required).

### Risk: Concurrency overload on large repos
Mitigation:
- Default concurrency cap (e.g., `min(16, GOMAXPROCS*4)`) and allow `--jobs` override in config.

### Risk: VS Code extension currently expects GraphQL
Mitigation:
- Keep GraphQL command until VS Code migrates.
- Optionally have `repo graphql` internally run core and materialize.

---

## 13) Definition of Done (DoD)

### Streaming guarantees
- Every command:
  - emits zero or more item events
  - emits a terminal `done` with exitCode
  - closes the stream
- No command prints directly to stdout/stderr except through event renderers.

### Adapter guarantees
- CLI: all commands use registry; GraphQL only as a dedicated `repo graphql`.
- MCP: no stubs; all tools validate args and run core.
- VS Code: a dedicated streaming mode exists (even if extension migration is later).

---

## 14) Implementation checklist (copy/paste)

- [ ] Add `Stream` region with `Emitter`
- [ ] Add `Concurrency` helpers (semaphore + errgroup pattern)
- [ ] Add `Deps` initializer
- [ ] Add `Registry` and register `bundle.list`
- [ ] Migrate `bundle` cobra subcommands to registry
- [ ] Add `scope → file stream` helper and migrate `file.list`
- [ ] Add `folder` stream and migrate `folder.list/tree`
- [ ] Rewrite `analyze` to stream violations + progress
- [ ] Rewrite `fix` to stream fixes + remaining violations
- [ ] Migrate ticket commands to registry and stream results
- [ ] Wire MCP server to registry (remove stub)
- [ ] Add VS Code stdio streaming command
- [ ] Keep `repo graphql` transitional; back it with core data access
- [ ] Ensure `RenderJSONL` terminal `done` is always emitted (exit code propagation) fileciteturn3file10

---

## Appendix: Notes about repo tooling constraints (project context)

The dev docs explicitly state the repo CLI must remain a consolidated single-file entrypoint, owning engine, CLI, MCP, and rendering behavior, and that JSONL is the machine-consumable stream format with a terminal done payload. fileciteturn3file0

