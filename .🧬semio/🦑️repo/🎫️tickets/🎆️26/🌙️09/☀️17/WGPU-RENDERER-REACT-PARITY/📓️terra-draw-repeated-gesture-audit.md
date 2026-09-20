# Draw8 Repeated Rectangle Audit

## Finding

Draw8 proves that the second gesture reached the application route, but it does **not** prove that the second retained owner was constructed with `shapeRect`. The failure is therefore downstream of physical mouse delivery and upstream of the document projection.

`🗑️generated/astra-runtime/draw-react-8/draw/react/failure.json` records this exact ordered sequence:

1. `setActiveUtility`
2. `canvasPointerMove`, `canvasPointerDown`, `canvasPointerMove`, `canvasPointerUp`
3. `setActiveUtility`
4. `canvasPointerMove`, `canvasPointerDown`, `canvasPointerMove`, `canvasPointerUp`

The second `setActiveUtility` reaches the guest exchange, so it is a real second physical arm, not the prior commit's reset. The reset is an `Effect::SetActiveUtility` consumed directly by the React host at [ShellHost/🟦️.tsx](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5777), and does not invoke the plugin exchange. The receipt contains no `drawing.gesture` fault, nonzero rejected count, output fault, or reservation refusal.

The second pre-gesture capture, [rectangle-two-before.png](../🗑️generated/astra-runtime/draw-react-8/draw/react/rectangle-two-before.png), shows the first committed outline and the Rectangle control's selected presentation. The final image has one `Rectangle` tree row; the second expected row is absent. This excludes a target-selector failure and an omitted DOM pointer sequence.

## Owned boundary

The action ledger cannot reveal this missing fact: it intentionally stores only controller, action and outcome, dropping descriptor arguments in [input-ledger/🟦️.ts](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎛️input-ledger/🟦️.ts:231). The Draw adapter consequently serializes React action args as `null` at [canvas-interactions/📜️script.ts](../🔬️canvas-interactions/📜️script.ts:250), so it cannot establish the active utility supplied to either retained owner.

The decisive value is captured when the retained operation is built: [editor/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1572) stores `drawing_active_utility(view_state)` in `DrawingGestureOperationPayload`. The instance owner makes a fresh `DrawingSession` from that payload at [editor/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:490) and, for a pointer-down, only commits a shape when the retained session's captured utility is a shape at [canvas-pointer-down/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:363). A `selectDirect` payload instead finishes the pointer-up as a pick path, with no document mutation.

The host does update its utility register before forwarding the arming action, at [ShellHost/🟦️.tsx](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6749), and generic pointer dispatch rebuilds its view state from that register at [ShellHost/🟦️.tsx](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:7005). Source review therefore does not establish a stale-register bug; it establishes the exact hand-off that needs a two-gesture law.

## Required next evidence

Add one neutral retained-owner law beside [gesture-operation-owner/🦀️.rs](../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️gesture-operation-owner/🦀️.rs:176), exercising the real owner rather than `DrawingSession` alone:

1. Arm `shapeRect`; drive hover, down, move and up through the retained route; assert one `create_layer` and the reset effect.
2. Apply the reset to the host view-model, re-arm `shapeRect` with the next utility-register generation, and drive the same second route on the first mutation's snapshot.
3. Assert a second `create_layer`, a second reset effect, no `drawing.gesture.saturated`, and that each operation is retired before the next operation occupies its residue.

That law will distinguish a stale captured `active_utility_id`, a retained-owner hand-off/retirement error, and a missing second commit. It preserves the physical adapter's role: after the law is green, the adapter should wait for the second gesture's settled action outcome plus the two-row projection; it must not fabricate React action args.

**Confidence:** high for delivery/re-arm and the missing observability; medium for any individual retained-owner defect until the described owner-level law runs.

