# Round 3 — objects/bicycle-stand (Sheffield hoop)

State (done, 2026-11-20): rig, anchors, masses, summary, 2D of all four scales and three views, the true-extent symbol and the volumes of all four scales rebuilt; catalog written and spliced (`volumes-insert.py`: 1:50, 1:100, 1:200, 1:500 authored). `r3-validate.ts objects/bicycle-stand` VALID (with source and `--catalog-only --volumes`: schema 0, validateAsset 0, checkVolumes 0). `volumes-check.ts objects/bicycle-stand --product`: **PASS** (every rule at every scale).

Script (handcrafted, every number stated): `T/r3-bicycle-stand-model.py` (`build` = candidate under `🗑️generated/r3-bicycle-stand/`, `apply` = re-reads the catalog, writes roots + volumes in one write, writes `T/volumes/objects/bicycle-stand.json`). No earlier round-3 work existed for this asset.

## 1. Reference

| quantity | real value | source |
|---|---|---|
| tube | CHS 48.3 x 3.2 mm, 3.56 kg/m, A 4.53 cm², I 11.6 cm⁴ | EN 10219-2 (asset reference `en-10219-2`) |
| zinc coat | 70 µm mean on 3–6 mm steel ≈ 0.5 kg/m² → +0.076 kg/m (3.64 kg/m total) | EN ISO 1461 |
| outer width × height above ground | 0.75 × 0.80 m (asset); market range 0.75–1.2 m wide, 0.75–0.85 m high above ground, tube Ø 48 mm (Ziegler "Florida" B 800 × H 1000 incl. 250 embedment, Bohmeyer & Schuster 800/1000/1200 × 800, Schake 750 wide) | manufacturer data sheets (web search 2026-11-20), asset |
| bend | cold bent, centre-line radius 180 mm ≈ 3.7 D (pipe-bender range 3–4 D); outer 204 mm, inner 156 mm | asset 1:50/1:100 drawing, mass closure (7.47 kg hoop ⇒ R 0.181 m) |
| legs | straight 0–596 mm, centre lines at x = ±350.85 mm, embedded 0.3 m in concrete (not drawn) | asset provenance |
| top rail | straight 341.7 mm between the bend tangents, centre line 775.85 mm | geometry |
| cross rail | CHS 48.3 at 400 mm, welded between the legs; equal-diameter T joint: saddle reaches the leg axis at the front/back, the leg face in the hoop plane; visible intersection = two straight lines (planar ellipses) | geometry of equal cylinders |
| welds, vent holes | fillet a ≈ 3 mm (≈ 5 mm wide), galvanising vent holes ≈ 10 mm | practice |
| mass above ground | hoop 2.0989 m + rail 0.6534 m = 2.752 m × 3.64 = 10.02 kg | computed |

## 2. Audit (before)

- **Rig (root defect):** one bone `hoop` at the origin, angle 0, length 0.8 m — a horizontal line on the ground from x 0 to 0.8 (outside the stand on the right); `crossbar` from mid rail 0.70 m to the right, ending 0.35 m outside the stand. The structural solve therefore treated the whole U as one straight member lying on the ground: the moment view drew a triangle under the ground line, the forces view a single bar. `stand` had length 0.0483 m (eligible as a member, so the old structure invented "root" stubs).
- **Masses:** hoop 7.47 kg as one point mass, cross rail 2.5 kg (counts 0.7017 m axis to axis instead of the 0.6534 m between the leg faces); total 9.97 kg (zinc ignored).
- **2D:** 1:500 elevation band 160 mm wide (2× the floor, with a 460 × 150 mm cross bar not reaching the inner leg faces at 0.215); 1:500 decisions claimed 0.068 m front/plan while 0.0483 was drawn. 1:200 bends were single quadratics (6 % radius error). 1:50 front drew the cross rail bore and weld collars, which are hidden behind the near leg; 1:50/1:100 plan drew leg rings/dots and lower bend tangents at ±0.32665, which are hidden under the bends (true top view: bar with round ends). "Ground collars" drawn at 4 mm and modelled as 8 mm chamfered rings: no such part on an embedded hoop. Weld lines drawn vertical at 0.31265 (14 mm inside the rail, not where the saddle is).
- **Solids:** 1:50 body good in form but bound to the wrong rig; collars invisible (QA); 1:100/1:200 bends faceted (3 chords); 1:500 body an extruded flat A-plate 160 × 48 mm, thermal/structure a thin tube inside it (QA grade C); structure foot blocks larger than the legs.
- **Symbol (true extent):** bend drawn as a wide cubic (≈ 0.25 m), rail at 0.37–0.418.

## 3. Root fixes (old → new)

- `rig`: `stand` (root, length 0.0483 → 0) + `hoop` + `crossbar` → `stand`, `leg-left` (x −0.35085, 90°, 0.59585), `bend-left` (chord, −45°, 0.254558), `top-rail` (−45°, 0.3417), `bend-right` (−45°, 0.254558), `leg-right` (−45°, 0.59585), `crossbar` (child of `leg-left` at 0.4, −90°, 0.7017). All joints on the real tube centre line; every member carries `section` CHS 48.3 × 3.2 (A 4.5339e-4 m², I 1.15856e-7 m⁴, c 0.02415 m) and a uniform mass (legs 2.17, bends 1.03 with comY 0.0348 = arc centroid offset, top 1.24, rail 2.38 kg).
- `dimensions.massKg` 9.97 → 10.02 (steel + zinc, see table).
- `anchors`: `leg-left` → bone `leg-left` (0, 0); `leg-right` → `leg-right` (0.59585, 0); `lean-point` → `leg-right` (0.04585 = 0.55 m above ground); `lock-point` → `crossbar` (0.35085 = mid rail). Ids unchanged, so poses, clips, contacts, loads stay valid.
- `summary`: "plan shows … two leg dots" → true description (bent CHS, R 180 mm, welded rail, plan = bar with round ends).
- `representations` 50/100/200/500: all geometry redrawn per bone (section 4), all decisions rewritten.
- `symbols[bicycle-stand-symbol-true-extent]`: true bend (outer R 0.20415, inner 0.15585 as arcs), rail 0.37585–0.42415 between the leg faces, front tube 48.3, plan bar 48.3, leg centres at ±0.35085.
- Thermal `areaM2` unchanged (hoop 0.32 measured 0.3188, rail 0.106 measured 0.1088).

## 4. Per scale

2D ids per view: elevation `leg-left`, `bend-left`, `top-rail`, `bend-right`, `leg-right`, `crossbar` (+ `weld-l`, `weld-r` at 1:50/1:100: the visible saddle chevron from the leg face to the leg axis); front `leg-right`, `bend-right` (near side, everything else hidden behind it); plan `top-rail`, `bend-left`, `bend-right` (legs hidden under the bends).

| scale | 2D | body | thermal | structure |
|---|---|---|---|---|
| 1:50 | true 48.3 band, bends as true `A` arcs, rail between leg faces, chevrons | 6: legs/top/rail cylinders (revolve), bends true tori (32-pt section), rail axis-to-axis = true saddle union, leg ground faces `sectionFaces` | 6 `{of}` (one wall layer, B2) | 16: hollow CHS revolves on every bone (bends as two half-ring sweeps), 6 joint nodes Ø 58 mm (4 bend tangents, 2 welds), 2 fixed blocks 63 × 7.5 × 63 |
| 1:100 | same, bends as one cubic per arc | 6, bend sections 24-pt | 6 `{of}` | 16: hollow CHS, half rings 12 segments, nodes Ø 64, blocks 15 mm |
| 1:200 | true band, bends two quadratics per arc, rail axis to axis under the legs, no chevrons | 6, bend sections 16-pt | 6 `{of}` | 10: members `{of}` the body tubes (the tube is the member, D3), 2 weld nodes Ø 80, blocks 30 mm |
| 1:500 | 75.2 mm band (print floor) inward from the true outer edge, outer 0.75 × 0.80 and outer bend R 204 exact; front/plan 63 mm across | 6 round 75.2 mm tubes, bend sections 12-pt | 2: `th-hoop` = true 48.3 tube on the real centre line (one slender loft, ratio 43), `th-crossbar` `{of}` | 10: `{of}` body tubes, 2 weld nodes Ø 120, blocks 105 × 75 × 105 |

Final check line: `$BUN volumes-check.ts objects/bicycle-stand --product` → `PASS (3.4 s)`. Silhouette IoU elevation/front/plan: 1:50 0.997/0.999/0.999, 1:100 0.994/1.000/0.999, 1:200 0.992/1.000/1.000, 1:500 0.996/0.994/1.000, p95 0 everywhere except 1:500 (0 / 0 / 0, max 19 mm). Areas 1:50: hoop-galv 0.3188 (0.32), crossbar-galv 0.1088 (0.106); 1:500 total deviation 10 %. Structure outsideBody ≤ 3.75 mm at 1:50. Monotonic, copied, feature, bounds (1:500 depth +27 mm ≤ 75), slender green.

## 5. What I saw (final renders)

- 1:50 body sheet (`r3-sheet`, six views): front/back a single 48.3 tube with round top; right/left side the U with round bends, tangent seams, saddle joints of the rail at both legs; top a bar with round ends and the bend seams; three-quarter a convincing bent galvanised hoop, legs cut at the ground. Rig magenta lines lie exactly on the tube centre line (bends as chords). Reads as a Sheffield stand at first sight.
- 1:50 thermal: hoop red, rail green, leg cuts hatched as section faces; nothing poking out. 1:50 structure: blue hollow tubes, orange nodes at the four tangents and both welds, green blocks flush under the legs (no longer larger than the legs plus a few mm).
- 1:100 / 1:200 sheets and overlays: smooth bends (no facets), overlays green with hair-thin edges only; structure 1:200 nodes only at the welds.
- 1:500: round 75.2 mm tube U with smooth bends (first attempt with `lateralScale` squashed the bend sweep path and an oval loft showed facets: both rejected, see below), thermal true tube inside, members and blocks clean.
- 2D 1:50 light-line/solid: hoop band with seams at the bend tangents (the line hierarchy of the five members), rail with chevrons; front a bar with a rounded top and the tangent line; plan a bar with round ends and two tangent lines. 1:200 and 1:500 read as the same object, smaller.
- 2D forces (1:200, empty-sun): weight 98.3 N, two reactions 49.1 N, free-body inset is the U. Moments 1:50 lock-pulled: a proper portal-frame moment diagram (legs 89–100 N·m at the clamps, bends 113–133 N·m), instead of the old triangle under the ground.
- Clip `bike-parked` t 5 s structure sheet: unchanged geometry (fixed joints), loads only.

## 6. Still imperfect (stated plainly)

- The solver is a tree: the rail hangs from `leg-left` only, so its self-weight shows 8.19 N·m at the left weld (real fixed-fixed 1.47 N·m) and the lock pull enters through the left leg only. Not fixable in the asset; filed in `T/tool-requests.md` (2026-11-20, closed frames).
- 1:500: the 2D lateral rule (`extent-front-depth`, +15 mm max) keeps front/plan at 63 mm while the 3D tube is round 75.2 mm (12 mm = 0.024 mm on paper). Oval 3D sections were tried: `lateralScale` on a revolve squashes the sweep path (bends shifted, overlay red/blue), and an oval loft at the triangle budget of `volumes-monotonic` showed visible facets; round wins.
- Fillet welds (~5 mm) and the 10 mm galvanising vent holes are below the 1:50 feature and not modelled (said in decisions).
- 1:50 vs 1:100 bodies differ only in bend section resolution: the object is a true-size tube at both scales and nothing real lies between 7.5 and 15 mm except the welds; stated in the decisions.
- The bends are bones on their chords; their member lines therefore cut 29 mm inside the inner tube face at mid bend (idealisation, the member solids follow the real arc).

## Round 5 handcraft (2026-11-21)

Script: `T/r5-bicycle-stand-model.py` (imports the round-3 model, states the round-5 changes by hand; `build` → `🗑️generated/r5-bicycle-stand/🔣️.json`, `apply` writes catalog roots + volumes and the source), render/montage helpers `T/r5-objects-render.sh`, `T/r5-objects-montage.ts`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| tube | CHS 48.3 × 3.2 mm | EN 10219-2 |
| outer width × height above ground | 0.75 × 0.80 m (market 0.75–1.2 × 0.75–0.85 m) | Ziegler, Bohmeyer & Schuster, Schake data sheets |
| centre-line bend radius | 180 mm (3.7 D, cold bending range 3–4 D) | pipe-bender practice |
| cross rail | CHS 48.3 at 0.40 m, welded between the legs | asset / manufacturers |
| embedment | 0.25–0.30 m in concrete (not drawn) | manufacturers |
| mass above ground | 10.02 kg (3.64 kg/m incl. zinc × 2.752 m) | EN 10219-2, EN ISO 1461 |

### Model vs reference (1:50, measured on the solids / rig)

| quantity | model | reference | dev. |
|---|---|---|---|
| tube Ø | 48.3 mm | 48.3 | 0 % |
| outer width (leg outer faces) | 0.7500 m | 0.75 | 0 % |
| height (top of top rail) | 0.8000 m | 0.80 | 0 % |
| bend centre-line radius | 0.180 m | 0.180 | 0 % |
| cross rail axis height | 0.400 m | 0.40 | 0 % |
| leg centre distance | 0.7017 m | 0.7017 | 0 % |
| 1:500 band (elevation) | 75.2 mm, outer 0.75 × 0.80 exact | floor 75 mm | — |

### Grades (sheets opened: body, structure, thermal --cut at 1:50/100/200/500, 2D at all four scales, poses empty-shade, empty-sun, bike-leaning, lock-pulled, clips bike-parked t 4.8 s, sun-and-shade t 5760 s)

| scale | body before → after | thermal | structure | 2D |
|---|---|---|---|---|
| 1:50 | A → A | A → A | A → A | A → A |
| 1:100 | A → A | A → A | A → A | A → A |
| 1:200 | A → A | A → A | A → A | A → A |
| 1:500 | A → A | B → A | A → A | A (INVALID print range end) → A |

Defects seen and fixed:
- 1:500 thermal hoop: the slender true-tube loft had a section only every 22.5° in the bends; the bends showed as four-sided polylines (sagitta 3.5 mm). Now a section every 18° (sagitta 2.2 mm); finer steps would give the 1:500 thermal more triangles than 1:200 (`volumes-monotonic`, measured 1152 > 1080 at 15°).
- Catalog INVALID before: `PRINT range-end-feature` 1:500 at 1:1000 (leg-right, bend-right, top-rail, bend-left 0.192–0.194 mm < 0.2 mm). Cause: the 63 mm front/plan bars (capped at 63.3 mm by the lateral extent rule) with a 0.13 mm body pen. Fix: 1:500 body pen 0.13 → 0.14 mm (decision text updated en/de).
- Poses and clip frames: fixed joints, geometry identical and closed in every pose (only loads change).

Checks: `r3-validate.ts objects/bicycle-stand --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts objects/bicycle-stand --product` PASS; spliced with `volumes-insert.py`.

Remaining (with reason): the 1:500 thermal bends still show five chords at sheet zoom (0.004 mm on paper; the monotonic triangle budget of 1:200 forbids more); the tree solver's rail-hangs-from-one-leg idealisation (filed round 3, `tool-requests.md`).
