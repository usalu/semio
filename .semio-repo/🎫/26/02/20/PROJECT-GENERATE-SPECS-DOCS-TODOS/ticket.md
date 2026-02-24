---
goal: CLI-INFRA/REPO-TOOLING
---

# Ticket

## Summary

Implemented project generate requirements/docs/todos CLI commands with content-based extraction, 17 tests, zero license leaks

## Changes

- semio-repo/cli/main.go: Added `isLicenseText`, `isHeaderMetaLine` helpers, `ExtractMarkdownSection`, `ExtractFileHeaderSummary` (content-based), `ExtractFileHeaderRequirements` (content-based), `ExtractSectionLeadComments` (filters license/region markers), `ExtractDefinitionDocstring`, `findProjectByName`, `walkProjectFiles` (gitignore+generated filtering), `findFolderReadmes`, `EntityEntry` struct, `GenerateProjectRequirements`, `GenerateProjectDocs`, `GenerateProjectTodos`, `projectCommand` with generate subcommand registered in root
- semio-repo/cli/main_test.go: Added 17 tests covering isLicenseText, isHeaderMetaLine, ExtractMarkdownSection, ExtractFileHeaderSummary, ExtractFileHeaderRequirements, ExtractSectionLeadComments, GenerateProjectRequirements/Docs/Todos for coda+semio+semio-repo, invalid project error handling
- semio/SPECS.md, semio/DOCS.md, semio/TODOS.md: Generated output files (3146, 5769, 1 lines)
- semio-repo/SPECS.md, semio-repo/DOCS.md, semio-repo/TODOS.md: Generated output files (3029, 4847, 1 lines)
- coda/SPECS.md, coda/DOCS.md, coda/TODOS.md: Generated output files (25, 1, 1 lines)

## Log

- Analyzed existing CLI structure, entity types, requirements/docs/todos extraction patterns
- Implemented generate command with requirements, docs, todos subcommands
- Fixed block-index header extraction to content-based detection (avoiding license text leak)
- Fixed section lead comments to skip region markers and license sections
- Added isLicenseText (checks GNU/license/free software/warranty/redistribute/copyright keywords)
- Added isHeaderMetaLine (detects ID links, region markers, contributor years, emoji IDs)
- Fixed anyLicense logic (if ANY line in block is license, skip entire block)
- Added gitignore and IsGenerated filtering for TODOS
- Ran for all 3 projects, verified zero license text in SPECS.md and DOCS.md
- All 17 tests pass

## Todos

- [x] Implement ExtractMarkdownSection helper
- [x] Implement GenerateProjectRequirements
- [x] Implement GenerateProjectDocs
- [x] Implement GenerateProjectTodos
- [x] Add generate subcommand to projectCommand
- [x] Register projectCommand in root
- [x] Fix header extraction (content-based vs block-index)
- [x] Fix section lead comments (skip region/license)
- [x] Add tests (17 total)
- [x] Run for all projects and verify

## Plan

1. Add helper functions to extract markdown sections (Summary, Requirements, Docs) from README.md
2. Add helper functions to extract requirements/summary from code file headers and sections
3. Implement GenerateProjectRequirements/Docs/Todos that walk all entities in a project
4. Add `project <name> generate requirements|docs|todos` CLI commands
5. Register projectCommand in root
6. Extend tests
7. Run for all projects
