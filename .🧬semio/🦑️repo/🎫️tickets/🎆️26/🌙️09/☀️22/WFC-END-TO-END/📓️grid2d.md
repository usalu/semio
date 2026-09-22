# 🀄️ Grid2d fill slice

## Result

`cargo test -p semio-s-artifact-wfc-grid2d --features component-app-assembly --lib -j 4 -- --test-threads=4` with `RUST_MIN_STACK=33554432`:

```
test result: ok. 222 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
```

## What landed

- Non-mutating `fill` tool (`s.wfc.grid2d.fill.run`): puzzle-style shell, energy-style payload channel, `ToolRunTraceKind::None`.
- Live preview paints the tick payload (partial cell assignments from `WfcJob::observed()` / `preview()`), not the truncated `incomplete_grid` publication.
- Completion dispatches `commit-fill` → `Grid2dWindowConfig.solve_json`. Abort clears pending finish and never writes `solve_json`. Editor transient stays `NoTransient` (no `SetSolve`).
- Child `WfcJob` is bound to the framework operation/generation. Tick publish is deferred one step after the child’s Preview/Checkpoint/Complete so the payload ledger is not double-admitted in the same `drive_step`.
- Edit mode lists `fill` only via `.mode_tools` (`ModeDefinition.tools` stays empty). `solve` starts the fill run.

## Tests added / closed audit MISSING rows

- Fill: progressive partials, abort without commit, final payload ≡ `solve_with_job` (pipes).
- Language-agnostic partial vector + JSON Schema.
- Solve → `solve_json` via `commit-fill`; 25-byte inference preview; dedicated json/txt serializer tests.
- Preview: partial render ≠ empty ≠ finished.
- Did not duplicate pipes/terrain live outcome tests.

## Owns

Only `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d`.
