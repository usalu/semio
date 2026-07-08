# Print Navbar Chapter Chip

## Fix

`\semio@chrome@heading` is now updated from heading `titleformat` hooks:

- `\part` and `\chapter` always set the navbar chip title
- `\section` sets it when no chapter is active (`\c@chapter = 0`), covering zwischenbericht-style documents that use sections as top-level headings

## Verification

- `report.pdf` p3 — navbar chip shows **Introduction** (chapter), not subsection names
- `paper.pdf` p2 — navbar chip shows **Conclusion** (current section)
- `kompaktbericht.pdf` p2 — navbar chip shows **Impressum** (last section on page)
