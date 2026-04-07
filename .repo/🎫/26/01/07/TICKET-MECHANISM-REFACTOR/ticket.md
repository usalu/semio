# Ticket

## Todos

# Plan

# Previously

# Plan

# Changes

## Changes

## Log

## 2026-01-07

- Created ticket for refactoring ticket mechanism
- Analyzed current ticket structure in `./repo/cli/cli.go`
- Current: Uses YAML frontmatter in `ticket.md`
- New: Uses separate `ticket.json` + `plan.md` + `log.md` + `summary.md`
- Key changes: title instead of slug as input, llm is free string, plan can be moved from external file

Starting implementation...

### Changes Made

#### Type Changes

- Added `Definitions []string` field to `CheckpointSectionContrib`
- Removed `CheckpointDefinitionContrib` type (merged into sections)
- Added `TicketData` struct for new JSON format with Title, Prompt, LLM, Summary, Status, Author, Date, Commit, Checkpoints
- Updated `Ticket` struct with Data, JsonPath, PlanPath, LogPath, SummaryPath fields
- Added accessor methods to Ticket: GetTitle, GetPrompt, GetLLM, GetSummary, GetStatus, GetAuthor, GetDateCreated, GetDateFinished, GetCommit, GetCheckpoints

#### Function Changes

- Added path helper functions: GetTicketJsonPath, GetTicketPlanPath, GetTicketLogPath, GetTicketSummaryPath
- Updated `CreateTicket(title, prompt, llm, planPath string)` - creates ticket.json, plan.md, log.md, summary.md
- Added `CreateTicketLegacy(slug, prompt, model string)` for backwards compatibility
- Updated `ReadTicket` to support both JSON (new) and YAML frontmatter (legacy) formats
- Added `SaveTicketNew` for saving JSON format tickets
- Updated `CreateCheckpoint` to handle both formats
- Updated `FinishTicket` and `ReopenTicket` to handle both formats
- Updated `ListTickets` to detect both JSON and YAML format tickets

#### CLI Changes

- Updated `ticketOpenCmd` to use `<title>` instead of `<slug>`
- Changed `--model` flag to `--llm`
- Added `--plan` flag for optional plan file path

#### MCP Changes

- Updated `ticket_open` tool with `title`, `llm`, `planPath` parameters

#### GraphQL Changes

- Updated `TicketOpenInput` with title, prompt, llm, planPath fields
- Added `title` and `llm` fields to Ticket type
- Updated resolver for ticketOpen mutation

## Summary

# Summary

Refactored the ticket mechanism with the following changes:

1. **New file structure**: Tickets now create `ticket.json`, `plan.md`, `log.md`, `summary.md` instead of a single `ticket.md` with frontmatter
2. **Title-based naming**: Ticket folder name is derived from the title (capitalized slug)
3. **Flexible LLM field**: The `llm` field is now a free string that gets slugified
4. **Plan file support**: Optional `--plan` flag to move an existing markdown file to `plan.md`
5. **Checkpoint section metrics**: Checkpoints compute affected sections from git diffs, with definitions listed under their parent sections
