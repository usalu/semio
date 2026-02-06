# Ticket

## Todos

# Plan: Extend Ticket Mechanism With Goals And Subissues

## Goals

- Introduce `Goals` (GitHub Milestones).
  - Title, Description.
  - Assigned to tickets.
  - Sync with GitHub (Create/Edit/Close/Delete).
- Support Parent Tickets (GitHub Sub-issues).
- Support Ticket Delete (Delete GitHub Issue).
- No backward compatibility.

## Tasks

### 1. Repo CLI (`go/repo/main.go`) - Goals

- [ ] Implement `Goal` struct and storage (json/sqlite?).
- [ ] Implement `gh milestone` wrappers.
- [ ] Implement `semio repo clial list`, `create`, `update`, `delete` commands.
- [ ] Expose via GraphQL.

### 2. Repo CLI (`go/repo/main.go`) - Tickets

- [ ] Update `TicketData` with `Goal` and `Parent`.
- [ ] Update `repo ticket open` to accept `--goal` and `--parent`.
- [ ] Update `gh issue create` to include milestone and parent.
- [ ] Implement `repo ticket delete`.

### 3. GraphQL Schema & VS Code

- [ ] Update `graphql/repo/schema.graphql`.
- [ ] Update `js/vscode/extension.ts`.

### 4. Verification

- [ ] Verify `goal` lifecycle.
- [ ] Verify `ticket` with parent and goal.
- [ ] Verify `ticket delete`.

## Changes

## Log

## Summary

Bulk close
