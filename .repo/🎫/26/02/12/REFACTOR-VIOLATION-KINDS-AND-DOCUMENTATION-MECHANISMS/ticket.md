---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Refactored statute tree, documentation mechanisms, and file header format. Renamed constants, updated meta/groups/policies/autofix/fixtures. All 200+ tests pass.

## Changes

- **repo/cli/main.go**: Renamed statute constants (missing-header → missing-header-region, etc.), added 4 new statutes, updated StatuteMeta table, policy groups (File with nested Wrong Identification Id/Uri, Section with nested Wrong Format, Definition), FormatHeader to 6 params with [ID](URI) format and flat content (no License/Requirements subregions), headerPolicy with [ID](URI) regex validation, sectionPolicy isExempt simplified to `s.Name == "Header"`, requirementsPolicy with fallback to scan flat header spec lines when no Requirements child section, autofix for [ID](URI) format, added FileHeaderUri function.
- **repo/cli/main_test.go**: Updated FormatHeader tests (6 args, [ID](URI) checks), autofix tests (line-based [ID](URI) replacement), bulk removal of 15 old License/Requirements subregion patterns, Statute round-trip test expectations (missing-header → missing-header-region), renamed TestHeaderPolicyOldFormatId.
- **semio/assets/repo/some/folder/file_invalid.tsx**: Rewritten with no identification, AGPL+MIT (wrong license), no contributors, unnamed region, orphan definition.
- **semio/assets/repo/some/folder/file_fixable.tsx**: Rewritten with missing endregion name and name mismatch breachs (autofixable).
- **semio/assets/repo/some/folder/file_fixable_expected.tsx**: Rewritten with corrected endregion names.
- **semio/assets/repo/some/folder/file_fixed.{tsx,py,cs,go}**: Valid headers with [ID](URI), flat license, summary.
- **semio/assets/repo/some/folder/file_invalid.{py,cs,go}**: Old-format plain IDs.
- **semio/assets/repo/some/folder/file_empty_region.tsx**: Valid header with [ID](URI).

## Log

- Session 1: Renamed statute constants, updated meta table, policy groups, FormatHeader, headerPolicy, sectionPolicy, autofix, FileHeaderUri, fixture files.
- Session 2: Bulk removal of License/Requirements subregion patterns in tests, fixed FormatHeader test args, autofix tests.
- Session 3: Fixed Statute round-trip test expectations (5 tests), added requirementsPolicy flat header spec scanning fallback, rewrote file_invalid.tsx/file_fixable.tsx/file_fixable_expected.tsx fixtures. Verified all 200+ tests pass.

## Todos

- [x] Rename statute constants
- [x] Update StatuteMeta table
- [x] Update policy groups
- [x] Update FormatHeader to [ID](URI) flat format
- [x] Update headerPolicy for [ID](URI) detection
- [x] Update sectionPolicy isExempt
- [x] Update requirementsPolicy for flat header requirements
- [x] Update autofix for [ID](URI) format
- [x] Add FileHeaderUri function
- [x] Update all 11 fixture files
- [x] Fix FormatHeader tests (6 args)
- [x] Fix autofix tests
- [x] Remove License/Requirements subregion patterns from tests
- [x] Fix Statute round-trip tests
- [x] Fix TestRequirementsBreach
- [x] Fix TestFixNonAutofixableNotFixed
- [x] Fix TestFixApplyAutofixes
- [x] Verify all tests pass

## Plan

1. Rename statute constants and add new ones
2. Update meta table and policy groups
3. Refactor FormatHeader to [ID](URI) flat format
4. Update headerPolicy, sectionPolicy, requirementsPolicy
5. Update autofix mechanism
6. Update all fixture files
7. Fix all test failures
8. Verify full test suite passes
