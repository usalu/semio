# Audit: Coverage and Consistency over the Whole Catalog (2026-11-02, by QA5)

Run: `repo tools exec bun -- T/🧰️TS/consistency.ts <out.json>` (what `coverage.py --consistency` runs, with the output redirected into `T/🗑️generated/COV`). One process, whole catalog.

**Totals:** 149 assets, 534 authored representations, 517 ok, 17 not ok in 7 assets (7 of 149 assets, 17 of 534 asset/scale pairs). The other 142 assets pass at every scale. Not fixed here (other agents' assets).

| Asset | Scales failing | Rules and numbers (default pose unless stated) | Likely owner |
|---|---|---|---|
| market-parasol | 1:50, 1:100, 1:200, 1:500 | 1:50: front y [0, 2.960] vs elevation [0, 2.461] (dims 2.46); front x +-1.003 vs plan y +-1.177; plan x [-1.201, 1.294] vs elevation [-1.294, 1.294]; depth 2.005 vs 2.6. 1:500: plan x [0, 1.294] vs [-1.294, 1.294] (plan only the half canopy, widthM 1.294 vs 2.6, front/plan width 0.788 vs 2.6). Spread rules fail at every pose and clip instant (open-still, tilted-to-sun, furled, wind-flap, sun-tilt). | O1 (objects); QA9 reviewing |
| date-palm | 1:50, 1:100, 1:200, 1:500 | 1:50: plan y +-6.784 vs front x +-3.980 (plan width 13.567 vs depthM 7.0); front width 7.961 vs 7.0; plan length 6.831 vs 6.3. Spread rules fail in calm, windy, storm (storm: elevation y 12.571 vs front y 12.024) and every wind-sway, gust and diurnal-transpiration instant. 1:500: elevation width 6.870 and plan length 6.833 vs 6.3. | V1 (vegetation); QA3 reviewing |
| bar-stool | 1:50, 1:100 | pose unoccupied: elevation height 1.620 vs 0.870, front width 1.500 vs 0.420, front height 1.620 vs 0.870, plan width 1.500 vs 0.420 (a part, probably the swivelling seat or backrest, is carried far out of the stool). 1:200 and 1:500 ok. | F (furniture); swivel twist added later, QA7 |
| person-with-umbrella | 1:50, 1:100, 1:200, 1:500 | 1:50: elevation x [-0.225, 0.824] vs plan x [-0.143, 0.824]; front width 0.584 vs depthM 1.0; plan length 0.967 vs 1.05; plan width 0.584 vs 1.0 (the canopy is not reaching the full 1.0 m laterally in front and plan). 1:500 same pattern (0.603 / 0.567 vs 1.0). | H2 (humans) |
| pregnant-person | 1:500 | plan x [-0.356, 0.438] vs elevation [-0.353, 0.362]; plan length 0.793 vs 0.72 (pose hand-on-back). | humans group H3 (no report; TS handed it to H2/H3) |
| running-child | 1:500 | plan x [-0.148, 0.184] vs elevation [-0.152, 0.120]; plan length 0.332 vs 0.281 (pose standing-ready). | humans group H3 (as above) |
| seated-reader | 1:500 | front y [0, 1.296] vs elevation [0, 1.342]; front height 1.296 vs 1.34 (pose seated-upright). | humans group H3 (as above) |

## Notes
- Three of the humans (pregnant-person, running-child, seated-reader) fail only at 1:500, by 0.05 to 0.07 m: the plan or front 1:500 silhouette lacks or overshoots a small part (hand, foot, hair) relative to the elevation.
- market-parasol, date-palm and person-with-umbrella fail because the canopy or fronds are not carried consistently in front and plan: author them as extruded or revolved solids with the elevation profile (rule 4 in the TS broadcast).
- Raw output was `T/🗑️generated/COV/consistency.json` and `out.txt`; both deleted.
