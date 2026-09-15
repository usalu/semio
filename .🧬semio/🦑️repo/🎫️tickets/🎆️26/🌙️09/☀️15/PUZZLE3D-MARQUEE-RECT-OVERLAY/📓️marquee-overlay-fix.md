# Puzzle 3d marquee rectangle not visible

## Symptom

Rectangle/lasso marquee selection in puzzle 3d updates selection in the 3d viewport (objects highlight / `interactionSelect` commits) but the drag rubber-band rectangle is not painted.

## Root cause

Pointer routing was migrated to `WorldInteractionAuthority::marquee` (`WorldMarqueeGesture`), while `render_world_3d` still gated `paint_selection_marquee` on legacy `World3dState::marquee_active` / `marquee_points`. Those fields are only updated by `#[cfg(test)]` pointer handlers, so production wgpu (and any host using the authority) never had overlay points at paint time.

## Fix

- `world_marquee_overlay_points` reads the live authority gesture (falls back to legacy fields for oracle tests).
- Marquee paint uses the overlay draw lane (`overlay: true`), matching node-graph marquee parity.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
