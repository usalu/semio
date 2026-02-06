---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-VSCODE-EXTENSION
---

# Ticket

## Summary

Fixed VS Code Projects/Goals loading by making repo CLI interaction author parsing backward-compatible for legacy ticket JSON author objects, with regression tests and docs updates.
## Changes
- Updated `semio-repo/cli/main.go`:
  - Added custom `Interaction.UnmarshalJSON` for tolerant author decoding.
  - Added `parseInteractionAuthor` to accept both string and object author payloads.
- Updated `semio-repo/cli/main_test.go`:
  - Added `TestInteractionUnmarshalAuthorShapes` to validate both author JSON shapes.
- Rebuilt CLI binary at `semio-repo/cli/cli` so extension runtime uses patched parser.
- Updated `README.md` bundle docs with interaction-author compatibility behavior.
- Updated `AGENTS.md` SRS and codebase sections with interaction-author compatibility requirements and file documentation.

## Log
- Ran `./semio-repo/cli/cli goal tree` initially and hit: `json: cannot unmarshal object into Go struct field Interaction.interactions.author of type string`.
- Verified widespread legacy payloads via grep for object-form `"author": { ... }` in `.semio-repo/tickets`.
- Implemented tolerant parsing in CLI interaction model.
- Ran targeted Go test:
  - `go test ./... -run TestInteractionUnmarshalAuthorShapes -count=1` (pass).
- Rebuilt CLI binary:
  - `go build -o cli main.go` in `semio-repo/cli`.
- Re-verified CLI data loading:
  - `./semio-repo/cli/cli goal tree` (pass).
  - `./semio-repo/cli/cli --json graphql 'query Goals { ... }'` (pass).
  - `./semio-repo/cli/cli --json graphql 'query RepoStructure { ... }'` (returns data without parse error).

## Todos
- None.

## Plan
1. Reproduce failure through repo CLI/GraphQL path.
2. Patch interaction author decoding for backward compatibility.
3. Add focused regression tests.
4. Verify CLI/GraphQL success paths and extension dependency binary.
5. Update README/AGENTS docs and close ticket.
