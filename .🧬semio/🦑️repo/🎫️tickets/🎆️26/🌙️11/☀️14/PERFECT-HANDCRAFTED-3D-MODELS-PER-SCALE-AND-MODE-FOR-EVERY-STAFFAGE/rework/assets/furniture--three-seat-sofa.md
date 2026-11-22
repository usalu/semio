# `furniture/three-seat-sofa` — rework report

Source `volumes/furniture/three-seat-sofa.json`, catalog `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/three-seat-sofa/🔣️.json`.
Edit scripts (inputs, kept): `rework-three-seat-sofa-frame.py`, `rework-three-seat-sofa-decisions.py`, `rework-three-seat-sofa-cushions.py`.

## Pass 1 (2026-11-22)

The batch report `rework/14.md` holds a reference table for this asset but never started it (no section, no edits).

### Reference table (written before any edit)

Box-arm three-seat sofa with loose seat and back cushions on a hardwood frame; lengths in mm, from memory of manufacturer practice and DIN EN 12520 / EN 1728 (domestic seating, strength and test loads), EN 1021 (upholstery build-up).

| Quantity | Reference | Model 1:50 before | Verdict |
|---|---|---|---|
| Overall length / depth / height | 1900 – 2200 / 850 – 950 / 780 – 880 | 2000 / 900 / 850 | right |
| Seat height unloaded | 420 – 450 | 440 | right |
| Seat depth (front edge to back cushion) | 520 – 580 | 575 | right, upper end |
| Seat width per place | 500 – 600 | 533 | right |
| Seat cushion | 120 – 200 thick, domed crown 15 – 25 over the boxing seam, piped seams | 210 thick, flat plateau with a 12 mm rounded shoulder, piping top and bottom | crown missing |
| Back cushion | 120 – 180 thick, top 30 – 60 under the frame top, lumbar belly | 150 – 170, top at 800, belly | right |
| Back rake (seat to back face) | 100 – 110 degrees | frame face 8 degrees, cushion face about 10 degrees | right, lower end |
| Arm | 150 – 250 wide, 580 – 650 high, full depth | 200 / 620 / 900, 60 mm top roll | right |
| Plinth | 100 – 150 high, set under the cushions | 130, 10 mm proud of the cushions | right |
| Feet | 4 – 6 turned or block feet, 100 – 150 high, 45 – 60 diameter | 4 turned, 100 high, 45 – 60 | right |
| Frame rails | beech 25 x 90 – 100 on edge, front and back rail tied by cross stretchers every 500 – 600 | two rails, no stretcher, 18 mm deck spanning 800 | stretchers missing |
| Back frame | bottom rail, raked uprights 22 – 25 thick sawn to the rake (100 – 120 deep at the seat, 40 – 50 at the top), top rail, webbing or ply face | vertical 25 x 40 sticks, no face at 1:50, top rail overhanging | wrong |
| Arm frame | front and back post, top and bottom rail, inner panel, bottom rail bearing on the feet | bottom rail and posts began 5 mm above the foot heads | feet not connected |
| Suspension | serpentine springs or elastic webbing, or a ply platform | 18 mm ply platform | accepted (platform sofa) |
| Upholstery build-up | fabric 1, batting 20 – 40 over timber or ply, foam cores 40 kg/m3 | layers nested at 1:50 only; foam, batting and ply cores carried the material `upholstery-fabric`; ply core of the back frame folded back on itself at the top (stations 0.660 then 0.652) | wrong materials, broken loft, layers missing at 1:100 and 1:200 |
| Mass | 45 – 60 kg | 51.8 kg | right |

### Look (sheets read at full size)

1:50 body, thermal (with cut), structure, default pose and loaded poses; 1:100, 1:200, 1:500 body; 1:100, 1:200, 1:500 structure; 1:200 and 1:500 thermal cut; the 1:50 2D drawings. The body reads as the subject from all six directions: proportions are inside the reference at every scale, 1:500 is a correct block sign (75 mm feet, chamfered back block). The defects are in the frame and in the layers, listed in the table.

### Rebuilt

- **Structure 1:50** (`rework-three-seat-sofa-frame.py`): arm posts and bottom rails now reach the arm base at 0.10 m and sit on the foot heads; two cross stretchers 25 x 90 mm under the cushion joints (x = 0.733 and 1.267 m) between the base rails; the four back uprights are raked extrusions sawn to the rake (115 mm deep at the deck, 50 mm under the top rail, rear edge flush with bottom and top rail); a 12 mm ply face on their raked front edges carries the back cushions (id `member-back-face`, the id the 1:100 list already uses).
- **Structure 1:100, 1:200**: the two cross stretchers at their feature size (30 mm). 1:500 unchanged (two rails, deck, panels; a stretcher is below 75 mm).
- **Thermal 1:50**: every nested layer in its own material from the catalog layer table (HR foam, polyester batting, birch plywood); the ply core of the back frame re-drawn with monotonic stations ending under the top roll.
- **Thermal 1:100, 1:200**: the same nested cores as at 1:50 (foam cores of six cushions, batting of both arms and the back frame, ply cores of back frame and plinth), each one layer thickness inside its surface, so the cut sheet shows the build-up at every scale that has separate cushions. 1:500 keeps its region blocks.
- **Decisions** (`rework-three-seat-sofa-decisions.py`): second decision of 1:50, 1:100, 1:200 rewritten, en and de.
- **Rig**: unchanged. Ten bones (ground, frame, two arms, three seat and three back cushions on prismatic joints) are the right topology for a sofa with loose cushions; limits 90 mm (seat, foam on block) and 40 mm (back) are plausible.
- **2D**: not touched by the frame and layer work (outer form unchanged).
- **Body 1:50 and 1:100** (`rework-three-seat-sofa-cushions.py`): the three seat cushions get the domed crown of a real foam-and-wrap cushion. Envelope unchanged (top 0.44 m, full plan size at the top boxing seam); above the seam the top panel rises 20 mm to the middle through seven stations (1:50) or four (1:100) instead of a flat plateau with a 12 mm shoulder. Foam cores of the thermal list follow the new body. 1:200 keeps its chamfer (a 20 mm crown is below 30 mm), 1:500 its block.
- **2D 1:50 and 1:100** (`rework-three-seat-sofa-2d.py`): `seat-1..3` in the side elevation and `seat-2` in Section A-A redrawn by hand between the two seam corners with the dome stations (8 point arrays, text splice, the rest of the catalog file byte-identical). Checked with `r3-2d.ts` on the candidate before writing; plan unchanged (plan size unchanged).
- **Decisions**: first decision of 1:50 and 1:100 names the domed crown.

### Proof

- `r3-validate.ts furniture/three-seat-sofa --catalog-only --volumes` after the last publish: `VALID furniture/three-seat-sofa (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Rendered after the last geometry change: 1:50 body six views, all six non-default poses (three-quarter), two instants of each of the three clips, 1:100 / 1:200 / 1:500 body, thermal and structure, 1:50 and 1:100 thermal cut, 1:50 structure, 1:100 2D.
- Read after the last geometry change: 1:50 body six views and three-quarter, pose `seat-proof-load`, clip `sit-down-and-settle` at 0.6 s, 1:100 body six views, 1:100 thermal cut, the 1:50 2D candidate. Read before the cushion change only (frame and layers did not change afterwards): 1:50 structure six views, front close-up and pose `seat-proof-load`, 1:100 structure, 1:200 thermal cut. Rendered but not read: poses `one-seated`, `two-seated`, `lounging`, `impact-key`, `recovering-key`, clips `stand-up-and-recover` and `lie-down-on-sofa`, 1:200 and 1:500 sheets after the change (their lists changed only by the stretchers and cores at 1:200, not at all at 1:500), 1:100 2D.

### What remains imperfect (for the next pass)

- 1:50 structure, `member-seat-1..3` and `joint-seat-1..3`: the foam bearing band and its contact pad are rigid with the cushion bone, so in every loaded pose they sink through `member-deck` (by 60 mm seated, 90 mm at proof load) and the middle band touches the stretchers by 3 mm. The load path is right in the default pose only. A fix needs the band to stay on the deck while the cushion top sinks (a second bone per cushion or a member bound to `frame`), with the structure and pose data re-bound.
- 1:50 body, all poses: a loaded seat cushion sinks as a rigid block into the plinth (bottom piping disappears behind the plinth edge) instead of compressing; the back cushion slides down 18 – 30 mm instead of compressing towards the frame. The rig is planar in the long elevation (x, y), so a movement along z cannot be posed.
- 1:50 body, arms: no welt on the front and back arm panels, no seam between inner arm face and top roll; back cushions without a perimeter seam (the seat cushions have their piping). 1:50 feature size would carry an 8 mm welt.
- 1:50 body, `back-frame`: rear face vertical; a real frame of this type is raked 3 – 5 degrees at the rear too. Not changed because every 2D back shape and the depth 0.90 m hang on it.
- 1:50 structure: no serpentine springs or webbing (ply platform kept); the arm frame has board posts 140 x 25 instead of 40 x 40 posts with a separate front panel; no hanger-bolt plate between foot and arm bottom rail (5 mm, below the feature).
- 1:100 structure: back uprights are still vertical 25 x 40 sticks (only the 1:50 list got the raked uprights); 1:100 has two end uprights only.
- 1:200 and 1:500: not re-authored in this pass beyond stretchers and cores at 1:200; judged right for their feature size from the sheets read before the changes.
- Poses and clips listed above as "rendered but not read".

### Fundamental issues (outside this asset)

- `r3-sheet.ts` output folder `🗑️generated/r3-sheet/furniture/` is shared by all authors of a category and is emptied by other authors' clean-up while one is still reading (26 of my sheets vanished between render and read once). A per-author output folder flag would avoid it.
- Thermal layers thinner than the feature size (1 mm cover) cannot be told apart on the uncut thermal sheet at any scale; `validateAsset` accepts both "one surface per region" and nested cores at 1:100 / 1:200, so the catalog is inconsistent between assets on this point. The README should state which one is required.

## Pass 1 (second round) (2026-11-22)

Source and catalog `volumes` were identical at the start (no half-written edit). Edit scripts of this pass (inputs, kept): `rework-three-seat-sofa-r2-welts-loadpath.py`, `rework-three-seat-sofa-r2-thermal-layers.py`, `rework-three-seat-sofa-r2-decisions.py`.

### Judged against the reference table above (sheets read at full size before any edit)

1:50 body six views and three-quarter, 1:50 structure six views and three-quarter, 1:50 thermal cut, pose `seat-proof-load` (elevation). Proportions, rig (ten bones, prismatic cushions) and the frame of the first round hold. Deviations found:

| Scale, list | Part | Defect |
|---|---|---|
| 1:50 body | `arm-l`, `arm-r` | no welt on the front and back arm panels (reference: 8 mm welt at 1:50) |
| 1:50 body | `back-1..3` | no seam at all, while the seat cushions carry piping |
| every scale, structure | `member-seat-1..3`, `joint-seat-1..3` | a 30 x 150 mm "bearing band" at the front edge only, rigid with the cushion bone: it sank through `member-deck` in every loaded pose |
| 1:100 structure | `member-upright-1..2`, `member-back-face` | two vertical 25 x 40 sticks and a vertical face, while the body and the 1:50 frame are raked |
| 1:100, 1:200, 1:500 thermal | every surface solid | no `layer`: drawn as no layer of its region (grey on the sheet); at 1:500 the four region blocks also carried foam and plywood as material although they are the cover surfaces |

### Rebuilt

- **Body 1:50**: `welt-arm-l`, `welt-arm-r` (extruded rings, 8 mm wide, 4 mm proud, up the sides and over the top roll of the front and the back arm panel, flush at the bottom edge; outline taken at the plane 14 mm behind the panel face from the arm's own section numbers); `seam-back-1..3` (a thin loft through the stations of each back cushion, 8 mm thick, 3 mm proud: a knife-edge seam that follows belly and crown). All `texture: true`, like the seat piping.
- **Structure, every scale**: `member-seat-N` is now a 90 mm foam slab (0.513 x 0.45 m) on the cushion bone; new `member-seat-N-spring` (foam block from the deck to the slab, bound to `frame`, member of the cushion); `joint-seat-N` (1:50, 1:100) lies on the deck, bound to `frame`. At 90 mm proof load the slab ends 7 mm above the deck: nothing passes through the deck in any pose.
- **Structure 1:100**: four uprights sawn to the rake and a raked 15 mm ply face, the same frame as at 1:50.
- **Thermal 1:100, 1:200, 1:500**: every surface solid names its layer (`fabric`, feet `beech-foot`); the 1:500 blocks carry the cover material.
- **Decisions**: every scale, en and de, extended by what this pass modelled.
- **Rig**: unchanged (judged right). **2D**: unchanged on purpose. The outer form did not change; an arm welt 13 mm inside the arm outline is 0.26 mm on paper at 1:50 and would merge with the outline pen, the back seams lie on the silhouette. Read `r3-2d` 1:50 after the change: elevation, section and plan agree with the model.

### Proof

- After the last publish: `VALID furniture/three-seat-sofa (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`--catalog-only --volumes`).
- Rendered and read after the last geometry change: 1:50 body six views, three-quarter and rear three-quarter; 1:50 structure three-quarter in `seat-proof-load`; 1:100 body and structure six views; 1:200 body and structure six views; 1:500 body, structure, thermal six views; 1:100 thermal cut (before the layer names), 1:200 thermal cut (after); poses `seat-proof-load`, `two-seated`, `one-seated`, `lounging` (three-quarter); clips `sit-down-and-settle` 0.6 s, `stand-up-and-recover` 2 s, `lie-down-on-sofa` 3 s (elevation); 2D 1:50.
- Rendered, not read: poses `impact-key`, `recovering-key`; clip instants `sit-down-and-settle` 1.8 s, `stand-up-and-recover` 0.3 s, `lie-down-on-sofa` 1 s (the same cushion travel between the instants read); 1:50 thermal uncut after the last change (list unchanged at 1:50). Not rendered after the change: 1:100, 1:200, 1:500 2D (untouched).

### What remains imperfect

- 1:200 body, `arm-l`, `arm-r`: the single chamfer station at the top (0.46 to 0.52 m, both radii reduced at once) leaves a small ear at the front and rear top corners in the three-quarter view; needs one more station or a plain chamfer in one direction.
- 1:50 body, `seam-back-1..3`: over the top the seam is only as wide as the cushion's top cap (0.35 m) and ends there as a tab, because `back-N` itself ends in a flat 0.35 x 0.05 m cap; the cushion crown wants two more stations down to a rounded end, the seam with it.
- 1:50 body, `back-frame`: rear face still vertical (reference 3 to 5 degrees); every 2D back shape and the depth 0.90 m hang on it.
- 1:50 and 1:100 body: no fifth and sixth foot at mid length; a 1.8 m span on 25 x 110 rails is at the limit for three sitters (reference allows 4 to 6 feet).
- Thermal, every scale: the seat regions are still rigid with the cushion bone (surface and foam core sink through `th-base` in loaded poses); only the structure list got the spring block.
- 1:50 structure: arm frame as boards (front and back board 140 x 25, flat top rail) rather than posts with a separate front panel; no springs or webbing (ply platform kept); 1:200 and 1:500 arms are one plate in the arm's mid plane.
- Catalog `structure.assumptions[0]` still describes the cushion as one rigid body sinking into the frame; true for body and thermal, no longer for the structure list. Decisions written by the first round were extended, not re-read for sentences on the old bearing band.
- Sheets listed above as "rendered, not read".

### Fundamental issues (outside this asset)

- `volumes-render.ts` `thermalItems`: a thermal solid without `layer` in a region that has several layers gets no legend colour (grey) and nothing reports it; `validateAsset` accepts it. Either the validator should require `layer` there or the renderer should fall back to the outermost layer.
- A stray `git mv -n x y` (dry run on two non-existent names, nothing changed) was typed by mistake in one command of this pass.
