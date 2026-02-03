---
title: Streaming Refactor
status: open
prompt: Implement streaming-only refactor plan (CLI + VSCode + MCP)
commit: HEAD
---

# 🤖 Prompt

Implement the plan.
Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear. 
Dont ask in between, no confirmations, no matter the issue. Figure it out.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to add the plan.md, to track everything (todos, changes, summary, etc) in ticket.md

# 📝 Plan

[plan.md](plan.md)

# 📋 Todos

- [ ] Phase 0: Guardrails
- [ ] Phase 1: Core Engine API Replacement
- [ ] Phase 2: CLI Streaming-only + Colors
- [ ] Phase 3: VSCode Adapter v2
- [ ] Phase 4: MCP Adapter v2
- [ ] Phase 5: Cleanup & Validation

# 🪵 Log

- Ticket created.


## Todos
# Streaming-Only Refactor Plan (CLI + VSCode + MCP)

**Goal:** Make *every* command streaming-first and streaming-only (no request/response shaped APIs, no “batch response” output modes, no backwards compatibility).  
**Adapters:**  
1) **CLI** — colorful streaming CLI  
2) **VSCode** — GraphQL-oriented streaming over stdio  
3) **MCP** — MCP server adapter

This plan assumes the current code already has a streaming event model (`Event{Kind,...}`) and multiple adapters, but still contains:
- **Non-streaming output modes** (e.g. aggregating all events into one JSON array).
- **Request/response-shaped internal APIs** (e.g. `Engine.Run(ctx, Request{...})` and a dedicated `CmdGraphQL` path).
- **MCP tool handlers that aggregate** the stream into a single JSON response payload (cursor/limit wrapper).

The refactor below eliminates these shapes and makes **streaming the only representation** of command execution across engine + adapters.

---

## 1) North-star architecture

### 1.1 Single universal streaming interface

Replace “request structs” + “special-case commands” with a single streaming entry point:

```go
type Stream interface {
    Events() <-chan Event
    Cancel()
}

type Runner interface {
    Stream(ctx context.Context, cmd CommandID, input json.RawMessage) <-chan Event
}
```

**Rules:**
- Engine returns a **stream of `Event`** and *never returns a “result object”*.
- A command is done when it emits **exactly one** `KindDone` event and closes the channel.
- Every adapter maps its transport (CLI/stdin/stdout/MCP) to/from this stream.

### 1.2 Registry becomes the only command system

Unify “GraphQL” into the same registry mechanism as everything else:

- Remove special “engine command enum” routing (`CmdGraphQL`, `CmdInvoke`, etc.).
- Replace with one command space: `CommandID` strings.
- `graphql` becomes just another registry command (e.g. `graphql.query`), emitting:
  - `start`
  - `log` / `progress` (optional)
  - `result` (or `item`s)
  - `done`

### 1.3 One event schema, stable and documented

Keep one event schema (already present) but tighten invariants:

**Required invariants**
- `KindStart` must be first (per command execution).
- `KindDone` must be last and appear exactly once.
- `KindError` must include `ErrPayload{Code, Message, Fatal}`.
- `KindItem`/`KindResult` must have JSON in `Data`.
- Transport correlation is external (adapter adds request correlation fields, not the core event).

**Event schema (canonical)**
- `kind`: start|log|progress|item|result|artifact|error|done
- `command`: command id (`internal.ping`, `section.list`, `graphql.query`, ...)
- `message`, `progress`, `data`, `meta`, `artifact`, `error`, `done`

---

## 2) API & data model changes (Engine layer)

### 2.1 Delete the request/response shaped `Request` API

**Remove:**
- `type Request struct { Command Command; Args json.RawMessage; RepoRoot string; Verbose bool }`
- `type Command string` enum (`CmdGraphQL`, `CmdInvoke`, …)
- `Engine.Run(ctx, request)` entry point
- any “emitDone(out, code, status)” paths that assume request-level routing

**Add:**
- `Engine.Stream(ctx, cmdID CommandID, input json.RawMessage) <-chan Event`

**Why:** It makes “request” a transport concept (adapter-only), not an engine concept.

### 2.2 Rehome repo-root / verbose configuration

Currently the engine receives `RepoRoot` and `Verbose` through `Request`. That becomes engine configuration / dependencies:

- `EngineFactory(config Config) (*Engine, error)` already exists.
- Ensure repo-root is set once during engine creation.
- Ensure `Verbose` is stored on engine or passed through deps.

**Implementation options (choose one):**
1. Store `Verbose` in `Deps` / `Executor` so commands can access it.
2. Store `Verbose` on `Engine` and inject into `Emitter`.

### 2.3 Make GraphQL streaming-only

Currently GraphQL returns `interface{}` and is then packed into one `result` event.

Keep the event contract but make the executor *stream-ready* for future growth:

**Replace:**
```go
type GraphQLExecutor interface {
    Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error)
}
```

**With:**
```go
type GraphQLExecutor interface {
    ExecuteJSON(ctx context.Context, query string, variables map[string]any) (json.RawMessage, error)
}
```

- Engine/command emits one `KindResult` whose `Data` is already JSON.
- No intermediate interface/object conversions in engine.
- (Optional) later: support incremental result chunking as `KindItem` events.

### 2.4 Standardize error codes & cancellation

- Keep `ErrCanceled` as a real emitted error where appropriate.
- When `ctx.Done()` fires, emit:
  - `KindError { code: E_CANCELED, fatal:false }`
  - `KindDone { exit_code: 130, status:"error" }`
- Ensure every command respects context and always closes its channel.

---

## 3) CLI adapter: colorful streaming CLI

### 3.1 Delete non-streaming output format

**Remove** the JSON array aggregator renderer (collecting stream into `[]Event` then encoding).

- Delete `RenderJSON(out, stream)` (the non-streaming one).
- Remove the `--format json` option entirely.
- Keep **only**:
  - `compact` (human-readable, colorful)
  - `jsonl` (NDJSON streaming)

### 3.2 Implement real colorful output

`RenderCompact` currently prints plain text to stdout/stderr. Make it colorful:

- `KindError`: red (`fatal` bold red), show `Message`, optional `Detail` in dim/gray when `--verbose`.
- `KindLog`: dim gray by default; in verbose show full logs.
- `KindProgress`: cyan or magenta with a stable progress line (avoid flicker if not TTY).
- `KindItem`/`KindResult`:
  - Print `Data` prettified (already done) in normal color.
  - Optionally prefix with a green “✓” for result vs bullet for items.
- `KindArtifact`: yellow with `Type` + `URI`.

**TTY considerations**
- Detect if stderr/stdout is a TTY:
  - If TTY: enable colors and progress “in-place” updates.
  - If not TTY: no ANSI and print progress as regular lines.

### 3.3 CLI command wiring

Currently CLI commands call `runInvoke`/`runGraphQL` which build a `Request`. Replace with:

- `runCommandStream(cmdID string, input map[string]any)`
- Marshal input to JSON once.
- Call `engine.Stream(ctx, CommandID(cmdID), inputBytes)`
- Pipe to renderer

**GraphQL CLI**
- Keep `semio graphql` command if you want, but it calls the registry command `graphql.query`.
- That keeps everything uniformly streaming.

---

## 4) VSCode adapter: streaming stdio protocol (NDJSON)

The current VSCode adapter is already “streaming events out”, correlated by `id`. Keep that *shape* but remove any request/response semantics in your own protocol documentation and code.

### 4.1 Define the VSCode stdio protocol (v2)

**Input (client → server):** one JSON object per line (NDJSON)
```json
{"id":"req-123","command":"section.list","input":{"file":"README.md"}}
```

**Cancel message:**
```json
{"id":"req-123","cancel":true}
```

**Output (server → client):** one JSON object per line
```json
{"id":"req-123","event":{"kind":"log","message":"..."}}
{"id":"req-123","event":{"kind":"item","data":{...}}}
{"id":"req-123","event":{"kind":"done","done":{"exit_code":0,"status":"ok"}}}
```

**Rules**
- Server never emits a final “response” wrapper.
- Completion is signaled only by `event.kind == "done"`.
- Any parsing errors are emitted as `{"event":{kind:"error",...}}` *without* an `id` (or with a special `id:"_"`).

### 4.2 Engine call path

- Replace the `InvokeArgs` wrapper usage in the adapter.
- VSCode adapter directly calls:
  - `engine.Stream(childCtx, CommandID(request.Command), request.Input)`
- `request.Input` stays as raw JSON; no extra re-marshal required.

### 4.3 Concurrency and backpressure

- Maintain the per-request context map for cancellation.
- Add an output queue per request (buffered channel) if you see lock contention on encoder.
- Ensure all goroutines terminate on EOF.

---

## 5) MCP adapter: streaming semantics without “aggregated JSON response”

### 5.1 Hard constraint: MCP `call_tool` returns a result object

Most MCP SDKs require a final `CallToolResult`. That’s not “streaming results” by itself.  
However MCP **does** support progress/log style notifications (and some transports support streaming patterns).

**Therefore the refactor goal for MCP is:**
- **Never** aggregate stream items/logs/errors into a big JSON response body.
- Use MCP notifications (progress/log) while running.
- Return a **minimal final result** (e.g. “done” status + optional artifact pointer).

This satisfies “commands defined only with streaming” inside your codebase: the engine remains streaming, and MCP mapping is a thin transport adaptation with minimal terminal output.

### 5.2 MCP v2 behavior

For every MCP tool call:
1. Start engine stream
2. For each event:
   - `log` → MCP logging notification (or append as incremental text chunks if notification not available)
   - `progress` → MCP progress notification (if the request provides a progress token/task)
   - `item`/`result`:
     - Prefer: emit as “resource created” / “artifact written” and notify
     - Fallback: emit log lines like `ITEM {...}` for debugging mode only
   - `artifact` → convert to MCP resource URI references if supported
   - `error` → emit MCP error/log notification
3. When `done` arrives:
   - Return `CallToolResult` containing *only*:
     - summary string
     - or a compact JSON `{"status":"ok","exit_code":0,"artifact":"file://..."}`

### 5.3 Remove cursor/limit pagination wrapper

The current MCP tool wrapper collects all items, logs, errors and returns a JSON object with `items`, `cursor`, etc. That is explicitly non-streaming.

Delete:
- `mcpCursorLimit`
- the “collect items and return a big JSON” approach

If pagination is required for client UX:
- Implement it as separate commands that accept `cursor`/`limit` inputs and stream only that slice.
- But **the transport still streams** (the engine controls pagination, not MCP adapter collecting).

---

## 6) Registry & command implementations

### 6.1 Command contract

Each registry command should follow:

```go
type CommandHandler func(ctx context.Context, deps *Deps, input json.RawMessage, emit *Emitter)
```

- It must:
  - validate input
  - emit `log`/`progress`/`item`/`result` as needed
  - end with `emit.Done(exitCode, summary)`

### 6.2 No “return ToolResult” anywhere

The codebase currently has some “ToolX returns ToolResult” style helpers for GraphQL resolvers and/or MCP.

Refactor direction:
- Keep pure functions for logic (e.g. parsing, file operations).
- But **command outputs are events**, not return structs.
- GraphQL resolvers can remain request/response (that’s GraphQL), but your *commands* should not expose non-streaming variants.

---

## 7) Deletions checklist (explicit “no legacy”)

### 7.1 Remove entirely
- `RenderJSON` (stream-to-array)
- CLI `--format json`
- Engine `Request` struct and `Command` enum routing
- `CmdGraphQL`, `CmdInvoke`, and any special-cased engine run paths
- MCP “aggregate into JSON response” collector
- Any “compat” protocol versions in VSCode adapter (document only v2)

### 7.2 Remove references and dead code paths
- Any helper that produces “complete result” objects solely for CLI formatting
- Any “ToolResult” wrappers used only for old output modes

---

## 8) Implementation plan (phased, but no backwards compat)

### Phase 0 — Guardrails (same-day)
- Add tests asserting:
  - every command emits exactly one `done`
  - `done` is last event
  - channel closes after `done`

### Phase 1 — Core engine API replacement
- Introduce `Engine.Stream(ctx, cmdID, input)` and wire it to registry execution.
- Move GraphQL execution into registry as `graphql.query`.
- Delete `Request` and command enum routing.

### Phase 2 — CLI streaming-only + colors
- Remove `RenderJSON` and CLI format option.
- Implement colorful `RenderCompact` with TTY detection.
- Keep `RenderJSONL` as machine mode.

### Phase 3 — VSCode adapter v2
- Update adapter to call `engine.Stream` directly with `command` and raw input.
- Document NDJSON protocol and cancellation behavior.
- Remove any legacy wrappers (`InvokeArgs` usage inside adapter).

### Phase 4 — MCP adapter v2
- Replace collector with a live event-forwarder using MCP progress/log capabilities.
- Make tool results minimal and artifact-oriented.
- Remove cursor/limit aggregation.

### Phase 5 — Cleanup & validation
- Rip out now-unused types (`GraphQLArgs` can remain as input struct for the `graphql.query` command, but it’s no longer an engine request).
- Ensure go vet / tests pass.
- Add integration tests:
  - CLI jsonl output produces valid NDJSON of events
  - VSCode stdio roundtrip: send command, receive done
  - MCP tool call emits progress/log notifications (where supported) and returns minimal done

---

## 9) Test strategy

### 9.1 Unit tests
- `TestEmitterDoneOnce`
- `TestCommandAlwaysClosesChannel`
- `TestRenderJSONLStreamingDoesNotBuffer`
- `TestVSCodeAdapterCancelStopsStream`

### 9.2 Golden tests (output stability)
- CLI `compact` snapshots for:
  - normal success (items + done)
  - fatal error
  - verbose mode detail
- CLI `jsonl` snapshot: exact event ordering and JSON validity.

### 9.3 Contract tests (adapter-agnostic)
A reusable “command contract harness”:
- runs a command with a context + input
- asserts invariants (start→…→done)
- asserts `done.exit_code` mapping

---

## 10) Deliverables / final state

### 10.1 Public contracts
- **Event schema** (stable JSON fields)
- **CLI formats**: `compact` (TTY colors) and `jsonl` only
- **VSCode NDJSON protocol**: `{id, command, input}` → `{id, event}`
- **MCP mapping**: stream → MCP notifications + minimal final result

### 10.2 Code layout (recommended)
```
/engine
  engine.go        // Engine.Stream
  events.go        // Event types + invariants
  emitter.go
/registry
  registry.go
  commands/
    graphql_query.go
    section_list.go
/adapters
  cli/
    render_compact.go
    render_jsonl.go
  vscode/
    stdio.go
  mcp/
    server.go
    bridge.go
```

---

## 11) Notes on “no request/response”

- Internally, your system becomes **stream-only**: commands do not return values, only events.
- Transport protocols may still contain “a request envelope” (VSCode NDJSON line, MCP call_tool), but **you never implement a response-shaped API**:
  - no buffered “response”
  - completion is only `done`
  - data is only `item`/`result` events

This keeps the codebase’s command API purely streaming, while still operating within constraints of each adapter’s host protocol.

## Summary

Bulk close
