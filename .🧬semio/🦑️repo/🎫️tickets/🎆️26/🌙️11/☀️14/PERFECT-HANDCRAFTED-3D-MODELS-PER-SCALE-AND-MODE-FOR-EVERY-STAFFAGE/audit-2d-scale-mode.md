# Audit: per-scale and per-mode 2D representations versus the 3D `volume` (staffage archive)

Read-only audit. No source file was modified; no git command was run; `repo dashboard` was not used.
Archive: `🎓️teaching/🏛️architecture/🧍️staffage` (S below). Schema: `S/🧬️schema/🔣️.json` (4663 lines, `$id` semio.staffage/v2). Catalog: `S/🗂️catalog/<category>/<slug>/🔣️.json` (152 assets, 9 categories).
Helper scripts and tool output are in the ticket folder (`audit-2d-*.py`, `🗑️generated/audit-2d/`).

Line numbers refer to `S/🧬️schema/🔣️.json` unless a module path is given. Module paths are relative to `S/🔨️modules/`.

---

## 0. Summary

1. Each asset carries four preset-scale decisions under `representations` (keys `"50"`, `"100"`, `"200"`, `"500"`, schema line 4297). Each is either `AuthoredRepresentation` (its own pens, hatches, three viewpoint shape lists, decisions) or `UnavailableRepresentation` (cause, reason, limiting feature, alternatives). The validator forces every scale to be decided and refuses a coarser scale that is a copy of a finer one.
2. Across the catalog, 1:50 and 1:100 are authored for all 152 assets. 1:200 is omitted for 6 assets and 1:500 for 44. Every omission has the cause `below-printable-feature`; `no-physical-depiction` is never used.
3. The coarser scales are redrawn, not filtered: on the six samples, 0 to 65 percent of a coarser scale's shapes are byte-identical (ignoring `id`) to the finer scale, and the coarser scale also adds new shapes (for example the human at 1:500 is a silhouette set `sil-*` that has no counterpart at any finer scale). No coarser scale is a subset of a finer one in the validator's terms (see section 1.6).
4. The 3D `Volume` is a different mechanism: one solid list per asset, with `maxDenominator` as the only scale control. The 2D views never read `volume`. The 2D schema has no per-scale solid data at all. Catalog-wide, 4903 solids: 2987 have no `maxDenominator` (shown at every scale), 1916 have one (825 at 50, 578 at 100, 513 at 200; none at 500). At 1:500 only 2987 solids (60.9 percent) remain.
5. Thermal and structural modes draw no geometry of their own at a given scale beyond overlays (region fills, halos, arrows, dots/crosses, support markers, member diagrams). All those overlays are sized in paper millimetres and do not vary per authored scale. The only scale-dependent parts are the authored shapes they sit on, the label budget (`labelBudget`), and the section-marker threshold. Thermal layers are never drawn in 2D. Structural members (`kind: "member"`) have no 2D outline.

---

## 1. How the schema expresses per-scale 2D representations

### 1.1 Scale key and decision

| Def | Lines | Content |
|---|---|---|
| `ScaleDenominator` | 1293-1300 | `enum: [500, 200, 100, 50]` |
| `Asset.representations` | 4297-4320 | object, `required: ["50","100","200","500"]`, `additionalProperties: false`; each key `$ref` `Representation` |
| `Representation` | 1655-1665 | `oneOf`: `AuthoredRepresentation` (1438) or `UnavailableRepresentation` (1609) |
| `Asset.presentation.defaultScale` | 4206-4226 `Presentation` | `required: defaultPose, defaultClip, defaultScale` (a `ScaleDenominator`) |

Validator: `rep-undecided` (`🗂️catalog/🟦️.ts` line 396) when a key is missing; `rep-key-mismatch` (line 399) when `denominator` differs from the key.

### 1.2 `AuthoredRepresentation` (lines 1438-1538)

`required` (1442-1449): `status`, `denominator`, `validRange`, `pens`, `hatches`, `geometry`, `decisions`. `additionalProperties: false`.

| Key | Line | Shape |
|---|---|---|
| `status` | 1452 | `const: "authored"` |
| `denominator` | 1455 | `ScaleDenominator` |
| `validRange.minDenominator` / `maxDenominator` | 1458-1475 | numbers, both `exclusiveMinimum: 0` (required together) |
| `pens` | 1476 | `PenSet` (1302-1356) |
| `hatches` | 1479 | array of `Hatch` (1357-1396) |
| `geometry.elevation` / `front` / `plan` | 1485-1516 | arrays of `Shape`, each `minItems: 1` (required together) |
| `sections` | 1517-1533 | optional object, `minProperties: 1`, keys `elevation` / `front` / `plan`, each `ViewSection` (1539) |
| `decisions` | 1534 | `RepresentationDecisions` (1397-1437) |

`PenSet` (1302-1356): `required` `body`, `detail`, `texture`, `glazing`, `ground`. `body` and `ground` are numbers `> 0` and must not be null (validator `pen-required`, `🗂️catalog/🟦️.ts` 352). `detail`, `texture`, `glazing` are `number > 0` or `null`; `null` means the role is not drawn (`shape-role-undrawn`, line 342, when a shape of that role has no `penMm`). Pens are paper millimetres.

`Hatch` (1357-1396): `required` `id`, `title`, `pattern` (enum `lines | cross | dots | stipple`), `angleDeg`, `spacingMm`, `penMm`. Validator `pen-units` (line 352 area): spacing 0.03 to 20 mm, pen 0.03 to 2 mm (constants lines 67-69). `glazing-hatch` (line 335) forbids a hatch on a glazing shape.

`RepresentationDecisions` (1397-1437): `required` all eight of `silhouette`, `retained`, `merged`, `omitted`, `thinElements`, `hatching`, `styles`, `overlays`; each a `Text` (English and German). Validator `decision-missing` (line 444) and `DECISION_KEYS` (line 70).

`ViewSection` (1539-1555): `required` `plane` (number, metres along the view axis) and `label` (`Text`). The description (line 1541) defines the cut for each viewpoint: elevation = lateral z of a vertical plane; front = forward x; plan = height y. Validator `section-plane-outside` (line 383).

### 1.3 `UnavailableRepresentation` (lines 1609-1654)

`required`: `status` (`const: "unavailable"`, 1623), `denominator`, `cause` (enum `below-printable-feature` | `no-physical-depiction`, 1626-1632), `reason`, `limitingFeature`, `limitingSizeM` (`> 0`, 1642), `alternatives` (`minItems: 1`, 1646-1651). The cause "applies to all three viewpoints" (description, line 1626).

`Alternative` (1556-1608): `oneOf` of three objects: `{kind: "scale", denominator}`, `{kind: "symbol", symbol}`, `{kind: "asset", asset}`.

Validator (`🗂️catalog/🟦️.ts`):
- `unavailable-cause-missing` (line 401).
- `limiting-size` (line 409).
- `alternative-missing`, `alternative-unresolved` (416, 419, 426, 436): a scale alternative must be a different authored scale; a symbol must exist and draw in all three viewpoints (`alternative-symbol-view-missing`); an asset alternative must be the same category, and authored at that scale (`alternative-asset-category`, `alternative-asset-chained`, `alternative-asset-undrawable`).
- `unavailable-but-drawable` (line 506): an unavailable scale is refused when the asset's largest real dimension on paper is at least `UNAVAILABLE_MIN_PAPER_MM = 3` (line 40). Paper size = max(height, width, depth) × 1000 / denominator.

### 1.4 `Shape` (lines 810-1292) and the common fields

`ShapeRole` (526-535): `enum: [body, detail, texture, glazing, ground]`.

`Shape` is a `oneOf` of six branches, one per `kind`:

| `kind` | Branch line | Geometry `required` (in addition to common) |
|---|---|---|
| `capsule` | 833 | `x0, y0, x1, y1, r0, r1` (`followTip` optional, plan only) |
| `ellipse` | 921 | `cx, cy, rx, ry` (`rotationDeg` optional) |
| `rect` | 1003 | `x, y, w, h` (`r`, `followTip` optional; front only for followTip) |
| `poly` | 1088 | `points` (array of `[x, y]`), `closed` (`followTip` optional) |
| `path` | 1169 | `d` (SVG-style path in local metres, y up) |
| `member` | 1238 | `from` (`MemberEnd`, 4590), `to` (`MemberEnd`), `section` (`MemberSection`, 4618) |

Common `required` in every branch: `kind`, `id`, `bone`, `role`, `z`, `filled`, `tone` (0 to 1). Optional common fields: `region` (line 855 in the capsule branch), `regions` (858), `material` (862 area), `hatch` (868), `penMm` (871), `solid` (`Solid`, 724-809, `revolved` or `extruded` solids of the shape). Validator checks for `solid` use: `solid-axis-not-ancestor`, `solid-sweep`, `solid-taper-order`, `solid-lateral-scale`, `solid-follow-tip`, `solid-extent`.

`region` binds one thermal region, `regions` a merged form. Validator `region-ambiguous` (line 340): both at once is refused.

`Shape.description` (line 811): "region binds one thermal region, regions a merged form; hatch names a hatch of the representation; penMm overrides the pen of the role."

Symbol representations (`SymbolRepresentation`, 1666-1757) are a separate concept with the same geometry language; they are used as alternatives only.

### 1.5 Viewpoints

`PLANAR_VIEWPOINTS = ["elevation", "front", "plan"]` (`🧬️schema/🟦️.ts` 56). The elevation is the side view, front the front view, plan the top view. Every authored scale must have non-empty `elevation`, `front` and `plan` (validator `elevation-empty` line 447, `front-empty`, `plan-empty`).

`ViewSection` cut lines are drawn by `depiction/✂️section`; markers appear only when `denominator <= SECTION_MARKER_MAX_DENOMINATOR = 100` (`🎨️depiction/✂️section/🟦️.ts` 18; used at `🎨️depiction/🟦️.ts` 231).

### 1.6 Individuality and consistency rules

These are the validator rules that enforce per-scale individuality. All are in `🔨️modules/🗂️catalog/🟦️.ts` unless stated.

| Rule (issue `code`) | Line | What it enforces |
|---|---|---|
| `rep-undecided` | 396 | every preset scale 50, 100, 200, 500 must be authored or unavailable |
| `valid-range` | 441 | own denominator inside `validRange` |
| `valid-range-overlap` | 460 | authored neighbours do not overlap |
| `valid-range-gap` | 467 | authored neighbours leave no gap larger than one scale step |
| `representation-copied` | 473 | **every pair of authored scales is compared** with `compareRepresentations` (`🔨️modules/🖨️representation/🪞️copies/🟦️.ts` 76). A coarser drawing that is `identical` to, a `subset` of (every coarse shape reappears identically in the finer one) or `scaled` (equal up to one common factor: the denominator ratio or the drawn-extent ratio) a finer drawing is an error: "every scale must be redrawn". Shape signatures: viewpoint, bone, role, closedness and flattened rings rounded to 0.1 mm (`🪞️copies/🟦️.ts` 1-20). |
| `region-undrawn` | 455 | each thermal region must be drawn by at least one **elevation** shape at each authored scale, unless the region sets `hidden` |
| `extent-height` / `extent-width` | 385-386 | default-pose drawing extent within max(2 % of the declared value, 15 mm) of `dimensions` |
| `section-plane-outside` | 383 | section plane inside the asset extent |
| `suitability-*` (in `🩺️check`, not in `validateAsset`) | `🗂️catalog/🩺️check/🟦️.ts` | per authored scale, measured stroke, gap, feature and hatch spacing against the print profile `office-laser` (minStrokeMm 0.1, minGapMm 0.15, minFeatureMm 0.2, minHatchSpacingMm 0.4) |
| `range-end-*`, `nominal-*` | 764 (`validateRanges`) | the validRange ends are printable at the profile |
| `no shapes` / `empty scene` per scale × viewpoint × view | `🩺️check` | every authored scale is depicted in all seven views and all three viewpoints |

Consequence: a coarse scale cannot be a filtered copy of a finer one. Filtering would make the coarse set a subset of the fine set, which `representation-copied` rejects. A coarse scale must therefore contain at least one shape that does not occur identically in the finer scale, or be scaled from it by an exact factor (also rejected).

### 1.7 Where the 2D modes come from

- `VIEWS` (`🔨️modules/🔗️contracts/🟦️.ts` 245): `["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"]`.
- `depict()` (`🎨️depiction/🟦️.ts` 180-260): resolves the authored representation for the scale (`resolveDepiction`, 168), takes `sourceOf(state)` (`🎨️depiction/🧩️shapes/🟦️.ts` 38) for pens and hatches, and draws the shapes of the viewpoint. There is no level-of-detail filter (`🧩️shapes/🟦️.ts` header).
- The four graphic views differ only in style (`baseStyle`, `🧩️shapes/🟦️.ts`): line views fill with paper, solid views fill with tone (glazing at 45 % opacity) and add a half-pen outline only when `detailed`, i.e. `denominator <= 100`.

---

## 2. Six sample assets across the four scales

Method: `audit-2d-samples.py`; output `🗑️generated/audit-2d/samples.txt`. Counts are over all three viewpoints. "identical" means byte-identical after removing `id`, compared with the next coarser authored scale. This is stricter than the validator's signature (which rounds to 0.1 mm), so the validator may find more shared shapes than shown here. The samples are one per category, chosen by name (no metric was used): the human `standing-adult`, `labrador-dog`, `english-oak`, `city-car`, `cafe-chair`, and `gas-boiler` (an appliance with 1:500 unavailable).

### 2.1 standing-adult (human, height 1.69 m)

| Scale | Status | validRange | Shapes (elev / front / plan) | Roles | Pens body / detail / texture / glazing / ground | Relation to next coarser scale |
|---|---|---|---|---|---|---|
| 1:50 | authored | 20-75 | 139 (58 / 52 / 29) | body 69, detail 30, texture 40 | 0.25 / 0.13 / 0.10 / – / 0.30 | (finest) |
| 1:100 | authored | 75-150 | 79 (31 / 27 / 21) | body 59, detail 20 | 0.20 / 0.10 / – / – / 0.25 | 35 of 79 identical; 43 ids dropped; 6 new |
| 1:200 | authored | 150-350 | 57 (20 / 17 / 20) | body 56, detail 1 | 0.18 / 0.10 / – / – / 0.20 | 26 of 57 identical; 18 dropped; 0 new |
| 1:500 | authored | 350-1000 | 43 (13 / 14 / 16) | body 43 | 0.13 / – / – / – / 0.18 | 0 of 43 identical; 19 new; ids are `sil-*` silhouette masses |

What changes: eyes, brows, mouth, laces, eyelets, fingers and cuffs disappear by 1:200. Hair, ears and the hoodie merge into head and torso masses at 1:500. At 1:500 the far arm is omitted because it hides behind the near arm in profile. The 1:500 figure is a redrawn outline built from `sil-*` shapes, not a subset.

Decision text (1:500, `omitted`): "The far arm (it hides behind the near arm in profile), neck, sleeve cuffs, hood fold, laces, sole line."

### 2.2 labrador-dog (animal, height 0.72 m, width 1.44 m)

| Scale | Status | validRange | Shapes (elev / front / plan) | Roles | Pens | Relation to next coarser |
|---|---|---|---|---|---|---|
| 1:50 | authored | 20-75 | 107 (46 / 31 / 30) | body 87, detail 16, texture 4 | 0.35 / 0.18 / 0.10 / – / 0.50 | (finest) |
| 1:100 | authored | 75-150 | 94 (37 / 29 / 28) | body 87, detail 7 | 0.25 / 0.13 / – / – / 0.35 | 61 of 94 identical; 11 dropped; 4 new |
| 1:200 | authored | 150-350 | 62 (19 / 23 / 20) | body 62 | 0.18 / 0.10 / – / – / 0.25 | 16 of 62 identical; 33 dropped; 13 new |
| 1:500 | authored | 350-800 | 53 (17 / 19 / 17) | body 53 | 0.20 / – / – / – / 0.18 | 33 of 53 identical; 12 dropped; 20 new |

What changes: toes, claws, whiskers, nostril, tongue, pads and fur strokes go by 1:100 to 1:200. At 1:500 the head (0.23 m, 0.46 mm on paper) is merged into one mass, with the shoulder blade and upper arm combined, and the shank and hock of the hind leg combined. Decision text: "the head is 0.23 m = 0.46 mm long and cannot carry them." 1:500 keeps 33 shapes from 1:200 exactly, so this is a partial reuse with merges, not a filter.

### 2.3 english-oak (vegetation, height 18.2 m)

| Scale | Status | validRange | Shapes (elev / front / plan) | Roles | Pens | Relation to next coarser |
|---|---|---|---|---|---|---|
| 1:50 | authored | 20-75 | 231 (71 / 81 / 79) | body 66, detail 75, texture 90 | 0.25 / 0.18 / 0.13 / 0.10 / 0.35 | (finest) |
| 1:100 | authored | 75-150 | 106 (37 / 34 / 35) | body 47, detail 54, texture 5 | 0.20 / 0.13 / 0.10 / – / 0.30 | 18 of 106 identical; 122 dropped; 22 new |
| 1:200 | authored | 150-350 | 42 (15 / 14 / 13) | body 41, detail 1 | 0.18 / 0.13 / – / – / 0.25 | 5 of 42 identical; 53 dropped; 1 new |
| 1:500 | authored | 350-1000 | 10 (4 / 4 / 2) | body 10 | 0.15 / – / – / – / 0.25 | 1 of 10 identical; 31 dropped; 3 new |

What changes: single leaves, twig tufts, knots and bark fissures are removed by 1:100 (replaced by a few arcs); foliage is merged into lobed masses; at 1:500 the crown is seven sun masses plus one inner shade mass plus the leader. Only the body role remains at 1:500 (pen `detail` is null).

### 2.4 city-car (vehicle, length 3.9 m)

| Scale | Status | validRange | Shapes (elev / front / plan) | Roles | Pens | Relation to next coarser |
|---|---|---|---|---|---|---|
| 1:50 | authored | 20-75 | 103 (62 / 21 / 20) | body 22, detail 66, glazing 11, texture 4 | 0.25 / 0.18 / 0.10 / 0.13 / 0.50 | (finest) |
| 1:100 | authored | 75-150 | 78 (43 / 19 / 16) | body 22, detail 47, glazing 9 | 0.18 / 0.13 / – / 0.10 / 0.35 | 30 of 78 identical; 26 dropped; 2 new |
| 1:200 | authored | 150-350 | 51 (24 / 13 / 14) | body 22, detail 25, glazing 4 | 0.18 / 0.13 / – / 0.13 / 0.25 | 25 of 51 identical; 22 dropped; 0 new |
| 1:500 | authored | 350-1000 | 22 (6 / 8 / 8) | body 19, glazing 3 | 0.15 / – / – / 0.13 / 0.25 | 5 of 22 identical; 25 dropped; 1 new |

What changes: door gaps, handles, mirror, lamps, spokes, calipers, seats, tread and wheel detail go by 1:200. At 1:500 the windscreen, side glass and B-pillar become one band, and rim, disc and caliper merge into the tyre. Exhaust pipe (0.12 mm) and steering wheel (0.34 mm) are dropped at 1:500.

### 2.5 cafe-chair (furniture, height 0.88 m)

| Scale | Status | validRange | Shapes (elev / front / plan) | Roles | Pens | Relation to next coarser |
|---|---|---|---|---|---|---|
| 1:50 | authored | 20-75 | 61 (21 / 21 / 19) | body 33, detail 24, texture 4 | 0.50 / 0.25 / 0.13 / 0.18 / 0.70 | (finest) |
| 1:100 | authored | 75-150 | 30 (8 / 11 / 11) | body 23, detail 7 | 0.35 / 0.18 / 0.13 / 0.13 / 0.50 | 0 of 30 identical; 32 dropped; 11 new (full redraw) |
| 1:200 | authored | 150-350 | 22 (6 / 8 / 8) | body 19, detail 3 | 0.25 / 0.13 / – / 0.13 / 0.35 | 3 of 22 identical; 2 dropped |
| 1:500 | authored | 350-1000 | 17 (4 / 7 / 6) | body 17 | 0.18 / – / – / – / 0.25 | 5 of 17 identical; 3 dropped |

What changes: screws, rivets, ferrules, foot pads, finial and grain go by 1:100. Pad, apron and edge become one band at 1:100 and 1:200 (0.3 to 0.6 mm). Near and far legs are one projected line. The 1:100 drawing is nearly a new drawing rather than a filter of 1:50.

### 2.6 gas-boiler (appliance, height 0.72 m)

| Scale | Status | validRange | Shapes (elev / front / plan) | Roles | Pens | Relation to next coarser |
|---|---|---|---|---|---|---|
| 1:50 | authored | 20-75 | 58 (39 / 10 / 9) | body 36, detail 22 | 0.18 / 0.13 / – / 0.10 / 0.35 | (finest) |
| 1:100 | authored | 75-150 | 24 (13 / 6 / 5) | body 24 | 0.18 / – / – / – / 0.25 | 6 of 24 identical; 29 dropped |
| 1:200 | authored | 150-350 | 11 (4 / 4 / 3) | body 11 | 0.13 / – / – / – / 0.25 | 0 of 11 identical; 2 new |
| 1:500 | **unavailable** | – | – | – | – | cause `below-printable-feature`; limitingFeature "Control knobs of 10 mm size", 0.01 m; alternatives: `scale` 200, `scale` 100 |

Reason at 1:500 (quoted): "At 1:500 the boiler is 0.88 mm by 1.44 mm on paper, well under the 3 mm a recognisable drawing needs, and its smallest drawn part, the control knobs of 10 mm, is 0.02 mm, below the 0.2 mm office-laser minimum feature; draw it at 1:200 or larger."

### 2.7 Pattern across the six

- Coarser scales are redrawn. Shape counts fall by a factor of 2 to 4 per step (english-oak 231, 106, 42, 10), and the byte-identical share stays between 0 and 65 percent. Shapes that survive are reused as exact geometry, and new merged or silhouette shapes are added.
- Roles disappear in a fixed order: `texture` first (no texture at 1:200 for most assets), then `detail` (1 or 0 at 1:500), then `glazing` (absent at 1:500 for the human and the boiler). `body` and `ground` survive every scale they are authored at.
- Pens are set per scale (`PenSet` per `AuthoredRepresentation`); the catalog ranges are in section 3.1. A pen shrinks with the scale: the body pen of the samples is 0.25 to 0.35 mm at 1:50 and 0.18 to 0.20 mm at 1:200 and 1:500.

---

## 3. Catalog-wide tables

Source: `🗑️generated/audit-2d/records.json` (per asset), `tables.txt`, `tables-nonmember.txt`. Counts over 152 assets.

### 3.1 Scale decisions and shape counts per category

| Category | Assets | 1:50 authored/omitted | 1:100 | 1:200 | 1:500 | Mean shapes 1:50 / 1:100 / 1:200 / 1:500 (all shapes) | Mean non-member shapes 1:50 / 1:100 / 1:200 / 1:500 |
|---|---|---|---|---|---|---|---|
| 🌳️vegetation | 16 | 16 / 0 | 16 / 0 | 16 / 0 | 15 / 1 | 186.1 / 127.6 / 58.1 / 7.3 | same |
| 🏗️machinery | 16 | 16 / 0 | 16 / 0 | 15 / 1 | 9 / 7 | 106.6 / 61.2 / 45.1 / 34.9 | 64.5 / 55.6 / 39.1 / 33.6 |
| 🐕️animals | 14 | 14 / 0 | 14 / 0 | 12 / 2 | 7 / 7 | 83.9 / 66.9 / 56.9 / 42.1 | same |
| 👤️humans | 28 | 28 / 0 | 28 / 0 | 27 / 1 | 27 / 1 | 141.2 / 94.2 / 73.7 / 52.3 | same |
| 🔌️appliances | 24 | 24 / 0 | 24 / 0 | 22 / 2 | 3 / 21 | 36.8 / 18.7 / 11.5 / 23.0 (n=3 at 1:500) | same |
| 🚜️vehicles | 9 | 9 / 0 | 9 / 0 | 9 / 0 | 7 / 2 | 88.0 / 64.0 / 40.7 / 23.4 | 87.6 / 63.6 / 40.2 / 23.4 |
| 🛋️furniture | 12 | 12 / 0 | 12 / 0 | 12 / 0 | 12 / 0 | 54.0 / 34.2 / 23.2 / 17.8 | same |
| 🧱️components | 18 | 18 / 0 | 18 / 0 | 18 / 0 | 15 / 3 | 61.3 / 31.3 / 22.2 / 11.3 | same |
| 🪑️objects | 15 | 15 / 0 | 15 / 0 | 15 / 0 | 13 / 2 | 58.1 / 28.5 / 19.3 / 13.5 | 57.3 / 27.7 / 18.8 / 13.5 |
| **All** | **152** | **152 / 0** | **152 / 0** | **146 / 6** | **108 / 44** | **92.8 / 59.4 / 40.2 / 27.1** | |

Omitted-scale patterns: none omitted at 50 or 100. Omitted at 500 only: 38 assets. Omitted at 200 and 500: 6 assets (`expansion-valve`, `hedgehog`, `red-squirrel`, `kneeling-gardener`, `room-thermostat`, `thermostatic-radiator-valve`). Omission causes: 50 cases, all `below-printable-feature` (6 at 1:200, 44 at 1:500); zero `no-physical-depiction`. Alternatives used: 81 `scale`, 9 `symbol`, 0 `asset`.

Default scale (`presentation.defaultScale`): 50 for 73 assets, 100 for 70, 200 for 9.

Authored shapes, all scales together: 31,922 (poly 12012, path 7181, rect 5237, capsule 3403, ellipse 3179, member 910). Roles: body 7489 / 6592 / 5111 / 2878 at 50 / 100 / 200 / 500; detail 4756 / 2157 / 669 / 1; texture 1706 / 148 / 3 / 0; glazing 137 / 104 / 68 / 29; ground 23 / 22 / 15 / 14.

Pens across authored scales (min / mean / max, nulls counted separately):

| Scale | body | detail (null count) | texture (null) | glazing (null) | ground |
|---|---|---|---|---|---|
| 1:50 | 0.18 / 0.29 / 0.50 | 0.13 / 0.18 / 0.25 (3) | 0.10 / 0.12 / 0.13 (34) | 0.10 / 0.14 / 0.18 (69) | 0.18 / 0.43 / 0.70 |
| 1:100 | 0.18 / 0.23 / 0.35 | 0.10 / 0.14 / 0.18 (27) | 0.10 / 0.11 / 0.13 (88) | 0.10 / 0.12 / 0.13 (100) | 0.25 / 0.32 / 0.50 |
| 1:200 | 0.13 / 0.18 / 0.26 | 0.10 / 0.12 / 0.13 (38) | 0.10 (144) | 0.10 / 0.12 / 0.13 (102) | 0.15 / 0.25 / 0.35 |
| 1:500 | 0.13 / 0.15 / 0.20 | all null except one override | all null | 0.10 / 0.13 / 0.13 (93) | 0.13 / 0.21 / 0.25 |

Hatches: 90 at 1:50, 53 at 1:100, none at 1:200 or 1:500. Sections present (`sections` object): at 1:50 in 10 assets (elevation 5, front 5, plan 3).

Authored shape counts are per asset; the validator checks that each count is not zero in any viewpoint (`elevation-empty` etc.).

### 3.2 3D volume solids shown per scale (via `maxDenominator`)

Method: a solid is in the 3D model at denominator d when it has no `maxDenominator`, or when `d <= maxDenominator` (`🧊️volume/🧊️solids/🟦️.ts` line 21, `shownAt`). Source: `volume-shown.txt`.

| Category | Solids | Shown at 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|---|
| 🌳️vegetation | 731 | 731 | 707 | 707 | 707 |
| 🏗️machinery | 574 | 574 | 513 | 403 | 284 |
| 🐕️animals | 418 | 418 | 396 | 361 | 359 |
| 👤️humans | 1410 | 1410 | 1021 | 796 | 423 |
| 🔌️appliances | 352 | 352 | 227 | 189 | 176 |
| 🚜️vehicles | 534 | 534 | 490 | 414 | 414 |
| 🛋️furniture | 224 | 224 | 174 | 165 | 164 |
| 🧱️components | 291 | 291 | 242 | 242 | 242 |
| 🪑️objects | 369 | 369 | 308 | 223 | 218 |
| **All** | **4903** | **4903** | **4078** | **3500** | **2987** |

Note that the 2D views have no equivalent of this table. For the 2D drawings, every scale draws its own authored shape list, and the number of shapes per scale is in table 3.1.

---

## 4. Thermal and structural modes: schema and 2D drawing

### 4.1 Schema

Thermal model (`ThermalModel`, lines 3241-3309): `required` `environment`, `passengers`, `operating`, `regions`, `emitters`, `assumptions`. Optional `referenceEnvironment`, `baseMet`, `body`, `respiration`.

`ThermalRegion` (lines 2617-2894): `required` `id`, `title`, `kind` (17 values: skin, clothing, hair, fur, feather, foliage, bark, stem, surface, engine, exhaust, glazing, tyre, wall, heater, lamp, water), `model` (`ThermalModelKind`, 2541-2553: `body | leaf | passive | operating | conditioned | hydronic | coupled | fixed`), `material`, `areaM2`, `characteristicLengthM`, `geometry` (`cylinder | sphere | vertical-plate | horizontal-plate-up | horizontal-plate-down`), `skyViewFactor`, `layers` (array of `ThermalLayer`), `heatCapacityJM2K`, `source`. Optional: `body` (segment, metabolicShare, skinFraction), `leaf`, `phaseChange`, `hidden` (a `Text` explaining why the region is not visible), `operating` (key, heatW), `conditioned`, `hydronic`, `coupled`, `fixed`, `reference`.

`ThermalLayer` (2554-2616): `required` `id`, `title`, `kind` (13 values: tissue, clothing, fur, feather, hair, air-gap, shell, insulation, glass, masonry, timber, metal, polymer), `material`, `resistanceM2KW`, `source`. Optional `thicknessM`, `vapourResistanceM2PaW`, `heatCapacityJM2K`, `reference`.

`Emitter` (3107-3178): `required` `id`, `kind` (`breath | exhaust | vent | flue | fan | transpiration | plume`), `title`, `anchor`, `directionDeg`, `diameterM`, `source`. `Passenger` (3179) and `OperatingKey` (3203, `required` id, title, levels) complete the thermal model.

Structural model (`StructuralModel`, lines 3310-3366): `required` `gravityMS2` (> 0), `dynamics` (`quasi-static | inertial`), `supports` (array of `Slug`, anchors of kind `support`), `assumptions`, `clinical` (`const: false`).

Supports: `AnchorKind` (444-453): `contact | support | attachment | emitter | load`; `SupportType` (454-462): `contact | pin | roller | fixed`; `Anchor` (463-525).

Loads: `Pose.loads` (2096-2170, array of `AppliedLoad` at 2163). `AppliedLoad` (1869-1981): `required` `id`, `kind` (`carried | imposed | live | wind | snow | occupant | equipment`), `title`, `fxN`, `fyN`, `source`; a point load has `anchor` or `bone`+`x`/`y`; `z`, `fzN`, `momentXNm`, `momentYNm`, `momentNm` (default 0); `distributed` (`bone`, `fromM`, `toM`, `wxNPerM`, `wyNPerM`, `wzNPerM`); `reference`.

Members: a `Shape` of `kind: "member"` (branch at line 1238). `from` and `to` are `MemberEnd` (4590-4617: `x, y, z` required, metres from the bone origin at rest; `bone` optional, must be the shape's own bone or an ancestor, `member-end-not-ancestor`). `section` is `MemberSection` (4618): `{kind: "rect", w, h}` or `{kind: "circle", d}`. A member is a physical bar and must not be in a `paper-mm` symbol (`member-in-paper-symbol`).

Volume-level structure: `solid` hints on 2D shapes (`Solid`, 724-809) and 3D solids (`VolumeSolid`) both carry the same bone-frame geometry.

Catalog counts (from `records.json`): thermal regions 959 in total (passive 376, body 328, operating 182, conditioned 37, leaf 28, hydronic 6, coupled 2). Layers per region: 0 layers in 295 regions, 1 in 469, 2 in 124, 3 in 55, 4 in 16. Emitters 220 (humans 91, animals 34, vegetation 28). Operating keys 116. Hidden regions 11. Supports 177 (appliances 41, furniture 43, components 31, objects 32, vegetation 17, machinery 8, humans 5). Pose loads 905 (vegetation 641). Dynamics: quasi-static 126, inertial 26. Members in the 2D representations: 910 in total (690 at 1:50, 106 at 1:100, 102 at 1:200, 12 at 1:500). At 1:50 they are in machinery (674), objects (12) and vehicles (4); the breakdown at the other scales was not made.

### 4.2 2D drawing of the thermal mode

Module: `🔨️modules/🎨️depiction/🌡️thermal-view/🟦️.ts` (159 lines), `thermalView` line 79.

- The field is solved once in world space (`solveThermalAt`, called from `depiction/🟦️.ts` line 194). The view only maps region values to shapes of the chosen viewpoint.
- Each shape with `region` or `regions` (`regionsOf`) is filled by the colour of its region value (diverging ramp centred on the air temperature for temperatures, on 0 W/m² for fluxes). Shapes with no region are grey (`toneColour`). Glazing shapes are 60 % opaque.
- Open strokes that carry a region are drawn as a halo of the region colour under the ink line (`overlay`, line 122, `HALO_FACTOR = 2.6`, `MIN_HALO_MM = 0.3`).
- A second, colour-independent reading is a hatch: `PATTERN_FRACTION = 0.35` of the range beyond the centre gives a hot, cold or vapour hatch (`patternOf`).
- Merged shapes (`regions` with several ids) are painted with the area-weighted value of their regions (legend text).
- A region with no shape in the viewpoint is named as "Not visible from this viewpoint" in the legend (`unseenNote`).
- Thermal layers are not drawn. They enter only the solver (`resistanceM2KW` etc.). No glyphs are used.
- Mode-specific geometry: the region fills, halos and hatches. No per-scale geometry of its own.

### 4.3 2D drawing of the forces and moments modes

Module: `🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts` (635 lines). Functions `forcesView` (line 481), `momentsView` (line 586), `memberView` (line 101), `drawSupports*` (lines 155-229), `drawArrow` (259), `diagramPolygons` (355), `drawDiagram` (412).

- Members: bones are coloured by the chosen internal quantity (`memberView`). Eligible bones: `section` set, or `lengthM >= 5 %` of the drawn height (`eligibleBones`, line 79). The colour is taken from the shapes of the viewpoint bound to that bone. Member shapes themselves have no 2D outline (`flattenGeometry` returns no rings for `member`, `🔨️modules/🧮️evaluation/🟦️.ts` line 308), so members appear in 2D only through the non-member shapes of their bone.
- Supports: drawn as symbols at reaction points (`drawSupportsSide` / `drawSupportsProjected`), by support type.
- Force arrows: length `arrowLengthMm = min(14, max(7, 0.9 × heightMm))` (lines 238-241), where `heightMm` is the drawn figure height in paper mm (scale-dependent through `k`). Minimum 1.5 mm. Dash patterns per kind: applied (solid), reaction `[1.6, 0.7]`, weight `[0.5, 0.5]`, inertial `[0.15, 0.45]` (lines 49-50). Out-of-plane components as a dot (towards the viewer) or cross (away).
- Moments: a curl (counter-clockwise if positive) about the line of sight, or a double-headed arrow in the view plane; radius `1.6 + 3.2 × sqrt(part / max)` mm (line 246).
- Member diagrams (axial and shear): drawn perpendicular to members, `DIAGRAM_MM = 6` mm (line 27). Free-body inset 60 × 30 mm (lines 31-32). Centre of mass marker.
- Contact pressure quantity: a diverging ramp over the footprint (`pressureBase`).

Per-scale dependence in these views: all glyph and diagram sizes are fixed paper millimetres and do not depend on the authored scale. What depends on the scale is:
- the authored shapes they sit on (table 3.1);
- `k` (mm per metre): arrow length and figure height in paper mm;
- the label budget, `labelBudget(denominator)` (`🎨️depiction/🧱️canvas/🟦️.ts` 105-107): 8 at 50, 6 at 100, 4 at 200, 3 at 500 labels;
- the section markers (only at 100 and finer).

### 4.4 Do any modes have per-scale differences?

- Line and solid views: the shapes differ per scale (authored); the solid views add a half-pen outline only at 100 and finer (`baseStyle`, `detailed`).
- Thermal: differs per scale through the shapes and their `region` bindings only. Each thermal region must be drawn at each authored scale (`region-undrawn`, line 455), so the thermal coverage is per-scale by validation, but the geometry is not.
- Forces and moments: differ per scale only through the authored shapes and the label budget.
- No mode has its own per-scale geometry set. The per-scale individuality of the 2D drawings applies to all modes through the shared shape list.

---

## 5. The current `Volume` / `VolumeSolid` schema and its use

### 5.1 `Volume` (lines 4144-4173)

`description` (line 4145): "...It is authored once for every scale: a solid's maxDenominator drops fine detail from coarse scales. The silhouette of the solids seen along each viewpoint agrees with the authored shapes of the representations, and the bounding box of the solids at rest agrees with `dimensions`."

`required`: `status` (`const: "authored"`), `solids` (`minItems: 1`), `decisions` (array of `Text`, `minItems: 1`). `additionalProperties: false`.

`Asset.volume` (line 4287): the `Asset.required` array includes `"volume"`, so it is required. The `volume` property description says "not yet required; it becomes required once every catalog asset has one". That text is stale: it is required in the schema, and the validator reports `required-missing` when absent (`🗂️catalog/🟦️.ts`, `validateAsset`). Every catalog asset has one.

### 5.2 `VolumeSolid` (lines 3425-4143)

Six `oneOf` branches by `kind`: `box` (3432), `ellipsoid`, `capsule`, `revolve`, `extrude`, `loft`.

Common fields and their definitions (taken from the `box` branch, the others are the same):

| Field | Line | Definition |
|---|---|---|
| `id` | (box branch) | `Slug`, unique among the solids of the volume (`volume-duplicate-id`) |
| `bone` | | bone whose rest frame carries the solid, which the solid follows rigidly unless `weights` blends it (`volume-bone`) |
| `material` | | id of an entry of `materials`; colours the solid and, in the thermal view, binds it (`volume-material`) |
| `role` | | `ShapeRole`: body, detail, texture, glazing, ground (line-hierarchy role in the line views) |
| `tone` | | 0 (black) to 1 (white), for solid views |
| `region` | 3467 | `Slug`: the thermal region this solid's surface belongs to (`volume-region`) |
| `regions` | 3471 | array of `Slug` (`minItems: 1`): several regions for a merged solid |
| `maxDenominator` | 3479 | `ScaleDenominator`: largest preset denominator that still shows the solid; absent means every scale (`volume-max-denominator`) |
| `repeat` | | `VolumeRepeat` (3398) |
| `transform` | | `VolumeTransform` (3376) |
| `weights` | | up to four `{bone, weight}` (`volume-weights-*`) |
| `tags` | | free labels; `glyph` reserved |

Consumption: `maxDenominator` is read only in the 3D model and the axonometric view: `🧊️volume/🧊️solids/🟦️.ts` line 8 and 21 (`shownAt`), `🎨️depiction/🧊️axonometric/🧱️mesh/🟦️.ts` line 1, and the catalog check `🗂️catalog/🧊️volume/🟦️.ts` line 139-140. The three planar viewpoints (elevation, front, plan) never read `volume`; only the axonometric viewpoint (`🎨️depiction/🧊️axonometric`, `🧱️mesh`) does, and that viewpoint is outside the 2D seven-view set of section 1.7.

`region` and `regions` are read by the 3D model as tags (`region-<id>`) (`🧊️volume/🧊️solids/🟦️.ts` line 4) and by the validator `volume-region` (`🗂️catalog/🧊️volume/🟦️.ts` lines 138-140).

### 5.3 Catalog counts (`records.json`, `solids-total` 4903)

| Measure | Count |
|---|---|
| Assets with a `volume` | 152 of 152 (status `authored` in all) |
| Solids total | 4903 (mean 32.3 per asset; min 3, max 98) |
| Solids by kind | loft 1373, capsule 1125, box 873, revolve 628, ellipsoid 536, extrude 368 |
| Solids by role | body 3296, detail 1489, glazing 108, texture 10 |
| Solids with `maxDenominator` | 1916: value 50 for 825, 100 for 578, 200 for 513 (none at 500). Assets with any: 124 of 152 |
| Solids without `maxDenominator` | 2987 |
| Solids with `region` (single) | 4712 |
| Solids with `regions` (array) | 26 |
| Solids with `region` or `regions` | 4738 |
| Solids with neither | 165 |
| Assets with at least one region-bound solid | 152 |
| Mean `decisions` per volume | 2.94 |

Per-category breakdown of `maxDenominator` values: `tools` in `volume-shown.txt`, and the full list of assets with any `maxDenominator` value in `🗑️generated/audit-2d/volume-shown.txt` / `records.json`.

---

## 6. Validator run (`🩺️check`)

Command used (read-only, does not write into the repo): `repo tools exec bun -- <S>/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --no-physics --samples 2`, output `🗑️generated/audit-2d/check-catalog-nophysics.txt`. A single-asset check (`standing-adult`, output `check-standing-adult.txt`) passed with all four scales suitable under `office-laser`.

The full catalog run was started in the background; its result is appended below if it completed before this report was written.

Result of the full run (completed, exit 0): 139 `✓` and 14 `✗` lines in `🗑️generated/audit-2d/check-catalog-nophysics.txt` (the output also carries one manifest line; the 14 failing assets are listed there).

- The 14 failing assets: `child-on-kick-scooter`, `courier-with-hand-truck`, `cyclist`, `parent-with-stroller`, `person-with-white-cane`, `running-child`, `seated-reader`, `worker-carrying-box`, `red-fox`, `miscanthus-grass`, `shower-head`, `charcoal-grill`, `concrete-mixer-truck`, `forklift`.
- All failures are print-suitability problems (`suitability-feature` 27 messages, `suitability-gap` 3) under `office-laser`. No failure is `representation-copied`, `rep-undecided`, `valid-range*`, `region-undrawn`, `unavailable-but-drawable`, `decision-missing`, or a volume rule. So the copy rule is never triggered across the catalog, which agrees with section 2 (no coarse scale is a subset of a finer one).
- Consequence for section 3: the 14 failing assets are still counted in the tables above, which describe the archive as stored, not as passing the print check.

---

## 7. What I did not check

- I did not render any drawing (no SVG or PNG was produced). Statements about what is drawn come from reading `depiction` and `structural-view` and the representation data, not from output images.
- The copy comparison in section 2 is my own exact-match approximation (JSON minus `id`). The validator (`🪞️copies`) compares rounded flattened rings and a factor-scaled variant, so its shared-shape counts may be higher than mine. The validator's verdict is the authority, see section 6.
- I did not check the `scaled` copy path of the validator against any asset; I only read it.
- I did not check `penMm` overrides per shape (counted only where they occur as `null`/absent in pens).
- I did not check the 3D model output, GLB/FBX exports, or the symbols (`SymbolRepresentation`) beyond their schema role as alternatives.
- I did not check the physics results (`solveThermalAt`, `solveStructure`) or the numbers of the thermal and structural solvers; `check` ran with `--no-physics`.
- I did not read the tests folder (`S/🧪️tests`, `S/🧫️fixtures`) or the README beyond the schema and modules. Test coverage of per-scale rules is unknown.
- Per-category thermal and structural counts are from the JSON only, not checked against the README numbers.
- The `member` counts at 1:50 are counts of shapes that have no 2D outline; I did not verify whether any other 2D code path uses member shapes beyond `flattenGeometry`.
