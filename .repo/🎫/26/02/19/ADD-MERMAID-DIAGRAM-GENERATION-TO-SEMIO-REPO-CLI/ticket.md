---
goal: SEMIO-REPO-CLI/MERMAID-DIAGRAMS
---

# Ticket

## Summary

Add `mermaid` CLI command with three subcommands generating treemap-beta mermaid diagram strings for LOC visualization.

## Changes

- repo/cli/main.go: Added mermaid command with loc-by-projects-bundles-folders-files, loc-by-contributors, loc-by-language subcommands
- repo/cli/main_test.go: Added tests for all three mermaid subcommands

## Log

- 2026-02-19: Ticket opened. Implementing mermaid command in CLI Adapter section.

## Todos

- [x] Implement MermaidLocByProjectsBundlesFoldersFiles function
- [x] Implement MermaidLocByContributors function
- [x] Implement MermaidLocByLanguage function
- [x] Add mermaidCommand with subcommands to root
- [x] Add tests
- [x] Build and verify
- [x] Close ticket

## Plan

1. Add `MermaidLocByProjectsBundlesFoldersFiles` function that walks all projects→bundles→folders→files counting LOC and building a treemap-beta mermaid string.
2. Add `MermaidLocByContributors` function that uses git log to attribute lines to contributors.
3. Add `MermaidLocByLanguage` function that groups files by language extension and counts LOC.
4. Add `mermaidCommand` as a cobra command group with three subcommands.
5. Register in `NewRootWithConfig`.
6. Add unit tests in main_test.go.
