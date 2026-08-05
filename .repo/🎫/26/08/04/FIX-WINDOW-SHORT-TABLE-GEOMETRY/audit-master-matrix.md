# Master table consistency matrix

Scale: 216dpi. Pass: 9/12. Median left inset: 6.67pt. Median header top air: 4pt.

| Table | Kind | dL | dR | cream L/R | jit L/R | hdr air | body air | inset | photo | PASS |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| TOC | toc | 0 | 0 | 0/0 | 0/0 | — | — | — | — | FAIL |
| Meilensteine | window | 0 | 0 | 0/0 | 0/1 | 4 | 8.33 | 6.67 | — | PASS |
| Risiken | window | 0 | 0 | 0/0 | 0/1 | 4 | 8.33 | 6.67 | — | PASS |
| Akteure (p23) | window | 0 | 0 | 0/0 | 0/1 | 8.33 | — | 6.33 | — | PASS |
| Kopfbau | project | 0 | 0 | 0/0 | 0/1 | 6 | 12 | 20 | — | FAIL |
| Huerden | long | 0 | 0 | 0/0 | 0/1 | 4 | 8.33 | 6.67 | — | PASS |
| Ueberblick | window | 0 | 0 | 0/0 | 2/2 | 8 | — | 6.33 | — | FAIL |
| Marktplaetze | long | 0 | 0 | 0/0 | 0/1 | 3.67 | 11.67 | 6.67 | — | PASS |
| Datenfelder p79 | long | 0 | 0 | 0/0 | 0/1 | 3.67 | 11.67 | 6.67 | — | PASS |
| Datenfelder p83 | long | 0 | 0 | 0/0 | 0/1 | 3.67 | 11.67 | 6.67 | — | PASS |
| Datenfelder p85 | long | 0 | 0 | 0/0 | 0/1 | 4 | 12 | 8 | — | PASS |
| Glossar | long | 0 | 0 | 0/0 | 0/1 | 8 | — | 6.33 | — | PASS |

## Failures / notes
- **TOC** (p3): seamCream=3
- **Kopfbau** (p24): seamCream=3; insetOutlier 20 vs med 6.67
- **Ueberblick** (p77): jitterL=2; jitterR=2

TOC may differ in vertical air (half padding) — not counted as inset outlier.