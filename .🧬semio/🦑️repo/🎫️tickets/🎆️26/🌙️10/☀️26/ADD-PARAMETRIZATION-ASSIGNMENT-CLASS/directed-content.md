# Directed Assignment Content

The German and English assignment contents now pass six localized transition descriptions directly to the worked example graph, covering each ordered pair of the three shapes. The separate worked example ledger call was removed. The K16 instructions describe 20 selected directed edges and follow the printed arrow directions. Existing continuous parameters, tables, numbered sections, and criteria were preserved.

Source validation passed for both languages: exactly six graph arguments, no obsolete separate ledger invocation, and no undirected pair/120-edge wording. Final native English validation passes after geometry integration. The existing undirected K16 PDF verifier must be superseded for this task because its degree-15/120-segment assumptions do not describe ordered arcs.

## Retained Content Contract

`directed-content-check.cjs` passes for German and English against `directed-geometry.fixture.json`. It derives category names from each worked table, then independently reconstructs all six labels from the ordered-arc fixture and fixed dimension values. It also verifies the two matrices, directed exercise wording, absent separate ledger, and absent visible frontmatter.

The first native English compile revealed the unavailable TikZ backgrounds library in the pinned bundle; geometry ownership replaced it with core PGF layer declarations. The rerun uses the unchanged authored contents.

## English Native Probe

The native English assignment compiled successfully with Tectonic 0.16.9: three pages, 133600 bytes, correct title/author/en-US metadata, zero overfull or underfull boxes, and no out-of-page extracted characters. All three rendered PNGs were visually inspected at scale 2. The complete directed worked example has six legible descriptions and arrowheads; both capsule matrices remain on page two. The independent matrix PDF validator passes both 9×9 grids, 6.14 mm row pitch, and centered vector icons.

The rendered page three revealed a narrow ledger gutter: the first-column rules reach the start of the second-column directed labels. This was reported to geometry ownership for repair before final native revalidation.

## Final English Native Result

The final English compilation includes the repaired gutter and restored product-font digits. Final size is 132951 bytes across three pages; metadata remains Finger Exercise: Parametrization / Architecture / en-US. The final log has zero overfull or underfull boxes, and independent extraction finds no out-of-page characters. All three final page PNGs were regenerated and inspected individually; the six worked-example descriptions, the two matrices, and the complete directed graph plus ledger are clear and unclipped.

The retained `directed-ledger-check.py` independently compares all twenty arrow labels to the language-agnostic selected ordered-edge fixture, requires blank writing regions, and enforces a minimum 3 mm gap between each first-column rule and the adjacent second-column label. Final measured minimum gutter is 4.08 mm; each blank rule is 64.60 mm. Native PDF character inspection confirms Anta-Regular digits with only the arrows in CMSY10. The previously found ledger collision is resolved.

The final PDF and render evidence remain under the ticket generated directory for parent integration and cleanup. Authored retained inputs added here are `directed-content-check.cjs` and `directed-ledger-check.py`; the report is `directed-content.md`.
