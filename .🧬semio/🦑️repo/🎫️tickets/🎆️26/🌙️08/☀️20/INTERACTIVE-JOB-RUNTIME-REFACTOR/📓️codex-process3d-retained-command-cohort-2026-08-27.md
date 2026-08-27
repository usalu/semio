# Process3d Retained Command Cohort

## Outcome

The Process3d command surface is now partitioned honestly: 11 of 33 routes are `Migrated`, and 22 remain `BatchOnlyPendingRewrite`. The migrated cohort has 3 bounded routes and 8 resumable config routes. No migrated resumable route performs a scan followed by the monolithic command reducer.

## Migrated Routes

- Bounded, `Config`: `engagementAbort`, `setCamera`
- Bounded, `HostOnly`: `loadModelRequest`
- Resumable, `Config`: `setActiveUtility`, `engagementInput`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `setLocale`, `setContributions`

The resumable cursor owns its tool identity, extent, byte cursor, digest, completion, and retirement state. Each progress turn observes at most one 256-byte slice and checkpoints every step. The shared retained-command job owns per-operation cancellation and freshness validation before publication. Config publication now has an app-owned one-item preparation factory that validates operation, generation, and base revision, retains the exact base, prepares forward/inverse edits under live Store authority, and retires base, mutation, prepared output, description, and authority incrementally.

## Batch-Only Routes

`setSnapshot`, `setActiveExample`, `addStep`, `addWorkshopMachine`, `removeWorkshopMachine`, `updateWorkshopMachine`, `removeStep`, `removeSelectedStep`, `moveStep`, `updateStep`, `setStepEnabled`, `setStock`, `patchInspector`, `setCursor`, `stepCursor`, `stepCursorBack`, `stepCursorForward`, `engagementSubmit`, `worldPointerDown`, `worldFaceDragEnd`, `importModelFile`, and `exportModel` remain fail-closed. Their document, geometry, history, snapshot, or media work still needs bounded preparation ownership; labeling them migrated would recreate the r10 scan-then-monolith defect.

## Validation

The scoped Bun verifier passed with `routes=33`, `migrated=11`, `bounded=3`, `resumable=8`, `batchOnly=22`, and `scanThenMonolith=0`. It validates the language-neutral fixture with strict Ajv, independently recomputes the byte-cursor extent boundary, compares command rows, manifest labels, route constants, proof rows, and exact publication lanes, checks the Config preparation/freshness/retirement source witnesses, and rejects hostile activation, forged publication, and count drift. Evidence: `🧪️codex-process3d-retained-route-audit-2026-08-27.txt`.

The coordinator-owned official r12 run observed the repository-wide scan-then-monolith count fall from 53 to 27 and the remaining-route count fall from 636 to 597 while this cohort was joined. The scoped fixture remains the authority for the exact Process3d partition above.

The Rust timing/checkpoint/retirement tests were updated but not compiled or executed because the Flow cohort owns the exclusive compiler lease. No Cargo, Nx, formatter, rustfmt, compiler, or repository-wide verifier was run.

## Remaining Blockers

The 22 BatchOnly routes need domain-specific retained preparation machines that avoid cloning or traversing unbounded Process3d snapshots in a poll. A compiler-owned validation of the updated Rust tests remains outstanding; r10/r11 are pre-change baselines and therefore still list the old Process3d scan-then-monolith rows.
