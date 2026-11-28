# Gumball move accuracy under stale ticks (2D and 3D)

Ticket `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END`. Context: "T1 accuracy" note of `fem-history-undo-alternatives.md` and U7 of `fem2d-interaction-results-fixes.md` (a 60 px move landing at 22 px).

## 1. What the hosts sent (finding)

Both hosts sent DELTAS and advanced their running total when they SENT, not when the guest acknowledged:

| Host | Where | Behaviour |
| --- | --- | --- |
| 2D React | `Canvas2dHost/🧭️gumball/🟦️.tsx` `canvas2dGumballDrag` | `canvas2dGumballIncrement(gesture.total, total)` per move, `gesture.total = total` at send; `dispatch` is fire-and-forget, no in-flight gate, so ticks overlap freely |
| 2D wgpu twin | `Canvas2dHost/🎯️targets/🧊️wgpu/🦀️.rs` `drag` / `release` | same algebra in Rust (`increment`), same corpus |
| 3D React | `World3dHost/🧮️gumball-math/🟦️.ts` `worldGumballPump` | delta `sent -> latest`, `sent = latest` at send, one tick in flight (a stale refusal still happens) |

Guests (`fem2d_gumball_then`, `fem3d_gumball_then`/`fem3d_turn_then`) composed those deltas into the one open leaf, so a tick refused as `interactive-job.stale` took its displacement with it. The final move was short by exactly the dropped deltas. Acknowledgement-gating was rejected: the stale refusal is a terminal fault on the typed-operation completion frame, `onAction` is a watchdog-bounded promise that is not a reliable accept/refuse signal, so the host cannot tell a dropped tick from an applied one.

## 2. Design: cumulative from the drag start (idempotent ticks)

Every `stream` tick AND the `commit` carry the WHOLE offset (angle, factor) from the grab/press pose. The guest REPLACES its one open leaf with the newest tick instead of composing. A dropped tick costs nothing, the next tick or the commit carries the full gesture; with every stream tick dropped the commit alone (at rest, a one-shot) still carries it. When the pointer is back on the grab pose after a tick went out, the host streams and commits the identity, and the guest retracts the leaf.

### 2D (unconditional, FEM is the only `meta:gumball` consumer)
- `📐️Canvas2dHost/🧭️gumball/🟦️.tsx`: `canvas2dGumballIncrement` deleted; `canvas2dGumballDrag` streams the total (identity when back on the press once streamed, nothing when unchanged); `canvas2dGumballRelease` commits the total (identity when back on the press).
- `📐️Canvas2dHost/🎯️targets/🧊️wgpu/🦀️.rs`: `increment` deleted, `drag` and `release` as above (unused epsilon imports dropped).
- Guest `🕹️interaction/🧭️gumball/🦀️.rs` (2D): `fem2d_gumball_then` deleted; `fem2d_gumball_admits` (finite, positive factors) and `fem2d_gumball_moves` (admits and not identity); `continue_stream` and `finish_stream` replace the net with an admitted tick.

### 3D (opt-in `gumballCumulative` in the selection record)
`World3dHost` is shared by lowpoly, procedural-3d transforms and generation3d, whose guests compose deltas (`lowpoly_stream_then`, `GumballDispatch`). Flipping the shared host would break them without a way to run their suites here, so the flag `gumballCumulative` (next to `gumballLiveDispatch`, documented in `🔭️scene-schema`) selects the cumulative variant; the delta protocol and `🛠️gumball-live-protocol.json` stay unchanged for the others. FEM 3D sets it in `fem3d_selection_json`.
- `🧮️gumball-math/🟦️.ts`: gesture gains `cumulative` and `origin` (grab pose); `worldGumballPump` and `release` take `from = origin` when cumulative; a cumulative gesture that already streamed sends the identity when the pose is back on the origin; the start event carries `cumulative`.
- `🎛️marquee-and-gumball/🟦️.ts`: `cumulative: Boolean(selection.gumballCumulative)` in the `start` event.
- Guest `🕹️interaction/🧭️gumball/🦀️.rs` (3D): `fem3d_gumball_then` and `fem3d_turn_then` (quaternion composition, sign flips) deleted, same `admits`/`moves` split and replace semantics. A cumulative turn also removes the axis sign-flip bookkeeping: axis and angle are always from the grab.

Residual (documented, not a regression): a turn past 360 degrees is reported modulo a revolution in 3D (poses carry quaternions); the final rotation is identical.

## 3. Laws

| Law | Where | Status |
| --- | --- | --- |
| `a_stream_tick_dropped_as_stale_costs_the_committed_net_nothing` (translate, turn, scale; drops any subset of stream ticks, all of them; net equals the full gesture's, which equals the cumulative total) | 2D `🕹️interaction/🧭️gumball/🧪️tests/🔬️unit`, 3D same (3D includes a turn whose reported axis flips) | PASS 2D and 3D |
| `stream_ticks_in_flight_together_never_lose_the_gesture` (mounted app: overlapping cumulative ticks 0.25, 0.75, 0.875 in flight, commit 0.875, node moves by EXACTLY 0.875, undo exact; was `..._never_wedge_the_gesture`, which only asserted "moved") | 2D and 3D `🎮️commands/🧭️gumball/🧪️tests/🔬️unit` | 2D PASS; 3D compiled, not run (store-init stall, section 4) |
| `the_world3d_cumulative_gumball_protocol_lands_as_its_guest_edits_whatever_ticks_are_dropped` (replays the host corpus through the wire decode; offset and edit count equal with no tick dropped, each single stream dropped, all dropped) | 3D commands tests, corpus `World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-cumulative-protocol.json` | compiled, not run (same stall); the same corpus is validated by the TS host law below |
| `streamed_ticks_commit_the_net_leaf_once`, F2 `a_sixty_pixel_ring_drag_turns_the_slab_into_a_valid_solid`, selection record carries `gumballCumulative` | updated to cumulative ticks | PASS (3D interaction, 2D interaction); the 3D selection record law PASS |
| TS host law `🎯️gumball-cumulative` (replays the cumulative corpus through `worldGumballStep`; `three` re-derives the net rotation, `gl-matrix` the net offset) | `World3dHost/🧪️tests/🎯️gumball-cumulative`, registered in the react vitest config | 7 of 7 PASS |
| TS 2D corpus `🧪️gumball-dispatch` (corpus regenerated cumulative by an independent script; gl-matrix oracle now takes the commit as the net; new law: each stream tick equals a release at that very point) | `Canvas2dHost/🧪️tests` | 37 of 37 PASS |
| wgpu 2D twin `semio-renderer-wgpu-canvas2d-host --lib gumball` | 2 of 2 PASS: `the_shared_gumball_corpus_replays_exactly_as_react_does` replays the regenerated cumulative corpus exactly as React does. The `🎞️Scenes` seam test `a_live_gumball_drag_streams_and_commits_through_the_canvas_seam` (commit dx 24.0) was edited but not run (separate crate graph) |
| TS `🎯️gumball-cumulative` | 7 of 7 PASS |
| TS `🧪️gumball-dispatch` | 37 of 37 PASS |

Peer note: a first 3D build failed on a half-written framework edit (`PluginLifecycleFailure::plain_refusal` missing in `🏛️ownership/⚠️failure`); it was fixed by its owner within the hour, nothing of mine.

## Files
2D guest: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `.../🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`.
3D guest: same under `🧊️3d`.
Framework: `📐️Canvas2dHost/{🧭️gumball/🟦️.tsx,🎯️targets/🧊️wgpu/🦀️.rs,🧪️tests/🧪️gumball-dispatch/🟦️.ts,🧪️tests/🧫️fixtures/🧫️gumball-dispatch/🔣️.json}`, `🌐️World3dHost/{🧮️gumball-math/🟦️.ts,🎛️marquee-and-gumball/🟦️.ts,🔭️scene-schema/🟦️.ts,🧪️tests/🎯️gumball-cumulative/🟦️.ts,🧪️tests/🧫️fixtures/🛠️gumball-cumulative-protocol.json}`, `🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs`, react vitest config (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`).
The shipped wasm needs a rebuild (the wgpu 2D host and the guests are Rust); the React hosts are live through vite.
