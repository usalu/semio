# Windowed Short Tables — Visual Consistency Audit

PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf` (light)  
Rasters: `audit-win144-*.png` (144dpi), `audit-win288-*.png` (288dpi)  
Crops: `audit-win-<id>-d{144|288}-{seam,edgeL,head}.png`  
Supporting metrics: `audit-agent-windows-metrics.json`  
Generated: 2026-08-05

## Summary: **6 PASS** / **0 FAIL**

Primary targets (Meilensteine p18, Risiken p19, Überblick p77) all **PASS**.  
Also audited short window tables found via `Tabelle:` scan: Erfolgsindikatoren p23, Interviewinformationen p101, Übersicht p111 — all **PASS**.

| Page | Table | Verdict | Chip seam | Chip dL/dR @288 | Edge notches | Insets | Interior verts | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 18 | Meilensteine | **PASS** | 1× ~0.5–0.75pt, creamGap=0 | 0 / 0 | 0 | H≈6.5–7pt L, top air OK | none | flush continuous L |
| 19 | Risiken | **PASS** | 1× ~0.5–0.75pt, creamGap=0 | 0 / 0 | 0 | header↔body matched | none | solid edgeL rule |
| 23 | Erfolgsindikatoren | **PASS** | 1× 0.5pt, creamGap=0 | 0 / 0 | 0 | consistent | none | extra short window |
| 77 | Überblick | **PASS** | 1× ~0.5–0.75pt, creamGap=0 | 0 / 0 | 0 | consistent | none | cream after table = page gap before Marktplätze |
| 101 | Interviewinformationen | **PASS** | 1× 0.5pt, creamGap=0 | 1 / 0 (≤0.25pt) | 0 | body rows aligned | none | extra short window |
| 111 | Übersicht | **PASS** | 1× 0.5pt, creamGap=0 | 0 / 0 | 0 | header↔body aligned | none | extra short window |

## Checks

1. **Chip seam** — single hairline under chips; no double line; no cream gap  
2. **Chip L/R align** — outer chip edges flush with body borders (`dL/dR ≈ 0`)  
3. **Continuous outer edges** — no cream notches at row joins; no x-jitter  
4. **Text insets** — header vs body left pad + top air consistent (chip title pad may differ by design)  
5. **Interior verticals** — these short windows use whitespace columns (no interior verts) — consistent

## Method notes

- Verdicts are **visual-first** (Read on seam/edgeL/head crops at 144 + 288).  
- RGB confirms: chip→body creamGap=0; join notches=0 on outer border; body border luminance stays rule (~128) through row stack.  
- Überblick / short tables show cream **after** the table ends — not a join defect.  
- Long `Tabelle:` pages (Hürden p76, Marktplätze p78+, Glossar) excluded from this windowed-short scope.

## Crops

### p18 Meilensteine — PASS
- `audit-win-meilensteine-d144-seam.png` / `-edgeL.png` / `-head.png`
- `audit-win-meilensteine-d288-seam.png` / `-edgeL.png` / `-head.png`

### p19 Risiken — PASS
- `audit-win-risiken-d144-seam.png` / `-edgeL.png` / `-head.png`
- `audit-win-risiken-d288-seam.png` / `-edgeL.png` / `-head.png`

### p23 Erfolgsindikatoren — PASS
- `audit-win-erfolgsindikatoren-d144-seam.png` / `-edgeL.png` / `-head.png`
- `audit-win-erfolgsindikatoren-d288-seam.png` / `-edgeL.png` / `-head.png`

### p77 Überblick — PASS
- `audit-win-ueberblick-d144-seam.png` / `-edgeL.png` / `-head.png`
- `audit-win-ueberblick-d288-seam.png` / `-edgeL.png` / `-head.png`

### p101 Interviewinformationen — PASS
- `audit-win-interviewinfo-d144-seam.png` / `-edgeL.png` / `-head.png`
- `audit-win-interviewinfo-d288-seam.png` / `-edgeL.png` / `-head.png`

### p111 Übersicht — PASS
- `audit-win-uebersicht-d144-seam.png` / `-edgeL.png` / `-head.png`
- `audit-win-uebersicht-d288-seam.png` / `-edgeL.png` / `-head.png`
