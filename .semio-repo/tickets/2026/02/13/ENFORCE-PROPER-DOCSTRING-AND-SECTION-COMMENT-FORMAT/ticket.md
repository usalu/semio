---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Implemented Python triple-quote docstring detection, autofix (create/merge), DefinitionDocLines marking, and DefMissingIdentification for Python. Fixed 635 violations across Python/TS/C# files to 0. Added 3 new tests, updated 1 test. All 49 policy tests pass.
## Changes

- `semio-repo/cli/main.go`: Added Python `"""` docstring detection inside body (paren-depth-aware body start), override `isNativeDocstring=false` when `#` comments above def with `"""`, Python autofix for `DefNotNativeDocstring` (creates/merges `"""` from `#` comments), Python-specific `DefMissingIdentification` autofix (inserts inside `"""`), TS/C#/Rust identification autofixes also updated for native docstring format, `DefinitionDocLines` marks Python `"""` lines as doc lines
- `semio-repo/cli/main_test.go`: Updated Python test from "should NOT flag" to "should flag" for `#` comments, added `TestPythonTripleQuoteDocstringAutofix` (converts `#` → `"""`), `TestPythonTripleQuoteDocstringMerge` (merges `#` into existing `"""`), `TestPythonTripleQuoteDocstringExemptFromCommentBan`, updated `TestViolationsNonEmpty` for 0-violation codebase
- `semio/py/semio.py`: 533 definitions migrated from `#` comments to `"""` docstrings
- `semio/engine/engine.py`: 57 definitions migrated
- `coda/py/coda.py`: 25 definitions migrated
- `semio/js/semio.ts`: Fixed Constants section (extracted ID from JSDoc to `//`)
- `semio/js/sketchpad/Design.tsx`: Fixed Windows section (extracted ID from JSDoc to `//`)
- `semio/net/Semio/Semio.cs`: Fixed Expressions section (converted `///` to `//`)

## Log

- Opened ticket, gathered context on current format across all languages
- Found 5 broken sections (mixing section IDs into JSDoc/`///`), manually fixed 3
- Updated TS JSDoc detection to handle `/** Summary.` opening line format
- Updated C# detection to strip `<summary>/<remarks>` XML doc tags
- Built CLI → 0 violations after section fixes + detection updates
- Implemented Python `"""` detection: scans body after def line, paren-depth-aware for multi-line signatures
- Changed Python from `isNativeDocstring=true` default to actual `"""` detection
- Added detection: Python defs with both `#` above and `"""` inside fire `DefNotNativeDocstring`
- Added Python autofix: collects `#` comments, finds/creates `"""` in body, merges content
- Updated `DefinitionDocLines` to mark Python `"""` lines as doc lines
- Updated `DefMissingIdentification` autofix for Python (adds inside `"""`) and TS/C#/Rust
- Built and ran fix → 635 → 85 → 0 violations (3 rounds)
- Updated Python test case, added 3 new tests
- Updated `TestViolationsNonEmpty` for clean codebase
- All 49 policy-related tests pass

## Todos

- [x] Fix section format: regular comments only, no docstrings
- [x] Fix definition format per language: JSDoc(TS), """(Py), XML doc(C#), ///(Rust), //(Go)
- [x] Section order: identification, summary, specs, TODOs, docs
- [x] Definition order: summary, specs, TODOs, identification (last)
- [x] Implement violations and autofix
- [x] Write tests for all languages
- [x] Fix all codebase files
- [x] Verify zero violations
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
