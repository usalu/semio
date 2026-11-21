# Round 3 — `vegetation/miscanthus-grass` (flowering clump of Chinese silver grass, *Miscanthus sinensis*)

State: FINAL_STATE

No earlier round-3 work existed for this asset (no `r3-miscanthus-grass-*` scripts, no report, no `🗑️generated/r3-miscanthus-grass/`); started at step 1 of `round3-brief.md`. Starting state on 2026-11-20: the catalog `volumes` differed from `volumes/vegetation/miscanthus-grass.json` (a bulk pass had written subsets of the round-2 1:50 list into 1:100, 1:200 and 1:500; `r3-validate --catalog-only` INVALID, 49 findings: `volumes-copied`, `volumes-feature`); the 2D drawings of 1:50, 1:100 and 1:200 could not be drawn (`r3-2d.ts`: "no suitable representation": print-gap violations of dumped `*-patch-*` shapes). Nothing of either was worth keeping; every scale is authored anew.

Scripts (ticket folder): `r3-miscanthus-grass-plant.py` (the plant stated by hand per scale: every shoot, leaf, raceme, lobe as a table row), `r3-miscanthus-grass-kit.py` (vectors, rest rig, bone-local lofts, view frames), `r3-miscanthus-grass-model.py` (root fixes, volumes of all scales, 2D of all views, writes the candidate and the source), `r3-miscanthus-grass-texts.json` (bilingual summary, notes, references, decisions), `r3-miscanthus-grass-roots.py` (writes the named root members of the candidate into the catalog file), `r3-miscanthus-grass-ship.sh` (build, validate candidate, write roots, splice, validate catalog; stops on an invalid candidate), `r3-miscanthus-grass-suit.ts` (print-suitability probe), `r3-miscanthus-grass-centre.ts` (the validator's centre line of a slender solid against its authored path).

## 1. Reference

| item | real subject | source | asset (new) |
|---|---|---|---|
| species | the asset is 1.75 m tall with silky plumes and 1 cm culms: that is *Miscanthus sinensis* (German "Chinaschilf", the asset's own title); *M. x giganteus* grows 3 to 4 m with blades to 3 cm and rarely flowers in Central Europe | Flora of China 22 (Chen and Renvoize 2006); Hodkinson and Renvoize 2001 | summary and notes name *M. sinensis*; `dimensions` unchanged |
| habit | dense tussock of many erect to outward-leaning shoots from short rhizomes, leaves arching over into a fountain, flowering culms above the foliage in late summer | Flora of China 22 | 10 rigged shoots + 32 side and companion shoots, vase form |
| culm | 0.8 to 2 m tall, 3 to 10 mm thick, straight between nodes, leafy | Flora of China 22 | 4 culms 1.37 m to the panicle base (0.92 + 0.45 m), 13 mm (with sheaths) at the base, 5 mm under the panicle, straight segments with a slight knee at the rig node |
| leaf blade | 20 to 70 cm long, 6 to 20 mm wide, widest in the lower third, tapering to a fine tip, arching and hanging | Flora of China 22 | 244 blades at 1:50, 0.30 to 0.74 m long, 11 to 17 mm wide, width profile 0.5 / 1.0 (30 %) / 0.8 (68 %) / 0.12 (tip) |
| panicle | 15 to 30 cm, fan-shaped: axis 6 to 16 cm, shorter than the 10 to 40 racemes of 10 to 30 cm; nodding to one side, silky | Flora of China 22 | axis 12 cm, 21 racemes of 21 to 28 cm leaving at 4 to 30 degrees and nodding towards the lean of the culm |
| crown | tussock base about 0.3 to 0.5 m across, old culm stubs and litter | horticultural practice (cut back to 10 cm in spring) | mound 0.41 m across, 6.5 cm high, 7 pairs of 9 mm stubs 7 to 13 cm high |
| size | clump 1.5 to 2 m tall and wide | Flora of China 22 | 1.75 x 2.14 x 1.8 m (`dimensions`, unchanged) |

## 2. Audit (round-2 state)

- **Rig**: every one of the 22 child bones started at the same point (0, 0.12 m) on the axis: a real tussock has its shoots spread over the crown. All bones lay in one plane with no lateral place (`z`), so in front view the four culms and plumes stood in one line and the plan was an X of two fans. Culms reached only 1.60 m with the declared 0.8 m lower segment while `dimensions.heightM` is 1.75 m (the old plume solid was simply 0.45 m long on a 0.3 m bone). Root plate 0.12 m long although the crown mound is about 6 cm high. Section titles said "d = 1 cm" for sections of 12 mm and 8 mm.
- **Body**: 73 sausage-like blade segments (two per blade, kinked at the bone joint), 4 culms, 4 teardrop plumes, a flowerpot-shaped tuft: read as a spider (sheet opened at the start), not as a grass; coarser scales were subsets.
- **2D**: 1:50 to 1:200 unprintable dumps (`bl-l2a-patch-e0-0..15`, pens up to 1.8 mm), front and plan constructed boxes, 1:500 one outline.
- **Thermal**: `culms` 0.603 m2 was estimated for 45 culms of 4 mm; one emitter disc of 0.5 m floating at an anchor.
- **Text**: summary named *Miscanthus x giganteus* at 1.6 m.

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig.bones[*].x/y/z` of the 10 shoot bases | all (0.12, 0, -) | on the crown at 5 cm height: culms on an inner ring (x +-0.02 / +-0.07, z +-0.03 / +-0.06), blade bundles on an outer ring (x +-0.05 / 0.10 / 0.14, z +-0.02 / 0.09 / 0.13) |
| `rig.bones[*].z` of the 12 distal bones (`culm-*b`, `plume-*`, `bl-*b`) | absent | lateral gain of the parent segment (`LATERAL` in the plant script: culms +-0.10 to +-0.24 m, bundles +-0.06 to +-0.30 m): the planar rig is now the true side view of three-dimensional shoots; every joint hinges about a lateral axis through its own lateral place, so all poses and clips keep working unchanged and stay closed |
| `culm-*a.lengthM`, `mass.comX`; `culm-*b.x` | 0.8, 0.4; 0.8 | 0.92, 0.46; 0.92 (culms reach the declared height with a real 26 cm raceme) |
| `root-plate.lengthM`, `mass.comX` | 0.12, 0.06 | 0.065, 0.0325 |
| `culm-*a/b.section.title` | "d = 1 cm" | "d = 1.2 cm" / "d = 8 mm" (values unchanged) |
| `anchors` tp-blades-1..12 | on the old plates | 3 mm above the outlet strip on the leading leaf of each bundle bone (x, y, z) |
| `anchors` base-support footprint | 0.5 x 0.5 | 0.41 x 0.41 (the crown) |
| `thermal.regions[culms].areaM2` | 0.603 | 0.86 (factor 1.43 < 1.5, C16; hand value: mound 0.28, stubs 0.04, 4 culms 0.18, 6 main shoots 0.11, 32 side shoots 0.25; 1:50 outer area 0.864) |
| `summary`, `provenance.notes`, `provenance.references` | *M. x giganteus*, bundles as fans | *M. sinensis*, construction and measures; `flora-of-china-22`, `hodkinson-renvoize-2001` added |
| `representations.*.geometry`, `.decisions` | dumps and constructed views | redrawn per E1 (section 5), texts rewritten (en, de) |

`dimensions`, poses, clips, loads, materials, plumes area (0.4 m2), blades area (exchange) unchanged.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | BODY50: crown mound, 14 stubs, 14 basal leaves; per blade bundle a main shoot (stem on both bones, 6 + 9 leaves at its nodes) and 4 side shoots of 5 leaves on the rising bone; per culm two straight culm segments, 5 + 2 leaves, 2 companion shoots of 5 leaves, panicle axis and 21 racemes | every body part by reference (blades / culms / plumes) + 12 outlet strips on leading leaves | STRUCT50: rhizome disc, 8 culm rods (12 / 8 mm), 12 shoot-axis and midrib rods, 4 panicle axes, 22 joints, support disc, crown mass, 4 panicle masses |
| 1:100 | BODY100: 7 stubs, 9 basal leaves, 3 + 6 leaves per main shoot, 3 side shoots of 3 leaves, culms with 3 + 1 leaves and 1 companion, each panicle 7 flat silky lobes | by reference + 12 strips | STRUCT100, simplified rods, larger joints |
| 1:200 | BODY200: mound, 8 basal ribbons, per bundle a sheaf, 4 ribbons on the arching bone and 3 side ribbons, per culm 2 segments, 2 ribbons, panicle of 4 flat lobes | by reference + 12 strips | STRUCT200, no joints |
| 1:500 | BODY500: mound, per bundle a rising and an arching tongue and 3 side tongues, per culm 2 line segments, 3 upright tongues, one panicle plume (tongues are `envelope`) | by reference + 12 strips + 6 shoot axes (thermal only, C2) | STRUCT500 |

Construction rules: every leaf is one whole solid on one bone and is fixed where its node lies on the bone axis, so no pose opens a gap; a main shoot runs through the joint with a common tangent on both sides; blades are lens sections (ellipse, 1.6 mm at the midrib at 1:50) whose width axis stays horizontal; paths are shortened where they would leave the envelope of `dimensions` or touch the ground, with staggered lengths so that no "trimmed hedge" edge appears; the outermost leaves end on the envelope. Stems that bow less than six diameters are made straight because the validator draws such slender parts along their principal axis (measured with `r3-miscanthus-grass-centre.ts`: culm 10.7 mm, 1:200 sheaf 28.7 mm off the authored curve before the fix).

Final check line: CHECKLINE

## 5. 2D (ruling E1)

All four scales and all three views are the orthographic views of that scale's body in the rest frame of each bone (elevation: bone-local u, v; front: lateral, height; plan: ahead, lateral). Slender parts (blades, stems, culms, racemes, ribbons) are one open stroke each along the centre line with the pen of their true width (1:50 blades 0.22 to 0.34 mm, culms 0.26 mm, racemes 0.17 mm; 1:100 blades 0.16 to 0.17 mm; 1:200 ribbons 0.12 mm; minimum 0.1 mm); masses (mound, 1:100 and 1:200 panicle lobes, 1:500 tongues and plumes) are filled outlines of the two flanks of the projected loft. Every shape carries bone, region and material; layer order by depth of the view. All four scales resolve as printable (`r3-2d.ts` draws them; before: three of four failed).

## 6. What I saw

SEEN

## 7. Requests filed

- `tool-requests.md` (r3-, 2026-11-20): `volumes-cross-scale` compares slender solids as strokes on the 3D side (C24) but takes the 2D pair's IoU from filled closed shapes only, so a fountain drawn stroke by stroke gets the full 0.85 / 2-feature limit although its drawings differ leaf by leaf (1:100 against 1:50 before the panicle lobes: IoU 0.82 / 0.82 / 0.75, p95 94 / 64 / 79 mm against 30 mm, 2D pair 0.935 from the mound alone).

## 8. Still imperfect

IMPERFECT

The median-scale plant uses 136 individually curved blade meshes on their original tillers. Eleven twisting blades retain the finer stations needed to keep their folded faces disjoint. Each of 30 side tillers carries its longest arching outside blade into the coarse canopy. The paper strokes follow the posed midribs. The two former floating structural members now follow their stems and terminal blades. Both affected scales passed all six measured bounds, feature, simplification, cross-scale, thermal and structural rules. All six affected silhouettes and twelve nominal/endpoint paper samples passed. Independent readback measured 4,795,516 bytes for twelve scenes.
