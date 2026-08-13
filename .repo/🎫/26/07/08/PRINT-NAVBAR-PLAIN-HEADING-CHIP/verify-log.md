# Print Navbar Chapter Chip

## Fix

`\semio@chrome@heading` is now updated from heading `titleformat` hooks:

- `\part` and `\chapter` always set the navbar chip title
- `\section` sets it when no chapter is active (`\c@chapter = 0`), covering zwischenbericht-style documents that use sections as top-level headings

## Verification

- `report.pdf` p3 — navbar chip shows **Introduction** (chapter), not subsection names
- `paper.pdf` p2 — navbar chip shows **Conclusion** (current section)
- `kompaktbericht.pdf` p2 — navbar chip shows **Impressum** (last section on page)

## 2026-08-13 — Highest-Tier Running Header

The running-header mark now locks to the highest heading tier encountered. In the Zwischenbericht, `\part` therefore owns the outer chip and lower section, subsection, subsubsection, and paragraph headings repeat the retained part mark instead of replacing it. Repeating the mark is required to supersede KOMA's separate numbered lower-level marks.

### Verification

- `bun nx run @semio-tech/print:test-quick` — passed.
- `bun nx run @semio-tech/mit-bestand-bericht:build --skip-nx-cache` — light and dark PDFs built successfully.
- Light and dark text-position audit, pages 10–27 — outer chip stays on **Ergebnisse** (10–20), **Projektstand** (21–25), **Mittelverwendung** (26), and **Ergebnisverwertung** (27); no lower-level label appears there.
- Rendered inspection — pages 12, 24, 26, and 27 show unclipped, parity-mirrored part chips. On page 26, **Externe Softwareentwicklung** remains a body heading while the header chip reads **Mittelverwendung**.
- `bun nx run @semio-tech/print:build --skip-nx-cache -- report paper kompaktbericht` — all six light/dark fallback PDFs built successfully. Part-less reports continue to track their highest available tier (chapter or section).
- `header-part-probe.tex` and its retained `[DEBUG]` log confirm state transitions: part sets root level `-1`; section/subsubsection leave the root label unchanged; the next part updates it.
