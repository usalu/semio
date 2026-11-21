# Round 3: `vegetation/multi-stem-birch` (*Betula pendula*, three-stemmed clump of about 13 m)

STATE_LINE

## 0. Starting point, scripts, resume

### What I found

- No earlier round-3 work on this asset: no `r3-multi-stem-birch-*` scripts, no report, no `🗑️generated/r3-multi-stem-birch/`. Started at step 1 of the brief.
- The catalog `volumes` differed from `volumes/vegetation/multi-stem-birch.json` at 1:200 and 1:500 (the bulk pass of 03:40 to 04:10 had written 31 / 28 body solids there against 28 / 8 in the source). Both were round-2 visual hulls of the old drawings; neither was kept. The source as I found it is saved as `🗑️generated/r3-multi-stem-birch/source-before-r3.json` until the ticket is closed.
- Round-2 state: reviewers graded it C/C/C/B- ("Single trunk, flat tiers: not multi-stem, not airy", `qa-visual-2.md` row 292). My first render confirmed it: nine blobs on sticks, all three stems in one plane, rod branches, no stool, no roots.

### Scripts (ticket folder)

- `r3-multi-stem-birch-tree.ts`: the tree stated by hand in world metres: stool sections and roots, stem stations, the six boned branch chains, the forty unboned branches (bone, height, heading, rise, arch, length, radius), their sleeves, side shoots and tresses, the shoots and spires of the leaders, the bark marks, the 1:500 boughs.
- `r3-multi-stem-birch-kit.ts`: one part type (loft through elliptic sections with stated axes), conversion to bone-local solids, exact outlines, smoothing. Quaternion, hull and union code taken over from the japanese-maple kit and owned here.
- `r3-multi-stem-birch-model.ts`: writes the volumes source and the candidate document (rig offsets, anchors, bark areas, drawings, decisions, symbol).
- `r3-multi-stem-birch-draw.ts`: ruling E1, all 2D drawings as true views, and the plan symbol.
- `r3-multi-stem-birch-texts.ts`: every decision text, English and German.
- `r3-multi-stem-birch-roots.ts`: writes named members of the candidate into the catalog file in its base style, checks that every other member stays equal and in order (copy of the maple writer, owned here).
- `r3-multi-stem-birch-run.sh`: the ship chain (model, validate the candidate, write roots, splice, validate the catalog); `--dry` stops before the catalog.
- `r3-multi-stem-birch-look.sh`: the `r3-sheet.ts` six-view sheet of the candidate document, so that a rig change can be judged before anything is written to the catalog.
- `r3-multi-stem-birch-extent.ts`, `r3-multi-stem-birch-frames.ts`: probes (which parts set the extents; bone frames).

### Resume

`sh r3-multi-stem-birch-run.sh`, then `$BUN volumes-check.ts vegetation/multi-stem-birch --product` (about 9 minutes on the shared machine).

## 1. Reference

| quantity | real subject | source | asset |
|---|---|---|---|
| species and habit | *Betula pendula* (silver birch), grown as a multi-stem clump: several slender stems from one stool, slightly splayed; main branches ascend at 40 to 60 degrees from the stem and arch outwards; second and third order branchlets are thin and pendulous and hang in tresses of 1 to 2 m, longest at the branch ends; crown light and open, narrow-ovate, the stems visible far up into it | botany (Mitchell, *Field Guide to the Trees of Britain and Northern Europe*; Johnson and More, *Collins Tree Guide*); asset `summary` | as modelled |
| height | 12 to 15 m at 20 to 30 years, to 25 m | Mitchell; nursery practice for clump birches | 13.018 m (`dimensions`, kept) |
| crown spread | 6 to 9 m for an open-grown clump of 13 m (wider than a single stem because the stems splay) | Pretzsch et al. 2015 (`pretzsch-2015`, crown width of open-grown urban trees); nursery data | 9.205 m by 6.94 m (`dimensions`, kept): the clump is wider along the row of its outer stems |
| stems | 3 stems, 14 to 20 cm at breast height, base 20 to 27 cm, about 6 cm at 10 m, then a leader of 1 to 5 cm; gently sinuous; fused at the base | rig sections (d 14 / 18 / 16 cm lower, 10 cm upper); taper of young birch | radius over bark 0.104 / 0.120 / 0.112 m at the stool, 0.082 / 0.094 / 0.088 m at 1.3 m above the stool, 0.057 / 0.065 / 0.062 m at the mid joint, 0.033 m at the stem top, leader to 6 mm |
| stool | one root stock about 0.6 to 0.75 m across at the ground with shallow surface roots; birch roots are shallow and wide | botany | stool 0.74 by 0.70 m at the ground, five surface roots to 0.8 m |
| bark | white, peeling in papery strips, with dark horizontal lenticel bands; base dark, thick and deeply fissured to 0.5 to 1 m, black diamond fissures above it; a black triangular scar under every branch | Mitchell; Collins Tree Guide; asset `summary` | dark base 7 mm thicker to 0.85 m, 27 lenticel bands, 15 fissures, 46 branch scars at 1:50 |
| branches | many thin first-order branches all round each stem, 2 to 5 cm at the base, 1.5 to 3 m long; branch angle 40 to 60 degrees | botany; rig branch sections d 5 cm | 6 boned branches (radius 36 to 19 mm, tips to 5 mm) and 40 unboned ones (radius 19 to 6 mm at the stem, 5 mm at the tip, 0.65 to 2.85 m long), rising at 48 to 68 degrees from the horizontal and arching to 0 to 34 degrees |
| leaf | rhombic to triangular, 3 to 7 cm, doubly serrate, on a long stalk | Collins Tree Guide | not modelled singly; leaf ticks in the 1:50 drawing |
| wood | green birch about 900 kg/m3 | Wood Handbook 2021 (`wood-handbook-2021`) | masses kept (660 kg) |

`dimensions` lie inside the reference and were not changed. Modelled extents: see section 4.

## 2. Audit (round-2 state)

### Rig

- Planar: all 22 bones had z = 0, so the three stems stood in one plane (a fan), and all three left the root plate at the same point (0, 0.3).
- In-plane joint heights and lengths are plausible against the reference: stems 9.5 to 10.3 m plus a leader bone of 1.8 to 2.0 m, lowest boned branch at 3.7 m, branch angle 45 to 50 degrees from the stem, branch sections d 5 cm. They are kept.
- Defects fixed at the root (section 3): no lateral spread, no stool.

### Body, all scales

- Foliage: visual hulls of the old drawings, 20 lobes in 46 lofts, faceted, stacked; nothing hangs. A birch without pendulous branchlets is not a birch.
- Wood: stems as lofts read from the drawings, six rod branches that ended in the air; no stool form (a drum), no roots, no dark base, no branching beyond the six boned branches although the crown is 6.9 m deep.
- 1:500: 8 solids, a stem hull and a crown hull.

### 2D

- Constructed views that contradicted each other: the front view drew lobes "towards the viewer" that the elevation did not have, the plan was a ring of ellipses.
- The white stems were drawn white (tone 0.92) only in the 1:50 elevation and near-black (tone 0.15) in every other view and scale, against the asset's own decision text ("the white stems stay unhatched to read bright").
- At 1:100 and 1:200 the upper stem bones had no shape of their own (one `stem-l` shape on the lower bone drew both), so a pose that bends the upper stem did not bend the drawing.
- `plug` pieces at the joints, `followTip` quadrilaterals for the stems in front.

### Thermal

- `stem-bark` 18.78 m² and `branch-bark` 2.67 m² were round-2 values steered by the old model (the decisions say so: 14 raised to 18.78, 4 lowered to 2.67). A hand calculation of the real tree gives about 14 m² and 4 m² (section 3).
- Emitters were discs in the air at the anchors.

### Structure

- Members at 0.62 of the body section along the old outlines, foliage-bone members clipped to the silhouette: they followed the wrong form.

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig.bones[stem-l-1 / stem-c-1 / stem-r-1].y` | 0 / 0 / 0 | 0.11 / 0 / -0.11: the stems leave the stool side by side (0.11 m to either side of the centre), not from one point |
| `rig.bones[].z` | none (planar) | lateral offsets that carry every joint onto the splayed wood: `stem-l-1` -0.07, `stem-c-1` 0.13, `stem-r-1` -0.06 at the stool; `stem-l-2` -0.42, `stem-c-2` 0.72, `stem-r-2` -0.38; `fol-l-top` -0.33, `fol-c-top` 0.55, `fol-r-top` -0.30; the branch bones at the lateral position of their stem at the joint, the six `fol-*-a/b` bones at the drift of their branch (relative to the parent). The left and right stems lean to -z, the centre stem to +z: a tripod in plan, a V in the front view. Angles, lengths, masses, joints, poses and clips unchanged. |
| `anchors` `tp-crown-1..9` | points on the old lobes | 4 cm above the outlet lens on the sleeve of the same bone (`z` set) |
| `thermal.regions[stem-bark].areaM2` | 18.78 | 14 (factor 1.34, the value the asset had before round 2). Hand: three stems, mean d 0.145 / 0.18 / 0.16 m over 5 to 5.5 m and 0.09 to 0.10 m over 4.5 to 4.8 m = 3.6 + 4.6 + 3.9 = 12.1 m², leaders 0.6, stool with roots 1.6: 14.3 m². |
| `thermal.regions[branch-bark].areaM2` | 2.67 | 4 (factor 1.498, the C16 limit, and the value the asset had before round 2). Hand: six boned branches of 2.3 m at mean d 46 mm plus tips of 1.3 m at 19 mm = 2.4 m², seven scaffold laterals of 2.1 to 2.85 m at mean d 23 mm = 1.3 m²: 3.7 m². The 33 thinner laterals (1.6 m² of bark) are twigs of the crown and carry no bark region, see section 6. |
| `representations.*.geometry` | constructed views | true views of each scale's body, shape by shape (section 4) |
| `representations.*.decisions` | old forms | `silhouette`, `retained`, `merged`, `omitted`, `thinElements`, `overlays` rewritten at every scale, `styles` at 1:500 (en, de); `hatching` and the other `styles` are still true and kept |
| `symbols[multi-stem-birch-symbol]` | crown and stems of the old drawing | crown outline of the 1:100 body in all three views, the three stems at their real position and thickness, their sections at breast height in plan, four branch directions; `description` and `dimensionalNote` updated |

Tone root fix inside the drawings: the white stems are tone 0.92 with material `birch-bark` at every scale and in every view; stool and stem bases are dark (0.30 / 0.32, `birch-bark-base`).

## 4. What is modelled

MODEL_TABLE

How the forms are built (all stated by hand, no form is derived from a drawing):

- **Stool and roots.** A loft of 6 elliptic sections from 0.74 by 0.70 m at the ground to 0.26 m at 0.5 m, where it has dissolved into the three stem bases; five surface roots as flattened tapering lofts that run out to 0.7 to 0.8 m and end on the ground.
- **Stems.** One smoothed centre line per stem through 18 stations across its two stem bones and its leader bone, cut at the joints into one loft per bone with 8 cm of overlap, so poses stay closed. The lower 0.85 m is its own loft, 7 mm thicker, in dark bark.
- **Branches.** Six chains on the branch and foliage bones (branch, then the upturned tip); forty unboned branches bound to the stem or leader bone they grow from, in every azimuth, so the crown has depth.
- **Foliage.** Per branch a leafy *sleeve* along its outer part (a flat loft, 0.5 high to 1 wide, that swerves and swells from section to section), one or two *side shoots* with their own sleeve, and four to six *tresses*: lofts that leave the branch sideways at 25 degrees below the horizontal, turn down within their first third and hang plumb, 0.6 to 2.05 m long, 0.26 to 0.36 m wide, the outer ones longest. Round each leader a slim spire and six short tresses. Every second to fourth tress ends in a bare whip (slender, 9 mm).
- **Bark marks** (texture solids, 1:50 and 1:100): lenticel bands, diamond fissures, the scar under every branch.
- **Coarser scales.** 1:100 and 1:200 keep the skeleton and every sleeve and main tress (a tress is 3 mm and 1.5 mm wide on paper there), on fewer sections and stations, without whips, with fewer or no bark marks, fewer or no roots; 1:200 drops the mid tress of the side shoots. 1:500 is authored separately: one weeping bough per branch, a single mass that follows the outer branch and turns over to hang.
- **Thermal.** By reference to the body; the transpiration emitter as flat lenses on the upper side of sleeves, flush (D2).
- **Structure.** Wood under the bark along the same axes (bark 2 mm + 4 % of the radius), joints at the rig joints, support plate of 1 m under the stool, masses at the centres of mass, one foliage mass inside the sleeve of each foliage bone.

CHECK_SECTION

## 5. What I saw in the final renders

LOOK_SECTION

## 6. Still imperfect

IMPERFECT_SECTION

The crown retains its wind joints, stem leaders and off-centre sprays at the three coarser scales. Nine coarse crown masses now follow their actual foliage sleeves and outer hanging sprays. The bark leaders appear in all three 1:500 drawings. The revised branch, mass, silhouette, source and twelve paper-band checks passed. Independent readback of all twelve GLB scenes measured 8,275,732 bytes.

The 1:100 elevation now clips the original fine spray strokes to the actual posed foliage on the matching branch. The native 95th-percentile hard-edge distance improved from 33.75 mm to 7.50 mm against a 30 mm limit. The new projection overlap is 0.99773. The three 1:100 paper samples, source semantics and twelve-scene readback passed.
