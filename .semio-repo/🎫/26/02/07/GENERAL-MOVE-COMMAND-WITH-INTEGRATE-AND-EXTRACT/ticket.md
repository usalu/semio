---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Implemented general `move`, `integrate`, and `extract` CLI commands with MCP tool support. Added `ParseArtifactRef` for emoji-prefixed artifact ID parsing (📁 folder, 💻/📄 file, 🔖 section with #-delimited slugs), `UnSlugify`/`FindSectionBySlug`/`ResolveSectionName` for section slug resolution, `UpdateAgentsDocsPath`/`RemoveAgentsDocsEntry` for automatic AGENTS.md updates on file/folder moves. `moveCommand` dispatches on source/target kind pairs including cross-kind file→section (integrate+delete) and section→file (extract). `ToolFileMove`/`ToolFolderMove` now auto-update AGENTS.md headers. MCP handlers `sectionExtract` and `artifactMove` expose extract and move functionality. All commands wired into NewRootWithConfig and verified to compile and appear in CLI help.
## Changes

## Log

## Todos

## Plan
