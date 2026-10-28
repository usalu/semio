# Ledger Audit

Final font-9 candidates passed independent PDF verification in German light, English light and German dark. All three are three pages with explicit PDF language metadata. Both light K16 graphs retain 125.603 mm visible height; dark retains 126.243 mm under its inherited text metrics.

| Check | Result |
|---|---|
| Native directed arcs | 240 complete K16, 20 selected, six K3 |
| Native arrowheads | 266 |
| Worked ledger | Six filled entries, two columns, three rows |
| Task ledger | Twenty blank entries, two columns, ten rows |
| Shared rule width / row pitch | 64.6 mm / 6.3 mm |
| All value glyphs, including mathematics | Minimum 8.96638 PDF bp, equivalent to 9 TeX pt |
| Maximum German / English value width | 53.834 mm / 53.536 mm |
| Typography extraction tolerance | 0.2 PDF bp; nominal PDF font bounding boxes reach 0.026 mm below the rule, while native glyph appearance is clear in full-size review |
| Localized categorical changes | All six verified in displayed clockwise/reverse order |
| K3 labels | Numeric node identifiers; no Form or delta captions |
| Worked matrix | Four filled and four blank rows; 16 mm header; half-scale unnamed native solids |

The v13 language-neutral fixture preserves positive-real dimensions, categorical endpoint changes, MathJS numeric transition/cycle comparisons, D3 pairing/selection comparisons, binary-flag counterexamples and negative-schema cases. It explicitly records the six-entry shared ledger layout and 9 pt target.

Validation commands executed through the repo tool interface: production semantic/source verifier passed after font-9 fixture update; generic assignment suite passed 18 tests / 75 expectations; verify-mixed-pdf.py passed all three final candidates; verify-directed-pdf.py passed all three final candidates. Source contracts derive every localized ledger value directly from the language-neutral numeric and category transitions and reject graph-caption or explanatory-prose regressions.

The language-neutral 🥒️.feature now documents v13 rendering: no explanatory paragraphs, six filled changes below the uncaptioned K3, the shared 3-row/10-row ledger layout, 9 pt values, and unchanged mathematical model scenarios. Obsolete above-edge captions, printed criteria/cycle checks and no-ledger requirements were removed. Final authored test files: 🟦️.ts, 🧫️cases.json, 🧬️cases.schema.json and 🥒️.feature.
