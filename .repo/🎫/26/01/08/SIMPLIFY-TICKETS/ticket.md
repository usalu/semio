# Ticket

## Todos
# Plan

- [x] Analyze current ticket structure and Go implementation
- [x] Update Go types and structures in `repo.go` (remove `TicketCheckpoint`, `CheckpointFiles`)
- [x] Add `ComputeTicketFiles` and `computeAffectedSections` to `repo.go`
- [x] Update `TicketCloseInput` and `mutationResolver.TicketClose` in `repo.go`
- [x] Update CLI in `go/cli/main.go` (remove `ticket checkpoint`, update `ticket close`)
- [x] Update MCP server in `go/mcp/main.go` (remove `ticket_checkpoint`, update `ticket_close`)
- [x] Update VSCode extension in `js/vscode/extension.ts` (remove checkpoint refs, update finish command)
- [x] Update GraphQL schema in `graphql/repo/schema.graphql` (remove checkpoint types/mutations)
- [x] Update `AGENTS.md` and `README.md` with simplified ticket mechanism
- [ ] Migrate all existing tickets to new schema (drop contributions)
- [ ] Finalize SIMPLIFY-TICKETS ticket

## Changes

## Log

## Summary
