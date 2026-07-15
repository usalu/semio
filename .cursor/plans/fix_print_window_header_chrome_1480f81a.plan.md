---
name: Fix print window header chrome
overview: 'Fix the broken print "window" title-bar chrome in print/tex/semio-window.sty so it matches the OS UI''s tab/cap + cutout pattern: visible left title chip and right number chip, a non-continuous top border (only above the chips, gap open in the middle), and no duplicate/stray border lines.'
todos:
 - id: fix-header-store-gdef
   content: Change \def to \gdef for semio@window@header@number/title in semio_window_header_store_aux:nn
   status: completed
 - id: fix-empty-check
   content: "Fix the ctrl-box empty check in semio@window@header@muted to expand #1 before testing emptiness"
   status: completed
 - id: fix-gap-cutout
   content: Swap semio@window@gap@inbox for semio@window@gap in semio@window@header@muted to restore the cutout
   status: completed
 - id: remove-redundant-rule
   content: Remove the redundant trailing full-width stroke hbox in semio@window@header@muted
   status: completed
 - id: rebuild-and-verify
   content: Rebuild zwischenbericht, rasterize, and visually verify chips/borders/gap match the OS UI reference
   status: completed
 - id: update-ticket
   content: Update verify-log.md and close/reopen ticket PRINT-WINDOW-BORDER-GAP with summary
   status: completed
isProject: false
---

## Root cause (confirmed by rebuilding `zwischenbericht.tex` and rasterizing the cover page)

All symptoms trace to `print/tex/semio-window.sty`, specifically the muted window-header path used by every `Window`/`semiotable`/`semiofigure`/etc. environment (`\semio_window_header_muted_use:` → `\semio@window@header@muted`). Heading chips (`\semio@heading@pair@muted`, used by `\section`/`\paragraph`) are unaffected and render correctly — confirming the bug is isolated to the window-header code path, not the shared chip primitives.

### Bug 1 — title/number are always empty (chips collapse to blank slivers)

```318:326:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_header_store_aux:nn #1#2 {
  \group_begin:
  \ExplSyntaxOff
  \makeatletter
  \def\semio@window@header@number{#1}
  \def\semio@window@header@title{#2}
  \makeatother
  \group_end:
}
```

`\def` here is **local** to `\group_begin: ... \group_end:`, so both assignments are discarded the instant this function returns. By the time `\semio_window_header_muted_use:` actually reads `\semio@window@header@number`/`\semio@window@header@title`, they have reverted to their empty initial definitions:

```887:888:print/tex/semio-window.sty
\def\semio@window@header@number{}
\def\semio@window@header@title{}
```

This is why every window's title chip renders as an empty, 2×padding-wide box (just its own left/right hairlines sitting close together — the "duplicate border on the left") and why no title/number text ever shows. The existing codebase convention for this exact pattern (persisting a value across a catcode-scoping group) is `\gdef`, already used in `\semio@chrome@heading@set`:

```879:881:print/tex/semio-window.sty
\newcommand{\semio@chrome@heading@set}[1]{%
  \gdef\semio@chrome@heading{#1}%
}
```

**Fix:** change the two `\def`s in `\semio_window_header_store_aux:nn` to `\gdef` so they survive past `\group_end:`.

### Bug 2 — the "empty number" check tests the wrong thing

```1387:1398:print/tex/semio-window.sty
\newcommand{\semio@window@header@muted}[2]{%
  \sbox{\semio@window@tab@box}{\semio@heading@cap@muted{#2}}%
  \sbox{\semio@window@ctrl@box}{%
    \if\relax\detokenize{#1}\relax
      \hbox{}%
    \else
      \semio@heading@cap@muted@tab{#1}%
    \fi
  }%
  ...
```

`#1` here is the literal token `\semio@window@header@number` (not its expanded value), so `\detokenize{#1}` detokenizes the macro _name_, never `\relax` — the "empty" branch is unreachable and an empty-but-bordered ctrl chip is always drawn. **Fix:** expand `#1` into a temporary macro first and compare with `\ifx...\@empty` (standard LaTeX idiom), e.g.:

```latex
\edef\semio@window@header@numval{#1}%
\ifx\semio@window@header@numval\@empty
  \hbox{}%
\else
  \semio@heading@cap@muted@tab{#1}%
\fi
```

### Bug 3 — gap segment draws a top border, hiding the tab "cutout"

Per the live UI reference (`ui/js/react/index.tsx`, `windowCapFrameClass`/`windowGapFrameClass`, confirmed via explore): a cap/chip is `border-t + border-x, no border-b`; the gap between chips is `border-b only`; nothing draws a full-width top rule. The already-correct helper matching this is `\semio@window@gap` (used by working heading chips):

```1342:1363:print/tex/semio-window.sty
\newcommand{\semio@window@gap}[1]{%
  ...
      \hbox to \dimexpr\semio@window@gap@w\relax{%
        \colorbox{#1}{ ... }%
      }%
      \nointerlineskip
      \hbox to \dimexpr\semio@window@gap@w\relax{%
        \semio@window@stroke@h{semio-chrome-border-normal}{\semio@window@gap@w}%
      }%
  ...
}
```

But `\semio@window@header@muted` uses `\semio@window@gap@inbox` instead, which draws its stroke **first** (top) then the color fill — producing a continuous top border across the whole row and hiding the U-shaped cutout:

```1042:1045:print/tex/semio-window.sty
\hbox to \dimexpr\semio@window@gap@w\relax{%
  \semio@window@stroke@h{semio-chrome-border-normal}{\semio@window@gap@w}%
}%
```

**Fix:** swap `\semio@window@gap@inbox{semio-chrome-base}` → `\semio@window@gap{semio-chrome-base}` in `\semio@window@header@muted`.

### Bug 4 — redundant/mis-sized trailing border line

`\semio@window@header@muted`'s row wrapper also draws an extra rule after the tab/gap/ctrl row:

```1393:1396:print/tex/semio-window.sty
\nointerlineskip
\hbox to \linewidth{%
  \semio@window@stroke@h{semio-chrome-border-normal}{\semio@window@gap@w}%
}%
```

This duplicates the border the gap segment will already draw at its own bottom (once Bug 3 is fixed) and is sized to `\semio@window@gap@w` instead of `\linewidth`, so it never even spans correctly. The working `\semio@heading@pair@muted` has no equivalent trailing rule. **Fix:** remove this `\hbox` entirely.

### Residual "vertical gap between first line and content"

Once the chips render with real content/height (Bug 1), re-check whether the `\vskip -\semio@stroke@hairline` in `\semio_window_vskip_stroke_hairline:` still leaves a visible seam between the header row and the `tcolorbox` body; adjust the vskip amount if a gap remains after the above fixes (it may already resolve as a side effect of Bug 1).

## Implementation

1. Reopen ticket `PRINT-WINDOW-BORDER-GAP` (already open, covers this exact area — "borders should touch seamlessly").
2. Apply the four fixes above in [print/tex/semio-window.sty](print/tex/semio-window.sty).
3. Rebuild `zwischenbericht` (and `report`, if the unrelated pre-existing `\SemioReferences`/`etoolbox` build failure in that template doesn't block it — that failure belongs to ticket `PRINT-BIBLIOGRAPHY-REFERENCES-TABLE` and is out of scope here) using `bun print/script.ts build zwischenbericht`.
4. Rasterize page 1 with the existing `.repo/🎫/26/07/09/PRINT-WINDOW-BORDER-GAP/rasterize.ts` (and the standalone `minimal-window.tex`/`verify-window.tex` harnesses already in that ticket folder) and visually confirm:
   - Title chip shows its label text; number chip shows its number (or is truly absent/zero-width when there is no number).
   - Top border only appears above the chips (U-shaped cutout), not as one continuous line.
   - No duplicate hairlines at the left edge.
   - No visible gap between the header row and the window body.
5. Update `verify-log.md` in the ticket with before/after captures and close/summarize via `ticket_close`.
