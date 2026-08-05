# Long Data Tables — Final Visual Consistency Audit

PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`  
Method: visual crops at **144 / 288 / 432** dpi + RGB (`@napi-rs/canvas`) + PDF text-X insets  
Artifacts: `audit-agent-long.json`, crops `audit-long-*.png`  
Script: `audit-agent-long.ts` (temps in this ticket folder)

## Overall

**PASS** — no inconsistent long tables.

| Table | Pages | Role | Verdict |
|-------|-------|------|---------|
| Hürden (`H.a`) | 76 | single | **PASS** |
| Marktplätze · Zugang (`BB.M.a`) | 78 | single | **PASS** |
| Marktplätze · Datenfelder (`BB.M.b`) | 79–81 | first / cont / last | **PASS** |
| Depot-Shops · Datenfelder (`BB.D.b`) | 83–84 | first / last | **PASS** |
| Vermittlungsplattformen · Datenfelder (`BB.V.b`) | 85 | single | **PASS** |
| Glossar | 121 | single | **PASS** |
| Abkürzungen | — | absent | **N/A** |

Inconsistent tables: **none**.

## Coverage notes

- **Hürden** does not continue past p76.
- **Marktplätze · Zugang** is single-page on p78. Pages 79–81 are the sibling table **BB.M.b** (Datenfelder), not a Zugang continuation — audited as Datenfelder multi-page chrome.
- **Abkürzungen**: no table (or Abkürzungsverzeichnis) in the 128-page PDF.

## Checks (all zooms)

| # | Check | Result |
|---|-------|--------|
| 1 | Chip/title chrome seam → body | **PASS** — single hairline (~0.5–0.67pt), cream gap **0** |
| 2 | Chip L/R vs body outer border | **PASS** — L flush (Δ≤0.3pt). Title chip is tab-width (R tab edge intentional). Right **Tabelle:** chip flush with body R on Datenfelder pages |
| 3 | Continuous L/R outer edges at row joins | **PASS** — cream joins **0**, notch **0**, jitter **0** at 288/432 (144dpi ≤1.5pt AA only) |
| 4 | Mid-rule meets inner face of sides | **PASS** — clean T-junctions on `joinL`/`joinR`/`midmeet` crops; no double outer edge |
| 5 | Text insets header/body/rows | **PASS** — PDF text X identical for header key vs body IDs; left inset **~5.7–6.2pt** |
| 6 | Multi-page continuation chrome | **PASS** — p79–81 / p83–84 retitle chips + table-id chips, seam weld intact |

## Per-table metrics (primary 288 dpi)

| Table | Page | hair | cream | chipLΔ | cream joins | jitter L/R | left inset | top air | pdfX Δ |
|-------|------|------|-------|--------|-------------|------------|------------|---------|--------|
| Hürden | 76 | 0.5 | 0 | 0 | 0/0 | 0 / 0.75 | 5.92 | 8.0 | 0 |
| Markt Zugang | 78 | 0.5 | 0 | 0 | 0/0 | 0 / 0 | 5.92 | **11.5** | 0 |
| Datenfelder Markt | 79 | 0.5 | 0 | 0 | 0/0 | 0 / 0 | 5.99 | 8.0 | 0 |
| Datenfelder Markt | 80 | 0.5 | 0 | 0 | 0/0 | 0 / 0 | 6.17 | 8.0 | 0 |
| Datenfelder Markt | 81 | 0.5 | 0 | 0 | 0/0 | 0 / 0 | 5.74 | 8.0 | 0 |
| Datenfelder Depot | 83 | 0.5 | 0 | 0 | 0/0 | 0 / 0 | 5.99 | 8.0 | 0 |
| Datenfelder Depot | 84 | 0.5 | 0 | 0 | 0/0 | 0 / 0 | 5.92 | 8.0 | 0 |
| Datenfelder Vermitt | 85 | 0.5 | 0 | 0 | 0/0 | 0 / 0.75 | 5.74 | 8.0 | 0 |
| Glossar | 121 | 0.5 | 0 | 0 | 0/0 | 0 / 0.75 | 5.99 | 8.0 | 0 |

## Notable (not failures)

- **Marktplätze · Zugang top air 11.5pt** vs peers **~8pt**: wrapped multi-line header row vertically centers short labels (`ID`); left inset and seam/joins match peers. Not a cream-gap or double-baseline defect.
- Body copy is medium-gray; header/body alignment verified with PDF text X (not luminance-only ink hunt).

## Evidence crops (sample)

- Seam: `audit-long-huerden-p76-d288-seam.png`, `audit-long-markt-zugang-p78-d288-seam.png`, `audit-long-datenfelder-markt-p79-d288-seam.png`, `audit-long-glossar-p121-d288-seam.png`
- Joins: `audit-long-*-d288-joinL.png`, `audit-long-*-d432-joinR.png`
- Continuation heads: `audit-long-datenfelder-markt-p80-d288-head.png`, `audit-long-datenfelder-markt-p81-d288-head.png`, `audit-long-datenfelder-depot-p84-d288-head.png`
- Mid-meet: `audit-long-huerden-p76-d288-midmeet.png`, `audit-long-glossar-p121-d288-midmeet.png`
