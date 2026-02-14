# Ticket

## Todos

- [x] Explore current CLI filtering implementation
- [ ] Add filter flags to ticket list/tree commands
- [ ] Add filter flags to goal list command
- [ ] Add filter flags to contributor list command  
- [ ] Add filter flags to policy list command
- [ ] Extend bundle filtering to check file kinds within bundles
- [ ] Update GraphQL schema with filter input types
- [ ] Refactor VSCode extension filter section UI
- [ ] Test all CLI commands with filters
- [ ] Update documentation (AGENTS.md, README.md)

## Changes

## Log

### 2026-01-29 11:40 - Analysis

Current state:
- CLI has `--only-<kind>` and `--no-<kind>` flags for file/folder/bundle list/tree via `bindStreamFlags()`
- `StreamOptions` struct supports: ShowIgnored, ShowGenerated, ExcludeKinds, IncludeKinds, Filter, Regex, MatchCase, MatchWholeWord
- Ticket list/tree already uses `StreamTickets()` with `StreamOptions` but doesn't expose filter flags via CLI
- Bundle/contributor/policy/goal list commands don't have filter flags exposed

Plan:
1. Add `bindStreamFlags()` to ticket list/tree, goal list, contributor list, policy list CLI commands
2. Extend kind filtering to work semantically across all types (bundles with code files, etc.)
3. Update GraphQL schema with FilterInput type
4. Refactor VSCode extension: move filter toggles to dedicated filter section

## Summary

Implemented universal filtering system across repo CLI, GraphQL API, and VSCode extension. Added --only-<kind> flags to complement --no-<kind> flags. Added --filter, --regex, --match-case, --match-whole-word flags to all list and tree commands. Extended file kind filtering to non-file commands (bundles, tickets, goals, contributors, policies) by checking associated files. Updated GraphQL schema with FilterInput type and FileKind enum. Refactored VSCode extension filter UI with file kind toggles and show all/none/default actions.
