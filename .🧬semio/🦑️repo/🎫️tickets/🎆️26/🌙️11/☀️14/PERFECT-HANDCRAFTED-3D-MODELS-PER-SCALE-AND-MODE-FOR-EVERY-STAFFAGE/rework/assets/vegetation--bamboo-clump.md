# vegetation/bamboo-clump — rework report

## Pass 1 (2026-11-22)

No `rework/13.md` exists; the asset was last built by the round-3 author (`T/r3-vegetation-bamboo-clump.md`, source of 2026-11-21 14:39). This pass judged that state from renders and measurements and repaired what was measurably wrong.

### Reference table (yellow-groove bamboo, *Phyllostachys aureosulcata*, garden clump in a rhizome barrier)

| item | real subject | model (measured) | verdict |
|---|---|---|---|
| habit | running bamboo, culms erect, base slightly geniculate, tops nodding outwards, evergreen | six culms, smooth arcs through four rig joints each, tops nod beyond the last bone | right |
| culm height / diameter | garden 3–5 m (species to 6–8 m), 20–40 mm; height about 150–170 x diameter | 2.43–4.50 m with 16–29 mm (ratio 152–155) | right |
| taper | near-cylindrical lower third, about 80 % at mid height, a few mm at the tip | sections 24.0 / 22.5 / 18.1 / 12.1 mm on culm 1 (rig `section`), tip 3 mm | right |
| wall | about 1/8 of the diameter, diaphragm at each node | 0.13 D in the rig sections | right |
| internodes, nodes | 3–6 cm at the base, 15–20 cm at a third of the height, shorter upwards; sheath scar under a supranodal ridge | 21–27 nodes per culm, double ring at 1:50, single ring at 1:100, none at 1:200 and 1:500 | right |
| branching | from the lower-middle culm upwards, two unequal branches per node, alternating sides, 40–60 degrees, arching | from 40 % of the culm length, primary and secondary branch per node (188), 52–32 degrees, drooping | right (the species often branches lower; kept, a groomed garden clump is bare below) |
| leaves | 2–4(–5) per branchlet, lanceolate 5–15 cm x 1–2 cm, hanging | fans of 2–4, blades 10–14 cm x 1.7 cm, 2938 leaves at 1:50 | right |
| clump footprint | barrier loop some dm proud of nothing, 2 mm HDPE, 5 cm above the soil | 1.6 x 1.0 m loop, 5.5 cm proud | right |
| spread of the culms | culms fan outwards in every direction | lean 3–6 degrees in the long axis (rig angles) and 2.2–5.5 degrees across (bone `z` offsets) | right, but the clump is shallow across (two rows at z about +-0.25 m); see "remains" |
| rig | one chain per culm with joints at nodes, stiff at the base and compliant upwards | 4 bones per culm (a, b, c, fol), limits +-8 / +-14 / +-24 / +-14 degrees, fixed root plate | right topology for a cantilevered culm |
| attachment | every leaf hangs on a branchlet, every branchlet on a branch, every branch on a node | measured with `rework-13-bamboo-clump-attach.py` (surface samples within 6–12 mm): 1:50 0 loose of 3748; **1:100 450 loose of 1004; 1:200 86 loose of 124; 1:500 33 loose of 74** | **wrong at three scales** |

### Defects found

1. **Floating foliage at 1:100, 1:200 and 1:500 (body, and through `of` references thermal).** At 1:100 the branchlets had been left out as "below the scale", so 450 of the 792 leaf sprays hung in the air up to 58 mm from their branch. At 1:200 the 100 tufts began up to 123 mm out along a branch that is not in the body (feature 30 mm), at 1:500 the foliage masses stood up to 115 mm off their culm (feature 75 mm). On the sheets this read as loose lobes and "coins" beside the culms instead of branch tiers.
2. The 1:500 decision text speaks of three masses per culm; the body has 7 to 10 per culm plus the top (56).
3. `r3-validate --volumes` of the state I found is not clean: `volumes-silhouette` 1:50 iou 0.896 (limit 0.9) although the 1:50 source was not touched by me (see "Validation").

Everything else in the table was judged right from the six-view sheets, the 1:50 close-ups and the 2D sheets: the 1:50 body reads as a real *Phyllostachys* clump from all six directions.

### Rebuilt

- `T/rework-13-bamboo-clump-necks.py` (the numbers are stated at its top): every foliage solid that did not reach its support got its stem as one more ring of the same solid, so the mass now starts as a branchlet or branch and swells into the leaves.
  - 1:100: 598 sprays grow from their own branch on a 5 mm stem leaving at about 55 degrees (largest gap closed 58 mm).
  - 1:200: 94 tufts grow from the node of their branch (the base of the 1:100 branch of the same id) on a 15 mm stem (largest gap 123 mm).
  - 1:500: 38 masses grow from their culm on a 37.5 mm stem (largest gap 115 mm).
  - Ids, bones, materials and the thermal references are unchanged; the thermal lists follow through `of`.
- After the edit `rework-13-bamboo-clump-attach.py` reports 0 loose solids at all four scales.
