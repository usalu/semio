# Wheel Event Dispatch Parity

## Confirmed Evidence

Renderer Native86 ran1287tests:1281passed,sixfailed,zero skipped. The new retained Canvas burst tests reproduce two independent queue defects: two opposite events produce no settled camera, and two zoom-in events produce one factor. The mounted React host oracle processes each original DOM WheelEvent and emits one settled camera after both factors. The initial native camera is also unexpectedly1 rather than the authored2; a separate actual retained first-wheel law now distinguishes that initialization issue from batching.

The camera source audit confirms the causal distinction: the initial render correctly draws from Canvas2dScene, but scene_state inserts a retained zero viewport and rendering never initializes it from that canonical camera. Wheel subsequently replaces only the zero zoom with1 and computes the gesture from that default local camera. The earlier isolated native wheel law manually initialized local state and could not detect this boundary. The new actual paint-to-first-wheel law uses the published document and expects zoom2.2 with the canonical world anchor, without manual state mutation. A camera lifetime/initialization witness is required; zoom positivity cannot stand in for initialization or canonical-publication identity.

Terra traced a preceding host EventQueue coalescer that sums both deltas, overwrites coordinates and modifiers, and emits the result before discrete input. Removing the renderer AppWheel alone cannot recover event count or ordering. See the separate Terra wheel architecture audit for source locations and exact bounds.

## Coordinated Repair

Sol owns host queue laws and production after observed native RED. Scroll is non-replaceable ordered input; fixed-capacity refusal must preserve prior admitted events. The surviving replaceable pointer/metric sample must retain its original sequence relationship to discrete input.

Root owns renderer normalized-event dispatch. RuntimeApply already checks out one event and one interaction owner per bounded turn. That event can resolve its exact live target and apply once without AppWheel, FrameWheelCursor, or frame wheel phases. The immutable event carries its own x/y, both deltas, and complete modifiers. A target that retires before apply stays inert rather than redirecting to its successor.

The host normalization functions produce pixels, while generic Shell scroll currently multiplies delta by24. The replacement boundary must preserve pixels into retained UiEvent::Scroll and choose any engine-specific interpretation explicitly. Existing Shell tests manually dividing by24 do not establish physical wheel parity.

## Verification Gates

- Existing mounted React same-sign and opposite-sign burst cases remain the reference.
- Actual retained first-wheel test must start from authored camera12,-8,2 and end at its exact cursor-preserving camera after one1.1 factor.
- Actual normalized renderer Scroll tests must preserve every original event and produce one debounced final camera.
- Rework the old wheel-application-point fixture/tests when the old accumulator is removed: its stationary coalescing and overflow-tail semantics are superseded by the physical event contract. Preserve original-point coverage with real mounted React and actual renderer dispatch rather than a second mock accumulator.
- Host tests must cover modifiers, opposite deltas, mixed-event ordering, pagination, capacity refusal, and resumption without changing admitted input.
- Full renderer/UI native gates and fresh paired browser checks remain required after integration.

## Current Boundary

Checkpoint18 WGPU activation is running against coherent pre-wheel production, including the repaired per-pointer and secondary Canvas routing. Renderer production is held stable until that build completes. Test source preparation is independent. The first browser checkpoint remains necessary even while the wheel repair proceeds; it will establish current window, settings, and World3d behavior rather than claiming full parity from unit tests.


## Direct Dispatch And Mounted Camera Source

Native87 independently confirmed an initial authored zoom of 2 became 1.1 after one wheel instead of React's 2.2. The renderer now applies each normalized Scroll inside its existing single-event dispatch owner; AppWheel and the frame wheel phases have been removed. Shell receives normalized pixel delta_x/delta_y without multiplying by 24. Full per-event modifier snapshots reach the World consumer; generic Canvas scenes follow the existing retained UI event path because the bespoke scene gate admits only World/Graph/Map/Board.

Canvas initial camera seeding now carries the exact retained component identity from paint: window id, stable window generation, node id, borrowed key, surface kind and surface id. SceneSlot exposes its borrowed retained key. A normal document revision preserves this owner; a replacement key remounts. The read-only audit's earlier node-only proposal was rejected: reconcile updates the key on the same NodeId. The source change still needs native validation. Two actual retained lifecycle RED laws were added for an old checked-out camera deadline and component removal; their production retirement repair is intentionally pending the first failing receipt.

Host transport RED: four core laws all failed (43ms, Nx25.7s); cross-page law independently failed (42ms, Nx24.6s). Sol's queue and runtime generation-merge changes are ready, and the full host suite is running.
# Native91 Receipt and Camera Retirement Repair

The full renderer run reached 1,286 passes out of 1,290 laws. Real normalized dispatch now passes the first authored-camera wheel, multiple samples at their original positions, opposite wheel samples, and same-key refresh/replacement-key camera initialization. The two new removal/replacement deadline laws failed before the retirement implementation, publishing one obsolete camera action instead of zero.

Camera deadlines now carry the exact retained mount identity alongside their due time. A checked-out deadline verifies that identity before publishing or restoring. A newer scheduled deadline keeps ownership, and delayed retirement clears only the removed mount. The Interpreter consumes Canvas retirements at the existing one-retired-scene-per-step seam already used for Map. Tests cover expiry and cursor cancellation, removal and replacement, plus a delayed original retirement after the replacement's wheel; only the replacement may publish its expected 3.3 zoom.

The admitted-wheel fixture correctly compared its output but its final assertion confused an empty queue with a closed owner. Draining leaves the queue's allocation alive. The test now closes that existing bounded queue owner in at most two close steps before asserting terminal-empty. No host or runtime behavior changed for this test-only correction. Native92 is the next full verification receipt.
