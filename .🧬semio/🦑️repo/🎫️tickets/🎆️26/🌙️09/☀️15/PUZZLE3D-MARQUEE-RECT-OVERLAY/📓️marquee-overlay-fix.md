# Puzzle 3d marquee rectangle not visible

## Symptom

Rectangle/lasso marquee selection in puzzle 3d updates selection in the 3d viewport (objects highlight / `interactionSelect` commits) but the drag rubber-band rectangle is not painted.

## Root causes

1. **wgpu:** Pointer routing lives on `WorldInteractionAuthority::marquee`, but paint still read legacy `marquee_active` / `marquee_points` (test-only). Fixed via `world_marquee_overlay_points` + overlay lane paint.
2. **React (puzzle3d default):** `selection.method` is `pick` (click picks, drag marquees). Marquee hit-testing already treats `pick` like a rectangle, but `world3dMarqueeOverlayShape("pick")` returned `null`, so `SelectionMarquee` never mounted while drag preview highlighting still ran.

## Fix

- `world_marquee_overlay_points` reads the live authority gesture (falls back to legacy fields for oracle tests).
- Marquee paint uses the overlay draw lane (`overlay: true`), matching node-graph marquee parity.
- `world3dMarqueeOverlayShape` maps `pick` → `rect`; `World3dHost` paints marquee at `z-50`.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
