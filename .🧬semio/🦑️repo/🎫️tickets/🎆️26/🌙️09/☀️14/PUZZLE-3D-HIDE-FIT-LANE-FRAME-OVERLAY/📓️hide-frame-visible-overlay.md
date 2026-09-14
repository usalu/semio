# Hide Frame Visible Overlay When Fit Lane Is Enabled

## Symptom

Puzzle 3d edit mode showed **Sichtbares einpassen** (`ui.host.frameVisible`) on every world viewport pane. That control is a host-level manual reframe; puzzle 3d already publishes `fitJson` with `enabled: true` and mounts `WorldAutoFit` for document-driven framing.

## Root cause

`World3dHost` always painted `[data-slot="world-frame-instances"]` regardless of the scene's fit lane (`🌐️World3dHost/🟦️.tsx` overlay rail).

Puzzle 3d sets the lane in `🧊️main/🦀️.rs` via `world3d_fit_json(world_fit_revision(...), PUZZLE3D_FIT_PADDING)`.

## Fix

`world3dFrameVisibleOverlayOffered(fit)` returns false when `fit.enabled === true`. The overlay button is omitted; compute-status cancel pill unchanged.

## Law

`engine-contract` — `world3dFrameVisibleOverlayOffered defers to an enabled fit lane`.

## Files

- `🧰️framework/…/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/…/🎯️targets/⚛️react/🟦️.tsx` (re-export)
- `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts`
