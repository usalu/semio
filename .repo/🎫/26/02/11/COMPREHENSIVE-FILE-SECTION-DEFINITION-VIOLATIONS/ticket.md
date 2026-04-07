---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Completed comprehensive file/section/definition statutes with summary/requirements checks, SectionDocLines exemption system, extended test coverage (5 new tests), all tests passing

## Changes

### repo/cli/main.go

- Renamed `BreachCodeHeader*` constants to `BreachCodeFile*` for consistent naming
- Added 14 new statute constants: File (MissingHeader, WrongHeaderFormat, MissingId, MissingSummary, MissingRequirements, MissingDocs), Section (WrongFormat, MissingSummary, MissingRequirements, MissingDocs), Definition (WrongFormat, MissingSummary, MissingRequirements, MissingDocs)
- Updated `statuteInfoTable` with all 29 entries and autofixable flags
- Updated code policy Kinds list to include all 29 statutes
- Updated `headerPolicy`: distinguishes missing-id vs wrong-id, uses `BreachCodeFileWrongHeaderFormat` for missing License/Requirements subregions
- Added 4 autofix cases in `applyAutofixes`: MissingHeader, MissingId, MissingLicense, WrongLicense
- Extended `sectionPolicy` with section summary/requirements checks for non-exempt sections
- Added `realDefRanges` tracking to check definition summary/requirements (excluding `ExtraOrphanDefinitions`)
- Added `SectionDocLines`/`IsSectionDocLine` cache system to exempt section doc comments from inline comment breachs
- Refined `SectionDocLines`: only marks first contiguous comment block after section start if it contains at least one spec line
- Updated `SpecLines` to use `inLeadBlock` instead of `inSpecZone` for correct spec detection
- Added `IsSectionDocLine` check to both `BaseLanguage.ScanComments` and `TypeScriptLanguage.ScanComments`

### repo/cli/main_test.go

- Renamed breach constants in all test assertions
- Updated autofixable/non-autofixable lists in TestFixNonAutofixableNotFixed and TestFixStatuteMeta
- Updated display names in TestPolicyTreeCommand and TestFixtureBreachsGroupedInline
- Added 5 new test functions:
  - `TestSectionMissingSummaryAndRequirements`: Verifies sections without summary/requirements produce breachs
  - `TestSectionWithSummaryAndRequirements`: Verifies sections with summary/requirements produce no breachs
  - `TestDefinitionMissingSummaryAndRequirements`: Verifies definitions without summary/requirements produce breachs
  - `TestDefinitionWithSummaryAndRequirements`: Verifies definitions with summary/requirements produce no breachs
  - `TestSectionDocLinesExemptsDocComments`: Verifies doc comments are not flagged as inline comments

### semio/assets/repo/some/folder/file_fixed.tsx

- Added summary and spec comments to Types and Components sections
- Added summary and spec comments to FixedType, FixedKind, and FixedComponent definitions

### semio/assets/repo/some/folder/file_fixed.py

- Added summary and spec comments to Functions section and fixed_function definition

### semio/assets/repo/some/folder/file_fixed.go

- Added summary and spec comments to Package and Functions sections and FixedValue definition

### semio/assets/repo/some/folder/file_fixed.cs

- Added summary and spec comments to Classes section and FixedClass definition
- Changed FixedClass brace style to K&R (same line) for regex compatibility

## Log

- Fixed ExtraOrphanDefinitions (Go's package/import) being included in realDefRanges causing false positive breachs
- Fixed SectionDocLines exempting all comments after section start instead of only doc blocks with requirements
- Fixed C# fixture brace style for definition parsing compatibility
- Removed temporary debug output from tests

## Todos

- [x] Remove extraDefs from realDefRanges
- [x] Remove debug output from test
- [x] Run failing test to verify fix
- [x] Run full test suite (435s, all pass)
- [x] Extend test coverage (5 new tests)
- [x] Update ticket and close

## Plan

1. Fix ExtraOrphanDefinitions in realDefRanges (Go false positives)
2. Fix SectionDocLines to only exempt blocks with spec content
3. Fix C# fixture for definition parsing
4. Remove debug output from tests
5. Add new tests for section/definition summary/requirements
6. Run full test suite
7. Close ticket
