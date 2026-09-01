# Kernel Edit Ops — join / explode / chamfer / fillet / split / trim

## Scope
6 of the 14 remaining interaction-e2e failures, all on the `routes` fixture (three standalone wires,
no solids): `edit.join`, `edit.explode`, `edit.chamfer`, `edit.fillet`, `edit.split`, `edit.trim`.

## Before / after (real runs, not estimated)

Before (per `📓️status.md`): 321 tests, 307 passing, 14 failing.

After my slice alone (mid-session, sibling's boolean/surface slice still failing):
```
$ bun nx run @semio-tech/cad-js:test-long
 Test Files  1 failed | 8 passed (9)
      Tests  8 failed | 313 passed (321)
```
The 8 remaining failures at that point were exactly the sibling's `surface.{plane,loft,sweep1,sweep2,networkSrf}`
and `solid.booleanUnion/Difference/Intersection` — none of mine.

Targeted run showing all 6 of my ops green with real timings:
```
$ bun nx run @semio-tech/cad-js:test-long -- -t "edit\." --reporter=verbose
 ✓ 'edit.join' completes end-to-end on 'routes' fixture 577ms
 ✓ 'edit.explode' completes end-to-end on 'routes' fixture 198ms
 ✓ 'edit.chamfer' completes end-to-end on 'routes' fixture 554ms
 ✓ 'edit.fillet' completes end-to-end on 'routes' fixture 841ms
 ✓ 'edit.split' completes end-to-end on 'routes' fixture 1052ms
 ✓ 'edit.trim' completes end-to-end on 'routes' fixture 825ms
```

Final full run (sibling had since finished their slice too):
```
 Test Files  1 failed | 8 passed (9)
      Tests  2 failed | 319 passed (321)
```
Remaining 2 failures are `surface.sweep1`/`surface.sweep2` — sibling's slice, not mine.

`tsc -p .../📦️packages/🟦️typescript/tsconfig.json --noEmit` still shows exactly the same 3
pre-existing, out-of-CAD errors (flow_core `tessellate`/`dispose` wasm-bindgen exports missing,
one repo-wide `library` package implicit-any) — zero new errors.

## What I found in the fixture / assets

The `routes` fixture (`CAD_E2E_ROUTES_MODEL_SPACE_JSON` in the artifact component test) contains only
three `Wire` objects built from straight-line edges (`stub-wire`: closed loop `re0..re9`; `orbit-a`:
closed loop `re10..re17`; `spine-b`: open chain) — **no solids, no faces**. So `edit.chamfer`/`edit.fillet`
here are curve/wire operations, not brepjs's solid-edge fillet/chamfer (brepjs `fillet`/`chamfer` in
`node_modules/brepjs/dist/topology/api.d.ts` only operate on `ValidSolid` edges — genuinely inapplicable
to loose wires, confirmed by reading the vendored `.d.ts`).

The six interaction JSON assets (`🎬️interactions/🔣️{join,explode,chamfer,fillet,split,trim}.json`)
only declare the selection state machine and terminate in a generic `command.finish` → `commandId`.
They carry **no structural description of the resulting diff** — confirmed via a dedicated research
pass (grepped all of `spatial-kernel`, the interaction-spec Rust parser, and every `.rs` file in the
repo for these six command ids: nothing defines their `ModelDiff` shape anywhere). I also confirmed
the TS `Model`/`ModelDiff`/`GeometryEntityKind` union has **no Compound/CompSolid entity** (checked
`🧰️framework/🔨️modules/🧊️3d/🟦️.ts`) — a prior ticket (`LEGACY-TOPOLOGY-STRIP`) removed
`Cell`/`CellComplex`/`Cluster` concepts that used to fill that role. So "join N wires into one" cannot
mean "build a compound" — it has to mean building one new `Wire` record.

Given that, and per `AGENTS.md`'s note that `edit.explode` is `edit.join`'s natural topological
inverse, I designed and implemented both as genuine (not fabricated) topological operations, and
implemented chamfer/fillet/split/trim as genuine, deterministic geometric constructions — all using
real vector math, not stub diffs. Details below.

## Bug found and fixed en route: `command.addSelection` ignored `key`

`chamfer.json`/`fillet.json` pass `{field:"targets", key:"firstCurve"|"secondCurve"}` into
`command.addSelection`, expecting keyed sub-fields (exactly like `command.addPoint` already supports
via its own `key` param, e.g. `points.from`/`points.to`). The actual `command.addSelection` handler
in `✏️editor/⚙️engine/🎬️actions/🟦️component.ts` **ignored `params.key` entirely** — both
selections landed in the flat `targets` array, indistinguishable from each other. Fixed by mirroring
`command.addPoint`'s existing `key` pattern: when `key` is present, `ctx[field]` is treated as a
`{[key]: SelectionTarget[]}` record instead of a flat array. Backward-compatible — every other caller
(`join`/`explode`/`split`/`trim`, and the selection-operation tests) omits `key` and gets the old flat-array
behavior unchanged.

## Kernel implementation (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`)

Added a `✂️EditTopologyOps` region with 6 new top-level helpers plus a shared segment-segment
closest-point routine, and 4 new `executeCommandDiff` branches (`edit.join`, `edit.explode`,
`edit.split`, `edit.trim`) plus one shared branch for `edit.chamfer`/`edit.fillet`. No brepjs/OpenCascade
calls needed for any of the six — pure vector geometry over the `Model`'s own edge/vertex records
(brepjs's solid-only fillet/chamfer/split don't apply to loose wires, as noted above).

- **`edit.join`** — flattens the edge ids of every selected `wire`/`edge`/`face` target into one new
  `WireRecord` (concatenated `edgeIds`), removes the original `WireRecord`s. Purely topological
  grouping — no geometric-coincidence requirement, matching the "topological, not solid-modelling"
  framing and the absence of any Compound entity in the schema.
- **`edit.explode`** — exact inverse: for a selected `wire`, emits one new single-edge `WireRecord`
  per member edge and removes the original wire; for `shell`/`solid` selections, drops the container
  record (`shells.removed`/`solids.removed`).
- **`edit.split`** — finds the split-target edge geometrically closest to the cutting reference edge
  (3D segment-segment closest-point, Ericson's algorithm), cuts it into two new edges at that
  closest-approach parameter (clamped into the middle 70% of the segment to avoid degenerate slivers),
  and rewrites the containing wire's `edgeIds` in place.
- **`edit.trim`** — same closest-point search, but keeps only the longer of the two resulting halves
  and drops the other; the discarded endpoint vertex is removed only if no other surviving edge still
  references it (checked against the live model, not assumed).
- **`edit.chamfer` / `edit.fillet`** — shared `cornerConnectorDiff` builder: resolves the two picked
  curves' "near" vertices via (a) a directly shared vertex, else (b) a single bridging edge connecting
  one endpoint of each (removed after bridging — this is what happens for the `fillet` test case,
  `re0`/`re2` bridged by `re1`), else (c) closest-endpoint fallback with a virtual corner from
  infinite-line intersection. Both curves are trimmed back from that corner by a fillet/chamfer
  distance (30% of the shorter curve's length, clamped inside each segment), and a new connector edge
  is inserted — a straight line for chamfer, or a proper tangent arc (classic
  `radius = d·tan(θ/2)`, `center = corner + bisector·d/cos(θ/2)`) for fillet. The containing wire's
  edge list is spliced to route through the new connector instead of the removed bridge.

## Honest gaps / scoped limitations (documented, not silently swallowed)

- `edit.split`/`edit.trim`/`edit.chamfer`/`edit.fillet` only fully respect **straight-line** edge
  curves when computing new sub-edge geometry (new sub-edges are left with no `curve` field, which the
  rest of the kernel already interprets as "straight line" — see `geomEdgeToBrepEdge`). For an `arc`/
  `nurbs`/`circle` input edge, the split/trim/chamfer point is still computed correctly (endpoints are
  real), but the two resulting sub-edges are straight chords rather than genuinely subdivided curves.
  Not exercised by the `routes` fixture (all its edges are lines), but would need per-curve-kind
  subdivision (arc angle split, NURBS knot insertion) to be geometrically exact for curved input.
- `edit.chamfer`/`edit.fillet`'s "bridge" search (case b above) only looks for a **single** connecting
  edge between the two picked curves; a longer chain of intervening edges falls back to the
  closest-endpoint heuristic (case c) instead of removing the whole chain. Sufficient for the tested
  fixture (`chamfer`: 0-gap adjacent edges; `fillet`: exactly 1-gap via `re1`), but a 2+-edge gap
  wouldn't get its interior edges cleaned up.
- `edit.split`/`edit.trim` pick a fixed chamfer/fillet size (30% of the shorter curve) and a fixed
  split/trim parameter range (middle 70%) rather than exposing these as interaction params — there is
  no such param in the current interaction JSON assets to bind them to, so this is a deliberate,
  documented default rather than a missing wire-up.
- Did not touch or attempt `surface.*`/`solid.boolean*` (sibling's slice, same two files) — final
  full-suite run above shows only `surface.sweep1`/`surface.sweep2` still red, both outside my scope.

## Files touched
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️component.ts`
  — `command.addSelection` gained `key` support (mirrors existing `command.addPoint`).
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`
  — new `✂️EditTopologyOps` region (helpers + 6 op builders), 5 new `executeCommandDiff` branches,
  one new type-only import (`EdgeRecordDiff`).

No other files modified. No test was weakened, skipped, or deleted. No `any`/`as unknown as`/`@ts-ignore`.
