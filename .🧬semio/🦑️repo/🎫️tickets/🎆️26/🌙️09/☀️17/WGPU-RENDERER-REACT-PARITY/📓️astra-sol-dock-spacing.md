# Dock And Navbar Geometry Packet

## Result

The WGPU Dock now uses one allocation-free axis iterator for layout, painting, scene-body publication, tab bars, drop geometry, and resize hits. For `n` children it clamps the themed separator to the physical axis, reserves `(n - 1) * separator`, normalizes finite positive weights over the remaining extent, and assigns floating-point residue to the last child so the final edge is exact. Empty and single-child axes reserve no separator. Maximized stacks bypass axis separation.

The separator is transparent, matching React's `ResizableHandle`, while the existing wider physical resize hit is centered on it and clipped to the containing axis. Split-drag conversion uses the post-separator distributable extent. Shell captures the frame's `theme.gap_standard` as the same authority used by Dock before pointer routing.

React's `WindowChrome` body padding has been carried into WGPU scene publication as horizontal `theme.padding_standard` on each stack body. The paired runtime evidence already matched React's vertical scene bounds, so WGPU deliberately does not add a second vertical inset. The stack silhouette, glass, clipping, and cap geometry still own the full stack rectangle.

The navbar leading band now has one x-position authority. `render_navbar_step` advances the retained cursor; `navbar_leading_tab_row_rect` measures only the current tab at that cursor. It no longer re-adds every earlier tab width. The three-tab law includes a label that spans retained glyph steps and checks contiguous physical rectangles after resumption.

Panel chrome chips now match the actual React `PanelChromeTabBar` structure: leading icon, label, and a trailing `grip-vertical` drag handle, all measured from theme/icon tokens. The same measured width drives navbar, footer, open anchor tab rows, insertion preview, and physical drop-index resolution. Ordinary pane actions and other chrome buttons keep the ordinary chip geometry.

## Shared Geometry Contract

The schema-first neutral contract is:

- schema: `🧰️framework/🔨️modules/🖱️ui/🧬️schema/📐️dock-axis-geometry/🔣️.json`
- fixture: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️dock-axis-geometry/🔣️.json`
- language-neutral Bun/Ajv law: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📐️dock-axis-geometry/🟦️.ts`
- installed React component oracle: `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️mode/🧪️.story.tsx`, story `GeometryOracle`
- physical Playwright law: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts`

The fixture uses an 800×500 canvas, a named 3.2 spacing token, a 35/65 row, and a nested 70/30 column. Its expected frames are fixture vectors rather than production constants:

| stack | x | y | width | height |
| --- | ---: | ---: | ---: | ---: |
| left | 3.2 | 3.2 | 276.64 | 493.6 |
| right-top | 283.04 | 3.2 | 513.76 | 343.28 |
| right-bottom | 283.04 | 349.68 | 513.76 | 147.12 |

The React browser law reads the live `--spacing-single` value from the mounted DOM and checks canvas inset, horizontal and vertical handle extents, post-separator proportions, exact group coverage, and `WindowChrome` CSS padding. A second mounted provider story checks that the panel tab's trailing drag handle has the same physical icon size as its leading icon and that the button width contains label, gaps, both icons, padding, and border.

## Production Boundaries

The Dock production change is in:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`

The Shell production change is in:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs`

`stack_frame_rects` remains explicitly separator-free for the older theme-neutral fixture. Runtime consumers use the themed iterator, and `stack_frame_rects_with_separator` exposes the physical contract to the shared-fixture Rust law.

No camera, shader, window publication, guest lifecycle, or reference-image behavior changed in this packet.

## Verification

Executed language-neutral law:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace --skip-nx-cache -- bun test './🧰️framework/🔨️modules/🖱️ui/🧪️tests/📐️dock-axis-geometry/🟦️.ts'
```

Result: 2 passed, 0 failed, 15 assertions.

The scoped diff check is clean. `rustfmt --check` parsed the edited Rust sources without a syntax failure, then reported the shared repository's existing formatting differences. No Cargo/native/wasm job was started from this packet.

The actual React DOM oracle is authored but not executed in this packet because root owns the shared Storybook/browser build queue. Its canonical command is:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run workspace:test-storybook --skip-nx-cache
```

The target uses the root `📜️script.ts` to build Storybook, serve the static output on an allocated local port, and run the repository Playwright browser runner.

Native integration still needs root's Dock unit and Shell navbar/footer parity targets plus the renderer compile. The new Shell scalar can change measured owner-byte receipts; any budget update must use the native guard's actual value. Native21 and activation13 predate this geometry packet and are not acceptance evidence for it.

## Runtime Acceptance

After canonical activation, the paired journey must confirm:

- sibling scene viewports have exactly one themed separator between them;
- published scene bodies, split previews, drop targets, and resize hits agree with the visible frames;
- horizontal scene content inset matches React without shifting the already-matching vertical edges;
- three leading navbar tabs are contiguous after retained glyph resumption;
- panel tabs expose the trailing drag handle without clipping their labels or changing ordinary chip geometry.
