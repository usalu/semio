# r2-eb: cast export failure

## Root cause
- Throw site: `S/🔨️modules/🖨️representation/🟦️.ts:59` (`measureOnce`), reached via resolveRepresentation, resolveDepiction, depict, cast `render`. `[...rep.geometry.elevation, ...rep.geometry.front, ...rep.geometry.plan]` spread `undefined`: the in-progress front-view change (schema line 282 makes `front` required, assets such as `standing-adult` do not author it yet), and `plan` is also absent on some assets.
- Same hazard at `S/🔨️modules/🖨️representation/🪞️copies/🟦️.ts:30` (`rep.geometry[viewpoint].map`).
- `VIEWPOINTS is not defined`: no longer reproducible; `VIEWPOINTS` is imported from `S/🧬️schema/🟦️.ts` in every module that uses it (representation, copies, cast schema, cli, preview, check). It was a transient state of a concurrent edit.

## Fix
- `representation/🟦️.ts:59`: `...(rep.geometry.front ?? [])`, `...(rep.geometry.plan ?? [])`.
- `copies/🟦️.ts:30`: `(rep.geometry[viewpoint] ?? []).map`.
- Regression test: `S/🧪️tests/📏️print-suitability/🟦️.ts`, "measures a representation that authors no front or plan geometry".
- Not changed: `catalog/🟦️.ts:272-275` (validator requires non-empty front, the front-view owner's decision) and `evaluation/🟦️.ts:509-510` (receives `?? []` normalised geometry).

## Verification
- Full manifest export (all 98 casts) through `🧰️eb/stack.ts`: `ok 98`.
- Single casts shared-person-winter, shared-radiator-winter, shared-person-fanning: ok.
- Staffage vitest (q2 alias config): 1148 pass, 229 fail, 8 files. Failures are in asset-validation (64), three-views (149), schema-conformance, representation-copied, front-projection, dxf-reopen, continuous-rotation, smooth-continuity: `validateAsset` catalog/🟦️.ts:272 `Cannot read properties of undefined (reading 'length')` on fixtures without front. This is the front-view rollout of another agent (fixtures and validator disagree), not the export.
- Python visuals suites (9 modules incl. staffage_only), 87 tests, 38 errors, none is the spread error:
  - 22: `unsupported SVG paint '#NaNNaNNaN'` (`🐍️.py:500` from a cast frame; NaN colour written by the exporter, a thermal/palette edit in progress, open).
  - 11: `Staffage export failed: operatingKeyOf is not defined` / `frames is not defined` from a concurrent edit of `🌡️thermal/🟦️.ts` (transient; the same casts export fine on retry).
  - 5: `Set SEMIO_TEACHING_LANGUAGE to en or de` (runner needs `SEMIO_TEACHING_LANGUAGE=en`).

## Open
- Rerun the visuals suites with `SEMIO_TEACHING_LANGUAGE=en` once the thermal edit settles; investigate `#NaNNaNNaN` if it persists.
- The front-view owner must reconcile the catalog validator and fixtures with optional `front`.
