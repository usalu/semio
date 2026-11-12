# 🧍️ Staffage Internals: Code Map for the 3D Model Pipeline

Read-only study. Root S = `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage`. Paths are relative to S unless absolute.

**Snapshot caveat.** The tree is edited live by the ticket's own agents. Line numbers below were read at study time and some have drifted since. Verified just before writing: `🧬️schema/🟦️.ts:50` now reads `VIEWPOINTS = ["elevation", "front", "plan", "axonometric"]` and `PLANAR_VIEWPOINTS` was added at `:56`. `🔗️contracts/🟦️.ts` `EvaluatedState` is now at `:104` (was 76). Re-grep before relying on any line number. Items marked (inferred) were not read directly.

---

## 1. Evaluation, rig, animation

### 1.1 `evaluate` and the query

- `🔨️modules/🧮️evaluation/🟦️.ts:858`
  `export function evaluate(asset: Asset, query: EvaluationQuery, selector?: RepresentationSelector): EvaluatedState {`
- `EvaluationQuery`, `🔨️modules/🔗️contracts/🟦️.ts:43`:
  `export type EvaluationQuery = { readonly kind: "pose"; readonly pose: Slug; readonly timeS?: number } | { readonly kind: "clip"; readonly clip: Slug; readonly timeS: number } | { readonly kind: "transition"; readonly from: Slug; readonly to: Slug; readonly durationS: number; readonly timeS: number };`
- Constants: `KINEMATIC_STEP_S = 1 / PLANT_HZ` (`evaluation:33`), `PLANT_HZ = 240` (`animation:18`).

### 1.2 Geometric types (`🔗️contracts/🟦️.ts`)

- `Affine` `:68`: `readonly [number, number, number, number, number, number]`
- `EvaluatedBone` `:71`, `Point` `:74`, `Vec3` `:77`, `BoneFrame` `:80`: `BoneFrame = { o: Vec3; u: Vec3; v: Vec3; w: Vec3 }`

### 1.3 `EvaluatedState` (`contracts:104`)

| Field | Type | Meaning |
|---|---|---|
| `asset` | `Asset` | source asset |
| `query` | `EvaluationQuery` | the query evaluated |
| `timeS` | `number` | absolute query time (never wrapped) |
| `clipTimeS` | `number` | wrapped (loop) or clamped clip phase; `0` for poses |
| `frame` | `number` | `frameAt(clipTimeS, fps)` |
| `phase` | `Slug \| null` | phase id at the instant |
| `bones` | `Record<Slug, EvaluatedBone>` | per-bone state |
| `shapes` | `Record<Viewpoint, EvaluatedShape[]>` | placed shapes per viewpoint |
| `root` | `{x, y, z, angleDeg}` | `z` is always 0; `angleDeg` is the sagittal root rotation |
| `anchors` | `Record<Slug, {anchor, at: Point, z: number}>` | `at` is the 2D projected point; `z` is the 3D lateral (left +) world coordinate |
| `contacts` | `Slug[]` | active contacts |
| `loads` | `WorldForce[]` | applied loads |
| `thermal` | `ThermalInputs` | thermal inputs at the instant |
| `respiration` | `RespirationState \| null` | |
| `cycle` | `number` | `floor(t/D)` for looping clips, else 0 |
| `representation` | union | scale, symbol or none |
| `bounds` | `{minX, minY, maxX, maxY}` | 2D extent |
| `kinematics` | `{comVelocity: Vec3; segmentAccelerations: Record<Slug, Vec3>}` | m/s and m/s² |

Bone frames, precisely:

- `world: Affine` is a 2D elevation affine `[a,b,c,d,e,f]`, meaning `x' = a·x + c·y + e`, `y' = b·x + d·y + f`. Column 1 `(a,b)` is the projected `u`, column 2 `(c,d)` the projected `v`, `(e,f)` the projected origin. It is not a 4×4 matrix.
- `frame: BoneFrame` is the true 3D frame in world coordinates (x ahead, y up, z left), orthonormal `u`, `v`, `w`.
- `tip = o + u·lengthM`, a full `Vec3` including z.
- `origin: Point` is `apply(world, 0, 0)`.
- `kinematics.comVelocity` is a central difference over ±`KINEMATIC_STEP_S`. `segmentAccelerations` is a central second difference, with a one-sided formula at clip ends (inferred from the evaluation file's `kinematics` block).
- Discrepancy: the doc comment on `evaluate` says `timeS` is the wrapped clip time. The code returns the absolute query time. `clipTimeS` is the wrapped one. Decision D11 records the code's behaviour.

### 1.4 Rig (`🦴️rig/🟦️.ts`)

- World convention (contracts header): x ahead, y up, z left, ground y = 0.
- `placement(x, y, angleDeg): Affine` `:28`, returns `[cos, sin, -sin, cos, x, y]`.
- `compose(parent, local): Affine` `:34`.
- `effectiveOffset(bone, offset)` `:76`, realises offsets within joint limits.
  - Revolute: `angleDeg` clamped to `minDeg..maxDeg`; `abductionDeg` clamped to ±180; `twistDeg` clamped to `twistMinDeg ?? -180 .. twistMaxDeg ?? 180`.
  - Prismatic: `(dx, dy)` projected on `axisDeg`, scalar clamped to `minM..maxM`, realised translation `(s·cos, s·sin)`.
  - Fixed: all offsets ignored.
- `solveRig(rig, offsets, root): Record<Slug, Affine>` `:117`, 2D elevation affines. Uses the 3D chain when any bone is spread or twisted.
- `solveFrames(...)` `:176`, returns `null` when no bone is spread or twisted.
- `solveChain(rig, offsets, root): Record<Slug, SolvedFrame>` `:185`, always the 3D chain.
- `restFrames(rig)` `:190`.
- `carry(rest: Frame3, now: Frame3, point: Vec3): [number, number, number]` `:195`. Expresses `point − rest.o` in the rest axes and rebuilds it in the current frame.

3D chain composition, per bone with parent frame `P`, `φ = angleDeg + effective.angleDeg`, `α = abductionDeg`, `θ = twistDeg` (inferred from the chain block, read at study time):

1. Origin: `o = P.o + (x+dx)·P.u + (y+dy)·P.v + z·P.w`.
2. Sagittal about `P.w`: `u1 = cos φ·P.u + sin φ·P.v`, `v1 = −sin φ·P.u + cos φ·P.v`.
3. Spread about `v1`: `u2 = cos α·u1 + sin α·P.w`, `w2 = −sin α·u1 + cos α·P.w`. Positive α turns towards +z (left).
4. Twist about `u2`: `v = cos θ·v1 − sin θ·w2`, `w = sin θ·v1 + cos θ·w2`.
5. `tipZ = origin_z + u2_z·lengthM`.

Root base frame: `o = [root.x, root.y, 0]`, `u = [cos r, sin r, 0]`, `v = [−sin r, cos r, 0]`, `w = [0,0,1]`.

The 2D affine is the orthographic side projection of this chain, so all three views agree in every pose. Positive twist direction is documented in `README.md` ("bone pointing up turns counter-clockwise seen from above"). That claim is inferred from the algebra, not verified.

### 1.5 Animation (`🎞️animation/🟦️.ts`)

- `Sample` `:24`: `timeS`, `clipTimeS`, `frame`, `offsets: Record<Slug, FullOffset>`, `root`, `contacts`, `thermal`, `loadScales`, `loads`, `phase`.
- `ease(kind, t)` `:38`:
  `export function ease(kind: Ease, t: number): number` with cases `linear` (u), `smooth` (`u*u*(3-2*u)`), `in` (`u*u`), `out` (`1-(1-u)*(1-u)`), `step` (`u >= 1 ? 1 : 0`). `EASES` in `schema:428`.
- `wrapTime(timeS, durationS, loop)` `:50`. Non-loop clamps to `[0, D]`. Loop returns `t − floor(t/D)·D`, mapped to 0 if equal to D.
- `cycleOf(timeS, D, loop)` `:57`: `loop ? Math.floor(timeS / durationS) : 0`.
- `frameAt(timeS, fps)` `:62`: `Math.floor(timeS * fps + 1e-9)`.
- `sampleTrack(track, timeS, loop, durationS)` `:154`.
- Key location `locate` `:102`; `neighbours` `:133`; `hermite` `:144` (cubic Hermite with finite-difference tangents, used by `smooth`).
- Non-smooth segments blend with `weight = ease(keyA.ease, w)`. Thermal and load blends use the same weight. The module header says "smoothstep", which is only true for `smooth` (inferred from `core`).
- `sampleClip(asset, clip, timeS)` `:521`: wraps the time, builds the core sample, and applies root motion.
- `samplePose` `:535`, `transition(asset, from, to, durationS)` `:543` (synthetic two-key smooth clip, `rootMotion: "none"`).
- Root motion `rootMotion: "plant"`: `plantedX` integrates root x from planted contact anchors at `PLANT_HZ`. `groundedY` sets `y = −min(anchor.y)` over active contacts when `ground: "auto"`. Looping plant clips add `cycle · advanceM` to x.
- Observed: `abductionDeg` and `twistDeg` do not gain `cycle × turn` in the cycle block, unlike `angleDeg`, `dx`, `dy` and root (inferred from the code, not documented).

---

## 2. Existing `solid` hint (nearest 3D code)

### 2.1 Type and validation

- `🧬️schema/🟦️.ts:251`:
  `export type Solid = { readonly kind: "revolved"; readonly axisBone: Slug; readonly axis: "u" | "v" | "w"; readonly axisX: number; readonly axisZ?: number; readonly lateralScale?: number; readonly mirror?: boolean } | { readonly kind: "extruded"; readonly sweep: "v" | "w"; readonly z0: number; readonly z1: number; readonly zTip?: readonly [number, number] };`
- `Shape` `schema:256` carries `solid?: Solid`. A `member` shape must not set `solid`.
- Authoring validation: `🔨️modules/🗂️catalog/🟦️.ts` about lines 298–317 (inferred from the agent report): bone exists, `lateralScale > 0`, `axisBone` is the shape's bone or an ancestor (`solid-axis-not-ancestor`), `sweep` in `v|w`, `z0 < z1`, `zTip[0] < zTip[1]`.

### 2.2 Evaluation and projection

- `🔨️modules/🧮️evaluation/🟦️.ts`:
  - `:322` `UNION_CIRCLE_SEGMENTS = 32`.
  - `:450` `function solid3(solid: Solid, profile: readonly (readonly Point[])[], kin: Chain3, bone: Slug, viewpoint: Viewpoint, length: number): Point[][]`.
  - `:658` `function placeShape(...)`. Dispatch: identity, then member, then `if (kin !== undefined && shape.solid !== undefined) return { shape, rings: solid3(...), closed: true }`, then 2D affine for elevation, then `placeProjected`.
  - `:858` `evaluate` builds `kin = { now: solveChain(asset.rig, sample.offsets, sample.root), rest: restOf(asset) }`.
- Extruded: `lateral = sweep === "w"`; base `z` is `rest.o[2]` for lateral sweep. Span interpolates `z0..z1` along the bone to `zTip`. Each profile point is placed at both ends, then `unionPolygons` produces one outline.
- Revolved: `axisAt = axisRest.o + (axisX, 0, axisZ ?? 0)`, carried to now via `carry`. Half profile by default (meridian projection, radius on the lateral direction in front view). Full solid (`mirror: true`, or plan) is the union of hulls of 32-gon circles around the axis (plan silhouette is inferred from code, not tested).
- Front/plan shapes that are not solids go through `placeProjected` (per-vertex `carry`).
- Depth: `shapeDepth`, `restShapeDepth`, `shapeDepthIn` `evaluation:609–640`, consumed by `🎨️depiction/🧊️depth/🟦️.ts`.

### 2.3 Views

- `🔨️modules/🎨️depiction/🔭️views/🟦️.ts:24` (current):
  `export const CAMERAS: Readonly<Record<PlanarViewpoint, Camera>>` with `elevation` (paperX x, paperUp y, looking +z), `front` (paperX z, paperUp y, looking −x), `plan` (paperX x, paperUp z, looking −y).
- `project(viewpoint, p: Vec3): Point` `:36`. `cameraOf` `:30` throws `VolumeUnavailableError` for `axonometric` (current code, per its doc comment).
- `Camera` type `:21`.
- `VIEWPOINTS` (current `schema:50`) = `elevation, front, plan, axonometric`. `PLANAR_VIEWPOINTS` `schema:56` = first three. The `axonometric` viewpoint is the derived 3D view of the volume, being added now by the ticket.

---

## 3. Thermal

- `solveThermal(state: EvaluatedState, environment?: EnvironmentInput, options: ThermalOptions = {}): ThermalField` `🔨️modules/🌡️thermal/🟦️.ts:286`. Time is `state.timeS`.
- Clip wrapper `solveThermalAt(asset, query, environment?, options)` in `🌡️thermal/🕰️timeline/🟦️.ts:17`: rebuilds history by re-running `evaluate` per time, then calls `solveThermal` with `history`.
- Frame loop: `🎭️cast/🟦️.ts` about `:146–170`. Per frame: `evaluate`, `solveThermalAt` with a shared `thermalSession()` (`thermal:61`), `solveTransport` when needed.
- `ThermalField` `🔨️modules/🔗️contracts/🟦️.ts:176`:
  `export type ThermalField = { readonly environment: Environment; readonly regions: Readonly<Record<Slug, RegionField>>; readonly body: BodyField | null; readonly passengers: Readonly<Record<Slug, BodyField>>; readonly totals: { readonly netRadiativeW: number; readonly sensibleW: number; readonly latentW: number; readonly moistureGH: number; readonly emitterSensibleW: number; readonly emitterLatentW: number }; readonly assumptions: readonly Text[]; readonly notes: readonly Text[] };`
  `RegionField` `:142` includes `surfaceTempC`, `steadyTempC`, `skinTempC`, `emittedRadiantWM2`, `netRadiativeWM2`, `contactWM2`, `sensibleWM2`, `latentWM2`, `moistureGM2H`, `absorbedSolarWM2`.
- Quantity table `🌡️thermal/📏️quantities/🟦️.ts`: `surface-temperature` uses `pick: r => r.surfaceTempC`, centre = air temperature, shared range −20..60 °C. Skin: centre 33, shared 10..40. Emitted flux shared 230..700 W/m². Net radiative ±200. Latent ±300. Moisture ±450 g/(m²·h).
- Thermal view `🎨️depiction/🌡️thermal-view/🟦️.ts:61`: `export function thermalView(ctx, shapes, field): AnalyticView`. Span applies only to surface temperature. `colourOf = v => divergingColour(v, scale, true, theme)` (warm high).
- `scaleOf` `🎨️depiction/🖍️palette/🟦️.ts:76`:
  `export function scaleOf(values: readonly number[], centre: number, display: DisplayRange, minSpan: number, shared?: {min, max}): Scale`. Span mode gives `centre ± spanK`. Fixed mode uses the shared range with the centre clamped into it. Auto mode is symmetric about the centre with at least `minSpan` half-width (1 K for temperatures).
- Ramp `divergingColour` `palette:64`:
  `export function divergingColour(value: number, scale: Scale, highWarm: boolean, theme: Theme = "light"): string`. Linear fraction `f = min(1, |d| / halfSpan)` on each side of the centre, then `mix(weak, strong, f)`. Dark theme lifts strong ends towards `#f7f3e3` by 0.25.
- Stops (`STRUCTURAL_COLORS`, `🏗️structure/🟦️.ts:37`): compressionStrong `#d7301f`, compressionWeak `#fdae61`, zero `#d9d9d9`, tensionWeak `#abd9e9`, tensionStrong `#2c7bb6`.
- `thermalSpanK`: `DEFAULT_SPAN_K = 25` in the React target's `📏️range/🟦️.tsx:14`. Cast rows with `thermalSpanK` write `range: {mode: "span", spanK}` (`🎭️cast/🟦️.ts:149`). Absent means the absolute −20..60 °C range. The course uses one span per Scene (documented in S/README.md).
- Hatching: `PATTERN_FRACTION = 0.35` for thermal, beyond 35 % of the half-range.
- Emitters and transport: `solveTransport(asset, query, environment, options): TransportField` `💨️transport/🟦️.ts:89`. Emitter substeps `🔥️sources/🟦️.ts:46` (breath 4, plume 12, transpiration 60). Parcel constants `ALPHA_JET = 0.076`, `ALPHA_PLUME = 0.116` (`parcels:26–34`).
- Drawing `💨️transport-view/🟦️.ts`: parcels as discs (opacity 0.35), colour via `divergingColour` with a ±20 K span; pathlines as polylines with trail fading; streamlines dashed `[1.2, 0.8]` mm with arrow; condensation cyan `#34d1bf`; emitter markers as arrows.
- Layers are held in world metres and projected per view.
- Time variation: the schedule is `warm + timeS` (`framesOf`, cast:60–72). Each frame is one `query.timeS`.

---

## 4. Structure

- `solveStructure(state: EvaluatedState): StructuralResult` `🔨️modules/🏗️structure/🟦️.ts:545`. Also exported: `solveSemiDefinite(k, f)` (`:140`), `worldForce` (`:82`), `structuralLegend(quantity, result, range, include)` (`:727`), `freeBody` (`:761`), `DIAGRAM_INTERVALS = 24`, `DEFLECTION_INTERVALS = 120`, `DEFAULT_FRICTION = 0.6`.
- `StructuralResult` `contracts:255`:
  `export type StructuralResult = { readonly gravityMS2: number; readonly dynamics: "quasi-static" | "inertial"; readonly centreOfMass: Vec3; readonly totalMassKg: number; readonly segmentWeights: readonly WorldForce[]; readonly inertialForces: readonly WorldForce[]; readonly applied: readonly WorldForce[]; readonly reactions: readonly Reaction[]; readonly joints: Readonly<Record<Slug, JointLoad>>; readonly members: Readonly<Record<Slug, MemberDiagram>>; readonly residual: {...}; readonly notes: readonly Text[] };`
- Per-station data is `DiagramStation` `:235`: `s`, `at`, `z`, `axialN` (tension positive), `shearN`, `shearZN`, `torsionNm`, `momentNm` (sagging positive), `momentLateralNm`, `shearVector`, `momentVector`, `stressMinPa`, `stressMaxPa`. `MemberDiagram` `:252`: `{bone, stations, deflection}`. `JointLoad` `:232`.
- There is no per-bone scalar N/V/M in the result. Those live per station in `members[bone].stations`.
- Member colouring `🎨️depiction/🏗️structural-view/🟦️.ts`: each bone takes the station value of largest magnitude (`reduce`). Force view uses `axialN`, moment view uses the dominant bending moment, shear uses `shearOf`.
- Scale: `structuralLegend` returns auto symmetric about zero for structure (`shared` and `span` behave as auto here, as observed in code). `divergingColour(v, scale, false, theme)`: compression warm, tension cool.
- Hatch: `PATTERN_FRACTION = 0.15`. Compression is diagonal lines; tension is dots (SVG `🖼️svg` line ~74).
- Force arrows (`forcesView`, `:456`): `ARROW_MAX_MM = 14`, `ARROW_MIN_MM = 1.5`. Length `max(1.5, (mag/max)·longest)` with `longest = min(14, max(7, heightMm·0.9))`. Out-of-plane component drawn as dot or cross, radius `0.9 + 1.6·sqrt(|out|/max)` mm. Dash patterns: reaction `[1.6, 0.7]`, weight `[0.5, 0.5]`, inertial `[0.15, 0.45]`.
- Moment glyphs (`momentsView`, `:561`): radius `1.6 + 3.2·sqrt(part/max)` mm. Out-of-plane curls (`momentCurl`, `:527`). In-plane double-headed arrows (`momentVector`, `:539`). Symbols below 2 % of max are skipped.
- Member diagrams (`drawDiagram` `:387`): `DIAGRAM_MM = 6` mm ordinate for the peak. Side: axial +1, shear +1, moment −1.
- Scaling: `mmPerMetre(scale) = 1000 / denominator` (`🔨️modules/📐️units/🟦️.ts:35`). Arrow lengths and glyph sizes are paper millimetres and are independent of the denominator, except `longest` depends on model height.
- Tests: `🧪️tests/🏗️structure-oracle/🐍️.py`, `🧪️tests/🏗️structure-projection/🟦️.ts`.
- Not read: the body of `solveStructure` and `solveSemiDefinite` (inferred: assembly plus stiffness solve).

---

## 5. Depiction

- `depict(state: EvaluatedState, options: DepictionOptions, analysis: Analysis = {}): DrawingScene` `🔨️modules/🎨️depiction/🟦️.ts:148`.
- `Analysis` `:115`: `{ thermal?: ThermalField; structure?: StructuralResult; transport?: TransportField }`.
- `depictAll(...)` `:220`, `modelScene(state, {viewpoint, analysis?, diagrams?})` `:235` (CAD path, metres, layers STAFFAGE-BODY, -DETAIL, -TEXTURE, -GLAZING, GROUND).
- `DrawingScene` `contracts:364`: `widthMm`, `heightMm`, `background: string | null`, `layers`, `legends`, `metadata: {asset, title, query, timeS, view, viewpoint, scale, resolution, fidelity, omittedValues, representation, profile, paperUnits: "mm", modelUnits: "m", mmPerMetre, origin}`.
- Paper mapping: `paper: p => ({x: p.x·k, y: −p.y·k})`, `k = mmPerMetre(options.scale)`.
- `VIEWS` `contracts:273`:
  `export const VIEWS = ["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"] as const;`
  `View` `:276`.
- `DepictionOptions` `:294`, `DepictionCommon` holds view, viewpoint, scale, transparent, profile, representation, theme.
- `ModelScene` `:379`, `JobControl` `:384`: `{ signal?: AbortSignal; progress?: (done, total, label: Text) => void }`.

### 5.1 Dark vs light

- `🖍️palette/🟦️.ts`:
  - `Theme` `:15`.
  - `THEMES` `:21–24`: dark `background #001117`, `ink #f7f3e3`, `softInk #7b827d`, `toneDark #2e3c3d`, `toneLight #f1edde`. Light: `background #f7f3e3`, `ink #001117`, `toneDark #243235`, `toneLight #d3d2c5`.
  - `toneColour(theme, tone)` `:50`: linear sRGB mix from toneDark to toneLight, tone clamped to 0..1.
- Theme choice `🟦️.ts:117–119`: `dark-*` views get dark, `light-*` get light, and thermal/forces/moments read `options.theme` (default light).
- Style `baseStyle` `:121–129`:
  - line views: fill with paper background (hides lines behind), ink outlines;
  - solid views: fill `toneColour(theme, tone)`, glazing opacity 0.45, outline at half the role pen when `detailed` (scale denominator ≤ 100).
- Pen weights are not theme-dependent. They come from `PenSet` `schema:277` (`body`, `detail`, `texture`, `glazing`, `ground`, in paper mm, `null` = not drawn at that scale).
- Ground line `:167–170`.
- `drawShapes` `🧩️shapes/🟦️.ts:141`. Hatches for solid views at opacity 0.55.
- Viewpoint selection: `options.viewpoint` is read directly. Missing shapes throw `ViewMissingError` (no substitution). No default in `depict`; `defaultDepictionOptions` uses `"elevation"` (`:70`).

### 5.2 Cameras and projection

- Cameras as in section 2.3. Handedness: left-handed world (x ahead, y up, z left). Elevation seen from the right (−z), so ahead appears to the paper's right.
- `nearness(p, looking) = -(p·looking)` (`🔭️views:37`), larger is nearer, painted later.

### 5.3 Sheets

- `📑️sheet/🟦️.ts:30`: `PROJECTIONS = ["first", "third"]`. `composeSheet` `:171`. Third angle: plan above elevation, front to the right. First angle: plan below, front to the left.
- `🔭️multiview/🟦️.ts`: `resolveViews`, `resolveDrawing` (bodies not read).

---

## 6. Export

- No `📤️export/🟦️.ts` root and no exporter registry or common interface. Each writer is a free function taking a scene and returning a string or bytes. Dispatch is a `switch` in the CLI.
- Writers:
  - `🖼️svg/🟦️.ts:137`: `export function writeSvg(scene: DrawingScene, options: SvgOptions = {}): string`
  - `📕️pdf/🟦️.ts:207`: `export function writePdf(pages: readonly DrawingScene[], control?: JobControl): Uint8Array`
  - `📐️dxf/🟦️.ts:27`: `export function writeDxf(scene: ModelScene, note = ""): string`
  - `🖊️dwg/🟦️.ts:90`: `export async function writeDwg(scene: ModelScene, control?: JobControl): Promise<Uint8Array>` (Rust crate under `🖊️dwg/📦️packages/🦀️rust/`)
  - `🎞️frames/🟦️.ts:48`: `export async function writePngSequenceZip(schedule, input, frames: AsyncIterable<Uint8Array>, control?): Promise<Uint8Array>`
  - `🗜️zip/🟦️.ts:27`: `export function writeZip(entries, comment = ""): Uint8Array` (stored members, deterministic DOS date, CRC-32).
  - `🗜️zip/🟦️.ts:88`: `withPngText(png, entries)` embeds PNG tEXt.
- Format lists, three different ones:
  - CLI `🎬️cli/🟦️.ts:57`: `CLI_FORMATS = ["svg","pdf","dxf","dwg","png-sequence","mp4","webm","sheet-svg","sheet-pdf","sheet-dxf","sheet-dwg"]`.
  - Schema `schema:56–59` `EXPORT_FORMATS`, which includes `component` and no sheet variants.
  - React downloads `🎯️targets/⚛️react/🔨️modules/📤️downloads/🟦️.ts:20`: `ExportFormat = "svg" | "pdf" | "dxf" | "dwg" | "png-sequence" | "mp4" | "webm"`.

### 6.1 CLI (`🎬️cli/🟦️.ts`)

- Usage line `:64`. Flags: `--asset` (required), `--clip` xor `--pose`, `--view` (required, `VIEWS`), `--viewpoint` xor `--projection` (`first|third`, sheet formats only), `--time` (clip, still formats only, excludes `--fps`), `--environment` (thermal only), `--scale` (required), `--fps`, `--format` (required), `--out` (required), `--catalog`, `--lang` (required, no default), `--px-per-mm` (default 8), `--background` (default `#ffffff`, opaque raster only).
- Parsing `:67–96`: `--key value` pairs only, no boolean flags.
- `runExport` `:162–242`. Progress `"Evaluating frames"` and final `"Exporting"` / `"Exportiere"`.
- `main` `:246–257`: only `SIGINT` is handled (`controller.abort()`). Progress to stderr as `[done/total] label`. Written paths to stdout. Exit 1 on error, 3 for `DwgUnavailableError`.
- File naming, decision D14, at `cli:190`:
  ``const stem = `${options.asset}-${options.clip ?? options.pose}${options.time === undefined ? "" : `-t${options.time}`}-${options.view}-${where}-1to${options.scale}`;``
  where `where` is `sheet-<projection>` or the viewpoint. Suffixes: `-frame-NNNNNN.svg` for SVG frames, `.dxf`/`.dwg` per frame, `${stem}.pdf`, `${stem}.zip`, `${stem}.${format}` for mp4/webm.
- Divergence (inferred): the React `exportFilename` (`downloads:102–106`) uses underscores and a different order (`<asset>_<what>_<viewpoint>_<view>_1-<denominator>.<ext>`), so CLI and browser names differ.
- D14 is defined in the ticket board (see section 9), not in code.

### 6.2 Cast exporter (`🎭️cast/🟦️.ts`)

- Usage `:272`: `--manifest <file> [--ids a,b,c] --out <dir> [--catalog <dir>] [--parent <pid>]`. Unknown flags refused.
- `render` `:145`: `yieldToLoop()` (setImmediate), `control.signal?.throwIfAborted()`, then evaluate, thermal, structure or transport, depict with annotations off, `transparent: true`.
- `MAX_FRAMES = 600` `:36`, `thinned` `:51`.
- Dedup by SHA-256 of placed SVG text (`:189–197`).
- `WRITES_IN_FLIGHT = 16` `:113`. `patiently` `:131` retries EPERM/EBUSY/ENOTEMPTY 8 times.
- Atomic: writes to `.<id>.tmp-<pid>` then renames (`:222–234`).
- `CastOptions = JobControl & {catalog?, started?}` `:44`. `report` `:260` gives `{en: "Rendering cast frames", de: "Besetzungsbilder werden gerendert"}`.
- `main` `:317–340`: SIGINT, SIGTERM, SIGHUP on POSIX; SIGINT, SIGTERM, SIGBREAK on Windows. Exit 130 with `[cancelled]`.
- `watchParent` `:307`: polls `process.kill(pid, 0)` every 500 ms (`:297`), `unref`'d, aborts then `exit(130)` after 5 s.
- stderr lines: `[pid N] ppid M`, `[cast i/n] id`, `[done/total] …` every 50 frames, `[peak] MB resident`.

### 6.3 One exporter end to end: SVG

1. Entry `writeSvg(scene, options)` `🖼️svg/🟦️.ts:137`, returns a standalone SVG 1.1 string. `width`/`height` in mm, `viewBox = 0 0 widthMm heightMm`.
2. Writes `<title>`, `<desc>`, `<metadata>`, then `svgMarkup` `:125`: `<defs>` (hatch patterns, `patternDef` `:70`), optional background `<rect>`, one `<g id="staffage-<layer>" inkscape:groupmode="layer">` per layer.
3. `itemMarkup` `:82` writes text (`<text>` with font family) and paths (`<path d>` with `compactData` `:43`). Pattern fills add a second path with `fill="url(#…)"`.
4. `SvgOptions` `:32` = `{compact?: boolean}` hoists shared attributes; the cast exporter uses it.
5. Scene comes from `depict`. The writer imports `VIEW_TITLES`, `viewpointCaption`, `FIDELITY_NAMES`, `PROJECTION_NAMES`, `isSheet`. It does not call the analysis code. Escaping via `esc` `:27`.
6. Bytes: CLI `writeFile(path, string)` via `emit`; React `TextEncoder`.
7. Registration: `CLI_FORMATS` `cli:57`, `oneOf("format", CLI_FORMATS)` `cli:85`, invocation `case "svg": case "sheet-svg":` `cli:201–203`, one file per scene. Raster formats reuse `writeSvg` then call `rsvg-convert` via `ToolPort` (`cli:152–154`).
8. No progress or cancellation of its own (inferred): the frame loop checks the signal, the write is synchronous.

---

## 7. React target and site

Abbreviations: R = `🎯️targets/⚛️react`, RM = `R/🔨️modules`, RP = `R/📦️packages/🟦️typescript`.

- Public exports `RP/🟦️.tsx:4–15`: `StaffageFigure`, `StaffageValueTable`, `defineStaffageElement`, export functions, `VIEWS`, `LANGUAGES`, `SCALE_PRESETS`.
- `StaffageFigure` `RM/🧍️figure/🟦️.tsx:110`: `export function StaffageFigure(props: StaffageFigureProps): ReactElement`. Props (`:19–45`): `asset`, `query?`, `clock?`, `view`, `scale`, `viewpoint` (required), `representation?`, `theme?`, `frame?`, `language` (required), `zoom?`, `fit?`, `transparent?`, `annotations?`, `thermalQuantity?`, `forceQuantity?`, `range?`, `patterns?`, `diagrams?`, `environment?`, `onEvaluated?`, `onInspect?`, `className?`, `title?`, `live?`. `query` is used if given (`:131`).
- `<staffage-figure>` `RM/🧩️element/🟦️.ts`: `defineStaffageElement(catalog, name = "staffage-figure")` `:42`. Attributes (`:26`): asset, view, scale, lang, pose, clip, time, playing, rate, loop, zoom, fit, viewpoint, theme, transparent, no-patterns, analytic, thermal-quantity, force-quantity, live, no-dimensions, no-scale-bar, no-legend, no-labels, no-ground. Properties `state` `:59`, `clock` `:63`. Events `staffage-evaluated` (`:212`, `:238`), `staffage-inspect` `:248`. Language from `lang` or nearest `[lang]` (`:125–128`). No language means nothing renders (`:139–144`). `viewpoint=all` opens the sheet (`:136–138`). `pose` wins over `clip` (`:169–171`).
- Export and download buttons: `RM/🔬️detail/📤️export-panel/🟦️.tsx` (`FORMATS` `:15`, run button `:118`, cancel via `ProgressNotice` `:135`). Engine `RM/📤️downloads/🟦️.ts`.
- Viewpoint switcher: `RM/🎛️viewer/🟦️.tsx:243` renders `ViewpointSelector` `RM/🔭️viewpoints/🎚️selector/🟦️.tsx:31`. Choices `VIEWPOINT_CHOICES = [...VIEWPOINTS, "all"]` `RM/🔭️viewpoints/🟦️.ts:19`. Sheet `RM/🔭️viewpoints/🪟️sheet/🟦️.tsx`.
- View switcher: `RM/🎛️viewer/🟦️.tsx:242`, a `Segmented` over `VIEWS`.
- Archive app `RP/📚️app/🟦️.tsx`: `LanguageGate` `:29–53`, `SettingsMenu` `:57–123`, routes `RM/🗺️routing/🟦️.ts` (route list `:3`, viewer params `:5`, default viewer `:144–156`, default view `dark-line`).
- Site shell `🌐️site/🟦️.ts:5–9` mounts the archive on `#root`. Vite builder `🌐️site/🏗️builder/🌐️vite/🟦️.ts`. Site `package.json` project targets: `dev`, `dev-site` (port 6071), `build`, `test`, `typecheck`.
- Value choice: viewpoint is required with no default (decision D15). Theme: `th=` defaults to `auto`, following `useDarkSurface()`. App preference `Theme = "system" | "light" | "dark"` (`RM/💾️persistence/🟦️.ts:48`).

### i18n (`RM/🌐️i18n/`)

- `🟦️.ts`: `OFFERED = LANGUAGES` `:25`, `translator(language)` `:57–60` with `{name}` placeholders, `resolveLanguage(stored, browser)` `:50–54`, `inEveryLanguage(key)` `:63–65`.
- `export type Key = keyof typeof ENGLISH` `:16`: unknown keys are compile errors.
- English parts `🔠️english/`: `CORE_EN`, `COLLECTIONS_EN`, `COMPARE_EN`, `DETAIL_EN`, `PLAYGROUND_EN`, `VIEWER_EN`, `VIEWPOINTS_EN`, merged by spread. Keys are dotted, e.g. `"viewer.viewpoint.front": "Front elevation"` (`🎛️viewer.ts:2–8`). Duplicates silently override (inferred from the spread).
- German `🔡️german/` mirrors each part, typed `Record<keyof typeof VIEWER_EN, string>`. The merged German is typed `Record<keyof typeof ENGLISH, string>` (`🔡️german/🟦️.ts:8`), so a missing key fails compilation.
- No default language. `LanguageGate` lists every language's text.
- Catalog texts are `{en, de}` data, not bundle keys (`localized`, `:67–68`).
- Adding a label:
  1. Add the key to `🔠️english/🎛️viewer.ts` (the part that fits).
  2. Add the same key to `🔡️german/🎛️viewer.ts`.
  3. Use `t("viewer.myLabel")` from `useArchive()`, or `translator(language)(...)` outside React. Viewpoint names: `viewpointWord`, `VIEWPOINT_NAMES` (`RM/🔭️viewpoints/🟦️.ts:21–30`).
  4. Run `repo run @teaching/staffage-react:typecheck` and `repo run @teaching/staffage-react:test` (`language-selection` checks bundle parity).

### Accessibility and customisation conventions

- Figure: `role="img"`, `aria-label`, `aria-describedby` pointing to a visually hidden data table (`figure:47–74`, `:181–184`). Live region `role="status" aria-live="polite"` (`:205–207`).
- Element: `role="img" tabindex="0" aria-label aria-describedby` (`element:209`), `role="alert"` for problems (`:161`).
- Segmented control `RM/🪟️chrome/🟦️.tsx:70–84`: `radiogroup`/`radio`, `aria-checked`, roving tabindex, arrow keys (`:52–62`).
- Keyboard: figure arrows step values, Escape clears (`figure:160–167`). Settings popover Escape returns focus (`app:67–71`). `/` focuses search (`app:132–141`). Viewer stage `+ - 0 1` and Alt+arrows, `aria-keyshortcuts` (`stage:132`).
- Focus: `R/🎨️.css:3–6` `:focus-visible` outline 2 px.
- Hatch patterns on by default: the colour-independent reading (`element:4`, `patterns` `:190`).
- Reduced motion: clock starts paused under `prefers-reduced-motion` (`⏱️clock/🧱️core/🟦️.ts:4,23,37`), `preferences.motion` (`🧭️context/🟦️.tsx:26–37`), Tailwind `motion-reduce:` classes.
- Tokens: staffage defines no palette and uses design-system tokens (`var(--accent)`, `bg-panel`, etc.). Token source: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`. Staffage does not use `data-theme`. `prefers-color-scheme` appears only in the site head (`🌐️site/🏗️builder/🌐️vite/🟦️.ts:85–86`).
- Customisation hooks: `part="figure"` and `part="problem"` on the element, React `className`, `title`, `zoom`, `fit`, `transparent`, `annotations`. Preferences `textSize`, `playful`, `motion`, `theme`. CSS grid tokens `--staffage-columns`, `--staffage-rows` (`R/🎨️.css:13–14`).

---

## 8. Tests

### 8.1 Projects and commands

| Project | Directory | Targets |
|---|---|---|
| `@teaching/staffage` | `S/📦️packages/🟦️typescript` | `test`, `typecheck` |
| `@teaching/staffage-react` | `RP` | `test`, `typecheck` |
| `@teaching/staffage-site` | `S/🌐️site/📦️packages/🟦️typescript` | `dev`, `dev-site`, `build`, `test`, `typecheck` |

- `repo run @teaching/staffage:test --lang=en`
- `repo run @teaching/staffage-react:test`
- `repo run @teaching/staffage-site:test`
- Test levels in the scripts: staffage `fundamental|quick|long|exhaustive`; react and site `quick|long|exhaustive`, default `quick`.
- I did not run the tests. Only `--help` style reads were used.

### 8.2 Runners

- Vitest staffage: `S/🧪️tests/🎚️config/🟦️.ts`, node environment, `include: ["../../🧪️tests/*/🟦️.ts"]` (new cases need no config change), 180 s timeouts.
- Vitest react: `R/🧪️tests/🎚️config/🟦️.ts`, jsdom, `include: ["../../🧪️tests/*/🟦️.tsx"]`.
- Vitest site: `S/🌐️site/🧪️tests/🎚️config/🟦️.ts`.
- `.feature` (Gherkin): specifications only. No runner found: `parseFeature` from `@semio-tech/framework/test/gherkin` is not imported anywhere in S (grep empty). Test files cite them in headers (`@see ./🥒️.feature`). Inferred from the absence of an import.
- Python oracles: `S/🧪️tests/🧰️support/🧰️.ts:58–60`: `python(script, packages, args)` runs `uv run --quiet --with <pkg>… python -I <script> <args>` and returns stdout. Tests parse the last JSON line. Missing tools throw, never skip (`requireTool` `:42–50`, provisioned by `repo tools setup all`).

### 8.3 Case folder conventions

- `S/🧪️tests/<case>/` (about 85 folders, 77 with `🥒️.feature`):
  - `🥒️.feature`: Gherkin-style scenarios with `@capability-…` tags.
  - `🟦️.ts` (or `.tsx` in react): Vitest suite; its describe/it names echo the scenarios.
  - `🐍️.py`: third-party oracle (shapely, numpy, scipy, svgelements, pymupdf, ezdxf, lxml, reportlab) via `uv`.
  - `🧰️.ts`: shared helpers for the case.
  - Generators: `🔭️multiview-coverage/📊️matrix.ts` (sharded CLI writing `matrix.json`), `🖨️proofs.ts`, `🪞️derived.ts`, `⚓️landmarks.ts`.
  - `🖊️dwg-reopen/🦀️.rs`: Rust case; runner not checked.
  - Shared helpers: `S/🧪️tests/🧰️support/🧰️.ts`.
- `S/🧫️fixtures/<case>/` (about 70 folders): `🔣️.json` language-agnostic vectors, each with a `description` naming the hand-computed numbers, read via `fixture<T>(name)` (`support:13`). A few folders hold `🟦️.ts` builders (e.g. `🧱️synthetic-figure`, `🌡️thermal`).

Examples:
1. `🧪️tests/🧊️three-views/`: `🥒️.feature`, `🟦️.ts` (reads `🧫️fixtures/🧊️three-views/🔣️.json`, calls `python(…"🐍️.py", ["shapely"], [input])` at about line 142), `🐍️.py` (shapely extents), `🧰️.ts` (sweep and consistency rules).
2. `🧪️tests/🧊️solid-placements/`: `🐍️.py` rotates the rectangle profile by the bone's world angle. Fixture `🧫️fixtures/🧊️solid-placements/🔣️.json` has hand-computed cos and sin values.
3. `🧪️tests/🔭️multiview-coverage/`: largest case. `🧰️.ts` is the engine (matrix over asset × scale × viewpoint × mode × pose/clip). `🟦️.ts` calls its Python oracle (svgelements, scipy, shapely). Fixture `🧫️fixtures/🔭️multiview-coverage/🔣️.json`.

---

## 9. Decisions log

Canonical register: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🗂️work-board.md`, lines 63–159 (D1–D47). A summary of D1–D36 is in `…/📦️handoff.md` lines 174–211. Ids cited in code/README (D5, D7, D13, D14, D18, D19, D21, D24, D40) all resolve to this board. False positives in code grep: `D0` (`🌡️thermal/🍃️canopy/🟦️.ts:5`, a formula constant), `D1` in `🏗️structure/📋️load-cases/🔣️.json:18` (a load category).

- D1: side view is the existing `elevation` (rig plane); names `elevation | front | plan`.
- D2: unified 3D state contract (Vec3, BoneFrame, tip, anchors.z, root, WorldForce and AppliedLoad fields; momentNm renamed mzNm).
- D3: structural solver becomes 3D (approved).
- D4: transport, emitters and condensation become Vec3; directionRad dropped; windDirectionDeg is a bearing from +x towards +z.
- D5: every cast row requires `viewpoint`; `mirror` only with elevation.
- D6: frame is left-handed (x ahead, y up, z left); cameras elevation from −z, front from +x, plan from +y.
- D7: three-view sheet: plan above elevation sharing x; front beside it sharing y.
- D8: unavailable scales need a required `cause`.
- D9: an `asset` alternative must be drawable at that scale and the same kind; no chains.
- D10: `validRange` is a declared support range; coverage includes its ends up to 1:1000.
- D11: `timeS` is absolute; `clipTimeS` is the wrapped clip phase.
- D12: one environment evaluator in `🌡️thermal/🌤️environment`; wind blended as a vector.
- D13: cast index `heightM` is the physical height; paper extents in `boundsM`.
- D14: every export, download and preview filename includes the viewpoint.
- D15: missing views never tolerated; no `?? []` on geometry; no default viewpoint.
- D16: revolve `axisX` as absolute world coordinate; superseded by D24.
- D17: an asset at least 3 mm on paper must be drawable at that scale; below 3 mm may be unavailable with a cause.
- D18: `dimensions` is the physical envelope; extent tolerance max(2 %, 15 mm).
- D19: course casts: mirror of a laterally asymmetric asset needs a guard allowlist reason; size from real dimensions; plan rotation only as documented yaw.
- D20: manifest fixes (three missing poses; three 1:500 bands flipped to authored).
- D21: a solid's `axisBone` is the shape's own bone or an ancestor.
- D22: symbol `semantics` keeps true-extent, true-position, nominal.
- D23: moment vector is a double-headed arrow; sense by left-hand rule.
- D24: revolve axis local to its bone (`axisX`, `axisZ` offsets); supersedes D16.
- D25: `view-twin-missing` removed; `lateral-ambiguous` added (shapes on spread or twisted bones must be solids).
- D26: solids allowed in every view.
- D27: a true-extent symbol with all three views passing the print minimum satisfies D17 (print minimum not yet implemented, per handoff).
- D28: `lateral-ambiguous` flags a flat shape only when flattening error exceeds the 0.2 mm print minimum.
- D29: vegetation crosswind sway gets no validator exemption.
- D30: coverage matrix reports nominal-scale cells separately from `validRange` end cells.
- D31: revolved solid declares its bone-local axis explicitly (`u`, `v`, `w`). Not yet landed per handoff.
- D32: `--projection first|third` required, no default; third angle is D7; first angle is ISO 128.
- D33: a 1:500 band flagged `representation-copied` stays authored but counts as FAIL until redrawn.
- D34: every check uses the single D18 tolerance; a view reused across two bands is FAIL (`derived-view`).
- D35: thin parts become solids with physical thickness; geometry never widened to fake visibility.
- D36: asset `dimensions` is the single source; manifest `heightM`/`widthM` match within 1 mm (board says closed; handoff still lists four rows as open).
- D37: slender diagonal members get a `member` solid (prismatic bar between two bone-local points with physical section).
- D38: playground placement has a 3D position (x along the line, z lateral, yaw); each view projects it.
- D39: symbol ids are view-neutral; `*-plan` names renamed; scheme `<asset>-symbol`; no aliases.
- D40: every view is true orthographic with hidden-line removal; no dashed hidden lines; glazing does not occlude.
- D41: cross-view sync has two tiers: per-bone extents and anchor point-in-shape are a hard FAIL; group drift is reported only.
- D42: extruded solids get optional linear taper toward the bone tip (`zTip`).
- D43: an airborne region may have no drawn shape if its `hidden` note says so in en and de and an emitter gives the visible trace.
- D44: physical root yaw (`yawDeg`) rotates the whole evaluated 3D state before projection.
- D45: one anchor per physical contact or support, at its real x, y, z; lumped z-0 anchors are split.
- D46: revolved solids take optional `lateralScale` (> 0, default 1) for an elliptic cross-section.
- D47: extruded solids declare `sweep: "v" | "w"` explicitly, no default; `zTip` applies along the sweep axis.

---

## Open points for the implementer

- `axonometric` is being added to `VIEWPOINTS` now. `cameraOf` throws for it and says "replaced by the volume projection". Check the state before wiring a 3D view.
- Thermal and load blends use `ease(keyA.ease, w)`, not always smoothstep (module header says smoothstep).
- Twist does not accumulate per cycle, unlike angle and root (undocumented).
- The structural legend treats `shared` and `span` as auto (observed; may not be intended).
- CLI handles SIGINT only; the cast exporter also handles SIGTERM, SIGHUP, SIGBREAK.
- Three format lists exist (CLI, schema, React downloads) with different members.
- CLI and browser filename conventions differ (D14 is implemented with different ordering and separators in the browser).
