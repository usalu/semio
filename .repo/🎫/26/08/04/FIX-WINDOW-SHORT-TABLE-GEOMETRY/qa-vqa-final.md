# Visual QA — window/table geometry (2026-08-05)

PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`  
Rasters: `qa144-0NN.png` (dpi 144 / scale 2), `qa432-0NN.png` (dpi 432 / scale 6)  
Crops: `vqa2-<id>-d<dpi>-{seam,join,cell,chip}.png`  
Metrics: `qa-vqa2-report.json` + manual RGB for Kopfbau@144 (AA hides mid-rule)

Vision captions alone are **not** trusted; verdicts use `@napi-rs/canvas` RGB.

## Pass/fail

| Page | Subject | Check | dpi | Metric | Verdict | Evidence |
|------|---------|-------|-----|--------|---------|----------|
| 24 | Kopfbau | Single chip baseline | 432 | hairlines=**1**, thick=**0.33pt** | **PASS** | `vqa2-kopfbau-d432-chip.png` |
| 24 | Kopfbau | Cream gap under baseline | 432 / 144 | **0pt** (canvas weld; borderL≈128) | **PASS** | `vqa2-kopfbau-d432-seam.png`, manual@144 y385 canvas |
| 24 | Kopfbau | L-border join notches | 432 | notches=**0**, joins all RULE L≈128 | **PASS** | `vqa2-kopfbau-d432-join.png` |
| 24 | Kopfbau | Photo top pad | 432 / 144 | **5.5pt** / **5.0pt** | **PASS** | target ≈5.5pt (±1) |
| 76 | Hürden | Single chip baseline | 144 / 432 | 1×**0.5pt** / 1×**0.67pt** | **PASS** | `vqa2-huerden-d*-seam.png` |
| 76 | Hürden | Cream gap | 144 / 432 | **0pt** | **PASS** | report |
| 76 | Hürden | L-border joins | 144 / 432 | notches=**0**, creamJoins=**0** | **PASS** | `vqa2-huerden-d432-join.png` |
| 76 | Hürden | Text insets L / T | 144 / 432 | **6.5 / 8** · **6.33 / 7.83** pt | **PASS** | chrome+cellpad band |
| 78 | Marktplätze | Single chip baseline | 144 / 432 | 1×**0.5pt** / 1×**0.17pt** | **PASS** | `vqa2-markt-d*-seam.png` |
| 78 | Marktplätze | Cream gap | 144 / 432 | **0pt** | **PASS** | report |
| 78 | Marktplätze | L-border joins | 144 / 432 | notches=**0**, creamJoins=**0** | **PASS** | `vqa2-markt-d432-join.png` |
| 78 | Marktplätze | Text insets L / T | 432 | **6.33 / 11.5** pt | **PASS*** | *T taller than peers (see note) |
| 121 | Glossar | Single chip baseline | 144 / 432 | 1×**0.5pt** / 1×**0.33pt** | **PASS** | `vqa2-glossar-d*-seam.png` |
| 121 | Glossar | Cream gap | 144 / 432 | **0pt** | **PASS** | weld canvas@yAfter |
| 121 | Glossar | L-border joins | 144 / 432 | notches=**0**, creamJoins=**0** | **PASS** | `vqa2-glossar-d432-join.png` |
| 121 | Glossar | Text insets L / T | 432 | **6.5 / 7.83** pt | **PASS** | matches Hürden |

## Notes

- **Kopfbau@144 auto-detect** missed the hairline (mid-span AA → L≈183 outside rule band). Manual scale-from-432 + border/canvas scan confirms **1** baseline, **0** cream gap, photo pad **5.0pt**.
- **Marktplätze top inset 11.5pt** vs Hürden/Glossar **7.83pt**: left inset identical (**6.33–6.5pt**); extra top air is in the first header row strut, not a cream notch or double baseline. Not treated as a sty defect without a design target for header-row strut.
- **No `semio-table.sty` / window sty change** — no pixel-proven bug on the four requested pages.

## Overall

**PASS** (report-only). Sty untouched.
