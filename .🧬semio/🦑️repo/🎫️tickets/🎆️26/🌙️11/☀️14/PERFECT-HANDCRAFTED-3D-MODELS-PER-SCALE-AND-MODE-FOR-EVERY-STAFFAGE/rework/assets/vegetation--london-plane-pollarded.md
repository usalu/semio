# vegetation/london-plane-pollarded — rework report

## Pass 1 (second round), 2026-11-22

### State found

- No report existed and no batch report `rework/*.md` mentions the asset.
- Catalog `asset.volumes` and `T/volumes/vegetation/london-plane-pollarded.json` were identical (every solid of the four scales, every list, the decisions): nothing was left half-edited.
- Judged on the 1:50 sheet: the model was the "tree as ball cluster" of the brief. 282 body solids, all `polyhedron` dumps; 164 leaf "clusters" were near-spheres of 0.5 to 0.9 m stacked into a cauliflower, the same balls at 1:100, 1:200 and 1:500 (176 solids at 1:500). The rig lay almost flat in the elevation plane: five knuckles `k-w2, k-w1, k-c, k-e1, k-e2` side by side from west to east with lateral offsets of 5 to 30 cm, so from the front the crown had no skeleton, and eight `fol-*` bones of 0.1 m hung in space. The framework limbs were 0.12 m thick stubs that did not read under the balls.

### Reference (Platanus x hispanica, street pollard of about 50 years, re-cut every second winter, second summer)

| Quantity | Real subject | Model found | Model now (1:50) |
|---|---|---|---|
| Height in the second summer | 8 to 9 m (head at 3.7 to 4.5 m plus regrowth of 1.5 to 2 m a year) | 8.9 m | 8.6 m |
| Crown spread | 5.5 to 7 m, an upright mop: widest above its middle, flat-domed top, underside open so the knuckles show | 6.8 x 6.5 m ball cluster down to the head | 5.9 x 5.9 m, widest at 6.5 to 7 m |
| Bole | 0.60 to 0.65 m at 1.3 m, clear to 3.5 to 3.8 m, slight swelling under the head | 0.63 m | 0.64 m, clear to 3.7 m |
| Root collar | 0.9 to 1.0 m, concave flare, 4 to 6 buttress roots of about 1 m | 5 buttresses | 1.0 m, 5 buttress roots of 0.9 to 1.05 m |
| Pollard head | bole swells to 0.7 to 0.8 m where the framework limbs leave it, burrs | 0.62 m | 0.77 m at 3.95 m, three burrs |
| Framework limbs | 4 to 7, all round the compass, 1.0 to 1.5 m long, 0.25 to 0.35 m thick, rising at 30 to 45 degrees and turning up | 5, nearly in one plane, 0.55 to 0.6 m, 0.24 m | 5 in a ring (azimuth -10, 62, 128, 192, 268 degrees) of 1.1 to 1.35 m, 0.29 to 0.4 m thick, at 32 to 42 degrees turning up by 14 degrees, plus one short central limb |
| Knuckles (bolls) | swollen callus fists of 0.35 to 0.5 m at the limb ends with old cut stubs | 0.24 m with knobs | 0.38 to 0.43 m, three stubs each |
| Regrowth | 8 to 15 whips per knuckle, 25 to 50 mm at the base, 3 to 3.6 m after two summers, straight; the outer ones lean out at 40 to 60 degrees above the horizon and turn upright (orthotropic), the inner ones stand | 41 shoots, hidden | 68 shoots: per ring knuckle one leader (3.3 to 3.55 m, 48 mm) and ten whips (2.5 to 3.5 m, 32 to 42 mm, 26 to 78 degrees, turning up by 5 to 32 degrees), central knuckle one leader and six whips |
| Leaves | alternate, palmate with 3 to 5 lobes, 15 to 25 cm (to 30 cm on pollard shoots), petiole 5 to 8 cm, internodes 8 to 12 cm: every whip is a leafy wand of 0.6 to 0.9 m across, leafy from about half a metre above the knuckle to the tip | balls | one lobed plume per whip, 0.68 to 0.88 m across, in six ranks with seven or eight tiers along the whip |
| Sun and shade leaves | outer and upper plumes in the sun, the heart of every broom shaded | shade balls in the middle | shade plume in the heart of each broom (lower half of the leader) |
| Fresh mass above ground | 2 to 3 t (bole of 1.1 m3 green wood at about 950 kg/m3, head, limbs, root flare) | 1724 kg | about 3.0 t |

### Deviations of the model found

1. Foliage as ball clusters at all four scales (see above).
2. Rig topology: knuckles not in a ring; no bone for the framework limb proper; `fol-*` bones without meaning.
3. Framework limbs half as long and two thirds as thick as real ones; knuckles half the real size.
4. Crown reached down over the head: the open underside of a pollard, through which limbs and knuckles show, was missing.
5. Coarser scales were the 1:50 balls again.

### What was rebuilt

See the stages below.

Chisel: `rework-london-plane-pollarded-model.py` (every station, angle, length, radius and relief stated by hand) on the tree kit of author 12 (`rework-12-kit.py`, `rework-12-assemble.py`, `rework-12-draw2d.ts`, none of them edited); round `rework-london-plane-pollarded-round.sh`, 2D looks `rework-london-plane-pollarded-looks.sh`, publish `rework-london-plane-pollarded-publish.py` (catalog file and `T/volumes/vegetation/london-plane-pollarded.json` together, so the two agree).

#### Stage 1: 1:50 body and rig

- Rig, 21 bones (was 26): `root-plate` → `trunk` → `head`; six framework limbs `k-e, k-ne, k-nw, k-w, k-s` (ring) and `k-c` (centre) on the head; on each knuckle the leading shoot as its two annual increments `s-<x>a` → `s-<x>b`. The bones leave the elevation plane (`abductionDeg`), so the skeleton is right from the front and from above. The eight 0.1 m `fol-*` bones are gone: the other whips and the leaf plumes of a broom are rigid to `s-<x>a`, whose joint is at the knuckle, so a broom sways as one about its knuckle; the upper leader plume follows `s-<x>b`. Poses and clips keep their authored angles, carried to the new bones (`like` map: e←e2, ne←e1, nw←w1, w←w2, s←e1, c←c); loads are redistributed by leaf area per bone; joint limits and bone masses, centres and sections are recomputed from the solids.
- Body, 171 solids: bole as one loft through eleven stations with a five-lobed flare, five buttress roots, head as a second loft (scarred bark) with three burrs, six limbs each through six stations ending in the knuckle, three cut stubs per knuckle, 68 whips, 68 leaf plumes (62 sun, 6 shade hearts). Plumes are lofted through 16 to 20 rings so that seven or eight leaf tiers in six ranks show; they start 0.5 m above the knuckle, so the limbs, knuckles and whip bases stay visible from below, as on the real tree.
- Looked at and read: six views with rig; right side close-up; poses `gale`, `summer-breeze`; clip `gust` at 4 s. Nothing opens or pokes through.

#### Stage 2: 1:50 structure and thermal

- Structure, 127 solids: the wood itself as members at 0.55 of the bark radius along the true axes (bole, head, six limbs, all 68 whips, stubs, buttresses), connected inside their parents down to the fixed root plate disc of 1.8 m; joint spheres at the rig joints, mass spheres on the three heaviest segments. Read on the sheet: one connected load path from every whip tip to the support.
- Thermal, 183 solids: every bark and leaf solid is its region by reference (`trunk-bark` the bole and buttresses, `pollard-bark` head, burrs, limbs, knuckles and stubs, `shoot-bark` all whips, `leaf-sun` the outer plumes, `leaf-shade` the hearts), twelve emitter domes on plume tops. `areaM2` from the model: trunk-bark 14.8, pollard-bark 9.2 (measured outer surface 9.29; the sum of the solids is 15.4 because limb bases lie inside the head), shoot-bark 14.2. Read on the sheet and on the 1:100 cut: the same tree, hearts inside.

#### Stage 3: coarser scales

- 1:100 (149 body solids): same wood down to 37 mm (81 solids; whips under 37 mm at the base dropped), one plume per whip, wider and with five tiers.
- 1:200 (79): bole, head with burrs, limbs, buttresses, six leaders (kept at 85 mm so the shoot-bark region is drawn); eight upright sun masses per ring broom (one per stronger whip), four on the central broom, a shade heart per broom.
- 1:500 (51): bole with its flare (buttresses merged into it: only one of the five reached 240 mm and stood alone), head, limbs, leaders at 240 mm inside the foliage; four masses per ring broom, four on the central one, one shade heart over the head. The first attempt (two or three balloons per broom) read as a ball cluster again and was replaced.
- Extents of the four scales agree within 0.6 % (5.65 x 8.63 x 5.59 m); `dimensions` follow the model (height 8.63 m, width 5.65 m, depth 5.59 m, 3029 kg).

#### Stage 4: 2D drawings

- All three views of all four scales are redrawn from the model of the same scale (`rework-12-draw2d.ts`: outline per bone and bark region, one lobed foliage outline per broom and one per leader top, shade hearts hatched, painted in depth order). The whip wood is forced behind the foliage in every view (it showed as loose strokes on the crown in the first drawing). The true-extent symbol is redrawn from the 1:100 model. Read: 1:50 (elevation, front, plan before the last fix; elevation clean, strokes in front and plan), 1:100 after the fix (clean), 1:500.

#### Stage 5: decisions

- `volumes[scale].decisions` (body, thermal, structure) and `representations[scale].decisions` are rewritten in English and German for what is modelled at each scale.

### Validation

- Candidate: `r3-validate.ts --file … --volumes` → `VALID file/london-plane-pollarded (file as it is): schema 0, validateAsset 0, checkVolumes 0` (three times, after each stage; last 2026-11-22 13:3x).
- Published three times; final catalog check: see the last line of this report.

### What remains imperfect

1. All solids are `polyhedron` (the kit lofts hand stations into closed meshes); no `loft`/`revolve` kinds of the kernel are used. Sizes: 2.16 MB at 1:50.
2. Bark: the bole is smooth. The flaking plates of plane bark (the old model had 18 texture lenses) are not modelled; `texture: true` plates on the bole and the mottling are open at 1:50 and 1:100.
3. Knuckles are smooth swollen limb ends with three stubs; the knobbly callus surface and the many small cut faces of a real boll are not sculpted.
4. Foliage is one lobed plume per whip, not leaves: single palmate leaves (0.2 m = 4 mm at 1:50) would be `sheet` solids, about 4000 of them. The plume is right in habit (upright wands, open underside) but a leaf never shows.
5. Rig: only the leader of each broom has bones; the other whips of a broom move rigidly with `s-<x>a`. In `gale` (authored lean of 0.4 to 3.7 degrees) this reads right, a stronger pose would need a bone per whip group.
6. Thermal emitters are flat domes on plume tops (kit convention), visible as small caps on the thermal sheet.
7. 1:500: leaders are 240 mm rods inside the foliage (needed so that `shoot-bark` is drawn); they do not show from outside.
8. Not rendered after the last change: 1:50 poses `midday-heat`, `autumn-leaf-fall` (rendered, not read), clips `breeze-sway` and `diurnal-transpiration` (rendered, not read); 2D of 1:200 after the last fix (rendered, not read); thermal and structure sheets of 1:200 and 1:500 were read only in part (1:500 structure, 1:100 thermal cut). The last change touched only the 1:100 plume relief and the 2D paint order.
9. `leaf-sun` / `leaf-shade` `areaM2` (65.0 / 51.3 m2, `areaBasis: exchange`) are the authored leaf areas of the old model and were not re-derived for the new crown (5.65 m instead of 6.8 m spread: leaf area index now higher).

### Final catalog check (2026-11-22 13:43, after the third publish)

`r3-validate.ts vegetation/london-plane-pollarded --catalog-only --volumes` → `VALID vegetation/london-plane-pollarded (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`

Status of this pass: partial. Open for the next pass: items 2, 3, 4, 8 and 9 above; item 1 is a question for the dev (kit polyhedra against kernel lofts).
