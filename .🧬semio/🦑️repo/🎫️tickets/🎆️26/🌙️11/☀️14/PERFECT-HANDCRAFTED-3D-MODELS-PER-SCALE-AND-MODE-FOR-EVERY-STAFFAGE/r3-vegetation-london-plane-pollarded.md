# Round 3 — vegetation/london-plane-pollarded

State: DONE

Resumed on 2026-11-20 from the round-3 work of 2026-11-19 (scripts `r3-london-plane-pollarded-*`, no report yet). The catalog `volumes` equalled the source except one filler sentence another pass had appended to the 1:500 `decisions`; the source text was kept. No new scale was declared by the bulk passes (still 50, 100, 200, 500).

## Files

- Model (every section, shoot and cluster a number): `r3-london-plane-pollarded-model.ts`
- 2D redraw from the body (ruling E1): `r3-london-plane-pollarded-draw.ts`
- Root fixes: `r3-london-plane-pollarded-roots.json`, written by `r3-london-plane-pollarded-apply.py`
- Ship chain: `r3-london-plane-pollarded-run.sh [--write]` (model, redraw, candidate document, `r3-validate.ts --file`, then catalog)
- Helpers: `-probe.ts` (bone frames), `-groups.ts`, `-slices.ts`, `-cross.ts` (scale overlay), `-draw2d.ts`
- Source: `volumes/vegetation/london-plane-pollarded.json`; catalog asset `🌳️vegetation/london-plane-pollarded/🔣️.json`

## Reference (Platanus x hispanica, street pollard)

| Item | Real subject | Source | Asset |
|---|---|---|---|
| Clear stem of a street pollard | 3 to 4.5 m (clearance over footway and parked vehicles) | BS 3998:2010 (pollarding to a framework above the clearance height); own knowledge of European street pollards | 3.75 m to the head |
| Stem diameter, about 50 years, held small by pollarding | 0.55 to 0.75 m at breast height | Pretzsch et al. 2015 (urban Platanus allometry), reduced for the pollard | 0.63 m at 1.3 m, 0.74 m at the collar |
| Root flare | 1.5 to 2 x stem diameter at ground, 3 to 6 visible buttress roots | own knowledge | 1.2 m flare, five buttresses reaching 0.86 m |
| Pollard head (boll) | swollen 1.3 to 1.6 x stem diameter, burred | BS 3998:2010; own knowledge | 0.95 m against 0.63 m |
| Framework limbs with knuckles | 4 to 7 short limbs of 0.4 to 0.8 m set round the head, each ending in a fist of callus 1.5 to 2 x limb diameter | BS 3998:2010 | five limbs 0.5 to 0.6 m, limb 0.26 to 0.3 m, fist 0.44 m, ring at -10, 60, 125, 190, 270 degrees |
| Regrowth | 1.5 to 2.5 m a year, whippy and upright, 6 to 12 shoots per knuckle, base diameter 0.03 to 0.1 m after two years | own knowledge | 41 shoots, 3.4 to 3.6 m after two summers, leaders 0.11 m at the base, others 0.06 m |
| Crown after two summers | 6 to 7 m across, widest high, flat underside just above the knuckles, lobed outline | Pretzsch et al. 2015 scaled; own knowledge | 6.8 x 6.54 m, widest at 6 to 7 m height |
| Total height | stem + head + regrowth = 8.5 to 9.5 m | sum of the above | 8.90 m |
| Leaf | palmate, 0.12 to 0.25 m across | own knowledge | not modelled singly (4 mm on paper at 1:50), clusters of 0.7 to 1.2 m |
| Bark | flaking plates 0.1 to 0.25 m | own knowledge | plates of 0.16 to 0.22 m at 1:50 |

## Audit against the reference

- Rig: every knuckle, leading shoot and foliage bone had no `z`; all five limbs lay in the elevation plane, so the head was a flat fan instead of a ring. Root defect.
- `dimensions` 8.901 / 6.8 / 6.538 m agree with the reference; unchanged.
- Text: summary and provenance spoke of one summer of regrowth of about 4 m; 4 m in one year is not real, the two bones of each leader are two annual increments of about 2 m.
- 2D: elevation, front and plan were constructed separately (front and plan were not views of the elevation's body; plan masses on a circle while the limbs lay in one plane).
- Solids before round 3: qa-visual-2 grade C ("lollipop, no pollard head or knuckles, twigs float").
- Emitter anchors hung in the air beside the crown.
- `areaM2`: 1:50 measures trunk-bark 11.4 (declared 11.2), pollard-bark 7.0 (7.2), shoot-bark 4.0 (3.67); all within 15 %, nothing changed. Leaf regions are exchange areas (C19), untouched.

## Root fixes (in the catalog asset)

| Root | Old | New |
|---|---|---|
| `rig.bones[].z` k-e2 / k-e1 / k-c / k-w1 / k-w2 | none (0) | -0.05 / 0.26 / -0.3 / 0.25 / -0.05 |
| `rig.bones[].z` s-e2a / s-e1a / s-ca / s-w1a / s-w2a | none | -0.09 / 0.29 / -0.3 / 0.27 / -0.09 |
| `rig.bones[].z` fol-core / fol-top | none | 0.6 / 0.5 |
| `summary`, `provenance.notes` (en, de) | regrowth of one summer, about 4 m | two summers, about 2 m a year, 3.4 to 3.6 m, knuckles in a ring |
| 12 emitter anchors `tp-sun-*`, `tp-shade-*` | points in the air | on the outlet patches of the 1:50 crown (D2) |
| `representations[*].geometry`, all scales and views | 86/47/45, 44/42/41, 19/19/17, 3/3/2 shapes | redrawn as the orthographic views of that scale's body with the old ids, roles, bones and line hierarchy: 134/83/54, 64/52/44, 21/19/19, 5/3/2 |

Poses and clips are angle offsets and needed no change; `gale` and `gust` t = 1.2 s were rendered and stay closed.

## What is modelled

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:50 | 282: concave flare, 5 buttress roots, tapered stem, bark plates as texture lenses (those the elevation draws plus 8 on the far sides), head with 4 burrs, 5 limbs with fist, 3 callus knobs and 2 cut stubs each, 5 leaders in two increments with growth node, 36 further shoots, 164 leaf clusters and tip plumes, 7 shade clusters | 240: references, 12 outlet patches on cluster caps and undersides | 55: wood under the bark, 5 leaning limbs, 10 shoot sections, 25 joints, fixed root plate, 11 masses |
| 1:100 | 76: 3 buttresses, plain head and fists, leaders, 10 further shoots, 41 plumes (one per shoot), 4 shade clusters | 76 | 42 |
| 1:200 | 52: plain flare, head, 5 knuckle stubs, 2 outer shoots, 41 leafy spindles, one shade core | 60 | 22 |
| 1:500 | 25: stem with flare, head and knuckles as one swelling, 20 crown masses, the 2 outer shoots that bridge head and low masses | 30 | 20 |

Changes of this session (2026-11-20):

- 1:100 plumes and 1:200 spindles carried twisted, pagoda-like tips: the kernel orients a section without `rotation` by the bone axis, which degenerates where a shoot runs along its foliage bone. Every section now has an explicit untwisted frame, and the tips are rounded with real sections.
- 1:50 leaf clusters stand 0.22 m off their shoot instead of 0.13 m (the shoots read as stacked pawns before); the 1:100 plumes follow the same centres.
- 1:500 failed `volumes-structure-coverage` (outer shoot members crossed open air, 0.169 m against 0.075 m): the two outer leaders are now body solids and filled shapes in the 1:500 elevation; the decision text that claimed shoots were below 0.15 mm on paper was wrong (0.2 mm) and is corrected.

## Checks (final)

- `volumes-check.ts vegetation/london-plane-pollarded --product`: `PASS (163.8 s)`, every rule at every scale.
- `r3-validate.ts vegetation/london-plane-pollarded`: `VALID ... schema 0, validateAsset 0`; `--catalog-only`: `VALID`.
- Spliced: `volumes spliced: 1:50 authored, 1:100 authored, 1:200 authored, 1:500 authored`.

## What the final renders show

Opened: `🗑️generated/r3-sheet/vegetation/london-plane-pollarded.s{50,100,200,500}.body.png`, `.s50.body.three-quarter.png`, `.s50.body.gale.png`, `.s{50,100}.thermal.cut.png`, `.s{50,500}.structure.png`, `🗑️generated/r3-2d/vegetation/london-plane-pollarded.s{50,100,200,500}.png`, `🗑️generated/r3-london-plane-pollarded/cross.50-100.png`.

- 1:50 body: front, sides and back show a straight plated bole on a buttressed flare, the swollen head, limbs leaving in a ring with the bare shoots visible between head and foliage, and an upright lobed crown, widest high; top view is a full ring of clusters round the top group; nothing floats. Grade A.
- 1:100 body: same tree with smooth plumes, no twist. Grade A; the eight low plumes are slightly crumpled where the path bends sharply.
- 1:200 body: 41 spindles, clean. Grade A-; the crown outline is pointed (artichoke-like) where the 1:50 crown is rounded.
- 1:500 body: stem, swelling, lobed crown with low skirt masses and the two bridging shoots. Grade A.
- Thermal (1:50, 1:100, 1:200, 1:500 opened, cut): stem red, head and limbs green, leaders violet inside the crown, shade core cyan inside the sun shell, outlets flush on the clusters. Grade A.
- Structure (1:50, 1:500 opened): stem, head, five leaning limbs, shoot members on the rig bones, joints at the rig joints, root plate on the ground. Grade A. The 1:500 stem mass sits inside the stem member and is not visible.
- Pose `gale`: stem and crown lean together, no gaps.
- 2D: the three views of every scale are views of one body; heights 8.90 to 8.93 m, depth 6.56 to 6.63 m.

## Still imperfect

- The 1:100 and 1:200 structure sheets were rendered but not opened in this session; they come from the same function as the 1:50 and 1:500 sheets that were opened.
- 1:500 thermal: the five shoot-bark leaders start at the shoot joints, so a 0.5 m gap shows between the head swelling and the two outer leaders in the cut view (the body shoots cover it in the body list).
- 1:100 low plumes: a calmer path (cluster offset 0.1 m) fails `volumes-cross-scale` front by 4 mm (0.1125 against 0.10875), so the offset stays at 0.22 m; cross-scale 1:100 plan sits exactly on its limit (0.10875).
- 1:50 plan draws the five leader sections as bars over the foliage (inherited convention of the old `pl-shoot-*` strokes); in a true top view they are hidden.
- Single leaves and the fruit balls are not modelled at any scale.

The five-headed pollard has a 6.80 m measured crown. Its 171 leaf clusters retain their attachment points. Young shoots use their real tapered centrelines in elevation, and the structural cores sit inside the bark. The 1:50 elevation reached 0.9984 intersection over union and a 3.75 mm 95th-percentile boundary distance. The presentation GLB contains twelve scenes and occupies 4,583,016 bytes.
