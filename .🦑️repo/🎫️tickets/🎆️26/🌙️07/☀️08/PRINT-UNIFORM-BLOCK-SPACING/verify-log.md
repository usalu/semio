# Print Uniform Block Spacing — Reopen Verify

## Reopen scope

Superseded the original fix's deliberate 2×/1× before/after asymmetry
(`semio@block@sep@before@skip = 2 × semio@spacing@single`). Dev now wants
exactly one unit between every adjacent body element — paragraph<->paragraph,
paragraph<->window, window<->window, heading<->anything, nested blocks —
keyed to the paragraph-to-paragraph gap (`\parskip`).

## Mechanism

`\parskip` is set to `\semio@block@sep@skip` (the single unit, 0.2em).
Every block emits one `\addvspace`-based before-space via `\semio@block@before`
(max-merging, vanishes on an empty vlist so page tops stay flush); nothing
emits an explicit after-space — the tcolorbox `after skip` and the next
paragraph's `\parskip` deliver the gap instead. KOMA's own before/after skip
is neutralized to `1sp` (must stay >0pt to avoid run-in heading style) since
KOMA suppresses `beforeskip` under `@nobreak` (heading directly following
another heading); the real before-space for headings is emitted
unconditionally by `\semio@block@before` inside `\semio@koma@heading@lines`.

Two additional unshielded `\noindent`s were found and fixed during
implementation (both were harmless while `\parskip=0pt`, but became a second,
stacked unit once `\parskip` became the unit):
- `\semio_window_header_muted_use:` (window header row invoke)
- (chip row wraps `\semio@heading@row@wrap` / `\semio@window@header@row@wrap`
  were already the primary shield targets per plan)

A stray `\noindent` in the kind-window begin path (between the header row and
`\tcolorbox` open) was removed as dead weight.

## Files changed

- `print/tex/semio-core.sty` — replaced `\semio@block@sep@before@skip` /
  `@before@set` / `\semio@block@sep` / `\semio@heading@block@before` with
  `\semio@block@before`, `\semio@block@after`, `\semio@noindent@noparskip`.
- `print/tex/semio-window.sty` — `\AfterEndPreamble` sets `\parskip` to the
  unit and resets `\chapterheadstartvskip`; `\semio_block_sep_before:` now
  calls `\semio@block@before`; `\semio_block_sep_after:` and its 3 call
  sites deleted; tcbset `after skip` changed from `0pt` to the unit;
  `\semio@heading@block@after` deleted; `\semio@koma@heading@lines` now
  calls `\semio@block@before` unconditionally; `\semio@heading@install@spacing`
  uses `1sp`/`1sp`; `\semio@heading@row@wrap` / `\semio@window@header@row@wrap`
  / `\semio_window_header_muted_use:` shielded with `\semio@noindent@noparskip`;
  `SemioNest`'s `\semio@nest@paragraph` uses `\semio@block@before`/`@after`;
  stray `\noindent` removed from kind-window begin.

## Verification

1. `verify-uniform.tex` (ticket-local fixture) exercising all adjacencies —
   compiled clean via Tectonic, rendered to `uniform-p1.png`..`uniform-p5.png`.
   Visual inspection confirms: paragraph<->paragraph, paragraph<->window,
   window<->window, 5 stacked headings (chapter through subparagraph),
   heading<->window, window<->heading, and `SemioNest` all render with the
   same small, uniform gap. Page-top behavior (`\newpage`, chapter start)
   confirmed flush — no residual top-of-page skip.
2. Full build: `bun ./📜️script.ts build` in `print/` for all 6 templates.
   `report` fails on an unrelated, pre-existing WIP bug (`Undefined control
   sequence` in `\semio_image_cover_trim_distribute:nnn`, part of an
   in-flight anchor-aware image-cover-crop change already present in the
   working tree before this ticket reopened — confirmed via
   `git diff HEAD -- print/tex/semio-window.sty`, unrelated to spacing,
   not touched by this fix). The other 5 templates (paper, flyer,
   forschungsbericht, zwischenbericht, kompaktbericht) all built clean,
   light+dark, no errors.
3. Real production document: `bun ./📜️script.ts build` in
   `mit-bestand/bericht` — the `zwischenbericht` document matching the
   dev's original screenshot — built clean, light+dark, no errors.
   Rendered pages 10-11 (`zwb-p10.png`, `zwb-p11.png`) visually confirm
   uniform gaps between every subsection chip and its paragraph, and
   between consecutive paragraph/heading blocks, throughout dense
   real content (previously showed the reported inconsistent spacing).
