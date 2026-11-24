# 🧱️ Silhouette Batch SC: 23 Components (2026-11-23)

Fixer report for `U/🧰️silhouette/SC.tsv`. Paths as in `📋️silhouette-brief.md` (`S` staffage, `U` umbrella ticket); assets are `S/🗂️catalog/🧱️components/<id>/🔣️.json`.

## Result

- **20 of 23 green** in the real `🧊️catalog-silhouette` test (0 before). **3 stay red**: `reclaimed-trapezoidal-sheet-stack` (untouched), `used-insulated-copper-coils` (untouched), `sectional-garage-door` (6 failing checks down to 4). Numbers under "What stays red".
- Failing checks of the gate (bounding box, IoU, largest boundary distance, per scale and view): 53 before (51 measured plus the crash of the cobblestone bag and its hidden 48.8 mm), 9 after.
- No rule, limit, test, tool, engine, schema, fixture, README or manifest was edited. One declared dimension changed (`reclaimed-steel-stair` width), handed over in `U/🧰️silhouette/SC/manifest-changes.json`.
- Five engine findings with measurements were appended to `U/requests.md` (sections `## silhouette SC — …`).

## Proof (all run to the end in this session, final file state)

| What | Command (in `U/🧰️silhouette/SC`) | Result |
|---|---|---|
| Gate, every check of every scale and view | `m.sh <23 ids>` (`measure.ts`) | 20 assets `OK`; the 3 red ones as listed below |
| Real `🧊️catalog-silhouette`, 23 ids | `vt.sh final-silhouette 🧊️catalog-silhouette <ids>` | 19 passed, 4 failed (the 3 red ones and `reclaimed-lattice-girder`, which then hit the 10 MB reference limit); after the rivet repair `vt.sh final-silhouette-girder … reclaimed-lattice-girder`: 1 passed. Together 20 passed, 3 failed |
| `validateAsset`, 23 assets | `validate.ts` | 0 issues for all 23 (0 before) |
| Product check CLI, 23 assets | `🩺️check/🟦️.ts --asset <id>` | `✓` and exit 0 for all 23; every authored scale still "suitable under office-laser" |
| `🧊️three-views` | `suites.sh final <ids>` | 46 passed (46 before) |
| `🌡️catalog-physics` | same | 46 passed (46 before) |
| `🫁️breathing-catalog` | same | no test of the file names a component (290 skipped, as before) |
| `asset-power` | same, by id and whole file | 2 passed by id, 24 of 24 for the file (as before) |
| `asset-validation` | same, whole file by its describe names | 83 of 83 (as before) |
| Sheets | `look.sh after <id> <scales>` (53 sheets) | 1:50 and the coarsest authored scale of each of the 21 touched assets opened and looked at, plus every other scale a repair changed; no drawing became uglier or wrong. Two candidate repairs were dropped for what they did to the drawing or its print suitability (jamb brackets at 1:100 of the garage door; three stub outlines 0.13 mm apart in the plan of the wall panel, solved with layers and the detail pen instead) |

## Per asset

"Before" and "after" are the failing checks of the gate (limit in brackets where it helps: twice the smallest feature, 15 / 30 / 60 / 150 mm at 1:50 / 100 / 200 / 500). Every repair is a named, idempotent script `U/🧰️silhouette/SC/fix_<id>.py` (second run: "already repaired, nothing to write"); no generator of these assets exists in a ticket in a state that reproduces the catalog file, so the scripts are the last step. `sc.py` writes only the lines that change.

| Asset | Before | After | What was wrong and what changed |
|---|---|---|---|
| `masonry-wall-with-lintel` | 1:100 front section 127.5 mm | none | The sliver between the sloped sill and the reveal was undrawn at 1:100. Added the `sill-end-beyond` wedge of 1:50. |
| `panel-radiator` | 1:50 plan IoU 0.894 and 45.0 mm; 1:100 plan IoU 0.899; 1:100 elevation 30.5 mm | none | Two rows of grille slots are open to the floor (the fins are texture); the plan filled them. `top-grille` of 1:50 and 1:100 is drawn with those 16 slots open, as 1:200 already is. The upper bracket stroke of 1:100 follows the arm and the wall plate. |
| `reclaimed-cobblestone-bag` | rasteriser crash; behind it 1:50 front 48.8 mm | none | **Why it rendered so large:** each of the 384 setts of 1:50 was a box with `cornerRadius` 8 mm, about 2032 triangles each: 780 332 triangles, a view of 39 MB against the 10 MB the librsvg reference reads. Each sett is now a polyhedron of the same cube with its twelve arrises as 5 mm chamfers (24 vertices, 44 triangles; 16 896 in all, silhouette within 1 mm). Front: the joint line under the deck boards crossed the four open gaps; it is drawn under the boards only. |
| `reclaimed-concrete-wall-panel` | 1:50 and 1:100 plan 60.0 mm; 1:200 and 1:500 width 6.00 against 6.12 m | none | The six cut rebar stubs make the 6.12 m. Plan of 1:50 and 1:100 drew one of three per end (the panel leans, all three show): the other two added, one layer lower each. Bodies of 1:200 and 1:500 had no stubs: 1:200 gets the rods and draws them, 1:500 gets 75 mm blocks. |
| `reclaimed-floorboard-stack` | 1:50 elevation 15.1 mm | none | A 12 mm notch between the two drawn parts of each sling; the upper part is drawn down to where its solid ends. |
| `reclaimed-lattice-girder` | 1:50 elevation 26.3, front 19.9, plan 35.6 mm; then the 10 MB limit | none | Front: the bracing stub is a step, not a diagonal. Plan: the two slotted holes of the right bearing plate are drawn open. Slings: not `slender` at 1:50 (30 mm thick). 434 rivet heads were ellipsoids of 528 triangles (11.6 MB view): now revolves of the same shape, 105 000 triangles in all. |
| `reclaimed-paving-slab-pallet` | 1:50 plan 148.1 mm (tiles), front 16.9 mm | none | Plan: each band is a stroke around its pack instead of a closed outline (the 5 mm sliver beside the pack then agrees in drawing and body). Front: the band of the rear pack is drawn too, under the front one. |
| `reclaimed-precast-stair` | 1:50 elevation 15.5 mm; width 3.247 (1:100) and 3.254 m (1:200) against 3.2958 | none | The four bar stubs make the length. 1:50 draws the stub over its whole bar; 1:100 and 1:200 get the four bars in body and drawing. |
| `reclaimed-steel-column` | 1:50 elevation 20.6 mm | none | 25 mm sling not `slender` at 1:50; front and plan draw it to its tips. |
| `reclaimed-steel-stair` | width 4.9523 (1:50, 1:100) and 4.915 m (1:200) against 4.974 | none | 4.974 m is no length of the flight: body and drawings are 4.9524 m. `dimensions.widthM` 4.952 (manifest row to follow). 1:200 gets the two base plates (31 mm thick) in body and drawings. |
| `reclaimed-timber-roof-truss` | 1:50 front 26.3 mm, plan 26.3 mm | none (front 0.0 mm) | The bolts of each joint were one strip beside the gusset; each of the 56 bolts is drawn as its own outline from its solid. |
| `sliding-door` | 1:200 elevation 150.0 mm; 1:500 elevation 157.4 mm | none | 1:200: leaf drawn to its own top edge. 1:500: the two trolley blocks between leaf and track drawn. |
| `spiral-stair` | 1:50 elevation 49.5, front 18.8, plan 68.9 mm | none | Drawn now: the anchor studs and nuts on the base plate, the 22 mm foot of every baluster (side and front), the two packers and bolts of the top tread (plan). |
| `stair-flight` | 1:50 elevation 21.4, front 20.6 mm | none | Handrail strokes ended half a width beyond the tube; they end half a width earlier. |
| `tilt-window` | 1:200 front 62.1 mm | none (0.0 mm) | A 12 mm ring of nothing between frame and sash where the stops are; frame members drawn 80 mm wide. |
| `used-buffer-storage-tank` | 1:50 front 18.8 mm | none | Drawn and tessellated circle stood half a step apart at the bearers; the shell solid is turned 6 degrees about its own axis. |
| `used-radiant-ceiling-strip` | 1:50 and 1:100 front 45.0 mm | none | The return stub of the far end was not drawn in the front elevation; drawn as the mirror of the flow stub. |
| `used-steel-pipe-rack` | 1:50 elevation 46.0, front 24.4, plan 24.4 mm | none | Ties and straps not `slender` at 1:50 (drawn at true section). |
| `used-trench-convector` | 1:50 front 26.3 mm | none | The levelling brackets beside the trough were not drawn in the front elevation. |
| `wheelchair-ramp` | 1:200 elevation 105.0 mm (IoU 0.921) | none (IoU 0.976) | Handrails not `slender` at 1:200 (42.4 mm tube; the kernel finds no centre line for the bent tube and compared a straight chord). |
| `sectional-garage-door` | height 2.8611 against 2.85 m (1:50); 1:50 elevation 16.9, front 18.8, plan 30.2 mm; 1:100 elevation 55.7, front 41.2 mm | 1:50 elevation 16.9, front 18.8, plan 30.2 mm; 1:100 front 41.2 mm | Braces cut 16.7 mm shorter so their corner ends at the ceiling (all bodies and side elevations). 1:100: ceiling foot of the brace drawn. |
| `reclaimed-trapezoidal-sheet-stack` | 1:50 front IoU 0.904 and 23.2 mm; 1:100 front 31.8 mm, plan 37.5 mm | unchanged | Not touched, see below. |
| `used-insulated-copper-coils` | 1:100 elevation 43.0 mm | unchanged | Not touched, see below. |

## What stays red

- **`sectional-garage-door`**, 4 checks: 1:50 side section 16.9 mm (one cell at (-0.044, 0.475) m), 1:50 front 18.8 mm (five cells beside the cable drum at (±1.203, 2.830) m), 1:50 plan 30.2 mm (a 1 mm clearance between stile and cable stud that the body shows as one long slit and the drawing as filled pieces), 1:100 front 41.2 mm (the 7.5 mm slot between panels and tracks, open in the drawing, closed in the body by the jamb brackets and roller stems; drawing the brackets at 1:100 made the scale unsuitable for print, 0.188 mm against 0.2 mm, and did not close it, so it was taken out again). All four are decided by the hole and slit rules described in `requests.md`, none by a wrong shape I could name.
- **`used-insulated-copper-coils`**, 1 check: 1:100 side elevation 43.0 mm against 30 mm. All 162 drawn members lie within 1.5 mm of their solids; two crescents between windings are counted as openings in one mask and not in the other. File untouched.
- **`reclaimed-trapezoidal-sheet-stack`**, 4 checks, file untouched. 1:50 front IoU 0.904: the body has the 29 nested sheets with 0.75 mm of air between their flats, the drawing a solid bundle; the only repair I found (the body of 1:50 as one pack like 1:100, measured: IoU 0.972) throws away a stated authoring decision and still leaves the asset red, so I did not apply it. 1:50 front 23.2 mm and 1:100 front 31.8 mm: the triangle under the taut strap (see `requests.md`). 1:100 plan 37.5 mm: the top sheet of the 1:100 body carries its edge crest over the full 6 m while the drawing (and the 1:50 body) trim it back 150 mm; not repaired.

## Decision for the coordinator

- `manifest-changes.json`: `{ "reclaimed-steel-stair": { "widthM": 4.952 } }`. Until it is applied the manifest row says 4.974 m.
- The trapezoidal stack needs a ruling: is the 1:50 body allowed to keep 29 separate sheets (then the IoU gate cannot pass), or is it one pack at 1:50 too?

## Files

Updated (20 assets, repo-relative under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/`): `masonry-wall-with-lintel`, `panel-radiator`, `reclaimed-cobblestone-bag`, `reclaimed-concrete-wall-panel`, `reclaimed-floorboard-stack`, `reclaimed-lattice-girder`, `reclaimed-paving-slab-pallet`, `reclaimed-precast-stair`, `reclaimed-steel-column`, `reclaimed-steel-stair`, `reclaimed-timber-roof-truss`, `sectional-garage-door`, `sliding-door`, `spiral-stair`, `stair-flight`, `tilt-window`, `used-buffer-storage-tank`, `used-radiant-ceiling-strip`, `used-steel-pipe-rack`, `used-trench-convector`, `wheelchair-ramp` (each `<id>/🔣️.json`; that is 21 files: 20 green and the garage door).

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (five sections appended).

Created under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `silhouette-SC.md` (this report) and in `🧰️silhouette/SC/`: `manifest-changes.json`; the repairs `fix_masonry_wall_with_lintel.py`, `fix_panel_radiator.py`, `fix_reclaimed_cobblestone_bag.py`, `fix_reclaimed_concrete_wall_panel.py`, `fix_reclaimed_floorboard_stack.py`, `fix_reclaimed_lattice_girder.py`, `fix_reclaimed_paving_slab_pallet.py`, `fix_reclaimed_precast_stair.py`, `fix_reclaimed_steel_column.py`, `fix_reclaimed_steel_stair.py`, `fix_reclaimed_timber_roof_truss.py`, `fix_sectional_garage_door.py`, `fix_sliding_door.py`, `fix_spiral_stair.py`, `fix_stair_flight.py`, `fix_tilt_window.py`, `fix_used_buffer_storage_tank.py`, `fix_used_radiant_ceiling_strip.py`, `fix_used_steel_pipe_rack.py`, `fix_used_trench_convector.py`, `fix_wheelchair_ramp.py` with their library `sc.py`; the tools `env.sh`, `lib.ts`, `measure.ts` (`m.sh`), `where.ts` (`w.sh`), `tiles.ts` (`t.sh`), `tilemask.ts`, `rings.ts` (`r.sh`), `refs.ts`, `pairs.ts`, `png.ts` (`p.sh`), `validate.ts`, `roundtrip.ts`, `show.py`, `vt.sh`, `suites.sh`, `prove.sh`, `quick.sh`, `look.sh`.

Removed: `🗑️generated/silhouette/SC` (logs, mask pictures, sheets, copies of the original files).
