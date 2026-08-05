# Pixel-truth validation (2026-08-04)

PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`  
Vision model captions of crops were **unreliable** (reported flush/notches/double
lines where RGB scans show the opposite). Metrics below are from `@napi-rs/canvas`
sampling of rendered crops.

## Kopfbau / P.K.1 (page 24)

| Check | Result |
|--------|--------|
| Hairlines under chip | **1** (`t2-pk-chip.png` y64–65 only) |
| Cream gap under baseline | **0** |
| Photo top air | **5.5–5.67pt** canvas (`240,236,221`) then photo |

Crops: `t2-pk-seam.png`, `t2-pk-chip.png`, `t2-pk-join.png`

## Marktplätze · Zugang (page 78)

| Check | Result |
|--------|--------|
| Hairlines under chip | **1** (`t2-m78b-title.png` y60 only) |
| L-border at joins | **RULE through join** (`t2-m78b-joins.png` borderX=2, L≈128 at hrule rows) |
| Cream notches on L-border | **0** |

Crops: `t2-m78b-title.png`, `t2-m78b-joins.png`

## Risiken window table (page 19)

| Check | Result |
|--------|--------|
| Hairlines under chip | **1** (`t2-risk-seam.png` y50–53) |
| Cream gap | **0** |
| Cream notches on L-border | **0** |

## Sty changes that landed

- `semio-window.sty`: open muted caps; inline baseline on page-1; **no** overlay
  restroke of `frame.north` (removed double hairline).
- `semio-table.sty`: full-width owns@sides mid-rule + smashed L/R pillars;
  chrome weld; project band vmode canvas `\hrule` bodypad before photo.
