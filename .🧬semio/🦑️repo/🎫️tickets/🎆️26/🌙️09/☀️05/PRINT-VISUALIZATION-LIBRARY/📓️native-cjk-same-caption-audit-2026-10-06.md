# Native Same-Caption Font Restoration Audit

Strict language-neutral fixture was authored first as `📥️authored-inputs/native-cjk-same-caption-fixture-2026-10-06.json`. It covers EN/de × 8/12 TeX pt, pure 根子葉, Root 根 / Leaf 葉, two explicit leading/trailing spaces, explicit SemioMono mixed text, and en/de Latin ligatures/German. A plain font-system probe uses existing compileVizProbeDocument; no product edits or new helper module.

Actual compiler session 70779 exited 0 and produced 20 native hbox dimensions. Both matching logs contain zero Missing character warnings. First session 51693 exited 1 solely on an observer expression with expl3 tokens outside required syntax context; the corrected probe uses primitive scaled-point conversion. No font failure is inferred from that observer defect.

| Case | 8 pt native mm | 12 pt native mm |
|---|---:|---:|
| pure 根子葉 | 8.435035 | 12.652553 |
| mixed Sans | 23.083770 | 34.625655 |
| two leading/trailing spaces | 26.378706 | 39.568059 |
| mixed Mono | 25.361351 | 38.042000 |

Both languages have equal case dimensions. Explicit spaces add 3.294936 mm at 8 pt and 4.942404 mm at 12 pt. The fixture renders authored spaces as explicit TeX space tokens, keeping exact leading/trailing payload spacing observable rather than allowing TeX input whitespace normalization.

Bundled pypdf via Bun+Nx session 60271 exited 0 and retained exact PDF font resources and visitor runs: both mixed Sans captions select embedded /IHAIZV+Anta-Regular, then /BZIVMZ+NotoSansCJKsc-Regular-Identity-H for 根, then Anta for / Leaf, then Noto for 葉. Explicit Mono captions select /IXFAWU+ShareTechMono-Regular before Han and again for / Leaf after Han. LatinFollowing END after each closed group selects Anta. This proves restoration both within the same caption and after an ending CJK glyph/group boundary for the observed fixtures. Exact ToUnicode mappings and source cmap nonzero IDs remain 根=21167, 子=15353, 葉=34583.

Pure-run source metric comparison uses newly generated existing 🌏️.json family/characters/advances/unitsPerEm, bound to tracked font SHA-256 2c76254f6fc379fddfce0a7e84fb5385bb135d3e399294f6eeb6680d0365b74b. Each tested Han glyph has advance 1000 with unitsPerEm 1000. Its three-glyph derived width differs from actual native dimensions by at most 0.000000285 mm, below emitted six-decimal rounding. Exact current metric JSON hash and comparisons are retained in `📥️authored-inputs/native-cjk-same-caption-owned-advance-comparison-2026-10-06.json`. This compares the pure owned data contract; it does not claim a fresh execution of measurePrintSans.

PDF advance observer first encountered indirect W arrays and a blank page without Font resources (actual inspection exits 1). These adapter defects were fixed without altering native artifacts. Session 60271 then succeeded, but width audit detected get_original_bytes reconstructing a BOM in some pypdf strings; its initial Latin totals must not be used. Final exact-original_bytes rerun session 41756 exited 0. Font resource/visitor run evidence above remains valid. Raw observer source is retained in `📥️authored-inputs/native-cjk-same-caption-pdf-observer-capsule-2026-10-06.json`.

Source owner hashes before the compiler dispatch are retained in `📥️authored-inputs/native-cjk-same-caption-source-capsule-2026-10-06.json`. Generated native/PDF outputs and logs are confined to ticket generated folders. This scoped evidence is separate from root's other phase receipts and visual QA.

## Final PDF Advance Receipt

Session 41756 actual exit 0 recomputed 42 text operators for each language using exact original string bytes, avoiding reconstructed BOMs. Embedded Noto CID widths are 1000 for all three observed glyphs. At PDF size 7.970112 pt each Han advance is 2.8116784 mm; at 11.955168 pt each is 4.2175176 mm (PDF font-size rounding is independent from the native six-decimal dimension observer). The raw CID arrays retain 21167/15353/34583 and the selected embedded font per operator. Exact emitted advances are in the PDF observation capsule.

PDF whole-caption spacing is distributed between glyph string advances, TJ adjustments and text positioning operators; summing only Tj/TJ widths does not reconstruct all mixed-caption spaces. Consequently this audit validates exact selected glyph resources and individual operator advances together with actual native whole-hbox dimensions, and does not claim that a partial PDF text-operator sum equals the full caption width. Leading/trailing spacing is proven by the native dimensions and explicit fixture tokens. A future physical pen-position observer should retain Tm/Td offsets before asserting full mixed PDF envelope equivalence.

