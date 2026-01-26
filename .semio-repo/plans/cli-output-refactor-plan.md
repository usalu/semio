# Refactor Plan: Human-First, Colored, Streaming CLI Output (NDJSON via `--json`)

> Goal: **Every CLI command defaults to human + LLM-friendly output** (concise, visual, colored), **streamed live to the console**.  
> If `--json` is provided, output is **pure NDJSON** (one JSON object per line) suitable for piping and machine parsing.  
> **No backwards compatibility**: remove legacy `--format`, remove JSON pretty-print defaults, and delete old renderers.

---

## 0) Current State (what we’re changing)

Today, CLI output selection is controlled by `Config.Format` and `renderStream()`:

- `RenderJSONL()` streams JSON events (line-delimited JSON).
- `RenderJSON()` buffers all events then prints a JSON array.
- `RenderCompact()` prints either errors/logs to stderr and JSON-pretty payloads for items/results to stdout.

This means that the “default” output is still fundamentally **JSON payloads** (`event.Data` pretty-printed), not a human/LLM-focused display.

We will replace this with:

- **Default renderer:** `HumanRenderer` (colored, concise, semantic formatting)
- **Machine renderer:** `NDJSONRenderer` enabled only by `--json`

---

## 1) New CLI Contract (final behavior)

### 1.1 Default output (no flags)
- **Always** human-readable lines (not JSON)
- **Colored** (ANSI) when writing to a TTY
- **LLM-concise**: short, consistent, “summary first” messages that can be skimmed and pasted into an LLM prompt
- **Streaming:** every event is rendered as it arrives; no buffering until the end

### 1.2 `--json` output
- Writes **pure NDJSON** to stdout:
  - **One JSON object per line**
  - No surrounding array
  - No additional human text, no color codes
- Events must be emitted as-is (or in a stable schema) so consumers can parse incrementally.

> Notes:
> - NDJSON is already supported by the existing `RenderJSONL()`, but we will **rename/standardize** it as `NDJSONRenderer` and ensure it never prints anything else.
> - Remove `--format=jsonl|json|compact` entirely.

### 1.3 Stderr contract
- Human mode:
  - **Errors** and **logs** go to stderr, colored (red/yellow/dim).
  - Result summaries and item summaries go to stdout.
- JSON mode:
  - **Everything goes to stdout** as NDJSON, including errors/logs/progress/done events (one line per event).
  - Stderr should be used only for **fatal runtime failures of the CLI itself** (e.g., cannot marshal JSON, cannot write).

---

## 2) Remove Legacy Surface Area (no backwards compatibility)

### 2.1 Delete / remove
- `Config.Format` field
- CLI flag that sets it (`--format`)
- `RenderJSON()` (buffered array output)
- `RenderCompact()` (pretty JSON payloads)
- `renderStream()` switch on format strings

### 2.2 Replace with
- `Config.JSON bool` (or `OutputMode enum { Human, NDJSON }`)
- Root flag: `--json` (bool)
- Single entry point: `renderStream(cmd, config, stream)` that selects:
  - `HumanRenderer` when `!config.JSON`
  - `NDJSONRenderer` when `config.JSON`

---

## 3) Streaming Guarantees (real-time console output)

### 3.1 Enforce flush-after-write
Even though `os.Stdout` is typically unbuffered, wrappers and downstream piping can add buffering. To ensure streaming:

- Wrap stdout and stderr in `bufio.Writer`
- After **every rendered event**, call `Flush()`
- Avoid `fmt.Println` for hot-path; use `writer.WriteString()` + `Flush()`

### 3.2 Progress rendering
- If stdout is a TTY:
  - Render progress updates on a **single line** using `\r` (carriage return), then overwrite.
  - On completion or next non-progress line, print a newline.
- If not a TTY:
  - Print discrete progress lines periodically (e.g., every 5% or every N items) to avoid massive logs.

### 3.3 Cancellation behavior
- If context is canceled, ensure the renderer prints a final, clear line (human mode) and that `Done` event still flushes.

---

## 4) New Output Design (human + LLM concise)

### 4.1 Principles
- **One event → one small unit of output**
- **Stable prefixes** so an LLM can parse quickly:
  - `✓` success / done
  - `!` warning
  - `✗` error
  - `•` log/info
  - `→` item / result
- **Short lines**; avoid printing huge JSON blobs by default.
- **Summarize** item/result payloads into:
  - a primary label (kind)
  - 1–3 key fields (IDs, paths, counts)
  - optionally: “(+N more)” or “use --verbose” if needed

### 4.2 Add a dedicated `HumanFormatter` per command (recommended)
The engine emits generic `Event` items with `Meta.kind` and `Data`. For high-quality human output we should introduce a mapping:

- Command ID → `Formatter`
- `Formatter` knows how to summarize its events:
  - e.g., `analyze` events: show `ViolationKind`, file path, line, message
  - `ticket.create`: show created ticket ID, paths
  - `folder.tree`: show tree lines as-is (already human)

This avoids showing raw internal structs.

### 4.3 Fallback formatting (safe default)
If no command-specific formatter exists:
- Decode `event.Data` into `map[string]any` (or struct) and produce:
  - `→ <kind>: <id/name/path if present>`
  - If nothing obvious: `→ <kind>: (data)` with a short JSON snippet (truncate to N chars)

### 4.4 Color palette (ANSI)
- Green: success / done
- Yellow: warnings
- Red: errors
- Cyan/Blue: headings / key objects
- Dim: logs and debug

Respect `NO_COLOR` if you want to be a good citizen, but the requirement asks for colored output—so default to color when TTY.

---

## 5) Implementation Steps (ordered, concrete)

### Step 1 — Introduce output mode and flag
1. Replace:
   ```go
   type Config struct {
     Format string
     ...
   }
   ```
   with:
   ```go
   type Config struct {
     JSON    bool
     Verbose bool
     Repo    string
     Timeout time.Duration
   }
   ```
2. Add root persistent flag:
   - `--json` (bool)
3. Remove:
   - `--format`
   - any defaulting logic for `Format`

### Step 2 — Create renderer interface
Add:
```go
type StreamRenderer interface {
  Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (exitCode int, err error)
}
```

### Step 3 — Implement NDJSONRenderer (pure NDJSON)
- Based on existing `RenderJSONL()`, but:
  - Ensure it encodes exactly `Event` objects, one per line
  - No extra blank lines, no other text
  - `encoder.SetEscapeHTML(false)`
  - Flush after each `Encode()` if using buffered writer

### Step 4 — Implement HumanRenderer (colored, streamed)
Core logic:
- Track:
  - lastProgressWasInline bool
  - currentExitCode
- Switch on event.Kind:
  - `KindError`: print red `✗` line(s) to stderr
  - `KindLog`: print dim `•` line to stderr
  - `KindProgress`: update inline progress line (TTY) or occasional lines (non-TTY)
  - `KindItem` / `KindResult`: print `→` summarized output to stdout
  - `KindDone`: print green `✓` completion summary + exit code if nonzero

**Important:** do *not* pretty-print JSON payloads. Human mode must not emit JSON.

### Step 5 — Replace `renderStream()` with new selection
Replace the format switch with:

```go
func renderStream(cmd *cobra.Command, config *Config, stream <-chan Event) error {
  out := cmd.OutOrStdout()
  errOut := cmd.ErrOrStderr()

  if config.JSON {
     exit, err := NDJSONRenderer{}.Render(cmd.Context(), out, errOut, stream)
     ...
  }
  exit, err := HumanRenderer{Verbose: config.Verbose}.Render(cmd.Context(), out, errOut, stream)
  ...
}
```

### Step 6 — Add command-specific formatters (high value commands first)
Create:
```go
type EventFormatter interface {
  FormatItem(meta map[string]any, data any) (lines []StyledLine)
  FormatResult(data any) (lines []StyledLine)
}
```

Wire formatters by command:
- analyze
- fix
- ticket.create / ticket.list / ticket.close
- codebase.* / folder.* / file.* / section.* / definition.*

For each, define the “LLM concise” summary fields.

### Step 7 — Introduce lightweight styling utilities
This must not become a giant dependency. A small internal package is enough:

- `IsTTY(w io.Writer) bool` (best-effort)
- ANSI helpers:
  - `Colorize(s, color)` where color is enum
  - `Bold(s)`
  - `Dim(s)`
- `Truncate(s, max)` and `OneLineJSONSnippet(any, max)`

### Step 8 — Tests
Add tests for renderers:

1. **NDJSONRenderer**
   - Given a stream of N events, output has N lines
   - Each line parses as JSON
   - No extra prefix/suffix text

2. **HumanRenderer**
   - No line begins with `{` unless payload text actually contains it (guard against accidental JSON)
   - Errors go to stderr, results to stdout
   - Progress uses `\r` only in TTY mode (mock TTY detection)

3. **Exit codes**
   - `KindDone.ExitCode` is returned and converted into `ExitError` consistently

### Step 9 — Remove dead code and update docs
- Delete old renderer functions and config fields
- Update `--help` output and README examples
- Ensure examples show:
  - default colored output
  - `--json` piping examples

---

## 6) Acceptance Criteria (what “done” means)

### CLI behavior
- Running any command without flags prints **human-readable**, **colored**, **non-JSON** output.
- Output appears **incrementally** during long-running commands (progress and items), not only at the end.
- `--json` produces **strict NDJSON**, no color codes, no human text.

### Compatibility
- `--format` is gone.
- Any previous JSON array output is gone.
- No “legacy mode”.

### Developer experience
- Formatters are easy to add per command.
- Fallback formatting is safe, short, and never dumps full JSON by default.

---

## 7) Migration Notes (internal)

### MCP / VSCode server modes
These are not “human CLI output”; they are protocol endpoints. Keep their JSON protocol behavior unchanged.

However:
- Ensure they do not accidentally inherit `--json` or human-mode rendering if they share root config.
- Prefer explicit command config defaults:
  - `semio vscode` and `semio mcp` ignore human renderer and always speak their protocol.

---

## 8) Suggested Output Examples (target)

### Human mode
```
• analyze: scanning 1,284 files (repo: semio)
↻  42%  (540/1284 files)

→ violation  high  sketchpad/import-third-party  js/sketchpad/foo.tsx:12
→ violation  low   dev-docs/missing-file         README.md

✓ done  analyze  (2 violations)  3.2s
```

### JSON mode (`--json`)
```
{"kind":"log","command":"analyze","message":"scanning 1284 files"}
{"kind":"progress","command":"analyze","progress":{"current":540,"total":1284,"percent":42,"step":"files"}}
{"kind":"item","command":"analyze","data":{...},"meta":{"stream":"items","kind":"violation"}}
{"kind":"done","command":"analyze","done":{"exit_code":0,"status":"ok","summary":{...}}}
```

---

## 9) Work Breakdown (tickets)

1. **CLI flag + Config refactor** (remove `Format`, add `--json`)
2. **NDJSONRenderer** (pure NDJSON, flush per event)
3. **HumanRenderer MVP** (log/error/result/item + progress)
4. **TTY detection + ANSI style helpers**
5. **Command formatters** (analyze/fix/tickets first)
6. **Tests for renderers and exit codes**
7. **Docs refresh + examples**
8. **Delete legacy renderer code**

---

## 10) Key Implementation Details (gotchas)

- **Don’t JSON-indent** anything in human mode.
- Ensure **every `Write` is followed by `Flush`** when using buffered writers.
- If using inline progress updates, always emit a newline before printing a non-progress line.
- Guard against large payloads: truncate aggressively in human mode unless `--verbose`.

---

## Appendix: Minimal code skeleton (for orientation)

```go
// Config
type Config struct {
  JSON    bool
  Verbose bool
  Repo    string
  Timeout time.Duration
}

type StreamRenderer interface {
  Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error)
}

type NDJSONRenderer struct{}
type HumanRenderer struct{ Verbose bool }

// renderStream
func renderStream(cmd *cobra.Command, config *Config, stream <-chan Event) error {
  var r StreamRenderer
  if config.JSON { r = NDJSONRenderer{} } else { r = HumanRenderer{Verbose: config.Verbose} }

  exit, err := r.Render(cmd.Context(), cmd.OutOrStdout(), cmd.ErrOrStderr(), stream)
  if err != nil { return err }
  if exit != 0 { return ExitError{Code: exit} }
  return nil
}
```

---

**End of plan.**
