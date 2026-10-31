# Final morph review: excel / energy (`Beat4_Energy`)

Scene: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py` (808 lines, read in full). Judged from code intent plus 640x360 mid frames of `🗑️generated/morph/📊️excel/Beat4_Energy/en` (read: sheet is not among them; read ua, areas, ht, degreedays, energy, compare-03/04, shape-04, whatif-04, recap-04).

Caveat on frames: they are older than the current model. They show counts 35/33/35/37 and 1,296 kWh/a per type, building total 181,371; the engine now gives 36/35/33/36, 1,295.5 kWh/a per type and the same 181,371 total (the whatif frame shows 177,022). Re-capture `energy`, `compare`, `whatif`, `recap` before sign-off. The code reads all numbers from the engine, so the video will be right once re-rendered.

## Narration vs model (checked with the engine)

All four types are now equal: H_T 30.92 W/K, H_V 3.12 W/K, demand 1,295.5 kWh/a, 148.23 kWh/(m²·a); all share window diameter 1.3 m, shell 2.5 x 4 x 2.5 m. Only the capsule counts differ (36/35/33/36 = 140), so type totals differ (46,638 / 45,343 / 42,752 / 46,638 kWh/a).

- `compare` (en and de): already says equal envelope, equal heat loss. Matches. No stale "only C is higher / type C stands higher" text remains in the narration; the storyboard line "only C stands higher" (morph-excel-energy.md M12) is stale but is not in the scene.
- `types_row` footer "same nominal shell and porthole · same demand": matches.
- Dead code: `_types_row` colours the window label WARN `if t.window_diameter > 1.4`, never true now; harmless but remove it (constant INK).
- `uvalues` "Glass leaks most": true for U (2.33 vs 0.5-0.6 W/m²K); true as written.
- `shape`: 5.3 vs 3.0 m²/m² and 148 vs 100 kWh/(m²·a) come from the engine (`variant shared_roof_floor`); matches "would cut the demand".
- `energy`, `annual`, `recap`: no per-type claim; numbers live. No text claims type differences anywhere else. German `compare` is consistent with English.

## Per clause (24)

| Clause | Rating | Why |
| --- | --- | --- |
| hook | ACCEPTABLE | Tower with lit type A, "?" badge, theta labels; setup, the identity link starts in `sheet`. |
| sheet | GOOD | Lit tower capsule `morph` into the big capsule plus a part-wise `capsule_pairs` copy into the tab chip: object becomes sheet. |
| dims | GOOD | Three real dimension lines `scatter` into B2:B4, ghosts left behind (`ghost=0.35`); C to A, bound pair persists. |
| areas | GOOD | Faces (hidden ones as tinted squares) `unfold` into the D3:D9 chips, tint after landing (`after=`); frame in the face colour. Frame areas-04 shows chips sitting on D cells. |
| exposed | ACCEPTABLE | Back view, bolts, hatch, b readout; state change, no token travels (b token only appears in `mistake`). |
| uvalues | ACCEPTABLE | Heat tint by U is a continuous encoding, window ring; E column does not fly to the faces, the tint stands for it. |
| ua | GOOD | Seven F-cell chips `scatter` into the seven arrow shafts (A to C), lengths/widths from U·A, hidden faces on dashed leads; ua-03 shows legend while chips fly. |
| ht | GOOD | Arrows `gather` into the H_T token, SUM and SUMPRODUCT card; ht-03 shows arrows and the sum still agreeing (30.92). |
| climate | WEAK | Window swap, names list fade in and capsule fade out; pure fade-swap, but no idea is explained by it (inventory of names). Tolerable. |
| degreedays | GOOD | Twelve hatched shortfalls `gather` into one Σ token; "an area" is literally the hatched area. |
| mistake | GOOD | b cell token `morph` to an item token on the back wall, arrow grows, ΔH_T live; A to C and the arrow carries the effect. |
| fix | GOOD | The same b token morphs back to B6, state returns; a true reverse morph. |
| volume | GOOD | Translucent interior box faces `gather` into the V cell token, then the blue H_V arrow grows; C to A to C. |
| total | GOOD | H_T and H_V tokens `morph` into the stacked-bar segments in their colours. |
| load | ACCEPTABLE | Nine opaque slices `scatter` to eight arrows plus the Φ chip; slices are generic rectangles, but colours and count carry over. |
| annual | ACCEPTABLE | Floor plan morphs into the floor-area tile and the parked Σ token into the "degree days" tile (real chain); the product tiles themselves fade in rather than assemble from H and Σ. |
| energy | GOOD | 140 type-coloured dots from the tower `scatter` to the four count chips, and the tower is tinted by type; C to A with the quantity visible. |
| compare | ACCEPTABLE | Four capsule copies `scatter` (part-wise) into value item tokens, chart fades in under them (compare-04 shows the four equal 148 bars). Honest for equal types, but the chart is not a morph target of the capsules or tokens. |
| shape | GOOD | Faces of both capsules `unfold` into area-proportional strips (net rows); the shared-roof net is visibly shorter (shape-04: 5.3 vs 3.0). |
| trace | ACCEPTABLE | Dependency list with fade-in and an arrow, a count of dependents; static, no cell-to-cell travel. |
| whatif | GOOD | Seeds from the U cell `scatter` onto the 140 capsule windows, dots go red to green with the live change and the counter; A to C. |
| frozen | ACCEPTABLE | State change of D8 with a dashed stale frame and "rows add up to" readout; no event morph (typing a constant). Fine for a short clause. |
| repaired | ACCEPTABLE | Reverse of frozen by live state blend and fade, same as above. |
| recap | ACCEPTABLE | The 140 dots land on the legend tower (identity returns to the building); the legend symbols fade in as text and do not themselves travel (recap-04). |

## Verdict: SHIP

The video is genuinely morph-driven where ideas are explained: faces to areas, areas to U·A arrows, arrows to H_T, shortfalls to Σ, volume to V to H_V, dots to counts to what-if, with ghosts, reverse morphs (fix, whatif, recap) and live values. Remaining weaknesses are polish. Narration fits the model: the types are equal and `compare` says so; no clause claims a type difference.

Optional polish (none blocking):

1. Re-capture `energy`, `compare`, `whatif`, `recap` frames (stale counts) and re-check at 1280x720.
2. `compare`: morph the four value tokens into the bar tops (or fade the chart in from them) instead of a fade; line ~574-575.
3. `annual`: build the H, degree-day and 24 h tiles from existing objects (H_T/H_V tokens, Σ) rather than fading `units` in; line ~552.
4. `recap`: let the legend symbols travel from where they first appeared (`H_T`, `U·A`, `b`) instead of fading in; line ~805.
5. Remove the dead `WARN if t.window_diameter > 1.4` branch; line 687.

Summary: 1) SHIP. 2) Narration is consistent with the equal-type model; no text change needed. 3) Frames predate the latest model counts, re-capture four keys. 4) Weakest clauses are `climate` (fade only) and `trace`/`frozen`/`repaired` (state only). 5) Five optional polishes above.
