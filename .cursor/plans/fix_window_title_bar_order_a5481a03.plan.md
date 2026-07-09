---
name: Fix window title bar order
overview: Root-cause the window title-bar ordering bug (chips/rule rendering below the box content instead of above it) and unfreeze the stuck TeX build.
todos:
  - id: kill-stuck-tectonic
    content: Terminate orphaned tectonic processes blocking the watch build
    status: completed
  - id: fix-vtop-to-vbox
    content: Change \vtop to \vbox in \semio@window@header@muted (print/tex/semio-window.sty)
    status: completed
  - id: remove-dead-inbox-code
    content: Remove unused inbox-header/double-hairline leftovers from an abandoned attempt
    status: completed
  - id: rebuild-and-rasterize
    content: Rebuild zwischenbericht/report and rasterize to confirm chip row now renders above box content and cover title isn't clipped
    status: in_progress
  - id: update-ticket
    content: Update verify-log.md and close ticket PRINT-WINDOW-BORDER-GAP with the corrected root cause
    status: pending
isProject: false
---

# Fix Window Title Bar Rendering Order

## Root cause

`git diff --cached -- print/tex/semio-window.sty` shows the only two "stack chip-row + separator rule" constructs in the file use different TeX box primitives:

- [print/tex/semio-window.sty:1036](print/tex/semio-window.sty) `\semio@heading@row@wrap` (used for section/paragraph heading chips, which render correctly) wraps its content in `\vbox{ ... }`.
- [print/tex/semio-window.sty:1355](print/tex/semio-window.sty) `\semio@window@header@muted` (used for `Window` title bars, which are broken) wraps the same kind of content — chip row `\hbox`, then separator-rule `\hbox` — in `\vtop{ ... }` instead.

This is the actual bug. Per the TeXbook box model:
- `\vbox{A B}` sets the box's reference point (baseline) at the bottom of the **last** item `B` → depth stays near zero (rule has no depth), height carries almost the whole block. Inserted into the vertical list, the current point lands right after the separator rule, exactly where the `tcolorbox` body should begin. This is why headings work.
- `\vtop{A B}` sets the reference point at the bottom of the **first** item `A` (the chip row) instead. That flips the height/depth split: most of the visible content (chip glyphs) ends up counted as **depth**, hanging below the reference point. When this box is the very first thing on a page (the cover-page `Titel` window), that depth region pokes above the usable page area and gets clipped at the top. Further down the page, it causes the chip/rule row to visually land **after** the box content instead of before it, matching both screenshots.

## Fix

In [print/tex/semio-window.sty](print/tex/semio-window.sty), function `\semio@window@header@muted` (~line 1355-1381):

```1367:1381:print/tex/semio-window.sty
  \semio@window@header@row@wrap{%
    \vtop{%
      \nointerlineskip
      \hbox to \linewidth{%
        \usebox{\semio@window@tab@box}%
        \semio@window@gap@inbox{semio-chrome-base}%
        \usebox{\semio@window@ctrl@box}%
      }%
      \nointerlineskip
      \hbox to \linewidth{%
        \semio@window@stroke@h{semio-chrome-border-normal}{\linewidth}%
      }%
    }%
  }%
}
```

Change `\vtop{` to `\vbox{` (matching the already-correct heading pattern). No other geometry changes should be necessary; the existing `\semio_window_vskip_stroke_hairline:` single-hairline pull-up (already used consistently for both `Window` code paths) should then correctly overlap the header's bottom rule with the `tcolorbox` top border.

## Cleanup of dead code from an earlier abandoned attempt

`git diff --cached` also confirms an abandoned "inbox header" (`before~upper`) experiment was left in the file, no longer wired into the `tcbset` `semio~window` style (no `before~upper` key present). Repo-wide grep confirms these are unreferenced outside the file itself:

- `\semio@window@render@inbox@header` (~line 810)
- `\semio@window@header@inbox` (~line 1003)
- `\semio@heading@cap@muted@inbox` (~line 979, only used by the above)
- `\semio_window_vskip_double_stroke_hairline:` (~line 154, never called)

Remove these per the "no leftover migration/dead code" rule. Keep `\semio@window@gap@inbox` since it is actively used by the fixed `\semio@window@header@muted` path.

## Unfreeze the build

`ps aux` currently shows 13 orphaned `tectonic` processes left running from earlier verification attempts (spawned across several background shell calls in this and the prior session, none of which were cleaned up). These contend for CPU/file locks and are almost certainly why `bun nx run @semio-tech/mit-bestand-bericht:watch` looks frozen at "Running TeX...". Terminate the stray `tectonic` PIDs (not any IDE/editor process) before rebuilding.

## Verification

1. Kill stray `tectonic` processes.
2. Rebuild a template that exercises both the cover-page `Window[title=...]` row (e.g. `zwischenbericht` or the standalone verify tex already in the ticket folder) and a plain in-body `Window`/`Figure` (e.g. `report`).
3. Rasterize the relevant pages (ticket already has `.repo/🎫/26/07/09/PRINT-WINDOW-BORDER-GAP/rasterize.ts`) and visually confirm: chip row + separator sit above the box content, borders connect, and the cover title is no longer clipped.
4. Update `.repo/🎫/26/07/09/PRINT-WINDOW-BORDER-GAP/verify-log.md` with the corrected root cause and reopen/close the ticket with the fix summary.
