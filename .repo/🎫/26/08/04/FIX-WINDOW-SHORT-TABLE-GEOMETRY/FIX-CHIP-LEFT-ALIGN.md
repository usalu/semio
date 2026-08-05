# Chip left border alignment

## Symptom
Title chip left border looked inset from the table outer left (tiny gap / jog),
especially at the chip top-left corner.

## Cause
Chrome row painted a live `\semio@table@border@L` *behind* the muted chip.
Outer-edge scans looked flush (continuous border@L ink), but the chip's own
left/top strokes sat ~1 device pixel inside that rule.

## Fix
- `\semio@table@border@L@chrome` — width phantom only (like R@chrome).
- Chrome multicolumn uses L@chrome / R@chrome; chip paints L in the chrome band;
  body rows still use live `\semio@table@border@L`.
- Non-flush muted row: `\hspace*{-\hairline}` + span `\linewidth+2·hairline`.
- `\semio@noindent@noparskip` no longer groups away `\noindent`.

## QA (dark, 1152 dpi)
Zugang / Risiken / TOC / Kopfbau: chip run == body run (same s/e), top stroke
L == body L, dOuter = 0.