# Redraw nodes camera — hypotheses (rejected vs likely)

## Observed symptom

Clicking **Redraw nodes** jumps the view to a bbox-framed graph instead of easing from the **currently shown** camera.

## Shell model (facts)

- Each `BoardCanvas` receives `camera={camerasByPane[pane]}`.
- `camerasByPane = cameraDisplayOverrideByPane ?? boardPlayPaneCamerasBaseline`.
- One-shot redraw uses `patchFixture` + a RAF loop intended to blend **baseline** cameras toward `triptychCamerasFromFixture(fixture)` while keeping `fixture.camera` from the pre-layout fixture so JSON/WASM stay aligned with the shell.

## Hypothesis A — RAF killed by another effect (partially confirmed, fixed once)

`useEffect([boardRedrawPlaying, fixture])` previously cancelled `boardPlayNodesRedrawCameraAnimRafRef` on **every** `fixture` change. That ran in the same commit as `useLayoutEffect` that scheduled the ease → instant snap. **Mitigation:** remove that cancel from the `fixture` churn branch.

**Status:** still insufficient if users still see snap → not the only race.

## Hypothesis B — `from` captured from wrong render (likely)

The ease read `boardPlayPaneCamerasBaseline` inside `useLayoutEffect` on the commit **after** `patchFixture`. In theory that baseline should still be the pre-redraw shell cameras, but any ordering/batching quirk or Strict Mode re-run can make “ease `from`” diverge from what the user was actually seeing (especially if we later add `onCamera` sync).

**Rework:** snapshot **`from` into a ref at the start of `applyBoardRedrawOnce`, before `patchFixture`**, using the same `camerasByPane` the canvases use at click time. The RAF loop reads only that ref + `fixtureRef` for `to`.

## Hypothesis C — layout vs effect ordering (plausible)

`useLayoutEffect` runs before `useEffect`. Any `useEffect` that reacts to the same commit can still run after layout and interact with RAF ordering in subtle ways.

**Rework:** start the nodes-redraw RAF loop from **`useEffect([nodesRedrawCameraEaseTick])`** instead of `useLayoutEffect` so the ease starts **after** other effects on that tick (one frame of “old camera + new node positions” is acceptable and matches “from active camera” literally).

## Hypothesis D — driving ease via `cameraDisplayOverrideByPane` (rejected for now)

Reusing the post-play override channel would collide with `useEffect` that cancels nodes RAF whenever `cameraDisplayOverrideByPane !== null` unless we add a discriminant (post-play vs nodes-redraw). **Decision:** keep easing **baseline only**; do not move nodes ease onto override without that refactor.

## Hypothesis E — WASM `camera` drain events (unknown)

If the WASM host ever emits `camera` rows into `applyWasmDrainToScene` on descriptor sync, the imperative renderer could jump before React props ease. **Not observed in `sync_descriptor` path** (no `set_camera` there). Keep as watch item if future host changes add auto-fit.

## Implementation checklist (this ticket)

1. Ref `nodesRedrawEaseFromRef`: snapshot triptych `from` **before** `patchFixture`.
2. Ref `nodesRedrawEaseGenerationRef`: invalidate in-flight RAF ticks on redraw play start, post-play cancel, override-arm cancel, and each new redraw click.
3. Replace nodes ease `useLayoutEffect` → `useEffect`.
4. Re-verify `useEffect([boardRedrawPlaying, fixture])` does not cancel nodes RAF on fixture-only updates.

## Hypothesis F — ease effect deps must not include `cameraDisplayOverrideByPane` (confirmed)

If the nodes ease `useEffect` listed `cameraDisplayOverrideByPane` as a dependency, then when post-play finished and cleared override (`null`), the effect re-ran **without** a new `nodesRedrawCameraEaseTick` and could start a **second** ease from a stale `nodesRedrawEaseFromRef`.

**Rework:** guard with `cameraDisplayOverrideRef.current` (mirrored each render) and depend **only** on `[nodesRedrawCameraEaseTick]`.

## Landed code changes (`elements/client/lib/board/play/index.tsx`)

- Snapshot `camerasByPane` into `nodesRedrawEaseFromRef` at the **start** of `applyBoardRedrawOnce` (before `patchFixture`); ease reads this ref instead of `boardPlayPaneCamerasBaseline` from a later render.
- `nodesRedrawEaseGenerationRef`: bump on each redraw click, on post-play arm (with nodes RAF cancel), on override becoming non-null, and on redraw play start; RAF tick bails when generation differs.
- Nodes ease starter: `useEffect` (not `useLayoutEffect`), deps **`[nodesRedrawCameraEaseTick]`** only; play/override/suppress read from refs (`boardRedrawPlayingRef`, `cameraDisplayOverrideRef`).
- Post-play path: bump generation once when starting override so any in-flight nodes RAF ticks become no-ops even if cancel races.
