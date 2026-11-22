# Round 3 — animals/herring-gull (adult herring gull, Larus argentatus)

State: the roots are written to the catalog file and the volumes are spliced. `r3-validate.ts animals/herring-gull`: VALID (schema 0, validateAsset 0; `--catalog-only` VALID too). `volumes-check.ts animals/herring-gull --product`: **PASS** at every scale (16.8 s). Full `checkVolumes` on the trial document: 0 findings.

My own grades:
- Standing body: B+ at 1:50, B at 1:100, B- at 1:200.
- Structure: B+ at 1:50, B at 1:100.
- Thermal: B+.
- Spread-wing poses: C+ (see section 6).
- 2D: B+.

It reads as a herring gull from every side in the standing pose. It is not "perfect".

No earlier round-3 work existed for this asset. I started from the round-2 state, which was valid and green but wrong.

Chain (all in `T`):
- `sh r3-herring-gull-run.sh` builds the roots into the trial document, the three scales, the 2D redraw, the trial document with volumes and its validation.
- `sh r3-herring-gull-run.sh catalog` does the same, then writes the catalog file and splices the volumes.
- Run both with `PYTHONUTF8=1`.

Scripts:
- `r3_herring_gull_roots.py`: rig, poses (incl. the wing solver), anchors, materials, references, summary.
- `r3-herring-gull-kit.ts`: the bench.
- `r3-herring-gull-body.ts`: the 1:50 body, every station stated by hand.
- `r3-herring-gull-coarse.ts`: 1:100 and 1:200.
- `r3-herring-gull-skeleton.ts`: the structure list.
- `r3-herring-gull-model.ts`: the lists and the thermal list.
- `r3-herring-gull-decisions.ts`.
- `r3-herring-gull-2d.ts`: the redraw under ruling E1.
- `r3-herring-gull-io.py`, `-apply.py`, `-full.py`.
- `r3-herring-gull-sheet.ts`: a six-view sheet of the trial document.
- `r3-herring-gull-look.sh`, `-probe.ts`, `-area.ts`.

Inputs kept:
- `r3-herring-gull-orig.json` (the asset before round 3, without volumes).
- `r3-herring-gull-2d.json` (the redrawn members).

## 1. Reference

| measure | value | source |
|---|---|---|
| length (bill tip to tail tip) | 56-66 cm, wingspan 123-155 cm, mass 0.7-1.5 kg (asset 1.0 kg, span 1.45 m) | Cramp 1983 (asset reference) |
| wing (wrist to tip of longest primary) | 400-450 mm | Cramp 1983, as I remember the table |
| tail | 160-180 mm | same |
| bill | culmen 48-60 mm, depth at the gonys 17-21 mm, about 9 mm wide | same; width from my knowledge |
| tarsus | 58-70 mm | same |
| carpometacarpus / tarsometatarsus / femur | 71.7 / 66.0 / 58.1 mm | IMNH R-976 (Idaho Museum of Natural History virtual osteology, fetched 2026-11-20), units not stated, read as mm |
| coracoid / sternal keel | 50.1 / 73.1 mm | IMNH R-976 |
| humerus / ulna | about 135 / 140 mm | my estimate from span = 2 × (shoulder–wrist + wing) + body: 2 × (0.26 + 0.43) + 0.07 ≈ 1.45 m; no table at hand |
| tibiotarsus | about 105 mm | my estimate (gull legs: tibiotarsus ≈ 1.6 × tarsus) |
| middle toe | about 58 mm with claw; webs between three front toes; hind toe vestigial (about 5 mm) | my knowledge |
| standing posture | body tilted 10-20°, neck in a short S, crown about 0.33-0.36 m, belly about 0.10-0.11 m above the ground, about 2 cm of bare tibia above the ankle, primaries crossing over the tail and 5-8 cm beyond it | my knowledge |
| plumage | white head, neck, body and tail; pale grey mantle and wings; black primaries with white mirrors; white tertial crescent and secondary tips; yellow bill with a red gonys spot; pale yellow eye; pink legs | Cramp 1983 |
| tarsus section | about 7 × 4.5 mm; toes about 3 mm thick | my knowledge |

WebSearch found only the IMNH record with numbers. The other values are stated as my own knowledge, here and in the asset's references.

## 2. Audit (state before round 3)

**Rig.**
- The trunk was hinged at the tail base and 0.34 m long.
- The hips sat at (0.15, -0.05) at the belly with no lateral position.
- The wing was two bones (arm 0.20 m, hand 0.22 m) at z ±0.068, outside the body. That gives a span of 0.98 m against 1.45 m.
- A two-bone wing cannot fold. Its wrist landed 0.20 m behind the shoulder, so the folded wing was two rods lying on the back.
- Leg lengths: femur 60, tibiotarsus 100, tarsus 65, foot 75 mm (middle toe 58 mm).
- The standing trunk was tilted 20°.

**Dimensions.** The asset declared 0.51 m high, 0.692 m long and 0.20 m wide. The real standing bird is about 0.35 × 0.60 × 0.12 m.

**2D.**
- Tones were inverted: body 0.08 (black), wingtip 0.92 (white).
- The bill, legs and feet were drawn 2-5 × the real size: legs 21 mm wide against about 6 mm real.
- The wings were bars, and front and plan were constructed ellipses.
- There was no eye, no bill spot and no mirror.

**3D** (`qa-visual-3.md`: C+ / B- / B-).
- Wings were rods standing off the back; the head was a box on a tube neck.
- The structure was one rod per rig bone, with the wings not joined to the chain.
- The bill was 3.35 × its area (area-review).

## 3. Root fixes (catalog asset)

| member | old | new |
|---|---|---|
| trunk | origin at the tail base, 0.34 m | origin at the acetabula, 0.10 m to the neck base; centre of mass (0.035, -0.03) |
| tail | (0, 0.01), 175°, 0.14 m | pygostyle (-0.075, 0.016), 172°, 0.17 m |
| neck-1 / neck-2 / head | at (0.29, 0.035), 80 / 70 / 125 mm, rest angles 55 / 20 / -85° | at (0.10, 0.012), 70 / 65 / 125 mm, rest angles 16 / 55 / -97° (short S, standing). Every other pose keeps its old world neck and head directions |
| wing | `arm-n/f` 0.20 m, `hand-n/f` 0.22 m | `humerus-n/f` 0.135 m at (0.095, 0.016, ±0.036), `forearm-n/f` 0.14 m, `hand-n/f` 0.415 m. Rest = folded Z: elbow back near the mantle, wrist at the side of the breast, hand back over the tail. Limits humerus -90..200, forearm -200..40, hand -30..180; masses 0.03 / 0.015 / 0.04 kg |
| legs | thigh 60 mm at (0.15, -0.05), z 0; shank 100; tarsus 65; foot 75 mm | thigh 58 mm at the origin, z ±0.022; shank 105; tarsus 66; foot 58 mm. Rest = standing (femur forward, knee in the belly, tarsus 12° off vertical); thigh limits -100..80 |
| poses (9) | — | see the list below |
| anchors | contacts (0.04, -0.01), 70 × 50 mm; nostrils (0.075, 0.006); bill-tip (0.125, -0.004); eye (0.035, 0.012); back-plume (0.17, 0.11) in the old trunk frame | contacts (0.024, -0.0055), 60 × 55 mm; nostrils (0.087, 0.0005); bill-tip (0.1255, -0.0105); eye (0.03, 0.0155); back-plume (0.045, 0.0438) on the mantle |
| dimensions | 0.51 / 0.692 / 0.20 | 0.351 / 0.603 / 0.119 (height / length / breadth of the 1:50 model in the standing pose). Length matches the cited 56-66 cm; height and breadth follow from the cited bones and are not cited measurements themselves |
| materials, references, summary | — | added `eye-tissue` and `bone`; added the `imnh-r976` and `dumont-2010` references and the biometrics in `cramp-simmons-1983`; summary and the `standing` / `gliding` descriptions rewritten |
| representations 50 / 100 / 200 | see the audit | every shape redrawn (section 4b); all eight decision texts rewritten; 1:50 detail pen 0.13 mm; 1:500 `reason`, `limitingFeature` and `limitingSizeM` restated |

How the nine poses were rebuilt:
- **Wings:** each spread pose (gliding, gliding-up/-down, approach-a, landing, flare-key, touchdown) is stated as world span directions of humerus, forearm and hand plus a trailing direction in the trunk frame. The script solves angle, abduction and twist bone by bone, choosing the anatomical way round so the clips swing the wings correctly.
- **Legs:** solved anew so the old ankle position is kept, scaled to the new reach, with the knee ahead.
- **Root x:** shifted +0.17 m because the origin moved.
- **Grounded poses:** touchdown and settled were lowered onto their feet (root y 0.1932 / 0.1794).
- **Standing:** trunk 14°.

`areaM2` values are untouched. `structure.supports` is empty, so there are no support solids.

## 4. Model

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 46 | 64 | 79 |
| 1:100 | 29 | 31 | 55 |
| 1:200 | 16 | 18 | 24 |
| 1:500 | unavailable (as the representation) | | |

### 1:50 body
- A hull stated in 15 hand-written stations: full breast, 13 cm deep, 11 cm wide.
- The grey mantle lens and two white neck links.
- The head with a long crown, both eyes, and the deep hooked bill with the red gonys spot (texture).
- A square tail.
- Per wing, the folded wing as three strips on its three bones:
  - coverts on the humerus, plus the white tertial crescent;
  - secondaries on the forearm, plus their white tips (texture);
  - black primaries on the hand, plus the white mirror (texture).
- Per wing, three membranes tucked inside the hull: the propatagium, the arm web and the hand web. They only show in the spread poses.
- Per leg: thigh, feathered drumstick, about 2 cm of bare shank, a slim tarsus, two webs and three toes. The hind toe is left out.

### Thermal
- Every plumage region has nested down and contour shells at their real thickness.
- Two nares outlet slits on the bill, and a back-plume outlet patch on the mantle.
- Coarser scales use one shell per region.

### Structure (1:50)
- Skull and mandible, two cervical rods, spine, synsacrum, pygostyle with six rectrix shafts.
- Sternum with keel, twelve ribs, coracoids, furcula, scapulae, pubes.
- Humerus, ulna and radius, manus with three primary shafts.
- Femur, tibiotarsus, tarsometatarsus, toe bones.
- Eighteen joints, plus trunk and head masses.

### 1:100 and 1:200
- **1:100:** own stations; merged wing strips; one web plus the middle toe per foot; a bar skeleton with joints.
- **1:200:** a covert panel and the primaries, thickened to 30 mm and set into the flank. The arm strip is merged, so the spread wing at this scale has no arm part. Legs are slender strokes; the structure is a wire skeleton.

### 4b. 2D (ruling E1)
- **Elevation:** each shape is the traced outline of the solids that a hand-written table assigns to it, mapped into the product's shape frame through `evaluate` (residual 0.0000 mm).
- **Wing shapes:** they carry `solid` hints (elevation bands swept over their lateral span; plan sheets), because their bones leave the plane in the spread poses.
- **Tones** follow the real bird: white 0.95, grey 0.64-0.66, primaries 0.1, bill 0.8, legs 0.68.
- **Strokes drawn by hand at 1:50:** a primary line, a covert line, a toe line and the gape line.
- **1:200 legs** are 0.15 mm strokes.

### Final check line
`volumes-check.ts --product` PASS. Silhouette IoU elevation / front:
- 1:50: 0.981 / 0.955
- 1:100: 0.973 / 0.939
- 1:200: 0.955 / 0.937

Areas: bill 0.0025 against 0.002. Legs and feet are within the limit after I slimmed the tarsus to 7 × 5 mm, the toes to 3 mm and kept about 2 cm of bare tibia.

## 5. What I saw (PNGs opened with the Read tool)

**1:50 body, six-view sheet and the single right and three-quarter views:**
- From the side it reads as a herring gull at once: full white breast, upright thick neck in a short S, long-crowned head with a deep hooked bill, grey folded wing with a white lower edge, black primaries crossing over the tail and about 7 cm beyond it, long pink legs with webbed feet.
- Front: an egg-shaped breast with narrow wing rims and the legs 44 mm apart.
- Top: a spindle with the wings converging over the tail.
- Back: busy around the vent, where the tips of the wing strips are seen end-on.
- Three-quarter: convincing.
- Still visible: a small facet where the lower neck leaves the breast, and a slightly stepped wing panel (coverts over secondaries).

**1:100 and 1:200 body sheets:** the same bird, harder and simpler.
- At 1:200 the head has a small crease at the occiput.
- At 1:200 the legs are slender sticks (yellow, slender solids).

**1:50 structure sheet:** reads as a bird skeleton: S-neck, rib cage over the sternum and keel, wing bones in the folded Z, digitigrade legs, primary and rectrix shafts in the feathers. The skull is a smooth loft, not a true cranium. 1:100 is a readable bar skeleton with joints.

**1:50 thermal sheet, cut:** every region is coloured and the nested shells are visible; nothing pokes out. I rendered the 1:100 thermal sheet but did not open it.

**Pose `gliding` (six views) and clip `landing` at t = 1.6 s:**
- Spans and joint chains are right: 1.45 m, a slight dihedral, the wrist forward.
- Landing: the wings are raised in a V and the feet reach forward.
- The wings, though, are narrow strips with a lump at the elbow (section 6).

**2D, the product's own drawings:**
- 1:50 (light-line and light-solid): a clean gull in all three views. The near-side neck shows a small collar line in an earlier run; I fixed the layer order afterwards.
- 1:200: tiny but correct.
- Not opened: the 2D of 1:100.

## 6. Still imperfect

- **Spread wings.** Rigid solids on three bones cannot fold a feather fan. The spread chord is about 60 % of the real one (arm 0.08-0.11 m against 0.18 m), with gaps and lumps at elbow and wrist. At 1:200 there is no arm part at all. Request filed in `author-requests.md` ("r3 animals/herring-gull"): per-section skinning of lofts, or feather bones.
- **Primary projection:** about 7.5 cm beyond the tail, at the long end of the real range.
- **Small seams:** a facet at the base of the neck; a stepped covert/secondary edge; a messy vent seen from behind.
- **Skull** in the structure list is a smooth loft without orbits.
- **Values not from a table:** humerus, ulna and tibiotarsus lengths, and the standing height and breadth, are my estimates.
- **Not opened:** the `🗑️generated` renders of 1:100 thermal and the 1:100 2D; single-list `volumes-render.ts` views (I used the six-direction sheets instead).

## Round 5 handcraft (2026-11-20/21)

Scripts: the asset's chain, extended in place (`r3-herring-gull-body.ts`: head, neck, primaries; `r3-herring-gull-coarse.ts`, `r4-herring-gull-500.ts`: head and neck; `r3_herring_gull_roots.py`: tucked legs in the soaring poses), `r5-squirrel-gull-look.sh`. Build and splice: `PYTHONUTF8=1 sh r3-herring-gull-run.sh catalog`.

### Reference table (modelled to)

| measure | reference | source |
|---|---|---|
| length bill tip to tail tip | 560-660 mm | Cramp 1983 |
| wingspan | 1230-1550 mm (asset 1450) | Cramp 1983 |
| culmen / gonys depth | 48-60 / 17-21 mm | Cramp 1983 |
| tarsus | 58-70 mm | Cramp 1983 |
| tail | 160-180 mm | Cramp 1983 |
| femur / tarsometatarsus / carpometacarpus | 58.1 / 66.0 / 71.7 mm | IMNH R-976 |
| primary projection beyond tail (standing) | 50-80 mm | own knowledge |
| flight posture | legs folded back, feet trailing under the tail | own knowledge |

### Model vs reference (1:50, standing, spliced catalog)

| measure | model | reference | deviation |
|---|---|---|---|
| bill tip to primary tips / bounding length | 602.7 mm | dimensions 603 | 0.05 % |
| height / breadth | 346.7 / 118.8 mm | dimensions 351 / 119 | 1.2 / 0.2 % |
| culmen (feathering to tip, head bone 0.067-0.1255) | 58.5 mm | 48-60 | in range |
| bill depth at base / gonys | 19.4 / 17.4 mm | 17-21 | in range |
| femur / tibiotarsus / tarsus / middle toe (rig) | 58 / 105 / 66 / 58 mm | 58.1 / ~105 / 66.0 / ~58 | under 0.2 % |
| span in `gliding` | 1.45 m | 1.45 | 0 |
| bounding box 1:100 / 1:200 / 1:500 | 603.0 x 346.2 / 602.8 x 346.6 / 602.8 x 348.3 mm | same | under 1.4 % |

### Grades (before -> after)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B (flat-cut occiput sitting on a tube neck like a cap, neck-2 poking through the crown, a hard throat step where the lower neck left the breast) -> B+/A- | B+ (not re-opened) | B+ -> B+ (unchanged; skull a smooth loft) |
| 1:100 | B (same head and neck) -> A- | B+ (not opened) | B (not opened) |
| 1:200 | B- -> B (head domed; rendered, not opened after) | B (not opened) | B (not opened) |
| 1:500 | B (rendered, not opened after) | B (not opened) | B (rendered, not opened) |
| poses | `gliding` D (legs hanging straight down, wings a chain of narrow leaves), landing C+ -> `gliding` C+ (legs tucked, feet trailing under the tail), landing C+ | | |

### Changes

- Head 1:50/1:100/1:200/1:500: occiput domed (new end stations behind the condyle), crown lowered 3 mm (25 -> 22 mm half depth at 1:50), so the long flat-crowned gull head replaces the high round dove head.
- Neck 1:50 and 1:100: lower neck moved dorsally and slimmed (39 -> 31 mm half depth), ends domed; upper neck shortened so it no longer pierces the crown; the throat now runs into the breast without the step.
- Primaries 1:50: tried a deeper wedge and a sunk front end; the deeper rows lifted the structure's primary shafts out of the body silhouette (`volumes-structure-coverage` 8.2 mm > 7.5 mm, caused by the reshaped `mirror` texture solid) and the bill reshape raised the bill area to 0.003 m² (+31 %): both reverted to round 3.
- Root: poses `gliding`, `gliding-up`, `gliding-down` fold the legs (thigh -9, shank -36, tarsus -33, foot -80 degrees from rest; trunk-relative), description of `gliding` corrected (en, de). The `soar` clip inherits them; the landing keys keep their reaching legs.
- Checks: `volumes-check.ts --product` PASS at four scales (silhouette IoU elevation 0.961 / 0.956 / 0.949 at 1:50 / 1:100 / 1:200); `r3-validate.ts animals/herring-gull --volumes` VALID (checkVolumes 0); spliced.

### Opened

1:50 body before and after (six views and right view, five iterations), `gliding` before and after, landing t = 1.6 s after, 1:50 structure before, 1:100 body after. Rendered, not opened: 1:200/1:500 body after, thermal and structure at 1:50 and 1:500 after.

### Remaining (reason)

- Spread wings stay grade C+: three rigid strips per wing on the folded-rest rig cannot open a feather fan; chord about 60 % of the real one with gaps at elbow and wrist. A widened tucked web (+22 mm) changed nothing visible and poked through the folded wing, so it was reverted. Fix needs feather bones in the rig (request in `tool-requests.md`).
- A small dark sliver of the primaries shows between coverts and secondaries on the folded wing; hiding it moved the structure shafts out of the silhouette.
- Back view busy around the vent (strip ends seen end-on); skull in the structure list without orbits.

## Flight feathers and tucked limbs reviewed November 21

Each wing has ten primary feathers inserted along the 135 mm hand and twelve secondary feathers inserted along the 140 mm ulna. Each feather has its own joint, tapered vane, structural shaft and heat-exchange surface at every scale. The outer two primaries carry the pale subterminal mirrors at 1:50. The hand and forearm retain the original folded sagittal centre of mass after the feather masses are distributed. Wrist, arm and leading-edge skin follow the wing at all scales. Splitting the two neck segments and the shank and tarsus at 1:500 lets the gull extend its head and tuck its legs in flight.

Measured primary spreads are 0° standing, 50° gliding, 43° landing and 24.5° at touchdown. The left and right P9 tips are 1.394 m apart in steady glide. Body, surface-temperature and structural images were checked in the gliding pose at four scales. All thirteen volume rules passed at each scale. The independent twelve-scene GLB readback passed at 5,855,020 bytes. The maintained skeleton suite passed all thirteen tests. [Primary feather measurements](digital-gull-fan.json) and [secondary feather measurements](digital-gull-wing.json).
