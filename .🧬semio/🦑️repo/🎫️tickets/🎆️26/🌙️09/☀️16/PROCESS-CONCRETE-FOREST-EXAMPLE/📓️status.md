# Status: Process Concrete Forest Example

**Status:** closed 2026-09-16
**Goal:** Add the concrete forest example to process and apply steps using all machines available for concrete.
**Bookkeeping:** manual on disk — repo MCP failed to connect (connection closed) for the whole session.
**Report:** [concrete-forest-example-2026-09-16.md](./📓️concrete-forest-example-2026-09-16.md)

## Outcome
`process3d` ships `📚️examples/🌲️concrete-forest`: the real hexagonal-cut concrete forest piece (exact STEP, shipped
as `WorkingSolid::Reference` / `REFERENCE_SOLIDS`) with one kernel-replayed step per concrete-catalog machine — wall saw,
diamond saw, wire saw, core drill, rotary hammer, anchor setter, surface grinder. The react shell's picker offers Demo /
Concrete Forest; the load commits, the Artifact panel lists the seven steps, the workpiece renders the replayed piece and
toggling a step re-replays it (browser evidence in the report).

## Fixed on the way
1. STEP reader: optional `AXIS2_PLACEMENT_3D` slots, collinear B-splines read as lines (wrong volume before), p-curves on
   every imported coedge (no boolean ever succeeded on a STEP body before).
2. Validator: shared-vertex false positives in the self-intersection probe; sliver probe cost (8 s → 1 s); a chord-segment
   floor so coarse tolerances never collapse small circles.
3. process3d whole-document load path (`setActiveExample`): envelope Drop trap, missing `SemioMembers` roster +
   `genesis_process3d_child_pack`, publication lease never admitted outside tests.

## Left open (documented in the report)
- Kernel: through-holes across non-rectangular prisms, coincident-wall fuses, translation-sensitive booleans,
  ~1 s validation per boolean on the piece.
- process3d pre-existing red laws unchanged (render-harness serialisation, wasm bridge retirement factory, four
  publication/dispatch laws, the 200 000-turn maintenance-swap law); `setCamera`/`engagementAbort` fail the batched
  fixed-fold contract on the react lane.
- Lane ops: every framework-crate edit rebuilds ~all wasm plugins; vite on 6222 wedges on peers' registry regen
  (recycle by pid); deadlocked wasm-dev cargos had to be killed once.
