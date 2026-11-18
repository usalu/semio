# Playgrounds: section, floor plan and 3D over one scene

## Result

`#/playground` is now three playgrounds over one shared scene, each at its own address:

| Tab | Address | Shows |
|---|---|---|
| Section | `#/playground` (`vp=elevation` or `vp=front`) | side or front elevation, figures on a ground line |
| Floor plan | `#/playground/plan` | plan viewpoint, a `lengthM` by `depthM` area with metre grid and axes, figures dragged in x and z |
| 3D | `#/playground/3d` | neutral `semio.model/v1` scene of every placed asset (floor with metre grid, orbit, camera presets), loaded lazily |

The tab links, the zoom (`pz`), the scene (`p`, `len`, `dep`, `tl`, `cam`, `scale`, `view`, `bg`, `vp`) are in the address, so a link reproduces the scene and the tab; the local store (event-sourced, undoable) keeps the scene across tabs, reloads and back/forward.

## One scene, consistent positions

A `Placement` has `xM` (ahead), `zM` (the asset's left), `yawDeg`, `pick`, `offsetS`, `mirrored` and a track: `startS`, `durationS` (`null` = to the end), `speed`, `loop`, `holdS` (paused at a clip time). The `Playground` has `scale`, `view`, `viewpoint` (section only), `camera`, `background`, `lengthM`, `depthM`, `timelineS` (`null` = longest window).

- side elevation: paper x = x; front elevation: paper x = depth/2 + z (the line is `depthM` long, z = 0 in the middle); plan: paper x = x, paper up = +z (`🧪️playground/📐️geometry`, `linePosition`).
- 3D: `placeNode` puts a placement at `(z, 0, x)` of the neutral frame (README mapping `(x, y, z) -> (z, y, x)`); a positive yaw turns forward to the left (rotation about +Y). The 2D drawings do not turn figures; the panel says so.
- A walking figure is planted on its ground point in all three: the 3D composer removes the ahead travel of the rig root from every root-level translation (same rule as `PlacedFigure`'s `-rootM`).

## Playback

- Global: one `StaffageClock` of the page with the existing `Transport` (play, pause, step, scrub, rate, loop). Timeline length follows the longest window unless the reader sets it (`tl`).
- Per figure (`🎛️asset` panel and a play/pause button in each timeline row): pose or clip, pause (freezes at the current clip time), scrub clip time, loop, speed (0.1 to 4), start, length / until the end, clip offset, position, yaw, mirror.
- Timeline rows (`🎞️timeline/🖼️panel`): bar = window (clip repeats drawn as ticks), playhead, native keyboard sliders for start (bar) and length (right edge): Left/Right 0.1 s, Shift 1 s, Alt 0.01 s, Home/End; pointer drag commits one event on release; double-click the length handle = until the end. Changes are announced in a polite status.
- Rules in one pure module (`🎞️timeline`: `readTrack`, `timelineLengthS`); the 3D scene bakes the same rules into the master animation `timeline` (`🧊️model/🎞️retime`, unrolled loops, hold before/after the window, hold at a clip time), so one instant is one state in all three views.

## 3D playground

`🧪️playground/🧊️model/🟦️.tsx` (lazy through `SpatialPlayground` in `🧊️spatial/🚪️gate`): `prepareModel` per placement (one scale through the new `SpatialRequest.scales`, cached per asset object), then `composePlayground` (`🧩️compose`: prefix `p<i>-`, slot node `slot-<i>`, merged scenes `s<scale>-<mode>` and variants, floor with metre grid), then `ModelViewer.setDocument`. Progress over the phases with a Cancel button and "Try again"; a change of scale / placements / timeline re-prepares after a 200 ms settle, a change of representation or camera only switches variant, scene or preset; clicking a figure selects its placement; assets without a model at the scale are named ("No 3D model at 1:75: 1"), never replaced by another scale. The 3D code stays out of the entry chunk (`🧊️model-split` passes; `floor-grid`, `semio.model/v1` and the WebGL2 shader are only in lazy chunks).

## Not stale

- Rows come from the live catalog snapshot; figures read `row.asset`, caches are keyed by asset object (evaluator `WeakMap`, `prepareModel` `WeakMap`, 3D build key includes a per-object serial), so a reloaded category with a new revision redraws and rebuilds.
- A pick the new revision no longer has is named ("This asset no longer has ...") and its default plays; the stored pick is kept (not silently rewritten); the clip list offered is the new one.
- A fresh `StaffageClock` per page, paused when the page is not the opened one or motion is reduced; background copy of the card stays an inert preview (no 3D, no timeline).

## Changed and added files (react target `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`)

Changed: `🔨️modules/🧪️playground/🟦️.tsx` (rewritten as shell), `🧱️compose/🟦️.ts` (`linePosition` takes the ground extent), `📤️export/🟦️.ts` (tracks, depth), `💾️persistence/🟦️.ts` (track, depth, camera, timeline; `SECTION_VIEWPOINTS`; mirroring no longer rewritten by the reducer), `🗺️routing/🟦️.ts` (tabs, tokens, problems), `📍️placed/🟦️.tsx` (`track`), `🧊️spatial/🟦️.ts` and `🏭️prepare/🟦️.ts` (`scales`), `🧊️spatial/🚪️gate/🟦️.tsx` (`SpatialPlayground`), `⭐️collections/🧮️actions/🟦️.ts`, i18n `🔠️english/🧪️playground.ts` and `🔡️german/🧪️playground.ts`; `../../README.md` (Playgrounds section).

Added: `🧪️playground/🎞️timeline/🟦️.ts`, `🎞️timeline/🖼️panel/🟦️.tsx`, `🎛️asset/🟦️.tsx`, `🖼️stage/🟦️.tsx`, `📐️geometry/🟦️.ts`, `🧊️model/🟦️.tsx`, `🧊️model/🧩️compose/🟦️.ts`, `🧊️model/🎞️retime/🟦️.ts`.

Tests: new `🧪️tests/🎞️playground-timeline` (19 cases, three and `evaluate` as oracles) and `🧪️tests/🧪️playground-tabs` (11 cases: tabs, drag in plan, per-figure playback, timeline keyboard, transport, catalog revision, 3D ready / skipped scale / cancel and retry, German), each with a `🥒️.feature`. Updated `🧭️viewpoint-pages` (two section viewpoints, ground extent, `exact: false` label lookup), `🔭️viewpoints` (mirroring now kept in the scene, reported per tab), `🗺️address-roundtrip`, `💾️favorites-persistence`, `🧪️playground-compose` for the new `Placement`/`Playground` fields.

## Verification

- `repo tools exec tsc -- -p tsconfig.json --noEmit` clean for the react and the site package.
- react tests: all playground, viewpoint, address, favorites, collections, model-view and model-viewer cases pass (142). The full react run still has failures outside this work (`🧷️shared-borders`, `📏️range-span`, `📖️detail-sections` progress case, `🧊️model-downloads` export case: chrome / NumberField label structure changed by someone else's edit of `🪟️chrome`).
- site tests (`🧊️model-split`, stale chunk, host document) pass.
- Release build served on 127.0.0.1:6073: 3D tab with five placed figures prepared and shown on WebGL2 (floor grid, perspective camera), floor plan with metre grid and axes, timeline with five rows and playhead, selected-figure panel; no application errors in the console (the 404 / connection-refused lines are asset requests of the host, not of this page).

## Known limits

- Mirroring applies to the side elevation and the floor plan (across the forward axis); a mirrored front or 3D would be another drawing, so mirrored placements are drawn unmirrored there and the page says so.
- Yaw is drawn in the floor plan (2D turn about the ground point) and in 3D; the side and front elevations do not turn a figure, since the 2D evaluator cannot.
- The plan's pointer hit box stays the unturned footprint box.
- SVG export is for the two drawings; the 3D scene is shared by link.
- The 3D playground builds one scale at a time (`scales: [scale]`), so changing the drawing scale prepares again (cached per asset thereafter).
