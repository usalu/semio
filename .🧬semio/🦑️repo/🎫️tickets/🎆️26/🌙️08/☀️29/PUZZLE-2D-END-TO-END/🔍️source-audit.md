# 🔍️ Puzzle 2D — Source Audit (pre-runtime)

> ⚠️ **Superseded in one important respect.** This audit's conclusion — "everything is wired, so the
> risk is purely runtime" — is right about *declarations* and wrong about *behaviour*. Auditing
> declaration sites cannot see a contract mismatch at a data boundary, and that is exactly what was
> wrong: the board engine's kind catalogs were never populated, so brush had no candidates and fill
> faulted. Read `🐛️root-cause-kind-catalogs.md` for what was actually broken. Kept as written
> because the method failure is the useful lesson: "every call site is present and dispatched" is not
> evidence that data survives the hop between two contracts.

Audit of the puzzle 2d artifact before runtime verification. Conclusion up front: **the 2D
implementation is complete and internally consistent at the source level**. Every tool the goal
names (brush, fill, select) is declared, wired, dispatched and unit-tested. Nothing is a stub.
Therefore the end-to-end risk is entirely in *runtime* (build → boot → render → interact), not in
missing code.

## 1. Tool / utility surface

| Surface | Kind | Id | Icon | Declared at | Status |
| --- | --- | --- | --- | --- | --- |
| Select | window utility (overview) | `select` | `mouse-pointer` | `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️component.rs` | implemented |
| Brush | window utility (overview) | `brush` | `paintbrush` | `…/🪛️utilities/🖌️brush/🦀️component.rs:10` | implemented |
| Fill | **mode-level tool** | `fill` | `paint-bucket` | `✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs:16` | implemented |

Registration is asserted by a unit test — `utility_registry_declares_utilities`
(`✏️editor/🦀️component.rs:2542`) requires the overview window to expose exactly
`[select, brush]` and to inject the `setActiveUtility` action.

Fill is deliberately *not* a window utility: its count slider is a mode-level tool measure keyed by
the tool id, asserted by `fill_count_slider_is_a_tool_measure`
(`🛠️tools/🪣️fill/🦀️component.rs`).

## 2. Command surface

`puzzle2d_command_variants!` (`✏️editor/🦀️component.rs:743-787`) declares **43 actions**. All are
handled — the fill family is dispatched at `✏️editor/🦀️component.rs:945-952`:

```
setFillCount            → set_fill_count::set_fill_count
brushFillSessionStep    → fill_session_step::fill_session_step
brushFillSessionAdopt   → set_fill_count::adopt_fill_job
brushFillSessionCancel  → set_fill_count::cancel_fill_job
brushFillSessionRetry   → set_fill_count::retry_fill_job
brushFillSessionDiscard → set_fill_count::discard_fill_job
brushCommitSlot         → commit_slot::commit_slot            (✏️editor/🦀️component.rs:1738)
```

No declared-but-unhandled action was found. Every action the fill measure's `on_change` references
(`setFillCount`, `brushFillSessionCancel`, `brushFillSessionRetry`) exists in the variant list and
in the dispatch match.

## 3. Fill runs as an async job with a real driver

Fill is not synchronous. `Puzzle2dFillLifecycle` moves through
Capturing → Queued → Running → CheckpointReady → Applying → AwaitingAdoption → Completed/Faulted/Cancelled.

The driver is the framework refresh cycle, **not** an in-plugin loop: `pending_effects()` on the
2D app delegates to `set_fill_count::reconcile_snapshot_read`, which returns
`Effect::DispatchAction{ action: "brushFillSessionStep", delay_ms: 1 }`. The framework calls
`pending_effects()` once per `refreshUi`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `response.requested_effects = …`),
so each step re-arms the next until a terminal state. `pump_fill_worker` re-queues on
`Yield` / `Submitted` / `Rejected` and queues adopt on terminal-with-checkpoint.

This differs from puzzle3d, which uses `action_interactive_job("fillBuildTick", …)`. Both are
valid; 2D's is the effect-driven variant.

## 4. Brush architecture (and a non-bug that looks like one)

The brush engine is `✏️editor/⚙️engine/🖌️brush/🦀️component.rs` — **1343 lines**, no `todo!`,
no `unimplemented!`, no silent empty-candidate return. It computes candidates from handle-link
compatibility, ranks them by handle proximity, weights them per kind, previews along the handle
normal at `suggestion_offset`, and emits `brushPreview` / `brushCandidates` / `brushPlace` into the
board event queue drained by `drainEventsJson`.

Brush options (suggestion-offset slider, per-kind distribution trees, candidate picker) live at
`✏️editor/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs`, tagged
`active_utility_id: Some("brush")` so the chrome only reveals them while brush is active.

**Investigated and dismissed:** the WASM bridge (`🌉️wasm/🦀️component.rs`) exports
`brushOpenSlot`, `brushCommitSlot`, `brushCancelSlot`, `brushSetCandidateIndex`,
`setBrushNodeSize`, `setBrushSessionJson`, `clearBrushSessionJson`, none of which the TypeScript
`Board2dWasmSession` type declares. This initially reads as a critical gap. It is not: `Board2dHost`
(`🧱️elements/Board2dHost/🟦️component.tsx`) only needs `setBrushKindWeights` (line 738) and
`brushCycleCandidate` (line 890, bound to Tab/Shift-Tab) because slot open/commit/cancel are driven
*inside* WASM by pointer events, and the plugin-side `brushOpenSlot`/`brushCommitSlot` commands act
on the plugin's own `BoardHost` for the context-menu "Suggest nodes" path. Two hosts, two routes —
by design.

## 5. Front-end wiring

- Utilities reach React via `resolveUtilityNodes()` and render as a toggle group in
  `🧱️elements/UtilityTree/🟦️component.tsx:248-254`, dispatching `setActiveUtility`.
- Mode tools become tabs via `buildToolTabs()` in `🧱️elements/ShellHost/🟦️component.tsx:5847`.
- `WindowMeasure` has 4 Rust variants (Group/Slider/Toggle/Select); the React renderer in
  `🧱️elements/ShellHelpers/🟦️component.tsx` handles all four. **No unrenderable measure variant.**
- Canvas is WebGPU via `attach_canvas` + `renderFrame`, not Canvas2D.

## 6. Prior open defects (June, likely stale)

Two 2D tickets are still `open`, both from 26/06/01, i.e. before the August refactors:

- `PUZZLE-2D-PLAY-MISSING-EDGES-AGAIN` — carries a **full completion summary** ("179 scene edges on
  all three panes … 121 react tests pass"). Fixed but never closed.
- `PUZZLE-2D-MULTI-WINDOW-SELECTION-CLEAR-SYNC` — background click / single-element pick do not
  clear selection on peer panes. **No summary → genuinely unfinished.** Re-check at runtime.

## 7. What this audit does not prove

Nothing here is runtime evidence. Source completeness does not establish that the plugin compiles
for wasm, that the dev server boots, that the canvas renders, or that brush/fill behave. Those are
verified separately and recorded in `🧪️runtime-verification.md`.
