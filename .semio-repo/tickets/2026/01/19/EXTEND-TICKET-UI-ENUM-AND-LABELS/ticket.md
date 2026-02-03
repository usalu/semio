# Ticket

## Todos
Review ticket tooling, schemas, and VS Code extension surfaces for ticket open/close, labels, and comment metrics.
Finish mandatory UI enum wiring across ticket open inputs, persistence, GraphQL/MCP surfaces, and generated types.
Fix label derivation to include all affected bundles plus repo label and align line-metrics comment formatting.
Update VS Code extension prompts/args, codegen outputs, devcontainer attach, and related tests.
Update README.md and AGENTS.md documentation for UI enum, labels, and metrics requirements.

## Changes

## Log
- Attempted `repo ticket reopen` for EXTEND-TICKET-UI-ENUM-AND-LABELS; command reported ticket already open.
- Refreshed plan steps for UI enum, labels, metrics, devcontainer, docs, and tests.
- Added TicketUI enum wiring across GraphQL schema, Go repo GraphQL types, VS Code extension prompts, and generated types; corrected copilot-chat slug and enum names.
- Updated SQLite schema with ticket ui column, repo label derivation to include @semio-repo fallback, and metrics comment formatting to use icon + path + spaced line deltas.
- Updated Go and VS Code test fixtures for TicketUI enum; regenerated VS Code GraphQL types.
- Documented ticket UI enum, label requirements, and metrics comment format in README.md and AGENTS.md (SRS/UI/Codebase sections).

## Summary

Bulk close
