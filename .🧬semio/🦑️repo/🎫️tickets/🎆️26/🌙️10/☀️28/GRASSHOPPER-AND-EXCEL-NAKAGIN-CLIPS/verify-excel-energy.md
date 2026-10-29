# Verification: excel / energy (`Beat4_Energy`)

Final verification after the fix round, per `verify-brief.md`. Audit: `audit-excel-energy.md`. Fresh frames (1280x720, 23 keys each, captured this session with `PYTHONUTF8=1`):

- EN: `🗑️generated/frames/📊️excel/Beat4_Energy/en/` (layout: `en/layout.md`, 112 strict findings in 19 of 23 frames)
- DE: `🗑️generated/frames/📊️excel/Beat4_Energy/de/` (layout: `de/layout.md`, 130 strict findings in 21 of 23 frames)

Nothing edited except this report. The 🏢️nakagin model was used as the reference for numbers; all on-screen values checked (30.92 W/K, 3.12 W/K, 681 W, 1,296 kWh/a, 148 kWh/(m²·a), 182,901 kWh/a, what-if 1.10 W/(m²·K): 173,115 kWh/a, 39 dependents) are consistent with each other and with the audit's recomputation.

## Verdict

**NOT READY.** The physics and the blocker are fixed, and most majors are fixed. Still open: the status-bar sums, one new text overlap on the `frozen` frame, the clipped `3.75 W/K` label, and bracket glyphs in stage text.

## Blocker

| item | status | evidence |
|---|---|---|
| ht: SUMPRODUCT narration contradicts the card | FIXED | en/ht and de/ht: card now `SUMPRODUCT(D3:D9,E3:E9,G3:G9) = 30.92 W/K` with the temperature factor G; caption "SUMPRODUCT does it in one formula ... back wall included" (DE "SUMMENPRODUKT ... die Rückwand eingeschlossen"). |

## Majors (13 in audit)

| item | status | evidence |
|---|---|---|
| climate: B8 shows `###` | FIXED | en/climate, de/climate: B8 reads `0.34 Wh/(m³·K)`. (Stage-panel brackets still open, see below.) |
| volume: names used before climate introduces them | FIXED | Order is now ht, climate, mistake, fix, volume; climate values readable. |
| trace EN: chain-panel labels collide with values | FIXED | en/trace: labels on their own line above values. |
| trace DE: "Gebäudesumme" collides | FIXED | de/trace: label and value separated. |
| whatif: formula bar stale at 2.33 | FIXED | en/whatif shows `1.1`; de/whatif shows `1,1`. |
| frozen: formula bar stale, D8 not selected | FIXED | en/frozen, de/frozen: D8 selected, formula bar `182900.7639`. |
| repaired: formula never shown going back | FIXED | en/repaired, de/repaired: D8 selected, `=SUM(D4:D7)` shown, value returns to 173,115. |
| compare: selection D4:D7 vs plotted per-area values | FIXED | en/compare, de/compare: selection is F4:F7. |
| compare: x-axis labels collide | FIXED | en/compare, de/compare: axis reads A B C D. |
| fix: Climate!B6 never shown | FIXED | en/fix, de/fix: Climate sheet, B6 highlighted at 0.5. |
| mistake: Climate!B6 never shown | FIXED | en/mistake, de/mistake: Climate sheet, B6 = 1.00, ΔH_T = +1.88 W/K. (The `3.75 W/K` label is now clipped, see new items.) |
| hook: no "teaching model" statement | FIXED | en/hook, de/hook: "Teaching model · simplified, rounded" / "Lehrmodell · vereinfacht, gerundet" present. |
| load: arrows (619 W) do not reach Φ = 681 W | FIXED | en/load, de/load: "air 62 W" / "Luft 62 W" arrow and label added; 619 + 62 = 681. |

## Minors (carried from audit)

| item | status | evidence |
|---|---|---|
| status-bar sums unformatted or mixing units | NOT FIXED | Still `Sum: 30.92126861` (ht), `8.725` (exposed), `34.04144861` (total), `1443.73613` (annual), `598.0573747` (compare), `188267.7853` (energy), and `5.83` (uvalues, U-values added). DE has the same numbers with decimal commas. |
| ua: arrow length and width not proportional | FIXED (wording) | Caption now "the larger the U·A ..., the longer and thicker its arrow". Geometry unchanged, which is acceptable for a qualitative claim. |
| volume: air arrows read as double-headed | FIXED | Single arrow; caption no longer claims in/out. |
| volume, load, annual: formula overlays cover the stage | FIXED | Overlays now stay inside the Excel window (both languages). |
| mistake: `3.75 W/K` label touches window | NOT FIXED (worse) | Now partly hidden behind the Excel window, reads as `75 W/K`. See new items. |
| compare: unit label touches window edge | FIXED | Chart moved right. |
| trace: dashed sheet arrow over the Type A count; blue arrow across column C | NOT FIXED | en/trace, de/trace: arrow still lies over "36" and through the per-capsule values. Cosmetic. |
| whatif: "1.46 W/K" label touches capsule arrow | NOT FIXED | en/whatif, de/whatif. Cosmetic. |
| shape: label count and collision | FIXED | "5 faces + window" / "5 Flächen + Fenster"; labels stacked, no collision. Hatch covers the roof only while the label says "roof, floor shared" (minor). |
| uvalues: tint has no legend | FIXED | "tint: low U green, high U red" / "Tönung: kleines U grün, großes U rot"; chips now "window: benchmark" / "Fenster: Richtwert". |
| recap: legend missing b, n, capsule count | FIXED | Rows for b, n, × (capsule count) and SUMPRODUCT present in both languages. |
| repaired: stage almost empty | NOT FIXED | Only the chain panel and a "1.46 W/K" label remain. |

## Fact findings

- HDD 1,586 K·d: unchanged. The Climate panel now names its source ("JMA normals 1991-2020, via the Wikipedia Tokyo climate table"), which is a partial step. The twelve monthly means were not checked against JMA in this pass. Still open per the audit's action item.
- Window chip now "benchmark" / "Richtwert": fixed.
- Type D explanation: not re-checked in this pass.

## New items (not in the audit)

| sev | frame | lang | defect | fix |
|---|---|---|---|---|
| M | frozen | en, de | Red caption "rows add up to 173,115 kWh/a" (DE: "Zeilen ergeben 173,115 kWh/a") overlaps the Building total value 182,901 kWh/a in the right panel. Text collision. | Move the red caption below the panel or above the value; keep it clear of the panel's last row. |
| M | mistake | en, de | `3.75 W/K` label (and the left `6.00 W/K` label) is partly hidden behind the Excel window; reads as `75 W/K`. Wrong value readable. | Shift the arrow labels left of the window edge or move the window; re-check both languages. |
| m | volume | en, de | "air 3.12 W/K" / "Luft 3,12 W/K" sits on the blue arrow and crowds the "3.09 W/K" label. | Offset the air label off the arrow. |

## Bracket glyphs in stage text (cross-cutting)

The audit's "brackets `( )` rendered as `[ ]`" item is fixed in cells and formula bar, but stage/panel text still shows brackets in both languages:

- whatif, trace: "Window U-value 1.10 W/[m²·K]" / "Fenster U-Wert 1,10 W/[m²·K]"
- shape: "148 kWh/[m²·a]" / "kWh/[m²·a]"
- climate panel: "0.34 Wh/[m³·K]" / "Wh/[m³·K]"
- recap legend: "W/[m²·K]", "kWh/[m²·a]"

Status: PARTLY. Treat as a major: replace with parentheses everywhere in stage text and recapture.

## Must change before READY

1. Format the Excel status-bar sums to the cell format, or hide them (all frames listed above, both languages).
2. Fix the `frozen` caption overlap with the Building total value (en, de).
3. Keep the `mistake` labels clear of the Excel window (en, de).
4. Replace `[ ]` with `( )` in all stage and panel text (whatif, trace, shape, climate, recap; en and de), then recapture.
5. Verify the twelve monthly means against JMA normals 1991-2020 and state the source and period on the Climate sheet.

Optional before publish (minor): trace arrows over "36" and column C; whatif label touching the arrow; repaired stage nearly empty; volume air label overlap.
