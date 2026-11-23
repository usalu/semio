# 🧾 Summary: Rectangular Frames with Glassy In-Content Chips

The silhouette model (body rectangle, chips sticking out, gap cutouts, polygon clip) is replaced across the monorepo by the frame model: one rectangle, content edge to edge, chips as glass overlays inside the frame. Design and frozen API: `design.md`. Per-lane detail: the `*-handoff.md` files.

## What changed

| Area | Result | Detail |
| --- | --- | --- |
| Contract | `window-frame-geometry/v1` schema, 12 cases, TypeScript and Rust geometry, `kurbo` oracle; border presentation renamed to `outlines.*` | `contract-handoff.md` |
| React compositor | `WindowChrome` rewritten: chip rows are overlays, no gap elements, no `clip-path`, `frameBorderSlot` | `compositor-handoff.md` |
| React elements | CardStack, OverviewCard, Panel, PanelTabBar, WindowPane, Window, Canvas dock, Skeletons, Popover, Select, ContextMenu, Tabs, Layout, introduction; `Canvas.bottomChips`, `Layout.canvasChips` | `elements-handoff.md`, `wave2-react-handoff.md` |
| Styling | `.window-frame-*`, clearance variables, chip glass rules; gap rules removed | `compositor-handoff.md` |
| wgpu renderer | Stencil union mask, `SilhouetteMask` and silhouette clips removed; `push_scissor`, `push_glass`, `push_shared_border`, `end_clip` | `wgpu-ui-handoff.md`, `wgpu-ui-api.md` |
| OS Rust | Dock module `🔲️window-frame-and-stack-geometry`, Shell `window_frames`, fixtures | `wgpu-os-handoff.md` |
| TUI | One box with chips inside; fixture `window-contour` | `tui-handoff.md` |
| Print | Chips inside rectangles in `semio-window.sty`; opaque fallback switch | `print-handoff.md`, `evidence/` |
| Products | os, course, pets, demonstrator, play, teaching clip presentation and visuals | `products-handoff.md` |
| Unrelated fixes | TypeScript errors, fixture rename, stale debug print in the job framework, PresenceBar test, CardStack case path | `unrelated-ts-fixes.md`, `wgpu-job-retirement-handoff.md` |

## Verification

| Check | Result |
| --- | --- |
| Leftover old names (whole repo) | None outside tests that assert their absence |
| UI React suite (native vitest) | 1629 passed natively; with the provisioned Playwright Chromium the stock config runs 55 files, 1362 tests, 0 failed (the 5 Chromium tests pass) |
| Storybook live (UI scope, 12 stories) | All chips inside their frame, glass on every chip, no clip paths, no gap elements; one bug found and fixed (zero-height frame); `live-react.md` |
| Storybook Playwright story tests (UI scope) | 67 of 68 pass after the Mode fixture was reconciled (`mode-separator-handoff.md`; Mode test run alone, the other 66 from the earlier full run); `TiledMapHost image render` needs a built wasm engine |
| Python teaching clip | geometry 8, presentation 15, visuals 10, interaction 21 passed |
| UI and OS typecheck | 0 errors |
| WindowChrome | 111 passed |
| Demonstrator | 105 passed |
| Rust `semio-ui-window-frame` | 4 passed |
| Rust border presentation | 14 passed |
| Rust render / shader contract / draw contract / webgpu / metal | 32 / 12 / 34 / 14 / 5 passed |
| Rust draw execution / pipelines | 22 / 1 passed |
| Rust `semio-ui-wgpu-light` (CardStack frames, PresenceBar) | 29 passed |
| Rust `semio-ui-wgpu-paint-laws` | 110 passed |
| Rust `semio-ui-wgpu-mounted-layout` | 24 passed (the close-owner test fixture compared the turn total instead of the fresh receipt; `remaining-rust-failures-a.md`) |
| TUI Rust | 102 / 100 / 8 passed; emphasis 5 passed |
| OS dock Rust | dock 2 passed, dock laws 76 passed (all rewritten frame laws), rerun after the shared dock fixture dropped separators and canvas inset (`mode-separator-handoff.md`) |
| OS Shell Rust | All nine Shell crates compile. shell-input 5, shell-view-chrome 6, shell-tests-chrome-2 10 passed. shell-tests-chrome-4: 98 passed, 4 failed. shell-tests-input-1: 24 passed, 49 failed. No failure involves frames, chips or hit-testing: they come from value-arena, document-lease and chrome-walk changes other agents have in flight (`wgpu-os-handoff.md`) |
| Print | PDFs compile with the pinned tectonic; page chrome test and 22 pipeline assertions pass |
| Live React (staffage site) | 21 frames, 62 chips, 0 chips outside a frame, 0 clip paths, 0 gap elements; `live-react.md` |

## Not verified

- D3D12 and Vulkan backends: cannot compile on macOS.
- The full OS shell with plugin apps in a running app: the OS playground could not be served (the repo graph service failed while reading the project graph, and it needs built wasm engines). Dock, canvas, layout, window and context menu were checked live in Storybook instead.
- `TiledMapHost image render` story test (needs a built wasm engine), `repo run` test targets (graph read stalls), pets tab CSS in a browser.
- OS renderer vitest suite has 41 failures unrelated to this work (Chromium, missing built wasm, stale package catalog digest).

## Notes

- `repo cargo` hung twice for hours with idle compiler wrappers; cancelled and rerun sequentially.
- A lane ran `git rm --cached` on `🃏️CardStack/bands.rs` against the rules; the file is tracked and modified, the index was checked read-only.
- Document-like OS panes start below the chip-row clearance; panes with an engine host are full-bleed.
- Unblocking the Shell crates needed fixes outside this task: media transport presenter, shell actions effect sink (`Mutex` on native), shell pointer import, host-winit dependency, theme editor and quick search tests.
