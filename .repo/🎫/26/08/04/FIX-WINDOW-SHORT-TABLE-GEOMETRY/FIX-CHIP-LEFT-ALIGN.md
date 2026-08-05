# Chip left border alignment

## Symptom
Tiny gap / jog between title-chip left border and table left border.

## Cause
Chrome row used side border slots + hairline pull. That left the chip top-left
~0.5pt inside the body `\semio@table@border@L`.

## Fix
`\semio@table@long@title@chrome@row` spans the full table with **flush** muted
chips (no L/R slots, no shrink, no pull). Chip L shares the body border x.

## QA (dark, 1152 dpi)
Zugang / Risiken / TOC / Kopfbau: topL = chipL = bodyL, d = 0.
Crop `flush-zugang.png`: one continuous L, no steps.
