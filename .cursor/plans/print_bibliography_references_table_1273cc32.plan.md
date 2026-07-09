---
name: Print Bibliography References Table
overview: Generalize biblatex citation support from the kompaktbericht-only hack into the shared `semio.cls`, make citation style configurable per document, and render the bibliography as a chrome-styled "references table" in the appendix/registers section, matching the existing List-of-Figures/Tables visual language.
todos:
  - id: cls-citestyle
    content: Add citestyle option + generic biblatex loading to semio.cls; remove kompaktbericht-only hack from zukunftbau.cls
    status: in_progress
  - id: core-labels
    content: Add References title function + reference/references kind labels to semio-core.sty
    status: pending
  - id: table-colspec
    content: Add reference table colspec/macro to semio-table.sty
    status: pending
  - id: window-references
    content: Implement defbibenvironment + SemioReferences chrome renderer in semio-window.sty
    status: pending
  - id: components-makeregisters
    content: Wire SemioReferences into makeregisters in semio-components.sty
    status: pending
  - id: templates-kompakt
    content: Move kompaktbericht bibliography from inline section to appendix registers
    status: pending
  - id: templates-zukunftbau-rest
    content: Add citations + registers call to forschungsbericht/zwischenbericht, extend references.bib
    status: pending
  - id: templates-report-paper
    content: Add references.bib + citations + registers to report and paper templates (paper gets alphabetic citestyle)
    status: pending
  - id: verify-build
    content: Run bun run print:test and inspect generated PDFs for citation rendering and references table styling
    status: pending
  - id: ticket-open-close
    content: Open repo ticket under goal r2602, do the work, close ticket with summary of touched files
    status: in_progress
isProject: false
---

# Print: Proper Bibliography, Citation Styles, References Table

## Current state
- Biblatex is only wired up as a special case in [print/tex/zukunftbau.cls](print/tex/zukunftbau.cls), hardcoded to `type=kompaktbericht` with `style=authoryear,backend=bibtex`, loading `references.bib` from the `zukunftbau` template folder.
- [print/template/zukunftbau/kompaktbericht.content.tex](print/template/zukunftbau/kompaktbericht.content.tex) calls `\cite{example2026}` inline and immediately follows with `\section*{Literaturverzeichnis}\printbibliography[heading=none]` mid-body — plain biblatex list styling, not the semio window/table chrome used everywhere else.
- report/forschungsbericht already have a generic "register" system: 14 window kinds (figure, table, listing, theorem, ...) each get an automatic `\listofX` populated via `\semio_window_register_write:nn` in [print/tex/semio-window.sty](print/tex/semio-window.sty), rendered through `\SemioTableRegister` (Nr/Title/Page columns, chrome header) from [print/tex/semio-table.sty](print/tex/semio-table.sty), and bundled into `\makeregisters` in [print/tex/semio-components.sty](print/tex/semio-components.sty).
- paper/flyer/zwischenbericht have no bibliography wiring at all.
- Tectonic already runs bibtex/biblatex correctly today (`print/dist/kompaktbericht.bbl` exists and is populated) — no build pipeline changes needed in [print/script.ts](print/script.ts).

## Decisions (confirmed)
- Roll bibliography support out to all 6 templates (report, paper, flyer, forschungsbericht, zwischenbericht, kompaktbericht) by moving biblatex loading into `semio.cls` itself.
- Citation style becomes a configurable class option (`\documentclass[citestyle=...]{semio}`), with a sensible default per document type so the templates collectively demonstrate different styles: `numeric` for report/forschungsbericht/zwischenbericht, `alphabetic` for paper, `authoryear` for kompaktbericht (keeps its current behavior).
- The reference list moves out of the document body into the appendix/registers area and is rendered with the same `SemioTableRegister` chrome as the other List-of-X tables, wired into `\makeregisters`.
- Flyer keeps the shared biblatex capability available (so it *can* cite) but, being a one-page landscape piece with no appendix, does not get a demo references table — this is a scoping call, flag if you want it forced in anyway.

## Changes

### 1. Generalize biblatex loading — [print/tex/semio.cls](print/tex/semio.cls)
- Add `\DeclareStringOption[numeric]{citestyle}` alongside the existing `type`/`theme`/`language` kvoptions.
- Unconditionally `\RequirePackage[style=\semio@citestyle,backend=bibtex,sorting=nyt]{biblatex}` (after `\ProcessKeyvalOptions*`), so every document type has citation macros available.
- Remove the kompaktbericht-specific `\RequirePackage[style=authoryear,backend=bibtex]{biblatex}` + `\addbibresource{references.bib}` block from [print/tex/zukunftbau.cls](print/tex/zukunftbau.cls); instead pass `citestyle=authoryear` when it loads `semio` for `type=kompaktbericht`, and `citestyle=numeric` (or leave default) for forschungsbericht/zwischenbericht. Each content `.tex` file keeps its own `\addbibresource{references.bib}` call in the preamble (same pattern as `\title{}`/`\author{}`), pointing at a bib file living alongside it.

### 2. Localize the "References" register title — [print/tex/semio-core.sty](print/tex/semio-core.sty)
- Add a fixed bilingual title function analogous to `\semio_toc_title_text:` (since "Literaturverzeichnis"/"References" doesn't fit the generic "Verzeichnis der X" plural pattern used by figures/tables), exposed as `\SemioReferencesTitle`.
- Add `reference`/`references` kind labels (de: Referenz/Referenzen, en: Reference/References) to the `%region Kinds` block for use in the table header ("Nr." / "Referenz" / "Jahr").

### 3. References table styling — [print/tex/semio-table.sty](print/tex/semio-table.sty)
- Add a `\semio@table@colspec@reference` (narrow Nr column + wide citation column) and `\SemioTableReference` macro, following the existing `SemioTableTwo`/`SemioTableThree`/`SemioTableRegister` pattern.

### 4. References table rendering — [print/tex/semio-window.sty](print/tex/semio-window.sty)
- New `%region References` block:
  - A local counter (independent of the citation style's own numbering) that increments per bibliography entry, so the table's Nr column stays simple (1, 2, 3, ...) regardless of whether the in-text citation style is numeric/authoryear/alphabetic.
  - A `\defbibenvironment{semioreferences}{...}{...}{...}` (biblatex hook) that opens/closes the `Table` chrome + `SemioTableReference` header row, and locally redefines biblatex's per-entry finishing macro so each entry lands in its own row (`Nr & formatted citation \\`) instead of biblatex's default list spacing.
  - `\NewDocumentCommand{\SemioReferences}{}{...}` — public entry point: opens the register window titled via `\SemioReferencesTitle`, calls `\printbibliography[env=semioreferences,heading=none]`, closes the window. Exact biblatex driver macro names will be nailed down and verified via `bun ./script.ts build kompaktbericht` iterative Tectonic builds, since biblatex internals require build-verify loops.

### 5. Wire into registers — [print/tex/semio-components.sty](print/tex/semio-components.sty)
- Append `\SemioReferences` at the end of `\makeregisters`, after the existing 14 `\listofX` calls.

### 6. Template updates
- [print/template/zukunftbau/kompaktbericht.content.tex](print/template/zukunftbau/kompaktbericht.content.tex): remove the inline `\section*{Literaturverzeichnis}\printbibliography[heading=none]`, keep the `\cite{example2026}` in "Methodik und Projektverlauf", add a `\makeregisters` (or standalone `\SemioReferences`) call in the appendix next to "Projektbeteiligte"/"Impressum".
- [print/template/zukunftbau/forschungsbericht.content.tex](print/template/zukunftbau/forschungsbericht.content.tex) and [print/template/zukunftbau/zwischenbericht.content.tex](print/template/zukunftbau/zwischenbericht.content.tex): add `\addbibresource{references.bib}` + a couple of `\cite{...}` calls in body sections; forschungsbericht already calls `\makeregisters`, add the same call to zwischenbericht's appendix.
- [print/template/zukunftbau/references.bib](print/template/zukunftbau/references.bib): extend with 2-3 more varied entries (article, inproceedings) shared by all three zukunftbau document types, demonstrating the table with realistic multi-row content.
- [print/template/report/report.content.tex](print/template/report/report.content.tex): add `\addbibresource{references.bib}` + citations in Background/Results; new `print/template/report/references.bib` with a few entries. `\makeregisters` is already called.
- [print/template/paper/paper.content.tex](print/template/paper/paper.content.tex): add `\addbibresource{references.bib}` + citations in Methods/Results; add an `\appendix` + `\makeregisters` call at the end (currently paper.tex has no registers at all); new `print/template/paper/references.bib`. This template gets `citestyle=alphabetic` via its `\documentclass[...]` options.
- [print/template/flyer/flyer.tex](print/template/flyer/flyer.tex) / [print/tex/zukunftbau.cls](print/tex/zukunftbau.cls) flyer path: no content changes (keeps default `citestyle=numeric` from `semio.cls`, capability available but unused).

### 7. Verify
- Run `bun run print:test` (drives [print/script.ts](print/script.ts) `TestScript`, which builds all 6 templates light+dark via Tectonic) and visually spot-check the generated PDFs in `print/dist/` for: in-text citations rendering per the configured style, and the new references table appearing at the end with chrome/table styling consistent with other List-of-X registers.

## Ticket
Work will be tracked under a new repo ticket associated with goal `🎯r2602` (Running Sketchpad — same goal used by the prior `PRINT-LA-TE-X-TECHNOLOGY` and `AUTOMATIC-PRINT-REGISTERS` tickets), closed with a summary of every file touched once builds are verified.
