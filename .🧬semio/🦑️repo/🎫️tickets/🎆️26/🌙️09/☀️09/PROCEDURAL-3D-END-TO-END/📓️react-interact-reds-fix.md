# 🎫️ React `interact` reds: Rectangle Wire Preview select/inspector, Hexagonal Mushroom Column inspector

Status: root cause of the wire `select` red NOT reproduced; no product defect found in source; probe hardened; one focused test added.
(No browser run was possible: the serve needs an activation receipt/staged wasm and the Rust build image is full; the repo test graph also fails on a missing generated `🌊️flow/🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts`.)

## Findings from source (current tree)
- Hexagonal Mushroom `inspector`: probe-only race (Inspection tab not mounted when the first example is measured, `rows: []`). The probe already opens the panel by result and polls/re-opens (`inspectionRows`, 12x poll). Not a product bug.
- Wire `inspector` follows from `select`: widget is derived from `activeObjectId ?? selectedIds[0]`; empty selection gives empty widget. Rust `PreviewInteractionMarks::graph_selection_ids` resolves `rect@wire` to widget `rect` correctly.
- Wire `select`: the mounted curve-only path is correct. `WorldInstanceNode` renders the wire as a `lineSegments` whose `onPointerMove` (hover) and `onClick` (select) both use `world3dLinePick(..., isCurveOnly && meshReachable)` and dispatch the identical `interactionHover`/`interactionSelect` target `{handle, rect@wire}` with merge `replace`. Pinned by the new test (passes).
- Hypothesis in the sweep note (picker builds hit geometry from `positions` only) is false: `world3dInstanceLocalCorners`/`meshBoundsCorners` fall back to `edgePositions`; the line raycast uses the edge geometry.
- Native delivery tests (`wgpu-selection-roundtrip-2026-09-15.md` §4.2) already show guest select of `rect@wire` publishes `["rect@wire#0"]`.
- Suspect left (unproven): the probe clicked at `Math.round`ed coordinates while the hover was measured at the fractional grid point; a thin line target (slop 0.12 world units, fixed) can differ by sub-pixels, and a missed R3F click falls to `onPointerMissed -> handleEmptyClick` which clears when no hover is retained (after the host pointer-up coarse AABB pick selected first). Probe now clicks the exact hovered point.

## Changes
- `🐍️interaction-matrix-probe.mjs`: `sweepForHover` returns the unrounded point.
- `🧰️framework/.../🌐️World3dHost/🧪️tests/🧩️component/🟦️.tsx`: new test "selects a curve-only wire through the same mounted line the hover answers".

## Test result
`vitest` (policy-driven, level long) `World3dHost/🧪️tests/🧩️component -t "curve-only wire"`: 1 passed.

## Needs a browser check
Run `interact` on a served build; if `Rectangle Wire Preview · select` is still red, capture `data-guest-selection-json` vs `data-selection-json` after the click and the dispatched actions (the pointer-up coarse select followed by `onPointerMissed` clear is the first thing to confirm).
