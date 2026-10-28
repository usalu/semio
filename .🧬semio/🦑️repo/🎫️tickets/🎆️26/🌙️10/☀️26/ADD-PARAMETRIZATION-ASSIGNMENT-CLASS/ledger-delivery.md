# Exercise-Style Changes Below the Example

The worked example now shows its six changes in the same two-column ruled edge list as the exercise. The K3 contains only the three solids, vertex numbers and directed arrows. No explanatory paragraphs or extra list heading were added.

One shared renderer draws both lists: identical arrow-labelled IDs, 64.6 mm writing rules, 6.3 mm row pitch and column positions. The example fills three rows per column; the exercise keeps ten blank rows per column. The left example column follows 1→2,2→3,3→1 and the right column contains the reverse directions. Compact numeric tuples follow the L,B,H table order, followed by the categorical change.

The filled values use a 9 pt font with their baselines raised above the rules. The original task tables, shared-dimension examples, outer K16 ring, inner 1→5→10→13→1 cycle, all240 directed arcs and twenty selected directions remain.

## Verification

- Version-thirteen schema, fixtures and differential semantic checks pass. All eighteen generic tests pass with75 assertions. The language-neutral feature now describes the external ledger and prose-free layout.
- Fresh DE, EN and dark native PDF checks verify six filled external entries, exactly20 blank task entries,64.6 mm rules,6.3 mm pitch, correct localized numeric/category changes and no delta captions in the graph.
- Minimum value glyph size is8.96638 PDF points, approximately9 TeX points. The longest German value is53.834 mm and English53.536 mm, within64.6 mm. Independent visual checks show readable values and clear descenders.
- Graph audits retain240 base arcs,20selected directions,6K3 arcs and266 native arrowheads with correct endpoint clearance and both closed K16 cycles. Table audits retain the eight-row example and both nine-by-nine task matrices.
- Native German light, English light and German dark builds pass. All three final German pages were rendered and individually inspected; English and dark versions were independently reviewed. The final German log has no overfull, underfull or missing-character errors.

## Final Artifact

- output/pdf/Fingeruebung-Parametrisierung.pdf
- Three A4 pages,127280 bytes
- SHA-256:65df0aee81ec33781d6af8918e2ffaf0bf706a125a4bea9aeb7643190c044dab

Authored inputs and audit reports remain in this ticket. Temporary build outputs and renders are removed after verification. Existing styling, original resources and concurrent repository changes are preserved.
