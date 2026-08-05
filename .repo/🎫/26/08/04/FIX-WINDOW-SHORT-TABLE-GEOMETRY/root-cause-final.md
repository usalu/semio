# Root Causes + Proof (Project Cards / owns@sides Joins)

Rebuild: `cd mit-bestand/bericht && bun ./script.ts latex`  
PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`  
Pages (this build): Kopfbau = **24**, materialrest24 = **77**

## 1. Cream gap / double hairline under title chips

### Root cause
`\SemioProjectHeadingSilent` does **not** draw visible chips. Chips come from
`\semio@table@long@title@chrome@row` → `\semio@window@header@invoke` →
open muted caps (`\semio@heading@cap@muted@open`) + inline baseline.

The “gray box top” is that **same** baseline (not a second band border). A cream
slit appeared when chrome weld under-pulled (`\\` strut depth) and/or page-1
tcolorbox overlay restroked `frame.north` parallel to the inline baseline.

Open-gap PAGE between title and key chips is intentional (open tabs); under the
chips themselves the fill is canvas down to the baseline.

### Fix
- Open muted caps + inline baseline (`semio-window.sty`)
- Page-1 overlay does **not** restroke baseline (`overlay~first={}`)
- Chrome weld: `-\extrarowheight-\dp\@arstrutbox-0.5\arrayrulewidth`

### Pixel proof (scale 4, under title chip)
`proof-pk1-seam-zoom.png` / scan of `proof-pk1-seam.png`:

| y (rel) | kind | RGB |
|--------|------|-----|
| −1 | CANVAS | 240,236,221 |
| 0..2 | RULE (0.75pt) | 123,130,125 |
| 3..24 | CANVAS | 240,236,221 |
| 25+ | PHOTO | orange |

→ **one** hairline, **0** page strip under chips.

## 2. Photo flush to content top (no toppad)

### Root cause
Bare band cell left the TikZ cover (`baseline=0pt`) flush on the chip baseline.
Plain `\vskip` is unpainted glue (reads as cream under the weld). An
`\hbox{\rule{…}{bodypad}}` can be swallowed by the tall upward TikZ box in
paragraph mode.

### Fix (`\semio@project@overview@band`)
`\vbox` with **vmode** canvas `\hrule height\semio@window@bodypad` then
`\nointerlineskip` + photo hbox. Photo height = `bandheight − bodypad`.

### Pixel proof
Same seam scan: **5.50pt** canvas between RULE and PHOTO (y Δ = 22px @ scale 4).

Proof images: `proof-pk1-pad.png`, `proof-pk1-band.png`, `proof-pk1-seam-zoom.png`

## 3. Vertical border notches at row joins

### Root cause
Side `\vrule`s live only inside row boxes and do not extend through `\noalign`.
An inset mid-rule left L/R slots empty (true cream notches). Grouped
`{\color…\vrule}` can pack to natural height on tall m-rows.

### Fix (`\semio@table@rule` + borders)
- Ungrouped stretchable `\semio@table@border@L/R` + restore text colour
- owns@sides: full-width `\hrule`, then **smashed** L/R pillars overlapping ±2pt
  into adjacent rows on top of the hrule (smash ⇒ band advance stays one hairline;
  plain negative-vskip pillars previously opened multi-pt page gaps)

### Pixel proof
`proof-markt-body.png`: `joingap` → **0** cream/page gaps at joins.  
`proof-markt-Ljoin.png`: border columns x=11–13 are RULE on join rows and mid-rows.

Proof images: `proof-markt-body.png`, `proof-markt-Ljoin.png`, `proof-markt-Ljoin-x8.png`

## Macros touched
- `print/tex/semio-table.sty`: `\semio@table@border@L/R`, `\semio@table@rule`,
  `\semio@table@long@title@chrome@row` weld, `\semio@project@overview@band`
- `print/tex/semio-window.sty`: page-1 overlay (no baseline restroke), open muted
  caps / invoke@tcb (already in place; comments aligned)
