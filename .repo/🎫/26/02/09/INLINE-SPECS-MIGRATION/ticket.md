---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Migrated requirements from AGENTS.md into distributed .bundle.md/.folder.md files with new code:requirements:implementation-syntax statute. Created requirementsPolicy, isSpecText, hasImplementationSyntax helpers, ScanComments spec exemptions. 11 new test subtests all passing.

## Changes

### repo/cli/main.go

- Added `BreachCodeRequirementsSyntax` constant and metadata entry
- Added `specKeywordPattern` regex for RFC 2119 keyword detection
- Added `isSpecText()`, `hasImplementationSyntax()` helpers
- Added `specLineCache` to PolicyContext with `SpecLines()`, `IsSpecLine()`, `IsSpecBlock()` methods
- Added nil guards to `ReadText()` and `Sections()` for empty PolicyContext in tests
- Implemented `requirementsPolicy()` scanning Header Requirements regions and section-start spec comments
- Wired `requirementsPolicy` into `codePolicy` Kinds list and Run function
- Modified `BaseLanguage.ScanComments` at 3 code points to exempt spec lines/blocks
- Modified `TypeScriptLanguage.ScanComments` at 2 code points to exempt spec lines/blocks
- Updated main.go header Requirements region to be implementation-agnostic

### repo/cli/main_test.go

- Added `TestRequirementsBreach` with 11 subtests covering isSpecText, hasImplementationSyntax, requirementsPolicy breachs, spec exemptions, and metadata

### New spec files

- semio/.bundle.md - Domain model requirements
- repo/cli/.bundle.md - Repo CLI requirements
- repo/vscode/.bundle.md - VS Code Extension requirements
- semio/engine/.bundle.md - Engine requirements
- .devcontainer/.folder.md - Devcontainer requirements
- semio/js/sketchpad/.folder.md - Sketchpad UI requirements

### AGENTS.md

- Cleaned up SRS section: removed ~580 lines of inline requirements, replaced with spec location index
- Updated documentation policy to describe distributed spec approach
- Updated Codebase docs for cli.go, main.go, cli_test.go, main_test.go with requirements system documentation

## Log

- Explored codebase structure: policy system, statutes, ScanComments, test patterns
- Designed statute and detection approach
- Implemented Go code in main.go (helpers, policy, exemptions, nil guards)
- Wrote 11 test subtests - all pass
- Created 6 spec files with migrated requirements from AGENTS.md
- Cleaned AGENTS.md SRS section
- Updated Codebase docs sections
- Final verification: all tests pass (1.225s)

## Todos

- [x] Add BreachCodeRequirementsSyntax statute and metadata
- [x] Add isSpecText and hasImplementationSyntax helpers
- [x] Add spec line detection to PolicyContext
- [x] Modify ScanComments to exempt spec zone lines
- [x] Implement requirementsPolicy function
- [x] Wire requirementsPolicy into codePolicy
- [x] Create .bundle.md and .folder.md files with migrated requirements
- [x] Migrate file-level requirements to header Requirements regions
- [x] Migrate section and definition requirements to code
- [x] Clean up AGENTS.md and README.md
- [x] Extend tests for requirements breach
- [x] Run all tests and verify

## Plan

1. Add new statute `code:requirements:implementation-syntax` for detecting code syntax in spec text
2. Add helpers: `isSpecText` (detects RFC 2119 keywords) and `hasImplementationSyntax` (detects backticks, function calls, etc.)
3. Add `SpecLines` method to PolicyContext to identify spec comment lines
4. Modify BaseLanguage.ScanComments and TypeScriptLanguage.ScanComments to exempt spec zone lines from inline/JSDoc/block breachs
5. Implement `requirementsPolicy` that scans all spec locations and checks for implementation syntax
6. Wire into codePolicy
7. Create .bundle.md / .folder.md files for bundles/folders with requirements
8. Migrate requirements from AGENTS.md SRS into source code at proper levels
9. Remove migrated requirements from AGENTS.md
10. Extend main_test.go with comprehensive tests
11. Verify all tests pass
