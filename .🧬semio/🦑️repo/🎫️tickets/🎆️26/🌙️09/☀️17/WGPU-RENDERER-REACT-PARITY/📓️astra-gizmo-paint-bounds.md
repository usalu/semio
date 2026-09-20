# Gizmo Paint Bounds and Projection

The completed checkpoint7 WGPU screenshots show large translucent axis-colored blocks across each viewport’s bottom edge. The real `widgets::gizmo::paint_orbit_view_gizmo` passes `[x-r,y-r,x+r,y+r]` to `DrawList::push_solid_overlay`, whose GPU rectangle contract is `[x,y,width,height]`. The defect grows with pane translation, matching the captured blocks.

Authored a language-neutral corpus and actual retained widget-paint law before changing production. The law covers three viewport positions/sizes and hovered/unhovered heads, requires fifteen bounded circular instances and centers coincident with the hit geometry. The independent Three oracle instantiates a camera, group, sprite and circle geometry to validate the visible head radius and all six signed axis projections. It exposed the existing mirrored horizontal basis and viewport-dependent axis scale.

The painter now emits radius-bounded rounded overlays with diameter dimensions. Its radius matches React’s64px circle texture,0.62sprite scale,22px group scale and1.1hover scale. The geometry now uses the camera’s right-handed view basis and the fixed22px group scale. Picking and paint consume the same corrected tip positions. Palette/opacity and shaft fidelity remain separate visual acceptance work.

## Validation

- Canonical focused React target passed1/1 Three oracle test,656skipped (57.8s total, log `🗑️generated/astra-runtime/gizmo-head-oracle.log`).
- Rustfmt parser accepted the new actual painter law.
- Native execution is pending the existing UI census; no native pass or repaired-runtime claim is made.
- Next browser activation must visually confirm removal of the blocks and correct heads before acceptance.

## Files

- `🖱️ui/🧫️fixtures/🧭️gizmo-tip-bounds/🔣️.json`
- `🖱️ui/🧪️tests/🔬️targets-wgpu-widget-metrics/🦀️.rs`
- `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs`
- `🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`
- `renderer/engine/🧪️tests/🔬️engine-contract/🟦️.ts`
