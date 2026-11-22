# objects/shopping-trolley — running author report

## Pass 1 (second round), 2026-11-22

Earlier work: round 3 (`r3-objects-shopping-trolley.md`, `r3-shopping-trolley-*.py`) built the whole model by hand; rework 15 (`rework/15.md`, section 1) added the castor and wheel bones. Source and catalog were identical at the start of this pass (no interrupted edit).

### Reference (Wanzl EL / ELX 101 class, 100 l wire basket trolley; own knowledge)

| quantity | real | model (measured) | verdict |
|---|---|---|---|
| L x W x H | 1.00-1.04 x 0.59-0.60 x 0.98-1.01 m | 1.05 x 0.57 x 1.00 m | in class |
| mass | 13-16 kg | 14 kg | ok |
| castors | 4 swivel castors, wheel o125 x 32, trail 30-40 mm, build height 150-160 mm | o125 x 32, trail 35, 155 mm; bones kingpin (continuous) + axle | ok |
| chassis | o25 tube, rails converging to the front for nesting, open at the rear, front cross tube, uprights leaning about 9 degrees | 545 -> 390 mm, rear open, 9.3 degrees | ok |
| basket | wire o4-5, rim o8, about 100 l, floor rising to the front, front wall raked | 113 l, floor +25 mm, rim 0.915 x 0.51/0.43 m, front wall raked 41 degrees from the vertical | rake not verified, see open points |
| basket fixing | rim and floor fixed to the uprights, front carried by struts | 1:50-1:200 brackets and struts; 1:500 nothing at the rear | defect at 1:500 |
| gate | hinged on the rear rim bar, swings up and in | bone `gate`, 0-80 degrees | ok |
| handle | o25 tube, o35 grip at about 0.98 m, coin lock | as real | ok |
| crate tray | wire shelf with front and rear lip about 40 mm | 1:50/1:100 lips on posts; 1:200 lips without posts | defect at 1:200 |

### Defects found (sheets of all four scales, three lists, poses, 2D, full validation)

- 1:500 structure: the basket rim was a loose group, 82 mm from the frame (open load path); the body had no rear fixing either.
- 1:500 2D: the front struts of the model were not drawn in elevation and front; the plan rim stroke (65 mm wide at 1:500) was centred on the rim and drew the basket 32 mm too wide all round (plan IoU 0.879).
- 1:500 body: castor housing 85 mm wide over a 75 mm wheel (drawn 75 mm).
- 1:200 body: both tray lips floated 34 mm above the tray (posts missing).
- 1:100 body: four 8 mm fork crowns below the 15 mm feature, hidden inside the 21 mm swivel block.
- 1:500 decisions text still said the struts were dropped.

### Rebuilt (`rework-shopping-trolley-pass1.py`, `rework-shopping-trolley-2d.py`)

- 1:500 body/thermal: `bracket-rim-l|r`, `bracket-floor-l|r` (o8 fixing wires from the uprights to rim and floor edge, long enough to be drawn at this scale), castor housings 75 mm wide. Structure: the same fixings as members plus the basket cage (`st-floor-edge-*`, `st-corner-rear-*`, `st-corner-front-*`), so goods -> floor -> corners/rim -> uprights and struts -> rails -> castors -> ground is one connected path. Decisions rewritten (en, de).
- 1:500 2D: `front-strut` (elevation), `front-strut-l|r` (front); plan `rim` stroke laid half a pen inside the basket edge. Silhouette IoU now 0.966 / 0.947 / 0.955 (elevation / front / plan).
- 1:200 body: `tray-lip-panel-front|rear`, a 40 mm wire panel under each lip wire (posts o7.5 x 40 cannot exist at a 30 mm feature); decision text updated.
- 1:100: fork crowns removed from body, thermal and structure (merged into the swivel block); decision text updated.
- Tried and withdrawn: a true 9 degree upright with a separate 80 mm handle arm at 1:500 (a part that short can be neither slender nor 75 mm thick); the upright stays one rod to the handle bar.

### Validation

`VALID objects/shopping-trolley (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`; published with `volumes-insert.py`; `--catalog-only` VALID.

### Rendered and read after the last change

1:500 body and structure, 1:200 body three-quarter, 1:100 body (all default pose). Rendered but NOT read (the image reader of the session stopped answering after the publish): 1:50 `gate-raised` from behind, `nesting-swing` t 3 s and 6 s, `load-groceries` t 8 s, `gate-raised` at 1:100 / 1:200 / 1:500, thermal 1:100 / 1:200 / 1:500 (cut), structure 1:100 / 1:200. Read before the change (unchanged data): 1:50 body, structure, thermal, `loaded`, `gate-raised` right side; 1:200 and 1:500 body; 2D 1:50 and 1:500.

### Open, stated plainly (for the next pass)

- All scales, body: front wall rake 41 degrees from the vertical (floor 0.62 m under a 0.915 m rim). My memory of the EL class says 25-30 degrees; not verified against a drawing, so not rebuilt. If corrected: floor front, front mesh, corner wires, front struts (they would lean back or move), all four scales, thermal bundles, 2D basket shapes.
- All scales: no child seat in the gate (variant without one), no plastic corner bumpers.
- Poses: `loaded` and clip `load-groceries` move nothing and show no goods (the load is a structural load only). No pose or clip swivels a castor or rolls a wheel.
- 1:500 body: upright runs straight from the rail end to the handle bar (13 degrees instead of 9.3; the 80 mm handle arm is merged), wheel 75 mm wide instead of 32 mm (feature rule).
- 1:50 body: crate tray is fixed although a real tray folds up for nesting; castor fork legs are flat plates without the pressed bead.
- 2D 1:100 / 1:200 were not redrawn in this pass (the silhouette rule passes; the 1:100 elevation may still draw the fork crown, the 1:200 tray stroke draws the lip ends as posts).
- Poses and clips after the last change: see the unread list above.

### Fundamental issues (outside this asset)

- A 2D stroke that outlines a body the 3D fills (plan `rim` over the mesh planes) is compared at pen width on the drawn side only: at 1:500 a 0.13 mm pen is 65 mm, so every such outline centred on the true edge costs half a pen all round (plan IoU 0.879 -> 0.955 by moving the stroke inside). Either the rule or the drawing convention (outline strokes lie inside the edge) should be stated in the README.
- `volumes-check.ts --scale N` used the product engine for 1:500 (`[p]`) and the owned engine for 1:200 and 1:100 (`[o]`) in the same session; the owned engine reported failures (1:200 feature `front-cross` 29.67 mm, silhouette front 0.857, cross-scale) that the product validator does not report. The tool does not say why it switched.
- `validator.md` has no section "Rule changes authors must know" (the task refers to one).
- The short-part gap: a part shorter than 20 diameters cannot be `slender` and must then be one full feature thick, so real short connectors (tray posts 40 mm at 1:200, handle arms 80 mm at 1:500) cannot be modelled at all; the README should say what replaces them (here: a panel, a merged rod).
