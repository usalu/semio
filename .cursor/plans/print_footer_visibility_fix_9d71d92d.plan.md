---
name: Print Footer Visibility Fix
overview: Fix the print LaTeX page geometry so the footer chrome bar is fully on-page and its outer spacing/positioning mirrors the header, and so `\textheight` correctly reserves room for both bars instead of overflowing.
todos:
 - id: expose-margin-token
   content: Add \semio@page@margin macro per document type in semio.cls
   status: completed
 - id: consolidate-geometry
   content: Replace the header/footer geometry block in semio-window.sty with one includehead/includefoot \geometry call sharing \semio@chrome@page@gap for top and bottom
   status: completed
 - id: remove-dead-code
   content: Remove the dead \addtolength{\topmargin} line and the old standalone \geometry{bottom=...} call
   status: completed
 - id: rebuild-and-verify
   content: Rebuild all templates (light+dark) and visually verify header/footer symmetry, footer visibility, and no content overflow, including the flyer's smaller margin
   status: completed
isProject: false
---

# Print Footer Visibility Fix

## Root cause

In [print/tex/semio-window.sty](print/tex/semio-window.sty) (around lines 1999-2005):

```1999:2005:print/tex/semio-window.sty
\setlength{\headheight}{38.14006pt}
\addtolength{\topmargin}{-21.14006pt}
\setlength{\headsep}{\semio@spacing@double}
\newlength{\semio@chrome@footer@reserve}
\setlength{\semio@chrome@footer@reserve}{\dimexpr\semio@spacing@double+\semio@chrome@footer@height+\semio@stroke@hairline\relax}
\setlength{\footskip}{\semio@chrome@footer@reserve}
\geometry{bottom=\semio@spacing@single}
```

`\footskip` is set to the full footer-bar reserve (~24.8pt) but `\geometry{bottom=...}` sets the bottom margin to a single spacing unit (~2.2pt), **without `includefoot`**. Per the `geometry` package, `height := textheight` by default — `headheight+headsep` and `footskip` are excluded from the page-height balance unless `includehead`/`includefoot` are set (confirmed via the package docs and build log verbose output: `\textheight=771.725pt` while `\paperheight - top - bottom` already accounts for the whole page, i.e. `\footskip` isn't subtracted at all).

Consequences, confirmed in the already-rendered `report.pdf` (see `.repo/🎫/26/07/08/PRINT-UNIFORM-BLOCK-SPACING/report-p4-spacing.png` and `report-p5-spacing.png`, and `print/dist/report.log` verbose geometry block):

- **Footer invisible**: without `includefoot`, the footer floats _below_ the last text line by `\footskip` (~24.8pt) _inside_ the `bottom` margin band, but that band is only ~2.2pt — the footer bar's bottom edge lands ~22.6pt past the physical page edge and is clipped/off-page.
- **Content overflows / "doesn't break properly"**: `\textheight` is computed as `paperheight - top - bottom` only, ignoring both the header and footer reserve, so pages are packed with text almost edge-to-edge (visible in `report-p4-spacing.png`, a TOC page whose rows run to the very bottom with no footer).
- The **header only works by coincidence**: `top` stays at the class's `margin=2.5cm`/`1.2cm` (from [print/tex/semio.cls](print/tex/semio.cls) lines 22-26), and because nohead's floating formula subtracts `\headheight+\headsep` from that same margin, report/paper still end up with a positive ~28.6pt gap above the header. This was never made an explicit, shared value with the footer, and `\addtolength{\topmargin}{-21.14006pt}` on line 2000 is dead code — `\topmargin` is always re-derived by `geometry`'s deferred computation, so this manual tweak has no visible effect (confirmed by matching the log's reported `\topmargin=-43.6581pt` against the pure `top - headheight - headsep - 1in` formula with no addtolength term).

## Fix

Consolidate the geometry setup into one explicit call using `includehead,includefoot` (the documented pattern for custom fancyhdr chrome bars), with `top` and `bottom` driven by **one shared token** so the footer's outer gap is identical to the header's:

1. In [print/tex/semio.cls](print/tex/semio.cls), expose the per-type page margin as a macro next to the existing `\RequirePackage[...,margin=...]{geometry}` calls (lines 21-27), e.g. `\newcommand{\semio@page@margin}{2.5cm}` / `{1.2cm}` for `flyer`, so `semio-window.sty` can reference the real margin instead of duplicating a literal.

2. In [print/tex/semio-window.sty](print/tex/semio-window.sty), replace lines 1999-2005 with:
   - Keep `\headheight` (38.14006pt) and `\headsep` (`\semio@spacing@double`) as-is — these currently produce the approved header look and stay untouched.
   - Remove the dead `\addtolength{\topmargin}{-21.14006pt}` line.
   - Compute one shared length, e.g. `\semio@chrome@page@gap`, as `\dimexpr\semio@page@margin-\headheight-\headsep\relax` — this reproduces today's actual header gap exactly (same formula that currently determines it implicitly), just made explicit.
   - Keep `\semio@chrome@footer@reserve` (footskip formula) as-is.
   - Replace the standalone `\geometry{bottom=\semio@spacing@single}` with **one** consolidated call:
     ```
     \geometry{
       includehead, includefoot,
       top=\semio@chrome@page@gap,
       headheight=\headheight,
       headsep=\semio@spacing@double,
       bottom=\semio@chrome@page@gap,
       footskip=\semio@chrome@footer@reserve,
     }
     ```
     With `includehead`/`includefoot`, `top`/`bottom` now directly mean "gap from page edge to bar", so setting both to `\semio@chrome@page@gap` guarantees the footer's outer spacing is pixel-identical to the header's, and `\textheight` is correctly shrunk to leave room for both bars (fixing the overflow/"not breaking" symptom).

3. Rebuild all six templates (light+dark: `report`, `paper`, `flyer`, `forschungsbericht`, `zwischenbericht`, `kompaktbericht`) via `bun nx run print:build` (or `script.ts build`) and visually verify (raster a few pages per template, focusing on the last page of multi-page docs) that:
   - The footer bar renders fully on-page with authors/page-number visible.
   - The footer's bottom gap matches the header's top gap.
   - No text is overflowing/overlapping the footer bar.
   - `flyer`'s smaller `1.2cm` margin doesn't produce a negative gap (since `\headheight` needs less than the margin) — if it does, that's a pre-existing header issue on `flyer` that must be called out/handled (e.g. a smaller `\headheight` for that type, out of scope for this fix if not caused by it).

## Files touched

- [print/tex/semio.cls](print/tex/semio.cls) — expose `\semio@page@margin` per document type.
- [print/tex/semio-window.sty](print/tex/semio-window.sty) — consolidate header/footer geometry into one symmetric `\geometry{includehead,includefoot,...}` call, drop dead `\addtolength{\topmargin}` line and the old lone `\geometry{bottom=...}` call.

## Process notes

- Work will happen inside a new ticket (e.g. `PRINT-FOOTER-VISIBILITY`) under goal `🎯r2602`, per repo workflow — verification artifacts (rasters, logs) go into the ticket folder.
- No other files need to change; `semio-components.sty`'s unrelated `\newgeometry{...}` (cover-page override) is untouched.
