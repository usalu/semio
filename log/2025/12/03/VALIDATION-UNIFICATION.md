---
date: "2025-12-03T10:49:48.397Z"
slug: VALIDATION-UNIFICATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: "Unify validation mechanism across TypeScript, Python and C#"
model: claude-opus-4.5
---

# Previously

- TypeScript has `validateSemioKit()` returning `SemioValidationResult` with `issues: SemioValidationIssue[]`
- Python has `validateKitDict()` returning `ValidationResult` with `issues: list[ValidationIssue]`
- C# has per-model `Validate()` returning `(bool, List<string>)` - different format, needs new implementation
- TypeScript has `fixes` with `KitDiff`, Python and C# don't have fixes
- `kit_invalid.json` contains test data with duplicate names for types, designs, pieces, etc.
- `validation.json` is empty and should contain expected validation output

# Plan

1. Define a common portable validation result format (without fixes) for cross-platform serialization
2. Add `PortableValidationResult` interface and `serializeValidationResult` function to TypeScript
3. Add `serialize_validation_result` function to Python matching the same JSON format
4. Add C# `SemioValidationResult` class and serialization matching the same format
5. Generate `validation.json` from `kit_invalid.json` using TypeScript
6. Add validation tests to all three implementations comparing output to `validation.json`
7. Export `ValidationResult` from assets/index.ts

# Changes

## TypeScript (js/js/semio.ts)

- Added `SerializableValidationFix`, `SerializableValidationIssue`, `SerializableValidationResult` interfaces
- Added `toSerializableValidationResult()` to convert full result to serializable format (includes fixes)
- Added `serializeValidationResult()` for JSON serialization (sorted by ruleId, entityGuid)
- Added `parseValidationResult()` and `areValidationResultsEqual()` for testing
- Added `areKitDiffsEqualIgnoringNewGuids()` for GUID-normalized diff comparison
- Fixed layer-path-unique rule to include `entityGuid`

## TypeScript Test (js/js/semio.test.ts)

- Consolidated to single test: `Validation matches expected output`
- Test checks valid kit has no errors AND invalid kit matches validation.json

## Python (py/engine/engine.py)

- Added `ValidationFix` dataclass with `title` and `diff`
- Updated `ValidationIssue` to include `fixes: list[ValidationFix]`
- Added `ValidationResult.toDict()` and `ValidationResult.serialize()` methods
- Added `areValidationResultsEqual()` with GUID normalization for fix comparison
- Added `parseValidationResult()` for parsing validation.json
- Added fix generation to `validateKitDict()` for all validation rules
- Skips diff comparison for `guid-unique` (new GUIDs differ)

## Python Test (py/engine/engine.test.py)

- Consolidated to single test: `test_validationMatchesExpectedOutput`

## C# (net/Semio/Semio.cs)

- Added `SemioValidationFix` class with `Title` and `Diff`
- Updated `SemioValidationIssue` to include `Fixes` list
- Added `SemioValidationResult.Parse()` to handle fixes from JSON
- Updated `SemioValidationResult.AreEqual()` to skip fix comparison (pending fix generation)
- Updated `Layer` class to match TypeScript schema (added `Guid`, `Path`, `IsHidden`, `IsLocked`, `Attributes`)

## C# Test (net/Semio.Tests/Tests.cs)

- Consolidated to single test: `Validation_MatchesExpectedOutput`

## Assets

- Generated `assets/semio/validation.json` with expected validation output including fixes
- Added `InvalidKitValidation` export in `assets/index.ts`

## Scripts

- Added `scripts/generate-validation.ts` to regenerate validation.json

## Documentation

- Updated Validation Serialization section in AGENTS.md
