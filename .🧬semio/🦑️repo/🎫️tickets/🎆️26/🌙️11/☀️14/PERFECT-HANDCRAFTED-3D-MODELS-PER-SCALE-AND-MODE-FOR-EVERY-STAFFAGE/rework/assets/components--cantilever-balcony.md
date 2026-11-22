# components/cantilever-balcony — rework report

## Pass 1 (2026-11-22)

`rework/18.md` has no section on this asset; the state found is the one of rounds 3 and 5.

### Reference (real subject: one-metre strip of a cantilevered RC balcony on an ETICS wall)

| item | real value | source |
|---|---|---|
| cantilever span | 1.2 to 2.0 m, here 1.5 m | usual residential balcony |
| slab thickness | 180 to 250 mm, about span/8 to span/10 at the root; 200 mm | EN 1992-1-1 7.4.2 slenderness of cantilevers |
| drainage fall | 1.5 to 2 % away from the wall | DIN 18531-5 |
| arris chamfer | 10 mm triangular fillet in the formwork | site practice |
| drip groove | 15 to 25 mm wide, 10 to 15 mm deep, 30 to 80 mm behind the edge | site practice |
| thermal-break element | insulation body 80 or 120 mm, tension bars 8 to 14 mm at 100 to 250 mm, pressure bearings, diagonal shear bars 6 to 8 mm | Schöck Isokorb T type K / K-U, Halfen HIT |
| wall | 180 to 250 mm RC, 100 to 200 mm ETICS mineral wool, 15 to 20 mm render | EN ISO 6946 build-up |
| guard height | at least 0.90 m, 1.10 m from 12 m fall height | MBO § 38, DIN 18065 |
| clear opening of the infill | at most 120 mm between bars and under the lower rail | DIN 18065 6.9, ETB guideline |
| post | SHS 40 to 60 mm at 1.0 to 1.5 m, base plate 120 to 160 mm square, 10 to 15 mm thick, four M10/M12 anchors, edge distance at least 60 mm | EN 10219, anchor approvals |
| handrail | CHS 42.4 to 60.3 mm | EN 10219 |
| infill | 12 mm round or square bars between two flats 40 x 8 to 40 x 10 | metalwork practice |
| barrier load | 0.5 to 1.0 kN/m at handrail height; imposed 4 kN/m² | EN 1991-1-1 6.3.1.2, 6.4 |
| smallest drawable feature | 7.5 / 15 / 30 / 75 mm at 1:50 / 100 / 200 / 500 | brief |

### Measured against it (state found)

| scale | list | finding |
|---|---|---|
| 1:50 | body | right: slab 200 to 177.5 mm with 1.5 % fall, chamfers, drip groove 25/15 x 15 at 70 mm, break module 120, wall 180 + 100 + 20, post SHS 50 x 50 x 4 hollow with rounded corners 120 mm inside the edge, plate 150 x 150 x 12 on grout, four M10 with nuts at 65 mm edge distance, handrail 60.3 x 2.9 with its top at 1.10 m, eight 12 mm balusters at 106.9 mm (95 mm clear), lower flat 80 mm clear above the slab. Nothing floats, all six views agree. |
| 1:50 | thermal | reads as the same object; the three slab skins (80 mm, matching `heatCapacityJM2K`) leave a 20 to 38 mm core band without region, visible only in the cut. Kept, stated under "remaining". |
| 1:50 | structure | real load path (tension bars bent into the wall, pressure bearings, shear bars, slab mesh, hairpins, anchors). Missing: the mass of the railing. |
| 1:100, 1:200, 1:500 | body | the railing was the 1:50 railing copied unchanged: hexagon nuts 17 mm, 5 mm studs, 8 mm cap, 2.9 mm tube wall, 4 mm post wall, 10 mm flats and 12 mm balusters at a 75 mm feature size, while the 2D drawings of the same scales were generalised correctly (foot, post, line). |
| 1:200, 1:500 | thermal | wall and break drawn as solids without a layer although their regions are layered, so the sheet shows them grey; railing as above. |
| 1:100 to 1:500 | structure | no mass solids; railing joint still four 10 mm studs at 1:200 and 1:500. |
| 1:100 to 1:500 | decisions | one generic sentence about the 1:50 railing, identical at every scale. |
| rig | | three bones (slab fixed at the wall, railing fixed on the slab, prismatic load carrier): right for a cantilever strip, joints at the mechanical points (wall face, post foot), nothing to add. |

### Rebuilt in pass 1

Scripts (the stated numbers): `rework-cantilever-balcony-pass1.py` (volumes source), `rework-cantilever-balcony-2d.py` (1:500 front view), `rework-cantilever-balcony-look.sh` (tiles one view over the scales).

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | unchanged (measured right) | slab top, soffit and edge now fill the whole concrete section (meet on the mid-plane 110 mm under the root surface and on two mitres 160 mm behind the face); no void in the cut any more | added `mass-railing` (24 kg, 566 mm up the post); reinforcement cage rendered without the slab member and read: tension bars, bearings, shear bars, mesh, hairpins and anchors all sit inside the concrete with cover |
| 1:100 (15 mm) | railing re-authored: foot 150 x 150 x 27 (grout and plate), closed 50 mm post into the handrail, closed 60.3 mm handrail, flats 40 x 15, eight 15 mm balusters at 106.9 mm; nuts, studs, cap, bracket, tube walls removed | filled slab regions; wall in three shells (the 20 mm render exceeds the feature); railing = body railing | foot as joint with four 15 mm anchors 90 mm deep, masses of slab and railing added |
| 1:200 (30 mm) | closed 50 mm post from the slab into the handrail, no foot, rails 40 x 30, eight 30 mm balusters (the 0.13 mm pen of the drawing) | filled slab regions; wall and break each in two layered shells (were unlayered, drawn grey) | 150 mm fixing block 90 mm into the slab as railing joint, masses added |
| 1:500 (75 mm) | 75 mm post, 75 mm handrail with its top at 1.10 m, one 75 mm bottom rail, two 75 mm balusters at ±340 mm; upper flat merged into the handrail | as 1:200 on the flat slab | 150 mm fixing block, masses of 150 and 120 mm |

2D: the drawings of 1:50, 1:100 and 1:200 already showed the generalisation the 3D now has (foot, post into the handrail, pen-line balusters) and were left; 1:500 front redrawn by hand (post 75 mm from the foot to the handrail, upper flat removed, the two balusters up to the handrail line, silhouette decision text en/de). Decisions of `volumes` rewritten per scale (body, thermal, structure; en and de).

Rig, poses, clips: unchanged. The three poses and three clips move only the massless load carrier and the load factors; rendered after the last change (self-weight, point-load-tip, loading-ramp t 1 / 5 s, moving-point-load t 1.5 / 4.5 s, sun-and-shade t 2 / 6 s) and read: body identical, the carrier joint travels from the wall to the tip along the slab axis.

### Validation

`r3-validate.ts components/cantilever-balcony --catalog-only --volumes` → `VALID components/cantilever-balcony (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after `volumes-insert.py` and the 2D edit).

### Remaining, stated plainly

- 1:100, 1:200, 1:500 thermal and structure were read after the last change as three-quarter views and as the cut right side (thermal) only, not as full six-view sheets; 1:200 body and 1:500 body were read as full sheets before the thermal fill (body unchanged since), 1:100 body after it.
- 1:500 body: the 75 mm bottom rail keeps its true centre line 85 mm above the post foot, so its clear gap above the flat slab is 27 mm instead of the real 80 mm; raising it would need the 2D chord line moved too.
- 1:50 thermal: the slab regions are now 87 to 110 mm thick while `thermal.regions[].heatCapacityJM2K` 184000 corresponds to a 73.6 mm storage skin (EN ISO 13786); the solids partition the slab, the capacity stays the effective one. Not reconciled in the thermal model.
- 1:50 body: no sealing upstand or plinth flashing at the wall foot and no door threshold (no material for it in the asset); the tension bars end at the section cut 140 mm below their bend (wall continues below the strip).
- 2D of 1:50, 1:100, 1:200 were read once (before the changes, they are untouched); 1:500 re-read after the redraw.

### Fundamental issues (outside this asset)

- `r3-sheet.ts --list thermal`: a solid of a layered region that names no `layer` is drawn in neutral grey with no legend entry and no warning, and `validateAsset` accepts it (found at 1:200 and 1:500 here); either the validator should require a layer on solids of layered regions or the sheet should colour them as the outer layer.
- `r3-sheet.ts --cut`, three-quarter panel: nested boxes of the wall show a triangular notch at the top corner of the cut (1:50 thermal), a clipping artefact of the tool, not of the solids.

## Pass 1 (second round) (2026-11-22)

State found: source `volumes/components/cantilever-balcony.json` and catalog `asset.volumes` identical (nothing half-edited), `r3-validate.ts --catalog-only --volumes` `VALID`. Reference table above kept; everything was rendered and read again at full size and judged against it.

### Read and judged (before any change)

| scale | list | sheets read | verdict |
|---|---|---|---|
| 1:50 | body | six views, plus a rear three-quarter close-up (turn 140, tilt 15) | signed: slab wedge with fall, chamfers and drip groove, break module, wall build-up 180 + 100 + 20, SHS post on grouted plate with four anchors and nuts, spigot and cap under the CHS handrail, two flats, eight balusters. Nothing floats or pokes through. |
| 1:50 | structure | six views, plus the cut right side | signed: tension bars through the break and bent down at the far face of the wall, diagonal shear bars from the slab bottom up to the wall top (the hanging direction), pressure bearings at the bottom, top and bottom mesh with edge hairpin, plate anchors 90 mm into the slab, masses at the centroids. |
| 1:50 | thermal | six views | reads as the same object, regions and layers coloured and named. |
| 1:100 | body, thermal, structure | six views each | right for 15 mm. |
| 1:200 | body | six views | defect: the wall was one mass although the 120 mm insulated skin is four features wide, the thermal list and the 2D elevation and plan of this scale show it, and the break module was drawn as its own part next to an undivided wall. |
| 1:200 | thermal, structure | six views each | right. |
| 1:500 | body, thermal, structure | six views each | defect: bottom rail 27 mm above the flat slab (neither open nor closed at a 75 mm feature; real clear gap 80 mm). |
| 2D | 1:50, 1:100, 1:200, 1:500 | all six panels each | agree with the 3D form; 1:500 front needed the rail line moved with the 3D rail. |
| rig | | | three bones (slab fixed at the wall, railing fixed at the post foot, massless prismatic load carrier along the slab axis 0 to 1.5 m): right for a cantilever strip; no lateral pivot exists in the subject, so `rig-lateral-rest.md` does not apply. |

### Changed (script `rework-cantilever-balcony-pass2.py`, the stated numbers)

- 1:200 body: `wall` replaced by `wall-core` (180 x 1300 x 1000) and `wall-insulation-render` (120 x 1100 x 1000, wool and render merged) so body, thermal list and drawing show the same build-up; body decision text rewritten (en, de).
- 1:500 body (and with it thermal and structure, which reference the body railing): bottom rail centre 85 → 138.2 mm above the post foot = 80 mm clear above the flat 200 mm slab; the two balusters start on the rail (175.7 mm) and are 849.3 mm long up to the handrail; body decision text says why (en, de).
- 1:500 2D front: bottom rail line and the feet of the two balusters 0.2643 → 0.3175 m.

### Proof

- `r3-validate.ts components/cantilever-balcony --volumes` → `VALID`, then `volumes-insert.py`, then `r3-validate.ts components/cantilever-balcony --catalog-only --volumes` → `VALID components/cantilever-balcony (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Read after the last change: 1:200 body / thermal / structure, 1:500 body / thermal / structure, 1:100 thermal (six views each), the 2D of all four scales, and the right side at 1:50 of the poses `self-weight`, `point-load-tip` and the clips `loading-ramp` t 1 / 5 s, `moving-point-load` t 1.5 / 4.5 s, `sun-and-shade` t 2 / 6 s (the body is identical in all, the carrier joint travels on the slab axis from the wall to the tip; default pose `distributed-live-load` is the pose of every sheet). 1:50 and 1:100 lists were not touched in this round and were read before it.

### Remaining, stated plainly

- 1:50 thermal: slab regions partition the whole concrete section (87 to 110 mm) while `heatCapacityJM2K` 184000 is the effective 73.6 mm storage skin of EN ISO 13786; a modelling convention, not reconciled in the thermal model.
- The strip has no door threshold, sealing upstand or screed (no material for them in the asset; the subject is the bare structural and thermal detail), and the wall bars end at the section cut 140 mm below their bend.
- 1:500 structure: the 150 mm fixing block of the railing stands 55 mm proud of the flat slab so the joint stays visible; the body has no foot at this scale.

### Fundamental issues

None new; the two of the first pass (unlayered solid of a layered region drawn grey without a finding; notch artefact of `r3-sheet.ts --cut` in the three-quarter panel) stand.
