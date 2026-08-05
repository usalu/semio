# Final compare vs Kinan Zwischenbericht branch

**Reference:** `🐳kinan/⛳wip` commit `16` / sty from commit `15` (last Kinan sty before the continuous-edge pass).  
**Current:** commit `17` + comment cleanup (flush chips, inner mid-rules, side pillars).

Crops in this folder: `FINAL-cmp-p*.png` — **top = REF**, **bottom = CURRENT**.  
Page rasters: `ref16-*.png` vs `cur16-*.png` @216dpi.

## Edge metrics (reliable scan)

| Table | REF dL | CUR dL | REF cream | CUR cream | REF jit | CUR jit | Verdict |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| TOC (p3) | 0 | 0 | 0 | 0 | 0 | 0 | same |
| Meilensteine window (p18) | 0 | 0 | 0 | 0 | 0 | 0 | same |
| Risiken window (p19) | 2 | 0 | 0 | 0 | 0 | 0 | improved |
| Kopfbau project (p24) | 0 | 0 | 0 | 0 | 0 | 0 | same |
| Hürden long (p76) | 0 | 0 | 0 | 0 | 0 | 0 | same |
| Überblick window (p77) | 2 | 0 | 0 | 0 | 0 | 0 | improved |
| Marktplätze long (p78) | 0 | 0 | 0 | 0 | 0 | 0 | same |
| Glossar long (p121) | 0 | 0 | 0 | 0 | 0 | 0 | same |

- **dL**: chip left − body left (px @216dpi). `0` = flush with chips.  
- **cream**: cream pixels on the L border at row joins. `0` = continuous.  
- **jit**: max horizontal drift of the L border across rows. `0` = straight.

## Visual read

- Window tables (Meilensteine, Risiken): current welds chip→body L edge cleaner than REF (flush chips).  
- Long tables (Marktplätze, Glossar, Hürden): join cream/jitter stay at zero; outer edge matches Kinan alignment.  
- Project (Kopfbau): chip/body edge and photo band match REF.  
- No regressions vs Kinan last on text inset / chip seam / continuous L edge across the sampled set.

## What changed vs Kinan last (sty only)

1. Window chips flush to `\linewidth` (no `\moveleft` jog vs tcolorbox sides).  
2. Long-table mid-rules inner-only + smashed L/R pillars at joins.  
3. Window mid-rules exact `\linewidth` (no half-hairline outer overlap).
