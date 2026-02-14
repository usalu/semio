---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY
---

# Ticket

## Summary

Implemented dual ID/URI system: refactored GetArtifactID/GetArtifactURI with collection types, added IdToUri/UriToId bidirectional conversion, updated MCP resource templates/handlers, added navigate MCP tool, added semio.navigate VS Code command, extended Go and VS Code tests, updated AGENTS.md and README.md

## Changes

## Log

- Explored codebase: Go main.go (GetArtifactID, GetArtifactURI, MCP resources), VS Code extension (tree items, navigate commands, tests)
- Identified gaps: missing collection IDs, incorrect section/definition URI encoding, missing project/draft/todo MCP resources, no IdToUri/UriToId, no navigate command, statute handler uses wrong prefix
- Refactored GetArtifactID with collection types and kind-specific emoji helpers
- Refactored GetArtifactURI with section/definition slug encoding via SectionIdValueToUriPath/DefinitionIdValueToUriPath
- Added ParseSectionUriPath for reverse URI parsing
- Added IdToUri/UriToId bidirectional conversion with emoji normalization
- Updated MCP resource templates: section/definition use slash-based paths, goal uses {path}, added projects/drafts/todos resources, fixed contributor/{github} and commit/{sha}
- Updated MCP resource handlers: section/definition use ParseSectionUriPath, statute prefix fixed, goal/commit implemented, added project/draft/todo handlers
- Added navigate MCP tool returning both URI and ID
- Added semio.navigate VS Code command handling all resource types
- Extended Go tests: 43 cases in TestArtifactIDAndURI, 21 in TestIdToUri, 30 in TestUriToId, plus helper tests
- Added semio.navigate to VS Code expected commands
- Updated AGENTS.md and README.md documentation

## Todos

- [x] Refactor GetArtifactID: add collection types, fix format per spec
- [x] Refactor GetArtifactURI: section/definition slug encoding, goal path, add missing types
- [x] Add IdToUri and UriToId conversion functions
- [x] Update MCP resource templates to match new URI scheme
- [x] Update MCP resource handlers (section no #, definition no #, statute prefix, implement goal/commit)
- [x] Add projects/drafts/todos MCP resources
- [x] Add navigate MCP tool + CLI command
- [x] VS Code: add semiorepo URI handler + semio.navigate command
- [x] Extend Go tests (TestArtifactIDAndURI all types, IdToUri, UriToId)
- [x] Extend VS Code tests (navigate command registration)
- [x] Update AGENTS.md and README.md
- [x] Run all tests, ensure passing
