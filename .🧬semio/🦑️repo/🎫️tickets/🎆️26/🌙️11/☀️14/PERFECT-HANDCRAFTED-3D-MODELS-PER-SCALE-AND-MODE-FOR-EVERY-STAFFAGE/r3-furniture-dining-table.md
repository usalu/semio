# Round 3 — furniture/dining-table

State: finished and spliced (2026-11-20). No earlier round-3 work existed for this asset (no `r3-dining-table-*` scripts, no report). The asset was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts (ticket folder): `r3-dining-table-kit.py` (the measures, world-to-bone helpers, tube section, plate outlines, masses, corrected rig), `-body.py` (every body solid of every scale stated by hand), `-lists.py` (thermal, structure, volume decisions), `-roots.py` (anchors, mass, references, notes, structural assumption, slide area, symbol), `-draw.py` (the 2D shapes and representation decisions of every scale, ruling E1), `-build.py` (candidate under `🗑️generated/r3-dining-table/` and `volumes/adhoc/r3-dining-table.json`), `-apply.py` (writes the validated candidate's root members into the catalog file in its own formatting and copies the volumes source; `volumes` then goes in through `volumes-insert.py`). Re-running build and apply changes nothing (checked: "members: none"). Scratch output under `🗑️generated/r3-dining-table/` and `volumes/adhoc/r3-dining-table.json` was deleted; the renders stay in the tools' own folders `🗑️generated/r3-sheet/furniture/` and `🗑️generated/r3-2d/furniture/`.

## 1. Reference

Subject: a solid oak extending dining table, 1.60 x 0.90 x 0.75 m closed, 2.20 m with the leaf. The table is split in the middle. Each half carries two legs and moves on synchronised slides. The 0.60 m leaf is stored under the top and rises into the opening.

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Overall size | 160 x 90 cm, height 75-76 cm, extension leaf 50-60 cm | Höffner listings of solid-wood tables 160 x 90 (articles 40305726, 13302906, 50308493), asset dimensions | 1.60 x 0.90 x 0.75 m, leaf 0.60 m (kept) |
| Top thickness | 20-27 mm in the listings, 30-40 mm on heavier solid-oak tables | Höffner listings, carpentry practice | 30 mm (asset layer `top-plank` 30 mm) with a 15 x 15 mm underside bevel on the free edges, square joint edge |
| Knee clearance | underside heights of 60-75 cm in the listings; 600-650 mm usual minimum for a dining table | Höffner listings, ergonomics practice | 0.61 m under the apron |
| Aprons | 20-25 mm boards, 70-100 mm deep, tenoned into the legs | carpentry practice | 25 x 80 mm, 20 mm tenons, set back 10 mm from the leg face |
| Legs | 60-80 mm square, often tapered to about 2/3 at the floor | carpentry practice | 70 mm square under the apron (0.58-0.72 m), tapered on all four faces to 45 mm at the floor; plan centres at x 0.135 m from each end, z +-0.375 m (asset anchors) |
| Extension slides | steel telescopic or ball-bearing table slides for one leaf in frame tables, synchronous; outer steel profile with an inner profile, about 850 mm for a 500 mm leaf | Häfele table fittings (hwt-pro article 5765), DABAG data sheet 68_1242_30 | 25 x 12 mm tube of 2 mm sheet on each long apron (0.62 m per half), a stationary 21 x 8 mm centre bar per side, 1.00 m long, running in the tubes of both halves (overlap 0.19 m at full extension) |
| Leaf storage | leaf stored under the top in the frame, lifted into the opening when the halves are parted | asset rig (leaf rises 30 mm), product practice | the leaf lies in a 30 mm shadow gap between top and aprons, on the aprons and the tubes, and rises 30 mm flush |
| Oak, steel | 700 kg/m3, 0.18 W/mK; steel 7800 kg/m3 | ISO 10456, Wood Handbook (asset references) | masses from the modelled volumes |

Sources: https://www.hoeffner.de/artikel/40305726 , https://www.hoeffner.de/artikel/13302906 , https://www.hoeffner.de/artikel/50308493 , https://www.hwt-pro.com/en/furniture-fittings/table-fittings/table-extensions/5765/haefele-ball-bearing-guide-for-1-insert-plate-asynchronous-or-synchronous-for-frame-tables , https://bilder01.dabag.ch/documents/68/68_1242_30.pdf (both new reference ids are in `provenance.references`).

## 2. Audit of the state before round 3

- Rig. `top-left` and `top-right` lay on the floor (y 0). Their structural bars, the distributed dinner loads and the moment diagrams were therefore drawn along the floor, 0.735 m below the top they stand for. `frame` (the slides) had its joint at the floor origin (x 0, y 0), while the rails are at x 0.30-1.30, y 0.68. `ground` (the floor reference) carried a length of 1.60 m, so it became a structural member. The moments view showed a "Floor reference 117 N·m" bar, and the old structure list hung the legs on it.
- Masses. The half centre was at comY 0.38 m (world). The modelled oak puts it at 0.667 m: the 30 mm top dominates.
- Mechanism (the root defect of the construction). The stored leaf (0.60 x 0.88 m at y 0.69-0.72) passed straight through both long aprons (y 0.64-0.72 at z +-0.36-0.38). A one-piece leaf of the table's depth cannot lie between aprons that hang under the top. The cleats that "bridged" runner and plate stood inside the same space.
- Leaf width 0.88 m against a 0.90 m top: a 10 mm step each side in the extended table.
- Leg feet were 45 x 50 mm (front drawing and anchor footprint 0.05 m) on a square leg.
- Solids (1:50): box aprons centred in the leg, a 20 mm apron with corner blocks, 7.6 mm steel strips as rails, legs "on the ground bone" in structure. The structure reused the body wholesale (`of` on every body solid). There were no tenons, slides or masses. 1:500 supports were oversized blocks (qa-visual: grade B in every list).
- 2D. The 1:50 elevation drew the apron directly under the top with the leaf behind it. The 1:200 and 1:500 elevations drew a 0.09-0.105 m "leaf bar" that stuck out of the table in the extended pose: 0.66-0.765 m against a top at 0.75 m.
- Thermal. `slide-rails` was 0.15 m2, the area of two thin strips; a real slide set is larger (section 3).

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `rig` `ground.lengthM` | 1.6 | 0 (a reference, not a member) |
| `rig` `frame` x, y | 0, 0 | 0.30, 0.6775 (start and axis of the centre bars); mass 5.0 kg at comX 0.8, comY 0.69 -> 2.6 kg at comX 0.5, comY 0 |
| `rig` `top-left`, `top-right` y | 0 | 0.735 (mid plane of the top: the beam the loads act on); mass 22.1 kg at comX 0.4, comY 0.38 -> 22.9 kg at comX 0.357 / 0.443, comY -0.068 (world 0.667) |
| `rig` `leaf` mass | 11.3 kg | 11.2 kg (0.60 x 0.90 x 0.03 m with bevels x 700) |
| `dimensions.massKg` | 60.5 | 59.6 (sum of the bones; height, width, depth unchanged) |
| `anchors` leg contacts | y 0, footprint 0.045 x 0.05 | y -0.735 (same floor point in the moved bone frame), footprint 0.045 x 0.045 |
| `anchors` `edge-right` | y 0.75 | 0.015 (same world point, 0.75 m) |
| `thermal.regions[slide-rails].areaM2` | 0.15 | 0.20. Hand sum: four tubes 4 x 0.62 m x 0.074 m perimeter = 0.184, plus open bore ends and the visible centre bar about 0.016. The model measures 0.215 (+7.6 %). This is within the factor 1.5 of C16, so it is applied and not sent to the area reviewer. |
| `structure.assumptions[0]` | halves on rails, leaf lifts "onto the rails" | halves slide in the plane of the top on the synchronised slide, centre bars stay, leaf rises from the shadow gap and is dowelled to both halves (en, de) |
| `provenance` | 9 references, notes | + `hoeffner-oak-160x90`, `haefele-table-slides`; notes rewritten to the construction (en, de) |
| `symbols[0]` | apron at 0.64-0.72, straight legs, rail at 0.66 | apron 0.61-0.69 under the shadow gap, tapered legs to the top, rail at 0.6775; description updated |
| `representations[*].geometry` | 10/7/9, 8/5/7, 5/4/3, 5/4/3 shapes | 21/10/6, 8/6/6, 8/6/3, 5/4/3 (elevation/front/plan at 1:50, 1:100, 1:200, 1:500), ruling E1 |
| `representations[*].decisions` | old texts | rewritten per scale (en, de) |

Unchanged: poses, clips, loads, contacts, the other areas, summary and title. The catalog index entry has no mass. No other asset references the dining table.

E1 (2D): every shape is drawn from the same measures as the solids of its scale. The elevation at 1:50 has the top with its end bevel and crease, the open shadow gap with the four blocks per half, the leaf in the middle bay, aprons with grain, tapered legs with the shoulder line, and the centre bar. That bar lies behind the apron when the table is closed and shows under the leaf when extended. The front view (the right end, nearer) has the bevelled top, the gap with the two end blocks in front of the leaf end, the end apron and two tapered legs. The plan has the two halves and the leaf below them, plus the stave joints (5 staves of 180 mm) as texture. The 1:500 leaf bar no longer leaves the table in the extended pose.

## 4. What was modelled (solid counts: listed / expanded with repeats)

### 1:50 — body 23 / 34, thermal 23 references (to the 34 expanded body solids), structure 39 / 48
- Body. Each half has:
  - a top in two plan extrusions: the upper 15 mm full size, the lower 15 mm as a tapered extrusion about the joint centre, which makes the 15 mm bevel on the three free edges with the joint edge square;
  - two legs as superelliptic lofts (exponent 10, eased arrises), 45 mm at the floor, 70 mm from 0.58 to 0.72 m;
  - long aprons (0.15-0.80 m, tenons inside the legs) and the end apron (tenons into both legs);
  - four oak blocks in the gap (two per long apron outside the leaf bay, z 0.365-0.39, recessed 10 mm) and two on the end apron;
  - two steel tubes (extrusions with a bore, 2 mm walls).

  The leaf is two plates with the same bevel on its long edges and square ends. Two 21 x 8 mm centre bars, 1.00 m long, sit in the bores.
- Thermal. Every body solid is referenced into its region with the region's one layer (`top-plank`, `leaf-plank`, `apron-board` for aprons and blocks, `leg-solid`, `rail-steel`). There are no hidden regions and no emitters.
- Structure.
  - Members: top plates, aprons, legs and tubes per half (each with its own bone); the leaf plates; the centre bars (`frame`).
  - Joints: 20 mm tenons (long and end aprons, both sides), the gap blocks, 100 mm slide shoes at the inner tube ends where tube and bar overlap in every pose, and the alignment dowels in the joint edges. The left half carries three dowels at z 0 and +-0.30, the right half two at +-0.15. Closed, they enter the other half; extended, they enter the two ends of the leaf.
  - Supports: four felt glides of 36 mm diameter under the legs, one per anchor.
  - Masses: one per moving bone at its centre.

### 1:100 — body 10 / 15, thermal 10 references, structure 26 / 35
Square-edged 30 mm halves and leaf (the bevel is one feature), shadow gap with the stored leaf, 25 x 80 mm aprons, tapered legs, one 18 x 20 mm steel slide per side (tube and bar merged). The blocks are omitted. Structure: members by reference; tenons, one 15 mm slide link per half and side, dowels of 15 mm diameter, glides, masses.

### 1:200 — body 10 / 15, thermal 10 references, structure 22 / 26
30 mm plates, the gap of one feature, 30 x 80 mm aprons, straight 60 mm legs (the 25 mm taper is below the feature), one 30 mm slide bar per side. Structure: members; one 60 mm frame corner per leg and one slide link per half and side as joints; glides; masses.

### 1:500 — body 7 / 9, thermal 8 / 9, structure 16 / 17
- Per half: one 75 mm slab for top and gap, a U frame of the three aprons below it (75 mm walls, 0.60-0.675 m), two 75 mm legs.
- The leaf is a 75 mm block, 0.78 m wide, that lies inside slab and frame while stored and fills the opening flush when extended.
- Thermal adds one hidden 75 mm slide bar per side inside the frame.
- Structure: slabs, frames, legs, leaf, the two slide bars, 75 mm glides (the leg section), masses.

`decisions`: two bilingual sentences per scale (body; thermal and structure).

## 5. Checks (final, on the catalog asset)

- `volumes-check.ts furniture/dining-table --product`: `PASS (5.4 s)`.
- Silhouette IoU elevation / front / plan: 1:50 0.997 / 0.996 / 1.000; 1:100 1.000 / 1.000 / 1.000; 1:200 1.000 / 1.000 / 1.000; 1:500 1.000 / 0.997 / 1.000. The p95 boundary distance is 0 everywhere.
- Thermal outer area against `areaM2`:
  - 1:50: table-top 3.055 (+1.8 %), leaf-top 1.222 (+7.2 %), apron 0.933 (+1.4 %), legs 0.678 (-9.7 %), slide-rails 0.215 (+7.6 %).
  - 1:100: all within 24 % (the merged slide 0.153 against 0.20).
  - Totals: 1:200 +1.2 %, 1:500 +19.6 %.
- `r3-validate.ts furniture/dining-table`: `VALID ... schema 0, validateAsset 0`. The candidate with `--volumes` gave `checkVolumes 0`. The catalog `volumes` equal `volumes/furniture/dining-table.json`.

## 6. What the final renders show (opened with the Read tool)

Seen:
- `r3-sheet` body at 1:50 (six views, with rig), 1:100, 1:200, 1:500; 1:50 and 1:500 in the pose `extended`; 1:50 from below (turn 30, tilt -30);
- thermal cut at 1:50; structure at 1:50 (six views and from below);
- `r3-2d` at 1:50 (and enlarged crops of elevation and front), 1:500, 1:50 `extended`, and 1:50 `edge-test-load` forces and moments.

Rendered but not opened: 1:100 structure and thermal, 1:500 structure, 1:100 and 1:200 2D sheets.

- Front (from the right end): a bevelled top over a dark shadow gap with the two end blocks and the leaf end behind them, the end apron, and two legs that taper from under the apron to the floor. It reads at once as a solid wood dining table.
- Right and left side: a thin top whose bevelled ends make it look lighter. Under it is the open gap, with the stored leaf filling the middle bay, two blocks per half and the leg tops at the corners. Then the aprons meet at the centre joint, and the legs are tapered with the shoulder line at 0.58 m.
- Top: two halves with the centre joint.
- Three-quarter: a credible oak table with an underside bevel and a floating top.
- From below: aprons tenoned between the legs, the blocks, the steel tubes on the inner apron faces with the centre bar showing at the joint, and the underside of the stored leaf in the bay. Nothing floats or pierces anything else.
- Extended: the halves part by 0.30 m each. The leaf sits flush between them, with the centre bars visible under it where the aprons end. In 2D the leaf and its stave joints show in the plan, and the slide bar shows below the leaf in the elevation.
- Thermal cut: red halves, green leaf in the gap, violet aprons and blocks, yellow legs, cyan tubes.
- Structure: blue members, orange blocks, slide shoes and tenons, green glides under the feet, magenta masses under the top.
- Moments (edge test): the bars of both halves and the leaf now run along the top at 0.735 m. The floor-reference bar is gone.
- 1:100 to 1:500: the same table, simplified scale by scale. 1:500 is a slab on a set-back frame with four legs, and stays recognisable down to the 3.2 mm sign.

## 7. Still imperfect

- **Leaf support when extended.** Raised flush, the leaf is held only by the alignment dowels of the two halves and by friction. Its underside is 30 mm above the centre bars. Real tables of this type add a lift bracket or a table lock. The rig has no bone for one, and in the stored pose it would have to share the slot with the tubes, so none was modelled.
- **No apron under the leaf.** The long aprons end at the centre joint, so in the extended pose the 0.60 m bay has no apron. Some products give the leaf its own short aprons. This one does not, because a stored leaf with aprons would not fit the 30 mm gap.
- **Long aprons are cantilevered.** They run 0.63 m from the leg to the joint and are tied to the top only by two blocks outside the leaf bay. That is true to this construction, but less stiff than a frame closed at the joint.
- **Plan drawing at 1:50.** Short segments of the top outline over the leaf ends draw slightly heavier, because the leaf outline under the top coincides with the top edge. This is the product's line merging, not the geometry.
- **1:500 leaf.** It is 60 mm per side narrower than the top (below the 75 mm feature), so the extended 1:500 plan shows small notches when magnified.
- **Omitted fittings.** Screws, table locks and the slide balls are not modelled. Felt glides exist only as structural supports, because 4 mm pads would lift the legs off their anchors.
- **Test load.** The 750 N edge load remains an authored estimate, as the asset already states.
- **Not run.** The repo test suites were not run, only the two checks of the brief.

## Round 5 handcraft (2026-11-21)

Sheets rendered with `sh r5-sheets.sh furniture/dining-table before "closed extended set-for-dinner edge-test-load pulled-apart-key" "extend-table 1"`. Opened with the Read tool: body 1:50 (six views with rig), 1:50 structure, 1:50 thermal cut, 1:50 `extended`, a montage of body 1:100 / 1:200 / 1:500 / 1:500 `extended`, a montage of structure and thermal cut at 1:100 / 1:200 / 1:500, the 2D at 1:50, a montage of the 2D at 1:100 / 1:200 / 1:500 with the poses `set-for-dinner` and `pulled-apart-key` and the clip `extend-table` at 1 s, and the 1:50 body from below (turn 30, tilt -22). Measurements: `r5-furniture-extent.ts furniture/dining-table 50`.

### Reference against model (1:50, pose closed)

| Quantity | Reference (source as in section 1) | Model | Deviation |
| --- | --- | --- | --- |
| Length x depth x height | 1600 x 900 x 750 mm (asset, Höffner 160 x 90 listings) | 1600 x 900 x 750 | 0 % |
| Top thickness | 30 mm (layer `top-plank`; 20-40 mm in listings) | 30 (15 + 15 bevelled) | 0 % |
| Underside of apron (knee clearance) | at least 600 mm | 610 | in range |
| Apron | 25 x 80 mm | 25 x 80 (0.61-0.69 m) | 0 % |
| Leg | 70 mm square under the apron, 45 mm at the floor | 70 / 45 | 0 % |
| Leg centres | x 0.135 m from the ends, z +-0.375 m (anchors) | 0.135 / +-0.375 | 0 % |
| Leaf | 600 x 900 x 30 mm | 600 x 900 x 30 (621 with bevels) | 0 % |
| Slide tube / centre bar | 25 x 12 mm tube, 21 x 8 mm bar, 1.00 m | 25 x 12 / 21 x 8 / 1.000 | 0 % |

### Grades (before = after; no geometry change)

| Scale | Body | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | A | A | A- |
| 1:100 | A | A | A- |
| 1:200 | A | A | A- |
| 1:500 | A- | A | A- |

What I saw:
- 1:50 reads at once as a solid oak extending table: bevelled top, shadow gap with the blocks and the stored leaf, aprons tenoned between tapered legs. From below you see the tubes on the inner apron faces, the centre bar at the joint and nothing floating.
- The `extended`, `set-for-dinner`, `pulled-apart-key` and `extend-table` frames stay closed: the leaf rises between the halves and nothing passes through anything.
- The coarser scales are clean step-by-step generalisations: square plates, straight legs at 1:200, and a slab-and-U-frame at 1:500.
- Thermal regions are tidy at every scale. The structure is the timber itself (D3 allows this: the top, aprons and legs are the members), with tenon and slide joints. The glides and masses are inside the legs and the top, so they cannot be seen.

Checks: `r3-validate.ts furniture/dining-table --volumes` gives `VALID ... schema 0, validateAsset 0, checkVolumes 0` (print suitability included). `volumes-check.ts --product` gives `PASS` with 41 rule lines ok.

Changes: none. Every measure is within 3 % and I found no visual defect at grade B or worse.

Open points:
- The leaf held flush in the extended pose sits 30 mm above the centre bars. Only the dowels and friction hold it, and there is no lift bracket. A bracket needs its own bone, and any cleat under the leaf collides with the tubes in the stored position. I checked this against the measured extents.
- The structure grade is A- only because supports and masses are hidden inside the members.

## 21 November lift mechanism and stored clearance

The dining table now has four captive screw lifts on two stationary crossheads. The digital and printed 1:500 boards keep their 30 mm thickness and the stowed leaf clears the tops. Four lift stations passed 121 positions at all four scales, strict product validation passed, and an independent reader reopened all twelve presentation scenes in a 3,849,244-byte GLB. The C-section runners have a 12 mm inward slot for the crosshead brackets. The two stationary crossheads span the centre bars and hold four shouldered nuts. Four 16 mm posts lift the leaf at x = 640 and 960 mm and z = ±245 mm. The screw heads bear against the underside through both animation phases. The fine crossheads have five lightening pockets apiece. The fine triangulated steel volumes and the material density account for the change to a 64.65 kg table. This is a modeled mechanism and its load estimates have not been tested on hardware.

The language-independent dimensions are in [digital-leaf-case.json](digital-leaf-case.json). [digital-leaf-check.mjs](digital-leaf-check.mjs) checks contact, minimum nut engagement and the stored boards with Three.js. Shipped input SHA-256 a2fd99e3124a01c712bbc4ffaa877c81cefa39d228504453b89ba049b5cee264.
