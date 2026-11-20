# Round 3 — components/wheelchair-ramp

No earlier round-3 work existed for this asset; the catalog `volumes` equalled `volumes/components/wheelchair-ramp.json` at the start (the bulk passes of 03:40 to 04:10 had not touched it). Scripts: `r3-wheelchair-ramp-kit.py` (facts and solid writers), `r3-wheelchair-ramp-model.py` (every solid of every scale and list), `r3-wheelchair-ramp-roots.py` (roots and 2D, smallest in-place replacements), `r3-wheelchair-ramp-zoom.ts` (close-ups; the shared sheet leaves 5 mm per pixel on an 11.7 m ramp).

## 1. Reference

| item | real value | source |
|---|---|---|
| running slope | 1:12 (8.33 %, 4.76 degrees) maximum | ADA 2010 section 405.2 (asset reference `ada-2010`) |
| rise per run | 0.30 m here, 0.76 m maximum | ADA 405.6 |
| landings | 1.5 m long, as wide as the run | ADA 405.7, DIN 18040-1 4.3.8 (`din-18040-1`) |
| clear width | 1.20 m between wheel guards | DIN 18040-1 4.3.8 |
| edge protection | 100 mm kerb on both sides of runs and of landings | ADA 405.9, DIN 18040-1 4.3.8 |
| handrail | top of gripping surface 0.85 to 0.90 m (DIN), 0.865 to 0.965 m (ADA); round 30 to 45 mm; continuous gripping surface, extensions 0.30 m level beyond each run, ends turned down or to the side | ADA 505, DIN 18040-1 4.3.6 |
| handrail tube | CHS 42.4 x 2.6, 2.55 kg/m, A 3.54 cm2, I 7.6 cm4 | EN 10219-2 (`en-10219`) |
| posts | CHS 48.3 x 3.2, 3.56 kg/m | EN 10219-2 |
| base plates and anchors | 10 mm plate, two M10 anchors (nut 17 mm across flats, 8.4 mm high), anchors never across a movement joint and at least 50 mm from a concrete edge | common steel-railing practice (my knowledge) |
| concrete ramp on grade | reinforced slab 150 mm on side walls about 200 mm over compacted fill, pours separated by movement joints with 20 mm dowels, 10 to 15 mm chamfered arrises | common practice (my knowledge); 25 kN/m3 EN 1991-1-1 (`en-1991-1-1`) |

## 2. Audit (before)

| root | finding |
|---|---|
| kerbs | ended at 10.2 m: the 0.6 m high top landing had no edge protection (both cited standards require it on landings) |
| posts | at 1.5, 3.3, 5.1, 6.6, 8.4, 10.2 m: four of six base plates sat across a pour joint or half over the kerb end (plate 1.44 to 1.56 m on a kerb that starts at 1.5 m) |
| posts | drawn 50 x 50 mm square in plan although the cited section is the round CHS 48.3; plate 120 x 100 mm as wide as the 100 mm kerb (no edge distance), 20 mm thick in the drawing and 10 mm in the text |
| post head | post ran full size (wider than the rail) into the rail: no continuous gripping surface |
| rail ends | 2D return was a 42 mm pen stroke at x 0.015 (reaching -0.006) beside a rail polygon starting at 0; plan capsules reached 21 mm beyond the declared length |
| support anchors | four 2 m footprints at 2.5, 5, 8, 11 m: the last one reached 0.3 m beyond the ramp end, none matched a pour |
| body section | `areaM2` 0.2 for a 1.4 x 0.15 m slab (0.21) |
| body mass | centre 8.127 m was the wedge alone, the kerbs were in the mass but not in the centre |
| thermal areas | the outer faces of the kerbs (about 1.8 m2) belonged to no region |
| rig | joints, lengths, slope angle (4.76364 degrees) and carrier travel (3.6125 m) are right; unchanged |
| `dimensions` | 10.5 x 1.4 x 1.471 m are right (rail start to landing end, rail top 0.6 + 0.85 + 0.0212); unchanged |
| solids | QA round 1: structure one filled wedge (C at every scale), floating rails at 1:500, 1:100 a reduced copy of 1:50 |

## 3. Root fixes (old -> new)

- Kerbs run to the end of the top landing (11.7 m) in the elevation and plan of 1:50, 1:100, 1:200; front kerbs drawn in front of the posts with their chamfers (1:50).
- Posts at 1.6, 3.3, 5.0, 6.7, 8.4, 10.1 m (uniform 1.7 m, 100 mm clear of every joint and kerb end) in every view of every scale; round in plan; 1:50 shows post (48.3 mm), cap, 16 mm stem, slanted plate 120 x 70 x 10 and nuts; 1:500 elevation gains the six post lines so that the rail no longer floats.
- Rail 1:50: outline with the two 60 mm bends and capped legs (the two return strokes are gone), axis line follows the bends, front rail from 0.75 m, plan capsules between the true ends (1:50, 1:100).
- `anchors` ground-0..3: x 2.5 / 5 / 8 / 11 -> 3.3 / 5.85 / 8.4 / 10.95, footprint length 2 / 2 / 2 / 2 -> 3.6 / 1.5 / 3.6 / 1.5 m (one bearing per pour).
- `rig.bones.body`: mass 12721.24784 -> 12791.14 kg (kerbs 10.2 m longer in total, chamfered), centre (8.12692, 0.21923) -> (8.06643, 0.22615); section area 0.2 -> 0.21 m2.
- `rig.bones.handrail`: mass 92.0055 -> 86.45 kg (rails 2 x 9.431 m x 2.55, posts 12 x 0.669 m x 3.56, stems, plates 120 x 70 x 10, anchors), centre y 1 -> 0.937.
- `dimensions.massKg` 12813.253 -> 12877.59.
- `thermal.regions`: `ramp-side` 7.02 -> 9.15 m2 (both flanks up to the outer kerb arris plus the outer chamfer; factor 1.30), `kerb` 3.48998 -> 3.77 m2 (top, inner chamfer and inner face over 2 x 10.24 m; factor 1.08). `ramp-surface` 12.26995 and `handrail-steel` 4.2 stand (model 12.27 and 3.96).
- `provenance.notes`: the construction in words (pours, slab on side walls over fill, mass lumped at 25 kN/m3, kerbs, handrail parts).
- Poses, clips, loads, rig joints: unchanged, they do not depend on the moved parts.

## 4. Model

Expanded solid counts (body / thermal / structure): 1:50 74 / 82 / 123, 1:100 20 / 22 / 47, 1:200 17 / 22 / 36, 1:500 15 / 19 / 34.

- **1:50 body**: four pours; eight kerb pieces with 10 mm chamfers (the run pieces are laid on the slope and mitred into the landing pieces); each rail one loft of 22 stated sections (capped leg, bend, mitres at the four slope changes, bend, capped leg); per post a turned tube with domed cap, a 16 mm stem into the rail, a plate lying on the sloping kerb top and two hexagonal nuts.
- **1:50 thermal**: 100 mm surface layer between the kerbs on every pour, 100 mm flank layer per pour and side, kerbs as their own 50 mm layer, steel as the outer surface of every steel part. Hollow tube shells were tried and withdrawn: the validator counts the bore (7.15 against 4.2 m2), see `author-requests.md`.
- **1:50 structure**: 150 mm slab per pour on two 200 mm side walls (slab on grade at the toe of run 1), four 20 mm dowels across each of the three joints, kerbs, rails as hollow 42.4 x 2.6 tubes between solid bends, posts as hollow 48.3 x 3.2 tubes with cap plugs, stems, plates and nuts as joints, one 30 mm bearing layer per pour at its anchor, mass at the centre.
- **1:100**: square kerb strips, plain-ended rail tubes, plain posts; thermal skins per pour and flank without layers; structure with one slab, two walls, dowel rows as bars, hollow tubes, foot flanges.
- **1:200**: one wedge, kerb strips, slender true rails, posts; structure with solid bars and foot joints.
- **1:500**: wedge with the kerbs merged, slender true rails on posts at the 75 mm size floor standing on the slab edge; kerbs kept as slender strips in thermal.

## 5. Checks

`volumes-check.ts components/wheelchair-ramp --product`: PASS at every scale (silhouette IoU elevation / front / plan 1:50 0.999 / 0.999 / 1.000, 1:100 0.999 / 1.000 / 1.000, 1:200 0.923 / 0.975 / 1.000, 1:500 0.909 / 0.981 / 1.000; thermal areas 1:50 surface 12.27, side 8.86, kerb 3.77, steel 3.96 m2). `r3-validate.ts components/wheelchair-ramp --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only`: `VALID`. Spliced with `volumes-insert.py`; roots written by `r3-wheelchair-ramp-roots.py --apply` (re-read, smallest replacements, everything else byte-identical).

## 6. What I saw

Sheets opened after the last change (`r3-sheet.ts`, six views each): every scale in body, thermal and structure, plus the 1:50 thermal cut, the 1:50 pose `wheelchair-on-run-2`, close-ups (`r3-wheelchair-ramp-zoom.ts`) and the product 2D of all four scales (`r3-2d.ts`).

- **1:50 body**: right and left side show the two runs, both landings, the kerb along the whole raised length, the rail parallel to the surface with its two level extensions and bends; front and back show the trough of slab and kerbs with one post column per side; top shows the three joints and the kerbs to the landing end; three-quarter close-ups show the capped post, the stem into the rail, the slanted plate with two nuts on the chamfered kerb and the bend with its capped leg. Nothing floats, nothing gaps. Grade A.
- **1:50 thermal**: red surface layer between the kerbs, green flank layers, purple kerbs, yellow steel; the cut shows the 100 mm layers as shells around the unmodelled core. The outer kerb face is hatched (it belongs to the side region whose skin lies behind it). Grade A, with the stated limit that the tube wall is not a shell.
- **1:50 structure**: back view open as a trough (slab on two walls, no filled wedge), front view with the mass inside, top view with the green bearing layer at the toe; the cut close-up shows walls, slab, dowels across both landing joints, plates and nuts as joints, bearing layers under every pour. Grade A.
- **1:100**: square kerbs, plain rail ends, plain posts into the rail; thermal as at 1:50 without layers; structure as trough with flanges at the post feet. Grade A.
- **1:200**: one wedge without joints, kerb strips, slender rails on posts; thermal with surface, flanks, kerbs; structure trough. Grade A.
- **1:500**: wedge without kerbs, rails resting on twelve 75 mm posts that stand on the slab edge (no floating rail); thermal keeps the kerbs as strips; structure trough with post foot joints. Grade A for what the scale can carry.
- **2D**: 1:50 elevation with bends, stems, plates and the kerb to 11.7 m, front with chamfered kerb ends, post, stem and rail, plan with round posts on plates; 1:100 and 1:200 with the moved posts and long kerbs; 1:500 with six post lines under the rail.
- **Pose** `wheelchair-on-run-2`: nothing of the body moves (the carrier bone has no solid), rig joint of the carrier on run 2 as expected.

## 7. Still imperfect

- The steel thermal layer (2.6 mm tube wall) is the solid body tube, not a shell, because the validator counts a sealed bore as outer surface (request filed in `author-requests.md`).
- 1:500 posts are 75 mm (size floor) under a true 42.4 mm rail; a true post would fail `volumes-feature`.
- The body mass keeps the asset's lumped 25 kN/m3 over the gross volume (walls, slab and fill together); the fill is not a solid of its own and the asset has no fill material.
- Silhouette IoU in the elevation is 0.92 at 1:200 and 0.91 at 1:500 (limit 0.90): rails and posts are pen strokes there, compared at pen width against round bars.
- The cited sections were kept: a post 48.3 x 3.2 at 1.7 m spacing is at its plastic limit under the 1.0 kN/m handrail load of the `handrail-push` pose (M about 1.45 kNm); that is the asset's structural data, not changed here.

State: DONE
