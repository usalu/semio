# Round 3 — `humans/wheelchair-user`

State (2026-11-20): written to the catalog (provenance, dimensions, materials, rig, anchors, representations with the 2D of all four scales redrawn from the bodies and new drawing decisions, poses, clip keys, thermal areas, structure) plus the manifest row (height 1.44, width 1.085) and `volumes` spliced with `volumes-insert.py`. `r3-validate.ts humans/wheelchair-user --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/wheelchair-user --product`: `FAIL: 4 measurement(s) over 2 rule line(s)` — every rule green at every scale except `volumes-thermal-coverage` on the body-model areas `jeans` and `shoes` at 1:50 and 1:100 that the author may not change (C18); request filed in `author-requests.md` (2026-11-20). Print suitability (office-laser) holds at every scale and at both ends of every valid range (the old 1:50 drawing was unsuitable at 1:50 and 1:75: 0.06 mm gap between seat rail and push handle).

Started fresh (no earlier round-3 scripts existed). Scripts in the ticket folder: `r3-wheelchair-user-ref.ts` (reference numbers), `-roots.ts` (rig, anchors, poses and every push-clip key solved with the product's `solveChain`; result cached under `🗑️generated/r3-wheelchair-user/`), `-frame.ts`, `-kit.ts` (constructors, adapted copies from the seated office worker), `-s50.ts` (1:50 body, every section by hand), `-coarse.ts` (1:100, 1:200, 1:500 bodies, each stated anew), `-lists.ts` (thermal, structure, root areas), `-texts.ts` (3D and 2D decisions, provenance), `-build.ts`, `-draw2d.ts` (2D redraw, ruling E1), `-write.py`, `-run.sh`, `-sheet.ts`, `-suit.ts`, `-probe.ts`.

## 1. Reference

| quantity | reference | used | source |
|---|---|---|---|
| stature / mass | 1.75 m, 75 kg | unchanged | asset provenance, `thermal.body` |
| hip joint above sitting surface | 0.09 to 0.10 m | hip 0.60 m, cushion top 0.52 (rear) to 0.55 (front), buttocks set 1 to 2 cm into the foam | Winter 2009; ANSUR II seated heights (my knowledge) |
| shoulder joint / C7 above hip (seated) | 0.818 H − 0.52 H = 0.52 m / 0.86 H − 0.52 H = 0.60 m | 0.53 / 0.58 m (trunk leaning 12 degrees back) | brief; de Leva 1996 |
| vertex above hip (seated) | sitting height 0.52 H ≈ 0.91 m above the seat, i.e. ≈ 0.82 above the hip | 0.83 m to the skull, 0.84 to the beanie (height 1.44 m) | ANSUR II (my knowledge) |
| head: chin to vertex / length / breadth | H/7.5 = 0.233 / 0.197 / 0.155 m | 0.233 / 0.20 / 0.154 m | brief; ANSUR II male |
| shoulder joint spacing, bideltoid | biacromial 0.40 to 0.43 m minus 2 × 3 cm; bideltoid ≈ 0.47 | 0.352 m; 0.47 m in the hoodie | ANSUR II male |
| hip joint spacing / seated hip breadth | 0.17 to 0.18 / 0.38 to 0.40 m | 0.175 / 0.394 m in jeans | Bell et al. 1990; ANSUR II |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.326 / 0.256 / 0.189 | 0.323 / 0.255 / 0.186 (unchanged) | Winter 2009 |
| thigh / shank / ankle height | 0.245 H / 0.246 H / 0.039 H = 0.429 / 0.43 / 0.068 | 0.426 / 0.43 / 0.085 in a sneaker with a 19 mm sole | Winter 2009 |
| foot / shoe | 0.152 H = 0.266 m; low sneaker EU 43 ≈ 0.29 × 0.105 m | heel to toe 0.292 m, 0.105 m wide, sole 19 mm | my knowledge |
| girths | chest 0.98, waist 0.86, thigh 0.56, calf 0.38, upper arm 0.32 m (75 kg man) | hoodie 0.35 × 0.26 at the chest, jeans thigh 0.18 × 0.16, leg 0.11 × 0.12, sleeve 0.11 | ANSUR II (my knowledge) |
| rear wheels | 24 inch: ETRTO 540 bead seat (r 0.270), 37-540 tyre outer r ≈ 0.305; box rim; 24 to 36 spokes laced 2-cross; hand rim Ø 530 mm of 19 mm tube, 25 to 30 mm outboard on 6 tabs; quick-release axle | r 0.305, rim 0.249 to 0.2735, 24 spokes 2-cross, hand rim r 0.265 at 30 mm outboard on 6 standoffs | ETRTO; Cooper and De Luigi 2014 (asset reference); manufacturer catalogues (my knowledge, no lookup) |
| castors / wheelbase | 4 to 6 inch; rear axle to castor axle 0.40 to 0.45 m | 135 mm (asset), wheelbase 0.44 m (rig, unchanged) | Cooper 2014 |
| seat | width 0.40 to 0.44, depth 0.40 to 0.46, rear seat-to-floor 0.44 to 0.48 with 3 to 6 cm dump, cushion 50 to 80 mm, back height 0.30 to 0.40 above seat, backrest angle 90 to 100 degrees | 0.42 / 0.43; rear rail 0.45, front 0.49; cushion 60 mm; back sling 0.53 to 0.86 m; posts 3 degrees back | ISO 7176-7 measures; active chair catalogues (my knowledge) |
| overall | width of an active chair with 0 degree camber: seat + ≈ 0.22 m | 0.639 m | — |
| segment masses and centres | de Leva 1996 | unchanged except trunk, foot, toe and neck centres scaled to the new lengths | asset references |

No web search was made; "my knowledge" values are not verified against a datasheet.

## 2. Audit (old asset)

- Rig: shoulder joints only 0.303 m apart (`z` ±0.1516); hip joints directly above the rear axle, so the buttocks (≈0.12 m behind the hips) lay inside the back sling; trunk without lumbosacral offset and dead upright; neck and head with ±8 degrees; shank 0.4165 m (−14 mm); foot bone 0.182 m from ankle to ball (the ball sat 0.2 m ahead of the ankle).
- Poses and clips: all arm offsets were pitch-only, so the hands stayed at z ±0.15 while the hand rims are at ±0.30: in `hands-on-rims`, `push-end`, `ramp-ascent` and all 192 push keys the grip point floated 0.3 to 0.43 m from the axle (rim radius 0.265) and 0.15 m inboard of the rims; the hand did not turn with the rim (45 degrees of wheel, 37 degrees of hand).
- Chair solids: hand rims inboard of the tyres (z 0.30 against tyre 0.33) — a wheelchair has them outboard; 12 radial spokes of 9 mm to the middle of the tyre; tyre 45 mm wide; capsule-chain frame; footplate a 0.32 × 0.34 m board; wheels at z ±0.33 so the chair was 0.70 m wide.
- Man: box torso, tube limbs, hands as one loft with a capsule thumb, no gloves although the regions are "gloves" and "bare fingertips", no pocket or hood form beyond a slab.
- 2D: constructed views; 1:50 unprintable at 1:50 and 1:75 (gap 0.06 to 0.14 mm); front and plan did not agree with one body.
- Thermal and structure: references to the old bodies; structure of capsule members on the body.

## 3. Root fixes (old → new)

| member | old | new |
|---|---|---|
| `pelvis` x / y | 0 / 0.295 | 0.06 / 0.295 (hip joints 60 mm ahead of the rear axle) |
| `torso` y / angle / length / com | 0 / 0 / 0.4557 / (0.215, 0) | 0.04 / 12 / 0.495 / (0.2178, 0.01) |
| `neck` x / angle / length / comX | 0.4557 / −8 / 0.098 / 0.05 | 0.495 / −20 / 0.116 / 0.058 |
| `head` x / angle / length / com | 0.098 / 8 / 0.1764 / (0.075, 0) | 0.116 / 10 / 0.13 / (0.03, −0.012) |
| `arm-l/r` x / y / z | 0.4116 / 0 / ±0.1516 | 0.43 / −0.035 / ±0.176 |
| `thigh-l/r` angle / z | −92 / ±0.0876 | −89.19 / ±0.0875 |
| `shank-l/r` angle / length / comX | −79 / 0.4165 / 0.1857 | −80.81 / 0.43 / 0.1917 |
| `foot-l/r` angle / length / com; `toe` angle / length / com | 57.2 / 0.1821 / (0.0775, −0.0032); 23.8 / 0.0637 / (0.0294, 0.0294) | 62.96 / 0.1485 / (0.052, −0.022); 17.04 / 0.066 / (0.03, 0.006) (sole flat on the footplate) |
| bone titles `frame`, `rear-wheel`, `castor` | folding-chair wording | rigid frame, 24 inch, 135 mm |
| anchors `rim-grip`, `rim-grip-l` | (0.0931, 0) | (0.082, −0.026): middle of the palm over the knuckles |
| anchors `mouth`, `nostrils`, `crown` | (−0.0049, −0.0785), (0.0294, −0.1018), (0.17, 0) | (−0.061, −0.106), (−0.035, −0.108), (0.142, 0.01) on the new head |
| poses | pitch-only arms | all four solved in 3D (shoulder angle, abduction and twist, elbow flexion with ±8 degree carrying angle, forearm pronation, wrist flexion and deviation): `seated-level` palms down on the thighs; `hands-on-rims` grip at 100 degrees from ahead, outboard; `push-end` at 55 degrees after 45 degrees of wheel; `ramp-ascent` at 74 degrees. Grip misses 0.02 / 1.9 / 0.7 / 0.9 mm. Descriptions rewritten. |
| clip keys `push-cycle` (128), `ramp-roll` (64) | pitch-only arms | every key solved again: on the rim the hand turns with the wheel from the catch (100 / 98 degrees) through 0.1875 s (0.25 s on the ramp); in the recovery it follows through, rises up to 0.13 m above the rim and comes back to the catch (Catmull-Rom path); trunk, neck, head, wheel and castor keys unchanged. Worst grip miss 4.0 mm. |
| `representations[*].geometry` | 75/55/42 … 12/17/20 constructed shapes | redrawn from the bodies (section 4) |
| `representations[100,200].pens.texture` | null | 0.10 mm (the spokes are texture strokes) |
| `representations[*].decisions`, `provenance.notes` | old construction | rewritten (en, de) |
| `thermal.regions[cushion].areaM2` | 0.63 | 0.89 (C16, factor 1.41: cushion 430 × 410 × 60 mm on a seat sling 420 × 410 mm, both faces, plus the padded back sling 450 × 330 mm, both faces) |
| `materials` | — | `bone` added (structure) |
| `dimensions` | 1.45 / 1.09 / 0.70 / 90.2 kg | 1.44 / 1.085 / 0.639 / 90.22 kg (bbox of the 1:50 body in `seated-level`; mass = rig sum) |
| manifest row | 1.45 / 1.09 | 1.44 / 1.085 |

Not changed on purpose: wheel radius, wheelbase, castor bone, frame and wheel masses, structural assumptions (still true), clip timing, contacts, tracks, thermal layers, emitters, breathing.

2D redraw (ruling E1, `-draw2d.ts`): each shape is the outline of its part's solids in `seated-level`. Frame, pelvis, trunk, neck, head and legs (pitch-only bones) are flat outlines solved into their bone frames through the product's placement (`inverse`, residuals under 0.01 mm). Rear wheels and castors (revolving bones): every revolve part is its side profile (disc, or annulus as two half rings) with an `extruded` solid over its lateral span, the same shape in all three views, so front and plan stay right at any wheel angle (checked at `push-cycle` t = 0.5 s). Sleeves and forearms are revolved profiles, gloves and finger tips plates, so the spread arms of every pose project. Slender parts are strokes (spokes texture, drawstrings detail, 1:200 and 1:500 tubes body lines). In front and plan the axle plates are merged into the side tubes and the 1:100 hubs omitted (they printed narrower than 0.2 mm). Shape counts 1:50 / 100 / 200 / 500: elevation 136 / 88 / 53 / 29, front 90 / 60 / 40 / 28, plan 82 / 59 / 40 / 28.

## 4. What was modelled (body / thermal / structure)

- **1:50 — 131 / 128 / 181.** See the 1:50 decisions in the catalog: man in hoodie (rolled hem, hood folded down, neckline band, drawstrings, kangaroo pocket), jeans, sneakers (upper, midsole, toe box, toe sole), half-finger gloves (palm, thenar, gloved proximal phalanges, glove cuff) with bare middle and distal phalanges and thumb tip, beanie with cuff, face with nose, lips, eyes, brows, ear lobes; rigid chair with bent side tubes, lower tubes, axle plates and receivers, back posts with push grips, castor housings, forks, wheel locks, footrest loop, cross tubes, footplate, seat sling, cushion, back sling; rear wheels with tyre, box rim, flanged hub, 24 two-cross spokes, hand rim on six standoffs; castors with tyre and web wheel. Thermal: nested shells (tee/fleece/hoodie, briefs/jeans, socks/sneaker), face and neck skin below the beanie, beanie shell, three outlet patches. Structure: full skeleton (8 rib hoops, scapulae, clavicles, finger bones), joints, de Leva masses, chair parts by reference, bearings as joints, wheel and castor masses per side, the frame mass on twelve parts.
- **1:100 — 65 / 62 / 131**, **1:200 — 40 / 35 / 80**, **1:500 — 26 / 28 / 52**: authored anew per scale (`-coarse.ts`), the merging is stated in each scale's decisions (mitten hands, simplified wheels with 12 / 6 / no spokes, slender tubes from 1:200, wheel rings and seat/back blocks at 1:500, hands carried by the forearm masses at 1:500 with their own thermal solids).

Final check `volumes-check.ts --product`: silhouette IoU elevation / front / plan 1:50 0.983 / 0.988 / 0.993 (p95 1.9 / 16.9 / 16.9 mm), 1:100 0.991 / 0.989 / 0.991, 1:200 0.978 / 0.986 / 0.982, 1:500 0.986 / 0.982 / 0.986; bounds, feature, monotonic, cross-scale, slender, views-disagree, structure coverage, bones, copied green at all scales; thermal at 1:50: face-skin +1 %, beanie +16 %, hand-skin +22 %, gloves +11 %, hoodie +24 %, frame-metal +2 %, tyres +12 %, cushion 0 %, **jeans +39 %**, **shoes +38 %**; 1:100 **jeans +37 %**, **shoes +34 %**, all others within 30 %; totals 1:200 +17 %, 1:500 +12 %. Line: `FAIL: 4 measurement(s) over 2 rule line(s)`.

## 5. What I saw (Read on the PNGs)

- 1:50 body, `seated-level`, six views and large right side, front and three-quarter: a man of normal proportions sitting back against the sling, hands palm down on the thighs, knees at about 100 degrees, sneakers flat on the footplate between the castors; hood folded on the upper back, drawstrings and pocket on the front, beanie with cuff; chair reads as an active rigid chair: bent side tube into the hanger, castor housing with fork and trailing castor, wheel lock lever, push handles, axle plate, hand rims outboard of the tyres, 24 crossed spokes. First state had a dark oversized axle plate, a hood like a backpack, a jeans hem 2 cm above the shoe and flat slipper-like sneakers — all corrected.
- 1:50 `hands-on-rims` (six views) and `ramp-ascent` (six views): gloves on top of the hand rims just behind the top, elbows back and out, the hands outboard of the wheels in front and plan; `push-cycle` t = 0.5 s: hands in the recovery above and ahead of the rims, wheels turned, spokes and castor turned.
- 1:50 thermal (cut, six views): nested hoodie and jeans shells, face skin and beanie shell, tyres and frame metal coloured apart, nothing outside the body. 1:50 structure (six views): skull, mandible, spine with curvature, rib hoops, scapulae, pelvis, long bones, finger bones in the lap, the chair frame, wheels and castors as members.
- 1:100, 1:200, 1:500 body (six views each): the same seated man and chair in fewer masses; at 1:500 wheel rings, castor discs, seat and back blocks and line tubes still read as a wheelchair. 1:100 and 1:200 shoulders were boxy and were given the trapezius slope of 1:50.
- 2D (`r3-2d`) 1:50 line and solid, 1:100 clip frame, 1:200 and 1:500 line: clean drawings; at 1:200 and 1:500 the true-size panels are small and only the silhouette could be judged.
- Overlays (`volumes-render`) 1:50: elevation, front and plan green; the spokes were red/blue until they became texture on both sides.
- Rendered, not opened in the final state: 1:100 thermal and structure sheets, 1:200 / 1:500 thermal and structure sheets.

## 6. Still imperfect

- `volumes-check` red on `jeans` and `shoes` (body-model areas, C18); request filed (C27).
- Grades as I see them: 1:50 body A− (superellipse sections without separate pectorals or scapulae under the hoodie; no cloth folds except the stacked jeans hem and rolled hoodie hem; the knee is the overlap of two domes with a visible seam; the hand rim grip at the catch needs the wrist at its limits of 70 degrees flexion and 30 degrees deviation, see the solver report), chair A− (0 degree camber because the rig turns both wheels about one lateral axle — a cambered wheel would wobble; the castor swivel circle overlaps the footrest when a castor turns backwards; no side guards, no anti-tippers). 1:100 A−, 1:200 B+, 1:500 B (blocks and line tubes).
- The trunk in `ramp-ascent` leans only about 12 degrees forward of vertical (the clip's trunk keys were kept).
- Chair and garment dimensions are from my knowledge of products and standards, not a looked-up datasheet.
- Tests and fixtures that embed this asset were not regenerated or run.

## Round 5 handcraft (2026-11-21)

Script: `r5-wheelchair-user-edit.py` (edits the hand-stated rows of `r3-wheelchair-user-s50.ts` and `r3-wheelchair-user-coarse.ts`); renders `r5-sheets.sh`; rebuild, 2D redraw and splice with the round-3 pipeline (`r3-wheelchair-user-run.sh`, `r3-wheelchair-user-write.py` after checking that the candidate differs from the catalog only in the 2D geometry, `volumes-insert.py`).

### Reference and model (1:50, seated-level; H = 1.75 m)

| quantity | reference | model | dev. |
|---|---|---|---|
| upper arm / forearm / hand | 0.186 H = 0.326 / 0.146 H = 0.256 / 0.108 H = 0.189 | 0.323 / 0.255 / 0.186 | -0.8 / -0.4 / -1.6 % |
| thigh / shank | 0.245 H = 0.429 / 0.246 H = 0.430 | 0.426 / 0.430 | -0.6 / 0 % |
| head chin-vertex | H/7.5 = 0.233 | 0.233 | 0 |
| shoulder joint spacing | biacromial 0.40-0.43 minus 2 x 3 cm = 0.34-0.37 | 0.352 | in range |
| hip joint spacing / seated hip breadth | 0.17-0.18 / 0.38-0.40 | 0.175 / 0.394 | in range |
| hip joint above cushion | 0.09-0.10 | 0.08 (rear) to 0.05 (front) plus 1-2 cm foam set | in range |
| vertex above hip seated | 0.82 | 0.83 skull | +1.2 % |
| rear wheel | 24 in, 37-540, outer r 0.305, hand rim Ø 0.53 30 mm outboard | same | 0 |
| seat width / depth / rear height | 0.40-0.44 / 0.40-0.46 / 0.44-0.48 | 0.42 / 0.43 / 0.45 | in range |
| folded hood on the back | 2-3 layers of 6-8 mm fleece, 20-35 mm proud | 27-34 mm (before 36-42 mm) | in range |

### Grades (before -> after; body / thermal / structure)

- 1:50: A- / A- / A- -> A- / A- / A- (hood no longer reads as a backpack). 1:100: A- / A- / B+ -> A- / A- / B+. 1:200: B+ / B+ / B+ -> B+ / B+ / B+. 1:500: B / B / B -> B / B / B (calf on the shank).
- Opened before: 1:50 body (seated-level), 1:50 structure, 1:500 body, 1:50 push-cycle t = 0.5 s. Opened after: 1:50 body, 1:50 2D line and solid, 1:50 ramp-ascent. Rendered after and not opened: 1:100 to 1:500 sheets, hands-on-rims and push-end (the rig, poses and the parts moved by them are unchanged).

### Changes

- Hood folded on the upper back flattened at 1:50, 1:100 and 1:200 (centre moved onto the hoodie back, thickness 16-20 mm half instead of 24-28 mm); 1:500 shank with a calf station. 2D of all scales redrawn from the bodies by `r3-wheelchair-user-draw2d.ts`.

### Checks

- `r3-validate.ts humans/wheelchair-user --catalog-only`: `VALID ... schema 0, validateAsset 0`.
- `--volumes`: 4 measurements red, unchanged from round 3 and pending with the area reviewer (`author-requests.md` 2026-11-20): `jeans` 0.847 against 0.6091 (+39 % 1:50, +38 % 1:100), `shoes` 0.185 against 0.1332 (+39 %, +34 %).

### Remaining

- Body-model areas `jeans` and `shoes` (C18/C27, reviewer).
- No cloth folds beyond the stacked jeans hem and the rolled hoodie hem; superellipse sections without separate pectorals; 0 degree camber (one lateral axle in the rig); trunk lean in ramp-ascent only 12 degrees; 1:500 faceted by the product's coarse tessellation.

Both castor forks now retain 0.20 mm paper outlines at the 1:350 endpoint. The complete paper document, 1:200 feature, silhouette and cross-scale checks passed. All four live heat checks and all twelve exported scenes passed.

## Rear axle attachment reviewed November 21

The 72 × 150 × 15 mm frame plates and bearing receivers now remain between the frame and wheel hubs at all four scales. Their thermal references use the frame-metal region. Previously the 1:200 structural axle marker was 38.3 mm outside the visible wheelchair. The published correction passed the structural check on both sides, all four strict scale checks and twelve exported scenes.
