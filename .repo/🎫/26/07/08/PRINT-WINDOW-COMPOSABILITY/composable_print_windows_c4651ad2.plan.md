---
name: Composable Print Windows
overview: Add a generic, non-registered "Window" chrome primitive plus fixed-gutter horizontal composition macros to the print LaTeX stack, then rebuild the zwischenbericht cover page as 6 vertically-stacked window rows (Titel, Aktenzeichen, Förderzeitraum, Beschreibung, Antragstellende Institution, Kooperationspartner) followed by a 7th row of 3 side-by-side logo windows (Zukunft Bau, BBSR, BBR) — all spaced with the same single spacing unit used everywhere else, mirroring the UI's Window/HorizontalWindows/VerticalWindows composability.
todos:
  - id: ticket
    content: Check repo tickets/goals freshness and open new ticket under goal r2602/updateddocs
    status: completed
  - id: window-primitive
    content: Add generic non-registered Window environment to semio-window.sty
    status: completed
  - id: window-row
    content: Add SemioWindowRowTwo/SemioWindowRowThree fixed-gutter horizontal composition macros to semio-window.sty
    status: completed
  - id: core-fields
    content: Add antragstellendeinstitution/kooperationspartner tl vars, setup keys, and macros to semio-core.sty
    status: completed
  - id: coverpages
    content: Rebuild makecoverpages in semio-components.sty as 6 Window rows + 1 SemioWindowRowThree logo row
    status: completed
  - id: callsites
    content: Update zwischenbericht(.tex/-dark.tex) and forschungsbericht/zwischenbericht content templates to use new fields
    status: completed
  - id: verify
    content: Build all templates, rasterize zwischenbericht page 1, verify chrome + fixed single-unit spacing, close ticket
    status: completed
isProject: false
---

# Composable Print Windows (vertical + horizontal)

## Context
- `print/tex/semio-window.sty` currently only offers **registered/numbered** window "kinds" (`Image`, `Table`, `Blockquote`, ...) via `\semio_window_kind_define:nnnnn` — each gets a counter, register/list-of entry, and numbered chip. There is no generic, unregistered chrome box and **no horizontal composition primitive** (the only side-by-side example is ad hoc `minipage`+`\hfill` in `flyer.content.tex` and the cover-page logo row).
- The cover page (`\makecoverpages` in [print/tex/semio-components.sty](print/tex/semio-components.sty)) is currently **plain centered text** (`\vspace`/`\vfill`/`\par`), not Window chrome at all, and the logo row uses elastic `\hfill` gaps instead of the fixed single-spacing-unit gutter used everywhere else (`\semio@spacing@single` = UI's `--spacing-single` / `gap-single`, per `print/tex/semio-tokens.sty` ↔ `ui/styling/tokens.json`).
- The UI counterpart (`ui/js/react/index.tsx`) composes windows via plain `HorizontalWindows`/`VerticalWindows` flex wrappers with a fixed `gap-single`. This plan mirrors that: a generic `Window` box + fixed-gutter row composition.
- Confirmed with user: every cover-page row gets real window chrome; Antragstellende Institution / Kooperationspartner become two new dedicated metadata fields (currently merged into `\author`); the logo row uses a fixed single-unit gutter (not elastic `\hfill`).
- Note: `print/tex/semio-window.sty`, `semio-core.sty`, `semio-table.sty`, `semio-components.sty` and the zwischenbericht sources are being actively edited by other concurrent sessions (open tickets `PRINT-UNIFORM-BLOCK-SPACING`, `TOC-SEMIO-WINDOW-TABLES`, `MIT-BESTAND-ZWISCHENBERICHT-LA-TE-X-WORKSHOP`). Re-read each file immediately before editing to work off the latest state.

## 1. Ticket
Check `repo://tickets` again for freshness, then open a new ticket (no existing one covers window composability) under goal `🎯r2602🎯updateddocs` (same bucket as the sibling doc tickets), e.g. slug `PRINT-WINDOW-COMPOSABILITY`.

## 2. Generic `Window` primitive — [print/tex/semio-window.sty](print/tex/semio-window.sty)
Add a new region (after the kind-definitions block, before `%region Panels`) with a **non-registered** chrome environment reusing the existing `semio~window~tier~structural` tcolorbox style and the existing muted header renderer, and the existing `title` key from `semio / window / kind`:

```latex
%region GenericWindow
\NewDocumentEnvironment { Window } { O{} } {
  \tl_clear:N \l_semio_window_kind_title_tl
  \keys_set:nn { semio / window / kind } { #1 }
  \semio_block_sep:
  \semio@window@header@muted { } { \tl_use:N \l_semio_window_kind_title_tl }
  \noindent
  \begin{tcolorbox}[semio~window~tier~structural]
} {
  \end{tcolorbox}
  \semio_block_sep:
}
%endregion GenericWindow
```
`\semio_block_sep:` (already `\addvspace{\semio@spacing@single}`) gives free, exact-single-unit **vertical** composition — identical rhythm to every other window kind, satisfying "same spacing" for stacking.

## 3. Fixed-gutter horizontal composition — same file
Add a `WindowRow` region with equal-width columns separated by a **fixed** `\semio@spacing@single` gutter (not `\hfill`), mirroring `\SemioTableTwo`/`\SemioTableThree` naming/precedent in `semio-table.sty`:

```latex
%region WindowRow
\newlength{\semio@window@row@gutter}
\setlength{\semio@window@row@gutter}{\semio@spacing@single}
\newlength{\semio@window@row@col@w}

\newcommand{\semio@window@row@col}[2]{%
  \begin{minipage}[t]{#1}#2\end{minipage}%
}
\newcommand{\SemioWindowRowTwo}[2]{%
  \par\noindent
  \setlength{\semio@window@row@col@w}{\dimexpr(\linewidth-\semio@window@row@gutter)/2\relax}%
  \semio@window@row@col{\semio@window@row@col@w}{#1}\hspace{\semio@window@row@gutter}%
  \semio@window@row@col{\semio@window@row@col@w}{#2}\par
}
\newcommand{\SemioWindowRowThree}[3]{%
  \par\noindent
  \setlength{\semio@window@row@col@w}{\dimexpr(\linewidth-2\semio@window@row@gutter)/3\relax}%
  \semio@window@row@col{\semio@window@row@col@w}{#1}\hspace{\semio@window@row@gutter}%
  \semio@window@row@col{\semio@window@row@col@w}{#2}\hspace{\semio@window@row@gutter}%
  \semio@window@row@col{\semio@window@row@col@w}{#3}\par
}
%endregion WindowRow
```
Each column can itself contain a `Window` (or any kind environment), so rows and stacks nest — matching UI's nested `HorizontalWindows`/`VerticalWindows`.

## 4. New metadata fields — [print/tex/semio-core.sty](print/tex/semio-core.sty)
Next to `\l_semio_aktenzeichen_tl`/`\l_semio_foerderzeitraum_tl` (tl decl, setup key, direct macro):
```latex
\tl_new:N \l_semio_antragstellendeinstitution_tl
\tl_new:N \l_semio_kooperationspartner_tl
...
antragstellendeinstitution .tl_set:N = \l_semio_antragstellendeinstitution_tl,
kooperationspartner .tl_set:N = \l_semio_kooperationspartner_tl,
...
\NewDocumentCommand{\antragstellendeinstitution}{m}{\tl_set:Nn \l_semio_antragstellendeinstitution_tl {#1}}
\NewDocumentCommand{\kooperationspartner}{m}{\tl_set:Nn \l_semio_kooperationspartner_tl {#1}}
```

## 5. Rebuild `\makecoverpages` — [print/tex/semio-components.sty](print/tex/semio-components.sty)
Replace the plain-text stack with 6 `Window` rows + 1 `SemioWindowRowThree` logo row, each field still optional via the existing `\tl_if_empty:NTF` guards (title/subtitle share one row; DOI keeps its own optional row for other templates):
```latex
\NewDocumentCommand{\makecoverpages}{}{%
  \begin{titlepage}
    \semio_theme_apply:
    \centering
    \vfill
    \begin{Window}[title=Titel]
      {\SemioSans\Huge\@title\par}
      \tl_if_empty:NTF \l_semio_subtitle_tl {} {\vspace{\semio@spacing@single}{\large\tl_use:N \l_semio_subtitle_tl\par}}
    \end{Window}
    \tl_if_empty:NTF \l_semio_aktenzeichen_tl {} {\begin{Window}[title=Aktenzeichen]{\SemioSans\tl_use:N \l_semio_aktenzeichen_tl\par}\end{Window}}
    \tl_if_empty:NTF \l_semio_foerderzeitraum_tl {} {\begin{Window}[title=Förderzeitraum]{\SemioSans\tl_use:N \l_semio_foerderzeitraum_tl\par}\end{Window}}
    \tl_if_empty:NTF \l_semio_doi_tl {} {\begin{Window}[title=DOI]{\SemioMono\tl_use:N \l_semio_doi_tl\par}\end{Window}}
    \tl_if_empty:NTF \l_semio_kurzfassung_tl {} {\begin{Window}[title=Beschreibung]{\SemioSans\small\tl_use:N \l_semio_kurzfassung_tl\par}\end{Window}}
    \tl_if_empty:NTF \l_semio_antragstellendeinstitution_tl {} {\begin{Window}[title=Antragstellende~Institution]{\SemioSans\tl_use:N \l_semio_antragstellendeinstitution_tl\par}\end{Window}}
    \tl_if_empty:NTF \l_semio_kooperationspartner_tl {} {\begin{Window}[title=Kooperationspartner]{\SemioSans\tl_use:N \l_semio_kooperationspartner_tl\par}\end{Window}}
    \SemioWindowRowThree{%
      \begin{Window}[title=Zukunft~Bau]\semio@logo@slot[3.5cm]{zukunftbau-logo.pdf}\end{Window}%
    }{%
      \begin{Window}[title=BBSR]\semio@logo@slot[3.5cm]{bbsr-logo.pdf}\end{Window}%
    }{%
      \begin{Window}[title=BBR]\semio@logo@slot[3.5cm]{bundesbauministerium-logo.pdf}\end{Window}%
    }
    \vfill
  \end{titlepage}
}
```
Single `\vfill` top/bottom centers the whole 7-row stack; internal spacing is uniformly `\semio@spacing@single` via each `Window`'s `\semio_block_sep:`.

## 6. Update call sites
- [mit-bestand/bericht/zwischenbericht/zwischenbericht.tex](mit-bestand/bericht/zwischenbericht/zwischenbericht.tex) and `zwischenbericht-dark.tex`: split the merged `\author{...}` block into `\antragstellendeinstitution{Leibniz Universität Hannover\\...}` and `\kooperationspartner{Universität der Künste Berlin\\...}`.
- `print/template/zukunftbau/forschungsbericht.content.tex`: `\author{Projektkonsortium}` → `\antragstellendeinstitution{Projektkonsortium}`.
- `print/template/zukunftbau/zwischenbericht.content.tex`: `\author{Projektteam}` → `\antragstellendeinstitution{Projektteam}`.
- Leave `\maketitle`/`\semio_maketitle:`-based templates (`report`, `paper`, `flyer`, `kompaktbericht`) untouched — they don't call `\makecoverpages`.

## 7. Verify
- Re-read all touched files right before editing (concurrent sessions are live on them).
- `cd print && bun ./script.ts test` to build all template PDFs (light+dark).
- Rasterize zwischenbericht page 1 (reuse the pdfjs+canvas approach from the `PRINT-UNIFORM-BLOCK-SPACING` ticket's `measure-spacing.ts`) to confirm: 6 stacked window rows render with bordered chrome and equal single-unit gaps, and the logo row shows 3 equal-width bordered windows with a fixed single-unit gutter (not elastic).
- Store any temp scripts/screenshots inside the new ticket folder only.
- Close the ticket with a summary of files touched.

## Files touched
- `print/tex/semio-window.sty` (new `Window` env + `SemioWindowRowTwo`/`SemioWindowRowThree`)
- `print/tex/semio-core.sty` (new `antragstellendeinstitution`/`kooperationspartner` fields)
- `print/tex/semio-components.sty` (`\makecoverpages` rewrite)
- `mit-bestand/bericht/zwischenbericht/zwischenbericht.tex`, `zwischenbericht-dark.tex`
- `print/template/zukunftbau/forschungsbericht.content.tex`, `zwischenbericht.content.tex`
