# 🔗️ Interfaces: staffage multiview (owner: sonnet-contracts)

Status: draft 1, 2026-11-02. Names below are the agreed contract. Other workers code against them. Change requests go through `main`.

Paths: L = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder.

## 1. Landed contract types (L/🔨️modules/🔗️contracts, L/🧬️schema)

All fields are required unless marked optional. There is no compatibility layer.

| Type | Change |
|---|---|
| `Vec3` | `{ x, y, z }` objects. World order is (ahead, up, left). Exported from contracts. |
| `BoneFrame` | `{ o, u, v, w }`, each `Vec3`. `u` runs along the bone, `v` is its normal, `w` is the lateral axis. It is the `solveChain` frame. |
| `EvaluatedBone` | gains `frame: BoneFrame` and `tip: Vec3`. There are no separate `originZ` or `tipZ` fields; use `frame.o.z`. `origin: Point` stays. |
| `EvaluatedState` | gains `root {x, y, z, angleDeg}` and `anchors[id].z`. `kinematics.comVelocity` and `segmentAccelerations` are `Vec3`. |
| `WorldForce` | `{ id, at: Point, z, fxN, fyN, fzN, mxNm, myNm, mzNm, bone, kind, title }`. `momentNm` is renamed `mzNm`. |
| schema `Anchor` | `z?`, default 0. |
| schema `AppliedLoad` | `z?`, `fzN?`, `momentXNm?`, `momentYNm?`, all default 0. `momentNm` stays and means the moment about the lateral axis (Mz). |
| schema `AppliedLoad.distributed` | `wzNPerM?`, default 0. |

Owners of the follow-up work:
- sonnet-rigs: `evaluate()` fills frames, tip, anchors.z, root and the 3D accelerations.
- sonnet-structural: `worldForces()`, the 3D solver, and the Structure region of contracts (`Reaction`, `JointLoad`, `DiagramStation`, `StructuralResult`).
- sonnet-thermal, decision D4: Parcel, Pathline, Streamline, condensation and EmitterState become `Vec3`; `directionRad` is dropped; `windDirectionDeg` becomes a horizontal bearing from +x towards +z. Thermal edits the Transport region of contracts; contracts does not.

The old `momentNm` references on `WorldForce` (structure module, structural-view, structure harness, `worldForces()`) are not fixed by contracts. They sit inside the structural rewrite.

## 2. Conventions (full derivation in `📐️conventions.md`)

- Asset-local and scene frames are one frame: x ahead, y up, z lateral, with left of the asset = +z. In handedness terms this frame is left-handed. See `📐️conventions.md` section 1 for the inconsistency report.
- Elevation (side) is seen from the asset's right, which is -z. Paper x = world x and paper y = world y, with ahead on the right.
- Front is seen from ahead, from +x looking back. Paper x = world z, so the asset's left is on the right of the paper. Paper y = world y.
- Plan is seen from above. Paper x = world x and paper "up" = world z, so the asset's left is up the page.
- A view switch changes only this camera basis. Pose, time, root, scale, seed and environment are untouched.

## 3. Schema gaps closed (L/🧬️schema/🔣️.json and 🟦️.ts, in lockstep)

1. `SymbolRepresentation.geometry` requires `elevation`, `front` and `plan`, each with `minItems: 1`. This is the "Alternative" geometry gap. `AuthoredRepresentation.geometry` already required all three.
2. `UnavailableRepresentation` gains a required `cause`: `"below-printable-feature" | "no-physical-depiction"`.
   - `below-printable-feature`: a real feature is smaller than the profile can print at this scale.
   - `no-physical-depiction`: the asset has no meaningful physically scaled drawing at this scale.
   - An unavailable scale is unavailable in all three viewpoints. A half-available scale does not exist.
3. Fidelity vocabulary, carried by resolver results, never stored:
   - `physical`: an authored scale representation, or a symbol of semantics `true-extent` (model metres, real size).
   - `conventional-symbol`: a symbol of semantics `true-position` or `nominal` (paper millimetres). Its size carries no dimension.
   - A `symbol` alternative is offered with its fidelity.

## 4. Validator failures (L/🔨️modules/🗂️catalog `validateAsset`, `Issue.code`)

Existing: `elevation-empty`, `front-empty`, `plan-empty`, `rep-undecided`, `alternative-missing`, `alternative-unresolved`, `extent-height`, `extent-width`.

New (each fails with a path, a code and an en/de message):

| Code | Meaning |
|---|---|
| `unavailable-cause-missing` | An unavailable scale has no `cause`. |
| `symbol-front-missing`, `symbol-plan-missing`, `symbol-elevation-missing` | A symbol lacks authored geometry for that view. |
| `alternative-symbol-view-missing` | An unavailable scale offers a symbol alternative that has no geometry for some view. |
| `extent-front-height`, `extent-front-depth` | The front extents do not match `heightM` and `depthM`. |
| `extent-plan-width`, `extent-plan-depth` | The plan extents do not match `widthM` and `depthM`. |
| `symbol-extent-*` | The same four checks, plus elevation, for `true-extent` symbols. |

Extent rule: the measured extent excludes role `ground` and is taken at the default pose. It must be within max(2 %, 15 mm) of the dimension.

The authored catalog is expected to fail these. The failure list is the migration backlog.

## 5. Resolver API (L/🔨️modules/🔭️multiview/🟦️.ts, exported from the barrel)

```ts
type DrawingSource = { kind: "scale"; denominator: number } | { kind: "symbol"; symbol: Slug };
type Fidelity = "physical" | "conventional-symbol";

resolveDrawing(asset, query: EvaluationQuery, viewpoint: Viewpoint, source: DrawingSource, mode: View, options): ResolvedDrawing | DrawingUnavailable
resolveViews(asset, query, source, mode, options): { state, analysis, views: Record<Viewpoint, ResolvedDrawing | DrawingUnavailable> }
```

- `query` is a pose, a clip at time t, or a transition. This is the existing `EvaluationQuery`.
- `viewpoint` is `elevation | front | plan`. `mode` is one of the seven `View`s. `options` is the `DepictionOptions` remainder (language or textless, theme, environment, transport, ...). It excludes `view`, `viewpoint`, `scale` and `representation`.
- `ResolvedDrawing = { kind: "drawing"; scene: DrawingScene; state: EvaluatedState; fidelity: Fidelity; resolution; viewpoint; mode }`.
- `resolveViews` calls `evaluate` once and solves the analysis (thermal for thermal, structure for forces and moments) once. It then projects the three viewpoints from that one `EvaluatedState`. This is the only supported way to produce synchronized top, front and side views. Consumers must not call `evaluate` per viewpoint.
- `DrawingUnavailable = { kind: "unavailable"; code; viewpoint; mode; requested; reason: Text; cause; limitingFeature; limitingSizeM; alternatives: { alternative: Alternative; fidelity: Fidelity | "other-asset" }[] }`.
  - `code` is `scale-unavailable | scale-unsuitable | view-missing | symbol-unknown | symbol-scale`.
- There is no silent fallback of any kind: no other scale, no other view, no blank scene.
- `depict()` throws `ViewMissingError` when the selected representation has no shapes in the requested viewpoint. The resolver turns it into `view-missing`.
- Query errors (unknown pose or clip) still throw.

Consumers:
- Viewer, site and React: call `resolveViews`.
- Exports (SVG, PDF, DXF, DWG, frames, video): call `resolveDrawing` per viewpoint, or `resolveViews` for sheets.
- Casts: `resolveDrawing`.
- Validation: call `validateAsset` and the resolver over `[pose | clip samples | transitions] x viewpoint x scale x mode`.

## 6. Update 2026-11-02 (rulings D6, D8 to D28)

Validator rules in force (`🔨️modules/🗂️catalog`), beyond section 4:

| Code | Rule |
|---|---|
| `unavailable-cause-missing` | `cause` required (`below-printable-feature` or `no-physical-depiction`). |
| `unavailable-but-drawable` (D17, D27) | Largest dimension at least 3 mm on paper at that scale; satisfied only by a `true-extent` symbol alternative with all three views and passing symbol extents. The symbol print-minimum part of D27 is not implemented yet. |
| `cause-missing-symbol` | The reason or limiting feature names a symbol id the asset does not have. |
| `symbol-semantics-missing` | `semantics` is required (`true-extent` | `true-position` | `nominal`). |
| `alternative-asset-category`, `alternative-asset-chained`, `alternative-asset-undrawable` (D9) | An `asset` alternative must be the same category, authored at that scale, and not itself unavailable. Needs `AssetContext.assets` as a map id to Asset. |
| `valid-range-gap` (D10) | Two adjacent authored presets leave denominators between their ranges. `valid-range-overlap` stays. |
| `validateRanges(asset, profile)` (D10) | Separate export. Measures print-profile suitability at both ends of every `validRange`; codes `range-end-stroke`, `-gap`, `-feature`, `-hatch`. Slow (minutes for the catalog), so run it explicitly. |
| `solid-axis-not-ancestor` (D21) | `axisBone` of a revolved solid is the shape's own bone or an ancestor. |
| `lateral-ambiguous` (D25, D28) | A non-solid shape on a spread or twisted bone whose paper error `extent * sin(theta_max)` exceeds the print profile minimum feature at some band. One issue per bone, listing shapes, bands, theta and error. |
| `extent-*`, `symbol-extent-*` | Tolerance is max(2 % of declared, 15 mm). Geometry only; ground-role shapes are excluded. |

Removed on ruling: `view-twin-missing` (D25) and `solid-on-elevation` (D26). Solids are allowed in every viewpoint.

Stable shared API (do not change without telling contracts):
- `🔨️modules/🎨️depiction/🔭️views/🟦️.ts`: `CAMERAS`, `Camera`, `project(viewpoint, Vec3) -> {x, y}` (paper-up metres), `nearness(viewpoint, Vec3)` (larger is nearer), `ViewMissingError`. Re-exported from the multiview module and the package barrel.
- `DrawingScene.metadata.fidelity: "physical" | "conventional-symbol"` is the field consumers read to show the symbol semantics to the reader; `ResolvedDrawing.fidelity` carries the same value. `RepresentationResolution` `unavailable` carries `cause`.
- `depict()` throws `ViewMissingError` for a viewpoint without shapes; `evaluate()` throws it when a representation has no `front` or `plan` array at all.
- `areaWeighted` was removed from depiction shapes (thermal owns the exposure-weighted version).
- Terminology: captions follow `📖️terminology.md` (viewpoint names, `Plan section` / `Grundriss-Schnitt` for a plan cut; the side elevation caption names its viewpoint too).
