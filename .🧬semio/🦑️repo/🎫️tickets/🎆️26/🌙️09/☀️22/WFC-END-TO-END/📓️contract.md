# 🀄️ WFC end to end — execution contract

Ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️22/WFC-END-TO-END/` (`$T`).
Repo: `/Users/ueli/Documents/semio`. Plugin: `✏️s/🔌️plugins/🀄️wfc`.

Repo MCP was not connected when this ticket was opened. Append progress to `📓️status.md`. Scratch logs go in `$T/🗑️generated/<slice>/` and are deleted when the slice is done. Reports stay.

Other agents edit this repo at the same time. Re-read a file immediately before editing it. Make small targeted edits. Do not revert unrelated changes. Do not run modifying git commands.

## What is already true

The shared engine job (`⚙️engine/💼️job/🦀️.rs`) is already an `InteractiveJob`:

- Stages: `InitializeDomains`, `FindMinimumEntropySlot`, `ChooseCandidate`, `PropagateCompatibilityEdge`, `DetectContradiction`, `BacktrackTrailEntry`, `CommitSlot`, `MaterializeCheckpoint`, `MaterializeCommit`, `Complete`.
- `WfcPreview.incomplete_grid: Vec<Option<u32>>` is the partial assignment.
- `step` honours `context.is_cancelled()` and returns `StepOutcome::Cancelled`.
- Preview, checkpoint, commit, and the `wfc-unsatisfiable` fault are already published.

Each artifact editor's `Solve` command ignores that. Bitmap (`✏️editor/🦀️.rs`, `BitmapCommandWork::step`) matches `Solve` to a single `CompleteWithEphemeral` that calls `solve_transient` → `solve_with_job`, which drains the child job to the end and paints only the finished transient. The other four artifacts follow the same one-shot shape. Grid3d's edit mode says the solve is a routed inference and declares no tool run.

## What interactive means

Puzzle 3d fill is a framework tool run. The user starts it, the viewport paints provisional results while it runs, and the framework panel supplies pause, resume, step, and abort. The relevant registration is `.tool(fill_tool::definition(...))` plus `.mode_tools(...)` in the puzzle 3d editor, and the viewport reads `doc.tool_run()` while a run is live.

WFC's collapse is not a document edit. Bitmap's editor already states that law: the snapshot has no field for the collapse, and writing one would lie about what the document persists. Energy's simulation is the non-mutating tool-run precedent: `mutating: false`, `trace: ToolRunTraceKind::None`, each tick's `ToolRunTick::payload` is what the window reads through `ToolRunView::payload`, and the same framework panel drives the run.

So each WFC editor grows a **fill** tool that uses puzzle 3d's interaction shell and energy's non-mutating payload channel. The preview window paints `incomplete_grid` (decoded into that artifact's own output) on every tick, the way puzzle 3d paints provisional placements.

## Fill tool (every artifact, same shape)

| | value |
|---|---|
| tool id | `fill` |
| mutating | `false` |
| rebase | `Restart` |
| reconfigure | `Restart` |
| trace | `ToolRunTraceKind::None` |
| revalidate job | none |
| run job | `s.wfc.<ident>.fill.run` |

`<ident>` is `bitmap`, `grid2d`, `wfc2d`, `grid3d`, `wfc3d`.

Stages are the engine stage ids (`wfc.initialize-domains` … `wfc.complete`), labels English then German. Counters: `observations`, `decided` (cells or slots whose assignment is `Some`), `backtracks`. Reasons: `collapsed` (success), `contradiction` (the unsatisfiable answer, not a fault), `cancelled`, `fault` (the job could not run).

`windows` lists the preview window kind so each tick dirties that body.

Registration follows the artifact editor's existing app builder, the way energy registers `.tool(tools::simulation::definition())`. The edit mode lists `fill`. The existing `solve` action starts this run instead of draining a headless job. `solve_with_job` stays as the deterministic oracle the finished run must match.

## Run job

One framework step does one engine `WfcJob::step` (preparation stages before the child exists are also one unit per step: build model, build topology, apply pins). Do not drain to commit inside a step.

Each preview or commit tick writes a payload the preview window can render:

- bitmap: partial output pixels (undecided cells keep a documented empty index), width, height, `contradiction`, `done`.
- grid2d / grid3d: partial assignments (`x,y` or `x,y,z` → tile id or absent), `contradiction`, `done`.
- wfc2d / wfc3d: partial slot assignments (slot id → tile id or absent), `contradiction`, `done`.

On commit, also publish the existing `SetSolve` transient so the result remains after the run is dismissed. A contradiction is a successful answer with `contradiction: true` and no pixels, matching today's `solve_transient`. A job that cannot run is a fault.

Cancellation is the engine's `StepOutcome::Cancelled`. Pause and resume are the framework tool-run ledger; the job must be retargetable across them without restarting the collapse unless the document or seed changed (`Restart` policies cover that).

## Preview windows

While `tool_run` is non-terminal and its payload decodes, the preview draws that payload. A partial payload must be visually different from both the empty buffer and the finished collapse. The graph, input, and grid-authoring windows stay as they are.

The finished cache is whatever that artifact already uses. Do not invent `SetSolve` on an artifact that does not have it.

| artifact | live partial | finished cache already on disk |
|---|---|---|
| bitmap | output window reads the tick payload (`decided` mask; palette index 0 stays a real colour) | `BitmapTransient` / `SetSolve` |
| grid2d | preview reads the tick payload, not `solve_json` | `Grid2dWindowConfig.solve_json` (`Grid2dInferenceCommit`). Editor transient is `NoTransient`. Window transient is hover only. |
| wfc2d | preview already paints whatever `assignments` the transient holds, including a partial map | `Wfc2dTransient` / `SetSolve` |
| grid3d | preview must stop calling `solve()` inside `render` while a run is live, and draw the tick payload | `Grid3dPreviewResidency` (last full assignments). There is no editor transient. |
| wfc3d | preview must stop calling `solved_transient()` (a synchronous full solve) while a run is live | `Wfc3dTransient` written when the run completes, not on every render |

Abort leaves that finished cache untouched.

The run job owns the child `WfcJob` and reads `WfcJob::preview()` (or `observed()`) in process. Do not paint from the published `incomplete_grid` alone: that list is capped at 256 cells and can set `truncated`.

`mutating: false` makes the framework begin finalize when the job completes, so the finished cache write happens in that completion step, before dismiss clears the tick payload.

## Audit corrections

Read `📓️audits.md` in this ticket. Terminal example solves and mutation outcomes are already tested. Add the in-progress fill tests from this contract, plus a test for each MISSING row that belongs to your artifact. Do not duplicate a test that already asserts the finished solve.

## Outputs that must be tested

For every artifact, each of these has a test that fails if the output is wrong:

1. Every example's finished collapse (existing outcome tests stay, and any example without a live solver assertion gains one).
2. The fill run's final payload equals `solve_with_job` for the same snapshot and seed.
3. Stepping the run publishes at least one partial payload whose decided-cell count is strictly between 0 and the finished count, and a later step has a greater decided count (or the problem is smaller than two cells, in which the test uses a fixture that has room to collapse).
4. Aborting mid-run yields `Cancelled` and does not publish `SetSolve`.
5. The preview render of a partial payload differs from the empty render and from the finished render.
6. Inference commit fields, io channels, and mutation outcomes that an audit marks MISSING get a test in the same change. Do not delete a passing test to make a gap look closed.

Language-agnostic: the tick payload is a JSON object with a normative JSON Schema leaf. A Python oracle (stdlib only) checks the partial-assignment record shape against one fixture vector. A Rust test reads the same vector.

## Gates a slice runs before it reports done

From the plugin directory conventions already used by the extract ticket:

- `cargo test -p <artifact-crate> --features component-app-assembly --lib -j 4 -- --test-threads=4` with `RUST_MIN_STACK=33554432`.
- The new fill tests are in that run and pass.
- Do not claim a test passed without reading the `test result:` line.

One cargo invocation at a time. Capture stdout and stderr under `$T/🗑️generated/<slice>/` and delete those logs when the slice is done.

## Slice ownership

| slice | owns |
|---|---|
| bitmap | `🗿️artifacts/🖼️bitmap` fill tool, preview paint, tests |
| grid2d | `🗿️artifacts/🔲️grid2d` |
| wfc2d | `🗿️artifacts/◻️2d` |
| grid3d | `🗿️artifacts/🧱️grid3d` |
| wfc3d | `🗿️artifacts/🧊️3d` |

No slice edits `⚙️engine` unless a missing accessor blocks every artifact, and then it adds the accessor without changing solve results. No slice edits another artifact's folder. Plugin-root registration (app manifest `.tool`) is done inside the artifact crate's `create_*_editor` when that function already lives in the artifact crate. If the tool can only be mounted from `🀄️wfc/🦀️.rs`, the slice that needs it edits only its own match arm and re-reads the file first.
