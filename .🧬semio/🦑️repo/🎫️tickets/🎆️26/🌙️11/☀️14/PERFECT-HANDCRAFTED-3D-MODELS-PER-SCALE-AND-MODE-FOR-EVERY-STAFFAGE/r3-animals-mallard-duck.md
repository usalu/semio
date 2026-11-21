# Round 3 — animals/mallard-duck (mallard drake, Anas platyrhynchos)

State (final, 2026-11-19): roots written to the catalog file, volumes spliced. `r3-validate.ts animals/mallard-duck` and `--catalog-only`: VALID (schema 0, validateAsset 0). `volumes-check.ts --product`: every rule green at every scale except one line at 1:50 and 1:100 (`areaDeviation:bill` 0.73 / 0.69, a physiological area I may not change; request filed in `author-requests.md`, "r3 animals/mallard-duck").

A first round-3 session did sections 1 to 3 and the first models and was cut off before looking at the final renders and before writing sections 4 to 8. A second session (resume) re-ran validator and check (both as described above), looked at every sheet, found the defects listed in section 4a and fixed them at the root and in the models, then finished this report.

Chain (all in `T`): `sh r3-mallard-duck-run.sh` (roots into the trial document, models, 2D redraw, trial document with volumes, validation), `sh r3-mallard-duck-run.sh catalog` (the same, then writes the catalog file and splices). Scripts: `r3_mallard_duck_roots.py` (rig, poses, anchors, materials, references), `r3-mallard-duck-kit.ts` (frames, flank lookup, lists), `r3-mallard-duck-body.ts` (1:50 body, every station by hand), `r3-mallard-duck-coarse.ts` (1:100, 1:200), `r3-mallard-duck-skeleton.ts` (structure), `r3-mallard-duck-model.ts` (lists, thermal), `r3-mallard-duck-decisions.ts`, `r3-mallard-duck-2d.ts` (ruling E1), `r3-mallard-duck-io.py`, `r3-mallard-duck-apply.py`, `r3-mallard-duck-full.py`, `r3-mallard-duck-sheet.ts` (six-view sheet of the trial document), `r3-mallard-duck-view2d.ts` (enlarged 2D preview), `r3-mallard-duck-inside.ts`, `r3-mallard-duck-probe.ts`, `r3-mallard-duck-tile.ts`. Input kept: `r3-mallard-duck-orig.json` (the asset before round 3, without volumes), `r3-mallard-duck-2d.json` (redrawn members).

## 1. Reference

| measure | value | source |
|---|---|---|
| total length, bill tip to tail tip (stretched) | 50-65 cm | Cramp & Simmons 1977 (asset reference) |
| mass, drake | 1.0-1.5 kg (asset 1.15 kg) | Cramp & Simmons 1977 |
| wing (wrist to tip of longest primary) | 257-285 mm | Cramp & Simmons 1977, as I remember the table |
| tail | 80-91 mm | same |
| bill (culmen from the feathers) | 51-61 mm, 21-23 mm wide, about 20 mm deep at the base | same; width and depth my knowledge |
| tarsus | 42-48 mm | same |
| middle toe with claw | 52-60 mm | same |
| humerus / ulna / carpometacarpus | about 92 / 78 / 57 mm | Woelfle 1967, as I remember the tables (authored estimates in the rig) |
| femur / tibiotarsus / tarsometatarsus | about 50 / 86 / 45 mm | Woelfle 1967, same remark |
| cervical column | 16 vertebrae, about 160-170 mm, carried as an S whose lower third lies inside the breast plumage | my knowledge, not a cited table |
| trunk skeleton | acetabulum to shoulder joint about 105-115 mm, acetabulum to pygostyle about 85-90 mm, sternum about 100 mm with a keel of 25-30 mm, pelvis 40-45 mm wide at the acetabula | my knowledge |
| body with plumage | about 28-30 cm from breast to rump, 12-13 cm wide over the folded wings, 12-13 cm deep | my knowledge |
| standing height | crown at about 0.30 m relaxed, 0.35 m alert; belly 4-6 cm over the ground; the ankle at the lower edge of the flank feathers | my knowledge |
| head | about 65 mm from nape to bill base, 45 mm deep, 38 mm wide; eye about 8 mm | my knowledge |
| drake plumage | dark green head and upper neck, white ring at the neck base, chestnut breast, pale grey flanks, black rump and tail coverts with two curled central feathers, white-edged tail, dark blue speculum on the secondaries | Cramp & Simmons 1977 |
| bone tissue | 2.1 g/cm³ | Dumont 2010 |

I had no web access to the tables in this run; values marked "as I remember" are stated so in the asset's references too.

## 2. Audit (state before round 3)

- Rig: the trunk was hinged at the tail base and 0.34 m long; the hips sat at (0.17, -0.06) in that frame, 60 mm below the axis at the level of the belly, so the legs hung under the body like a goose's and every water pose folded the knee up into the back. Neck 75 + 70 + 60 = 205 mm (real about 165 mm) rising from the top of the breast: with the 0.105 m head the drawing stood 0.46 m high, a goose. Hips and knees had no lateral position (both legs in the mid-plane), wings at z ±58 mm outside the body, 0.30 m long from a point on the back. Foot bone 75 mm (middle toe 52-60 mm). Leg bone lengths 50 / 85 / 45 mm were right.
- `dimensions`: 0.46 high, 0.60 long, 0.20 wide; a standing mallard is about 0.30 high, 0.43 long (0.50-0.65 is the stretched length of the skin), 0.135 wide.
- 2D: goose proportions (neck longer than the body is deep), wings as bars beside the body in plan and front, legs as 17-22 mm strips (real tarsus 9 mm), front view an ellipse with a neck column, tones inverted (neck ring and speculum black, head light), no eye, tail a short stub.
- 3D (`qa-visual-3.md`, B- / C+ / B): the body an egg with a tapered goose neck, wings as leaf plates standing off the flank, legs in the mid-plane; thermal with a plate through the back; structure one rod per rig bone.

## 3. Root fixes (catalog asset)

| member | old | new |
|---|---|---|
| trunk | origin at the tail base, length 0.34 | origin at the hips, length 0.105 (hips to neck base) |
| trunk centre of mass | (0.17, -0.005) | (0.035, -0.035): ahead of the hips, low (flight muscles on the keel; authored estimate) |
| tail | (0, 0.012), 172° | (-0.088, 0.004), 160° (pygostyle 88 mm behind the hips, tail carried level with the back) |
| neck-1 / neck-2 / neck-3 | at (0.3, 0.035), 75 / 70 / 60 mm, rest 62 / 20 / -30° | at (0.105, -0.02) (thoracic inlet between the shoulders), 60 / 55 / 50 mm, rest 46 / 53 / -18° (the lower neck runs forward inside the breast) |
| head | rest -62° | rest -91° (the sum of the neck and head rest angles is unchanged, so every pose keeps its head direction) |
| wings | at (0.225, 0.03), z ±0.058, 180°, 0.30 m | shoulder joint at (0.105, -0.015), z ±0.03, 171.5°, 0.248 m (to the primary tips over the rump) |
| thighs | at (0.17, -0.06), z 0 | at the origin, z ±0.02 (acetabula) |
| shanks | z 0 | z ±0.018 (knees splayed to ±38 mm, feet 76 mm apart) |
| feet | 75 mm | 58 mm (middle toe) |
| poses, all 19 | root x -0.18 | root x 0 (the origin moved to the hips) |
| poses `waddle-0` .. `waddle-7` (resume) | thigh offsets up to +106.5 deg (old angles on the new hips): the swinging knee stood 43 mm over the hip, the far thigh poked through the back at t 0.3 and t 0.7 of `waddle` | solved anew by `walk_legs`: toe joint and foot direction kept (planted and lifted feet unchanged), tarsus turned as little as possible until the ankle is 80 to 130 mm from the hip, thigh and shank from the forward IK solution; every knee now at most 3 mm over the hip, inside the flank (e.g. `waddle-3` far thigh 106.52 -> 6.96, tarsus -30.11 -> 94.86, foot -15 world kept) |
| poses `standing`, `wing-stretch` | far leg 60 mm behind the near one, shank level under the rump | far foot 22 mm behind the near one (thigh 12, shank -63, tarsus 34) |
| water poses (swimming, dabbling, upending, paddle-0..5) | legs folded for hips at the belly | thigh and shank solved anew: ankle up to 45 mm further towards the belly side (never beyond 118 mm from the hip), knee ahead; tarsus and foot directions kept |
| anchors | contacts (0.04, -0.01), footprint 70 x 50; waterline (0.17, -0.02), 300 x 120; nostrils (0.058, 0.011); bill-tip (0.105, -0.003); eye (0.032, 0.014); back-plume (0.17, 0.095) | contacts (0.03, -0.006), 58 x 50; waterline (0.03, -0.05), 260 x 120; nostrils (0.064, 0.0095); bill-tip (0.1055, -0.0024); eye (0.029, 0.0158); back-plume (0.07, 0.0372) on the back between the wings |
| materials, references | - | materials `bone`, `eye-tissue`; references `woelfle-1967`, `dumont-2010`; the biometrics added to `cramp-simmons-1977` |
| `dimensions` | 0.46 / 0.60 / 0.20 | height / length / breadth of the 1:50 model in the standing pose (see section 4). The length agrees with the cited bone and feather lengths; standing height and breadth are not cited measurements themselves |
| `representations` 50 / 100 / 200 | see audit | every shape redrawn (below); decisions `silhouette`, `retained`, `merged`, `omitted`, `thinElements` rewritten; 1:500 `reason` restated with the new sizes |

`areaM2` values untouched. `structure.supports` is empty in this asset (contacts come from the poses), so there are no support solids. Joint limits unchanged; every pose lies inside them.

## 4. Models (final)

### 4a. Defects found on the renders in the resume session and fixed

| seen | cause | fix |
|---|---|---|
| far primaries stood as a fin 12-16 mm above the rump (three-quarter from behind, 1:50 and 1:100; 1:200 wing tip the same) | rear strip stations had both edges forced to one lateral value, so the section stood upright | `strip` takes a separate lateral position for the upper edge (`zTop`); the rear stations now run from the flank across the back towards the midline (z 13 -> 3 mm), so the primaries lie on the rump and the tips of both wings nearly meet over it, as on a standing drake |
| upper wing edges read as rods along the back silhouette | superellipse exponent 3.0-3.4 gave box sections with a rounded 6 mm bead at each edge | sections lens-shaped (exponent 1.7 scapulars, 1.9 primaries, 2.0 wing and speculum, default 2.0 at 1:100/1:200): feather edges run out flat on the plumage |
| folded wing 4 mm proud of the flank (width 0.134 m, front and plan disagreed by 7.5 mm) | outer surface offset 4.5 mm | wing proud 2.7 mm at 1:50 (speculum 3.8), 2.8 mm at 1:100, 3.6 mm at 1:200: width over the wings 0.130 m (reference 12-13 cm) at every scale |
| neck a uniform 46 mm tube, head hardly wider than the neck from ahead | neck-2/neck-3 half widths 18-23 mm | neck tapers 45 -> 31 mm wide under the head (head 39 mm): 1:50 `NECK_2_50`, `NECK_3_50`; 1:100 and 1:200 `neck-upper` the same |
| white ring a raised collar, 16 mm wide | ring 1.5 mm proud | ring 0.6 mm proud, 13 mm wide |
| tail a round paddle 54 mm wide | fan half width 27 mm | wedge 43 mm wide at the base, pointed (all scales) |
| feathered tibia stuck out under the belly as a stick (far leg, left side) | drumstick tapered to 4 mm at 0.84 of the shank | drumstick ends at 0.73 as a fuller tuft that blends into the flank, bare shank from 0.6 (1:50, 1:100); thigh ends 7 % past the knee instead of 15 % |
| 2D elevation 1:50: near shank drawn into the flank; 1:200: near leg stroke drawn into the body | the strokes lay above the body in the layer order | near shank and near leg stroke at z 2.95 in the elevation (under the body, over the far leg): only the part under the belly shows, as on the body |
| `waddle` t 0.3 / 0.7: far / near thigh through the back | see root fixes (`walk_legs`) | poses solved anew |

### 4b. Per scale and list (solid counts body / thermal / structure)

- 1:50 (41 / 63 / 65): hull in three tone lofts (rump, flanks, breast) on 17 hand stations; neck-1 inside the breast, neck-2, neck-3 and the ring; head and bill as lofts across the bill axis with both eyes; tail wedge and the curled central feathers; per side primaries, wing, speculum and scapulars lying on the flank; thigh, drumstick, bare shank end, tarsus, three toes, two webs, hallux. Thermal: every plumage solid with its contour and down shell (C29, C31 unchanged), bill, legs, webs, two nares outlets, the back plume patch flush between the wings. Structure: skull with upper bill, mandible, three cervical segments, thoracic column, synsacrum with pubes, caudal vertebrae and pygostyle, sternum with keel, six ribs per side, coracoids, furcula, scapulae, folded humerus / forearm / hand, femur, tibiotarsus, tarsometatarsus, three toe bones, joints at the rig joints, masses at the trunk centre of mass and the head.
- 1:100 (24 / 26 / 47): authored anew: hull on 11 stations, two neck pieces, head, bill, tail; three wing strips per side; drumstick, shank end, tarsus, one web with the middle toe. Thermal one shell per region, nares as one outlet, plume patch. Structure: the same skeleton with merged ribs (three per side), slender cervicals, column and limb bones.
- 1:200 (14 / 16 / 25): hull with thighs in one mass, two neck pieces, head, bill, tail, two wing masses per side, each leg a slender shank end and a slender foot stroke. Thermal and structure simplified accordingly.
- 1:500: unavailable (unchanged reason, the drake is 0.9 mm long at that scale).

Final check line: `FAIL: 2 measurement(s) over 2 rule line(s)` = `volumes-thermal-coverage` 1:50 `areaDeviation:bill` 0.732 and 1:100 0.694 (limit 0.3); every other rule green at 1:50, 1:100, 1:200 (silhouette IoU elevation / front / plan 0.985 / 0.963 / 0.992 at 1:50, 0.974 / 0.943 / 0.983 at 1:100, 0.940 / 0.891 / 0.979 at 1:200, views-disagree, bounds, feature, monotonic, cross-scale, structure coverage, bones, slender, copied).

## 5. What I saw in the final renders

All with `r3-sheet.ts` (six views, rig on and off), plus `r3-mallard-duck-view2d.ts` for the 2D and `r3-2d.ts --file` for the product drawing.

- 1:50 body, front: upright drake, head 39 mm wide over a neck that narrows under it, bill seen end-on centred, white ring flush, chestnut breast, the folded wings show as a thin rim beside the flank, legs 76 mm apart with webbed feet turned slightly out. Right side (elevation): flat belly 4-5 cm over the ground, full breast ahead of the neck base, S-neck, rounded crown, the spatulate bill, wing covering the flank from the shoulder to the rump with the speculum, primaries over the rump, white wedge tail with the black curl. Back: wing tips nearly meeting over the rump, the curl between them, the tail wedge. Left side: mirror of the right side; the far leg's feathered tibia shows as a small tuft at the lower flank. Top: teardrop body, wings along both flanks, back strip between them, tail wedge, head and bill on the axis. Three-quarter from front and from behind (turn 50 / 210 / 230): the wing edges run out flat on the plumage, no fins, no rods, no gaps.
- 1:50 poses and clip: `upending` (head and neck under water, legs paddling up, wings folded on the body) and `waddle` t 0.3 and t 0.65: stance foot flat on the ground, swing foot lifted with the toes folded back, both knees inside the flank.
- 1:100 / 1:200 body: the same drake with fewer sections; recognisable at a glance in every view; 1:200 legs as single strokes.
- Thermal 1:50 and 1:100 (cut): nested contour/down shells per region, the green neck down inside the breast where the lower neck runs, the bill and legs single, the back plume patch flush between the wings, nothing poking out of the body.
- Structure 1:50 and 1:100: keel and sternum under the ribcage, ribs, scapulae along the back, furcula, the folded wing bones along the flank, femur forward and down from the acetabulum, tibiotarsus back to the ankle, tarsometatarsus and toes; the cervical column follows the S in the neck; masses magenta at the trunk centre of mass and the head.
- 2D 1:50 / 1:100 / 1:200: the true orthographic views of these bodies (layer order by visibility), the legs only where they show under the belly.

## 6. Still imperfect (stated plainly)

- `bill` thermal area: the modelled bill at its cited size has 0.0037-0.0038 m² outer area against the declared 0.0022 m²; I may not change `areaM2` (C18/C27); request filed.
- Plumage is modelled as smooth tone zones: no individual feather edges (tertials, flank feather rows) at 1:50; the drake's vermiculation is a tone, not geometry. A zoologist would accept the silhouette and the proportions, not a feather-level portrait.
- The feathered tibia shows as a small tuft at the lower flank of the far leg in the standing pose and as an oval flank tuft while walking: a real mallard shows the same flank-feather bulge but softer.
- `waddle`: the stance foot sweeps about 150 mm per stance (advance 0.25 m per 0.8 s cycle), so the femur still swings about 130 degrees through the cycle (real ducks walk with a nearly fixed femur and shorter steps at higher frequency). Every knee is now inside the flank; a shorter, faster stride would be more lifelike, but the clip's timing, contacts and advance are root data I left unchanged.
- The neck-1/neck-2 seam and the head/neck seam show as faint creases at 1:50 (separate links per bone so that every pose stays closed).
- Structure at 1:100 and 1:200: cervical, column and limb bones are slender members (3-5 mm), drawn as lines; they are thinner than the real bones (7-9 mm) because they are under the feature size of those scales.

## 7. Files

Changed in the resume session: `r3-mallard-duck-body.ts` (strip with `zTop`, rear primaries, lens sections, neck, ring, tail, drumstick, thigh), `r3-mallard-duck-coarse.ts` (rear primaries / wing tip, neck, tail, drumstick, wing proud), `r3_mallard_duck_roots.py` (`walk_legs` for the waddle keys), `r3-mallard-duck-2d.ts` (near shank and near leg stroke layer), new `r3-mallard-duck-width.ts` (lateral extents per solid). Outputs: `volumes/animals/mallard-duck.json`, `r3-mallard-duck-2d.json`, the catalog file `🐕️animals/mallard-duck/🔣️.json` (rig, poses, anchors, provenance, materials, representations, dimensions 0.30 / 0.437 / 0.13 m, volumes).

## Round 5 handcraft (2026-11-21)

Scripts: the asset's chain, edited in place (`r3-mallard-duck-body.ts`: wing layer order, head and bill width, eye; `r3-mallard-duck-coarse.ts`: heads, bills and wing layers of 1:100, 1:200, 1:500; `r3-mallard-duck-model.ts`: 1:200 nares outlet). Build and splice: `sh r3-mallard-duck-run.sh catalog`. Sheets: `bash r5-fauna-render.sh animals/mallard-duck <before|after> s2 swimming dabbling upending wing-stretch waddle@0.3 paddle@0.3 dabble@2` into `🗑️generated/r5-mallard-duck/`.

### Reference table (modelled to)

| measure | reference | source |
|---|---|---|
| length bill tip to tail tip, standing (not stretched) | about 0.43-0.44 m (stretched skin 0.50-0.65 m) | Cramp & Simmons 1977; standing length own knowledge |
| crown height standing relaxed / breadth over the folded wings | about 0.30 m / 0.12-0.13 m | own knowledge (round 3 table) |
| bill culmen / width | 51-61 mm / 21-23 mm | Cramp & Simmons 1977; width own knowledge |
| head width with plumage | about 38 mm | own knowledge |
| tail | 80-91 mm | Cramp & Simmons 1977 |
| tarsus / middle toe with claw | 42-48 / 52-60 mm | Cramp & Simmons 1977 |
| femur / tibiotarsus / tarsometatarsus | about 50 / 86 / 45 mm | Woelfle 1967 |
| humerus + ulna + carpometacarpus (folded wing to the primary tips) | about 92 + 78 + 57 mm, wing chord 257-285 mm | Woelfle 1967, Cramp & Simmons 1977 |

### Model vs reference (1:50, standing, spliced catalog; `r5-fauna-measure.ts`, rig)

| measure | model | reference | deviation |
|---|---|---|---|
| bounding length | 438 mm | 437 (dimensions) / about 430-440 | 0.2 % |
| height / breadth | 300 / 130 mm | 300 / 120-130 | 0 % / in range |
| bill along the axis (feathering to tip) | 56 mm | 51-61 | in range |
| bill width | 22 mm (was 24, +4 %) | 21-23 | in range |
| head width | 38 mm (was 40, +5 %) | 38 | 0 % |
| tail bone / feathers | 85 mm | 80-91 | in range |
| femur / tibiotarsus / tarsus / middle toe (rig) | 50 / 85 / 45 / 58 mm | 50 / 86 / 45 / 52-60 | under 1.2 % |
| folded wing bone (shoulder to primary tips) | 249 mm | wing chord 257-285 from the wrist | short of the chord because the folded wing bone starts at the shoulder and the primaries tips lie over the rump (as round 3) |

### Grades (before -> after)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B+ (speculum and primaries broke through the wing coverts as a row of tiles along the lower rear wing edge; head 5 % and bill 4 % too wide) -> A- | A- -> A- (nested contour/down shells, plume patch flush) | B+ -> B+ (keel plate deep in the front view; unchanged) |
| 1:100 | A- (head wedge from the front, tiny primary slivers under the wing) -> A- | B+ (not opened) | B+ (not opened) |
| 1:200 | B (head a flat-crowned wedge, ragged lower wing edge with the flank showing through in patches, pointed bill) -> A- | B+ (rendered, not opened) | B (opened after: deep sternum wedge, bulky skull; unchanged) |
| 1:500 | B (wedge head, cone bill) -> B+ | B+ (opened after) | B (slender bars; not opened) |
| poses | waddle t 0.3 A- (both knees inside the flank, stance foot planted), upending B+ -> unchanged; swimming, dabbling, wing-stretch, paddle t 0.3, dabble t 2 rendered | | |

### Changes

- 1:50 wing: layer order of the folded wing put right: primaries under the coverts (proud 2.2 mm, they show only behind the wing end, as on a folded mallard wing), speculum above the coverts (6.2 mm) so it reads as one continuous band with its edge instead of interleaved tiles.
- 1:100 / 1:200: coverts 4.2 / 5.6 mm and scapulars 7.4 mm proud, so the wing edge is one clean line with no flank showing through; 1:200 nares outlet 18 mm (the feature floor of the redrawn head).
- Heads at 1:100, 1:200, 1:500 restated with an occiput station and a domed crown (7 and 6 stations, Catmull-Rom between them) instead of the flat-crowned wedge; bills at 1:200 and 1:500 end in the rounded spatula with the nail station instead of a cone point.
- Head lateral half widths 4 % narrower and bill 5 % narrower at every scale (head 38 mm, bill 22 mm); eye moved 0.6 mm inwards to stay flush.
- 2D of 1:50, 1:100, 1:200 redrawn from the new bodies by the chain (ruling E1); 1:500 unavailable as before (the drake is 0.9 mm long).
- Checks: `r3-validate.ts animals/mallard-duck` VALID (schema 0, validateAsset 0); `--volumes`: checkVolumes 2, the same two `areaDeviation:bill` lines as round 3 (1:50 0.75, 1:100 0.72); `volumes-check.ts --product` otherwise green (cross-scale IoU 0.99-1.0, silhouette as round 3). Spliced.

### Opened

1:50 body before and after (six views, three-quarter from behind twice), 1:200 body before and twice after, 1:500 body before and after, 1:100 body after, 1:50 structure before, 1:200 structure after (B: sternum a deep wedge plate, skull bulky, limb bones slender bars), 1:50 thermal cut after, 1:500 thermal cut after (B+: tidy regions, plume patch on the back), pose `upending` before, pose `swimming` after (A-: paddling legs, body closed), clip `waddle` t 0.3 after, 2D 1:50 after. Rendered, not opened: thermal 1:100/1:200, structure 1:100/1:500 after, poses dabbling, wing-stretch, clips paddle t 0.3 and dabble t 2, 2D 1:100 and 1:200.

### Remaining (reason)

- `bill` thermal area 0.0038 m² against the declared 0.0022 m² (C18/C27, physiological value, already in `author-requests.md`): checkVolumes stays at 2.
- Plumage as smooth tone zones with straight seams between rump, flank and breast; no feather edges at 1:50.
- `wing-stretch` and any spread wing: one rigid wing strip per side (feather bones requested in `tool-requests.md`, 2026-11-21).
- Coarse thermal and structure sheets and several poses were rendered but not opened in this round.
