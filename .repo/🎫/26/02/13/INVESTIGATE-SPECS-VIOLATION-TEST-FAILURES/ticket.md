---
goal: unassigned
---

# Ticket

## Summary

Fixed two failing tests and a dead-code bug: (1) JSDoc test content changed to place JSDoc NOT above a definition so it is no longer exempted as definition docstring, (2) inline test content changed to place comment AFTER code so it is no longer the first comment block after section start (section doc exemption), (3) fixed brace nesting bug in TypeScript ScanComments where IsDefinitionDocLine was unreachable dead code inside the IsSectionDocLine block
## Changes

1. Fixed test content in both tests to avoid exemption triggers
2. Fixed brace/nesting bug in TypeScript ScanComments inline comment handling

## Log

### Root Causes

**JSDoc test**: `DefinitionDocLines()` traces upward from `const x = 1` (line 17, matched by definition regex) and marks the JSDoc lines (14-16) as definition doc lines. `IsDefinitionDocLine(file, 14)` returns true → JSDoc is exempted.

**Inline test**: `SectionDocLines()` → `markSectionDocLines()` iterates from `MySection` start+1, skips empty lines, finds `// This is a regular comment not a spec.` as the first comment, marks it as section doc line 14. `IsSectionDocLine(file, 14)` returns true → comment is exempted.

**Code bug**: In TypeScript `ScanComments` (line ~10038-10047), the `IsDefinitionDocLine` check is nested inside the `IsSectionDocLine` block but after a `break`, making it unreachable dead code.

### Fixes

**Test 1 (JSDoc)**: Remove the `const x = 1;` definition after the JSDoc so it's NOT a definition docstring. Replace with a non-definition statement.

**Test 2 (inline)**: Insert a non-comment line between the section start and the comment, so it's NOT the first comment block after the section. Add `const x = 1;` BEFORE the comment.

**Code bug**: Fix the brace structure so `IsSectionDocLine` and `IsDefinitionDocLine` are separate `if` blocks at the same level.

## Todos

- [x] Investigate root causes
- [x] Fix TypeScript ScanComments brace bug
- [x] Fix JSDoc test content
- [x] Fix inline test content
- [x] Run tests to confirm passing

## Plan

1. Fix the code bug in TypeScript ScanComments (misplaced braces)
2. Fix test 1 content: remove `const x = 1;` definition
3. Fix test 2 content: move code before the comment
4. Run both tests to confirm
