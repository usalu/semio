# Plan: Standardize Semio Tests Across All Implementations

## Objective
Refactor all semio tests across TypeScript, Python, C#, Go, and Rust to be identical in functionality and naming. Use the TypeScript (semio.ts) tests as the blueprint.

## Target Test Suite Structure

All implementations must have exactly these tests:

1. **Roundtrip/Json/Metabolism** - Kit -> Json -> Kit
2. **Roundtrip/Zip/Metabolism** - Zip -> Kit -> Zip -> Kit
3. **Flatten/Nakagin Capsule Tower** - Kit -> Flatten -> Diff -> Apply = Flat
4. **Flatten/Nakagin Capsule Tower/Slanted** - Kit -> Flatten -> Diff -> Apply = Flat
5. **Flatten/Nakagin Capsule Tower/Twisted** - Kit -> Flatten -> Diff -> Apply = Flat
6. **Flatten/Nakagin Capsule Tower/Dancing** - Kit -> Flatten -> Diff -> Apply = Flat
7. **Flatten/Capsule Dream** - Kit -> Flatten -> Diff -> Apply = Flat
8. **Diff/Metabolism** - Kit + Diff = DiffedKit & DiffedKit + InvertedDiff = Kit
9. **Validation/Invalid** - Invalid Kit -> Validate = Invalid Report
10. **Validation/Metabolism** - Metabolism Kit -> Validate = Empty report

## Current State Analysis

### TypeScript (js/semio/semio.test.ts) - BLUEPRINT ✓
- Has all tests, well-structured

### Python (py/semio/semio.test.py) - NEEDS UPDATE
- Has tests but naming differs
- Needs standardization

### Go (go/semio/semio_test.go) - NEEDS UPDATE
- Has tests but structure differs slightly
- Needs standardization

### C# (net/Semio.Tests/Tests.cs) - NEEDS UPDATE
- Has tests but naming differs
- Missing Import/Export tests, Diff test
- Needs standardization

### Rust (rs/semio/semio.rs) - NEEDS CREATION
- No test file exists, need to create tests module

## Tasks

1. [x] Create plan.md
2. [ ] Read all current test files
3. [ ] Update TypeScript tests (if needed for naming)
4. [ ] Update Python tests to match TypeScript structure
5. [ ] Update Go tests to match TypeScript structure
6. [ ] Update C# tests to match TypeScript structure
7. [ ] Create Rust tests module
8. [ ] Run all tests and fix any issues
9. [ ] Document in AGENTS.md if needed
10. [ ] Close ticket with summary
