# Ticket

## Summary

Fix sync github label synchronization by reconciling GitHub repository @ labels with local project and bundle labels and verify full coverage.

## Changes

- repo/cli/main.go
- repo/cli/cli
- README.md
- AGENTS.md
- .repo/tickets/2026/02/04/SYNC-GITHUB-COMMAND/ticket.md

## Log

- Updated milestone sync to list milestones with GET query parameters, resolve or create goal milestones by title, update stored milestone URLs, and apply milestones by title to issues.
- Normalized project and bundle `@` label validation during GitHub sync and added repository-wide issue label sweep for invalid `@` labels.
- Refined `sync github` documentation in README and AGENTS.
- Validation: `go build ./...` passes in `repo/cli`; `go test ./...` currently fails due pre-existing unrelated test/environment expectations (`TEST-GOAL` already exists and formatter output mismatches).
- Ran `./repo/cli/cli sync github` successfully.
- Audited GitHub issues and found `0` invalid labels starting with `@`.
- Final verification confirms full repository `@` label coverage from local projects and bundles.
- Added repository label catalog sync that creates missing valid `@` labels and deletes invalid `@` labels.
- Rebuilt CLI binary with `go build -o go .` in `repo/cli`.
- Verified repository labels against `./go bundle list` + project labels: expected `32`, missing `0`, invalid `0`.

## Todos

- None.

## Plan

- None.
