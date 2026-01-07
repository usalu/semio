# Plan

## Changes Required

### 1. Ticket Structure Changes
- Ticket folder name = capitalized title slug (e.g., "Fix Button" → "FIX-BUTTON")
- Create separate files: `ticket.json`, `plan.md`, `log.md`, `summary.md`
- `llm` field is a free string (not enum), stored as slug

### 2. ticket.json Format
```json
{
  "title": "Fix Button",
  "prompt": "Fix the button component...",
  "llm": "claude-opus-4",
  "status": "open",
  "author": "Name <email>",
  "date": {
    "created": "2026-01-07T10:00:00Z",
    "finished": null
  },
  "commit": "abc123...",
  "checkpoints": [...]
}
```

### 3. Checkpoint Changes
- Requires at least one file
- Computes git diff on specified files
- For each file, compute affected sections based on line ranges
- Sections have line metrics (added/removed)
- Definitions are listed under their section (no separate line metrics)
- A section/definition is affected if any diff line falls within its range

### 4. API Changes
- `ticket create <title>` with `--prompt`, `--llm`, `--plan` (optional path to move)
- `ticket checkpoint` unchanged (year/month/day/slug with --file --prompt --model)

### 5. Files to Update
- `go/repo/repo.go`: Types, CreateTicket, ReadTicket, SaveTicket, CreateCheckpoint
- `go/cli/main.go`: ticketCreateCmd parameters
- `go/mcp/main.go`: ticketCreate handler
- GraphQL schema and resolvers
