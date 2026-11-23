# 🔭 Live React Check

Served app: staffage site (`staffage-site-native` launch entry, Vite prototype mode, `http://localhost:6072/`), overview page, 2026-11-23. It renders `OverviewCard` grids on top of `WindowChrome`.

Measured in the page:

| Law | Result |
| --- | --- |
| Frames (`[data-window-frame]`) | 21 |
| Chips (`[data-window-frame-chip]`) | 62 |
| Chips with any edge outside their own frame rectangle (1 px tolerance) | 0 |
| Content planes that do not span the full frame (1.5 px tolerance) | 0 |
| Content planes with a `clip-path` | 0 |
| Old `[data-window-silhouette]` elements | 0 |
| Gap elements (`window-chrome-gap`) | 0 |
| Chip surface | `background: color(srgb 0.197 0.256 0.264 / 0.4)`, `backdrop-filter: blur(40px) saturate(1.45)` on top and bottom chips |
| Console errors | none |

Seen in the screenshot: cards form a gap-free grid of rectangles; the title chip sits inside the top-left corner and the count and "Explore" chips inside the bottom edge, with the card drawing visible behind them.

Not covered here: Storybook stories, the OS shell, context menus, the dock, wgpu, TUI and print.

## 📚 Storybook Check (UI scope)

Served natively with the `os-shell-frames-native` launch entry (`http://127.0.0.1:6090`, recipe in `os-shell-live-recipe.md`), 2026-11-23. Measured per story: frames, chips, chips outside their frame, content planes with a `clip-path`, chips with a backdrop filter, gap elements.

| Story | Frames | Chips | Outside | Clip paths | Glass chips | Gaps |
| --- | --- | --- | --- | --- | --- | --- |
| `mode--tab-stack` | 2 | 2 | 0 | 0 | 2 | 0 |
| `mode--maximize-stack` | 4 | 4 | 0 | 0 | 4 | 0 |
| `mode--quad-layout` | 8 | 8 | 0 | 0 | 8 | 0 |
| `window--default`, `--with-controls`, `--with-utility-bar` | 1 each | 1 each | 0 | 0 | 1 each | 0 |
| `providers--panel-dock-story` | 2 | 2 | 0 | 0 | 2 | 0 |
| `layout--panel-default` | 1 | 1 | 0 | 0 | 1 | 0 |
| `canvas--default` | 2 | 2 | 0 | 0 | 2 | 0 |
| `layout--default` | 6 | 6 | 0 after the fix below | 0 | 6 | 0 |
| `contextmenu--default` (opened by right click) | 1 | 1 | 0 | 0 | n/a | 0 |

Found and fixed: in `layout--default` two open panels without content had a frame of height 0 that stayed `pending`, with the chip row hanging 24 px below it and clipped away. A frame is now never shorter than its chip rows (`min-block-size` rule on `[data-window-frame]` in `🎨️styling/🖌️ui/🎨️.css`); both frames are 24 px high, `ready`, and contain their chip.

The context menu is one rectangle; its border path draws the four frame edges plus the two inner edges of the "Actions" chip.

Not covered: the OS shell with plugin apps (needs built wasm engines and plugin components), `paneltabbar--panel-variant` and `footer--default` render no frame of their own.
