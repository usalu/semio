---
name: LaTeX Table OS Style Parity
overview: Add a new semio-table LaTeX package that gives print/tex tables the same visual language as the React Table component (`ui/js/react/index.tsx`) used by the OS product — hairline horizontal row dividers only, shaded header row, matching row heights/spacing/fonts — then migrate the one existing tabular call site in `print/` to use it.
todos:
  - id: ticket
    content: Open a ticket for the LaTeX table styling work
    status: completed
  - id: semio-table-sty
    content: Create print/tex/semio-table.sty with SemioTable environment, SemioTableHeaderRow/SemioTableRow, using existing spacing/stroke/color tokens
    status: completed
  - id: register-package
    content: Register semio-table in print/tex/semio.cls
    status: completed
  - id: migrate-callsite
    content: Migrate the tabular block in print/template/zukunftbau/zwischenbericht.content.tex to SemioTable
    status: completed
  - id: build-verify
    content: Run bun print/script.ts test and visually check the workpackages table in the built PDFs (light + dark)
    status: completed
  - id: close-ticket
    content: Close the ticket with a summary and list of touched files
    status: completed
isProject: false
---

# LaTeX Table OS Style Parity

## Problem

`print/` (the `@semio-tech/print` LaTeX toolchain) has no styled data-table primitive. The only real usage today is a bare `tabular` with zero styling:

```14:19:print/template/zukunftbau/zwischenbericht.content.tex
\makeworkpackages{%
  \begin{tabular}{ll}
    AP1 & Partner A \\
    AP2 & Partner B \\
  \end{tabular}%
}
```

`Table` (defined in [print/tex/semio-window.sty](print/tex/semio-window.sty)) is only the OS-style window *chrome* (tcolorbox frame + title chip) that wraps arbitrary content — it does not style the `tabular` grid inside it. Meanwhile the OS UI's canonical `Table` component in [ui/js/react/index.tsx](ui/js/react/index.tsx) (lines ~18736-18847) has a well-defined look:

- No vertical rules, no outer frame, no striping — **horizontal hairline dividers only**, one under the header and one under every body row (`borderNormalBottomClass` = `border-b !border-normal`)
- Header row: `bg-window` shaded background, `font-medium`, height `h-large` (9 spacing units)
- Body rows: height `h-medium` (7 spacing units), `text-sm`, horizontal cell padding `p-single`/`px-single`

Static print output can't reproduce hover/selection states (those are interactive-only), so parity is scoped to the structural/visual styling: rules, shading, spacing, row height, font weight.

## Design: `print/tex/semio-table.sty`

New package (follows the repo's existing one-concern-per-file pattern: `semio-window.sty` = chrome, `semio-fonts.sty` = fonts, `semio-tokens.sty` = tokens, `semio-table.sty` = data-grid styling), built on `array` + `colortbl`, reusing tokens already emitted into `semio-tokens.sty` — no token-generation changes needed:

- `\semio@spacing@unit` (0.2em) → row heights as multiples, mirroring the `size-medium`/`size-large` token scale (7×/9× compact unit) used by the React `h-medium`/`h-large` classes
- `\semio@stroke@hairline` → rule width
- `semio-chrome-border-normal` → rule color
- `semio-chrome-window` → header shading (theme-aliased in [semio-core.sty](print/tex/semio-core.sty), so light/dark both resolve automatically)

```latex
\ProvidesPackage{semio-table}[... semio data table styling]
\RequirePackage{array}
\RequirePackage{colortbl}
\RequirePackage{semio-core}

% row/header heights as unit multiples matching ui size-medium (7x) / size-large (9x)
% \semio_table_rules_apply: sets \arrayrulecolor, \arrayrulewidth (hairline),
%   \tabcolsep (single spacing), \extrarowheight (body row height)

\NewDocumentEnvironment{SemioTable}{m}{ % #1 = column spec, no '|' (no vertical rules)
  \semio_table_rules_apply:
  \tabular{@{}#1@{}}
}{
  \endtabular
}

\NewDocumentCommand{\SemioTableHeaderRow}{m}{
  \rowcolor{semio-chrome-window}\rule{0pt}{<header-height>}\bfseries #1 \\ \hline
}
\NewDocumentCommand{\SemioTableRow}{m}{
  #1 \\ \hline
}
```

Usage becomes:

```latex
\begin{SemioTable}{ll}
  \SemioTableHeaderRow{AP & Partner}
  \SemioTableRow{AP1 & Partner A}
  \SemioTableRow{AP2 & Partner B}
\end{SemioTable}
```

Register the package in [print/tex/semio.cls](print/tex/semio.cls) (`\RequirePackage{semio-table}` alongside the other `semio-*` packages, after `semio-window`).

## Migrate the one call site

- [print/tex/semio-components.sty](print/tex/semio-components.sty): `makeworkpackages` currently just forwards its argument into `\begin{Table}...\end{Table}` unchanged — no change needed there (it stays content-agnostic), but its only caller must switch content.
- [print/template/zukunftbau/zwischenbericht.content.tex](print/template/zukunftbau/zwischenbericht.content.tex): rewrite the `tabular{ll}` block to `SemioTable` with a header row (`AP`/`Partner`) using `\SemioTableHeaderRow`/`\SemioTableRow`.

Per the no-tech-mixing rule, `mit-bestand/bericht/zwischenbericht/zwischenbericht.tex` (a separate, real-content report that also contains two ad-hoc `tabular` blocks) is **not** touched by this ticket — it's a different technology/consumer that can adopt `SemioTable` separately if asked.

## Verification

- `bun print/script.ts test` (from `print/`) — regenerates `semio-tokens.sty`, fetches fonts, compiles all 6 templates (light + dark) via Tectonic, asserts every PDF exists.
- Visually spot-check the built `zwischenbericht.pdf` / `zwischenbericht-dark.pdf` workpackages table for: header shading in the correct theme color, hairline dividers only (no vertical rules), row heights visually close to the OS table's proportions.

## Process

Per repo workflow: open a new ticket (`.repo/🎫/26/07/08/...`) before editing, since no existing open ticket covers LaTeX table styling; close it with a summary and the full list of touched files once verified.
