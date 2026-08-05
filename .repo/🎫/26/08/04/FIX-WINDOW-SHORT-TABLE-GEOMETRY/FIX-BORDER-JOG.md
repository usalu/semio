# Border Jog / Overlapping Side Segments

## Symptom
At some PDF zoom levels, longtable L/R outer borders read as stacked “tiny” overlapping segments with jogs at mid-rule joins (Zugang BB-08/09/11).

## Cause
`owns@sides` mid-rules used full-width `\hrule` plus smashed L/R pillars that climbed **+3pt into the row above**. Same-x double paint (row `\vrule` + pillar) rasterized as overlapping nubs / steps at fractional zoom.

## Fix
`print/tex/semio-table.sty` — `\semio@table@rule@band`: one picture-frame hbox
`[L stub | inner hairline | R stub]` spanning only the join band (no follow-up pillar pass, no +3pt climb). Windowed tables unchanged (`owns@sides` false → full-width `\hrule` to the frame inner face).

## QA
- Zugang p55 @ scales 4.7 / 5.5 / 6: `leftDevPx=0`, `rightDevPx=0`, mid thickness ≈ one hairline (not 2×).
- Crop `jog-zugang-6-L.png`: L border ink continuous on the same x columns through joins; T-junctions without double thickness.
- TOC p3 mid clusters ≈ 5–6px @ scale 8 (hairline 6px) — no continuation double hairline.
- No downward pillars → Glossar→Abk stub regression path closed.

Artifacts: `border-jog.ts`, `jog-zugang-*.png`, `user-jog-L.png`, `user-jog-R.png`.
