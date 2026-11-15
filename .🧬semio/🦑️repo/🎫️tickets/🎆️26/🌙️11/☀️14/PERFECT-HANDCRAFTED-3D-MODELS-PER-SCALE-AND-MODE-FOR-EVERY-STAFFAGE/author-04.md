# Author 04 report (machinery) — INCOMPLETE, stated plainly

Only 1 of 8 assets is finished; tower-crane is half done; six are not started. I ran out of working budget; the 20-subagent limit was already used by the other authors, so I could not delegate.

## Status per asset
| asset | state |
|---|---|
| machinery/scroll-compressor | modelled at 1:50/1:100/1:200 (1:500 unavailable), passed every rule of the old tool, rendered and looked at (50 and 100/200 sheets), SPLICED. Needs re-check and re-splice under C1-C6: add `anchor: "base-contact"` to the support solid (the generator `author-04-scroll-compressor.py` already carries it; re-run it, check, splice) and re-verify areas as exposed surface. |
| machinery/tower-crane | generator `author-04-tower-crane.py` writes all four scales (body, thermal, structure, decisions en/de) to `volumes/machinery/tower-crane.json`; NOT spliced, does not pass. |
| oil-pumpjack, plate-heat-exchanger, scissor-lift, solar-thermal-collector, sorption-wheel, wheel-loader | not started (only `author-04-inspect.ts` output exists in `🗑️generated/author-04/`). |

## scroll-compressor (spliced)
Solids body/thermal/structure: 1:50 16/9/10, 1:100 8/8/8, 1:200 6/8/8. Check line: PASS (old tool, before C1-C6). Hand modelled: shoulders on the shell, Bezier dome revolve, flanged discharge, collared suction, lidded terminal box with a gland that completes the 300 mm width, sight window (ring, scroll plate, 8 mm spiral wrap loft, hub, glass); thermal references plus plume ring emitter; structure thin-walled shell and dome members, joints, masses, contact pad support. Coarser scales are redrawn (plain cylinder, merged window, hidden scroll disc at 1:200). `areaM2` corrected at the root (old, new): shell 0.1 to 0.252, dome 0.05 to 0.0886, terminal 0.008 to 0.0138, feet 0.02 to 0.182, suction 0.005 to 0.0057. Renders looked right; the glass is opaque in the viewer so the spiral is hidden behind it.

## tower-crane (not passing) — what was learnt
- 1:50: every 2D member shape (322) becomes a real bar, grouped into repeat families straight from the asset (`members()`/`lattice_bars()`); IoU elevation/front/plan 0.972/0.962/0.963 (renders match closely: tower head, cab, jib, counter-jib overlay green). Left: Hausdorff 0.27/0.22/0.036 m against the limit 0.015 m (a 72 m object with lattice members cannot meet an absolute 15 mm; the elevation miss is the red `tower-head-ties` drawn zigzag, front 0.22 near the tower head), structure outside-body 0.07 m (pendant/winch vent), bounds ok after trimming diagonals.
- 1:100 / 1:200: IoU 0.47-0.55 per view. Cause is in the 2D, as the previous author found: mast, jib and counter-jib are drawn as open lines or self-crossing thin polygons, so the filled truth does not contain the lattice while any real 3D truss does. Proposed in `author-requests.md` terms: draw the lattice at these scales as closed `member` shapes like 1:50, or exempt open-stroke lattice drawings from the silhouette rule. Not done.
- 1:500: IoU 0.947/0.957/0.9996 (prisms match the closed envelopes), Hausdorff 1.0 m, `tower-head` and grille below 75 mm (grille is 40 mm, pyramid apex 36 mm: need merging), cross-scale to 1:200 fails by construction (prism vs lattice: volume x5), lattice thermal area 3.3x.
- Thermal/structure are derived mechanically from the body in the generator (`derive()`); thermal `areaM2` for all regions still needs the C1 exposed-surface measurement and the root correction listed old/new; `volumes-copied` flags the derived lists as copies of the finer scale and needs hand-differentiated lists.
- Bug found and fixed on the way: `lib.ext` plan profiles take world coordinates (skip body was misplaced by 3 m).

## Tools and files
- `author-04-lib.py` (per-scale Volume builder, bone-frame conversion), `author-04-inspect.ts`, `author-04-area.ts`, `author-04-areafix.py` (area table and root correction), `author-04-run.sh`, generators `author-04-scroll-compressor.py`, `author-04-tower-crane.py`. `author-04-tower-crane.py` contains a few dead `if False` lines to remove.
- The shared check tool was mid-update (missing export `outerArea`) when I stopped, so nothing was re-checked under C1-C6.
- Requests: appended `## 04 machinery` to `author-requests.md` (areaM2 convention; now answered by C1).
