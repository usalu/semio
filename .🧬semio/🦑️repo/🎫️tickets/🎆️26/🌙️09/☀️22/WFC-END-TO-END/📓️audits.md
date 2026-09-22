# 🀄️ Audit corrections (2026-09-22)

Sources: puzzle fill, engine job, and the five artifact audits. Facts only.

## Puzzle 3d fill shell

Framework actions, already injected for any tool run: `toolRunStart`, `toolRunPause`, `toolRunResume`, `toolRunStep`, `toolRunAbort`, `toolRunFinalize`, `toolRunDismiss`. Pause stops scheduling. Step while paused drives one unit of fuel. Abort retires provisional work and must not commit. Dismiss is only from a terminal state.

Puzzle fill itself is document-mutating (`mutating: true`, `ToolRunTraceKind::Instance3d`, revalidate job, provisional `create_object` ops). WFC does not copy that mutation path. It copies the shell. The non-mutating precedent is energy simulation: `mutating: false`, `ToolRunTraceKind::None`, window reads `ToolRunView::payload`.

One engine `WfcJob::step` is one solver sub-unit, not one collapsed cell. A preview is published on every `CommitSlot` and on a cadence during earlier stages. `WfcPreview.incomplete_grid` is a partial assignment capped at 256 cells. `WfcJob::preview()`, `observed()`, and `domain_masks()` expose the in-process state before commit.

## Per artifact

### bitmap
Editor `solve` is one retained step: `CompleteWithEphemeral` → `solve_transient` → `solve_with_job`. Output window paints `BitmapTransient`. `contradiction` is not read by render; an unsatisfiable solve looks like an empty canvas. Examples `rooms-16` and `flowers-24` already have live solver tests. No in-progress collapse test. MISSING: inference checkpoint request, contradiction-specific render, direct txt/json IO calls.

### grid2d
Editor transient is `NoTransient`. Solve starts the non-mutating `fill` tool run; completion writes `Grid2dWindowConfig.solve_json` via `commit-fill` (`Complete`, not `CompleteWithEphemeral`). While the run is live the preview paints the tick payload (partial cell assignments from `WfcJob::observed()`); abort leaves `solve_json` untouched. Examples `pipes` and `terrain` already solve live — do not duplicate those outcome tests. Closed by fill unit tests plus the solve→`solve_json` path, the 25-byte inference preview, and dedicated stdio serializer tests in this slice.

### wfc2d
`SetSolve` transient holds slot assignments. Preview already paints a partial map (assigned slots filled, others outlined). Editor solve still drains `solve_with_job` in one step. Examples `two-room-corridor`, `wall-roof-facade-strip`, `hex-ring`, `terrain-ring` already solve live. MISSING: tests under `🚪️io/`.

### grid3d
Edit mode registers the non-mutating `fill` tool; `solve` starts that run. While a fill run is live, the editor preview paints the tick payload and does not call `solve()`. On completion the full assignment is stored in `Grid3dPreviewResidency` (abort leaves it untouched). The run job reads `WfcJob::preview()` / `domain_masks()` in process. Examples `blocks` and `pipes-3d` already match committed outcomes — do not duplicate those live outcome tests. Closed by fill unit tests plus inference preview and `io()`/composition tests in this slice.

### wfc3d
Editor and viewer call `solved_transient()` during render, which solves synchronously. Instances exist only for assigned slots, so a partial map can already draw. Examples `two-room-corridor`, `wall-roof-facade-strip`, `tower-stack` already have live outcome tests. MISSING: a transient unit test, tests under `🚪️io/`. No in-progress collapse test.
