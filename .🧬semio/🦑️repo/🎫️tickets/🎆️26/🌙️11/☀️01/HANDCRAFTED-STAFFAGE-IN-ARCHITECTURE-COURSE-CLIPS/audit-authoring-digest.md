# 🧍️ Staffage Authoring Digest (read-only audit)

Scope: how to author one new handcrafted asset in `🎓️teaching/🏛️architecture/🧍️staffage`. Read-only: no file was changed except this report.
Sources read: `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`, `🗂️catalog/🔣️.json`, `👤️humans/seated-reader/🔣️.json`, `🪑️objects/patio-heater/🔣️.json`, the module headers of `🔨️modules/🗂️catalog/🩺️check` and `🔨️modules/🎨️depiction/🖨️preview`, `📦️packages/🟦️typescript/📋️project.json` and `📜️script.ts`, `🧪️tests/`, and the old ticket `ARCHITECTURAL-STAFFAGE-ARCHIVE` (authoring-v2, authoring, content-worker-brief, completion-audit-2026-10-30, staffage-catalog-fixes, six-category-print-check).

## Caveats first

- The old guides are partly stale. `authoring.md` describes a per-category `CategoryDocument` file; the current layout is one file per asset. Use `authoring-v2.md` plus the schema and typed twin as the contract.
- The old guides call `repo tools exec bun -- <file>`. Module headers still document that form. `project.json` also defines `repo run @teaching/staffage:test` and `:typecheck`.
- The manifest sync script (`sync-manifest.ts`) exists only in the old ticket's `🧰️scripts`. No sync tool exists in the archive now. The manifest is validated by the check, so every new asset needs a hand-written manifest entry (see section 2).
- The schema lists `appliances` (`🔌️appliances`) as a category, but that folder does not exist yet. The manifest has 107 entries across nine categories, all `completion: authored`.
- At audit time `🧬️schema/🔣️.json` and `🟦️.ts` had uncommitted modifications. Re-check the schema before relying on field names.
- No check or test was executed. Commands below come from module and script headers. Running them writes caches and generated PNGs.

## 1. File layout

- Asset file: `🗂️catalog/<category folder>/<asset-id>/🔣️.json`. Folder names: `👤️humans`, `🐕️animals`, `🌳️vegetation`, `🛋️furniture`, `🪑️objects`, `🚜️vehicles`, `🏗️machinery`, `🧱️components`.
- Asset-id is a kebab-case slug, and it must equal the `id` field and the folder name.
- Top-level shape:
  ```
  {
    "$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument",
    "version": "semio.staffage/v2",
    "asset": { ... }
  }
  ```
  The relative `$schema` path has three `../` levels (asset folder, category folder, `catalog`).
- Manifest: `🗂️catalog/🔣️.json` (`$defs/Manifest`) has `categories`, `collections` and `entries`. Each asset needs exactly one entry.
- Reference template: `🧫️fixtures/🚶️reference-walker/🔣️.json`, a small valid v2 figure. Catalog assets must be richer than it.
- Scratch and generated output (per AGENTS.md): scripts and previews go in the ticket folder, and previews go under `🗑️generated`. Delete generated files when done.

## 2. Required fields

Asset (`$defs/Asset`, all required): `id`, `category`, `title`, `summary`, `tags`, `provenance`, `dimensions`, `materials`, `rig`, `anchors`, `representations`, `symbols`, `poses`, `clips`, `thermal`, `structure`, `presentation`, `exports`, `completion`.

- `title`, `summary`, and every `Text` (`{en, de}`) must be non-empty in both languages. No default language.
- `completion`: `planned | authored | verified`. Use `authored` for new content.
- `exports`: list of `svg pdf dxf dwg png-sequence mp4 webm component`. `seated-reader` uses the full list.
- `dimensions`: `heightM`, `widthM`, `depthM`, `massKg`, all positive. These are the default-pose bounding box and total mass.
- `presentation`: `defaultPose`, `defaultClip`, `defaultScale` (one of 500, 200, 100, 50), `viewpoints` (`elevation`, `plan`).
- `tags`: slugs.

Manifest entry (`$defs/ManifestEntry`, all required): `id`, `category`, `title`, `heightM`, `widthM`, `poses`, `clips`, `representations` (keys 50, 100, 200, 500, each `authored` or `unavailable`), `symbols`, `analytics`, `viewpoints`, `exports`, `distinction` (Text), `completion`.

- The validator checks that the entry matches the asset: `poses` are the non-key poses, `clips`, `symbols`, `viewpoints`, `exports`, `completion`, the four representation statuses, `analytics`, and height/width within 5% and 10% of the asset's dimensions.
- `analytics` must contain `thermal` iff the asset has thermal regions, and `transport` iff it has emitters.
- A manifest entry counts toward its category. Category counts must match.
- Collections (`street-life`, `courtyard-trees`, `accessibility`, `structural-references`, `construction-site`) list asset ids and must resolve.
- Every `unavailable` representation's `asset`-type alternative must resolve.

Provenance (`$defs/Provenance`): `author` (string), `method` (const `handcrafted`), `created` (`YYYY-MM-DD`), `references` (list of `{id, citation, url?}`, ids are slugs), `notes` (optional Text).

## 3. Rig, poses and clips

Rig (`$defs/Rig`): `kind` is `anatomical | botanical | mechanical | structural`, `root` is a bone id, `bones` is a list.

Bone: `id`, `parent` (null for the root), `title`, `x`, `y`, `angleDeg`, `lengthM`, `joint`, optional `mass` and `section`. Revolute joints take `minDeg`/`maxDeg`. Prismatic joints take `minM`/`maxM` along `axisDeg`. Fixed joints take no limits. The rig must be one tree with one root, and joint limits must be respected by every pose and key.

Pose (`$defs/Pose`):
- `id`, `title`, `description`, `bones` (map of bone id to `{angleDeg?, dx?, dy?}`), `ground` (`auto | authored`), `contacts` (anchor ids), `thermal` (binding), `loads`.
- Optional `key: true` marks an in-between pose. Key poses are not named poses, so each asset still needs at least 3 named poses.
- Optional `root` `{x, y, angleDeg}`.

Clip (`$defs/Clip`): `id`, `title`, `description`, `durationS`, `fps`, `loop`, `rootMotion` (`none | plant`), `keys`, `contacts`, `phases`, `events`, `tracks`. Optional `advanceM` (required for looping planted locomotion).
- `keys`: `{t, pose, ease, bones?}`. Ease is `linear | smooth | in | out | step`. Use `smooth` for continuous motion, `step` for switches.
- `contacts`: `{anchor, fromS, toS}` spans. A span crossing the loop seam is written as two spans.
- `phases`: `{id, fromS, toS, title, ballistic?}`. `ballistic: true` is the only way an instant without contact is legal.
- `events`: `{t, kind, title, target?}`. Kinds: `contact-on, contact-off, ignition, shutdown, load-on, load-off, thermal, motion, operating`. Inhale and exhale events are not allowed.
- `tracks`: `{target, ease, keys[{t, v}]}`. Targets: `activityMet`, `externalWorkW`, `emitter:<id>`, `operating:<id>`, `load:<id>` (scale), `region:<id>:<field>`, `breathing` (pattern index, `step`).

Rules from the audits:
- Looping clips must close: the closing key equals the first key, except on unlimited revolute joints where whole 360° turns are allowed (`loop-closing`).
- Locomotion `advanceM` must match the planted travel per cycle within 2 mm (`clip-advance`).
- Each instant must have a contact unless a `ballistic` phase covers it (`unsupported-instant`).
- Use `dynamics: "quasi-static"` by default. Use `inertial` only when keys are dense and smooth enough, with at least 8 smooth keys per cycle.

## 4. Thermal binding

Model (`$defs/ThermalModel`): `environment` (reference case), `baseMet`, `body`, `respiration`, `passengers`, `operating`, `regions`, `emitters`, `assumptions` (Text list).

Environment: `airTempC`, `meanRadiantTempC`, optional `skyTempC` and `groundTempC`, `relativeHumidity`, `windMS`, `windDirectionDeg`, `solarWM2`, `diffuseFraction`, `pressurePa`. Presets live in `🗂️catalog/🌤️environments/🔣️.json`.

Regions (`$defs/ThermalRegion`):
- Required: `id`, `title`, `kind` (skin, clothing, hair, fur, feather, foliage, bark, stem, surface, engine, exhaust, glazing, tyre, wall, heater, lamp, water), `model`, `material`, `areaM2`, `characteristicLengthM`, `geometry` (cylinder, sphere, vertical-plate, horizontal-plate-up, horizontal-plate-down), `skyViewFactor`, `layers`, `heatCapacityJM2K`, `source`.
- `model` is `body | leaf | passive | operating | conditioned | fixed`. Each model adds its own block:
  - `body`: `{segment, metabolicShare, skinFraction, subject?}`. Metabolic shares must sum to 1 per body. Skin fractions must sum to 1.
  - `leaf`: `{stomatalConductanceMaxMS, leafAreaIndex, photosynthesis}`.
  - `operating`: `{key, heatW}`, where `key` names an operating key.
  - `conditioned`: `{interiorTempC, interiorFilmM2KW}`.
  - `fixed`: `{surfaceTempC}`, reference fixtures only.
- `layers` run innermost first: `{id, title, kind, material, thicknessM?, resistanceM2KW, vapourResistanceM2PaW?, heatCapacityJM2K, source, reference?}`. Bare skin has `layers: []`.
- Optional `hidden` (Text) explains why a region is not drawn in elevation. Otherwise every region must be drawn at every authored scale (`region-undrawn`).

Physiology and respiration:
- `body` (`$defs/BodyPhysiology`): `species` (human, dog, cat, horse, cow, deer, bird, sheep, goat, pig, fox, squirrel, hedgehog, duck, gull), `massKg`, `skinAreaM2`, `basalMetabolismW`, `metWM2`, set points, `sweating`, `panting`, skin blood-flow limits.
- The physiology mass is not the same as `dimensions.massKg`. Seated-reader uses 61 kg for physiology and 64.5 kg for dimensions.
- `respiration`: `noseEmitter`, `mouthEmitter` (emitter ids), `exhaledTempC`, `exhaledRelativeHumidity`, `patterns`, `motion`, `source`.
  - Pattern: `{id, title, rateBpm, tidalVolumeL, inhaleFraction, pauseAfterInhaleFraction, exhaleFraction, pauseAfterExhaleFraction, route (nose | mouth | nose-and-mouth), source, reference?}`. The four fractions sum to 1.
  - Motion: `{bone, angleDeg, dy}` per tidal volume.
- `passengers`: a second living body (rider, toddler). Regions name it in `body.subject`. Poses bind it in `thermal.passengers`.

Emitters (`$defs/Emitter`): `kind` (breath, exhaust, vent, flue, fan, transpiration, plume), `anchor` (or `anchors` for distributed), `directionDeg`, `diameterM`, optional `operatingKey` with `volumeFlowM3S`, `temperatureRiseK`, `vapourKgPerKg`, optional `region`, `source`.

Operating keys (`operating`): `{id, title, levels: [{v, title}]}`. Level 0 is off and 1 is rated. Patio-heater's burner has `off 0`, `low 0.46`, `high 1`.

Pose bindings (`$defs/ThermalBinding`):
- Bodies: `activityMet`, `breathing` (pattern id), optional `externalWorkW`, `passengers`.
- Machines: `operating` map of key id to level.
- Every pose: `regions` for every region (`exposedFraction`, `sunlitFraction`, optional `wettedness`, optional `contact {fraction, tempC}`), and `emitters` for every non-breath emitter (strength 0..1). Missing emitter bindings default to 0.
- Seated-reader's pose uses `emitters: {"body-plume": 0.6}` and no breath entries. That matches the rule.

Authoring rules from the v2 guide:
- Do not author temperatures. Only the reference fixtures use `fixed`.
- Every physical parameter needs a `source` and, when it comes from literature, a `reference` id that resolves to `provenance.references`.
- Use `authored-estimate` or `illustrative` when no real checkable source exists. Do not tag an estimate as `literature`.
- Clothing: `resistanceM2KW` from ISO 9920 (1 clo = 0.155 m²K/W), including the trapped air layer. Vapour resistance is `I/(16.5e-3·i_m)` with `i_m ≈ 0.38` unless sourced.
- Heat capacity guide: skin plus clothing about 10 to 30 kJ/(m²K), steel body about 6 kJ/(m²K), leaves about 0.5 to 1 kJ/(m²K).
- Plume emitter on each body at the crown or back anchor.

## 5. Structural binding

- Bone mass (`$defs/SegmentMass`): `kg`, `comX`, `comY`, `distribution` (`point` or `uniform`), `source`, `reference`.
  - Humans use de Leva and Winter segment fractions, tagged `literature`.
  - Sum of all bone masses should equal `dimensions.massKg`. Seated-reader sums to 64.50 kg exactly.
- Section (`$defs/Section`, members only): `areaM2`, `secondMomentM4`, `extremeFibreM`, `material`, `title`.
- Anchor (`$defs/Anchor`): `id`, `bone`, `x`, `y`, `kind` (contact, support, attachment, emitter, load), `title`. Supports add `support` (`contact | pin | roller | fixed`), `normalDeg` (default vertical), `footprintLengthM`, `footprintWidthM`, `frictionCoefficient`.
  - Contacts sit at the ball of the foot or the tyre patch, not at the ankle. Every contact needs a footprint.
- `structure` (`$defs/StructuralModel`): `gravityMS2` (9.80665), `dynamics`, `supports` (anchor ids), `assumptions` (Text), `clinical` (false unless a clinical model). Seated-reader uses `inertial`; patio-heater uses `quasi-static` with a base contact support.
- Loads (`$defs/AppliedLoad`, in `pose.loads`): `id`, `kind` (carried, imposed, live, wind, snow, occupant, equipment), `title`, `anchor` or `bone` plus `x`/`y`, `fxN`, `fyN`, optional `momentNm`, `distributed` (`{bone, fromM, toM, wxNPerM, wyNPerM}`), `source`, `reference`.
  - Patio-heater's `high-in-breeze` pose carries three wind loads.
  - Live loads follow EN 1991-1-1: balcony 4 kN/m², stairs 3 kN/m², barrier line load 1 kN/m.
- Reserve bones (`fixed` joints with mass) are allowed for static parts, such as patio-heater's `base`, `bottle`, `pole`, `burner-head` and `reflector`. Do not invent anatomical joints for static objects.

## 6. Provenance tags

- Parameter `source` (`$defs/ParameterSource`): `measured | literature | calibrated | authored-estimate | illustrative`.
- Attach a `reference` slug only when it resolves in `provenance.references`. Each reference has `id` (slug), `citation`, optional `url`.
- Materials carry `source` and `reference` as well.
- Ids used in the two example assets: `de-leva-1996`, `winter-2009`, `drillis-contini-1966`, `dubois-1916`, `gagge-1971`, `icrp-66-1994`, `iso-9920-2007`, `ashrae-2021`, `incropera-2018`, `iso-10456-2007`, `en-1991-1-4`, `en-14543`, `en-1442`, `turns-2012`.
- `provenance.notes` (Text) records the basis of the estimates, as patio-heater does.

## 7. Representations and LOD

All four keys `"50"`, `"100"`, `"200"`, `"500"` are required. Each is one of two forms.

Authored (`status: "authored"`):
- `denominator`: 500, 200, 100 or 50.
- `validRange`: `{minDenominator, maxDenominator}`, containing the own denominator. Typical ranges: 500 → 350…1000, 200 → 150…350, 100 → 75…150, 50 → 20…75. Ranges of neighbouring scales must not overlap (`valid-range-overlap`).
- `pens`: `{body, detail, texture, glazing, ground}` in paper mm. `detail`, `texture` and `glazing` may be `null` when the role is not drawn at that scale.
- `hatches`: `{id, title, pattern (lines | cross | dots | stipple), angleDeg, spacingMm, penMm}`.
- `geometry.elevation` (required) and `geometry.plan` (optional).
- `decisions`: eight Text fields, all required: `silhouette`, `retained`, `merged`, `omitted`, `thinElements`, `hatching`, `styles`, `overlays`.

Shapes (`$defs/Shape`): `kind` is `capsule | ellipse | rect | poly | path`. Common fields: `id`, `bone`, `role` (`body | detail | texture | glazing | ground`), `z`, `filled`, `tone` (0..1), optional `region` or `regions` (for merged silhouettes at small scales), `material`, `hatch`, `penMm`.
- Path shapes use SVG path syntax in local metres, y up.

Unavailable (`status: "unavailable"`): `denominator`, `reason` (Text), `limitingFeature` (Text), `limitingSizeM`, `alternatives` (at least one: `scale`, `symbol` or `asset`). Never leave a scale undecided.

Authoring rules:
- Every scale is designed for paper, not copied from one set. A 1.80 m person is 3.6, 9, 18 and 36 mm tall at 1:500, 1:200, 1:100 and 1:50.
- 1:500: one composed silhouette (1 to 4 closed paths), at most one internal edge. Thin parts are thickened, merged or dropped, and the decision says which. Detail usually `null`.
- 1:200: main masses separated, one to three internal edges, no texture.
- 1:100: articulated clothing outlines, mitten hands with thumb, shoes, hair mass, restrained hatching.
- 1:50: full articulation where printable.
- All representations bind to the same rig bones and must reproduce `dimensions` in the default pose (`extent-height` ±5%, `extent-width` ±10%).
- Every region drawn at a scale is listed in the shape's `region` or `regions`. Merged shapes are painted with area-weighted values and labelled as merged in `decisions.merged`.
- Distinct drawings: each scale must differ from the others (`representation-copied`). Shift a coarser drawing by about 0.5 mm if needed, as the fixes audit did.

Symbols (`$defs/SymbolRepresentation`): `id`, `title`, `description` (Text), `semantics` (`true-extent | true-position | nominal`), `units` (`model-m | paper-mm`), `denominators`, `pens`, `hatches`, `geometry`, `dimensionalNote` (Text).
- `true-extent`: model metres at real size.
- `true-position`: insertion point at the model origin, geometry in paper mm. Seated-reader's marker is `true-position` with paper-mm units.
- `nominal`: fixed-size sign.

Print suitability (`🗂️catalog/🖨️print-profiles/🔣️.json`):
- Default profile `office-laser` (600 dpi): min stroke 0.1 mm, min gap 0.15 mm, min feature 0.2 mm, min hatch spacing 0.4 mm.
- Stricter `draft-300dpi` exists for review.
- Authored drawings must pass the default profile, or be redesigned or declared unavailable.
- Pens are at most about 1.99 mm. Pens of 0.13 to 0.18 mm are typical at 1:500.

## 8. Commands

Run from the repo root (`cd /c/git/semio`). Full paths are shown relative to the repo root.

Catalog check (validation, evaluation at every pose and clip sample, thermal and structural solves, all views at each authored scale, print suitability):
```
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset seated-reader
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --category humans
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file 🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/seated-reader/🔣️.json
```
Options: `--samples 9`, `--profile <id>`, `--no-physics`, `--catalog <root>`. Without `--asset` or `--category` the manifest is validated against all assets. Prints `✓` or `✗` per asset, exits 1 on any failure.

Library tests (Vitest, all cases under `🧪️tests/<case>/🟦️.ts`, plus Gherkin `.feature` files that carry the language-agnostic contract):
```
repo run @teaching/staffage:test
repo run @teaching/staffage:typecheck
```
`test` takes an optional level argument (`fundamental | quick | long | exhaustive`) and forwards other arguments to Vitest. `project.json` runs `repo ./📜️script.ts test`. The `--lang=en` form in the staffage README does not match `script.ts`, so do not rely on it. Asset-relevant cases: `✅️asset-validation`, `🧬️schema-conformance` (uses Ajv as the third-party oracle), `🧬️schema-twin`, `🌡️catalog-physics`, `📏️print-suitability`, `🧭️representation-resolution`, `🎭️cast-export`, `🧫️`-fixture based cases.

Preview (PNG and SVG per pose, clip time, symbol, view, or contact sheet):
```
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset seated-reader --scale 100 --sheet --out <ticket>/🗑️generated/<asset>
repo tools exec bun -- .../🖨️preview/🟦️.ts --asset seated-reader --scale 50 --clip page-turn --time 1.5 --view elevation --out <ticket>/🗑️generated/<asset>
repo tools exec bun -- .../🖨️preview/🟦️.ts --asset seated-reader --scale 100 --print-size --dpi 600 --out <ticket>/🗑️generated/<asset>
```
Flags: `--scale 500|200|100|50`, `--pose <id>`, `--clip <id> --time <s>`, `--symbol <id>`, `--view elevation|plan`, `--sheet`, `--print-size` (10 mm calibration bar), `--dpi`, `--profile`, `--language en|de`, `--no-png`, `--transport parcels,pathlines,streamlines,condensation`, `--catalog`, `--file`. PNG rendering uses the system `rsvg-convert` behind the rasterizer interface, so pass `--no-png` if it is missing. An unavailable or unsuitable scale writes an explanation card and exits 1.

Look at the PNGs with the Read tool at print size and zoomed, then iterate. Delete generated files when done.

## 9. Pitfalls from past audits

Schema and content:
- Using the v1 guide's per-category `CategoryDocument` layout, or any v1 fields: `lod`, top-level `geometry`, `coreTempC`, `riseK`, `surfaceTempC`, `insulationM2KW`, region-level `metabolicShare`, breath `periodS`/`duty`, `inhale`/`exhale` events. None of these is in the v2 `$defs`. Confirm rejection with the check, since I did not test the schema's `additionalProperties` handling.
- Writing `null` for an absent optional field (`skyTempC`, `groundTempC`). Omit the key instead. Null sky and ground temperatures broke pose and clip evaluation in panel-radiator.
- Authoring temperatures or fixed surface temperatures outside reference fixtures.
- Setting a non-zero emitter or operating level in a pose and forgetting it in another pose. Unbound entries default to 0, and every pose must bind every region and emitter.
- Metabolic shares not summing to 1 (`metabolic-share`), skin fractions not summing to 1 (`skin-fraction`), thermal regions not drawn by any shape at a scale (`region-undrawn`), regions with no binding in a pose (`binding-region-missing`).
- Duplicate keys at the same time in a track, such as `books-5-l` at t = 5 s in bookshelf. Remove the duplicate.
- Missing `hatch` id referenced by a shape (heat-pump-outdoor-unit had a missing `fins` hatch).
- Inconsistent manifest entries: the four representation statuses, `analytics`, poses or clips lists, or dimensions outside tolerance.

Thermal values:
- Many machine surfaces reached 327 to 339 °C at the default scenario. Fixes were lower `heatW` values (for example city-car brakes 1406.3 W, forklift brakes 6011.7 W, electric-kick-scooter bms-controller 1.8 W) and capped braking levels (0.05) in named poses. The `heatW` values were then re-checked against the physics.
- heat-pump-outdoor-unit had a negative `heatW` (coil-guard -220 W) and a base-tray value of 185.5 W, and both were changed to bring surfaces into range (-7.6 °C and 36 °C). Negative values are a sign to review the physics, not a fix.

Print suitability:
- `suitability-gap` and `suitability-feature` violations were common. In the office-laser scan 57 assets failed, with about 1112 gaps and 280 thin fills.
- A pen-join pass that only thickens one pair does not converge, because thickening pulls the next pair into the 0.15 mm band. Fix shape by shape: raise `penMm` on the failing shapes until `(penA + penB) / 2 ≥` centre distance, and keep pens at or below 1.99 mm. Do not rewrite outlines or extents to fix print issues.
- Thin fills and strokes need the feature and stroke limits, not just thicker pens. Thin spokes, poles and fingers are thickened only if that stays within outline tolerance, merged into a mass, or dropped (and the decision says which).
- Scales that look identical at the coarser size fail `representation-copied`. Shift the coarser drawing by 0.5 mm or redraw it.

Dimensions and drawings:
- `japanese-maple` 1:500 was outside the 2% / 15 mm tolerance. The fix rescaled the default pose to match. Check extents after any shape edit.
- Machines: set `presentation.defaultScale` to 100 where a 500 drawing is unreadable.

Verification:
- The completion audit (2026-10-30) passed the catalog check 107/107 and library tests 845 passed, 1 skipped (DWG reopen when the DWG tool is absent). Typecheck passed for all three packages. The React and site suites also passed. Treat these as the baseline.
- The check exit code is the gate. Do not declare an asset done unless the check prints `✓` for it and every authored preview was looked at.

## 10. Authoring checklist

1. Pick a category folder and create `🗂️catalog/<category>/<asset-id>/🔣️.json` with the schema header.
2. Write `provenance` with real references, then `materials`, `dimensions`, and `rig` with masses and sections. Check that masses sum to `massKg`.
3. Add `anchors`, then the `thermal` physiology or object model (regions, layers, emitters, operating keys, respiration), each with `source`.
4. Add at least three named poses (and key poses if clips need them), binding `thermal` and `loads` in every pose.
5. Add at least two clips, with keys, contacts, phases, events, tracks, and `advanceM` for planted loops.
6. Add `structure` (dynamics, supports, assumptions).
7. Design all four representations (or `unavailable` with a reason and alternatives), pens, hatches and decisions. Add symbols where useful.
8. Set `presentation` and `exports`, `completion: "authored"`.
9. Add the manifest entry in `🗂️catalog/🔣️.json` (poses, clips, symbols, representations, analytics, viewpoints, exports, distinction, dimensions) and any collection membership.
10. Run the catalog check on the asset and on the manifest. Run the library tests and typecheck.
11. Render previews at 50, 100, 200 and 500, print size and zoomed. Read the PNGs. Iterate.
12. Keep scratch scripts and previews in the ticket folder; delete generated outputs when done.
