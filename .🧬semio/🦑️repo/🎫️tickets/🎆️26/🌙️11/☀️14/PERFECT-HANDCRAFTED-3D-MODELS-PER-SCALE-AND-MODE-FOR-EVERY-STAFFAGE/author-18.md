# Author 18

Sources `volumes/components/<slug>.json`, generators `author-18-garage.py` (uses `author-18-lib.py`, which builds on `author-17-lib.py`), 2D root fix `author-18-fix2d-garage.py`, area helper `author-18-areas.sh`. Check = the product engine (`volumes-check.ts` without `--fast`), Solid counts are body / thermal / structure (entries, repeats not expanded).

## components/sectional-garage-door — spliced

| | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| solids | 44 / 48 / 24 | 29 / 29 / 19 | 13 / 17 / 17 | 13 / 17 / 18 |

Check (all four scales, product engine): passes every rule except the four lines below.
- `volumes-feature` 1:50: the eight thermal skin slabs (0.5 mm steel skins, declared layer thickness, B2) are below 7.5 mm; request in `author-requests.md` (`## 18 components/sectional-garage-door`). Body (8 mm) and structure (10 mm) pass.
- `volumes-thermal-coverage` 1:50: `track-steel` exposed 3.18 m2 vs `areaM2` 1.9 (ratio 1.67, not applied per C16, listed in `area-review.md`). Panel 1 %, hardware 8 %, spring 0 % (after the applied correction).
- `volumes-views-disagree` 1:200 / 1:500: tool counts only filled shapes (the elevation of these scales draws the panels and the track as open strokes); `tool-requests.md`. Not a 3D matter.
- Everything else passes: silhouettes IoU 0.97-0.99 / 0.98-0.99 / 0.88-0.95 (elevation / front / plan, plan weakest at 1:100 and 1:500 where thin strokes decide), bounds, monotonic, cross-scale, slender, copied, structure coverage with a support solid per anchor.

### What I modelled by hand (1:50)
Four sandwich panels as extrusions over 2.42 m with the V-chamfer joint groove, panel 1 above its 70 mm seal profile (rubber bulb in a retainer, one polygon), one embossed rib per panel (texture solid, as the 2D draws it), hinges as two leaves on the two joined panels plus a knuckle (four per joint, 12 in all), rollers on both sides of every joint plus the bottom roller on its L bracket, guide tracks as true 40 x 40 mm C-sections of 2 mm sheet (vertical extrusion, R 0.5 m quarter circle as a swept closed-profile revolve, horizontal), hollow ceiling hangers, end tie bar, shaft with grooved cable drums, the torsion spring as 122 real wire turns (8 mm tori, repeat) and the two bearing flags outside the tracks (disc round the shaft, strap down to the curve). Thermal: three stacked slabs per panel (outer skin 0.5 / PU core 39 / inner skin 0.5 mm), everything else referenced from the body. Structure: the panels and the track parts as members (`of`), hinge knuckles and bottom roller as joints, flags as shaft bearings, the coil as a mass, supports by type (fixed plates outside both rails at the floor, roller pins through the hangers, one solid each with a mirrored repeat).
Coarser scales are their own models: 1:100 merged hinge blocks, L-shaped one-piece track bars, corrugated sleeve for the coil; 1:200 slender shaft, 30 mm flags, thermal hinge strips inside the panels; 1:500 75 mm panel boxes, slender straight track bars with a 75 mm arc, no hangers/flags/spring (supports stay as 75 mm blocks).

### Roots fixed in the asset (2D and values)
- 2D 1:50/1:100: the two open edge strokes of the track (`track`, `track-b`) replaced by one closed filled outline `track-band` (an open pair cannot match a 40 mm rail in a fill comparison), hangers drawn as 40 x 350 mm rectangles (were pen lines), added the bearing flag (elevation polygon, front and plan rectangles), shaft end stubs and (1:50) the end tie bar in front.
- 2D 1:200: hangers and flag added to the elevation, flag to front and plan. 2D 1:500: drums added to the plan (front and elevation already drew them).
- `areaM2` `spring-assembly` 0.6 -> 0.85 (ratio 1.42, within the factor 1.5). `track-steel` stays 1.9 (see above).

### What I saw in the renders
Sheets of 1:50, 1:100, 1:200, 1:500 looked at, plus close-ups of the 1:50 body from inside (hinges, rollers, bottom brackets) and from outside, and the 1:500 plan at 4000 px. Arcs, flags, hinges, spring, nested thermal slabs and supports read correctly; the quarter-circle track was a filled disc in the first draft (open revolve profiles are closed to the axis), fixed by closing the loop.

### Left imperfect, stated plainly
- Cable, handle and lock are drawn at no scale and not modelled.
- 1:200/1:500: the spring shaft floats on the drums only at 1:500 (no flags there, as in the drawing).
- Plan IoU at 1:500 is 0.93 and depends on modelling the rails 75 mm across (product measure takes the stroke width from the member).

## components/sliding-door — spliced

| | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| solids | 19 / 27 / 11 | 8 / 8 / 8 | 5 / 5 / 7 | 4 / 5 / 7 |

Check (all scales, product engine): every rule passes (silhouettes elevation 1.0 / front 0.99-1.0 / plan 0.89-1.0 as section views, bounds, feature, monotonic, cross-scale, slender, copied, structure coverage incl. both supports) except `volumes-thermal-coverage` areaDeviation: `leaf-frame` 3.3, `track-steel` 3.7, `jamb-steel` 3.1 at 1:50 (and at 1:100; total 1.11 at 1:200, 0.85 at 1:500; glass 0 %). The declared areas (0.28 / 0.3 / 0.42 m2) are one face of each member (length x 50 or 100 mm); the closed hollow-section bodies expose 1.21 / 1.40 / 1.74 m2 (ratios 4.3 / 4.7 / 4.1, beyond the C16 factor 1.5), listed in `area-review.md`; not applied.

What I modelled by hand (1:50): jamb posts 100 x 100 mm with a rebate and a slender brush-seal strip in it, track housing 100 x 84 mm with its running slot and four domed 8 mm screw heads, the leaf as two stiles, two rails (50 x 50 mm), the 10 mm pane, two carriage hangers reaching into the slot with bolts, handles on both faces (plate + stand-off grip), the 20 mm bottom seal. Thermal: every steel member nested as skin-b / cavity / skin-a (the 50 mm cavity of the layer table does not fit the profiles: modelled 42 / 34 mm with the remaining core as skin-a, stated in the decisions), pane with `sectionFaces` -w. Structure: real hollow sections (RHS 100 x 50 x 4 track, 100 x 100 x 4 posts, 50 x 50 x 4 leaf members), carriage joints, a pin at the west and a roller at the east jamb head. 1:100 merges frame members into one window extrusion, 1:200 drops hangers/handle/seal, 1:500 is the portal plus one leaf block.

2D roots fixed in the asset: the plan is a horizontal section at 1 m (A-A) at 1:50 / 1:100 / 1:200: the outlines of the parts above the cut (`track-above`, 1:50 also `hanger-above-a/b`) were role `detail` strokes that the 3D section cannot contain; they are now role `texture` (pen 0.1 mm, set at 1:200). At 1:500 the plan draws the portal and the leaf as filled shapes of a plain top view, so the false section declaration (plane 1) was removed there (`sections` is `{}`).

Renders: sheets of 1:50, 1:100 and 1:500 looked at (after the section fix, the plan overlay of 1:50 is 0.89 with the leaf strip as the only blue area: handle grips outside the drawn 50 mm leaf depth); 1:200 rendered, numbers pass.

## components/solid-brick-wall-1960 — spliced

| | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| solids | 27 / 3 / 14 | 2 / 2 / 2 | 2 / 2 / 2 | 1 / 1 / 2 |

Check (all scales, product engine): PASS, every rule. No `areaM2` changed.
Modelled: 1:50 the wall as built, 15 mm lime plaster, 13 brick courses of 65 mm (365 mm) with the 12 mortar bed joints of 10 mm as lighter solids, 20 mm render; thermal as three stacked layer slabs (plaster and render referenced from the body, one brick slab), the cut edges `sectionFaces` -v/+v/-w/+w; structure the 13 courses as compression members and a 7.6 mm bearing block at the base anchor. 1:100 plaster merged into one 380 mm slab plus the 20 mm render, 1:200 370 mm masonry plus a 30 mm render skin (the smallest feature), 1:500 one block. At the coarser scales the thermal slab that is not the outer face carries `sectionFaces` -u, because the product counts every thermal solid of a region at 1:100 and coarser (the core slab would add its room face: measured 2.0 m2 instead of 1.0 before the fix). Decisions at 1:200 and 1:500 start with "Minimal:" for the equal counts.
Renders: sheets of 1:50 and 1:200 looked at (elevation/front/plan IoU 1.000, nested thermal slabs and member courses read correctly); 1:100 and 1:500 rendered, numbers pass. The head joints are not modelled (inside the cut, the 2D draws none): stated in the 1:50 decision.

## Not started
- none (all three assets spliced); open items are the failing lines named per asset above.
