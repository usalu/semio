# Round 3: `vegetation/japanese-maple` (*Acer palmatum*, three-stemmed garden specimen, about 35 years)

**State (2026-11-20).** The catalog asset is spliced and valid:
- `r3-validate.ts vegetation/japanese-maple --catalog-only`: VALID, schema 0, validateAsset 0.
- `volumes-check.ts vegetation/japanese-maple --product`: **PASS (50.7 s)** at 1:50, 1:100, 1:200 and 1:500.

## 0. Starting point, scripts, resume

### What I found

- There was no earlier round-3 work on this asset: no `r3-japanese-maple-*` scripts and no report.
- The round-2 state was VALID, but the check FAILED in three places:
  - `volumes-cross-scale` at 1:200 and 1:500 (p95 0.71 m against 0.54 m);
  - the 1:500 bark area total.
- Reviewers had graded it C/C/C/C in `qa-visual-2.md` ("Crown is a stack of flat pancakes; no branching visible; branch rods float; not a maple") and in `model-findings.md` row 89.
- The renders confirmed this:
  - every tier was a stack of flat discs with straight, ribbed edges;
  - all branches lay in one plane, so the tree had only east and west limbs;
  - the plan was elongated lobes;
  - two of the old solids were slivers of a few mm (`crown-core-up-3`, `lobe-t1d-315-top-3`).
- I rebuilt everything from the reference.

### Scripts

All scripts are in the ticket folder. Every station, spray, limb and outlet is stated by hand; the only rules are the tier dip and the seating of tips into their spray.

- `r3-japanese-maple-tree.ts`: the tree in world metres per scale:
  - stem stations, collar, roots;
  - main limbs and secondary limbs, twigs;
  - spray rings with their sizes, heights and tilts;
  - top and inner sprays.
- `r3-japanese-maple-kit.ts`: tube and pad conversion to bone-local solids, outlines, and the depth-in-spray test. The conversion code is taken over from the apple-tree kit and owned here.
- `r3-japanese-maple-model.ts`: writes the volumes source and the candidate document, which carries:
  - the rig offsets, anchors and bark areas;
  - the 2D redraw, the drawing decisions and the symbol.
- `r3-japanese-maple-draw.ts`: ruling E1, the 2D drawings as true views.
- `r3-japanese-maple-texts.ts`: all decisions, in English and German.
- `r3-japanese-maple-roots.ts`: writes the named root members into the catalog in the file's base style. It checks that every other member stays equal and in order.
- `r3-japanese-maple-run.sh`: runs the whole chain: model, validate the candidate, write the roots, splice, validate the catalog.
- Probes:
  - `r3-japanese-maple-frames.ts`: bone frames;
  - `r3-japanese-maple-inside.ts`: structure or thermal solids that leave the body;
  - `r3-japanese-maple-cross.ts`: where two scales' silhouettes part.

### Resume

`sh r3-japanese-maple-run.sh`, then `$BUN volumes-check.ts vegetation/japanese-maple --product`.

## 1. Reference

| quantity | real subject | source | asset |
|---|---|---|---|
| species and habit | *Acer palmatum*, garden specimen, often multi-stemmed. Several smooth stems splay from a short root collar in a vase. Branches are near horizontal and carry the foliage in layered, flat sprays (domed above, flat beneath). The crown is a broad dome, wider than high. | botany (Vertrees and Gregory, *Japanese Maples*, 4th ed., 2009); `provenance.notes` | as modelled |
| height and spread at 30 to 40 years | 4 to 6 m high, 5 to 7 m wide | Vertrees and Gregory 2009; nursery practice | 4.43 m high, 5.29 m (x) by 5.19 m (z) (`dimensions`, unchanged) |
| stems | 3 to 5 stems of 10 to 20 cm, smooth grey-brown bark with lenticels, slightly sinuous | botany; rig sections | 3 stems of d 0.17 / 0.15 / 0.14 m (rig sections, kept), tapering to 0.06 to 0.09 m; leaning 6, 22 and 24 degrees and splayed in plan |
| branching | tiers of near-horizontal scaffold limbs that fan out in every direction from the stems, about 0.6 to 0.8 m apart in height; second and third order branches hold the foliage at their ends | botany; `provenance.notes` (tiers at about 1.7, 2.5 and 3.3 m) | tiers centred at 1.7, 2.4 and 3.15 m, top to 4.43 m |
| leaf | palmate, 5 to 7 lobes, 5 to 10 cm | botany | palmate leaf marks of the 1:50 drawing (about 9 cm) |
| root system | shallow and fibrous, modest flare, a few surface roots | botany | collar flare r 0.30 m at the ground, 4 surface roots to 0.4 m |
| wood | green maple about 900 kg/m3 | Wood Handbook 2021 (`wood-handbook-2021`) | masses kept (176 kg) |

`dimensions` agree with the reference and were not changed. The modelled extents are 5.29 x 4.43 x 5.16 to 5.19 m.

## 2. Audit (round-2 state)

### Rig

- The rig was planar: all 20 bones had z = 0.
- The three stems therefore stood in one plane (a fan, not a vase), and the tiers had only an east and a west limb.
- Joint heights and lengths were plausible against the reference (stems 2.4 to 2.7 m, tiers at 1.4 to 3.2 m, leader to 3.95 m), so the in-plane rig is kept.
- The defect is the missing lateral spread. I fixed it at the root (section 3).

### Body, all scales

- 18 pads at 1:50 were visual hulls of the old drawings: flat discs with ribbed edges, stacked.
- Branches were rods: straight, floating at the tips, in one plane.
- There were no roots, and the collar was a drum.
- The plan was four elongated lobes.

### 2D

- The drawings were constructed views that contradicted each other: front and plan drew sector pads that the body did not have.
- The elevation had tier strokes `tier-*` and centre lines `limb-*-line`.
- Two near-zero sliver strokes (`crown-core-up-3-line`, `lobe-t1d-315-top-3-line`) were drawn in all three views.
- Patch fills `crown-fill-1/2` were in the 1:200 plan.

### Thermal

- `stem-bark` 2.3 m² and `limb-bark` 1.55 m² were hand estimates for thinner stems (d 0.12 m) and six bare limbs of 1.2 m.
- They are below the tree the rig sections describe; see section 3.

### Structure

The members were clamped to 78 % of the body half width in the old pads instead of following the wood.

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig.bones[].z` | none (planar) | lateral offsets that carry every joint onto the splayed stems: `t1w` 0.27, `t2w` 0.43, `t1e` -0.2, `t2e` -0.31, `t3w` -0.1019, `t3e` -0.1056, `stem-c-top` and `fol-core-up` -0.1093, `fol-core-low` -0.0675 (m, relative to the parent). The west stem leans to +z, the east stem to -z and the centre stem slightly to -z, so the stems form a V in the front view and a tripod in plan. Angles, lengths, masses, poses and clips are unchanged. |
| `anchors` `tp-sun-1..7`, `tp-shade-1..5` | points on the old pads | 4 cm outside the outlet patch of the spray that carries each one (same bones, `z` now set) |
| `thermal.regions[stem-bark].areaM2` | 2.3 | 3.45 (x 1.5, the C16 limit; the hand value is 3.9, see `area-review.md`) |
| `thermal.regions[limb-bark].areaM2` | 1.55 | 2.32 (x 1.497; the hand value is 2.6, see `area-review.md`) |
| `representations.*.geometry` | constructed views | true views of each scale's body, shape by shape, with the same ids, bones, roles, z order, tones and hatches (details below the table) |
| `representations.*.decisions` | old forms | `silhouette`, `retained`, `merged`, `omitted`, `thinElements` and `overlays` rewritten at every scale (en, de); `hatching` and `styles` still true and kept |
| `symbols[japanese-maple-symbol]` | crown outlines and stems of the old drawing; "six branch directions" from the centre | crown outlines of the 1:100 body (the tiers as separate rings in the elevations), the stems at their real position and thickness, the stem sections at breast height in plan, the six tier limbs as directions; `description` and `dimensionalNote` updated (5.27 x 5.15 x 4.43 m) |

How the drawings were redrawn:
- Front and plan sector shapes (`f-t1-0`, `p-t2-120`, ...) are the union of the sprays of their sector.
- Arcs and leaf marks are placed on sprays that are visible, and only on sprays large enough to carry them.
- Lenticels follow the stem.
- New shapes:
  - `wood-<tier bone>` per limb bone and view: a filled outline of the main and secondary limbs, region `limb-bark`;
  - `wood-stem-c-top` in the elevations;
  - `twig-*` texture strokes in the 1:50 elevation.
- Removed shapes:
  - `tier-*`, `limb-*-line`, `t3?-mass`, `fl-*` and `pl-*` (all replaced by the wood outlines);
  - `crown-fill-*` and the two slivers;
  - the 1:500 plan limb lines, which are hidden under the crown.
- Foliage shapes without a region got `leaf-sun` or `leaf-shade`, except shapes that carry `regions`.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 160: collar with 4 surface roots; 3 sinuous stems and the leader (lofts on 7 to 8 stations, smoothed); 6 main limbs along the rig with bow and wiggle; 12 secondary limbs fanning to every quarter; 32 twigs (`slender` + `texture`) seated in their sprays; 101 leaf sprays (details below) | 140: every bark and leaf body solid by reference; 12 outlet lenses | 54: 23 members (wood under the bark, secondary limbs as members of their bone), 10 joints, round root plate, 11 wood masses on the wood axes, 9 foliage masses |
| 1:100 | 76: one flared collar loft, stems and limbs with fewer sections, secondary limbs on 2 stations, no twigs; 53 broader sprays with the 1:50 rhythm of size, height and tilt | 82, 6 outlets | 54 |
| 1:200 | 45: collar, stems, leader, 6 main limbs on 3 stations; 34 broad sprays | 48, 3 outlets | 42 |
| 1:500 | 31: collar, 3 stems and leader as plain lofts; 26 flat masses on `stem-c` (8 + 8 + 4 + 4 top + 2 inner), with the gaps between the tiers left open at the rim | 39: the 6 main limbs as hidden slender thermal solids; 2 outlets | 36 |

The 101 sprays at 1:50:
- Each is a convex domed pad loft: 7 sections, flattish underside, widest a little below the middle.
- They are arranged as:
  - tier 1: 12 outer, 12 drooping fringe and 8 inner sprays, reaching 2.65 m;
  - tier 2: 10 + 10 + 6, reaching 2.5 m;
  - tier 3: 8 + 8 + 4, reaching 1.75 m;
  - the top: 6 ring and 6 fringe sprays plus a cap to 4.43 m;
  - 10 shade sprays round the stems between the tiers.
- Each tier dips 5 to 7 cm towards one side.
- The limbs follow the dip of their own layer.

Final check (`--product`):

| quantity | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| silhouette IoU (elevation / front / plan) | 0.997 / 0.995 / 0.997 | 0.997 / 0.995 / 0.998 | 0.996 / 0.994 / 0.998 | 0.990 / 0.990 / 0.994 |
| cross-scale IoU (elevation / front / plan) | | 0.953 / 0.959 / 0.988 | 0.985 / 0.981 / 0.977 | 0.987 / 0.994 / 0.999 |
| stem-bark area deviation | 14.1 % | 14.9 % | 15.9 % | 15.7 % |
| limb-bark area deviation | 13.2 % | 16.7 % | | |
| area total (C15) | | | 7.4 % | 3.0 % |

Notes on the check:
- The 1:100 cross-scale p95 in elevation is 0.0675 m against a limit of 0.0787 m.
- At 1:100 the area limit is 30 %.
- Structure stays inside the body at every scale.
- `volumes-copied` and `volumes-monotonic` are ok at every scale.

## 5. What I saw in the final renders

All images are under `🗑️generated/r3-sheet/vegetation/` and `🗑️generated/r3-2d/vegetation/`.

### 1:50 body, six views

- **Front:** the three stems rise from one collar and open into a V. The three tiers step back towards a domed top, and the dark shade sprays show between the tiers. The leader and its twigs show in the gap under the top.
- **Right and left side:** the stems splay in a wide V. Each tier is a band of overlapping domed sprays with flat undersides, slightly tilted, sizes and heights varying. The tiers are narrower towards the top.
- **Back:** as front, mirrored.
- **Top:** a round, scalloped dome of sprays; the tiers step inwards to the cap.
- **Three-quarter:** reads at first sight as a layered garden Acer palmatum.

### 1:50 from below (turn 30, tilt -12)

- The secondary limbs fan out under every tier from the stem joints.
- Pad undersides are flat.
- The collar with its roots is closed; no stem cap pokes out (fixed after an earlier render showed a knob).

### Other scales and lists

- **1:100, 1:200 and 1:500 bodies:** the same tree, generalised step by step.
  - 1:200: broad pads, three tiers.
  - 1:500: eight broad masses per lower tier, the slits between the tiers still open, clearly the same layered maple.
- **1:50 and 1:100 structure:**
  - members along the splayed stems and the curved limbs;
  - the round green root plate;
  - masses on the wood and at the spray centroids.
  - An earlier render showed Catmull-Rom loops at the tier-2 joints and a collar mass reaching below the ground. Both are fixed.
- **1:50 thermal (cut):**
  - stem bark red, limb bark green inside the sun pads (violet), shade sprays yellow round the stems;
  - lens outlets on the tier tops;
  - nothing pokes out apart from the 2 cm proud lenses (D2).
- **1:500 thermal (cut):** hidden limbs inside the tiers, the shade outlet under tier 1.
- **Pose `storm` (1:50) and pose `autumn-snow` (1:100 drawing):** the crown leans with its limbs and no joint opens.
- **2D, all four scales:** clean scalloped outlines per foliage group, the shade hatch on the inner foliage, splayed stems, the leader, the palmate leaf marks at 1:50. 1:500 shows a stepped crown with slits over three thin stems.

## 6. Still imperfect

1. **Foliage abstraction.** Foliage is domed pad lofts, not leaves. Close up, the tiers read as clusters of cushions; the texture of fine palmate leaves is only drawn at 1:50.
2. **Rig is planar with offsets.** The stems lean out of the rig plane only through their solids and the joint offsets. In poses, each bone still turns about the world z axis, which is fine for the small sway angles of the poses (at most 4.3 degrees). A real out-of-plane rig needs the schema.
3. **Branches without bones.** Secondary limbs and twigs carry no bone of their own. They move rigidly with their tier bone in poses and clips.
4. **Bark areas at the cap.** `stem-bark` and `limb-bark` are applied at the C16 limit. The hand values (3.9 and 2.6 m²) are listed for the reviewers in `area-review.md`. The check passes with the capped values.
5. **Drawing details.** The 1:200 drawing keeps the original line weights, so some foliage shapes are heavier. In the front views the end-on tier limbs show as small discs where a gap reveals them; that is true to the view.
6. **Not opened.** Thermal at 1:100 and 1:200, and structure at 1:200 and 1:500, were rendered or checked but not opened. The check and the containment probe cover them.
7. **Debug lines.** The scripts print `[DEBUG]` lines for outlet burial and for tips that cannot be seated. At the final state the only ones are two hidden 1:500 limbs whose tips are pulled into the coarse tier masses.

## November 21 terminal sprays

The installed tree uses 107 individual branch pads on nine animated crown bones. Its opposite palmate leaves have seven lobes and independent petioles. The fine woody terminals keep the proximal and distal segments of their actual bend. The printed outlines follow the crowns in every view. All four scales passed 13 geometry rules, the full document and print checks passed, and twelve independently reopened scenes totalled 7,726,424 bytes. The inspected default view retains a faceted surface to the foliage pads.
