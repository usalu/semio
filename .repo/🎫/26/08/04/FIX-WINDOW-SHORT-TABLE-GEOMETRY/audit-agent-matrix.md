# Cross-Type Consistency Matrix - Zwischenbericht Tables

PDF: `e:/semio/mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`
Raster: **216dpi** (scale 3.000) via `@napi-rs/canvas` / pdfjs.
Measured: 2026-08-05T00:52:17.754Z

## Overall: **PASS**

| Table | Page | Kind | chipBodyDL | chipBodyDR | joinCreamL | joinCreamR | jitterL | jitterR | seamCream | headerTopAirPt | bodyTopAirPt | leftInsetPt | Flags |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| TOC | 3 | toc | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 5 | 8.67 | 6.33 | - |
| Meilensteine | 18 | window | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 8 | 9.33 | 6.67 | - |
| Risiken | 19 | window | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 8 | 9.33 | 6.67 | - |
| Kopfbau | 24 | project | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 5.67 | 5.67 | - | - |
| Huerden | 76 | long | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 8 | 8 | 6.67 | - |
| Ueberblick | 77 | window | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 7.67 | 9.67 | 6.33 | - |
| Marktplaetze | 78 | long | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 11.33 | 10.67 | 6.67 | - |
| Datenfelder | 79 | long | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 7.67 | 10 | 6.67 | - |
| Glossar | 121 | long | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 8 | 11 | 6.67 | - |

## Outlier rules

- chipBodyDL/DR: fail if abs(delta) > 2 px
- joinCream* / seamCreamUnderChip: fail if > 0
- borderJitter*: fail if > 1 px
- leftTextInsetPt: fail if abs(x - median) > 1.25 pt (median = **6.67** pt)

## Outliers

_None._

## Medians

- leftTextInsetPt median: **6.67** pt
- headerTopAirPt median: **8.00** pt
- bodyTopAirPt median: **9.33** pt

Crops: `audit-mtx-p*.png` in this ticket folder.
