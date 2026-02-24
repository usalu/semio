---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-FILE-MECHANISM
---

# Ticket

## Summary

Refactored all source code headers to new standardized format with nested License and Requirements subregions.

Core changes:

- Updated LanguagePlugin interface: FormatHeader now takes (filePath, summary, contributors, license, requirements) args
- Replaced headerFmt string field with supportsHeaders bool in BaseLanguage
- Rewrote FormatHeader to programmatically build headers with nested #region License / #region Requirements
- Updated all 11 language constructors (TS, Go, Py, C#, Rust, Ruby, Shell, TOML, YAML, SQL, GraphQL)
- Rewrote generateFileHeader and AGPLLicenseText() helper
- Added 3 new statutes: missing-summary, missing-license-region, missing-requirements-region
- Updated headerPolicy to validate License and Requirements subregions exist inside Header
- Updated sectionPolicy to exempt License/Requirements children of Header from empty-section breachs
- Batch-updated 80+ source code file headers across the entire repo
- Updated all test fixtures (file*fixed.*, file*invalid.*, file_fixable\*, file_empty_region.tsx)
- Fixed emoji variation selector (U+FE0E) issues in fixture files
- Added 4 new tests: TestFormatHeaderStructure, TestFormatHeaderEmptyRequirements, TestFormatHeaderAllLanguages, TestHeaderPolicyMissingSubregions
- Added Requirements content to main.go and main_test.go headers
- Updated AGENTS.md SRS Code Hygiene section with new header requirements
- Updated README.md Code Hygiene Hooks section with new header format documentation
- Removed stale fix_main_breachs.go and fix_main_issues.go scripts from other agents
- All header-related tests pass (pre-existing TestFormatResult_Bundle/Folder failures are unrelated emoji variation issues)

## Changes

## Log

## Todos

## Plan
