# Round 3 — `vegetation/date-palm` (mature female date palm, *Phoenix dactylifera*)

State 2026-11-20: catalog asset rewritten at the root and spliced; `r3-validate.ts vegetation/date-palm --catalog-only` **VALID** (schema 0, validateAsset 0); `volumes-check.ts vegetation/date-palm --product` **PASS (97.1 s)** at every scale. No earlier round-3 work existed for this asset (no `r3-date-palm-*` scripts, no report); started from the round-2 state.

Scripts (ticket folder): `r3-date-palm-model.ts` (root fixes, volumes of all scales, 2D redraw, writes candidate and source), `r3-date-palm-parts.ts` (every part stated by hand in world space, per scale), `r3-date-palm-rig-fix.ts` (crown table, pose and clip offsets, loads, tracks), `r3-date-palm-kit.ts` (frames, lofts, outlines), `r3-date-palm-roots.py` (writes named members into the catalog file), `r3-date-palm-ship.sh` (build → validate candidate → write roots → splice → validate catalog; stops on an invalid candidate), `r3-date-palm-fit.sh` + `r3-date-palm-reach.ts` (frond lengths fitted to `dimensions`), `r3-date-palm-extent.ts`, `r3-date-palm-rig.ts` (probes), `r3-date-palm-baseline.json` (input: the round-2 rig, poses, clips and anchors from `git show HEAD`, so the rig fix is idempotent).

## 1. Reference

| item | real subject | source | asset (new) |
|---|---|---|---|
| habit | single unbranched stem, terminal crown of pinnate fronds, persistent leaf bases (kernaf) in spiral phyllotaxis, fibrous sheath (lif) under the crown, no crown shaft | Zaid & de Wet 2002 (FAO PPP 156, botanical description); Barreveld 1993 (FAO bulletin 101) | as modelled |
| height | 15–25 m at maturity; 10–15 m common in plantations | Zaid & de Wet 2002 | 12.5 m (`dimensions`, unchanged; stem to 8.4 m, crown base 8.6–9.0 m) |
| stem diameter | 30–50 cm without, 50–60 cm with leaf bases; little taper | Zaid & de Wet 2002 | 0.52 m at 1 m, 0.46 m at 7.7 m; leaf bases as blocks 12 × 20 cm, 5.5 cm proud |
| base | root initiation zone slightly swollen, dense mat of adventitious pencil roots (≈1 cm) at soil level | Zaid & de Wet 2002 | collar 0.72 m at ground, 14 roots o3.8 cm (1:50) |
| fronds | 3–6 m long; 10–26 new per year, 60–120+ green; petiole about a quarter of the length with spines (acanthophylls 5–20 cm); leaflets 20–50 cm, 1–3 cm wide, stiff, induplicate (V), 120–240 per frond | Zaid & de Wet 2002; Barreveld 1993 | 16 fronds 3.0–3.6 m in four tiers; spiny petiole 6–19 % of the length; leaflets 14–50 cm at 38–26° to the rachis, V 12–16°; spines 7 and 12 cm |
| crown | 6–10 m wide; lowest fronds drooping, youngest upright (spear leaf) | Zaid & de Wet 2002 | 6.3 × 7.0 m (`dimensions`, unchanged), tiers at 5°/−42°, 27°/−3°, 55°/37°, 77°/67° (proximal/distal chords) + 2 spear leaves |
| fruit bunches | stalk (peduncle) 1–2 m, flattened, arching out from the leaf axils and hanging below the fronds; 50–150 strands 30–60 cm; dates 3–7 cm; bunch 5–20 kg | Zaid & de Wet 2002; Barreveld 1993 | 2 bunches, stalk 1.3 m (2.6 × 5.2 cm), 11 strands 0.45–0.6 m, dates 4 × 2.8 cm in helices, 20 kg (rig) |
| frond mass | fresh frond about 5–7 kg | Barreveld 1993 (dry 2–3 kg, ~50 % moisture) | 6 kg (3.5 + 2.5), total frond mass unchanged 96 kg |

## 2. Audit (round-2 state)

- **Rig**: 8 fronds, all eight proximal bones from one point on the stem axis (crown tip); crown placed only through pose abduction/twist; azimuths −148/−125/−100/−55/−8/38/80/172° (a 92° gap, l3/l4 bunched); real crowns have many more fronds from a spiral of insertions over the crown height. Frond masses 12 kg each (twice the real). Bunch mass centres 0.8 m along the bunch bone, in the air between boot and bunch.
- **Body**: sparse stiff spike fronds (agave, `qa-visual-2.md`), flat blades, no rachis or leaflets, no spines; a dark slab under the crown; stem a plain loft with ring extrusions (counted 2.5× as bark, `model-findings.md`); root flare 1.4 m wide (real collar ≈0.7 m); bunches as flat extrusions; 1:500 crown a folded fan.
- **2D**: frond blades 2 mm plates of constant width drawn with pen overrides of 1.8 mm; front and plan constructed, not projections; stem rings and diamonds; patch shapes (`skirt-*`, `*-plug*`, `*-mass`).
- **Thermal**: `stem-bark` areaM2 13 against a hand calculation of 15–16 m² (stem 0.3–7.7 m o0.49 → 11.4, collar and roots 0.9, boot 2.6, petiole stubs 0.9).

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig.bones` fronds | 8 frond pairs from one point, rest angles fanned in the plane | 16 frond pairs `fr-l1..l9`, `fr-r1..r7` (a = petiole + lower blade, b = upper blade) at golden-angle azimuths (−5° + k·137.5°) in four tiers, origins on the crown at their insertion (u 0.20–0.59 m, 0.14–0.26 m from the axis), rest angles = calm chords, joints ±45° / ±30°, masses 3.5 + 2.5 kg; lengths fitted to `dimensions` (`FIT` in `r3-date-palm-rig-fix.ts`) |
| `rig.bones` bunch-l/-r `mass.comX/comY` | 0.8 / 0 (in air) | centroid of the strands (1.32 / ±0.03 m, inside the bunch below the bone tip) |
| `poses` calm/windy/storm, `clips` (18 keys) | offsets for 8 fronds | offsets for 16 fronds solved exactly (angle, abduction, twist; blade normal up); wind response = the elevation change the round-2 poses declared for windward/leeward fronds, blended by azimuth |
| `poses[].loads` | 8 frond loads 1 / 25 / 100 N | 16 frond loads 0.5 / 12.5 / 50 N (same total drag) on the upper blades |
| `clips.gust.tracks` | 8 frond load tracks | 16 (same keys) |
| `anchors` tp-fronds-1..12 | on the round-2 frond plates | 3 mm above the transpiration outlet on the upper blade of 12 fronds (tiers A, B, two of C) |
| `thermal.regions[stem-bark].areaM2` | 13 | 15.8 (factor 1.22 < 1.5, C16; 1:50 outer area 16.18, hand value above) |
| `summary`, `provenance.notes` | 8 fronds, crown shaft | 16 fronds in four tiers, boot with stubs, measures and sources; references `zaid-de-wet-2002`, `barreveld-1993` added |
| `representations.*.geometry` | constructed views | redrawn per E1 (section 5) |
| `representations.*.decisions` | old counts | `silhouette`, `retained`, `merged`, `omitted`, `thinElements` rewritten (en, de) |

`dimensions` unchanged (they agree with the reference). Poses checked: storm 1:50 sheet (stem bends, crown streams to the lee, closed), gust t 2.5 s 1:100 sheet rendered, 1:200 storm drawings opened (no gaps).

## 4. What is modelled

| scale | body (entries / expanded) | thermal | structure |
|---|---|---|---|
| 1:50 | 617 / 3010: root collar, 14 pencil roots, 3 stem lofts, 3 spirals of leaf-base blocks (210, texture), boot, 3 whorls × 9 square-cut petiole stubs, 2 spear leaves, per frond half a rachis loft and two leaflet vanes (V), leaflets every 4 cm as texture blades whose tips stand free of the vanes, spines on the petioles (texture), 2 bunches: stalk, 11 strands, dates in helices (texture) | 156: every non-texture solid by reference in stem-bark / fronds / fruit, 12 transpiration outlets as thin patches on the rachis | 85: root plate, 3 stem members, crown, 32 rachis members along the arches, 2 bunch stalks, joints at 4 stem joints, 16 frond bases, 16 frond knees, 2 bunch bases, root-ball support o1.2 m, 7 masses |
| 1:100 | 355 / 1084: 8 roots, coarser leaf-base blocks, 2 whorls × 6 stubs, 1 spear, rachis + 2 vanes per frond half, leaflets every 9 cm (texture), 7 strands per bunch | 140 | 85 (same members, 15 mm minimum sections) |
| 1:200 | 43 / 48: collar, stem, boot, 6 stubs, 1 spear, one flat arched blade per frond half (30 mm), stalk + cluster per bunch | 55 | 85 |
| 1:500 | 40: stem (flaring into the ground), boot, 32 blades (80 mm), stalk + cluster per bunch | 52 | 51 (no frond joints) |

Frond geometry: the rachis is a parabolic arch over each bone with tangent continuity at the knee (half the kink on each side), so the tip droops a further half-kink; section tables for rachis width/depth, leaflet length (σ 0.2 → 0.6 → 1: 14 → 50 → 8 cm), leaflet angle 40 → 26° are in `FrondGeom`.

Final check line: `PASS (97.1 s)`; silhouette IoU elevation/front/plan 0.910/0.899/0.989 (1:50), 0.915/0.903/0.986 (1:100), 0.944/0.940/0.974 (1:200), 0.923/0.922/0.946 (1:500); stem-bark outer area 16.18 m² at 1:50 (2.4 % off 15.8); bounds within 2 %; structure inside body at every scale; monotonic, cross-scale, copied, slender, feature ok.

## 5. 2D (ruling E1)

All four scales, all three views redrawn from the 3D parts in the calm pose: stem, collar + roots, boot + stubs, spear leaves and bunches as flat outlines (union of the projected parts, simplified at a third of the feature, mapped into each shape's frame by the product's own placement); fronds (spread bones) as `solid` extruded plates (sweep v) per eighth (1:50), seventh (1:100), sixth (1:200), fifth (1:500) of each frond half, each plate's plan outline the vane edges and its v-span the rachis-to-V-edge band on the chord of the arch; leaflets and spines as texture strokes (1:50, 1:100), leaf bases as small outlines on the side facing the viewer, strands as detail lines and dates as small rings (1:50). Layer order per view by depth. Opened: 1:50 (catalog), 1:100, 1:200 storm; 1:500 rendered, not opened.

## 6. What I saw (final renders)

- **1:50 body, six views**: a slender armoured stem with a dense spiral of leaf bases (lower 0.5 m bare, as weathered palms are), a fibrous boot with three whorls of cut petiole stubs, two bunches of beaded strands on long stalks hanging below the crown, sixteen arching feather fronds: lowest tier drooping to −60° at the tips, upright young fronds and spear in the centre; in plan an even star. Close view (turn 200, tilt 8): leaflet combs on every frond, V of the leaflets visible, spines at the petioles. Reads as a date palm at first sight.
- **1:50 storm**: stem bends to the lee, windward fronds lift, leeward fronds stream down, no gaps at frond knees or bases.
- **1:50 structure**: stem members of the declared sections, rachis members following the arches, root-ball disc, bunch masses inside the bunches. **Thermal**: three regions, nothing outside the body.
- **1:100**: same palm, coarser leaf bases and leaflets. **1:200, 1:500**: lanceolate arched blades, stem, boot, bunch masses — a palm, not a fan or a plate.
- **2D 1:50, 1:100**: clean outlines, frond plates with leaflet combs, leaf-base pattern on the stem, bunches with strands.

## 7. Still imperfect

1. 16 fronds stand for a real crown of 60–120: the crown is clearly a palm but airier than an old date palm; adding more rigged fronds would multiply pose/clip/load data (each frond needs its own spread bones).
2. Frond lengths 3.0–3.6 m are at the low end of the range because the declared crown 6.3 × 7.0 m bounds them; the crown is slightly elliptic for the same reason.
3. The V of the leaflets is kept shallow (12–16°) so that the extruded 2D plates (which cannot be V-shaped) agree with the body; real induplicate V can be steeper.
4. Leaf bases, leaflets, spines and dates are texture: they move rigidly with their bone and take no part in silhouettes or areas.
5. In the 2D the frond plates show a thin transverse line at each segment joint (one plate per segment).
6. Fronds have no member between the two blade bones beyond the rachis core; leaflet drag is lumped in the upper-blade load.
7. Not opened: 1:100 and 1:500 thermal sheets, 1:500 2D (rendered and checked by the validator only).

The coarse frond drawing now uses the measured bent stations of all 32 articulated blade segments. Each strip retains its own thickness, lateral tilt and depth in front, plan and elevation. Five print checks from 1:260 to 1:1000, the full document check, all three silhouette comparisons and twelve exported scenes passed. The GLB occupies 7,482,600 bytes.
