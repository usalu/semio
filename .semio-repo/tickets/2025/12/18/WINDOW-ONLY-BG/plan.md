# Previously

- Multiple UI elements and overlays painted their own backgrounds (panel/temporary/popover/window tokens), resulting in layered filled surfaces.
- GoldenLayout chrome and Sketchpad shell containers also applied background fills.

# Plan

- Keep `Window` as the only component that paints a filled background surface.
- Make all other elements and overlays backgroundless (transparent), relying on borders/blur for separation.
- Update GoldenLayout overrides so chrome is transparent.
- Update root docs to describe the window-only background mechanism.

# Changes

- Made level backgrounds transparent via `getLevelBgClass` and removed per-element background fills from core elements.
- Updated Sketchpad shell and app overlays to remove `bg-base`/`bg-panel`/`bg-temporary` usage.
- Updated GoldenLayout CSS overrides to remove window chrome background fills.
