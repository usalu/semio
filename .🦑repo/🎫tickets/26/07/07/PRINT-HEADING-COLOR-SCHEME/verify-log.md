# Verify Log — Kind-Prefixed Hierarchy Numbering

## Build

- `bun ./script.ts build report paper` — success (light + dark)

## Visual checks (rasterized PNGs in ticket folder)

### report (language=de)

- Page 3 headings: `Kapitel: 1`, `Abschnitt: 1.1`, `Unterabschnitt: 1.1.1`, `Unterunterabschnitt: 1.1.1.0`, `Absatz: 1.1.1.0.0`
- Page 4: `Kapitel: 2`, window `Zitat: 2.a`, `Abbildung: 2.b` with custom title chip

### paper (language=en)

- Page 2 headings: `Section: 1` … `Section: 5`

## Format

- Left chip: `Kind: hierarchy-number` (localized kind label via babel `\iflanguage`)
- Window left chip: `Kind: parent-path.letter` (a, b, c … resets when hierarchy path changes)
- Right chip: title at line end (unchanged)

## Notes

- Hierarchy path tracked in LaTeX2e (`\semio@hierarchy@path`) when headings render; synced to expl3 for windows
- TOC/list hooks disable heading chips during `\tableofcontents`, `\listoffigures`, `\listoftables`
