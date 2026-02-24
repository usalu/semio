# Hook & Event Architecture Research

## File Inventory

### 1. Primary Source Files

| File | Lines | Role |
|------|-------|------|
| `semio-repo/cli/main.go` | 40,523 | Monolithic CLI — contains ALL hook types, dispatch, config gen, command |
| `semio-repo/cli/main_test.go` | 17,659 | All tests including hook tests |
| `semio-repo/go/events.go` | 291 | Shared event kinds for CLI→server communication (EventKind) |
| `semio-repo/go/emit.go` | 51 | HTTP POST helper to send events to semio-repo server |

### 2. Hook Configuration Files (Generated)

| File | Client | Format |
|------|--------|--------|
| `.github/hooks/semio-repo.json` | copilot-chat | JSON: `{hooks: {EventName: [{type,command,timeout}]}}` |
| `.cursor/hooks.json` | cursor-chat | JSON: `{version:1, hooks: {eventName: [{command}]}}` |
| `.windsurf/hooks.json` | windsurf-chat | JSON: `{hooks: {event_name: [{command, show_output}]}}` |
| `.claude/settings.json` | claude-code | JSON: `{hooks: {EventName: [{matcher, hooks:[{type,command}]}]}, permissions: {...}}` |
| `.factory/hooks.json` | droid | JSON: `{hooks: {EventName: "command"}}` |

---

## Architecture Overview

The hook system has **3 layers**:

```
┌─────────────────────────────────────────────────┐
│  Layer 1: Native Client Events (Inlet Adapter)  │
│  Each editor sends its own native event names    │
│  (e.g. Copilot: "PreToolUse", Cursor: "preToolUse", │
│   Windsurf: "pre_run_command", Claude: "PreToolUse") │
└────────────────────┬────────────────────────────┘
                     │ ResolveHookEvent()
                     ▼
┌─────────────────────────────────────────────────┐
│  Layer 2: Neutral HookEvents (Canonical)        │
│  15 canonical events that map 1:1 across editors│
│  e.g. "agent.tool.terminal.starting"            │
└────────────────────┬────────────────────────────┘
                     │ dispatchHook() → RunHook()
                     ▼
┌─────────────────────────────────────────────────┐
│  Layer 3: HookResult (typed per event)          │
│  Dispatched, logged, tracked in open tickets    │
│  Formatted back to client-native format (Outlet)│
└─────────────────────────────────────────────────┘
```

---

## Event Type Definitions

### A. Engine Events (`main.go` L77-319)

Internal streaming events for CLI command execution (NOT hooks):

| Type | Value | Purpose |
|------|-------|---------|
| `Kind` (string) | `start`, `log`, `progress`, `result`, `artifact`, `error`, `done` | Stream event kinds |
| `Event` (struct) | L98 | Carries command output data |
| `Progress` | L287 | Progress bar data |
| `Artifact` | L296 | URI-based output |
| `ErrPayload` | L304 | Error details |
| `DonePayload` | L313 | Exit code + status |

### B. Shared EventKind (`semio-repo/go/events.go` L17-47)

CLI→server events for state changes (ticket, goal, todo, file, folder, section operations):

```go
type EventKind string  // e.g. "ticket.open", "goal.close", "commit", etc.
```

27 event kinds total: `ticket.{open,close,reopen,change}`, `goal.{open,close,reopen,change}`, `contributor.{add,remove}`, `commit`, `todo.{create,change,delete}`, `draft.{create,delete}`, `file.{create,move,delete}`, `folder.{create,move,delete}`, `section.{create,move,delete}`, `integrate`, `extract`

### C. HookEvent (`main.go` L34476-34521)

Agent lifecycle hooks — the core of this research:

```go
type HookEvent string
```

| Constant | Value | Kind |
|----------|-------|------|
| `HookGitCommitStarting` | `git.commit.starting` | git |
| `HookGitCommitEnded` | `git.commit.ended` | git |
| `HookAgentStarted` | `agent.started` | agent |
| `HookAgentEnded` | `agent.ended` | agent |
| `HookAgentPromptSubmitting` | `agent.prompt.submitting` | agent |
| `HookAgentCompacting` | `agent.compacting` | agent |
| `HookAgentToolStarting` | `agent.tool.starting` | agent |
| `HookAgentToolEnded` | `agent.tool.ended` | agent |
| `HookAgentToolPlanUpdating` | `agent.tool.plan.updating` | agent |
| `HookAgentToolSearching` | `agent.tool.searching` | agent |
| `HookAgentToolSearched` | `agent.tool.searching.ended` | agent |
| `HookAgentToolCodeEditing` | `agent.tool.code.editing` | agent |
| `HookAgentToolCodeEdited` | `agent.tool.code.edited` | agent |
| `HookAgentToolTerminalStarting` | `agent.tool.terminal.starting` | agent |
| `HookAgentToolTerminalEnded` | `agent.tool.terminal.ended` | agent |

---

## Key Structs & Functions (with line numbers)

### Types (main.go)

| Line | Name | Purpose |
|------|------|---------|
| 34483 | `HookEvent` | Lifecycle event kind string |
| 34505 | `AllHookEvents` | Slice of all valid events |
| 34525 | `HookKind` | "git" or "agent" |
| 34534 | `HookEventKind()` | Returns kind for event |
| 34545 | `HookContext` | Input metadata for hook execution |
| 34560 | `HookPlanStep` | Plan step name+status |
| 34567 | `HookResult` | Interface: IsAllowed(), GetMessage() |
| 34574 | `HookResultBase` | Base: Allowed, Message, Raw |
| 34585 | `HookResultAgentBase` | Agent base: session, timestamp, client, llm, transcript, messageID, parent |
| 34598 | `HookResultAgentStarted` | Agent started |
| 34604 | `HookResultAgentEnded` | Agent ended |
| 34610 | `HookResultAgentPromptSubmitting` | Prompt + base |
| 34617 | `HookResultAgentCompacting` | Chat + base |
| 34624 | `HookResultAgentToolStarting` | Name + Input + base |
| 34632 | `HookResultAgentToolEnded` | Name + Input + Response + base |
| 34641 | `HookResultAgentToolPlanUpdating` | Steps + base |
| 34648 | `HookResultAgentToolSearching` | Query + Include + Exclude + base |
| 34657 | `HookResultAgentToolSearched` | Query + Include + Exclude + Response + base |
| 34667 | `HookResultAgentToolCodeEditing` | Path + Old + New + All + base |
| 34677 | `HookResultAgentToolCodeEdited` | Path + Old + New + base |
| 34686 | `HookResultAgentToolTerminalStarting` | Command + base |
| 34693 | `HookResultAgentToolTerminalEnded` | Command + PID + Terminated + Stdout + Stderr + base |
| 34703 | `HookResultGitCommitStarting` | Message + base |
| 34711 | `HookResultGitCommitEnded` | SHA + base |
| 34719 | `HookLogResponse` | Audit log response |
| 34727 | `HookLogEntry` | Raw + Event + Response for audit |
| 34735 | `BlockedToolPatterns` | Denied git commands |
| 9633 | `EditorHookMapping` | Client + ConfigPath |

### ToolKind Classification (main.go)

| Line | Name | Purpose |
|------|------|---------|
| 34926 | `ToolKind` | plan, code_search, code_edit, terminal, generic |
| 34940 | `classifyTool()` | Maps tool names to ToolKind |
| 34975 | `classifyCommandKind()` | Maps shell commands to ToolKind |
| 35046 | `resolvePreToolUse()` | ToolKind → pre-event |
| 35061 | `resolvePostToolUse()` | ToolKind → post-event |

### Client-specific Inlet Adapters (main.go)

| Line | Name | Client |
|------|------|--------|
| 35076 | `resolveCopilotEvent()` | copilot-chat |
| 35105 | `resolveCursorEvent()` | cursor-chat |
| 35158 | `resolveWindsurfEvent()` | windsurf-chat |
| 35201 | `resolveClaudeCompatibleEvent()` | claude-code, droid, codex, antigravity |
| 35227 | `ResolveHookEvent()` | Master resolver: tries neutral first, then client adapter |

### Outlet Adapter

| Line | Name | Purpose |
|------|------|---------|
| 35248 | `vsCodeEventFromHookEvent()` | HookEvent → VS Code native name |
| 35295 | `formatVSCodeHookOutput()` | Produces VS Code JSON output |

### Core Logic (main.go)

| Line | Name | Purpose |
|------|------|---------|
| 35420 | `logHook()` | Audit log to `.semio-repo/📜/🪝/🤖/<session>/<ts>_<kind>.json` |
| 35456 | `dispatchHook()` | Event → typed result via switch |
| 35584 | `RunHook()` | Sets root, dispatches, logs, tracks ticket |
| 35592 | `trackHookInOpenTicket()` | Records hook events in the latest open ticket's agents |
| 36392 | `hookCommand()` | Cobra command: `hook <event> <client>` |

### EditorProvider Interface (main.go)

| Line | Name | Purpose |
|------|------|---------|
| 9640 | `EditorProvider` | Interface: Kind, Configure, ResolveNativeEvent, FormatHookOutput, NativeEventFromHookEvent, GenerateHookConfig, HookMapping |
| 9953 | `CopilotEditorProvider` | copilot-chat → `.github/hooks/semio-repo.json` |
| 9992 | `CursorEditorProvider` | cursor-chat → `.cursor/hooks.json` |
| 10032 | `WindsurfEditorProvider` | windsurf-chat → `.windsurf/hooks.json` |
| 10072 | `ClaudeCodeEditorProvider` | claude-code → `.claude/settings.json` |
| 10112 | `DroidEditorProvider` | droid → `.factory/hooks.json` |
| 10152 | `CodexEditorProvider` | codex → no config (empty) |
| 10179 | `AntigravityEditorProvider` | antigravity-chat → no config (empty) |
| 10215 | `AllEditorProviders()` | Returns all 7 providers |
| 10229 | `GetEditorProvider()` | Lookup by client slug |

### Config Generation (main.go)

| Line | Name | Purpose |
|------|------|---------|
| 36520 | `configureCommand()` | `configure` cobra command |
| 36568 | `configureGitHooks()` | Writes `.git/hooks/pre-commit` and `post-commit` |
| 36640 | `generateCopilotConfig()` | Copilot hooks JSON |
| 36680 | `generateCursorConfig()` | Cursor hooks JSON |
| 36762 | `generateWindsurfConfig()` | Windsurf hooks JSON |
| 36816 | `generateClaudeCodeConfig()` | Claude Code settings JSON (merges existing) |
| 36853 | `generateDroidConfig()` | Droid hooks JSON |

### Ticket Agent Event Tracking

| Line | Name | Purpose |
|------|------|---------|
| 12576 | `TicketAgentEventSectionRef` | Section ID + definition refs |
| 12583 | `TicketAgentEventCodeBlock` | Sections + LOC for old/new code |
| 12590 | `TicketAgentEvent` | Kind, Timestamp, Prompt, Query, File, Denied, Line, Old, New |
| 12604 | `TicketAgent` | Session, Contributor, System, Client, LLM, Transcript, Plan, Events, Diff |

### Blocked Tool Patterns (main.go L34735-34743)

Always-denied shell commands:
- `git checkout`
- `git stash` / `git stash pop` / `git stash drop` / `git stash apply`
- `git reset --hard`
- `git clean -fd`

---

## Test Coverage (main_test.go)

| Line | Test Function | Coverage |
|------|---------------|----------|
| 9511 | `TestTrackHookInOpenTicketUsesStableSessionIDs` | Session ID stability across hooks |
| 13846 | `#region Hook Tests` | Region start |
| 13848 | `TestValidateHookEvent` | All 15 events + invalid case |
| 13892 | `TestHookEventKind` | git vs agent classification |
| ~13920 | `TestIsToolBlocked` / `TestIsToolAllowed` | Blocked pattern matching |
| 14019 | `TestRunHookAgentEvents` | All agent events + git commit starting |
| 14067 | `TestRunHookToolBlocking` | Tool blocking via HookAgentToolStarting |
| 14085 | `TestRunHookToolAllowed` | Allowed tool passes |
| ~15990-16010 | `TestDispatchHookPromptSubmitting` | Prompt extraction |
| ~16015-16030 | `TestDispatchHookCompacting` | Chat extraction |
| ~16035-16050 | `TestDispatchHookToolStarting` | Tool name + input extraction |
| ~16065-16085 | `TestDispatchHookToolEnded` | Tool response extraction |
| ~16090-16110 | `TestDispatchHookPlanUpdating` | Plan steps extraction |
| ~16120-16145 | `TestDispatchHookToolSearching` | Query + include/exclude extraction |
| ~16150-16195 | `TestDispatchHookCodeEditing` | Path + old/new/all extraction |
| ~16195-16215 | `TestDispatchHookCodeEdited` | Code edited result |
| ~16220-16240 | `TestDispatchHookTerminalStarting` | Terminal command extraction |
| ~16240-16265 | `TestDispatchHookTerminalEnded` | Terminal ended fields |
| ~16270-16295 | `TestDispatchHookGitCommitEnded` | SHA + message extraction |
| ~16300-16320 | `TestDispatchHookGitCommitStarting` | Pre-commit result |
| ~16320-16365 | Agent-specific extraction tests | Session, parent, tool name from stdin |
| ~16400-16460 | Code search/edit extraction tests | extractCodeSearchFromInput, extractCodeEditFromInput |
| 16693 | `TestHookResultJSONFields` | JSON serialization |
| 16727 | `TestHookResultOmitEmpty` | omitempty behavior |
| 16742 | `TestNativeHookEventMappingWithRealData` | 90+ real-world event mappings across all 7 clients |
| 16924 | `TestNativeHookEventMappingFromRealLogFiles` | Regression tests from actual log files |
| 17015 | `#endregion Hook Tests` | Region end |
| 17326 | `TestEditorProviderHookMapping` | All providers have non-empty client |

---

## Data Flow

### Invocation (from client hook config → CLI)

```
Editor Native Event (e.g. Copilot "PreToolUse") + stdin JSON payload
    ↓
./semio-repo/cli/cli hook <nativeEvent> <client> [--tool-name X] [--tool-args Y]
    ↓
hookCommand() cobra handler
    ↓
ResolveHookEvent(eventStr, client, toolName, stdinJSON)
    ↓  (tries neutral first, then client-specific adapter)
HookContext{Event, Client, Timestamp, RepoRoot, ToolName, ToolArgs, FilePath, ParentInfo, Input}
    ↓
RunHook(hctx)
    ├─ dispatchHook(hctx) → typed HookResult
    ├─ logHook(hctx, result) → writes .semio-repo/📜/🪝/🤖/<session>/<ts>_<kind>.json
    └─ trackHookInOpenTicket(hctx, result) → appends to latest open ticket's TicketAgent.Events
    ↓
Format output for client (e.g. VSCode hookSpecificOutput JSON) → stdout
```

### Tool Classification Flow

```
toolName (e.g. "run_in_terminal")
    ↓ classifyTool()
ToolKind (e.g. ToolKindTerminal)
    ↓ If terminal/generic, check stdin for command
    ↓ classifyCommandKind() (e.g. "grep" → ToolKindCodeSearch)
    ↓ resolvePreToolUse() / resolvePostToolUse()
HookEvent (e.g. HookAgentToolTerminalStarting)
```

---

## Native Event Mappings Summary

### Copilot Chat (VS Code) — PascalCase
`SessionStart` → `agent.started`
`Stop` → `agent.ended`
`SubagentStart` → `agent.started` (parent="subagent")
`SubagentStop` → `agent.ended` (parent="subagent")
`UserPromptSubmit` → `agent.prompt.submitting`
`PreCompact` → `agent.compacting`
`PreToolUse` → depends on tool kind
`PostToolUse` / `PostToolUseFailure` → depends on tool kind

### Cursor — camelCase
`sessionStart`, `sessionEnd`, `stop`, `subagentStart`, `subagentStop`, `beforeSubmitPrompt`, `preCompact`, `preToolUse`, `postToolUse`, `postToolUseFailure`, `beforeMCPExecution`, `afterMCPExecution`, `beforeReadFile`, `afterFileEdit`, `beforeShellExecution`, `afterShellExecution`, `afterAgentResponse`, `afterAgentThought`, `beforeTabFileRead`, `afterTabFileEdit`

### Windsurf — snake_case
`pre_user_prompt`, `post_cascade_response`, `post_setup_worktree`, `pre_mcp_tool_use`, `post_mcp_tool_use`, `pre_read_code`, `post_read_code`, `pre_write_code`, `post_write_code`, `pre_run_command`, `post_run_command`

### Claude Code / Droid / Codex / Antigravity — PascalCase
`SessionStart`, `SessionEnd`, `SubagentStart`, `SubagentStop`, `Stop`, `UserPromptSubmit`, `PreCompact`, `TaskCompleted`, `Notification`, `TeammateIdle`, `PermissionRequest`, `PreToolUse`, `PostToolUse`, `PostToolUseFailure`
