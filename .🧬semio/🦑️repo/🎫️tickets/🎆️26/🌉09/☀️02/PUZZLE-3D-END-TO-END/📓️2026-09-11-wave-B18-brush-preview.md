# Wave B18 — Brush preview JSON after B17

Ticket: `26/09/02/PUZZLE-3D-END-TO-END` (moon). Continues
`📓️2026-09-11-wave-B17-brush-arm.md`. Does not revert B17 leftover utility carry.

## User-visible land

Click Brush → leftover / world lane `activeUtility=brush` (B17, unchanged). Hover a vortex
while Brush stays armed. Guest `render_body` / `suggestionsTick` must publish a non-null
`brushPreviewJson` so the host `data-brush-preview-json` / vortex ghost is non-null.

C2 / #45b `brush-preview-place` was `FAIL preview=null` after B17: leftover no longer
disarmed Brush, but the guest tick still looked the utility map up under the window KIND.

## Hop

B9 `puzzle3d_addressed_window_id` already keys **render_body** / `handle` /
`puzzle3d_retained_reduce` by instance. The live miss was a different call site.

| #   | Hop                                     | Evidence                                                                                               | Verdict  |
| --- | --------------------------------------- | ------------------------------------------------------------------------------------------------------ | -------- |
| 1   | `#brush` → `SET_ACTIVE_UTILITY`         | host leftover `activeUtility=brush` (B17)                                                              | OK       |
| 2   | host map                                | `activeUtilityByWindowId["puzzle3d-main-perspective"]=brush`                                           | OK       |
| 3   | leftover hover                          | leftover `hoveredId=seed-left-001:v*` overlays, utility stays `brush`                                  | OK — B17 |
| 4   | `suggestionsTick` / `registerBrushMesh` | `[DEBUG] puzzle3d.utility.publish … window=Some("puzzle3d-main") utility= map_hit=false`               | **FOLD** |
| 5   | `suggestions_tick`                      | `active_utility == brush` is false (`PUZZLE3D_DEFAULT_UTILITY` is `""`) → no target, cache never warms | fold     |
| 6   | `render`                                | `brushPreview.lane utility= preview=0` even when leftover hover names a vortex                         | fold     |

`Puzzle3dPrecomputeCommandWork` (the path `suggestionsTick` actually takes) used
`view.window_id.or(command.window_id())`. The tick ViewModel's `window_id` is the KIND
`puzzle3d-main`. `puzzle3d_scene_active_utility` then did
`active_utility_by_window_id.get("puzzle3d-main")` against a map keyed by
`puzzle3d-main-perspective` → `map_hit=false`.

B9's helper could not save that path: it treated a kind-valued `view.window_id` as a
successful hit, so `command.window_id` / `focused_window_id` / roster instances never ran.
`World3dHost.dispatch("suggestionsTick")` also sent no `windowId` (only `surfaceId`).

`world_brush_preview_json` then gated on `envelope.active_utility == "brush"` **and** a
warmed `session.brush_preview` cache. Host leftover hover cannot fill that cache.

## Fix

Guest, `✏️editor/🦀️.rs`:

- `puzzle3d_window_id_is_kind` / `puzzle3d_first_instance_id` — a kind-valued id is not an
  addressing hit when an instance candidate exists.
- `puzzle3d_addressed_window_id` — prefer instance among keyed / `window_id` / focused /
  fallback / roster, then fall back to kind.
- `puzzle3d_utility_map_hit` — exact map hit, else (kind only) focused pane, else the first
  instance-prefixed map key. Instance miss stays a miss (B9 unarmed pane still `select`).
- `puzzle3d_scene_active_utility` + `scene_step` `map_hit` tap use that helper.
- `Puzzle3dPrecomputeCommandWork` (fill + prologue + publish) and
  `Puzzle3dWindowCommandWork` resolve through `puzzle3d_addressed_window_id`.

Host, vite-live:

- `🌐️World3dHost` `dispatch` stamps `windowId: windowInstanceId ?? surfaceId` on every
  world action, including `suggestionsTick`.

B17 `leftoverOverlayArmedBrushUtilityV1` / `leftoverOverlayCarryingUtilityV1` /
`leftoverOverlayCarryingSelectionV1` are unchanged.

## Laws

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1`
with `RUST_MIN_STACK=33554432`:

| Law                                                                                    | Result                                                                                                                                                 |
| -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `suggestions_tick_at_the_window_kind_still_publishes_the_instance_brush_preview` (new) | 1 passed — tick addressed at `puzzle3d-main` after arming `puzzle3d-main-perspective`; `map_hit=true utility=brush`; `brushPreview.lane … preview=281` |
| `hover_committed_in_brush_publishes_preview`                                           | 1 passed                                                                                                                                               |
| `each_window_instance_publishes_its_own_armed_utility_into_its_world_lane`             | 1 passed — unarmed pane still `preview=0`                                                                                                              |
| `set_active_utility_dirties_the_world_body`                                            | 1 passed                                                                                                                                               |
| `set_active_utility_emits_no_ops_and_no_history_entry`                                 | 1 passed                                                                                                                                               |

`SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/⚛️react/vitest.config.ts --testNamePattern='carries the armed utility|leftover activeUtility|hover leftover select keeps an armed brush'`:

- Test Files 1 passed | 23 skipped
- Tests **3 passed** | 883 skipped — B17 leftover carry not reverted

Real coverage of the new law: kind-addressed `suggestionsTick` after an instance arm used
to print `window=Some("puzzle3d-main") utility= map_hit=false` and `preview=0`. It now
prints `window=Some("puzzle3d-main-perspective") utility=brush map_hit=true` and
`preview=281` for `seed-left-001:v0`.

## :6014

HTTP 200 (not :6013). Host `windowId` stamp is vite-live. Guest addressing /
`puzzle3d_utility_map_hit` ride the **next wasm**. Until that build, live
`data-brush-preview-json` can stay null: old `PrecomputeCommandWork` still prefers
kind-valued `view.window_id` over the new command `windowId`.

## Constraints held

- No git.
- No `:6013`.
- B17 leftover utility carry not reverted.
- B9 per-instance render: an unarmed pane still does not inherit another pane's brush.
- Hover leftover still does not invent a pick.
