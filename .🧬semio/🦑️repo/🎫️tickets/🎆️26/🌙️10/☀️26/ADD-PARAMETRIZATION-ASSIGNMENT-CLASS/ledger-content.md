# Worked Example Ledger Content

Both localized sources preserve the existing metadata, heading tree, table titles, and exercise commands. The worked graph has no arguments. Its six directed changes use the separate ledger, in the shared renderer argument order 1→2, 2→1, 2→3, 3→2, 1→3, 3→1. Changes follow the table parameter order L, B, H, then the categorical form transition. No explanatory prose was added.

## Verification

Full sources were authored in ticket-generated temporary files, atomically renamed, read back, and compared exactly. Obsolete shortstack, Delta, and Form-prefix labels are absent.

| Source | SHA-256 |
| --- | --- |
| de | 819b7978938a859e63f13d562e2c72ab877c7e8f3f325ac145980517a1ae11a4 |
| en | 0e862445c5e8d0e3e441ee5d64762741a472db18574b6b17c3ca70a05061d5d8 |

## English Native Compilation and Visual Review

Tectonic completed successfully after the shared renderer whitespace and value-baseline corrections. The final native log has no overfull, underfull, missing-character, or LaTeX warnings. PDFium rendered all three English pages. Visual review confirms the six filled changes occupy single lines within their writing rules, clear of the rules after the baseline lift. The graph contains shapes, vertex numbers, and arrows without delta text. Both exercise tables remain on page 2; the complete graph and 20 empty ledger lines remain on page 3. The source hashes above were checked again after compilation and remain unchanged.

## Final 9 pt Ledger Refinement

The final shared renderer uses a 9 pt value font with 10 pt leading, retaining the same writing rules, row pitch, and baseline lift. The final English native recompilation completed successfully. Its final log has no overfull, underfull, missing-character, or LaTeX warnings. Both content source hashes were checked again and match the frozen values above. This 9 pt compilation supersedes the earlier English compilation.

The final 9 pt PDF was rendered again with PDFium and all three pages were visually reviewed. All six English changes remain on single lines within the writing rules and their descenders clear the rules. No delta or form-change captions appear on the graph. The exercise tables, complete directed graph, and 20 blank exercise ledger lines remain intact.

The independent final PDF audit passed: every ledger glyph is at least 8.96638 PDF points (approximately 9 TeX points); the longest entry is 53.536 mm within its 64.6 mm rule. All six localized numerical and categorical changes match their directed identifiers. Native matrix and graph-endpoint checks also passed.
