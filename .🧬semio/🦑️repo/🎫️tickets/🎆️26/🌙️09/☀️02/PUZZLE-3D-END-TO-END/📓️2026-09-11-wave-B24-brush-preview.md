# Wave B24 — Latched Tick Preview Still Dies On Leftover SurfaceVisible

## Verdict

`bun 🔍️browser-probe.ts --only=brush-stroke --port=6014` from this moon ticket is still **`brush-preview-place FAIL preview=null`**.

Latest clean proof: `🗑️generated/probe-2026-09-11T22-23-53.md` and `🗑️generated/b24-brush-stroke.txt`. Later `22-28-01` / `22-30-39` died mid-arm (`actor-activation.revoked` / `Agent disconnected`) after shared-vite HMR — not a payload verdict.

Guest leftover hover still publishes 252–313 byte `brush_preview_json` on the same `suggestionsTick` / `render()` that logs `preview=252…313`. Host `#puzzle3d-main-perspective [data-brush-preview-json]` stays `rawLen=0`. Do not revert forceReload / replaceBodies / leftover pump. Do not revert B17 / B9 / B18 / B23.

## What this pass landed

1. **Session latch** — `Puzzle3dSessionState` now check-in/check-out `brush_live_target`. `suggestionsTick` used to latch the vortex on a throwaway `Puzzle3dPlayApp`; leftover `refresh-ui` / `plugin_render_surface` checked out a fresh app with collision/fill only, so `world_brush_preview_target` saw `vortex=None` and projected the boot spine (`previewBytes: 0`). Law: `leftover_refresh_after_suggestions_tick_still_publishes_latched_brush_preview` (clears guest hover after the tick; the latched target must still publish). Guest rust rematerialized (`@semio-tech/puzzle-plugin:materialize-dev --skip-nx-cache`).
2. **Leftover hash omit** — `refreshUi(..., replaceBodies)` deletes `window:` cache hashes when leftover `activeUtility=brush` and hover names a vortex (`leftoverBrushPreviewWindowHash`), same idea as leftover Inspection. applyHostEffects leftover replace no longer asks for a hashed-unchanged boot body.
3. **Leftover pump re-arm** — every leftover InteractionView with brush + vortex sets pump `pending` (not only the first hover key). Sequential pump kept; no 14× overlapping `{ kind: "full" }`.
4. **Window-body leftover refresh** — pump now refreshes live `sessionWindowInstances` body keys (`kind: "partial", windowBodies`) instead of the whole chrome, so leftover hover does not wait on a hung full `refreshUi`.
5. **Latest replaceBodies wins** — an older empty leftover `replaceBodies` apply no longer clobbers a newer leftover generation. Non-replaceBodies cancel is unchanged.

## Latest clean probe (`22-23-53`)

- `leftover brushPreview refresh { hover: seed-left-001:v3, pump: true, replaceBodies: true }` — **1** start (still before the 252–313 tick lanes).
- Then 4 leftover `lane utility=brush preview=0`, then tick lanes `preview=282…252`.
- `brushPreview.assemble` — **4** runs, all `spinePreview: 0, collectedPreview: 0, previewBytes: 0` (boot / arm, all before leftover refresh).
- `leftover forceReload` — **0**. The leftover `refreshUi` guest-rendered empty windows; the host store never loaded the tick payload.
- Host bind stays `rawLen=0`. `brush-place hop preview=null`.
- Battery hard faults PASS on that clean run. Do not use `:6013`.

## Remaining hop

The projected `window:puzzle3d-main-perspective` BuiltNode / SurfaceVisible still does **not** include the guest `brush_preview_json` from the `render()` / `suggestionsTick` that logged `preview=252…313`.

Leftover pump still starts on the first leftover hover *before* the tick warms the latch / cache (`gate reason=no-free-candidate` then later `compute bytes=252…283`). That first leftover SurfaceVisible is the boot world (`preview=0` on all assembles). The tick payload never becomes the applied window body — leftover `refreshUi` either hangs past apply (`forceReload=0`) or applies the empty request that started first.

Next work must apply the **post-tick** leftover / `refresh-ui` tree (live target is now in the session slot) and keep an earlier empty replaceBodies from winning. Do not invent pose / `objectKind` on the host.

## Laws run this pass (not a browser PASS)

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 \
  leftover_refresh_after_suggestions_tick_still_publishes_latched_brush_preview \
  suggestions_tick_at_the_window_kind_still_publishes_the_instance_brush_preview \
  hover_committed_in_brush_publishes_preview \
  each_window_instance_publishes_its_own_armed_utility_into_its_world_lane \
  set_active_utility_dirties_the_world_body
# 5 passed

SEMIO_TEST_LEVEL=long bun x vitest run --config <renderer-engine-react-vitest> \
  --testNamePattern='forceReload queues loadSnapshot|keeps the leftover vortex id on an armed brush|omits the cached world-body hash|carries the armed utility|leftover activeUtility|hover leftover select keeps an armed brush|keeps a spine brush preview|does not clobber|retains last brush preview JSON'
# 9 passed
```

`[DEBUG]` prefixes stay until a probe PASS.

Repo MCP was unavailable; work stayed in moon ticket `2026/09/02/PUZZLE-3D-END-TO-END`.
