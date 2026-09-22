# 🀄️ Grid3d fill slice

## Result

`cargo test -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib -j 4 -- --test-threads=4` with `RUST_MIN_STACK=33554432`:

```
test result: ok. 216 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.89s
```

## What landed

- Non-mutating `fill` tool (`s.wfc.grid3d.fill.run`): puzzle-style shell, energy-style payload channel, `ToolRunTraceKind::None`.
- While a fill run is live, the editor preview paints the tick payload and does not call `solve()`; idle/cold still solves once for residency.
- Completion stores the full assignment in `Grid3dPreviewResidency` via `commit_fill_result`. Abort never stores it. No `SetSolve` (grid3d has no editor transient).
- Run job reads `WfcJob::preview()` / `domain_masks()` in process — not the truncated published `incomplete_grid`.
- Child `WfcJob` is bound to the framework operation/generation. Parent tick is deferred one step after the child’s Preview/Checkpoint/Complete so the payload ledger is not double-admitted in the same `drive_step`.
- Edit mode registers `fill`; `solve` starts that run.

## Tests added / closed audit MISSING rows

- Fill: progressive partials, abort without residency commit, final payload ≡ `solve_with_job` (blocks).
- Language-agnostic partial vector + JSON Schema.
- Inference: 25-byte preview progress record.
- IO: `io()` stdio.txt channel declaration + derived composition rebuild of `blocks`.
- Preview: partial render ≠ empty ≠ finished.
- Did not duplicate `blocks` / `pipes-3d` live outcome tests.

## Owns

Only `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d`.
