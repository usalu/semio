# Sol Compact Tree Geometry

## Scope

This packet brings compact retained Tree layout and hit geometry to the mounted React Window Measures contract. It keeps the standard Tree presentation unchanged.

## Contract

The existing schema-validated neutral contract is:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🌳️window-measures-tree-parity/🔣️.json`
- schema: `semio.window-measures-tree-parity/v1`
- actual React oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎚️window-measure-controls/🟦️.tsx`

The contract pins label/group rows at 14.4 px, Select/Input rows at 16 px, Slider/Checkbox rows at 22.4 px, and the compact value column at 104 px. Root recorded the real mounted DOM evidence in `🗑️generated/astra-runtime/checkpoint20-iab/react-window-options-metrics.json`; its earlier React oracle receipt was 8/8.

## Production boundary

`TreeRowMetrics` now derives from the Tree's `TreePresentation`, carries inline flow, and derives a row copy for the authored item control. Compact label rows use `SIZE_TINY * 1.5`; Input and Select use `control_height_small`; other controls use `control_height`; standard rows preserve the theme's original geometry.

Mounted layout resolves the nearest owning Tree and authored item before it measures a row, calculates nested retained height, or feeds the flex solver. `MountedLayoutJob` carries the window's inline flow. Tree inline controls are vertically centered against the resolved row height and anchored to the matching physical edge.

Paint and retained-hit registration receive the same router inline flow. Disclosure header bands, chevrons, drag handles, control columns, and flex insets derive from the same scoped metrics. RTL puts the value column on the physical left and mirrors the chevron and drag handle.

## Source validation

The following files parse with `rustfmt --edition 2021 --emit stdout`:

- `layout/🦀️.rs`
- `mounted_layout/🦀️.rs`
- `flex/🦀️.rs`
- `events/🦀️.rs`
- `engine/🦀️.rs`
- the updated Tree row and mounted-layout test modules

Native142 initially stopped at compile because the offset retained-paint hit-registration call lacked the new inline argument. That exact second call now passes `window.router.flow().inline`; both registration paths match the new signature. No Cargo result is claimed by this packet. Root owns Native142/UI23 and later runtime validation.

## Files changed

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs`

