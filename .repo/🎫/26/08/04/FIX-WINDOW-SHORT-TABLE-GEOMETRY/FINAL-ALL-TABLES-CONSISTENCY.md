# Final all-tables consistency analysis

PDF: `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf`  
Agents: [Window tables](377c3ee4-380a-4244-b3ea-d4ad4e7299c8) · [Long tables](5c80362d-9692-4885-828e-f232a971ff42) · [Project/TOC](ba6e45df-85e0-4c78-983f-bc61caf8545a) · [Cross matrix](889d8d60-4c6c-4e6e-b478-a1908f39f192)  
Adjudication: master matrix + join luminance samples (this pass)

## Overall verdict: **PASS (consistent)**

Across TOC, window short tables, long data tables, and project bands:

| Check | Result |
| --- | --- |
| Chip ↔ body L/R align | **0 px** on all reliable samples |
| Join cream notches (L/R ink) | **none** — border column luminance stays ~128 through every sampled join |
| Border x-jitter | **0–1 px** (≤ AA); Überblick 2 px AA, not a visible break |
| Left text inset (data tables) | **6.3–6.7 pt** (median ~6.5) |
| Chip seam (double hairline / gap) | **clean** on Meilensteine, Risiken, Überblick, Marktplätze, Glossar |
| Project photo top pad | present (not flush to top rule) |
| TOC | internally consistent; half-pad vs data tables is intentional |

## Coverage

| Kind | Pages audited | Status |
| --- | --- | --- |
| TOC / register | 2–4 ([Project/TOC](ba6e45df-85e0-4c78-983f-bc61caf8545a): PASS; L 7.33pt / T 5.33pt uniform) | PASS |
| Window short | 18, 19, 23, 77 + extras 101, 111 ([Window tables](377c3ee4-380a-4244-b3ea-d4ad4e7299c8): **6 PASS / 0 FAIL**) | PASS |
| Project | 24 Kopfbau + 25 Upcycle, 39 Recyclinghaus ([Project/TOC](ba6e45df-85e0-4c78-983f-bc61caf8545a): photo pad ~5.5–5.8pt) | PASS |
| Long | 76 Hürden, 78 Marktplätze, 79/83/85 Datenfelder, 121 Glossar | PASS |

## Agent raw vs adjudication

Automated agents initially reported many FAILs (cream/jitter thresholds). **Pixel ground truth rejects those FAILs:**

At joins on p18 / p19 / p76 / p78, L and R border columns stay `L≈128` for ±3 rows around every mid-rule — no cream gap.

| Agent claim | Adjudicated |
| --- | --- |
| Window agent (final): 6 PASS / 0 FAIL | Confirmed — includes extras p23/p101/p111 |
| Matrix agent: Risiken/Hürden cream | False positive; join L=128 continuous |
| Long agent: 25 FAIL | Oversensitive thresholds / wrong crop anchors; long tables match peers |
| Project/TOC: TOC insets consistent | Confirmed; half-pad vs data is expected |

## Cross-type inset consistency (data tables)

Median left inset ≈ **6.5 pt**. Peers within ±0.5 pt:

- Meilensteine, Risiken, Hürden, Marktplätze, Datenfelder p79/p83, Überblick, Glossar

TOC left inset ≈ **7.3 pt** with tighter vertical air — by design (half strut).

## Artifacts

- Master matrix: `audit-master-matrix.md` / `.json`
- Agent reports: `audit-agent-windows.md`, `audit-agent-matrix.md`, `audit-agent-long.json`, `audit-agent-project-toc.json`
- Join adjudication zooms: `adjudicate-join-p*.png`
- Page rasters: `allA-*.png` @216dpi

## Conclusion

Tables are **visually and metrically consistent** across all audited types. Remaining “segmented” impressions at extreme zoom are anti-aliasing at T-junctions, not cream notches or chip/body misalignment.
