# Draw Owner Root-Cause Audit

## Scope

This is a source and captured-runtime audit of Draw10's first-rectangle / missing-second-rectangle result. It does not run Cargo while native work is active, and it does not change product source.

## Captured runtime evidence

[`draw/react/failure.json`](🗑️generated/astra-runtime/draw-react-10/draw/react/failure.json) records this action order:

1. Arm `shapeRect`.
2. Hover move, Down, Move, Up at the first rectangle coordinates.
3. Arm `shapeRect` again.
4. Hover move, Down, Move, Up at distinct, non-overlapping second rectangle coordinates.

Every routed action is recorded as `applied`. The captured physical diagnostics say both sequences were inside the canvas, non-cancelled, and had no modifiers. The assertion then observes two layers rather than the required three: the first new rectangle exists and the second does not.

That rules out a missing browser PointerUp, a cancelled gesture, coordinate overlap, and the local Canvas host dropping the second drag. It does **not** prove the Rust operation payload captured `shapeRect`: the public ledger deliberately omits arguments, and `applied` records route completion rather than a committed mutation or publication.

The failure console contains no `drawing.gesture.*` rejection, registry admission refusal, output fault, or stale-operation diagnostic. Its reactor snapshots report a peak of one live typed-operation slot. That makes registry saturation an unsupported explanation for this capture.

## Actual owner boundary

The active source makes the following boundary concrete:

| Concern | Source evidence |
| --- | --- |
| Payload utility capture | [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1572) reads the utility from the request view state using `drawing_active_utility`. |
| Per-window utility lookup | [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:45) gives the view-window map precedence. |
| Reuse rule | [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:496) only cancels the active retained session when the canonical base revision or captured utility differs. [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:503) otherwise routes a new operation key to that active session. |
| Idle hover handling | [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:587) starts a point query for an idle move. The single-flight React input lane awaits each route before it runs the next event in [`🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️ShellHost/🎯️input-ledger/🟦️.ts:422), so the captured hover is not evidence that a query remained live when Down arrived. |
| Commit and reset | [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:196) creates a layer for a shape drag; [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:212) emits the Direct Select reset; [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:621) retires the owner when the session is idle and no trace remains. |

The key-insensitive reuse at line 503 is the primary owner-level boundary to test. It is necessary for separate Down, Move, and Up operation keys to share one retained gesture, but its correctness across a committed revision, view refresh, utility reset, and rearm has not been demonstrated.

## Causal conclusion

No single implementation defect is established by the available Draw10 evidence. Claiming that the reuse rule, a stale revision, or a utility-transfer failure is the actual cause would be a guess.

The capture proves the fault is after React's local event acceptance and before the final observable layer count. It cannot distinguish these mutually exclusive outcomes:

1. The second operation payload captured `selectDirect`, despite the host's local `shapeRect` diagnostic; the owner then correctly took the direct-pick path and emitted no create-layer mutation.
2. The payload captured `shapeRect`, but the reused or newly admitted retained owner did not reach a shape commit.
3. A second shape commit was produced and was rejected or superseded before its revision was published.

The retired host JSON diagnostics did not contain payload utility, canonical base revision, retained-session FSM phase, emitted effects, mutation identity, or publication result. The public action ledger cannot supply those missing facts. The root cause is therefore presently **unobserved at the owner/payload-to-publication boundary**, rather than established as a browser, WGPU, or React pointer defect.

## Precise fail-first regression

Add the regression beside the real app tests in [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:341), before repairing the owner. It must use `context::drawing_app()` and `settled()` so it exercises the registered `DrawingInstanceOperationOwner`, not a standalone `FixedOperationRegistry`.

The law is:

1. Create the production drawing-canvas view context using `DRAWING_PLAY_WINDOW_CANVAS` (`drawing-composite`), and arm its **per-window** utility map with `shapeRect`.
2. Send and settle an idle `CanvasPointerMove`, then Down, Move, Up for rectangle A. Assert the snapshot gains exactly one layer and the Up receipt includes the Direct Select reset.
3. Start from the newly published snapshot/revision. Create a fresh view context for the same window instance, rearm that per-window map to `shapeRect`, then send and settle a distinct idle move, Down, Move, Up for rectangle B.
4. Assert the final snapshot has exactly two additional layers, both are distinct rectangle creations at the two coordinate ranges, and both Up receipts carry the Direct Select reset.

The test must not reuse the first `ViewModel` for the second cycle. Its fresh post-publication view is what validates canonical-revision capture, new-view transfer, owner retirement, and rearm together. Including the idle moves matches Draw10's actual wire order and covers the query path before each Down.

This law fails for each of the three unobservable outcomes above, while the existing test at [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:341) only proves one shape drag. The closest owner test manually retires registry entries at [`🦀️.rs`](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️gesture-operation-owner/🦀️.rs:220); it does not dispatch the real owner or FSM and therefore cannot verify this lifecycle.

## Confidence and limits

| Finding | Confidence |
| --- | --- |
| The captured physical gesture reached the host and was not cancelled or locally dropped. | High |
| A real first shape commit is observable; the second final layer is absent. | High |
| Current tests omit the two-gesture, post-publication, fresh-view owner lifecycle. | High |
| The line-503 owner reuse rule is the first precise owner boundary that the missing regression must exercise. | High |
| Any particular one of utility capture, owner/FSM progress, or publication rejection caused Draw10. | Not established |

No test was run for this audit.
