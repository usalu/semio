# 🧹️ Wave W0-C2: plugin progress-measure removal

Status: **landed**. The puzzle 3d, 2d and 5d crates compile again after W0-C removed `WindowMeasure::Progress`,
`MeasureProgressStep` and `MeasureProgressStepKind`. No compatibility shim was added and the removed types were
not re-added. No framework files were touched.

Paths are relative to `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/`. `E3`, `E2` and `E5` stand for
`<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 1. What changed

### Puzzle 3d fill: `🧊️3d/…/E3/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`

- **Removed:** `FILL_PROGRESS_STEP_PAGE`, `progress_steps`, `progress_measure`, and the `FillProgressSummary` and `MeasureProgressStep*` imports.
- **Restored `cancel_measure` (`:50`)** as a `WindowMeasure::Toggle`. This is the shape the tool had before the Progress row (git `599a5d8450^`).
  - id `puzzle3d-play-fill-cancel`, icon `circle-stop`, label `fill_cancel`.
  - Action `cancelFillBuild {job, operation, generation}`.
  - Text: `"<stage> · <applied> / <max> locked"`.
  - It is present only while a job identity exists and the run is not done.
- `measures` (`:68`) adds `cancel_measure`.

### Puzzle 3d brush: `🧊️3d/…/E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs`

- `brush_search_measures` (`:66`) no longer pushes the `puzzle3d-play-brush-search` Progress row. It returns only the placement `Select`, whose label still carries `brush_search_summary`.
- The row never had a cancel; leaving the vortex is the cancel, so no replacement control was needed.
- `brush_search_summary` and `brush_search_stage` stay. The stage function is still covered by the brush precompute tests, and wave 2 (W2-C) will reuse it.
- Docstrings updated.

### Puzzle 3d main window: `🧊️3d/…/E3/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:755`

- The stale comment now points at `brush_search_stage` instead of `WindowMeasure::Progress`.

### Puzzle 2d fill: `◻️2d/…/E2/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`

- **Removed:** `FILL_PROGRESS_STEP_PAGE`, `progress_steps`, `progress_measure` and the `MeasureProgressStep*` imports.
- **Added** a private `stage_text` (`:72`), which returns the localized stage including the fault code.
- `measures` (`:77`) restores the pre-Progress `puzzle2d-fill-cancel` Toggle (git `d8dce87ca0^`):
  - icon `x`, label `fill_cancel`.
  - Action `brushFillSessionCancel {generation}`.
  - Shown only while `is_running`.
- The existing `puzzle2d-fill-retry` Toggle stays.
- Both toggles now carry `stage_text` as their `text`, so the phase and fault reason stay visible and accessible.

### Puzzle 2d fill tests: `◻️2d/…/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`

- `live_fill_publishes_a_progress_row_with_cancel` is replaced by `live_fill_publishes_a_cancel_toggle_with_its_generation`. It checks:
  - An idle session has no cancel.
  - A running session has a cancel with label "Cancel fill", text "Searching" and the generation-12 action.
  - A running session has no retry.
- `fill_progress_localizes_stage_fault_and_retry` is replaced by `fill_toggles_localize_stage_fault_and_retry`. It checks:
  - In German, the cancel is labelled "Füllen abbrechen" with text "Anwenden".
  - A faulted run has no cancel, and its retry text starts with "Füllen fehlgeschlagen" and contains `puzzle2d-fill-hostile`.

### Puzzle 5d fill: `🖐️5d/…/E5/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs`

- **Removed:** `FILL_PROGRESS_STEP_PAGE`, `progress_steps` and `fill_progress_measure`.
- **Added `fill_cancel_measure` (`:48`)** as a `WindowMeasure::Toggle`. It keeps the cancel that the Progress row carried, in the 3d shape:
  - id `puzzle5d-play-fill-cancel`, icon `circle-stop`, label `fill_cancel`.
  - Action `cancelFillBuild` with the job identity or `{}`.
  - Text: `"<stage> · <applied> / <requested> locked"`.
- `measure` adds it.

### Puzzle 5d terminology: `🖐️5d/…/E5/🗣️terminology/🦀️.rs`

- New label `fill_cancel` with EN "Cancel fill" and DE "Füllen abbrechen", the same in both terminologies. This matches 2d and 3d.

### Puzzle 5d tests: `🖐️5d/…/E5/🧪️tests/🔬️unit/🦀️.rs`

- Removed the `find_measure_progress` helper.
- `fill_progress_row_reflects_the_wrapped_session_summary` is replaced by `fill_cancel_toggle_reflects_the_wrapped_session_summary`. It checks the id, label, the exact stage/applied/requested text, `pressed == false` and the `cancelFillBuild` action.
- The old precondition `idle.fill_progress().done` fails today, and it fails on its own before any measure is involved: a fresh 3d session reports `done == false`. It is replaced by the law "the toggle is absent exactly when `done`".

Terminology labels that are now unused (2d `fill_progress`/`fill_accepted`/`fill_tested`; 3d and 5d `fill_progress`/`fill_collision`/`fill_rejected`/`fill_tested`) raise no dead-code warnings, because `app_labels!` generates pub fields. They were left in place for the waves 1–2 `ToolRunDefinition` counters.

## 2. Public API as landed

```rust
// puzzle 3d  editor::puzzle3d::modes::edit::tools::fill
pub fn cancel_measure(precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Option<WindowMeasure>   // replaces progress_measure
// puzzle 5d  editor::puzzle5d::modes::edit::options::fill
pub fn fill_cancel_measure(precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Option<WindowMeasure>   // replaces fill_progress_measure
// removed: pub const FILL_PROGRESS_STEP_PAGE (2d, 3d, 5d); pub fn progress_measure (2d, 3d)
```

## 3. Verification (foreground; logs in `T/🗑️generated/W0-C2/`)

### Compile checks

Each crate reached its own warnings, and none of the warnings are in the edited files.

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4` | OK, 96 warnings | `check-3d.txt` |
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly -j 4` | OK, 4 warnings | `check-2d.txt` |
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly -j 4` | OK, 1 warning | `check-5d.txt` |

### Tests

`RUST_MIN_STACK=134217728` was set on every run.

**Puzzle 2d.** `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -j 4 -- tools::fill` gives **5/5 passed** (`test-2d.txt`).

**Puzzle 5d, the rewritten test.** `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -j 4 -- fill_cancel_toggle` gives **1/1 passed** (`test-5d-toggle.txt`).
- A temporary `[DEBUG]` probe confirmed the Toggle branch actually runs: text `"Preparing · 0 / 40 locked"` (`test-5d-debug.txt`). The probe has been removed.

**Puzzle 5d, the whole `fill` filter.** `cargo test … -- fill` gives 3 passed and 3 failed, both in parallel and with `--test-threads=1` (`test-5d.txt`, `test-5d-serial.txt`).
- `cancel_fill_build_pins_the_count_to_what_is_locked` and `set_fill_count_carries_a_large_count_and_retargets_the_planner` fail with the framework fault `interactive-job.owner-registration`: "tool factory key 's.puzzle.puzzle5d@1/*#editor/copy' is already registered". It is raised at `🔌️plugin/🦀️.rs:20539`, in app construction, and is not caused by this lane.
- The third failure was the old idle precondition. Section 1 explains it; the test is now green.

**Puzzle 3d.** `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- brush_search cancelling_a_stepping_fill_job engagement_abort_tears fill_cancel --test-threads=4` gives 6 passed and 1 failed (`test-3d.txt`).
- `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run` and `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one` now **pass**. Both read the `puzzle3d-play-fill-cancel` Toggle through `fill_cancel_identity` (`E3/🧪️tests/🔬️unit/🦀️.rs:1029`), which already expected a Toggle. This shows the cancel is reachable again. §0.9 had listed both as red.
- The one failure is `precompute::brush::tests::brush_search_progress_counts_blocked_candidates_as_a_verdict_not_a_silence` ("every candidate docks into the parked body", `free != 0`). It tests the precompute brush engine counters, not the measures, and sits outside this lane (W2-C / precompute).

## 4. Commands to register in launch.json

None new.

## 5. Deviations

1. The brief put stage and counters into the removed Progress row. As a stopgap until waves 1–2, the stage caption and the locked-of-requested count now travel as the cancel/retry Toggle `text`, so the run stays visible and accessible. The per-verdict step lines are gone.
2. **5d had no cancel control before the Progress row.** Its cancel lived only inside Progress, so it became a Toggle, following the 3d pattern. That required one new 5d label, `fill_cancel`.
3. The old 5d idle precondition was replaced by an equivalence law (see §1).

## 6. Foreign edits

- `🖐️5d/…/E5/🗣️terminology/🦀️.rs`: one added label line (`fill_cancel`). The file was owned for label removal only; the addition was needed for an accessible cancel label.

## 7. Open items

- Waves 1–2 delete these cancel toggles together with `cancelFillBuild` / `brushFillSessionCancel` (§3.6) and bring the progress back through `ToolRunDefinition` ticks.
- The React test `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎚️window-measure-controls/🟦️.tsx:43,212` still uses a `puzzle3d-fill-progress` Progress fixture. It belongs to W0-B (the `WindowMeasureProgress` TSX removal).
- Framework fault `tool factory key … already registered` in 5d app-constructing tests: needs an owner.
- 3d brush precompute failure `brush_search_progress_counts_blocked_candidates_as_a_verdict_not_a_silence`: needs an owner (W2-C).
- Re-run `cargo check -p semio-framework-os-renderer-wgpu --tests` (W0-C open item 5); it was blocked only by the 3d crate, which compiles again now.
