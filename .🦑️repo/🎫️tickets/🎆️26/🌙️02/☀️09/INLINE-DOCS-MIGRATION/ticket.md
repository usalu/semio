---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Migrated all doc content from centralized AGENTS.md Codebase section and README.md Ecosystems/Bundles into distributed bundle README.md files. Renamed .bundle.md/.folder.md to README.md. Introduced BreachCodeDocsMissingReadme statute with docsPolicy and 5 tests.

## Changes

- Renamed 6 spec files (.bundle.md/.folder.md → README.md)
- Created 15 new bundle README.md files with # Summary, # Docs, # 💯️Requirements sections
- Added BreachCodeDocsMissingReadme constant, metadata, and Kinds list entry
- Implemented docsPolicy function checking for missing README.md, Summary, and Requirements
- Migrated README.md Bundles content (Code Hygiene, Repo Tooling, Ticket System, etc.) to repo/cli/README.md, .devcontainer/README.md, repo/vscode/README.md, compose/js/README.md
- Migrated README.md Ecosystems content to compose/js/README.md, compose/net/README.md, compose/py/README.md
- Replaced AGENTS.md Codebase section (~2380 lines) with compact README.md references (~80 lines)
- Replaced README.md Ecosystems+Bundles sections (~856 lines) with reference table (~33 lines)
- Updated AGENTS.md SRS section references from .bundle.md/.folder.md to README.md
- Added 5 TestDocsBreach subtests covering missing README, missing Summary, missing Requirements, clean README, and deduplication

## Log

## Todos

## Plan
