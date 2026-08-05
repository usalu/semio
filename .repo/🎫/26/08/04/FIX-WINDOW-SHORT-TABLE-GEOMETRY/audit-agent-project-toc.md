# Visual Audit — Project Tables + TOC/Register

**Verdict: PASS**  
PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`  
Ticket: `26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY`  
Crops: `audit-ptoc-*.png`

## Scope

| Area | Pages | Role |
|------|-------|------|
| Inhaltsverzeichnis (register) | p2–p4 | Main TOC table |
| Project catalogue | p24 (P.K.1 Kopfbau), p25 (P.K.2 Upcycle), p39 (P.K.16 Recyclinghaus) | Photo band + meta + data rows |
| p125 | bibliography hit for “Kopfbau” | **Not** a project card |
| Data reference | p18 Meilensteine | Half-pad delta note only |

## Checks

### 1) TOC — internal consistency

| Check | Result | Evidence |
|-------|--------|----------|
| All TOC rows same pad / inset | **PASS** | p2/p3/p4 body+Ljoin: left inset **7.33pt**, top inset **5.33pt**, spread **0** |
| Chip / header alignment | **PASS** | `*-chip.png`, `*-head.png`: Inhaltsverzeichnis pillar continuous with body L; page header chips track table L/R |
| Continuous L/R | **PASS** | Ljoin + Rjoin crops; no broken pillars |
| Join notches | **PASS** | `pageInsideJoins=0` on all TOC crops; visual T-joins clean |

### 2) Project catalogue bands

| Check | Kopfbau p24 | Upcycle p25 | Recyclinghaus p39 |
|-------|-------------|-------------|-------------------|
| Photo top pad ≈ cell pad (~5.5pt) | **5.50pt** PASS | ~5.5pt visual PASS | **5.83pt** PASS |
| Chip aligned to body borders | PASS | PASS | PASS |
| Continuous sides | PASS | PASS | PASS |
| No cream gap under chips | PASS (0pt) | PASS (0pt) | PASS (0pt) |
| Photo not flush to top rule | PASS | PASS | PASS |

Representative crops: `audit-ptoc-project-kopfbau-photo.png`, `…-chip.png`, `…-Ljoin.png` (same set for upcycle / recycling).

Catalogue continues through later P.K. pages with the same overview-band chrome; no second distinct project-card geometry found on p125.

### 3) TOC vs data insets (expected half-pad)

| Sample | Left inset | Top inset |
|--------|------------|-----------|
| TOC p2–p4 | 7.33pt | **5.33pt** |
| Data Meilensteine p18 | 6.67pt | **8.00pt** |
| Project photo pad (Kopfbau) | — | **5.50pt** |

Register/TOC intentionally uses half vertical padding (`\semio@table@vtop@toc` / `@vbot@toc` in `semio-table.sty`). Denser TOC rows vs full data-table air is **by design**, not a bug.

## Issues

None.

## Crops written

- Full pages: `audit-ptoc-page-002.png` … `004.png`, `024.png`, `125.png`
- TOC: `audit-ptoc-toc-p{2,3,4}-{chip,body,Ljoin,Rjoin,head}.png`
- Project: `audit-ptoc-project-{kopfbau,upcycle,recycling}-{chip,photo,body,Ljoin,Rjoin,head}.png`
- Data ref: `audit-ptoc-data-meilensteine-*.png`
