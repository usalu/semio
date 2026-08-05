# Cross-table consistency matrix (multi-zoom)

PDF: `zwischenbericht/dist/zwischenbericht.pdf` (rebuild after p-column + strut retune)  
Method: RGB via `@napi-rs/canvas` at scales **2 / 4 / 6** (not vision captions).

## Fix landed this pass

1. **Chrome weld** uses `\prevdepth` (font-correct row depth).
2. **Columns `m` → `p`** so wrapped headers no longer inflate top air on short labels
   (Marktplätze was **11.5pt** vs peers **~7.8pt**).
3. **Struts retuned** for p-columns: `vtop=1.52em`, `vbot=1.02em` (TOC half:
   `1.20em` / `0.70em`).

## Results (primary: scale 2; scale 6 cross-check)

| Surface | Page | top air | left inset | hairline | cream gap | L-notches | photo pad |
|---------|------|---------|------------|----------|-----------|-----------|-----------|
| TOC | 3 | **4.0** (half) | 6.5 | single | 0 | 0 | — |
| Meilensteine (window) | 16 | **6.5** | 6.0 | single* | 0 | 0 | — |
| Risiken (window) | 17 | **6.5** | 6.5 | single* | 0 | 0 | — |
| Kopfbau (project) | 22 | — | — | single | 0 | 0 | **5.5** |
| Hürden (window) | 70 | ~6.5† | 6.5 | single* | 0 | 0 | — |
| Überblick (window) | 71 | **6.5** | 6.5 | single | 0 | 0 | — |
| Marktplätze Zugang (long 7.2pt) | 72 | **6.0–6.2** | 6.0–6.5 | single | 0 | 0 | — |
| Datenfelder (long 7.2pt) | 73 | **6.0–6.2** | 6.5 | single | 0 | 0 | — |
| Glossar (long) | 115 | **6.0–6.5** | 6.5 | single | 0 | 0 | — |

\*Window crops can show a false “second band” from chip **text** ink (~40% mid-span);
full-width RULE is a single cluster (verified on `final2-risk.png` / `final2-meil.png`).  
†Hürden auto-anchor is noisy (running header); join notches **0** on dedicated render.

## Consistency verdict

| Param | Target | Status |
|-------|--------|--------|
| Chip hairlines | 1 | **OK** across types |
| Chip↔body cream gap | 0 | **OK** |
| L-border join notches | 0 | **OK** |
| Text top air | ~6–6.5pt (TOC ~4) | **OK** — Markt aligned with peers |
| Text left inset | ~6–6.5pt | **OK** |
| Project photo pad | 5.5pt | **OK** |

Crops: `final2-*.png` in this ticket folder.
