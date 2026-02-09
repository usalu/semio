---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Migrated specs from AGENTS.md into distributed .bundle.md/.folder.md files with new code:specs:implementation-syntax violation kind. Created specsPolicy, isSpecText, hasImplementationSyntax helpers, ScanComments spec exemptions. 11 new test subtests all passing.
## Changes

### semio-repo/cli/main.go
- Added `ViolationCodeSpecsSyntax` constant and metadata entry
- Added `specKeywordPattern` regex for RFC 2119 keyword detection
- Added `isSpecText()`, `hasImplementationSyntax()` helpers
- Added `specLineCache` to PolicyContext with `SpecLines()`, `IsSpecLine()`, `IsSpecBlock()` methods
- Added nil guards to `ReadText()` and `Sections()` for empty PolicyContext in tests
- Implemented `specsPolicy()` scanning Header Specs regions and section-start spec comments
- Wired `specsPolicy` into `codePolicy` Kinds list and Run function
- Modified `BaseLanguage.ScanComments` at 3 code points to exempt spec lines/blocks
- Modified `TypeScriptLanguage.ScanComments` at 2 code points to exempt spec lines/blocks
- Updated main.go header Specs region to be implementation-agnostic

### semio-repo/cli/main_test.go
- Added `TestSpecsViolation` with 11 subtests covering isSpecText, hasImplementationSyntax, specsPolicy violations, spec exemptions, and metadata

### New spec files
- semio/.bundle.md - Domain model specs
- semio-repo/cli/.bundle.md - Repo CLI specs
- semio-repo/vscode/.bundle.md - VS Code Extension specs
- semio/engine/.bundle.md - Engine specs
- .devcontainer/.folder.md - Devcontainer specs
- semio/js/sketchpad/.folder.md - Sketchpad UI specs

### AGENTS.md
- Cleaned up SRS section: removed ~580 lines of inline specs, replaced with spec location index
- Updated documentation policy to describe distributed spec approach
- Updated Codebase docs for cli.go, main.go, cli_test.go, main_test.go with specs system documentation

## Log

- Explored codebase structure: policy system, violation kinds, ScanComments, test patterns
- Designed violation kind and detection approach
- Implemented Go code in main.go (helpers, policy, exemptions, nil guards)
- Wrote 11 test subtests - all pass
- Created 6 spec files with migrated specs from AGENTS.md
- Cleaned AGENTS.md SRS section
- Updated Codebase docs sections
- Final verification: all tests pass (1.225s)

## Todos
- [x] Add ViolationCodeSpecsSyntax violation kind and metadata
- [x] Add isSpecText and hasImplementationSyntax helpers
- [x] Add spec line detection to PolicyContext
- [x] Modify ScanComments to exempt spec zone lines  
- [x] Implement specsPolicy function
- [x] Wire specsPolicy into codePolicy
- [x] Create .bundle.md and .folder.md files with migrated specs
- [x] Migrate file-level specs to header Specs regions
- [x] Migrate section and definition specs to code
- [x] Clean up AGENTS.md and README.md
- [x] Extend tests for specs violation
- [x] Run all tests and verify

## Plan
1. Add new violation kind `code:specs:implementation-syntax` for detecting code syntax in spec text
2. Add helpers: `isSpecText` (detects RFC 2119 keywords) and `hasImplementationSyntax` (detects backticks, function calls, etc.)
3. Add `SpecLines` method to PolicyContext to identify spec comment lines
4. Modify BaseLanguage.ScanComments and TypeScriptLanguage.ScanComments to exempt spec zone lines from inline/JSDoc/block violations
5. Implement `specsPolicy` that scans all spec locations and checks for implementation syntax
6. Wire into codePolicy
7. Create .bundle.md / .folder.md files for bundles/folders with specs
8. Migrate specs from AGENTS.md SRS into source code at proper levels
9. Remove migrated specs from AGENTS.md
10. Extend main_test.go with comprehensive tests
11. Verify all tests pass
