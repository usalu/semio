# Fauna: anatomical templates and the animal polish pass (`fauna-*`)

Run from `T` with the repo bun (`BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`; `./author-29-bun.sh` wraps it). Output only under `T/🗑️generated/fauna/`. Status of each asset is at the end (kept current after every asset).

## Pieces

| file | what |
|---|---|
| `fauna-lib.ts` | `class Fauna extends Zoo` (author 41/29 lofts constrained by the scale's own drawings) plus anatomical templates; `prof`, `PP`, `Lod` |
| `fauna-shape.py` | textual 2D patch of shape members (copy of author 29's tool plus `del`); `python3 fauna-shape.py <asset 🔣️.json> <patch.json> [--apply]` |
| `fauna-map.py` + `fauna-rules-<slug>.json` | `kit-thermal --map` string from regex rules over the body ids |
| `fauna-<slug>.ts` / `fauna-<slug>2d.ts` / `fauna-run-<slug>.sh` | per asset: 3D generator per scale, 2D root repair, the run chain (generate, thermal, emitter, structure, skeleton, check) |
| `fauna-dumpring.ts`, `fauna-probe.ts` | read the drawn rings of an asset / probe spans |

Per asset: `./fauna-run-<slug>.sh <50|100|200>` then `$BUN volumes-render.ts animals/<slug> --scale N`, look at the PNG, `python3 volumes-insert.py volumes/animals/<slug>.json "<asset 🔣️.json>"`.

## Anatomical sources the templates follow

Ellenberger/Dittrich/Baum, *Handbook of Animal Anatomy for Artists* (skeleton, ribcage, limb joints); Goldfinger, *Animal Anatomy for Artists* (surface forms of cat, dog, horse, deer, pig, sheep); Baumel et al., *Handbook of Avian Anatomy: Nomina Anatomica Avium* (keel, wing, tarsometatarsus); Proctor and Lynch, *Manual of Ornithology* (folded wing, remiges, tail); Olsen and Larsson, *Gulls of Europe, Asia and North America* and Cramp/Simmons (BWP) for herring gull and pigeon dimensions; breed and species standards for proportions.

## Templates (`Fauna`)

- `path(id, bone, pts[{p, rl, rs, e, roll}], look, {lat})`: loft along a 3D path, every section perpendicular to the tangent, `rl` along the lateral reference, `rs` perpendicular; the base of all organic parts. `blob(id, bone, at, [along, lateral, sagittal], look, dir)`: oriented ellipsoid (joints, eyes, knobs).
- Birds: `wing` (coverts and arm as a thick lens keeping the drawn thickness, carpal knob, secondaries and primaries as thinner sheets that end in a point, tertials; three levels of detail `lod` 2/1/0 = 1:50/1:100/1:200), `tail` (rectrix wedge thin in elevation, wide in plan, undertail coverts), `bill` (upper mandible with culmen and hooked tip above the gape line, lower mandible with gonys, tip stations so the drawn length is reached), `foot` (webbed or toed: web sheet, three toes, hind toe), `eye` (orbital ring plus eyeball).
- Every template reads its extents from the scale's own drawings (`ys`/`zs` span helpers over the shape ids) and only fixes the cross-section anatomy, so the silhouette rules hold by construction; coarser scales use fewer pieces (`lod`), never the finer solids with parts deleted.

## 2D repairs at the root (herring gull, applied to the asset)

Against Olsen and Larsson (bill: exposed culmen 50-62 mm, depth at gonys 20-26 mm, width 15 mm; tarsus 60-72 mm and about 8 mm thick; tail about 40 mm deep at the base): bill (elevation polygon, front ellipse, plan capsule) at 1:50 and 1:100 was 87 x 49 x 32 mm, now 82 x 26 x 17 mm; tibiotarsus, tarsus and foot at 1:50 (elevation, front, plan consistently): 21 mm thick strokes and a 21 mm tall foot are now 10-15 mm, 9 mm and 9 mm; the elevation tail at 1:50, 1:100, 1:200 was a 125 mm deep shield and is now a wedge 55 mm deep at the base and 16/18/30 mm at the tip; the folded hand (primaries) at all scales is a 18 mm thick sheet (was 30 mm like the arm; `solid` z-range of the hand shapes). The 1:100/1:200 leg strokes are pen widths (C24) and stay.

## More templates and tools added after the gull

- `Fauna.ear` (pointed leaf ear: broad toward the front as drawn, thin in x, tapering to the tip), `Fauna.ridge` (dorsal ridge following the drawn top edge), `Fauna.lateralPair` (paired flank masses just inside the drawn plan half width).
- `fauna-affine.ts` (`polyMap`: fits local to paper coordinates of any 2D `poly` shape from its own points and ring, so a repair is written in world coordinates for every view), `fauna-trunk2d.ts` + `fauna-<slug>-spec.json` (waisted plan outline and optional elevation outline for hindbody and chest, from knots), `fauna-quadtrunk.ts` (rebuilds hindbody and chest lofts from the repaired outlines, adds the dorsal ridge), `fauna-addth.py` (thermal references for new body solids), `fauna-joints.py` (moves generated joint solids onto the limb), `fauna-restore-struct.py` (restores a structure list from the spliced asset).
- Lessons for the next author: front and plan shapes with a `solid` (extruded) are the bbox of the elevation poly, so a repair of the elevation poly needs the same `points` in the front shape; a `path`-kind shape needs `kind: poly` plus `del: [d]`; the kit-structure regeneration puts joints at z = 0 (outside a body whose limb sits at z = +-offset) and is therefore not rerun on assets with hand-sized structure.

## Status (all spliced with `volumes-insert.py`; 1:500 untouched where it exists)

| asset | what was rebuilt | 2D repaired at the root | check under the current product validator |
|---|---|---|---|
| herring-gull | 1:50 41 body solids, 1:100 14, 1:200 13: bill with culmen/hook/gonys, eyes, tapered neck, stepped folded wing (coverts lens, carpal knob, secondaries, primaries, tertials), rectrix wedge tail with coverts, feathered thigh with knee, slim tibiotarsus/tarsus, webbed foot | bill 1:50/1:100, leg and foot 1:50 (three views), tail wedge 1:50/1:100/1:200, hand sheet thickness 1:50/1:100/1:200 | only `volumes-thermal-coverage` area findings of physiological regions: bill +140 %, legs-skin +167 %, web-feet +224 % at 1:50 (before +219/+499/+483 %) |
| pigeon | 1:50 41, 1:100 27, 1:200 13: wings as coverts lens and tapered primaries (shoulder at 1:100), rectrix wedge tails, bill with cere, eyes with ring, toed feet with hallux, joints moved onto the legs | tail wedge 1:50/1:100 (elevation and front), 1:200 kept as drawn (the thin 1:200 tail pushed the elevation IoU to 0.88) | only area findings of physiological regions (legs-feet, beak, cere, eye, as before) |
| domestic-cat | 1:50/1:100: leaf ears (flat, pointed, cupped) instead of cones, whisker pads; 1:200 unchanged (ears at the 30 mm floor must stay cones) | none | only the area findings that the spliced asset already had under the current validator (nose-skin, eyes, tongue, pads) |
| roe-deer | 1:50/1:100/1:200: trunk rebuilt from repaired outlines (flank tuck, deep brisket, waist, wider hips), elbow, carpus and hock masses at 1:50, dorsal ridge at 1:50/1:100 | elevation and plan outline of hindbody and chest, front ellipse widths | only area findings (hooves, nose-skin, 1:200 and 1:500 totals), all present before |
| pig | 1:50/1:100/1:200 trunk lofts from repaired outlines (rounded rump, tucked belly, hip and rib waist), dorsal ridge at 1:50/1:100 | elevation and plan outline of hindbody and chest | only area findings (ears, hooves, skin-head, skin-legs, snout, tail), all present before |
| sheep | same as pig (fleece) | same as pig | only area findings (ears, hooves, totals), all present before |

Not done: hedgehog (legs are still stubs because the 2D elevation draws none), goat, mallard-duck, red-fox, red-squirrel (not touched); labrador-dog, dairy-cow, horse (another author).

Imperfect, stated plainly:
- gull: the neck is a tapered tube, the head a faceted dome; 1:100/1:200 legs are pen-width tubes (C24 pens); the 1:50 web foot is a 9 mm slab (feature floor), so its exposed area stays above the declared 0.0037 m2 (finding for the area reviewer, C18).
- pigeon: 1:100 and 1:200 are coarse (1:200 follows a drawing whose neck is a straight column); the wing lens is still a smooth solid, individual remiges are not modelled.
- cat: the head still reads as a short rodent-like snout because the drawn elevation and plan head outlines were not redrawn (the pads and ears only help); limbs are the author-41 tubes.
- roe-deer, pig, sheep: the trunk now has a waist, tuck and rounded rump but the 1:100 and 1:200 trunks share the 1:50 outline knots (each scale's lofts are rebuilt from that scale's own repaired shapes, `volumes-copied` holds); pig and sheep still read as barrels from the side because the legs, shoulder and ham of the 2D drawings are not modelled as separate masses; the ridge is below the visual noise at these sizes.
- the 2D drawings of the front and plan views remain constructed drawings (C8a): where they contradict the elevation I did not redraw them beyond the outlines listed above.
