# Ticket

## Summary

Fix GitHub milestone listing to use GET query parameters and update sync docs.
## Changes

- @semio-repo/go/main.go
- README.md
- AGENTS.md

## Log

- Updated milestone sync to list milestones with GET query parameters, resolve or create goal milestones by title, update stored milestone URLs, and apply milestones by title to issues.
- Normalized project and bundle `@` label validation during GitHub sync.
- Refined `sync github` documentation in README and AGENTS.

## Todos

- Run `./@semio-repo/go/go sync github` to validate milestone and label reconciliation.

## Plan

- None.
