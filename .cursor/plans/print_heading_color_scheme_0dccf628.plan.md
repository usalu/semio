---
name: Print Heading Color Scheme
overview: Give `\part`/`\chapter`/`\section`/`\subsection`/`\subsubsection`/`\paragraph` bordered/filled title banners that reuse the UI's actual color tiers (primary/secondary/tertiary/normal) and reserve the theme's "emphasized" text color for headings only, instead of the current situation where ordinary default styling already looks like the UI's hover/emphasis state.
todos:
  - id: ticket
    content: Open repo ticket under goal 🎯r2602 for the print heading color scheme work
    status: completed
  - id: heading-styles
    content: "Add %region Headings to print/tex/semio-window.sty: titlesec require, semio~heading~base tcbset style, per-level tier color definitions"
    status: completed
  - id: titleformat
    content: Wire \titleformat/\titlespacing for \part, \chapter (guarded), \section, \subsection, \subsubsection, \paragraph to render each inside its tcolorbox tier style with SemioSans
    status: completed
  - id: template-demo
    content: Extend print/template/report/report.content.tex to exercise section/subsection/subsubsection/paragraph under an existing chapter
    status: completed
  - id: build-verify
    content: Run bun ./script.ts build in print/, rasterize resulting PDFs to images, visually verify colors/contrast in light+dark themes across report/paper/flyer
    status: completed
  - id: close-ticket
    content: Close the ticket with summary and list of touched files
    status: completed
isProject: false
---

## Problem

Confirmed by exploring [print/tex/semio-window.sty](print/tex/semio-window.sty), [print/tex/semio-core.sty](print/tex/semio-core.sty) and [ui/styling/tokens.json](ui/styling/tokens.json):

- `\part`/`\chapter`/`\section`/`\subsection`/`\subsubsection`/`\paragraph` have **zero custom styling today** — they render as plain KOMA-Script (`scrreprt`/`scrartcl`) defaults, with no border, no fill, no distinct color.
- The `semio-primary`/`semio-secondary`/`semio-tertiary` brand colors are currently only used for the 14 special "window element" kinds (Image/Table/Blockquote/...), tier-mapped as visual→primary, logical→secondary, structural→tertiary (`print/tex/semio-window.sty` lines 41-43).
- `semio-chrome-foreground` (theme foreground, dark `#001117` in light theme / light `#f7f3e3` in dark theme) is exactly the UI's `text-emphasized` / `border-emphasized-color` token — the color the UI only shows on hover/active/selected states. In print this "emphasized" color is already the ambient default (body text, window box text, panel text), which is the misalignment the user is pointing at.
- The UI's resting/"normal" state uses gray (`--color-gray` `#7b827d`), aliased in print as `semio-chrome-border-normal`.

## Target mapping

| Level | Fill | Border | Text |
|---|---|---|---|
| Part | `semio-primary` | `semio-primary` | `semio-chrome-foreground` (emphasized) |
| Chapter | `semio-secondary` | `semio-secondary` | `semio-chrome-foreground` (emphasized) |
| Section | `semio-tertiary` | `semio-tertiary` | `semio-chrome-foreground` (emphasized) |
| Subsection | `semio-secondary` | `semio-secondary` | `semio-chrome-foreground` (emphasized) |
| Subsubsection | `semio-chrome-border-normal` (gray) | `semio-chrome-border-normal` | `semio-chrome-foreground` (emphasized) |
| Paragraph | `semio-chrome-canvas` (i.e. no visible fill, blends with page) | `semio-chrome-border-normal` (normal) | `semio-chrome-border-normal` (normal, gray) |

Border color mirrors fill color for the 5 filled levels (same convention already used by the window-element tier boxes: `colframe` = tier color). All six levels get the same bordered-banner treatment (confirmed with user) — Paragraph just has no accent fill and normal (not emphasized) text, so it reads as the lowest-emphasis / "default" level.

No new color tokens are needed — every color above already exists in `print/tex/semio-tokens.sty` / the theme aliases in `semio-core.sty`.

## Implementation

All changes live in [print/tex/semio-window.sty](print/tex/semio-window.sty) (already the home of the tier-color convention and already loads `tcolorbox`), inside a new `%region Headings` block, following the file's existing region-comment convention (see `%region Panels`):

1. Add `\RequirePackage{titlesec}` and define one shared `tcolorbox` base style (`semio~heading~base`) with hairline `boxrule`, `arc=0mm`, `\semio@chrome@padding` insets — same primitives already used by `semio~window`.
2. Add `\semio_heading_define:nnnn {name}{fill}{border}{text}` (mirrors the existing `\semio_window_define_tier:nn` helper) to register one `tcbset` style per level: `semio~heading~part`, `~chapter`, `~section`, `~subsection`, `~subsubsection`, `~paragraph`, using the mapping table above.
3. Redefine each sectioning command with `\titleformat{...}[block]{...}{<numbering>}{0pt}{\begin{tcolorbox}[semio~heading~<level>]<numbering>}[\end{tcolorbox}]` plus a matching `\titlespacing` using `\semio@spacing@single`/`\semio@spacing@double` for before/after vertical rhythm, consistent with existing spacing tokens. Text set in `\SemioSans` (same family already used for window/chip titles) with a decreasing size scale Part→Paragraph.
4. Guard `\chapter` styling with `\@ifundefined{chapter}{}{...}` since `scrartcl`-based `paper`/`flyer` types have no `\chapter`.

## Verification

- Extend the existing `report` template content ([print/template/report/report.content.tex](print/template/report/report.content.tex)) to exercise the full hierarchy at least once (add a `\section`/`\subsection`/`\subsubsection`/`\paragraph` under one existing `\chapter`) — no new example files, per repo convention; this template already ships as the canonical demo.
- Run `bun ./script.ts build` (or the `report` target) in `print/` to compile light + dark PDFs via Tectonic, then rasterize a page to PNG (Tectonic/tectonic-produced PDF -> image) to visually confirm: correct fill/border/text colors per level, legible contrast in both light and dark theme, and that `paper`/`flyer` (no `\chapter`) still compile cleanly.
- Fix any KOMA/titlesec interaction issues found during the real compile (e.g. spacing, page-break behavior for `\part`) before considering this done — do not claim it works without having compiled and visually inspected the PDF output, per repo rules.

## Ticket

Per repo workflow rules, this work will be tracked in a new ticket under goal `🎯r2602` (Running Sketchpad — same goal used by the prior `PRINT-WINDOW-ELEMENT-TAXONOMY` ticket which established the tier-color convention this reuses).

## Out of scope

Ambient body/prose text (plain paragraphs not under any heading command) keeps using the theme foreground color as before — only the six named heading/sectioning levels are restyled. Window-element chrome (chips, panels) is untouched.
