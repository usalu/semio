# Authoring objects: 3D volumes of the 15 `objects` assets

All 15 assets have a spliced volume (`volume-insert.py`), sources in `volumes/objects/<slug>.json`, generators `authoring-objects-<slug>.py` with the shared `authoring-objects-lib.py` (world coordinates in, bone-local explicit data out; every loft section carries an explicit `rotation`, no `lateralScale`, no closed revolve profile). Validation: `validateAsset` volume rules 0 findings on all 15, Ajv errors under `/volume`: 0 on all 15 (some catalog files fail Ajv elsewhere, see `authoring-requests.md` item 5). Every pose of every asset was walked with `authoring-objects-posecheck.ts` (bounding box and separated solid pairs; detachments listed are by design, see remarks); signed solid volumes were checked positive with `authoring-objects-solids.sh`. Numbers from `authoring-objects-report.ts`.

Columns: bbox error in percent x (`widthM`) / y (`heightM`) / z (`depthM`) at rest, then in the default pose (`presentation.defaultPose`, the pose `dimensions` describe); IoU elevation / front / plan at 1:50 at rest.

## Assets missing a gate

- **cooking-pot**: elevation IoU 0.60 (authored elevation is a section).
- **market-parasol**: rest bbox z -53.2 % and y +11.3 %, rest IoU 0.27/0.43/0.32 (rig rest has collinear ribs); default pose open-still passes the bbox gate (-1.2/+0.1/-1.4 %).
- **playground-swing**: front 0.76, plan 0.77 (evaluator misplaces the authored seat and chain shapes).
- **retractable-bollard**: rest y -33.3 %, elevation 0.70, front 0.71; default pose raised exact.
- **shopping-trolley**: front 0.60 (evaluator misplaces the authored gate).

All other assets pass every default gate. In each failing case the body is the physically right one and was not distorted.

| slug | solids | bbox % rest x/y/z | bbox % default pose | IoU e / f / p | validator issues | poses checked | remarks |
|---|---|---|---|---|---|---|---|
| bicycle-stand | 7 | +0.1/+0.0/+2.1 | empty-sun: +0.1/+0.0/+2.1 | 1.00 / 1.00 / 1.00 | 0 | empty-shade, empty-sun, bike-leaning, lock-pulled | Hoop is two lofts (legs, bends, top in one path) so feet end flat; collars to 1:100. |
| chalkboard | 17 | +0.0/+0.0/+3.1 | leaf-left: +0.0/+0.0/+3.1 | 1.00 / 0.99 / 1.00 | 0 | leaf-left, leaf-middle, leaf-right | Frame set to the back (z -0.005..0.04) so the leaf (z -0.04..-0.005) can slide past the centre post; front/plan draw the frame full depth 0.08 (inconsistent with a sliding leaf). Hangers and grips 1.5-3 mm proud (depth +3.1 %). |
| charcoal-grill | 47 | +1.1/+0.0/+0.6 | grilling-closed: +1.1/+0.0/+0.6 | 0.97 / 0.98 / 0.96 | 0 | cold-closed, lit-lid-open, grilling-closed, embers-vents-low | Hollow bowl and lid (ring stacks). Elevation draws one front side handle that front/plan contradict (not modelled); plan draws the lid handle at z +0.08..0.14 (elevation coordinates reused), modelled centred. Lid open: side handles separate from lid by design. |
| cooking-pot | 13 | +0.0/+0.0/+0.0 | boiling: +0.0/+0.0/+0.0 | 0.60 / 1.00 / 1.00 | 0 | cold, heating, boiling | FAILS elevation gate (0.60): the authored elevation is a section (air above the water not drawn, recall 1.00, precision 0.60); front and plan show the closed pot. Plan draws water radius 0.072 vs elevation 0.080. |
| drinking-fountain | 27 | +2.5/-0.0/+2.5 | drinking: +2.5/-0.0/+2.5 | 0.97 / 0.99 / 0.95 | 0 | idle-sun, drinking, idle-shade | Rim bead 2.5 % over width/depth. Nozzle drawn as circle in two views (impossible); modelled upward. Jet lowered 0.25 m in idle poses (detaches by design). |
| market-parasol | 38 | +2.6/+11.3/-53.2 | open-still: -1.2/+0.1/-1.4 | 0.27 / 0.43 / 0.32 | 0 | open-still, tilted-to-sun, furled | FAILS rest gates (z -53 %, y +11 %, IoU 0.27/0.43/0.32) by construction: rig rest has the 8 ribs collinear. Gores are baked in the open pose; in default pose open-still bbox -1.2/+0.1/-1.4 %, uniform cone (radius 1.28 m in every azimuth bin), furled folds with the ribs. Only 2 of 8 stretchers have bones. Reviewer: check rest view looks odd on purpose. |
| patio-heater | 21 | +0.0/+0.0/+0.0 | high: +0.0/+0.0/+0.0 | 0.99 / 0.99 / 1.00 | 0 | off-afternoon, low, high, high-in-breeze | Wheels moved outside the housing (z +-0.185, drawn hidden); knob lies on its side (axis z, rig joint about w) while two views draw a circle. |
| playground-swing | 30 | +0.0/-0.0/+0.0 | rest: +0.0/-0.0/+0.0 | 0.99 / 0.76 / 0.77 | 0 | rest, pendulum-forward, pendulum-back | FAILS front (0.76) and plan (0.77): authored front/plan placement of the swinging `seat` and `chain-*` shapes is wrong in `evaluate` (seat front at x -0.098..-0.04, chain at x 0; plan seat z -0.098..-0.04); elevation 0.99. Seat 0.42 x 0.52 x 0.058 per extrusion hints. |
| retractable-bollard | 17 | +0.0/-33.3/+0.0 | raised: +0.0/+0.0/+0.0 | 0.70 / 0.71 / 1.00 | 0 | raised, lowered, moving-up, raised-nudged | FAILS rest gates (y -33 %, elevation 0.70, front 0.71) because dimensions describe the raised default pose and the authored elevation/front are cuts with an empty sleeve (recall 1.00, precision 0.70). In default pose raised: bbox exact (0.0/0.0/0.0 %). Raised: ram rod meets shaft base, no solid detaches wrongly. |
| shopping-trolley | 53 | +0.6/+0.0/+2.1 | empty: +0.6/+0.0/+2.1 | 0.94 / 0.60 / 0.95 | 0 | empty, loaded, gate-raised | FAILS front (0.60, recall 0.64): authored front `gate` (solid hint on the gate bone) is placed 34 % off by `evaluate`; wire mesh modelled as a 4 mm shell with wire loops. Plan/elevation 0.95/0.94. |
| stepladder | 23 | -0.0/+0.0/+2.2 | open-empty: -0.0/+0.0/+2.2 | 0.82 / 0.93 / 0.94 | 0 | open-empty, open-in-use, rated-load-test | Plan draws rear-brace-1 at x 0.105, elevation at x 0.0 (on the rail); modelled on the rail. Elevation 0.82 close to the gate. |
| storage-crate-stack | 40 | +0.0/+0.0/+0.0 | high: +0.0/+0.0/+0.0 | 0.99 / 0.99 / 1.00 | 0 | high, half, low | Uniform taper factor gives rim 0.61 x 0.37 instead of 0.60 x 0.38. Nesting poses half/low: rims/feet bbox gaps are nesting, not detachment. |
| street-lamp | 15 | +0.2/+0.0/+0.0 | dusk-off: +0.2/+0.0/+0.0 | 0.99 / 0.94 / 0.96 | 0 | dusk-off, lit-night, dimmed-night, lit-in-wind, day-off | Pole lean pose lit-in-wind shifts bbox x (1.38 m) as intended. |
| tealight | 6 | +0.0/+0.0/+0.0 | lit: +0.0/+0.0/+0.0 | 0.98 / 0.99 / 1.00 | 0 | unlit, lit, flicker-peak | Dimensions describe the lit pose; unlit flame sinks into the wick. |
| waste-bin | 15 | +0.8/+0.3/+0.0 | closed-empty: +0.8/+0.3/+0.0 | 1.00 / 0.94 / 0.97 | 0 | closed-empty, lid-open, full-load | Hinge brackets at z +-0.19 (plan) vs +-0.187 (front); lid-open: skirt separates from shell by design. |

## Not used / noted for the coordinator

- Poses checked: all poses of each asset (column), default pose plus at least one other for every asset.
- Revolve cannot make open-top vessels; stacks of tapered ring extrusions are used (7 assets). See `authoring-requests.md` (objects author).
- Re-run of the whole set: `authoring-objects-run.sh <slug>` per asset, `authoring-objects-report.ts` for the table.
