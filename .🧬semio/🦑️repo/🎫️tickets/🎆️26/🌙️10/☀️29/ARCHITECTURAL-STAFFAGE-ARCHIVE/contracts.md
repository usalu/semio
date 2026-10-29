# Staffage Archive — Shared Contracts (owner: Opus coordinator)

Root: `🎓️teaching/🏛️architecture/🧍️staffage/` (abbrev. `S/`). Keep every absolute path < 200 chars.

## Layout and ownership

| Path | Content | Owner |
|---|---|---|
| `S/README.md` | domain model, conventions, run instructions | coordinator |
| `S/🧬️schema/🔣️.json` | JSON Schema draft-07 `semio.staffage/v1`, the normative contract | core worker |
| `S/🧬️schema/🟦️.ts` | TS twin of the schema | **coordinator only** |
| `S/🔨️modules/🔗️contracts/🟦️.ts` | runtime contracts between modules | **coordinator only** |
| `S/🔨️modules/📐️units/🟦️.ts` | SI, scales, paper conversion, line weights, LOD | core worker |
| `S/🔨️modules/🦴️rig/🟦️.ts` | forward kinematics, joint limits | core worker |
| `S/🔨️modules/🎞️animation/🟦️.ts` | easing, key sampling, tracks, contacts, transitions, root plant | core worker |
| `S/🔨️modules/🧮️evaluation/🟦️.ts` | `evaluate(asset, query)` → `EvaluatedState`; shape flattening; path parser | core worker |
| `S/🔨️modules/🗂️catalog/🟦️.ts` | catalog loading + semantic validation (`validateAsset`, `validateManifest`) | core worker |
| `S/🔨️modules/🌡️thermal/🟦️.ts` | `solveThermal(state, environment?)` → `ThermalField` | thermal worker |
| `S/🔨️modules/🏗️structure/🟦️.ts` | `solveStructure(state)` → `StructuralResult` | structure worker |
| `S/🔨️modules/🎨️depiction/🟦️.ts` | `depict(state, options, analysis)` → `DrawingScene`; `modelScene(state)` → `ModelScene` | depiction worker |
| `S/🔨️modules/📤️export/<format>/🟦️.ts` | `🖼️svg` `writeSvg`, `📕️pdf` `writePdf`, `📐️dxf` `writeDxf`, `🖊️dwg` `writeDwg`, `🎥️video`, `🎞️frames` | export worker (svg: depiction worker) |
| `S/🗂️catalog/🔣️.json` | manifest | coordinator / manifest helper |
| `S/🗂️catalog/<emoji><category>/🔣️.json` | `CategoryDocument` per category | one content worker per category |
| `S/📦️packages/🟦️typescript/` | library `@teaching/staffage` (barrel `🟦️.ts`, `📜️script.ts`, `📋️project.json`) | scaffold worker |
| `S/🎯️targets/⚛️react/📦️packages/🟦️typescript/` | `@teaching/staffage-react`: `StaffageFigure`, custom element, the archive app | website worker |
| `S/🌐️site/` (+ `📦️packages/🟦️typescript`) | `@teaching/staffage-site`: html, entry, css, Vite builder, dev/build targets | scaffold + website worker |
| `S/🧪️tests/<emoji-case>/` | `🟦️.ts` (vitest), `🥒️.feature` (language-agnostic), `🐍️.py` oracle | owner of the tested module |
| `S/🧫️fixtures/<case>/🔣️.json` | language-agnostic vectors | owner of the tested module |

Category folders: `👤️humans`, `🌳️vegetation`, `🐕️animals`, `🪑️objects`, `🚜️vehicles`, `🧱️components`.

## Conventions (normative)

- SI internally. World: metres, x right, y up, ground y = 0. Angles authored in degrees CCW. Temperatures authored °C, solved in K.
- Bone local frame: origin at joint, +x along the bone. Rest transform (x, y, angleDeg) in parent frame. Pose offsets add to rest.
- Paper: mm, y down. `mmPerMetre = 1000 / denominator`. A 1.80 m person is 3.6 / 9 / 18 / 36 mm at 1:500/200/100/50.
- Viewport zoom is a separate CSS transform on top of paper mm (1 mm = 96/25.4 px at 100 %). It never changes the scene.
- LOD: denominator > 200 → 0; > 100 → 1; > 50 → 2; ≤ 50 → 3. Shapes with `lod` > current level are omitted.
- Line weights (ISO 128 pens, mm) per role and preset; custom scales snap to the nearest preset by log-distance:

| role | 1:500 | 1:200 | 1:100 | 1:50 |
|---|---|---|---|---|
| body | 0.18 | 0.25 | 0.35 | 0.50 |
| detail / glazing | 0.13 | 0.13 | 0.18 | 0.25 |
| texture | – | 0.13 | 0.13 | 0.13 |
| ground | 0.25 | 0.35 | 0.50 | 0.70 |

- Annotation text 2.5 mm (labels 1.8 mm), constant in paper space.
- Time: seconds; frame n at t = n / fps; looping clips wrap t mod duration; non-looping clips clamp.
- Root motion `plant`: during a contact span the anchor's world x is held at the value it had when the span began; root x is solved per
  instant by a deterministic integration at 240 Hz from t = 0; vertical ground uses `ground: "auto"` (lowest active contact on y = 0).
- Structural sign conventions: axial N > 0 tension, < 0 compression. Bending moment M > 0 sagging (tension at the bone's local −y face).
  Joint moment = moment of all external loads on the child subtree about the joint, CCW positive (N·m). Reactions world axes.
  Colours: compression warm (#d7301f → #fdae61), tension cool (#2c7bb6 → #abd9e9); zero neutral grey. Hatch patterns `compression`
  (diagonal) and `tension` (dots) make the reading colour-independent.
- Thermal palette: same warm/cool ramp, centred on air temperature for surface temperature, on 0 for fluxes; legends always name the
  quantity, unit (°C, W/m²) and centre. Streamline/particle legend states whether parcels carry airflow or water vapour.
- All UI text and legends are `{ en, de }`; no default language.
- Every expensive operation accepts `JobControl` (progress + AbortSignal).

## Module APIs

```ts
// 📐️units
export const SCALE_PRESETS: readonly [500, 200, 100, 50];
export function mmPerMetre(scale: DrawingScale): number;
export function lodFor(scale: DrawingScale): Lod;
export function lineWeightMm(scale: DrawingScale, role: ShapeRole): number | null;
// 🦴️rig
export function solveRig(rig: Rig, offsets: Record<Slug, BoneOffset>, root: { x: number; y: number; angleDeg: number }): Record<Slug, Affine>;
// 🎞️animation
export function sampleClip(asset: Asset, clip: Clip, timeS: number): Sample; // offsets, root, contacts, thermal, loadScales, phase
// 🧮️evaluation
export function evaluate(asset: Asset, query: EvaluationQuery): EvaluatedState;
// 🗂️catalog
export function validateAsset(asset: Asset): readonly Issue[]; export function validateManifest(manifest: Manifest, assets: readonly Asset[]): readonly Issue[];
// 🌡️thermal
export function solveThermal(state: EvaluatedState, environment?: Environment): ThermalField;
// 🏗️structure
export function solveStructure(state: EvaluatedState): StructuralResult;
// 🎨️depiction
export function depict(state: EvaluatedState, options: DepictionOptions, analysis?: { thermal?: ThermalField; structure?: StructuralResult }): DrawingScene;
export function modelScene(state: EvaluatedState, options: { viewpoint: Viewpoint; analysis?: ... }): ModelScene;
// 📤️export
export function writeSvg(scene: DrawingScene): string;                // width/height in mm, viewBox in mm, <g> per layer
export function writePdf(pages: readonly DrawingScene[], control?: JobControl): Uint8Array; // vector, page = scene size, 1 mm = 72/25.4 pt
export function writeDxf(scene: ModelScene): string;                 // AC1015+, $INSUNITS=6 (m), layers with ACI and lineweight
export function writeDwg(scene: ModelScene): Promise<Uint8Array>;   // genuine AC1024 through the in-repo Rust writer (WASM/CLI)
```

## Asset content requirements (every asset)

≥ 3 poses (or configurations / load states), ≥ 2 clips, rig appropriate to category, elevation geometry with LOD levels, materials,
thermal regions bound by shape `region`, ≥ 1 emitter where physically meaningful (breath, exhaust, transpiration, lamp plume), masses on
bones, supports/contacts, applied loads where meaningful, provenance with references, `source` on every physical parameter.

## Manifest (48)

Humans 12: walking-commuter, seated-reader, running-child, wheelchair-user, elderly-with-cane, cyclist, worker-carrying-box,
parent-with-stroller, jogger, person-with-umbrella, leaning-person, tour-guide.
Vegetation 8: english-oak, lombardy-poplar, multi-stem-birch, scots-pine, date-palm, weeping-willow, hornbeam-hedge, miscanthus-grass.
Animals 6: labrador-dog, domestic-cat, horse, pigeon, roe-deer, dairy-cow.
Objects 8: park-bench, cafe-chair, office-swivel-chair, height-adjustable-desk, street-lamp, market-parasol, waste-bin, playground-swing.
Vehicles/machines 6: city-car, city-bus, excavator, tower-crane, forklift, cargo-bike.
Components 8: cantilever-balcony, glulam-beam, steel-column, stair-flight, sliding-door, tilt-window, glass-balustrade, tied-canopy.

## Verification gates

G1 contracts + scaffold run (`repo run @teaching/staffage:test` green). G2 engine reference tests (scale sizes, determinism, plant,
cantilever, thermal cases) green with third-party oracles. G3 all 48 assets validate. G4 site builds, browser inspection of each
category × 4 scales × 7 views. G5 exports reopened (qpdf/pdftoppm, rsvg, DXF reader, DWG reader, ffprobe). G6 audits.
