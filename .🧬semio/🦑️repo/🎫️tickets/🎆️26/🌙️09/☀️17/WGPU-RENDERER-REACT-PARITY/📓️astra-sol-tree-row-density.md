# Sol Tree Row Density

## Scope

This packet repairs the WGPU Actions and Search pane Tree row density, clipping, and scrolling against the current React implementation. It does not change the already validated 160 px inline-control width, 16 px inline-control height, or 22.4 px ordinary control height.

## Evidence and cause

The independent browser capture at `🗑️generated/astra-runtime/tree-style-checkpoint-15/tree-style.json` records:

- `action.clearSelection` and `action.selectAll` at 24 px height and 24 px pitch;
- a 16 px Tree row icon;
- a 1,344 px intrinsic Tree content height;
- a scrollable ancestor rather than compressed rows.

The pre-repair WGPU checkpoint published roughly 12.042824 px rows. The cause is the flex solver rather than the Stack classifier: Tree, TreeSection, and TreeRow authored fixed heights inherited `FlowStyle::default().shrink = 1.0`. Negative free space therefore divided the viewport deficit across every supposedly fixed band. The fixed-band style now sets `shrink = 0.0`; overflow remains intrinsic and belongs to a Scroll root.

## Contract and fixtures

Added the language-neutral schema and fixture:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🌳️tree-row-density/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-density/🔣️.json`

The fixture names the first two Actions rows, the terminal Abort row, 24 px row geometry, 16 px icon geometry, 55 action rows plus one category row, 1,344 px intrinsic content, a 480 px viewport, and a bounded scroll distance.

The existing TypeScript oracle `engine/🧪️tests/🌳️tree-row-rects/🟦️.ts` now validates that fixture with JSON Schema 2020, checks the actual React capture, checks React's fixed/min row and icon source styles, and derives the captured intrinsic height independently as `(1 + 55) * 24 = 1344`.

## Production repair

- Fixed Tree bands opt out of flex shrink in the WGPU layout solver.
- Actions and Search documents publish a vertical `LayoutSpec::Scroll` root.
- The Actions body, like Search, caps its visible pane to the available window body while retaining intrinsic content height.
- Retained scroll dispatch clamps offsets to the measured direct-child content extent.
- Shell wheel input dispatches a real retained `UiEvent::Scroll` to the pane owner.
- Retained paint and hit publication inherit Scroll/clip ancestors. Ordinary content is clipped to the pane; an open Select owns its popup route outside that ancestor clip, and its popup rows remain overlay-owned in the host registry.
- Host hit publication intersects ordinary retained targets with the physical pane body, so off-viewport rows cannot receive pointer input.
- The clipping walk uses a fixed 64-entry ancestor array. The new overlay bit lives only in the heap-backed hit registry element; `RetainedPaintFrame` and `UiSurfaceSlot` gain no fixed-size field.

## Laws

The Rust laws cover:

- a 1,344 px Tree inside a 480 px Scroll viewport retaining 24 px rows and pitch;
- bounded scroll offsets, including reverse-to-zero and terminal maximum clamping;
- both Actions and Search publishing vertical Scroll roots;
- a full Shell Actions build and normal chrome walk publishing the first two rows at 24 px, initially clipping Abort, revealing and painting Abort after wheel input, then restoring the first row and clipping Abort after reverse wheel input.

## Validation receipt

Focused TypeScript through Bun and Nx:

`NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run <tree-row-rects> --config <wgpu-vitest-config>`

Result: **1 file passed, 9 tests passed**, 2026-09-20 10:58 CEST.

All edited Rust sources were parsed with `rustfmt --edition 2021 --emit stdout`. Cargo/native, WASM activation, and browser runtime validation remain root-owned and are not claimed here.

Exact native filters for the root lane:

- `fixed_tree_bands_keep_their_intrinsic_pitch_inside_a_short_scroll_viewport`
- `scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero`
- `actions_and_search_publish_vertical_scroll_roots`
- `the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it`

The root browser probe independently has a React-only 5/5 reference receipt for 24 px first rows, initially clipped Abort, wheel reveal, and reverse restoration. A fresh WGPU runtime receipt must follow the next activation.

## Files changed in this packet

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🌳️tree-row-density/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-density/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️tree-row-rects/🟦️.ts`
