# Round 3 — animals/pigeon (rock pigeon, Columba livia)

State: roots written to the catalog file, volumes spliced. `r3-validate.ts animals/pigeon`: VALID (schema 0, validateAsset 0). `volumes-check.ts --product`: 1:200 and 1:500 PASS; 1:50 and 1:100 fail one line each (`areaDeviation:legs-feet`, a physiological area I may not change; request filed). My own grade: body B+ at 1:50, B at 1:100, B- at 1:200; structure B at 1:50, C+ coarser; thermal B+; 2D elevation and plan B+, front B-. It reads as a pigeon from every side; it is not "perfect".

I continued the work of an author who was cut off: his rig fix was already in the catalog (which left the asset INVALID, 6 extent issues, because the 2D was still drawn for the old rig), his 1:50 body existed, thermal, structure, coarse scales, 2D and report did not.

Chain (all in `T`): `sh r3-pigeon-run.sh` (trial document, validation), `sh r3-pigeon-run.sh catalog` (writes the catalog file and splices). Scripts: `r3-pigeon-kit.ts` (frames, flank lookup, station smoothing), `r3-pigeon-body50.ts`, `r3-pigeon-coarse.ts` (every station by hand), `r3-pigeon-skeleton.ts`, `r3-pigeon-model.ts` (lists, thermal), `r3-pigeon-decisions.ts`, `r3-pigeon-2d.ts` (ruling E1), `r3_pigeon_roots.py`, `r3-pigeon-io.py`, `r3-pigeon-apply.py`, `r3-pigeon-full.py`, `r3-pigeon-view2d.ts` (enlarged 2D preview), `r3-pigeon-inside.ts`, `r3-pigeon-probe.ts`, `r3-pigeon-validate.ts`. Inputs kept: `r3-pigeon-orig.json`, `r3-pigeon-orig-materials.json` (untouched snapshot), `r3-pigeon-2d.json` (redrawn members).

## 1. Reference

| measure | value | source |
|---|---|---|
| body length, bill tip to tail tip | 29-36 cm | Johnston & Janiga 1995 (asset reference) |
| mass | 230-380 g (asset 0.35 kg) | Johnston & Janiga 1995 |
| femur / tibiotarsus / tarsometatarsus | 39.1 / 58.1 / 31.7 mm (males), 37.1 / 56.0 / 30.2 (females) | Johnston 1990, Wilson Bull. 102 |
| humerus / ulna | 45.4 / 54.9 mm (males), 43.5 / 52.6 (females) | Johnston 1990 |
| wing chord (wrist to tip of longest primary) | about 223 mm | Gibbs, Barnes & Cox 2001 |
| tail | 95-110 mm | Gibbs et al. 2001 |
| bill (exposed culmen) | about 18 mm, cere on its basal half | Gibbs et al. 2001 |
| tarsus thickness, toe thickness | about 5 mm, about 4 mm; three front toes about 35 / 27 / 29 mm, hallux about 17 mm | my knowledge, not a cited table |
| head with plumage | about 32 mm wide, 33 mm deep; eye about 9 mm, mid-way between bill base and nape | my knowledge |
| folded wing | wrist at the side of the breast, elbow at the back, primaries along the lower edge, tips 2-3 cm short of the tail tip | my knowledge |
| sternum | about 70 mm long with a keel about 25 mm deep; 12 cervical vertebrae in an S; glenoid to acetabulum 50-60 mm | my knowledge (Baumel et al. 1993 for the names) |
| bone tissue | 2.1 g/cm³, stiffer than mammal bone | Dumont 2010 |

## 2. Audit (state before round 3)

- Rig: trunk hinged at the tail base (165 mm long), hips and shoulders had no real lateral position (hips z 0, shoulders z ±50 mm, wider than the bird), femur 45, tibiotarsus 55, tarsus 28, humerus 60, ulna 75, hand 150 mm; folded wing angles gave no Z-fold; jaw joint 34 mm in front of the skull joint (mid-bill) and 20 mm long.
- `dimensions`: 0.25 high, 0.33 long, 0.15 wide; a folded pigeon is about 0.10 wide.
- 2D: toe bars 64 mm wide and legs 14 mm thick (real 4-5 mm), wings as boxes at z 45-75 mm outside the body, front and plan constructed from ellipses and capsules, thighs drawn over the wing.
- 3D (`qa-visual-3.md` C+/B-): wings flat plates on the flank, cone head, tail and toes missing against the drawing, 1:200 a bean bag; structure was a rod per bone, legs and bill at the 7.5-30 mm floor (legs-feet area 0.016 against 0.002).

## 3. Root fixes (catalog asset)

| member | old | new |
|---|---|---|
| trunk | origin at the tail base (x -0.05), length 0.165 | origin at the hips, length 0.11; neck, tail and shoulder offsets shifted so that they stay in place |
| trunk centre of mass | on the axis (comY 0) | comY -0.022 (flight muscles on the keel; authored estimate guided by Pennycuick 2008) |
| thigh / shank / tarsus length | 0.045 / 0.055 / 0.028 | 0.039 / 0.057 / 0.031; hips at z ±0.02 |
| humerus / forearm / hand | 0.06 / 0.075 / 0.15, z ±0.05, rest angles -167 / 163 | 0.045 / 0.054 / 0.2, z ±0.022, rest angles -200 / 202 (real Z-fold) |
| jaw | x 0.034, y -0.006, length 0.02 | x 0.008, y -0.009, 7.5°, length 0.046 (quadrate joint) |
| joint limits | - | thigh, shank, tarsus, toes, forearm, hand adapted to the new rest angles |
| poses (23) | - | every leg re-solved so that the foot joint and the directions of tarsus and toes stay; open-wing offsets shifted by the change of the rest angles; `pecking-reach` and `pecking-strike` re-pitched (beak tip off by 1.8 and 0.3 mm) |
| anchors | contacts y -0.0095; `eye` (0.024, 0.007); `back-plume` (0.107, 0.062) in the old trunk frame | contacts y -0.003; `eye` (0.022, 0.006); `back-plume` (0.0074, 0.0223), on the back between the wings |
| materials, references | - | material `bone`; references `johnston-1990`, `gibbs-2001`, `dumont-2010` |
| `dimensions` | 0.25 / 0.33 / 0.15 | 0.224 / 0.315 / 0.097 (height / length / breadth of the 1:50 model in the standing pose). Length agrees with the cited 29-36 cm; height and breadth follow from the cited bone lengths and are not cited measurements themselves |
| `representations` 50 / 100 / 200 | see audit | every shape redrawn (below); `decisions.silhouette`, `thinElements` (and `retained` at 1:50) rewritten, the other decision texts unchanged |

`areaM2` values untouched. `structure.supports` is empty in this asset (contacts come from the poses), so there are no support solids.

## 4. Model

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 35 | 38 | 75 |
| 1:100 | 30 | 33 | 49 |
| 1:200 | 15 | 20 | 30 |
| 1:500 | unavailable (as the representation) | | |

- 1:50 body: keeled trunk loft (12 stated stations, Catmull-Rom between them), undertail coverts, three overlapping neck links, head with steep forehead, upper bill, lower bill on the jaw bone, cere, two eyes, two tail layers; per side scapulars, coverts and primaries lying on the flank (looked up from the trunk surface), thigh, drumstick, bare shank end, tarsus, three front toes and hallux at true thickness.
- 1:50 thermal: references by region with the single plumage layer; nostril outlets on the cere, plume outlet as a flush patch of the back. No region has more than one layer, none is hidden.
- 1:50 structure: skull, mandible, three cervical segments, spine, synsacrum, pubes, pygostyle, sternum plate and keel, five ribs per side, coracoids, furcula, scapulae, humerus, forearm pair, manus, femur, tibiotarsus, tarsometatarsus, four digits per foot, feather shafts, 20 joints, masses for flight muscles and head.
- 1:100: fewer own stations, undertail in the trunk, feet as two bars; structure a bar skeleton with skull, sternum hull, synsacrum, three ribs, knee, ankle and toe joints.
- 1:200: trunk, neck, head with bill, tail, three wing masses per side, legs as two slender strokes each; bill, cere and eye are thermal references to the head mass, as the drawing merges them; structure one slender bar per bone plus skull, sternum, synsacrum, pygostyle.
- 2D (E1): each shape is the outline of the solids a hand-written table assigns to it, traced from a fine tessellation, simplified, set 0.25-1 mm inside the traced edge, mapped into the shape frame through the product's `evaluate` (residual 0.0000 mm). Layer order follows true visibility per view (thighs and drumsticks now under the trunk). Wing shapes: side-view profile extruded over a tapered lateral span (elevation), the same in up to six height bands with their own spans (front), thin sheets of the plan outline swept along the bone normal (plan). New shapes: drumsticks apart from the bare shanks, single toes, far-side scapulars and coverts. Strokes redrawn by hand: eye ring, neck sheen, one wing bar, three primary strokes, two breast scallops (open arcs instead of triangles), tail band; legs at 1:200 as 0.15 mm strokes.

Final check lines: 1:50 silhouette IoU elevation / front / plan 0.985 / 0.958 / 0.988, FAIL `areaDeviation:legs-feet` 0.61; 1:100 0.969 / 0.954 / 0.982, FAIL `areaDeviation:legs-feet` 0.58; 1:200 0.931 / 0.888 / 0.968, PASS; 1:500 PASS. These numbers follow from the redraw and prove nothing about quality.

## 5. What I saw (PNGs opened with the Read tool)

- 1:50 body, six-view sheet: side views read as a pigeon at once (full breast, small round head, short bill with cere, folded wing with the primaries along its lower edge, tail 2-3 cm past the wing tips, thin legs). Front: egg-shaped breast, wings as narrow rims. Back: two scapular ridges with a groove, coarse. Top: spindle with both wings converging over the tail. Three-quarter: convincing; the wing wrist is still angular.
- 1:50 structure sheet: a bird skeleton (S-neck, keel, wing Z-fold, digitigrade legs, ribs around the muscle mass). The skull is a cone with a braincase bulge, not a real skull form.
- 1:50 thermal sheet: every region coloured on the body, nothing poking out; nostril outlets too small to see at sheet size.
- 1:100 body sheet: same bird with harder edges. 1:200 body sheet: recognisable bird; head and bill one wedge, angular neck.
- 1:100 structure sheet: bar skeleton with sternum hull and pelvis shield; readable.
- Pose `wings-half-open` (3D sheet with rig and enlarged 2D): bones and wing pieces rise and spread in all views, 2D follows. Clip `walk-cycle` at 0.3 s: a walking pigeon, feet on the ground.
- 2D: product sheet 1:50 (6 mm long on paper, reads as a pigeon), enlarged previews 1:50 and 1:200.

Not opened: thermal and structure sheets of 1:200, thermal of 1:100 (rendered), the enlarged 2D of 1:100 and of `pecking-strike`, single-list renders from further directions (I used the six-view sheets instead), the product 2D sheets of 1:100 and 1:200.

## 6. Still imperfect

- Two check lines fail: `legs-feet` outer area 0.0032 m² against the declared 0.002 m² at 1:50 and 1:100. Hand calculation of the real bare legs gives about 0.0039 m²; request in `author-requests.md` ("r3 animals/pigeon").
- Open wings: each wing is three rigid pieces shaped for the folded pose. Opened, the coverts stand off as a flap and there is no secondary surface between arm and hand. A real spread wing needs skinned solids or pose-specific feathers.
- Skeleton thickness is ruled by `volumes-feature`: long bones 7.5 mm at 1:50 (real 3-5 mm shafts), digits and mandible as thick as toe and lower bill, slender bars of 1-3 mm at 1:100 and 1:200. Described in the same request.
- 2D front view: the wings are stepped prisms (necessity of the `solid` hint that the validator demands for bones that leave the paper plane). Outlines at 1:200 are angular.
- Seams: black depth steps where neck meets breast and nape in walk frames; scapular groove on the back.
- Representation decision texts `merged`, `omitted`, `hatching`, `styles`, `overlays` were not rewritten.
- No support solids (the asset declares no `structure.supports`).
