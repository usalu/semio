# vegetation/scots-pine — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. One real defect of the model was removed (two needle plates floating at 1:500), one 2D drawing error corrected, the body decisions of three scales rewritten. The 1:50 foliage, the bark relief, the rig of the two small limbs and the crown plan were judged and are listed as open; they were not rebuilt in this pass.

### State found

- No earlier report for this asset; no batch report in `T/rework/*.md` mentions it. Source `T/volumes/vegetation/scots-pine.json` and catalog `asset.volumes` were identical.
- `r3-validate.ts vegetation/scots-pine --volumes` at the start: `INVALID … checkVolumes 1` — `volumes-connected 1:500: "pl-n-c1" (+9) 183.3 mm from "limb-e1"`: at 1:500 the two small limbs (`limb-n`, `limb-s`) had been deleted from body, thermal and structure while their needle plates were kept, so 20 lobes hung in the air.
- Rendered and read: 1:50 body (six views, three-quarter and right close-ups without rig), 1:50 thermal, 1:50 structure, 1:500 body, 1:500 structure, 1:200 thermal, poses `storm` and `snow` at 1:50, 2D at 1:50 and 1:500. Rendered but not read (budget): pose `windy`, clips `gust` t 2/5, `wind-sway` t 0.9/2.7, `diurnal-transpiration` t 6/14, 1:100 body/thermal/structure, 1:200 body/structure, 1:500 thermal.

### Reference table (Pinus sylvestris, mature forest-edge tree) against the model

| Feature | Reality | Model | Verdict |
|---|---|---|---|
| Height | 20–30 m mature (up to 35–40 m) | 22.18 m | right |
| Diameter at breast height | 35–55 cm at 22 m; h/d 45–60 forest-grown | bole section d 50 cm (lower part), flare 0.92 m at the ground | right |
| Taper | about 1 cm per m in the clear bole, faster in the crown | 50 cm (0.4–7.4 m), 34 cm (7.4–13.4 m), 25 cm (13.4–17.6 m), leader 20 → 9 cm | right; three stepped sections rather than a continuous taper in the rig sections, the body solids taper continuously |
| Clear bole | lower branches shed, 50–60 % of height bare; dead stubs and snags remain | first limb at 11.9 m (54 %), two stubs at 5.6 and 6.6 m, three snags | right |
| Bark | lower bole: grey-brown plates 5–10 cm wide, fissures 1–3 cm deep; upper bole and limbs: thin orange flaking bark | two tones with a clean change at 7.4 m; smooth surfaces in 3D, fissure strokes only in the 1:50 elevation | bark relief not modelled (fissures 10–30 mm are above the 7.5 mm feature of 1:50) |
| Branching | pseudo-whorls of 3–6 branches per annual shoot; mature crown keeps a few strong, sinuous limbs leaving at 50–80° and turning up at the end | six main limbs in three pairs at 46–64° from the bole axis, crooked, turning up; two small limbs | plausible; limbs a little steep (30–40° on the upper pair) |
| Limb base diameter | 10–20 cm for a 4 m limb on a 25–34 cm bole | 11–12 cm at the collar swelling, 9–10 cm behind it, 10 cm tips of the plate branches | at the thin end of the range |
| Crown form | flat-topped or umbrella-shaped, open, of separate flat needle plates, sky between the tiers | nine plates in tiers, sky between them | right, the species reads at once |
| Crown spread | 6–10 m, roughly round in plan, irregular | 12.08 m × 7.91 m (1.5 : 1), nearly rectangular in plan | **deviation**: all six main limbs lie within ±40° of the rig plane (plan azimuths 157, 338, 203, 24, 140, 320°); only the two small limbs go across |
| Needle plates | flat cushions 0.4–0.8 m thick: bare branch fan below, shoots with needle brushes (needles 4–7 cm in pairs, kept 2–3 years on the outer 20–40 cm of every shoot) standing up on top | stacks of 10–15 smooth ellipsoid lobes (0.6–1.25 m half extents) with four small "tuft" lobes, branch axis and six side branches below | **deviation**: reads as cloud/balloon clusters; no shoot or needle form at 1:50, round undersides instead of flat ones |
| Mass | 2.2 m³ stem wood × 850 kg/m³ green + crown 400–600 kg ≈ 2.4–2.9 t | 2800 kg | right |
| Rig | bole bends progressively under wind, limbs and plates swing; every limb is a cantilever on the bole | root plate (fixed), bole in three parts + leader, six limbs each with a plate bone, two stubs; plates carry a lateral `z` | right for the six limbs; the two small limbs and their plates are bound rigidly to `trunk-3` and neither sway nor droop under snow |

### What was changed (script `T/rework-scots-pine-coarse-limbs.py`)

- **1:500 body**: `limb-n` and `limb-s` modelled for the 75 mm feature — the true path of the 1:200 limbs (bole surface to the plate centre, three sections), radii by hand 62/52/46 mm and 60/52/46 mm (tip thickened from 44 mm to 92 mm diameter so the limb stays above the feature). Both plates now sit on their limbs.
- **1:500 thermal**: `th-limb-n`, `th-limb-s` (region `limb-bark`, by reference).
- **1:500 structure**: `member-limb-n`, `member-limb-s` (wood inside 4 mm bark, joined to the bole member), so the load path of the two plates reaches the bole.
- **1:500 2D** (catalog `representations.500.geometry`, text splice, every other byte of the member kept): `limb-n`, `limb-s` in elevation, front and plan directly below `bole-top`; plan: `limb-e2` moved from z 3.0843 to 2.999, it was drawn as a diagonal stroke across the needle clouds although it lies under them (visible in the earlier `r3-2d` plan, gone now).
- **Decisions**: the body text of 1:100, 1:200 and 1:500 was one identical generic paragraph; each now says in en and de what that scale models and leaves out; structure text of 1:500 corrected.
- Published with `volumes-insert.py`; the catalog file grew from 3.8 MB to 6.6 MB only because `volumes-insert.py` writes the `volumes` member in its own layout (the catalog member had been written compactly by another tool); content of all other members unchanged (compared member by member: only `volumes` and `representations.500.geometry` differ).

### Validation

- Before: `INVALID vegetation/scots-pine …: schema 0, validateAsset 0, checkVolumes 1`.
- After publishing: `VALID vegetation/scots-pine (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`--catalog-only --volumes`, full run; repeated after the plan z change, see last line of this report).

### Judged and left as it is, with the reason

- **Lobes at the coarse scales**: 1:100, 1:200 and 1:500 carry the same 122 needle lobes as 1:50 with fewer faces. A lobe is 1.2–2.5 m across, 16 to 33 smallest features of 1:500, so it is a legitimate form at every scale and the 2D of every scale is the true outline of these lobes; what the coarse lists drop (surface roots, knots, snags, side branches, whorl swellings) is what falls under the feature. Not rebuilt.
- **Poses** `storm` and `snow` read naturally: progressive lean of the bole, plates tilted with the wind, limbs and plates drooping on both sides under snow; no crack opens at a limb collar or between limb and plate.

### Open list for the next pass (precise)

1. 1:50 body, needle plates (all nine: `pl-w1/e1/w2/e2/w3/e3/top`, `pl-n-*`, `pl-s-*` on `trunk-3`): smooth ellipsoid stacks; a sculptor's plate is a flat cushion with a flat, open underside showing the branch fan and an upper side of upright shoots with needle brushes. Rebuilding changes every silhouette, so elevation, front and plan sprays of all four scales must be redrawn with `r3-project2d` (thermal by-reference entries follow the ids).
2. 1:50 body, `trunk-1` and `root-plate`: plated bark relief (plates 5–10 cm, fissures 1–3 cm) not modelled; `trunk-2/3`, limbs: smooth, acceptable (thin flaking bark is below the feature).
3. Rig: `limb-n` and `limb-s` with their plates have no bones (solids on `trunk-3`); they need two limb bones resting laterally (`T/rig-lateral-rest.md`) plus plate bones, bound in poses `windy`, `storm`, `snow` and clips `wind-sway`, `gust`, and the solids, 2D shapes, structure members and masses re-expressed in the new frames at all four scales.
4. Crown plan 12.1 m × 7.9 m: at least one pair of main limbs should leave across the rig plane (or the small limbs be grown to full limbs of 3.5–4 m) so the plan becomes roughly round, 9–10 m; changes `dimensions.depthM`, the plan symbol, front and plan drawings of all scales and the wind loads.
5. Limb collars 11–12 cm on a 25–34 cm bole are thin for 4 m limbs carrying 45–68 kg plates; 14–18 cm would be truer (body, structure members and `rig.bones[].section` of the six limbs, all scales).
6. Not read in this pass after the last change (rendered only, or not rendered again): pose `windy`, all three clips, 1:100 and 1:200 in all three lists, 1:500 thermal sheet, 2D of 1:100 and 1:200. Nothing was changed at 1:50, 1:100 or 1:200 geometry, so these are the state found.
7. 1:500 2D `limb-n` / `limb-s` strokes are the 1:200 outlines (tip drawn 44 mm wide against the 92 mm of the 1:500 solid, 0.1 mm on paper); passes `volumes-silhouette`, could be redrawn to the thickened tip.

### Fundamental issues (outside this asset)

- `volumes-insert.py` rewrites the `volumes` member in an expanded layout while the catalog file of this asset held it (and `representations`) compactly on one line each: the first publish of an asset therefore changes about 190 000 lines and nearly doubles the file (3.8 → 6.6 MB) without any change of content. One layout for both writers would keep diffs readable.
- The catalog file mixes two number spellings (`0.00001886` from a JavaScript writer, `1.886e-05` from Python): a Python `json.dumps` round trip of the whole file is not byte-stable, so root fixes outside `volumes` have to be text splices (as done here).
- A full `r3-validate.ts … --volumes` run of this asset takes 6 minutes wall time (159 CPU seconds) under the shared load; the per-scale form (`--scale 500 --rules …`) took under a minute.

Final proof after the last change (2026-11-22): `r3-validate.ts vegetation/scots-pine --catalog-only --volumes` → `VALID vegetation/scots-pine (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; source and catalog `volumes` agree; generated PNGs and logs of this asset deleted.
