---
name: TOC Window Table Border Consistency
overview: "Make the TOC (and the other register-style tables: Glossary, References, list-of-X) visually match the bordered \"Window\" table look used by e.g. `Netzwerk` in `mit-bestand`, by giving the breakable `longtable` rendering a real left/right/bottom border and consistent row separators, and by removing the leftover flag/branching that currently makes these tables render inconsistently (or, for Glossary, skip the bordered/breakable path entirely)."
todos:
  - id: reopen-ticket
    content: Reopen TOC-SEMIO-WINDOW-TABLES ticket via repo MCP
    status: completed
  - id: colspec-border
    content: Add left/right border+padding cells to the three @long colspecs in semio-table.sty
    status: completed
  - id: closing-rule
    content: Add bottom closing rule via \endlastfoot in \semio@table@long@render
    status: completed
  - id: continuation-chrome-border
    content: Match border+padding in \semio@table@long@continuation@chrome multicolumn
    status: completed
  - id: remove-long-flag
    content: Remove \ifsemio@table@register@long boolean; make SemioTableRow always separate rows; make Register/Reference/Glossary table macros call the long renderer unconditionally
    status: completed
  - id: remove-window-toggles
    content: Remove the global long-flag toggling in semio_register_list_begin/end/reset in semio-window.sty
    status: completed
  - id: fix-glossary-bug
    content: Remove the stray \global\semio@table@register@longfalse override in SemioGlossaryListOf
    status: completed
  - id: rebuild-verify
    content: Rebuild zwischenbericht (light+dark), verify no errors, and visually compare TOC/Glossary pages against the Netzwerk table
    status: in_progress
  - id: close-ticket
    content: Update verify-log.md and close the ticket with summary and touched files
    status: pending
isProject: false
---

# TOC Window Table Border Consistency

## Root cause

`Netzwerk` (`makeworkpackages` in [print/tex/semio-components.sty](print/tex/semio-components.sty)) renders via the generic `Window` environment: a muted title capsule row, then a `tcolorbox` with `colframe`/left/right/bottom rules and `\semio@chrome@padding` insets, containing a plain `SemioTableThree` `tabular` with hairline separators between every row (`\SemioTableRow` → `\semio@table@row@sep`).

TOC/Glossary/References/list-of-* go through `\semio_register_list_begin:n` → `\semio@register@window@open` in [print/tex/semio-window.sty](print/tex/semio-window.sty:256). That path only renders the muted title capsule (`semio_window_header_muted_use:`) and then a bare `longtable` (`\semio@table@long@render` in [print/tex/semio-table.sty](print/tex/semio-table.sty:159)) — **no `tcolorbox` is ever opened**, so there is no left/right/bottom border and no padding, and (since the earlier multi-page fix) inter-row hairlines were disabled for `longtable` via the `\ifsemio@table@register@long` flag ([print/tex/semio-table.sty](print/tex/semio-table.sty:185)). This is the visual inconsistency the user is pointing at.

A `tcolorbox` can't reliably wrap a `longtable` (their page-break mechanisms don't coordinate), so the fix draws the border/padding as part of every table row (colspec decorations), which repeats automatically on every page — instead of wrapping the whole thing in a box.

There is also a real bug making it worse: `SemioGlossaryListOf` ([print/tex/semio-window.sty](print/tex/semio-window.sty:434)) explicitly does `\global\semio@table@register@longfalse` right after opening the register window, so the Glossary (rendered via `\listofglossaries` in `zwischenbericht.tex`) silently falls back to the old non-breakable, non-bordered `tabular` path — a second, different visual style. This override is removed as part of the fix.

## Changes

### [print/tex/semio-table.sty](print/tex/semio-table.sty)
- Extend the long-mode colspecs (`semio@table@colspec@register@long`, `@reference@long`, `@glossary@long`, [print/tex/semio-table.sty:84-88](print/tex/semio-table.sty)) with an outer border+padding cell on each side, matching the Window chrome stroke:
  `@{\color{semio-chrome-border-normal}\vrule width\semio@stroke@hairline\hspace{\semio@chrome@padding}} ... @{\hspace{\semio@chrome@padding}\color{semio-chrome-border-normal}\vrule width\semio@stroke@hairline}`.
  Since `\semio@chrome@padding` and `\semio@spacing@single` (used for `\tabcolsep`) are both `0.2em` ([print/tex/semio-tokens.sty](print/tex/semio-tokens.sty:51-58)), this lines up with `Window`'s padding without further tuning.
- Add a closing bottom rule (`\hline`-equivalent, same color/width) emitted via `\endlastfoot` in `\semio@table@long@render` ([print/tex/semio-table.sty:159-183](print/tex/semio-table.sty)), so the box visually closes on the last page — mirroring `Window`'s `bottomrule`.
- Update `\semio@table@long@continuation@chrome` ([print/tex/semio-table.sty:147-157](print/tex/semio-table.sty)) so its `\multicolumn` uses the same border+padding decoration on both sides, keeping the vertical border unbroken through the repeated title row on continuation pages.
- Remove the `\ifsemio@table@register@long` boolean entirely: simplify `\SemioTableRow` ([print/tex/semio-table.sty:185-193](print/tex/semio-table.sty)) back to always drawing `\semio@table@row@sep` between rows (matching `Netzwerk`'s per-row hairlines), and make `\SemioTableRegister`/`\SemioTableReference`/`\SemioTableGlossary` call `\semio@table@long@render` unconditionally (drop the dead short-mode `\else` branch in each). The short colspecs (`semio@table@colspec@register`/`@reference`/`@glossary`, without `@long`) stay — they're still used by `\SemioTableBegin`'s `nrp`/`nry` aliases, unrelated to this bug.
- Validate empirically that reinstating per-row `\noalign{\hrule}` inside `longtable` doesn't reintroduce the earlier "one entry per page" pagination bug (the prior fix's root cause was actually the `expl3` header macro in `\endhead`, not the row separator, but this needs confirming with a real rebuild). If it does regress, fall back to omitting the inter-row hairline (keep the outer border/padding only) rather than reintroducing broken pagination.

### [print/tex/semio-window.sty](print/tex/semio-window.sty)
- `semio_register_list_begin:n` ([print/tex/semio-window.sty:266-281](print/tex/semio-window.sty)): remove the `\global\semio@table@register@longtrue` block — no longer needed once the long renderer is unconditional.
- `semio_register_list_end:` / `semio_register_list_reset:` ([print/tex/semio-window.sty:283-295](print/tex/semio-window.sty)): remove the matching `\global\semio@table@register@longfalse`.
- `SemioGlossaryListOf` ([print/tex/semio-window.sty:434-448](print/tex/semio-window.sty)): remove the stray `\global\semio@table@register@longfalse` line so Glossary renders through the same bordered/breakable path as TOC.

## Verification

1. Reopen ticket `TOC-SEMIO-WINDOW-TABLES` via repo MCP (`ticket_reopen`), since this is a direct continuation of that work.
2. Rebuild `zwischenbericht` (light + dark) with `cd mit-bestand/bericht && bun ./script.ts build`.
3. Confirm no `Overfull`/`Misplaced \noalign` errors and that page count / TOC entry count stay consistent with the last known-good state (11 pages, TOC spanning pages 2-3 per [verify-log.md](.repo/🎫/26/07/08/TOC-SEMIO-WINDOW-TABLES/verify-log.md)).
4. Render TOC pages and the `Netzwerk` (`Arbeitspaketzuordnung`) page to images and visually compare: left/right/bottom border present on every TOC page, padding matches, row separators match, continuation title row border is unbroken.
5. Render the Glossary pages (`\listofglossaries`, page ~ where "Glossar" title appears) to confirm it now also uses the bordered/breakable style instead of the old flat table.
6. Update `verify-log.md` in the ticket with the new findings and close the ticket with a summary listing the touched files.
