# Round 3: components/spiral-stair

State: finished and spliced. `r3-validate.ts components/spiral-stair --catalog-only` returns `VALID ... schema 0, validateAsset 0`. `volumes-check.ts components/spiral-stair --product` passes every rule at every scale except one: `volumes-thermal-coverage` fails at 1:50 and 1:100 for `balustrade-steel` (+114 % and +111 %). This fails on purpose. The guard is now real, the stored area belongs to the old one-baluster-per-tread design, and the correction is more than a factor of 1.5, so C16 does not let me apply it. I have listed it in `area-review.md`.

No earlier round-3 work existed for this asset: no `r3-spiral-stair-*` scripts, no partial report, and the source matched the catalog. The starting check was not green: column area +90 %, 1:200 cross-scale and structure failed.

Scripts (all in the ticket folder):

- `r3-spiral-stair-kit.py`: the real geometry (lapped tread outline, hubs, helix, baluster positions), the masses and centres of mass, and the solid writers.
- `r3-spiral-stair-volumes.py`: every solid of every scale and list, stated part by part. It writes `volumes/components/spiral-stair.json`.
- `r3-spiral-stair-roots.py`: root values and the 2D drawings of all scales and of the symbol. Without a flag it writes a candidate to `🗑️generated/r3-spiral-stair/🔣️.json`. With `--apply` it re-reads the catalog and splices only `provenance`, `summary`, `dimensions`, `rig`, `anchors`, `representations`, `symbols` and `thermal`. It keeps the file's own layout and refuses to write if anything else changes.

## 1. Reference

| item | real value | source |
|---|---|---|
| construction of a kit spiral stair | core column with tread units stacked on it; each unit is a hub sleeve one riser long, welded to a pressed tread pan | manufacturer practice (Mylen, Arke, Fontanot class), provenance `spiral-catalogues` |
| core column | CHS 168.3 x 6.3: 25.2 kg/m, A 32.1 cm², I 1068 cm⁴ | EN 10219-2 |
| hub sleeve | CHS 177.8 x 4.0 (17.1 kg/m) over the 168.3 core | EN 10219-2 |
| tread pan | 5 mm plate, 22.5° sector to R 0.8 m, folded flange 65 mm (pan 70 mm deep), 12 to 14 kg with hub | catalogues; own calculation 13.1 to 13.4 kg |
| nosing lap | open stairs (no risers) need at least 30 mm | DIN 18065 (Baunetz Wissen summary) |
| open riser | clear gap between treads at most 0.12 m where children are expected; 0.187 - 0.07 = 0.117 m | MBO section 38, state codes |
| walking line | R 0.6 m: going 0.236 m, 2 s + g = 0.61 m | DIN 18065 |
| guard height | at least 0.90 m, measured vertically over the nosing | MBO section 38, DIN 18065 |
| guard openings | at most 0.12 m: three 12 mm balusters per tread at R 0.775 m, 101 mm centres, 89 mm clear | MBO section 38 |
| handrail, posts | CHS 42.4 x 2.6, 2.55 kg/m | EN 10219-2 |
| base plate | Ø 400 x 20 mm with four M16 anchors (nut AF 24) | typical detail |
| headroom | 16 risers per turn: 2.99 m - 0.07 m = 2.92 m | own calculation |

Sources: [Baunetz Wissen, Unterschneidung](https://www.baunetzwissen.de/glossar/u/unterschneidung-48163), [Baunetz Wissen, Stufen: Regeln](https://www.baunetzwissen.de/treppen/fachwissen/treppenelemente/stufen-definition-begriffe-und-regeln-3352555)

## 2. Audit findings (before this round)

- **Treads:** sectors met edge to edge with no nosing lap. DIN 18065 asks for at least 30 mm on an open stair.
- **Pan depth:** 60 mm deep, so the open riser was 127 mm, more than 120 mm.
- **Guard openings:** 15 balusters, one per tread, left 292 mm openings. That is no guard under MBO section 38.
- **Handrail height:** 0.93 m over the tread centre, which gives only 0.84 m (rail top 0.86 m) over the nosing. That is below 0.90 m.
- **Rail ends:** the handrail ended in the air, with no newel at the bottom and no end post at the top.
- **Column:** a plain 168.3 tube with an open top. Its open bore counted 1.37 m² of area, so column-steel read +90 %. There were no hub sleeves, although the 2D drew sleeve joints. The plan drew a bore although a real column is capped.
- **Rig masses:**

  | part | old | real |
  |---|---|---|
  | tread | 8 kg | 13.1 to 13.4 kg |
  | balustrade | 25.2 kg | 57.0 kg |
  | column | 78.6 kg | 91.0 kg |
  | total | 223.7 kg | 349.5 kg |

- **heightM:** 3.75 m, matching the old rail; the corrected rail makes it 3.886 m.
- **Rig directions:** the rig is planar, so all tread bones point along +x. Their joints are at the right heights on the column axis, so I kept them (see section 6).
- **1:100 solids:** almost a copy of 1:50, with 12 mm balusters below the 15 mm feature.
- **1:200 and 1:500:** the handrail floated, carried by nothing. It was faceted on 15 to 19 stations. The 1:200 thermal list kept 15 balusters that the body did not have.
- **Structure:** there was no hub joint. The rail and balustrade were unsupported at 1:200 and 1:500.

## 3. Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| `dimensions.heightM` | 3.75 | 3.886 (top of the rail at the end post) |
| `dimensions.massKg` | 223.721 | 349.45 |
| `rig` column mass | 78.56 kg, comX 1.4 | 90.96 kg, comX 1.123; title says the core is capped |
| `rig` tread-k mass | 8 kg, comX = projected radius, comY 0 | 13.09 to 13.43 kg, comX and comY from plate, flange and hub; title "pressed pan on its hub sleeve" |
| `rig` balustrade mass | 25.16 kg, com (0, 2.427) | 57.01 kg, com (-0.004, 2.116); title gains the posts and 43 balusters |
| `anchors` rail-point | x 0.5, y 3.733 | x 0.695, y 3.885, z -0.343 (rail top at the end post) |
| `thermal` treads | areaM2 6.67 | 7.95 (hand calculation, factor 1.19) |
| `thermal` column-steel | 1.65 | 1.84 (hand calculation, factor 1.12); characteristic length 0.178 |
| `thermal` balustrade-steel | 1.1917 | not changed: 2.55 proposed in `area-review.md` (factor 2.14) |
| `summary`, `provenance` | | rewritten. Updated citations for DIN 18065, EN 10219 and the catalogues, added `mbo-38`, new notes on geometry and formulas. |
| `representations` 50/100/200/500 | | all three views redrawn from the kit geometry (E1). Details below the table. |
| `symbols[0]` | | treads, column and rail redrawn the same way; dimensional note updated |

What changed in the drawings:

- **Treads:** the 70 mm bars take their true projected extent of the lapped sector. In plan the lapped outlines are drawn upper tread over lower.
- **Column:** Ø 177.8 mm (the hub sleeves) with the cap, and sleeve joints at the tread tops at 1:50.
- **Balustrade:** 43 balusters, a newel and an end post at every scale. The rail is split into pieces of at most 30° so it can be depth-sorted against the column in elevation and front.
- **1:500:** the saw-tooth outline is recomputed with a 75 mm foot and 75 mm treads.
- **Removed:** the old nosing lines, `column-bore`, and the plan baluster dots (they are hidden under the rail).
- **Decision texts:** all keys rewritten in en and de.

Poses and the `climb` clip did not need changes: tread heights, walking line and azimuths are unchanged. I checked them with `r3-2d --pose walker-on-tread-8` (light-line, forces, moments): weight 3.43 kN, base reaction 4.22 kN, residual 0. I also checked `--clip climb --t 5` (light-solid, thermal).

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 58 | 58 | 73 |
| 1:100 | 23 | 23 | 56 |
| 1:200 | 23 | 23 | 41 |
| 1:500 | 22 | 22 | 26 |

**1:50**
- **Body:**
  - Core column CHS 168.3, modelled filled: its sealed bore is not outer area (see section 6).
  - Base plate Ø 400 x 20 with four holes, four M16 studs with hexagon nuts, and an 8 mm cap with a chamfer.
  - 15 hub sleeves 177.8 x 4.0, each one riser long.
  - 15 pans, each a 7.5 mm plate (the scale's feature; real 5 mm) plus a 5 mm flange to 70 mm depth, open underneath, with the nosing edge lapping 30 mm.
  - 43 balusters with welded feet.
  - Newel and end post with 70 mm flanges.
  - Handrail swept every 3.75° with domed ends.
- **Thermal:** `{of}` references:
  - treads = plates and flanges
  - column-steel = core, sleeves, cap, plate and anchors
  - balustrade-steel = rail, posts, flanges and balusters
- **Structure:**
  - Core as the column member.
  - Each pan (plate and flange) as its tread member, with its hub sleeve as the joint.
  - Rail, posts and balusters as members, with the post flanges as joints.
  - Supports: the base plate with its anchors (fixed), and a roller pin on the cap (top tie).
  - Mass points for the column, all 15 tread units and the balustrade.

**1:100**
- **Body:** core, sleeves and cap merge into one Ø 177.8 column; base plate without anchors; 15 solid 70 mm lapped treads; 43 slender balusters, two posts, rail every 7.5°.
- **Structure:** 168.3 core rod, 15 treads with a 15 mm hub collar as joint, the balustrade members, both supports, and the masses.

**1:200**
- **Body:** column and plate, coarser lapped treads, all 43 balusters as slender rods, posts, and the rail every 7.5° without domes.
- **Structure:** core, treads with 30 mm collars, balustrade members, supports, and masses for the column and balustrade.

**1:500**
- **Body:** column with a 75 mm foot, 15 sectors of 75 mm (no lap; it is below the feature), 43 balusters, posts, and the rail every 11.25°.
- **Structure:** column member, treads, balustrade members, a 75 mm disc and a roller as supports, and the masses.

Final check line: `FAIL: 2 measurement(s) over 2 rule line(s)`. Both are `areaDeviation:balustrade-steel` (1:50 1.141, 1:100 1.106); everything else passes.

| scale | silhouette IoU elevation / front / plan | cross-scale IoU elevation / front / plan |
|---|---|---|
| 1:50 | 0.936 / 0.942 / 0.999 | |
| 1:100 | 0.937 / 0.944 / 0.999 | 0.999 / 0.999 / 1 |
| 1:200 | 0.962 / 0.966 / 0.997 | 1 / 1 / 1 |
| 1:500 | 0.989 / 0.991 / 0.982 | 1 / 1 / 1 |

Other area figures:

- 1:50 deviation: treads 0.9 %, column-steel 0.9 %.
- 1:100 deviation: treads 20.7 % (the pans are solid at this scale), column-steel 1.3 %.
- Total area deviation: 3.4 % at 1:200, 7.9 % at 1:500.

The 1:50 and 1:100 silhouette IoU is lower than before (0.936 against 0.996). The overlay shows full agreement; the drop comes from 43 pen-wide strokes of 12 mm, where a sub-millimetre offset is a large share of a thin stroke. The p95 boundary distance is 11 mm against the 15 mm limit.

## 5. What I saw in the final renders

- **`r3-sheet` 1:50 body, six views, plus three-quarter views at turn 200 / tilt 20 and turn 120 / tilt -35 (from below):**
  - Front and the two sides show the stacked sleeve column on its plate and nuts, 15 pans fanning out, and three balusters per tread under a smooth helical rail.
  - The newel stands on tread 1 and the end post on tread 15, with the rail domes just beyond them.
  - The top view shows the lapped leading edges.
  - From below, the pans are open with their 70 mm flanges and the hub joints read as column segments. Nothing floats and nothing interpenetrates visibly.
- **1:50 structure, six views:** blue pans and balustrade, orange hub joints stacked up the core, the green base support and the head roller.
- **1:50 thermal, cut:** red pans, green column and plate, purple balustrade. Nothing pokes out.
- **1:100 body and structure:** the same stair simplified, with solid treads and orange 15 mm collars.
- **1:200 body:** reads at a glance, with all balusters and a smooth rail.
- **1:500 body:** the rail is smooth on 11.25° sections. A first 1:500 render with 30° sections showed a visibly polygonal rail; I refined it.
- **`volumes-render` 1:50 overlays:** green throughout, with a few one-pixel red baluster slivers.
- **`r3-2d` all scales:** in elevation the rail and balusters pass behind the column and reappear. In plan the lap edges count the 15 steps.

## 6. Still imperfect

- **`balustrade-steel` area:** the stored `areaM2` is still 1.1917 against a real 2.55 m². It waits for the area reviewer (`area-review.md`), and until then 1:50 and 1:100 fail `volumes-thermal-coverage`.
- **Filled core:** the core column is modelled solid. The product `outerArea` has no C31 flood (filed in `tool-requests.md`), so a sealed bore of 1.37 m² would count as outer area. The real tube is hollow; mass and section use the hollow values.
- **Plate thickness:** the 1:50 pan plate is 7.5 mm, the scale's feature, against a real 5 mm. Plate alone below the feature fails `volumes-feature`, and C30 covers only hollow solids.
- **Rig directions:** the rig is planar. Every tread bone points along +x while its tread turns by 22.5° steps, and the solids carry the azimuth. Joint positions are right; a bone along each tread needs a lateral bone direction the schema does not have.
- **Generalisation trade-off:** I kept all 43 balusters at 1:200 and 1:500 (the 1:500 lines read as a band). `volumes-cross-scale` compares stroke-drawn slender parts in 3D but only filled shapes in 2D, so leaving balusters out at the coarse scales failed it. Filed in `tool-requests.md`.
- **Not modelled:** the top landing slab and the tie to it are outside the asset; only the roller support stands for the tie. Chequer-plate texture is also not modelled.
