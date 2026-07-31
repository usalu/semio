---
name: Arbitrary Brep Cut Fuse
overview: Remove the torus-specific boolean guards in the brepkit wrapper so the kernel performs arbitrary cut/fuse/intersect through brepkit's general boolean engine, then verify the sphere-cut-with-torus fixture actually produces a cut solid.
todos:
 - id: reopen-ticket
   content: Reopen ticket 2026/06/09/ASYNC-FLOW-EVAL-AND-COMPUTING-NODE-CHROME
   status: completed
 - id: remove-guards
   content: Remove torus guards + SolidSource machinery in geometry/brep/brepkit/lib.rs; simplify register_solid and translate_sync
   status: completed
 - id: build-wasm
   content: Rebuild flow/core WASM via nx wasm @semio-tech/flow-core
   status: completed
 - id: verify-fixture
   content: Run eval-sphere-cut-fixture.ts and confirm brep_bool_cut_5 yields a geometry handle without hanging
   status: completed
 - id: run-tests
   content: Run brepkit wrapper + flow/procedural tests for regressions
   status: completed
 - id: handle-hang
   content: If brepkit hangs, report external brepkit root cause instead of re-adding guard
   status: completed
isProject: false
---

# Arbitrary Brep Cut & Fuse in the Kernel

## Root cause

The sphere fixture produces nothing because [geometry/brep/brepkit/lib.rs](geometry/brep/brepkit/lib.rs) hard-rejects any cut/intersect whose operands include a torus with overlapping AABBs. The error surfaces as `brep_bool_cut_5.error = "boolean cut with intersecting torus is not supported yet"`. This is a wrapper-level guard, not a brepkit call result.

The `SolidSource` enum, `Entry.source`, `entry_source`, `solid_bounds_overlap`, the `source` param on `register_solid`, and the `translate_sync` source branch exist _only_ to feed these guards.

## Changes in [geometry/brep/brepkit/lib.rs](geometry/brep/brepkit/lib.rs)

- Delete the guard blocks in `cut_sync` and `intersect_sync`; both should just call `boolean(&mut self.topo, op, a_id, b_id)` and register the result.
- Remove the `SolidSource` enum and the `source` field from `Entry`.
- Remove `entry_source` and `solid_bounds_overlap` (no remaining callers).
- Simplify `register_solid` to drop the `source` parameter; update all call sites (`box`/`sphere`/`cylinder`/`cone`/`torus`/`fuse`/`cut`/`intersect`/`mirror`/`fillet`/`chamfer`).
- Simplify `translate_sync` to the single in-place path (mutate via `transform_solid`, return `shape.clone()`), matching `rotate_sync`/`scale_sync`. The torus-specific re-register branch goes away.
- Keep code organized within the existing `#region` structure.

## Build & verify (execution, not plan mode)

- Rebuild WASM: `nx wasm @semio-tech/flow-core` (runs `bun ./📜️script.ts wasm` in `flow/core`).
- Run the existing ticket eval script to confirm the cut succeeds and completes fast:
  `bun .repo/🎫️/26/06/09/ASYNC-FLOW-EVAL-AND-COMPUTING-NODE-CHROME/eval-sphere-cut-fixture.ts`
  Expect `brep_bool_cut_5.out.out` to be a `geometry` handle (no `error`) and elapsed well under the 30s worker timeout.
- Run brepkit wrapper + flow/procedural tests to confirm no regressions.

## Risk / contingency

brepkit `d470b7c` (2.101.3) has torus-aware boolean paths but no torus boolean tests, and the prior ticket reported a hang on this exact intersecting sphere+torus. The 30s timeout + worker restart in [flow/worker-client.ts](flow/worker-client.ts) stays as a safety net.

- If brepkit returns a clean result: done.
- If brepkit errors but does not hang: surface the real brepkit error (no wrapper guard) and report it.
- If brepkit genuinely hangs: the true root cause is inside the external brepkit kernel. I will stop and report this (it requires a brepkit-level fix / version bump, not a wrapper guard) rather than silently re-adding the guard.

## Ticket

Reopen `2026/06/09/ASYNC-FLOW-EVAL-AND-COMPUTING-NODE-CHROME` (it introduced the guard) and record this follow-up there.
