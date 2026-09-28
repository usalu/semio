# Retained Tree Row Occlusion

WGPU27 publishes complete projection hit/AX rows but still visibly paints blank opaque bordered strips. The previous projection law only asserted hits and dimensions, so it could not catch missing text.

Source cause: the owning Tree paints all labels, icons and selected/hover backgrounds. Its reconciled identity rows are Stack nodes with activation descriptors. The later retained walk paints these same nodes with `paint_stack_frame`, which adds an opaque panel and control border over the previously emitted Tree content. This also explains the visible horizontal borders absent from React.

A new retained-frame regression consumes the existing neutral English/German Tree action fixture. It requires emitted label glyphs, no opaque control chrome on idle rows, and retained hit targets. The installed React/testing-library oracle checks the same action labels retain transparent, borderless activation buttons. Native execution is pending behind ongoing compiler jobs; no red assertion receipt yet.

Repair now skips only generic Stack control chrome on retained TreeItem/TreeSection identity rows. Their tree-owned visuals and hit/activation records remain in their established owners; ordinary activatable Stack chrome is unchanged. The test was authored before the repair; no pre-fix test execution is claimed.

React focused action tests passed: 5/5, 48 unrelated tests skipped; Vitest 9.01s, Nx 11.0s, exit 0. The exact shell Projection law now additionally checks glyphs in every published row and refuses later opaque quads covering their centers. The new WASM28 build is running for physical browser acceptance; no native/runtime success claimed yet.
