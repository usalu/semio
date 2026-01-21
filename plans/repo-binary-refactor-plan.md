# Clean Streaming Refactor Plan (Core + Adapters) — Repo Binary First, `extension.ts` Later

This plan assumes you are **allowed to change the public CLI/API** and you want a **clean architecture**:

- **Core (engine)** contains all business logic and emits **streams of events**.
- **Adapters** translate environments (CLI, JSON tooling, MCP, VS Code) into core requests and render/transport the event stream.
- **Every command is streaming**: even “instant” commands emit at least one `result` event followed by a terminal `done` event.

It also includes a **later** plan for `js/vscode/extension.ts` once the new output contract is implemented.

---

## 1) Goals and Non‑Goals

### Goals
- Single, deterministic “engine” that can be reused by:
  - CLI humans (compact text)
  - Tools (stable machine output)
  - MCP server
  - VS Code extension
- Streaming-by-default execution model for all commands.
- Clean output contract that supports:
  - progress
  - partial results
  - structured errors
  - final result

### Non‑Goals (for the first iteration)
- Maintaining backward compatibility with the current CLI stdout format.
- Supporting interactive prompts (binary remains non-interactive).

---

## 2) Current Integration Constraints (Why We Must Change `extension.ts` Later)

Today the VS Code extension executes `repo graphql ...` and does `JSON.parse(stdout)` on the entire stdout payload. That makes it sensitive to *any* additional output on stdout and assumes a single JSON object/string, not a stream. (This is why the old plan kept a compatibility mode.) See existing behavior in the repo refactor notes. fileciteturn5file0

In the new “clean” architecture, the CLI in JSON mode will emit **NDJSON event streams** (one JSON object per line). The extension will be updated afterward to consume that stream and extract the terminal `result`.

---

## 3) Target Architecture

### 3.1 Packages / directories (recommended)
You no longer constrain yourself to a single `main.go`; implement a real “core + adapters” layout:

```
cmd/repo/
  main.go

internal/core/
  engine.go
  requests.go
  errors.go

internal/events/
  events.go
  schema.go

internal/adapters/cli/
  cobra.go
  render_compact.go
  render_jsonl.go
  exitcodes.go

internal/adapters/mcp/
  server.go
  tools.go
```

**Principle**: `internal/core` must not import Cobra, os.Stdout, MCP libs, or VS Code specifics.

---

## 4) The Streaming Contract (the “new API”)

### 4.1 Event schema
Define a single event type used everywhere:

```go
package events

type Kind string

const (
  KindStart    Kind = "start"     // command accepted, includes command + args summary
  KindLog      Kind = "log"       // structured log message
  KindProgress Kind = "progress"  // percent/message/step
  KindResult   Kind = "result"    // the “payload” (may be partial or final)
  KindArtifact Kind = "artifact"  // produced files/paths/urls
  KindError    Kind = "error"     // structured error (may be non-fatal or fatal)
  KindDone     Kind = "done"      // terminal; includes exit_code + optional error summary
)

type Event struct {
  Kind      Kind            `json:"kind"`
  Command   string          `json:"command,omitempty"`
  ID        string          `json:"id,omitempty"`       // correlation id for multi-step commands
  Message   string          `json:"message,omitempty"`
  Level     string          `json:"level,omitempty"`    // debug/info/warn/error
  Progress  *Progress       `json:"progress,omitempty"`
  Data      json.RawMessage `json:"data,omitempty"`     // command-specific payload
  Artifact  *Artifact       `json:"artifact,omitempty"`
  Error     *ErrPayload     `json:"error,omitempty"`
  Done      *DonePayload    `json:"done,omitempty"`
}

type Progress struct {
  Current int    `json:"current,omitempty"`
  Total   int    `json:"total,omitempty"`
  Percent int    `json:"percent,omitempty"`
  Step    string `json:"step,omitempty"`
}

type Artifact struct {
  Type string `json:"type"` // file, url, stdout_snippet, etc.
  URI  string `json:"uri"`
  Note string `json:"note,omitempty"`
}

type ErrPayload struct {
  Code    string `json:"code"`            // stable codes: E_AUTH, E_PARSE, E_NETWORK, E_INTERNAL, ...
  Message string `json:"message"`         // user-facing
  Detail  string `json:"detail,omitempty"`// optional debug detail
  Fatal   bool   `json:"fatal,omitempty"`
}

type DonePayload struct {
  ExitCode int    `json:"exit_code"`
  Status   string `json:"status"` // ok | error | canceled
}
```

**Rules**
- Every command emits:
  1) `start`
  2) (0..n) logs/progress/result/artifact/error
  3) exactly one terminal `done`
- In JSON streaming mode, every event is one NDJSON line on stdout.

### 4.2 Command “result” payloads
To keep the API clean:
- `Event.Data` contains the command’s output object, **not** wrapped in extra envelopes.
- The adapter decides whether to keep the full stream (tools) or render a compact summary (humans).

For GraphQL, `result.data` should be the standard GraphQL response object (e.g. `{ "data": ..., "errors": ... }`).

---

## 5) Core Engine API (streaming first)

### 5.1 Single entrypoint
In `internal/core/engine.go`:

```go
type Engine struct {
  // dependencies: executors, filesystem, network clients, config, etc.
}

func (e *Engine) Run(ctx context.Context, req Request) <-chan events.Event
```

The returned channel:
- is closed after emitting `done`
- never blocks forever (respect ctx cancellation)
- guarantees `done` even on panic (recover -> emit fatal error -> done)

### 5.2 Request types
In `internal/core/requests.go` define a typed request model:

```go
type Command string

const (
  CmdGraphQL Command = "graphql"
  CmdAnalyze Command = "analyze"
  CmdFix     Command = "fix"
  CmdPolicy  Command = "policy"
  CmdTicket  Command = "ticket"
  // ...
)

type Request struct {
  Command Command
  Args    json.RawMessage // command-specific args struct marshaled to JSON
  // shared options:
  RepoRoot string
  Verbose  bool
}
```

Command-specific argument types:

```go
type GraphQLArgs struct {
  Query     string                 `json:"query"`
  Variables map[string]any         `json:"variables,omitempty"`
}

type AnalyzeArgs struct { /* ... */ }
type FixArgs struct { /* ... */ }
```

**Why JSON args**: it makes requests easy to construct from CLI/MCP/VS Code while keeping the core decoupled.

### 5.3 Dispatch
Engine `Run`:
- emits `start`
- dispatches to `runGraphQL`, `runAnalyze`, etc.
- each handler emits stream events and ends with `done`

---

## 6) Adapters

### 6.1 CLI adapter (Cobra remains here)
`internal/adapters/cli/cobra.go`
- Parses flags / subcommands.
- Builds `core.Request` with command args.
- Calls `engine.Run`.
- Chooses a renderer based on `--format`.

**CLI output modes**
- `--format=compact` (default): human + LLM-friendly
- `--format=jsonl`: NDJSON stream (tooling)
- `--format=json`: buffered JSON array (optional, not recommended, but can help some tools)

> Recommendation: make `jsonl` the canonical machine mode. Keep `json` as a convenience wrapper that collects events and outputs `[{...},{...}]`.

### 6.2 Renderers

#### Compact renderer (`render_compact.go`)
Consumes the event stream and prints:
- A brief header on `start`
- Progress lines (throttled) to stderr
- On `result`, print a minimal summary + optionally a short excerpt
- On `artifact`, print a single “Produced: …”
- On fatal error, print a concise error line and (if verbose) detail
- No colors by default; add `--color=auto` later if desired

**Compact formatting rules**
- Max ~80–120 columns.
- Top line is always a single-sentence status summary.
- Lists are capped (`--max-items`, default 20).
- Truncate large JSON values with a stable truncation marker.

#### JSONL renderer (`render_jsonl.go`)
- Writes each event as one JSON line to stdout.
- Never writes anything non-JSON to stdout.
- Optionally writes nothing to stderr unless `--debug-stderr` is set.

### 6.3 MCP adapter
`internal/adapters/mcp/server.go`
- Implements MCP tool listing and tool calls.
- Each tool call:
  - constructs a `core.Request`
  - runs `engine.Run`
  - streams progress/logs as MCP notifications (if desired)
  - returns the final `result` payload from the last `result` event before `done`
  - if fatal error occurs, return an MCP error result

Important: MCP should not need special core logic—only translation and event handling.

---

## 7) “Every command is streaming” Implementation Pattern

For each core command handler, use the same skeleton:

```go
func (e *Engine) runX(ctx context.Context, req Request, out chan<- events.Event) {
  defer closeWithDone(out, exitCode)
  emit(out, events.Event{Kind: KindLog, Level: "info", Message: "..."})

  // for long tasks:
  for i := 0; i < total; i++ {
    if ctx.Err() != nil { emitCanceled(...); return }
    emit(out, events.Event{Kind: KindProgress, Progress: &events.Progress{Current: i, Total: total}})
    // work...
    if partialReady { emitResultPartial(...) }
  }

  emitResultFinal(...)
}
```

Even “instant” commands still:
- emit `start`
- emit one `result`
- emit `done`

---

## 8) Clean CLI Redesign (since API changes are allowed)

### 8.1 New flag set (consistent across subcommands)
Global:
- `--format compact|jsonl|json`
- `--verbose`
- `--repo <path>` (optional)
- `--timeout <duration>` (optional)
- `--max-items <n>` (compact only)

Command arguments should be structured and consistent. Examples:

**GraphQL**
- `repo graphql --query '<gql>' --vars '{"x":1}' --format jsonl`

**Analyze**
- `repo analyze --scope @semio --format compact`

**Fix**
- `repo fix --scope @semio --apply --format compact`

You can keep legacy positional args if you want, but the new API will be easier to consume from VS Code/MCP.

### 8.2 Exit codes
Define stable exit codes in `adapters/cli/exitcodes.go`:
- `0` ok
- `1` generic error
- `2` usage/args error
- `3` auth error
- `4` network error
- `130` canceled (SIGINT / context canceled)

Core emits `done.exit_code`; CLI adapter uses it as the process exit code.

---

## 9) Migration Strategy (Binary First)

### Phase A — Core scaffolding and one command
1. Create packages and move shared dependencies construction into `cmd/repo/main.go`.
2. Implement `events.Event` schema.
3. Implement `core.Engine` and `Run` with panic recovery + guaranteed `done`.
4. Port **one** command end-to-end (recommend `graphql` first):
   - core handler: execute GraphQL and emit `result` with full GraphQL response
   - CLI adapter: new cobra subcommand
   - renderers: compact + jsonl

### Phase B — Port remaining commands one by one
For each command:
1. Identify current inputs (flags/args).
2. Define `Args` struct.
3. Implement `runCommand` in core that emits progress/results.
4. Implement CLI command wiring + compact renderer summary rules.

### Phase C — Remove legacy paths
Once all commands are ported:
- delete old direct stdout printing code paths
- enforce “no printing in core” by review + lint conventions

---

## 10) Later: `js/vscode/extension.ts` Refactor Plan (consume streams)

After the binary’s `--format=jsonl` is stable:

### 10.1 Switch invocation
Change args to include:
- `--format jsonl`
- structured flags instead of positional strings (recommended)

### 10.2 Parse NDJSON stream
Implementation outline:
- spawn the binary
- read stdout line by line
- `JSON.parse(line)` into event objects
- accumulate:
  - last `result` event’s `data` payload
  - any fatal `error`
  - terminal `done.exit_code`

### 10.3 Produce GraphQL response for `urql`
If your core emits GraphQL response objects as result payloads:
- take the final `result.data` and return it as the HTTP-equivalent body:
  - `JSON.stringify(resultPayload)` if it is already `{ data, errors }`
  - or `JSON.stringify({ data: resultPayload })` if resultPayload is raw `data` only

### 10.4 Surface diagnostics
- if `error.fatal`, throw and include `error.message`
- optionally attach streamed `log` messages to the extension output channel

---

## 11) Acceptance Criteria

### Binary
- `repo <cmd>` in compact format:
  - is readable and stable
  - has minimal noise
- `repo <cmd> --format jsonl`:
  - emits NDJSON only on stdout
  - always ends with `done`
  - includes structured `error` on failures
- All commands are implemented with streaming and respect cancellation.

### Extension (later)
- Extension works with the new CLI by parsing NDJSON and using final result.
- No reliance on raw stdout JSON.

---

## 12) Practical Tips (to keep it clean)
- Keep `events.Event` stable: treat it like an API.
- Put command-specific payloads behind `Event.Data` so event “envelope” stays consistent.
- In compact rendering, **never print raw huge JSON** by default—print summary + provide `--format jsonl` for full fidelity.
- Make partial results explicit (e.g., `result` events can include `"partial": true` inside `data` if you want).

---
