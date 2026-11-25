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
| UI React suite (native vitest) | Final run with the provisioned Playwright Chromium: 77 files, 1635 tests, all passed (includes the 5 Chromium tests) |
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
| OS Shell Rust (one consolidated pass, all crates together) | chrome-1 61, chrome-2 10, chrome-3 41, chrome-4 102, chrome-6 95, docs-1 41, docs-2 85 (11 ignored since their first commit: need a live hub, guest runtime or browser), docs-3 59, input-1 73, input-2 72, input-3 79, windows-1 71, windows-2 60, shell-input 5, shell-view-chrome 6: all passed, 0 failed. Handoffs: `shell-all-handoff.md`, `shell-docs-windows-handoff.md`, `shell-chrome-input-handoff.md`, `input1-final-handoff.md`, `remaining-rust-failures-a.md`, `remaining-rust-failures-b.md` |
| OS Shell Rust chrome-5 | 57 passed, 0 failed (own confirmation run). Product bug: a settings tree row and its inline control share one key, so a click on the Appearance Select was routed as a row click and the popup never opened. A failing test now retires its documents (`RetiringShell`), so it no longer breaks later tests. `semio-ui-wgpu-engine-laws` 72 passed (agent run). `shell-chrome5-handoff.md`, `shell-chrome5-select-handoff.md` |
| Print | PDFs compile with the pinned tectonic; page chrome test and 22 pipeline assertions pass |
| Live React (staffage site) | 21 frames, 62 chips, 0 chips outside a frame, 0 clip paths, 0 gap elements; `live-react.md` |

## Not verified

- D3D12 and Vulkan backends: cannot compile on macOS.
- The full OS shell with plugin apps in a running app: the OS playground could not be served (the repo graph service failed while reading the project graph, and it needs built wasm engines). Dock, canvas, layout, window and context menu were checked live in Storybook instead.
- `TiledMapHost image render` story test (needs a built wasm engine), `repo run` test targets (graph read stalls), pets tab CSS in a browser.
- OS renderer vitest suite has 41 failures unrelated to this work (Chromium, missing built wasm, stale package catalog digest).

## Notes

- `semio-ui-wgpu-laws` is green (lib 46, clipboard ingress 2, prepared refusal 1); the clipboard test compared the step progress with the turn total (`clipboard-and-stack-handoff.md`).
- A production stack hog is fixed: the kernel pool state (342 KB) and its turn future (722 KB) lived on the calling thread and overflowed a 4 MiB stack; the slot tables are heap-built and the turn future is boxed. The input-1 crate passes 73 of 73 on the default stack.
- When the memory limit was introduced, one agent stopped its own build with `pkill -f "repo cargo"`, which may also have ended a build of another session on this machine.
- A chip tint (`--chip-tint`, 10% foreground veil over chip glass) was added by another agent; the generated Rust constant `levels::CHIP_TINT_PERCENT` was missing and was added to the tokens file as the generator emits it.

- Product bugs found and fixed while making the Shell crates green: document ingress refused documents whose generation differed from the turn's; `dispatch_action` dropped two ordered maps without retiring them; retired 3D worlds were advanced on an ownerless context; an engine surface slot was 1.9 MB inline (a per-thread table of about 492 MB) and is boxed, as is the 156 KB paint frame, so `ShellState` is 51 240 bytes; the Shell had no arm for `Effect::ClipboardWrite`; user-visible literals now go through the English and German tables with fault codes; date-field Enter commits the typed draft; row-reorder payloads are ignored on non-tree drop targets; undo and redo accept ctrl or meta without alt; an unknown locale shows both tongues; `/hub` clears a route-not-found state.
- Memory: parallel `repo cargo` runs from several agents flooded the 16 GB machine. Since then one cargo invocation runs at a time behind a shared lock, with `-j 3`, two test threads and small package lists.
- Open flags: the German fault texts are not reviewed by a native speaker; `layout-stepping/🦀️.rs` still holds another agent's `[DEBUG] layout job` line (only made to compile); `scope_plugin_catalog_v1` is not compiled for wasm here.
- `repo cargo` hung twice for hours with idle compiler wrappers; cancelled and rerun sequentially.
- A lane ran `git rm --cached` on `🃏️CardStack/bands.rs` against the rules; the file is tracked and modified, the index was checked read-only.
- Document-like OS panes start below the chip-row clearance; panes with an engine host are full-bleed. This is decided from the declared window kind, so the first frame is already correct (`pane-first-frame-handoff.md`; dock laws now 77 passed).
- Unblocking the Shell crates needed fixes outside this task: media transport presenter, shell actions effect sink (`Mutex` on native), shell pointer import, host-winit dependency, theme editor and quick search tests.

# 🧾 Round 2 (2026-11-25): Remnants, Laws, Sites, Clips and Print

Plan, decisions and lane list: `round2.md`. Audits: `audit2-*.md`. Recipes that work on a Mac: `recipes2.md`.

## What changed

| Area | Result | Detail |
| --- | --- | --- |
| Tokens | The level knobs and `chipTintPercent` of `🎨️styling/🔣️.json` reach CSS, Rust, Python and LaTeX through the projection; no hand-copied glass or tint value is left; the theme setter can override the tint. C# stays palette-only (no C# renderer draws chrome) | `print-tokens-handoff.md`, `bands-dialect-tint-handoff.md`, `python-handoff.md` |
| React UI module | Tabs, Panel, Select, ContextMenu and Command are rectangles with tinted glass chips and ruled rows; hand-drawn borders and dead `rounded-*`/`shadow-*` removed and guarded by a test; hover adds no line through nested groups; active seams close on all four sides | `web-ui-handoff.md`, `visual-defects-handoff.md`, `finishing-handoff.md` |
| Layout | The floor runs beneath the glass navbar and footer; docks, windows and chips keep the bands as clearance | `ui-red-tests-handoff.md` |
| Drag | One shared mechanism: detached glass ghost, every drop place shown, rectangles only | `react-drag-handoff.md` |
| OS renderer | Diagram nodes, 3D labels, presence, popups, dialogs and status chips are frames or tinted glass; suite 39 failures to 7 environmental | `web-os-handoff.md`, `os-suite-handoff.md` |
| Sites | Cards no longer lift; language choosers tile the client; peer label is a chip inside the frame; overlay scroll thumb; hub admin navbar is a glass band; presentation handles square | `web-products-handoff.md` |
| Clip presentation | No inset ring and no gutters; picture content fills the complete frame beneath the chips; Table rows are compositor lines | `presentation-hub-handoff.md` |
| Python clips | One owner for frame, chips and glass; BIM, capsule, casting, trees, Excel, cooling, heating, optimization and Nakagin chrome moved to it | `python-handoff.md`, `python-call-sites.md`, `python-stills.md` |
| Print | Chip glass is two token layers at the level of what it covers; photos, figures and tables show through; portable `print-build.ts` | `print-tokens-handoff.md` |
| wgpu | Chip glass takes the covered level; navbar and footer are glass bands; Select and context menu rows; square controls | `wgpu-laws-handoff.md`, `rust-final-handoff.md` |
| CardStack, TUI | Gap and sliver rules and the chip-abut module removed; one schema dialect | `bands-dialect-tint-handoff.md` |

## Verification

| Check | Result |
| --- | --- |
| UI React suite | 83 files, 1713 tests pass |
| OS renderer suite | 3594 pass; 7 tests and 2 suites fail for the environment (flow wasm not built, pinned bun not installed, package catalog digest) |
| Storybook frame visuals in Chromium (new suite) | 9 of 9; failed 9 of 9 on the build before the fixes |
| Styling, print tokens, hub admin, presentation, demonstrator | 327, 62, 29, 4, 105 pass |
| Course | 875 pass, 1 fails (spawns the pinned bun) |
| Python clips | semio-style 14, surface 5, presentation 53, scenes 15, capsule, Grasshopper and Excel 158, cooling 3; `bim retirement` never finished (cancelled after 15 minutes), 4 primary-energy `scale.py` tests fail on staffage catalog heights |
| Rust, every crate touched | 0 failures in the last run of every group (`rust-final-handoff.md`); 11 tests ignored since their first commit |
| Print on macOS | junctions, report light and dark, assignment and photo page compile; images in `evidence/print2/` |
| Old names in the whole repo | Only in tests that assert their absence |

Images: `evidence/final/` (React, light, dark, 375 px, RTL, forced colours, reduced transparency), `evidence/web/`, `evidence/presentation-hub/`, `evidence/python/`, `evidence/print2/`.

## Not verified

- The running OS shell with plugin apps: no graph route (`repo run` stalls reading the project graph), and the app's preview server hangs opening `~/Documents` (a pending macOS folder-access decision is the likely cause). Static builds and headless Chromium were used instead.
- wgpu on screen (tests only), D3D12 and Vulkan, print on Windows and Linux in round 2.
- Full clip renders; BIM, Grasshopper and Excel scenes were checked through static drivers of their own helpers. Published videos and posters were not regenerated.
- Rust groups that ran before the semantic ink change were not rerun after it (list in `rust-final-handoff.md`).

## Open

- React and wgpu keep docked windows between the bands; only the floor runs beneath them.
- A dragged Canvas dock tab is still extracted from the layout instead of keeping its room: the drop vectors address the extracted layout.
- wgpu tree drag ghost has no label, and its start threshold is 5 px (law 4 px); anchor places have no Rust twin.
- `LayeredOverview` shows the opened page blurred beneath its glass. That is the element's design and was left.
- About 20 hub, course, presentation and plugin test files still launch Chromium directly.
- The relocated occupants of the `need` clip need a look in a real render.
