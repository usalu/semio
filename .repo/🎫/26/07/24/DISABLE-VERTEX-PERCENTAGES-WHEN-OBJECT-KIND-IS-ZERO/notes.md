# Disable Vertex Percentages When Object Kind Is Zero

When an object-kind distribution weight is `0`, vortex percentages under that kind cannot change the fill outcome because joint contribution is `P(object)×P(vortex) = 0`. Those vortex sliders must be disabled.

## Approach

1. Optional `disabled` on `WindowMeasure::Slider` (React + wgpu).
2. `puzzle3d_joint_vortex_measures` sets `disabled: Some(true)` when parent object weight `≤ ε`.
3. `setVortexKindWeight` no-ops when the parent object kind weight is zero.
4. `WindowMeasureSlider` / `Slider` / wgpu `render_slider` honor `disabled`.

Aligned with concurrent distribution-slider work: vortex rows show global `P(vortex)` on `[0,1]`, but remain disabled under a zero object kind.
