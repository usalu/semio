# Compliance audit: staffage archive (read-only), 2026-11-10

Scope: `S` = `🎓️teaching/🏛️architecture/🧍️staffage` and the Python consumer `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py`. Snapshot taken while other agents edit the React target, the site and the depiction module; those are audited as they are. Nothing was modified, no state-changing git command was run.

Method: `git -c core.quotePath=false status --short -uall` and `git diff -U0` (read-only) for the changed set (301 entries under S and the Python consumer, 155 of them under catalog paths: 152 asset files plus the catalog module files). Greps for markers and throw sites over that set, a Python read-only sweep of all 152 catalog `🔣️.json` files (stdin only, nothing written), `ast`-free regex scans of imports and docstrings, JSON parsing of schema and twin, and reads of the ticket docs (`design.md` incl. Amendments, `staffage-volume-schema.md`, `staffage-volume-module.md`, `catalog-fix.md`) and `S/README.md`.

Note: the ticket docs are stale in places. `staffage-volume-schema.md` lists refusal sites (`depict`, `modelScene`, `cameraOf`, `depthOf`, `dimensionLine`, Python `:232/:718/:730`) that no longer exist; the axonometric depiction is implemented.

---

## 1. Comments inside definitions and docstrings

- No `//` or `/* */` comment inside any function, class, type or object body in the changed `.ts`, `.tsx` and `.py` files. Searched for indented `//`, indented `/*`, trailing `; //`, and indented `#` in Python. Only `//#region`/`//#endregion` markers at column 0 exist (structural, for example `🔨️modules/🧮️evaluation/🟦️.ts:40`). Violations: none found.
- Docstrings (`/** */` and Python `"""`): 1,122 in the changed set. Exactly one does not start with an emoji: `🧬️schema/🟦️.ts:550` ("Horizontal bearing the air moves towards, ..."). It is pre-existing (present at HEAD).
- Emoji uniqueness (interpretation: "unique" across the codebase): 204 distinct leading emojis over 1,122 docstrings. Most reused: `🧊` 75 times, `📐` 44, `📏` 39, `🧾` 29, `🏷` 29, `🧭` 28, `🧩` 25. Under a strict reading the rule is not met; no duplicate check was run per file.

## 2. Runtime dependencies and test oracles

Runtime (non-test) imports in changed TypeScript, from a regex scan of `import`, `export from`, `import()`, `require()`:
- Workspace: `@teaching/staffage` (48 sites), `@teaching/staffage-react`, `@teaching/staffage-site`. Internal.
- Node built-ins: `node:fs`, `node:fs/promises`, `node:path`, `node:url`, `node:util`, `node:crypto`, `node:child_process`. Platform built-ins.
- `react` (17 sites, React target). Package-level runtime dependencies `react` and `react-dom` ^19.2.3 in `🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json`. Pre-existing, not changed today. Violation of the no-runtime-library rule, unchanged by today's work.
- Library package `📦️packages/🟦️typescript/package.json`: no runtime `dependencies`, devDependencies only. Compliant.
- Python consumer: `import numpy as np` (`🐍️.py:31`) and `from manim import ...` (`🐍️.py:32`). Pre-existing, not added today (diff of the Python file adds only the viewpoint tuple and one docstring).
- External programs: CLI reaches `rsvg-convert`, `ffmpeg`, `uv` only through `ToolPort` (`🔨️modules/📤️export/🎬️cli/🟦️.ts:42-56`). Compliant. Exception, pre-existing: `🔨️modules/🎨️depiction/🖨️preview/🟦️.ts:43` calls `execFileSync("rsvg-convert", ...)` and `:46` `execFileSync("python", ["-c", PYMUPDF, ...])` directly, bypassing the interface.
- Root `package.json` (outside S) adds `assimpjs` (line 193) and `gltf-validator` (line 249), both in `devDependencies`. `assimpjs` is imported by no test (`grep` over `🧪️tests`): unused.

Test oracles per new test (folders under `🧪️tests/`, untracked unless noted):

| Test | Third-party oracle | Feature | JSON fixture |
|---|---|---|---|
| `🧊️axonometric-smoke` | none (vitest only) | none | none. VIOLATION: new feature with neither a language-agnostic test nor a third-party oracle |
| `🧊️model-bounds` | three GLTFLoader and Box3 (via `🧰️support/🧊️.ts`) | yes | sample figure |
| `🧊️model-catalog` | gltf-validator, three FBXLoader and GLTFLoader (via support) | yes | none |
| `🧊️model-exports` | ajv (kernel schema), fflate (unzip), gltf-validator, three loaders (via support) | yes | none |
| `🧊️model-frame` | three Box3, Matrix4, Quaternion, Vector3 | yes | `🧊️model-frame` |
| `🧊️model-looks` | three Color, Quaternion, SRGBColorSpace, Vector3 | yes | none |
| `🧊️model-skeleton` | three AnimationMixer, Matrix4, Vector3 | yes | `🧊️model-skeleton` |
| `🧊️solid-sweep` | ajv, gl-matrix | yes | `🧊️solid-sweep` |
| `🧊️volume-schema` | ajv, robust-predicates (exact predicates), shapely via `uv` (`🐍️.py`) | yes | `🧊️volume-schema` |
| `🎯️targets/⚛️react/🧪️tests/🧊️model-view` | not inspected | none in folder | none |

Changed existing tests with oracles: `🧪️tests/🔺️polygon-union` (polygon-clipping, shapely via `uv`).

Findings:
- VIOLATION: `🧪️tests/🧊️volume-schema/🟦️.ts` imports `robust-predicates`, which is declared in neither the root `package.json` nor the library `package.json`. Undeclared test dependency (it resolves only transitively, if at all).
- Features with no language-agnostic test: `🔨️modules/🎨️depiction/🧊️axonometric/` (its `🌗️shading`, `📐️visibility`, `🖇️stitch`, `🧭️edges`, `🧱️mesh` subfolders) has no dedicated test; only `🧊️axonometric-smoke` (no oracle, no feature) and the older `🧊️axonometric-projection` (`three`, pre-existing) and `🧊️axonometric-visibility` (pre-existing) touch it.
- Volume modules without a dedicated test: `🧊️volume/🌡️thermal`, `🏗️structure`, `🪧️glyphs`, `🎞️clips`, `⏱️job`, `🚫️refusal`. Only indirect coverage through the `model-*` tests.
- Design-required artefacts not built (design.md Tests section): per-view silhouette agreement against the authored shapes (no test in the `model-*` set); GLB and FBX structure-digest vectors under `🧫️fixtures/` (only `🧊️model-frame`, `🧊️model-skeleton`, `🧊️solid-sweep`, `🧊️volume-schema` fixtures exist). The per-solid-kind kernel vectors live in the framework, outside S, not checked.

## 3. Leftovers

- VIOLATION, `[DEBUG]` logs (new files, untracked):
  - `🎯️targets/⚛️react/🧪️tests/🧊️model-view/🟦️.tsx:10` `console.log("[DEBUG] axo", ...)`
  - `🎯️targets/⚛️react/🧪️tests/🧊️model-view/🟦️.tsx:14` `console.log("[DEBUG] axo", view, ...)`
  - `🧪️tests/🧊️axonometric-smoke/🟦️.ts:12` `console.log(`[DEBUG] resolveDepiction ...`)`
  - `🧪️tests/🧊️axonometric-smoke/🟦️.ts:15` `console.log(`[DEBUG] planar elevation ...`)`
- Not leftovers (product CLI output): `🔨️modules/🗂️catalog/🩺️check/🟦️.ts:157,166,172` `console.log`; `🔨️modules/🎨️depiction/🖨️preview/🟦️.ts:209` `console.log(path)`; `process.stderr.write` progress in `🎬️cli/🟦️.ts:336`.
- TODO, FIXME, XXX, HACK: none found in the changed set.
- `.only`: none. `.skip`/`skipIf`: none added today (grep over added diff lines). Pre-existing `it.skipIf` in `🎯️targets/⚛️react/🧪️tests/🧪️playground-compose/🟦️.tsx:64` and `🧪️tests/🎭️cast-export/🟦️.ts:358`.
- Commented-out code: none found.
- Scratch files inside S: none. No `*scratch*`, `*tmp*`, `*.log`, `*.jsonl`, `*.bak`, `*.orig`, `*migrat*` in S. No `🗑️generated` folder in S. Only ignored content: three `node_modules` folders.

## 4. Legacy, compatibility, migration constructs

VolumeUnavailableError (the "not available yet" refusal). Current state, all sites:
- Definition `🔨️modules/🔗️contracts/🟦️.ts:18`; message `:20` `is not available yet (${site}): the 3D volume pipeline is not built`; bilingual `reason` at `:26`.
- The only throw is `🔨️modules/🔗️contracts/🟦️.ts:35` inside `requirePlanarViewpoint` (`:34`). `requirePlanarViewpoint` has no callers (grep). The refusal is therefore dead code, and it names a feature that is now built (the axonometric depiction exists).
- Catch sites that can never fire: `🎯️targets/⚛️react/🔨️modules/🧩️element/🟦️.ts:226`, `🧍️figure/🟦️.tsx:133`, `📤️downloads/🟦️.ts:268`. Import only: `🔭️viewpoints/🟦️.ts:10`.
- Test that keeps it alive: `🧪️tests/🧊️volume-schema/🟦️.ts:16` import, `:226-229` constructs `new VolumeUnavailableError("glb", "test")` and asserts the text "not available yet" / "noch nicht verfügbar". A legacy refusal kept only for its wording.
- Python consumer: no `VolumeUnavailableError` (the ticket's `:232/:718/:730` sites are gone).
- Other refusal sites found: none for axonometric. `depict`, `modelScene`, `cameraOf`, `depthOf`, `dimensionLine`, `--viewpoint`, cast `authoredRepresentation`, `isViewpoint`, `metadataText` no longer refuse. Remaining "not built" refusal of a model: `🧊️volume/🟦️.ts:73` `ModelRequestError("model-no-volume")`, unreachable for a schema-valid asset (see below).

Defaults standing in for required data (added today, in the volume module):
- `🔨️modules/🧊️volume/🟦️.ts:277` `const bone = asset.rig.bones.find(...)?.title ?? { en: surface.bone, de: surface.bone };` The bone `title` is required (`Text`), so the fallback hides missing data and shows an id in place of a name.
- Dead guards for a now-required `volume`: `🧊️volume/🟦️.ts:73` `if (volume === undefined) throw ...`; `🎨️depiction/🧊️axonometric/🟦️.ts:105` `if (asset.volume === undefined) throw new ViewMissingError(...)`; `📤️export/🎭️cast/🟦️.ts:81`; `📤️export/🗜️zip/📦️models/🟦️.ts:23`.
- Optional-option defaults, not violations: `🧊️volume/🟦️.ts:76` `options.theme ?? "light"`, `:106` `thermalQuantity ?? "surface-temperature"`, `:107` `forceQuantity ?? "axial"`, `:75` `views ?? VIEWS`, `:71` `operation = silentOperation`. `🧷️frames/🟦️.ts:126` `glyphs.get(root) ?? { scale: HIDDEN }` silently hides a glyph that has no state (interpretation: a default for missing data).
- Pre-existing, not new: `🔨️modules/🧮️evaluation/🟦️.ts:569-570` `fallback` midpoint for NaN depths (present at HEAD).
- `sweep`: no default (`SolidSweepError` at `🧮️evaluation/🟦️.ts`, refuses anything but `v` or `w`). Compliant.

Version shims and deprecation markers: no `@deprecated`, no `legacy`, no `compat`, no migration code in S. Version labels are inconsistent (see section 10).

Migration scripts inside S: none.

## 5. i18n

- Bilingual refusals: `🔨️modules/🧊️volume/🚫️refusal/🟦️.ts`, all 13 `MODEL_REFUSALS` entries have `en` and `de`. Compliant.
- Schema `Text` (`🧬️schema/🔣️.json` `$defs.Text`): `required: ["en","de"]`, `additionalProperties: false`, no default language. Compliant. Twin `Text` in `🧬️schema/🟦️.ts` follows.
- React i18n bundles: key parity en/de in all nine files (`🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔠️english` vs `🔡️german`): 0 mismatches (for example `🧱️core.ts` 116/116, `📖️detail.ts` 239/239, `🧊️model.ts` 67/67, `🔭️viewpoints.ts` 36/36).
- VIOLATIONS, new English-only user-facing strings (CLI output and usage; added lines in `git diff`):
  - `🔨️modules/📤️export/🎬️cli/🟦️.ts:77` and `:79` (`USAGE`, `USAGE_MODEL`, English only).
  - `🎬️cli/🟦️.ts:109` `--span-k applies to the 3D model formats only`; `:120` `--views applies to the sheet formats only`; `:124` `--camera applies to the axonometric view only`; `:154` `--views must ...`; `:139` axonometric camera parse error.
  - `🔨️modules/🎨️depiction/🔭️views/🟦️.ts:70-71` `RangeError` messages for the axonometric camera (English only).
  - `🔨️modules/🎨️depiction/🖨️preview/🟦️.ts:53` `USAGE`, `:151`, `:153`, `:188`, `:193` (English only).
  - `🔨️modules/📤️export/🎭️cast/🟦️.ts:81-82` `no volume for the axonometric viewpoint` and `no ... geometry` (English only).
  - Caveat (interpretation): these are CLI-developer messages; the rule may not cover them. The same pattern predates today (for example `🎬️cli/🟦️.ts:87-93`).
- Internal errors English only, not user facing: `🧊️volume/🦴️skeleton/🟦️.ts:63`, `🎨️depiction/🧊️axonometric/🟦️.ts:104`.
- Catalog `volume.decisions`, all 152 assets: 447 entries in total, 1 to 6 per asset. All 447 are objects with non-empty `en` and `de`. 0 violations. 0 entries with `de` identical to `en`. Every asset has at least one entry.

## 6. Progress and cancellation

- `modelOf` (`🔨️modules/🧊️volume/🟦️.ts:71`): accepts `operation: Operation = silentOperation`. Honours it:
  - `:126` `if (operation.cancelled()) throw new CancelledError();`
  - `:127` `operation.report(Math.min(1, done / total));`
  - `:129` `if (operation.cancelled()) throw new CancelledError();`
  - `:259` `operation.report(1);`
  - Yields to the event loop between frames (`:60` `yieldToLoop`, `:128`). Granularity gap: inside one frame, `evaluate`, `solveThermalAt`, `solveStructure` and `solveTransport` are synchronous and cannot be interrupted.
- Exporters, all accept `control: JobControl = {}` and wrap work in `cancellable` + `jobOf`:
  - GLB `📤️export/🧊️glb/🟦️.ts:21-33` (`modelOf` with `job.operation(0)`, `tessellate` with `(1)`, `featureEdges` with `(2)`, `encodeGlb` with `(3)`). `encodeGlb` is synchronous, so cancellation is checked only between phases.
  - FBX `📤️export/🎬️fbx/🟦️.ts:19-32` (same pattern; `encodeFbxDetailed` with `job.operation(2)`).
  - Model JSON `📤️export/🗿️model/🟦️.ts:35-37`.
  - Bundle `📤️export/🗜️zip/📦️models/🟦️.ts:31` `...(control.signal ? { signal: control.signal } : {})`, `:35` `control.signal?.throwIfAborted();`, `:32` progress, `:48` final progress.
  - `🧊️volume/⏱️job/🟦️.ts:26` `cancelled: () => control.signal?.aborted === true`; `:37` maps `CancelledError` and `FbxCancelledError` to the signal reason.
  - Test coverage: `🧪️tests/🧊️model-exports/🟦️.ts:110-122` checks monotonic progress and cancellation with an `AbortError`.
- CLI batch paths: `runExport` (`🎬️cli/🟦️.ts:246`):
  - Honours: `:261` `control.signal?.throwIfAborted();` inside the frame loop; `:210` in `rasterise`; `:301` and `:317` pass `control.signal` to external tools.
  - Gap: the emit loops do not check the signal between files: `:288` `for (const [index, scene] of scenes.entries()) await emit(..., writeSvg(scene));` (no `control`, no check); `:292` `writePdf(scenes, control)`; `:296` `writeDxf(modelOf(index), noteOf(index))` per frame, no check; `:302` `writeDwg(modelOf(index), control)` per frame, no check.
  - `runModelExport` (`:222`): passes `control` to `writeGlb`, `writeFbx`, `writeModel`, `writeModelBundle` (`:230-236`). Honours cancellation through them.

## 7. Catalog data sanity (152 assets)

Read-only sweep of `🗂️catalog/*/*/🔣️.json`:

- Solids: 4,903 in total. Per asset min 3, median 27.5, max 98.
  - 10 smallest: `glulam-beam` 3, `solid-brick-wall-1960` 4, `room-thermostat` 5, `thermostatic-radiator-valve` 5, `etics-brick-wall-12` 5, `etics-brick-wall-30` 5, `pendant-lamp` 6, `tealight` 6, `kitchen-extractor-hood` 7, `split-ac-indoor-unit` 7.
  - 10 largest: `tower-crane` 98, `weeping-willow` 95, `cyclist` 87, `person-with-rollator` 84, `parent-with-stroller` 80, `city-bus` 78, `small-leaved-lime` 76, `vintage-car-1929` 75, `tram` 75, `concrete-mixer-truck` 73.
- Solids per kind: loft 1,373; capsule 1,125; box 873; revolve 628; ellipsoid 536; extrude 368.
- `repeat`: 654 solids in 95 assets. Largest count 70 (`spiral-stair`, `handrail`), then 46 (`finned-heat-exchanger`, `fin`), 40 (`motor-scooter`, `r-tyre`; `f-tyre` also 40).
- `bone` not a rig bone id (including `weights`): 0.
- `material` not an asset material id: 0.
- `region`/`regions` not in `thermal.regions`: 0.
- Duplicate solid ids within an asset: 0.
- Non-kebab-case solid ids: 0.
- Non-finite numbers: 0. Any numeric leaf with |x| > 60 m (excluding `maxDenominator`, `tone`, `count`): 0. Radius, size or `r0`/`r1`/`rx`/`ry` ≤ 0: 0.
- `maxDenominator` values: 50 (825 solids), 100 (578), 200 (513). None is 500, none is outside the set. Compliant.
- Loft sections: 12,543 in total, without `rotation`: 0.
- Assets whose every solid is bound to the root bone while the rig has more than 3 bones: none.
- Moving-clip coverage (a clip whose bone keys vary, or `rootMotion` true; this criterion makes all 152 assets "moving", since 359 clips carry `rootMotion`): fewer than 30 % of non-root bones carry a solid in 3 assets:
  - `glulam-beam`: 2 bones, the single non-root bone carries no solid (all solids on root).
  - `stair-flight`: same pattern, 2 bones.
  - `steel-column`: same pattern, 2 bones.
  - Outside the ">3 bones" rule (inferred: possible missing articulation only if the rig is meant to articulate).

## 8. Schema-first consistency

- `$defs` in `🧬️schema/🔣️.json`: 83. Every name has a same-named export in `🧬️schema/🟦️.ts`. The new `$defs` (`Vector2`, `Vector3`, `Quaternion`, `VolumeTransform`, `VolumeRepeat`, `VolumeSolid`, `Volume`) all have twin types. Compliant.
- Twin exports with no schema `$def`: `PlanarViewpoint` (design-sanctioned), `VolumeSolidBase`, `VolumeSolidKind` (new helpers, not schema-first), plus pre-existing `CircuitRow`, `HydronicCircuit`, `Language`, `MemberShape`, `PhaseTransition`, `RepresentationKey`, `UnavailableCause`, `WaterCoupling`.
- `ExportFormat`: schema `$defs.ExportFormat` = `[svg, pdf, dxf, dwg, png-sequence, mp4, webm, component, glb, fbx, model]`. Twin `🧬️schema/🟦️.ts:62` `EXPORT_FORMATS` is identical. Mismatches:
  - VIOLATION: React `🎯️targets/⚛️react/🔨️modules/📤️downloads/🟦️.ts:20` `ExportFormat = ... | webm | glb | fbx | model | model-zip`. No `component`, plus `model-zip` not in the schema.
  - VIOLATION: `🎯️targets/⚛️react/🔨️modules/🔬️detail/📤️export-panel/🟦️.tsx:21` `FORMATS` = `[svg ... model, model-zip]`, no `component`.
  - CLI `🔨️modules/📤️export/🎬️cli/🟦️.ts:64` `CLI_FORMATS` adds `sheet-svg, sheet-pdf, sheet-dxf, sheet-dwg, model-zip` (CLI-only, not in the schema); `:66` `MODEL_FORMATS` = `[glb, fbx, model, model-zip]`.
  - Python consumer: no export-format list.
- `Viewpoint`: schema `$defs.Viewpoint` = `[elevation, front, plan, axonometric]`; twin `🧬️schema/🟦️.ts:50` `VIEWPOINTS` identical; `🧬️schema/🟦️.ts:53` type; `🔨️modules/🔗️contracts/🟦️.ts` imports it (no own list); cast schema `🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json` (`Index.viewpoint`, `Cast.viewpoint`, `Provenance.view.viewpoint`) = `[elevation, front, plan, axonometric]`; Python `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py:39` `_VIEWPOINTS = ("elevation", "front", "plan", "axonometric")`. Identical. Compliant.
- `PLANAR_VIEWPOINTS` (`🧬️schema/🟦️.ts:56`) = `[elevation, front, plan]` is a twin-only subset. Design-sanctioned.

## 9. Path length

- Changed and new files: longest absolute path 125 characters (`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🔭️viewpoints/🪝️preference/🟦️.tsx`). Longest repo-relative path 97 characters (same file). In UTF-8 bytes the longest relative path is 141 bytes.
- Files over 200 characters: 0 (absolute or relative). Compliant with 200 and 256.

## 10. README and documentation drift (`S/README.md`)

- VIOLATION (false): `README.md:121`: "until its depiction exists, asking for `axonometric` throws `VolumeUnavailableError` (`🔨️modules/🔗️contracts`); `--viewpoint` and cast manifests refuse it the same way." The depiction exists (`🔨️modules/🎨️depiction/🧊️axonometric/🟦️.ts`, `🎨️depiction/🟦️.ts:157` `axonometricCamera`), the CLI draws it (`🎬️cli/🟦️.ts:123`), and the cast accepts it (`🎭️cast/🟦️.ts:80-81`).
- VIOLATION (false): `README.md:115`: "`profile` [axial, radial] (open, simple, radial ≥ 0)". The validator accepts closed loops (design Amendment A4): `🔨️modules/🗂️catalog/🧊️volume/🟦️.ts:220` `profileIssues(loop ? solid.profile.slice(0, -1) : solid.profile, ..., loop, ...)`, and the catalog uses one (`concrete-mixer-truck`, `drum-cone-cap`).
- VIOLATION (inconsistent): `README.md:15` diagram `modelOf(volume, options)`, while `README.md:125` and the code say `modelOf(asset, options, operation)` (`🧊️volume/🟦️.ts:71`).
- Drift: `README.md:119` names "ajv, shapely and robust-predicates oracles"; `robust-predicates` is not declared (section 2).
- Drift: `README.md:121` says `ExportFormat` gains `glb`, `fbx`, `model`. The React target has its own `ExportFormat` with `model-zip` and without `component` (section 8). The README does not document this.
- Schema version: `README.md:26` and twin `🧬️schema/🟦️.ts:35` `SCHEMA_VERSION = "semio.staffage/v2"` agree, and the asset files carry `version: "semio.staffage/v2"`. Compliant.
- CLI flags: `README.md:130-134` (`--format glb|fbx|model|model-zip`, `--pose`, `--clip`, `--view` default `light-solid`, `--environment`, `--span-k`, `--lang`) agree with `USAGE_MODEL` (`🎬️cli/🟦️.ts:79`) and `parseArguments` (`:107`). Compliant.
- Version labels inconsistent (not documented): `🔨️modules/📤️export/🖼️svg/🟦️.ts:16` `NAMESPACE = "https://semio.tech/staffage/v2"`, but React `📤️downloads/🟦️.ts:143` writes `xmlns:staffage="https://semio.tech/staffage/v1"`; `🧊️volume/🟦️.ts:264` `generator: "staffage volume/v1"`.
- Ticket doc stale: `staffage-volume-schema.md` ("Not available yet" table, Python refusals, "no catalog asset was touched") no longer matches the code. Not a README item, noted for the coordinator.

---

## Prioritised findings (15 most important)

1. `[DEBUG]` logs in new files: `🧪️tests/🧊️axonometric-smoke/🟦️.ts:12,15` and `🎯️targets/⚛️react/🧪️tests/🧊️model-view/🟦️.tsx:10,14`.
2. Dead legacy refusal `VolumeUnavailableError`: `🔨️modules/🔗️contracts/🟦️.ts:18,35` (only throw, in uncalled `requirePlanarViewpoint`), with never-firing catch sites `🎯️…/🧩️element/🟦️.ts:226`, `🧍️figure/🟦️.tsx:133`, `📤️downloads/🟦️.ts:268`, and kept alive by `🧪️tests/🧊️volume-schema/🟦️.ts:226-229`.
3. README claims a refusal that no longer exists: `README.md:121` ("asking for `axonometric` throws `VolumeUnavailableError`").
4. README contradicts code for revolve: `README.md:115` "open" versus closed loops accepted at `🗂️catalog/🧊️volume/🟦️.ts:220` (A4).
5. Schema-first violation, `ExportFormat`: React `📤️downloads/🟦️.ts:20` and `🔬️detail/📤️export-panel/🟦️.tsx:21` omit `component` and add `model-zip`; schema `$defs.ExportFormat` (twin `🧬️schema/🟦️.ts:62`) has no `model-zip`; CLI `🎬️cli/🟦️.ts:64` adds `sheet-*` and `model-zip`.
6. Undeclared test dependency `robust-predicates` (`🧪️tests/🧊️volume-schema/🟦️.ts:12` import, used at `:260`), absent from `package.json`; `assimpjs` declared at root `package.json:193` but never imported.
7. New feature with no oracle, no feature file and no fixture: `🧪️tests/🧊️axonometric-smoke/🟦️.ts` (depiction under `🎨️depiction/🧊️axonometric/`).
8. Cancellation not honoured between files in CLI batch emits: `🎬️cli/🟦️.ts:288, 292, 296, 302` (only `:261` checks the signal, and only in the evaluation loop).
9. Design-required tests absent: per-view silhouette agreement with authored shapes, and GLB/FBX structure-digest vectors in `🧫️fixtures/` (design.md "Tests").
10. Defaults masking required data: `🔨️modules/🧊️volume/🟦️.ts:277` bone title falls back to the bone id, although `title` is required.
11. Dead guards for the now-required `volume`: `🧊️volume/🟦️.ts:73`, `🎨️depiction/🧊️axonometric/🟦️.ts:105`, `🎭️cast/🟦️.ts:81`, `🗜️zip/📦️models/🟦️.ts:23`.
12. English-only new CLI and depiction user messages: `🎬️cli/🟦️.ts:77,79,109,120,124,154`, `🔭️views/🟦️.ts:70-71`, `🖨️preview/🟦️.ts:53,151,153,188,193`, `🎭️cast/🟦️.ts:81-82` (interpretation: may not count as user-facing).
13. Version label drift: SVG namespace `staffage/v2` (`🖼️svg/🟦️.ts:16`) against React `staffage/v1` (`📤️downloads/🟦️.ts:143`), and `generator: "staffage volume/v1"` (`🧊️volume/🟦️.ts:264`).
14. README signature drift: `README.md:15` `modelOf(volume, options)` against `modelOf(asset, options, operation)`.
15. Catalog articulation: `glulam-beam`, `stair-flight`, `steel-column` (2-bone rigs, all solids on the root bone, the one non-root bone carries none) (inferred: confirm the rigs are meant to be rigid).

Also noted, not in the top 15: emoji reuse across docstrings (204 distinct over 1,122); `🧬️schema/🟦️.ts:550` docstring without emoji (pre-existing); pre-existing runtime libraries `react`, `react-dom` (React package) and `numpy`, `manim` (Python consumer); pre-existing direct `rsvg-convert` and `python -c` calls in `🖨️preview/🟦️.ts:43,46`.
