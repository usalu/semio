# Lane `elements` handoff

All paths under `🧰️framework/🔨️modules/🖱️ui`. Coded against design.md and `compositor-handoff.md`.

## Per element: outside the rectangle before, inside now

| Element | Before (outside the rectangle) | Now (inside) |
| --- | --- | --- |
| `🖱️ContextMenu` | One stepped polygon: primary column plus fused wings measured by `contextMenuFusion`; cap row above the body; whole menu clamped as one body | Primary column = one `ContextMenuChrome` frame (title chip overlaid on its top row); every submenu wing = its own chipless `ContextMenuWingFrame` frame, `position: fixed`, left edge on the parent frame's right edge (flipped to the start side when overflowing, via `contextMenuSubmenuPlacement`) so the compositor resolves one coincident line; each frame scrolls its own `data-context-menu-scroll` menu. Fusion metrics, `data-silhouette-remeasure`, `context-menu-fusion-*` removed; folder `🔗️fusion` -> `🪜️submenu` (`contextMenuOpenWings`, `ContextMenuWing*`) |
| `🃏️CardStack` / `CardGrid` | Cap row of the lower card lifted onto the footer row of the upper card (`--card-stack-lift/cap/footer`, negative margin, band model `resolveCardBands`) | Cards are abutting rectangles, chips inside each. `useCardInterlock` only snaps heights to device pixels. `resolveCardBands`, `CardBand*` and their cases removed; `chipGapReadsAsSliver`, `snapCardHeight` stay in `bands.ts`. Lift/cap/footer CSS rules removed from `🎨️styling/🖌️ui/🎨️.css` |
| `🃏️OverviewCard` | Title chip row above the body, footer chips below | Title chip and footer chips are glass overlays inside the frame, body keeps clearance; hover value `"frame"` |
| `📑️Tabs` | Tab list a row above a borderless panel, selected tab outlined on all four edges (separate box) | Root is one four-edge rectangle; tab list is a glass chip (`data-tabs-chip`) overlaid at the start edge, panel spans the frame with `--tabs-list-clearance` padding; selected tab draws only its inner edges; only the selected trigger registers a frame |
| `🧭️PanelTabBar` | Tab row as chrome cap | Same component, now the content of the chip overlay of `Panel` (compositor chip); doc comments only |
| `🖼️Panel`, `📌️WindowPane`, `🪟️Window`, `🔝️Navbar`, `🔚️Footer`, `🔍️WindowSearch`, `📐️Layout`, `🗨️Popover`, `🔽️Select`, `🪢️SharedBorderRow` | Used cap/footer chips through `WindowChrome` or named silhouette | No code needed: they pass `titleChips`/footer chips to `WindowChrome`, which overlays them; tests and comments renamed (`data-window-frame*`) |
| `🎨️Canvas/⚓️dock` | `gapProps` (`mode-dock-tab-gap`), `silhouetteSlot`, `pointer-events-none` stack so gaps passed through | `frameBorderSlot="mode-dock-frame-border"`, no gap, stack no longer `pointer-events-none` (chip rows are, content plane is auto): pointer between chips reaches content |
| `🦴️Skeletons` | U-cutout windows | Two frames, title chip skeleton overlaid; `frameBorderSlot` |
| `🌈️Surface` | Gap hit test `isSurfaceActiveBackgroundPointer`, cutout geometry | Removed; a press anywhere in the frame (between chips too) activates it. Removed from the export lists in `🧷️panel-exports`, `📦️packages/.../📥️source` (name-only edits) |
| Introduction + geometry | Selectors `data-window-silhouette*`, gap | `data-window-frame*`; geometry doc |
| Stories | `silhouette-visual-*` ids | `frame-visual-*` (story and fixture) |

## Tests rewritten

`🌈️Surface/🧪️tests/frame-press.test.tsx` (new, replaces press-geometry), `🃏️CardStack/🟦️.test.tsx`, `bands.test.ts`/`bands.cases.json`/`bands.schema.json`, `interlock.feature`, ContextMenu `placement.tsx/.feature` and component test, OverviewCard (chips inside frame), Skeletons, Tabs boundary, Window (plane spans frame, no clip-path; fixture `expanded-actions-clip.json` deleted), Canvas `shared-border-dock.test.tsx` (frame rectangle plus chip inner edges at inset 1), Popover/Select boundary tests and features.

## Verified

`sh compositor-vitest.sh` with `COMPOSITOR_CONFIG=elements-vitest.config.ts` (this folder; lists the 18 test files of the lane): 136 pass, 2 fail. The two failures are `📐️Layout` tests that launch Playwright chromium (browser not installed here), not related. `repo run @semio-tech/ui-react:test` could not read the project graph (a stale import of the deleted `WindowSilhouette` at that time); `repo test-file` runs plain bun without jsdom.

## Open (other lanes)

- `🧪️tests/🎚️config` include list lacks `Surface/frame-press.test.tsx`, `CardStack/🟦️.test.tsx`, `bands.test.ts`, `chip-row.test.ts`, `Tabs/.../🟦️.tsx` (lane `uitests`).
- Rust: `🃏️CardStack/bands.rs`, `bands.test.rs` (reads removed `cases` of `bands.cases.json`), `stack.test.rs`, `🃏️CardStack/🎯️targets/🧊️wgpu/🦀️.rs`, `🎯️targets/🧊️wgpu/🪶️light/🦀️.rs` (`card_bands`), wgpu paint `text-and-structure-nodes` still use `resolve_card_bands`: drop the band model, keep `chip_gap_reads_as_sliver` and `snap_card_height` (lane `wgpu-ui`). `🪟️Window/🎯️targets/⌨️tui/🦀️.rs` still names silhouette (lane `tui`).
- `🧪️tests/🧪️owned-locale-detector-retirement/*` (contextmenu-and-more uses `context-menu-fusion-body`; control-chrome-2 and uiintroduction-appearance-1 query gap slots), `🧪️tests/📚️storybook-new-stories` (`gapElements`).
- Outside the UI module (not in this lane): print `semio-window.sty`, pets `.card .tab`, course/teaching/play card CSS, OS canvas footer tabs.
- No browser check done.
