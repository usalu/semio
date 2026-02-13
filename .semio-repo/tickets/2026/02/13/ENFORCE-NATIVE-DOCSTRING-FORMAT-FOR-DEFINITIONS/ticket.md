---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Enforced native docstring format for all definitions. Added ViolationCodeDefNotNativeDocstring violation kind with detection for TS/JS (JSDoc), C#/Rust (///), Go/Python (native). Added autofix converting // to JSDoc for TS/JS and // to /// for C#/Rust. Fixed all 2503 violations across 24 files. Added 10 new test cases (8 detection, 1 autofix, 1 exemption). Zero violations remaining.
## Changes

- Added `ViolationCodeDefNotNativeDocstring` violation kind under Definition > Wrong Format group
- Added detection in codePolicy for non-native docstring format per language
- Added autofix to convert `//` line comments to native docstring format (JSDoc for TS/JS, `///` for C#/Rust)
- Updated DefinitionDocLines to recognize JSDoc blocks before definitions
- Exempted definition JSDoc from comment ban in TypeScript ScanComments
- Updated definition summary/specs/identification checks to parse JSDoc blocks
- Treated Python `#` as native format (like Go `//`) since Python has no non-native alternative
- Fixed all 2503 violations across 24 files (TS/JS → JSDoc, C# → `///`)
- Added 10 new test cases (8 detection + 1 autofix + 1 exemption) across 3 test functions

## Log

## Todos

- [x] Add violation kind constant, meta, and group structure
- [x] Add detection logic in codePolicy
- [x] Update DefinitionDocLines to handle JSDoc blocks
- [x] Exempt definition JSDoc in TypeScript ScanComments
- [x] Update definition summary/specs/identification parsing for JSDoc
- [x] Add autofix logic
- [x] Run tests
- [x] Fix all codebase violations
- [x] Final verification

## Plan

1. Add `ViolationCodeDefNotNativeDocstring` as new violation kind
2. Add "Wrong Format" group under Definition with both DefWrongFormat and DefNotNativeDocstring
3. In codePolicy definition checking, add native docstring detection per language
4. Update DefinitionDocLines to mark JSDoc block lines as definition doc
5. In TypeScript ScanComments, exempt JSDoc blocks that precede definitions
6. Update the definition summary/specs/identification detection to parse JSDoc content
7. Add autofix case for converting // lines to JSDoc format
8. Run all tests
9. Fix all files in the codebase
10. Final verification of zero violations
