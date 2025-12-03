---
date: "2025-12-03T10:49:48.397Z"
slug: VALIDATION-UNIFICATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: "Unify validation mechanism across TypeScript, Python and C#"
model: claude-sonnet-4.5
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

- Added `PortableValidationIssue` and `PortableValidationResult` interfaces
- Added `toPortableValidationResult()` to convert full result to portable format
- Added `serializeValidationResult()` for JSON serialization (sorted by ruleId, entityGuid)
- Added `parseValidationResult()` and `areValidationResultsEqual()` for testing
- Fixed layer-path-unique rule to include `entityGuid`

## TypeScript Test (js/js/semio.test.ts)

- Added test `Portable validation result matches expected output (cross-platform)`

## Python (py/engine/engine.py)

- Added `ValidationIssue.toPortableDict()` method
- Added `ValidationResult.toPortableDict()` and `ValidationResult.serialize()` methods
- Added `areValidationResultsEqual()` and `parseValidationResult()` functions
- Updated validation messages to match TypeScript format (include parent names)

## Python Test (py/engine/engine.test.py)

- Added fixture for `expectedValidationJson`
- Added test `test_portableValidationResultMatchesExpectedOutput`

## C# (net/Semio/Semio.cs)

- Added `SemioValidationIssue` class
- Added `SemioValidationResult` class with `Serialize()`, `Parse()`, `AreEqual()` methods
- Added `SemioValidator.ValidateKit()` static method
- Updated `Layer` class to match TypeScript schema (added `Guid`, `Path`, `IsHidden`, `IsLocked`, `Attributes`)

## C# Test (net/Semio.Tests/Tests.cs)

- Added `SemioValidation_InvalidKit_MatchesExpectedOutput` test
- Added `SemioValidation_InvalidKit_HasAllExpectedRules` test

## Assets

- Generated `assets/semio/validation.json` with expected validation output
- Added `InvalidKitValidation` export in `assets/index.ts`

## Scripts

- Added `scripts/generate-validation.ts` to regenerate validation.json

## Documentation

- Added Cross-Platform Portable Validation section to AGENTS.md
