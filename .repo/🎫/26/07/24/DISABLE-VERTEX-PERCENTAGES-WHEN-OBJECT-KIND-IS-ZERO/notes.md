# Disable Vertex Percentages When Object Kind Is Zero

When an object-kind distribution weight is `0`, joint vortex (vertex) percentages are always `0` because `P(object)×P(vortex) = 0`. The fill/brush distribution UI must disable those joint sliders.

## Approach

1. Add optional `disabled` on `WindowMeasure::Slider`.
2. Emit `disabled: true` for joint vortex sliders when the parent object-kind weight is `≤ ε`.
3. Ignore `setVortexKindWeight` when the parent object kind weight is zero.
4. Wire `disabled` through React `WindowMeasureSlider` / `Slider` and wgpu `render_slider`.
