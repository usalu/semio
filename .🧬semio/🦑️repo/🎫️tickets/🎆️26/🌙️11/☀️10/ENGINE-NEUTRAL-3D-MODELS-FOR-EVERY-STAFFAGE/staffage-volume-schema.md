# Staffage volume schema: report

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder. Scope: schema, twin, contracts, validator, README, sample-figure volume, vectors and tests. No catalog asset was touched, no evaluation or export module was built.

## Coordinator action

`Asset.volume` is **optional** (`S/🧬️schema/🔣️.json` `Asset.properties.volume`, twin `readonly volume?: Volume`). At the end of the catalog wave: add `"volume"` to `Asset.required` in the schema, drop the `?` in `S/🧬️schema/🟦️.ts` (`Asset`), add `missing(asset.volume, "volume")` next to `missing(... "thermal.passengers")` in `validateAsset`, and give the other fixture assets (`🧫️fixtures/🧱️synthetic-figure`, …) a volume.

## Files

Schema and twin
- `S/🧬️schema/🔣️.json`: new `$defs` `Vector2`, `Vector3`, `Quaternion`, `VolumeTransform`, `VolumeRepeat`, `VolumeSolid` (six kinds, `oneOf`), `Volume` (83 `$defs` now); `ExportFormat` + `glb`, `fbx`, `model`; `Viewpoint` + `axonometric`; `Asset.volume`. Script: `T/volume-schema-edit.py` (idempotent).
- `S/🧬️schema/🟦️.ts`: `VIEWPOINTS` (4), `PLANAR_VIEWPOINTS`, `PlanarViewpoint`, `EXPORT_FORMATS` (11), `Vector2/3`, `Quaternion`, `VOLUME_SOLID_KINDS`, `VOLUME_PLANES`, `VOLUME_REPEAT_MAX` (256), `VOLUME_WEIGHTS_MAX` (4), `VolumeTransform`, `VolumeRepeat`, `VolumeSolidBase`, `VolumeSolid`, `Volume`, `Asset.volume?`.
- `S/🔨️modules/🔗️contracts/🟦️.ts`: `VolumeUnavailableError`, `VolumeFeature`, `isPlanarViewpoint`, `requirePlanarViewpoint`; `EvaluatedState.shapes` keyed by `PlanarViewpoint`.

Validator
- `S/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts` (new): `volumeIssues`, `profileVerdict`, `holeInside`, `holesOverlap`, `segmentsMeet`, `ringArea`, `strictlyInside`. Called from `validateAsset` in `S/🔨️modules/🗂️catalog/🟦️.ts` (`out.push(...volumeIssues(asset))`); exported as `volumeIssues` from the package barrel `S/📦️packages/🟦️typescript/🟦️.ts`.
- Rule ids: `volume-empty`, `volume-duplicate-id`, `volume-bone`, `volume-material`, `volume-region`, `volume-weights-bone`, `volume-weights-count`, `volume-weights-sum`, `volume-size`, `volume-non-finite`, `volume-extrude-range`, `volume-profile-degenerate`, `volume-profile-self-intersecting`, `volume-hole-outside`, `volume-holes-overlap`, `volume-loft-sections`, `volume-repeat`, `volume-transform`, `volume-max-denominator`.

Fixture, vectors, tests
- `S/🧫️fixtures/👤️sample-figure/🔣️.json`: hand-authored `volume` (17 solids, all six kinds, weights, repeat, `maxDenominator`); script `T/sample-volume-author.py`.
- `S/🧫️fixtures/🧊️volume-schema/🔣️.json` (new, language-agnostic; generator `T/volume-vectors-author.py`): 9 valid patches, 49 invalid patches (one or more per rule, `schema: accepts|rejects`, exactly-one-rule check), 14 polygons, 5 open polylines.
- `S/🧪️tests/🧊️volume-schema/🟦️.ts`, `🥒️.feature`, `🐍️.py` (shapely oracle); oracles: ajv, shapely (via `uv`), `robust-predicates` (exact segment intersection, 4000 seeded random segment pairs), hand-computed bounding box from `restFrames`.
- `S/🧫️fixtures/🧬️schema-conformance/🔣️.json` (defs list + 7 new names), `S/🧪️tests/🧬️schema-conformance/🥒️.feature` (v1 → v2).
- `S/README.md`: `v1` → `v2` in the layout table; new section `## Volume`.

Ripple of the new enum members (all fixed so the packages compile as before)
- `Viewpoint` stays the four-valued depiction option. Everything that indexes authored geometry or evaluated shapes is now `PlanarViewpoint` / `PLANAR_VIEWPOINTS` (mechanical rename `VIEWPOINTS` → `PLANAR_VIEWPOINTS`, `Viewpoint` → `PlanarViewpoint`) in: `🔨️modules/🖨️representation` (+`🪞️copies`), `🎨️depiction/🖨️preview`, `🗂️catalog` (+`🩺️check`), `🔭️multiview`, `📤️export/🎬️cli`, `📤️export/📑️sheet`, `🧮️evaluation`, `🎨️depiction/🧩️shapes`; all of `🎯️targets/⚛️react` (`Viewpoint` → `PlanarViewpoint`, i18n label strings kept); tests `🧪️tests/{🎨️seven-modes,🔤️caption-fit,📐️dimension-clearance,🎨️view-synchrony,🔭️multiview-coverage,🧊️axial-twist,🧊️depth-order,🧊️member-solid,🧊️three-views}`, `🎯️targets/⚛️react/🧪️tests/{🔭️viewpoints,🧭️viewpoint-pages}`.
- Data: `VIEWPOINT_CAPTIONS.axonometric` = "Axonometric view" / "Axonometrie" (`🎨️depiction/✂️section/🟦️.ts:29`); `sectionOf` returns `undefined` for axonometric (no cut).
- Python consumer `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py`: `VolumeUnavailableError`, refusal in cast validation and index reading.

## "Not available yet" refusal sites (replace each, never fall back)

All throw `VolumeUnavailableError(feature, site)` (`.reason` has `en` and `de`); `grep -rn VolumeUnavailableError S ../⚡️energy`. Lines are as of this report.

| Site | Feature | Behaviour |
|---|---|---|
| `S/🔨️modules/🔗️contracts/🟦️.ts:18` (class), `:34` `requirePlanarViewpoint` | all | the error and the narrowing helper every site below uses |
| `S/🔨️modules/🎨️depiction/🟦️.ts:149` `depict` | axonometric | throws; the future axonometric depiction (DrawingScene of the model) goes here |
| `S/🔨️modules/🎨️depiction/🟦️.ts:237` `modelScene` | axonometric | throws |
| `S/🔨️modules/🎨️depiction/🔭️views/🟦️.ts:31` `cameraOf` (used by `project`, `nearness`, structure projection) | axonometric | throws; `CAMERAS` is `Record<PlanarViewpoint, Camera>` |
| `S/🔨️modules/🎨️depiction/🧊️depth/🟦️.ts:20` `depthOf` | axonometric | throws |
| `S/🔨️modules/🎨️depiction/🏷️annotations/🟦️.ts:55` `dimensionLine` | axonometric | throws |
| `S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts:152` `--viewpoint` | axonometric | throws |
| `S/🔨️modules/📤️export/🎬️cli/🟦️.ts:86` `parseArguments` `--format` | glb, fbx, model | throws before the format enum check; `CLI_FORMATS` has no new entries |
| `S/🔨️modules/📤️export/🎬️cli/🟦️.ts:87` `--viewpoint` | axonometric | throws |
| `S/🔨️modules/📤️export/🎭️cast/🟦️.ts:80` `authoredRepresentation` | axonometric | throws |
| `S/🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts:108` `validateCast` | axonometric | issue string naming `VolumeUnavailableError` (cast JSON schema `🔣️.schema.json` still lists the three planar viewpoints) |
| `S/🎯️targets/⚛️react/🔨️modules/🧩️element/🟦️.ts:153` `viewpoint` attribute | axonometric | in-element alert with `.reason` |
| `S/🎯️targets/⚛️react/🔨️modules/🗺️routing/🟦️.ts:171` `isViewpoint` (URL parameters) | axonometric | throws |
| `S/🎯️targets/⚛️react/🔨️modules/📤️downloads/🟦️.ts:113` `metadataText` | axonometric | throws |
| `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py:232` (cast validation), `:730` (`_index`), class `:718` | axonometric | issue string / `VolumeUnavailableError` (Python) |

Not refusals but to extend when the pipeline exists: the React viewpoint UI (`🎯️targets/⚛️react/🔨️modules/🔭️viewpoints`, `🎛️viewer`, `⚖️compare`, `🧪️playground`, `💾️persistence`, `🗺️routing`) offers only `PLANAR_VIEWPOINTS`; the React `📤️downloads` module keeps its own seven-value `ExportFormat` (no glb/fbx/model); `📤️export/📑️sheet` draws the three planar views; `asset.exports` may now list `glb|fbx|model` (nothing consumes it yet).

## Commands

From `S/📦️packages/🟦️typescript` (library), `S/🎯️targets/⚛️react/📦️packages/🟦️typescript` (React), `S/🌐️site/📦️packages/🟦️typescript` (site):

```
repo run @teaching/staffage:typecheck           repo run @teaching/staffage:test --lang=en
repo run @teaching/staffage-react:typecheck     repo run @teaching/staffage-react:test --lang=en
repo run @teaching/staffage-site:build --lang=en
repo tools exec --light tsc -- -p tsconfig.json --noEmit --incremental false          # type-check each of the three packages, run in its folder
repo tools exec --light vitest -- run --config ../../🧪️tests/🎚️config/🟦️.ts volume-schema schema-conformance      # library tests, filter by folder name
repo tools exec --light vitest -- run --configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts               # React tests (React package folder)
python3 -m py_compile "⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py"                                           # Python consumer (the guard tests there need uv)
```

`repo run …` was refused by the host's capacity gate while other agents ran (free memory below the floor); `repo tools exec --light` runs the same tools without waiting. The shapely test needs `uv` (`repo tools setup uv`).

## Baseline versus final

Type-check (tsc errors, all pre-existing, identical set before and after): library 4 (`🔺️polygon` 3, `🏷️cast-provenance` 1), React 28, site 49 (React and site errors are missing generated framework files). New errors: 0 in all three.

Library vitest. Baseline = `git archive HEAD` of `🎓️teaching` run in a scratch copy with bun and vitest directly (`repo` cannot run outside a checkout; the copy lacks `🧰️framework`, so 4 files could not load there: `🎞️frame-timing`, `📑️multiview-sheet`, `🔭️multiview-coverage`, `🧬️schema-twin`). Final = working tree.

| | passed | failed | skipped | files |
|---|---|---|---|---|
| baseline (HEAD) | 1402 | 30 | 0 | 84 |
| final | 1492 | 31 | 25 | 85 |

The 31 failures are the 30 pre-existing ones (identical test names) plus `🧊️volume-schema` "equals the verdicts of shapely" (needs `uv`, missing on this host). Pre-existing causes: missing `uv`, `ffmpeg`, `rsvg-convert`, `qpdf` (about 20 files fail to load or run: they `requireTool` at import), plus catalog state of other waves (`🧍️plan-projection`, `📐️dimension-clearance`, `🎭️cast-export`, `🔄️continuous-rotation`, `🧭️representation-resolution`, `🪞️representation-copied`, `🧬️schema-conformance` "unavailable without reason": the sample's 1:500 is authored, so the removed key never existed). New volume tests: 88, of which 87 pass.

Because the `requireTool("uv")` files do not load here, the type-level refactor of `🧊️axial-twist`, `🧊️depth-order`, `🧊️member-solid`, `🧊️three-views`, `🔭️multiview-coverage` was verified by tsc only, not at run time. React vitest (final only): 76 passed, 1 failed (`📚️catalog-loading`: 19 live assets invalid through `solid-sweep` and extent findings of other waves; the scan `T/volume-catalog-scan.ts` shows no `volume-*` finding and no asset has a volume), 10 files fail to load on missing generated framework tokens.

## Authoring guide for `volume`

Frame: `u` along the bone from joint to tip, `v` its normal (u turned +90 degrees counter-clockwise in the side view), `w` lateral (left +); origin at the bone origin; metres. In the sample figure the pelvis, torso and head bones point up (so `v` is backward, the face is at `-v`), arm and leg bones point down (`v` forward), a foot bone points forward (`v` up). Read lateral positions from the plan shapes (arms w = ±0.225, legs and feet w = ±0.09), radii and lengths from the elevation. Give left and right their own solids. Every solid names `id`, `kind`, `bone`, `material`, `role`, `tone` (copy it from the same-id elevation shape), and `region` (the thermal region of that shape). Check: bounding box of all solids at rest equals `dimensions` within max(2 %, 15 mm) (`🧪️tests/🧊️volume-schema` `worldBox` computes it for axis-aligned rests).

`ellipsoid` (head, bone `head`: elevation ellipse `cx` 0.1 along u, radii from elevation `ry`, front `ry`, plan `rx`; centred on the origin, moved by `transform`):
```json
{ "id": "head", "kind": "ellipsoid", "bone": "head", "material": "skin", "role": "body", "tone": 0.85, "region": "head",
  "radii": [0.115, 0.095, 0.085], "transform": { "translation": [0.1, 0, 0] } }
```
`capsule` (left arm, bone `arm-l`, 0.6 m along u at w = +0.225, radii of the elevation capsule):
```json
{ "id": "arm-l", "kind": "capsule", "bone": "arm-l", "material": "cotton-clothing", "role": "body", "tone": 0.5, "region": "arms",
  "p0": [0, 0, 0.225], "p1": [0.6, 0, 0.225], "r0": 0.04365, "r1": 0.03395 }
```
`loft` (torso, bone `torso`, three sections along u: hip, waist, shoulder; `rx` is the lateral half width (w), `ry` the sagittal half depth (v), `exponent` 2 is an ellipse, larger is squarer):
```json
{ "id": "torso", "kind": "loft", "bone": "torso", "material": "cotton-clothing", "role": "body", "tone": 0.45, "region": "torso",
  "sections": [ { "at": [0, 0, 0], "rx": 0.16, "ry": 0.1261, "exponent": 2.4 }, { "at": [0.22, 0, 0], "rx": 0.18, "ry": 0.14, "exponent": 2.4 }, { "at": [0.45, 0, 0], "rx": 0.19, "ry": 0.1552, "exponent": 2.2 } ],
  "capStart": true, "capEnd": true }
```
`revolve` (hair cap on the head: profile [axial, radial] from the equator to the crown, the last radial 0; axis u through the bone origin; `lateralScale` makes the cross-section 0.895 as wide as deep):
```json
{ "id": "hair", "kind": "revolve", "bone": "head", "material": "hair", "role": "body", "tone": 0.2, "region": "head",
  "profile": [[0.1, 0.1007], [0.16095, 0.0872], [0.2056, 0.0504], [0.22005, 0.01749], [0.2219, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0], "lateralScale": 0.895 }
```
`extrude` (left shoe upper, bone `foot-l`: side profile in (u, v) on plane `xy`, pushed along w from 0.045 to 0.135; for `yz` the profile is (v, w) along u, for `zx` (w, u) along v; holes go in `holes`, strictly inside, apart):
```json
{ "id": "shoe-l", "kind": "extrude", "bone": "foot-l", "material": "leather-shoe", "role": "body", "tone": 0.15, "region": "feet",
  "profile": [[-0.06, -0.065], [0.2, -0.065], [0.21, -0.055], [0.21, -0.045], [0.14, -0.02], [0.03, -0.01], [-0.04, -0.01], [-0.06, -0.03]],
  "plane": "xy", "from": 0.045, "to": 0.135 }
```
`box` (left sole, centred on the origin, moved to u 0.075, v -0.0725, w 0.09; `cornerRadius` at most half the smallest edge):
```json
{ "id": "sole-l", "kind": "box", "bone": "foot-l", "material": "leather-shoe", "role": "detail", "tone": 0.08, "region": "feet",
  "size": [0.27, 0.015, 0.09], "cornerRadius": 0.005, "transform": { "translation": [0.075, -0.0725, 0.09] } }
```
Also in the sample: `weights` on the neck (`[{ "bone": "head", "weight": 0.7 }, { "bone": "torso", "weight": 0.3 }]`, solid framed by `head`), `repeat` on the buttons (`{ "count": 3, "translation": [0.08, 0, 0] }`, copy k moved by k times the step), `maxDenominator: 50` on nose and buttons (shown at 1:50 only), `transform.translation` to place centred kinds. Always write `decisions` (bilingual, at least one). Run `volumeIssues(asset)` (or `validateAsset`) and fix by rule id.

## Deviations from design.md

- `VolumeSolid` also has optional `tags` (the kernel `Solid` has them); `weights` items are `{ bone, weight }` (design: `weights?` over bones) with 1 to 4 entries and non-negative weights; `capStart` and `capEnd` of a loft are required (no default); `Volume.decisions` has at least one entry.
- Added `$defs` `Vector2`, `Vector3`, `Quaternion` (the design inlines them). The repeat count is bounded at 256 (the design said "bounded"). `Asset.volume` is optional until the coordinator flips it.
- The design lists `Viewpoint` + `axonometric`; to keep the three-view code correct the twin adds `PLANAR_VIEWPOINTS` / `PlanarViewpoint`, and `EvaluatedState.shapes`, `CAMERAS`, geometry keys use it. `DepictionOptions.viewpoint` and `DrawingScene.metadata.viewpoint` stay `Viewpoint`.
- Semantics the design left open and this schema fixes in its descriptions; the kernel `semio.model/v1` text must match: loft section frame (tangent to the next centre; first axis = bone-local `w` projected perpendicular to the tangent, `v` if parallel; `rx` on it, `ry` on the second), `rotation` a quaternion turning the section; `extrude.plane` mapping (`xy` u,v along w; `yz` v,w along u; `zx` w,u along v) and `taperTo` scaling about the plane origin; `capsule`/`revolve` `lateralScale` stretches along `w` (along `v` if the axis is parallel to `w`); revolve `sweepRad` starts in the plane of the axis and the sagittal normal, open sweeps get flat faces; `transform` order scale, rotation, translation; `repeat` copy k = step^k after `transform`; `maxDenominator` is the largest preset denominator that still shows the solid; a solid without region/regions belongs to no thermal region.
- Validator is stricter than shapely on purpose: repeated points are degenerate and a hole touching the outline is rejected (flagged `oracle: false` in the vectors).

## Open issues

- `uv`, `ffmpeg`, `rsvg-convert`, `qpdf` are not installed on this host; run `repo tools setup all` to execute the shapely oracle and the other tool-bound suites. The shapely verdicts of `🧫️fixtures/🧊️volume-schema` are hand-labelled and were not executed; the exact-predicate and ajv checks were.
- `volume-non-finite` has no JSON vector (JSON cannot carry Infinity); it is tested in code.
- The catalog-wide guard of the design (bounding box, per-view silhouette agreement, GLB/FBX validity) is not built; `worldBox` in the volume test is a sample-only prototype for axis-aligned rest frames and revolves about `u`.
- 19 live catalog assets currently fail `validateAsset` for other reasons (`solid-sweep` 351 findings, extent and `lateral-ambiguous`), none for volume.
- `T/🗑️generated` is cleaned by other agents; the raw logs of this run are gone, the numbers above are the record.
- Scripts kept in `T`: `volume-schema-edit.py`, `sample-volume-author.py`, `volume-vectors-author.py`, `vitest-summary.py`, `volume-catalog-scan.ts`.
