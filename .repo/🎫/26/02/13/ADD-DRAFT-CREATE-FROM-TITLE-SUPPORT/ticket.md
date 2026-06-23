---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-DRAFT-MECHANISM
---

# Ticket

## Summary

Changed draft creation to accept a title (consistent with tickets/goals) instead of a raw slug. Added VSCode commands for draft create/delete.

## Changes

- `repo/cli/main.go`: Renamed `DraftCreateInput.Slug` → `Title`, updated `CreateDraft` signature (`id` → `title`), CLI command (`create [slug]` → `create [title]`), MCP tool param (`slug` → `title`), GraphQL input type field, mutation resolver, and `ToolDraftCreate` param
- `repo/vscode/extension.ts`: Added `compose.draftCreate` (input box for title) and `compose.draftDelete` (inline action on draft tree items) commands
- `repo/vscode/package.json`: Registered `compose.draftCreate` and `compose.draftDelete` commands with icons and inline menu entries

## Log

- All draft tests pass: `TestToolDraftList`, `TestToolDraftLifecycle`, `TestParityDraftList`
- Tree node tests pass: `TestTreeNodeKindConstants`, `TestTreeNodeKindToEntityKindCoversAll`
- Go build succeeds with no errors

## Todos

- [x] Update DraftCreateInput struct: slug → title
- [x] Update CreateDraft function signature: id → title
- [x] Update CLI command: create [slug] → create [title]
- [x] Update MCP tool: slug param → title param
- [x] Update GraphQL input type: slug → title
- [x] Update GraphQL mutation resolver
- [x] Update ToolDraftCreate and callers
- [x] Add compose.draftCreate command to VSCode
- [x] Add compose.draftDelete command to VSCode

## Plan

### Problem

Draft creation accepts a raw `slug` parameter while all other entities (tickets, goals) accept a `title` that gets slugified. This is inconsistent. Additionally, the VSCode extension has no commands for draft create/delete.

### Changes

1. **`DraftCreateInput` struct**: Rename `Slug` → `Title` field
2. **`CreateDraft` function**: Rename `id` param → `title` (already calls `Slugify`)
3. **CLI command**: `create [slug]` → `create [title]`
4. **MCP tool**: `slug` param → `title` param with description "Draft title"
5. **GraphQL input type**: `slug` field → `title` field
6. **GraphQL mutation resolver**: Read `title` instead of `slug`
7. **`ToolDraftCreate`**: Rename param from `slug` → `title`
8. **VSCode extension**: Add `compose.draftCreate` and `compose.draftDelete` commands
