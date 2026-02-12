---
goal: AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/CONSISTENT-SECTIONS
---

# Ticket

## Summary

Fixed all code violations in extension.ts (45→0) and coda.py (70→0). Added section summaries with RFC2119 keywords, definition summaries, and spec comments. Key discovery: Python definition comments must be placed between decorators and def lines.
## Changes

- `semio-repo/vscode/extension.ts`: Added section summaries, definition summaries, and spec comments for all sections and exported definitions.
- `coda/py/coda.py`: Added 6 section regions with summaries, moved all definition summary/spec comments between decorators and def lines.

## Log

1. Ran `analyze` on both files to get exact violation lists.
2. Fixed extension.ts in 4 batches (3 initial + 1 for RFC2119 keyword requirement discovery).
3. Fixed coda.py orphan definitions by wrapping in sections.
4. Discovered Python comments before decorators are not detected as definition summaries.
5. Read `main.go` sectionPolicy to confirm: `ParseDefinitions` sets `def.Start` to the `def` keyword line, and backward scan breaks on non-comment `@decorator` lines.
6. Moved all 18 decorated function comments between decorator and def line. Result: 0 violations.

## Todos

- [x] Fix extension.ts section summaries
- [x] Fix extension.ts definition summaries and specs
- [x] Fix extension.ts inline comment violations (RFC2119 keywords)
- [x] Fix coda.py orphan definitions (wrap in sections)
- [x] Fix coda.py section summaries
- [x] Fix coda.py definition summaries and specs (move between decorator and def)
- [x] Verify 0 violations on both files

## Plan

1. Analyze violations in both files.
2. Add section summaries with RFC2119 keywords.
3. Add definition summaries and spec comments.
4. For Python decorated functions, place comments between decorator and def.
5. Verify 0 violations.
