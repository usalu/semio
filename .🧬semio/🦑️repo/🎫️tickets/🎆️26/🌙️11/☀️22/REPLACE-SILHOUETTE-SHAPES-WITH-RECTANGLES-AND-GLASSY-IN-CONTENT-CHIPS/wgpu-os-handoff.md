# wgpu-os handoff

## Model
A dock stack is one `WindowFrame` rectangle. Content is clipped by `push_scissor(frame)` and fills it (chip row included); chips are glass rects inside it, painted after the content; the border is the frame outline plus the chips' inner edges, resolved through the existing OS shared-border resolution; hit testing: chips first, everything else in the frame is content. `stack_body_rects` now returns the frame itself. The chip-row depth is `WindowFrame::safe_clearances()` / `clearance_rect()`.

## What changed
Pure geometry (new crate `semio-ui-window-frame`, registered in the framework workspace)
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowFrame/🦀️.rs`: `WindowFrame`, `WindowFrameEdge`, `WindowFrameChip`, `WindowFrameChipRegion`, `WindowFrameDock`, `WindowFrameSafeClearances`; `chip_regions()`, `safe_clearances()`, `clearance_rect()`, `contains()` (inclusive), `chip_at()`, `content_clip_rect()`, `border_path()`.
- `🔲️WindowFrame/🧪️tests/🦀️.rs`: runs every case of `frame.cases.json`, plus `kurbo` (dev-dependency oracle) for chip-inside-frame and no-overlap, and a no-cutout grid test.
- `🔲️WindowFrame/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`.

Dock (`OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock`)
- Folder `🔲️window-silhouette-and-stack-geometry` renamed `🔲️window-frame-and-stack-geometry`; `WindowSilhouette*` deleted; `stack_window_frame`, `push_window_frame_border`, `window_frame_border_edges`, `FrameBorderEdge` (border-paint.rs: outline plus chip inner edges); `dock_stack_body_rect` (horizontal body inset) removed; `stack_body_rects_with_frames`; `render_stack` uses `push_scissor`/`pop_scissor` then chip glass.
- Dock crate depends on `semio-ui-window-frame` (`🛰️dock/📦️packages/🦀️rust/Cargo.toml`) and re-exports the frame types.
- Tests folder `🪟️window-instances-geometry-and-silhouette` renamed `…-and-frame`; tests rewritten to the new laws (single rectangular scissor, chips inside, no cutouts, abutting frames without inset, border edge count, `body == frame`).

Shell and engine staging
- `window_silhouettes` -> `window_frames: HashMap<String, WindowFrame>` (dock component, input geometry, session, topology, pointer-input, fixture `🧬️shell-components/🔣️.json`); cursor override uses `frame.contains`.
- `panel_band`: bottom anchors no longer overhang the footer; the band ends at the body bottom with unchanged height (React `chromeHostedOpenPanelPositionStyle`, bottom chip row inside the canvas). Test `a_bottom_anchor_ends_on_the_canvas_bottom_chip_row_…` updated.
- Shell/engine tests renamed and rewritten (`window_frame_border_emits_…`, `frame_hit_intersections_cover_the_whole_frame_with_no_cap_gap`, region `FrameContent`).
- Scenes canvas2d: `end_silhouette_clip()` -> `end_clip()` (wgpu-ui rename).
- No `silhouette` identifier remains under OS outside the untouched meanings (infinite board LOD, world overlays, Scenes, glb, TiledMapHost).

## Pane clearance (Shell wiring)
- `pane_document_rect(window_rect, frame, edgeless)` (dock crate, `🔲️window-frame-and-stack-geometry`): document-like panes (retained UI documents: text, forms) are laid out in `frame.clearance_rect()` (top and bottom chip-row depth as inset) while the pick clip and scissor stay the whole frame, so scrolled content slides beneath the glass chips; panes with an engine host (`SurfaceState::engine_host_id_for_window`: world3d, node graph, tiled map, board2d) and panes without a frame are full-bleed.
- Wired in `📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window` phase 4 (`render_ui_document_step` gets `document_rect`); import added in `🪀️shell-view-panels/🦀️.rs`.
- Test: `document_panes_start_below_the_chip_rows_and_edgeless_panes_stay_full_bleed` (wgpu-unit, `…-and-frame`).

## Verified (workspace root `/Users/ueli/Documents/semio`, script `cargo-os.sh <log> <cargo args>` runs `repo cargo --store=shared --configuration=prototype -- <args>`)
- `test -p semio-ui-window-frame` (from `🧰️framework`): 4 passed, 0 failed.
- `test -p semio-renderer-wgpu-dock`: compiles; 2 passed, 0 failed.
- `test -p semio-renderer-wgpu-dock-laws` (mounts every rewritten Dock `wgpu-unit` test, app-mode layouts, window-template drag, stack-drop): 76 passed, 0 failed, including the new frame, border, scissor, abutting-frame and clearance tests.
- NOT run: Shell and engine shell-tests crates (`shell-dock`, `shell-input`, `shell-view-panels`, `shell-view-chrome`, `shell-state`, `shell-session`, `shell-tests-chrome-2/-4`, `shell-tests-input-1`). Command `check --keep-going --tests -p <those>` (run twice, ~20 min apart) cannot reach them: both runs stop on crates outside this lane that are unmodified or owned by other lanes, printing exactly:
  - `error: could not compile semio-renderer-media-native (lib) due to 4 previous errors` (`🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/🖼️presenter/🦀️.rs`: three `E0308 expected f32, found integer` at `row(0)`, `row(1)`, `row(2)` (closure `|index: f32|` at line 764), and `E0506 cannot assign to self.closed_stats because it is borrowed` at line 192). File is identical to HEAD.
  - `error: could not compile semio-renderer-wgpu-shell-actions (lib) due to 7 previous errors`: seven `E0277 RefCell<Vec<Effect>> cannot be shared between threads safely` at `🐚️Shell/…/📥️staging/🔀️actions/📨️command-dispatch/📇️directory-and-hub/🦀️.rs:87` and `🌉️ProgramBridge/…/🧩️extension-splice/🦀️.rs:216,63`. The crate's `Cargo.toml` (adds `semio-ui-retained-tree`) and `…/🎬️shell-actions/🎯️dispatch-action/🌐️open.rs` carry uncommitted edits from another lane.
  - Crates that did compile in those runs and contain my renames: `semio-renderer-wgpu-dock` and the shell vocab, chrome-state, surfaces, history, plugins, prefs, directory, sync and async crates.
- Tokens: `ui_styling` generated tokens were stale and broke `semio-ui-theme`; regenerated locally (gitignored files only).

## Open
- Re-run once the two blockers above are fixed (the `row(0.0)` literals and `closed_stats` borrow in `MediaTransportHost`, the `RefCell`/`Sync` errors in shell-actions): `./cargo-os.sh shell check --tests -p <the nine shell crates>` then `test -p` the same crates; fix any rename or expectation drift in Shell tests (`window_frames`, `stack_body_rects_with_frames`, `push_window_frame_border`, bottom panel band test, `FrameContent` region marker).
- The Dock border resolves through the OS `shared_borders` module rather than `DrawList::push_shared_border` (same result).

## Shell verification
`check --tests --keep-going` is green for all nine Shell crates (`shell-dock`, `-input`, `-view-panels`, `-view-chrome`, `-state`, `-session`, `-tests-chrome-2`, `-tests-chrome-4`, `-tests-input-1`). `test --no-fail-fast` ran with `repo cargo --store=shared --configuration=prototype` (via `cargo-os.sh`).

### Blockers fixed (none came from the frame migration)
- `MediaTransportHost/…/🖼️presenter/🦀️.rs`: closure `row` took `index: f32` but callers pass the integer row numbers 0, 1, 2; the parameter is now `u8` and widens with `f32::from`.
- `MediaTransportHost/…/🍎️audio-queue/🦀️.rs` (`close`): the stats are copied into a local before `self.closed_stats` is assigned, which ends the E0506 borrow of `self.context()`.
- `ProgramBridge/…/🧩️extension-splice/🦀️.rs` (`splice_native_extensions`): the effect sink captured by the `async move` closure was a `RefCell`, so the future (and every `shell_turn(…: Send)` turn above it) was not `Send`; it is a `std::sync::Mutex` now. The wasm path keeps `RefCell`, where turns are not `Send`.
- `shell-pointer/🦀️.rs`: the `PendingExtensionInvocation` import was gated to wasm32 while `run_extension_invocation` uses it natively; the gate is removed.
- `host-winit/📦️packages/🦀️rust/Cargo.toml`: the `testkit` fixture compiled into the lib uses `semio_ui_contract_values::testkit`; added as optional dependency enabled by the `testkit` feature.
- `🧪️tests/🎨️wgpu-theme-editor-and-accessibility` (`passive-regions-and-geometry`, `accessible-names-and-activation`, `mirror-address-liveness`): `chrome_accessibility_nodes` takes `&mut self`; the tests now bind `let mut shell` / `&mut ShellState`.
- `🧪️tests/🔬️wgpu-shell-input/🔎️quick-search-palette-keyboard`: `palette_chord()` hard-coded `ctrl`, but the accelerator is `meta` on macOS (`host_platform_uses_meta`); now platform-correct. Both palette tests pass.

### Test results (`test --no-fail-fast`)
| crate | passed | failed |
| --- | --- | --- |
| shell-dock | 0 | 0 |
| shell-input | 5 | 0 |
| shell-session, shell-state, shell-view-panels | 0 | 0 |
| shell-view-chrome | 6 | 0 |
| shell-tests-chrome-2 | 10 | 0 |
| shell-tests-chrome-4 | 98 | 4 |
| shell-tests-input-1 | 24 | 49 |

No remaining failure involves frames, chips, `window_frames`, the frame scissor, hit testing or the clearance rect.

### Left, caused by other lanes' unfinished work
- chrome-4, three tests (`accessibility_activation_updates_settings_pressed_button_projection`, `every_open_theme_section_publishes_within_the_retained_document_ceiling`, `theme_host_mutations_republish_the_mounted_retained_leaf_without_guest_refresh`): `UI value arena requires its actual registered Source … no accepting lifetime registration` (`collection-arena`), from the in-flight UI value arena and retirement rework.
- chrome-4, `a_discarded_chrome_walk_cannot_exhaust_successor_accessible_names`: asserts the chrome control names are empty after one `render_chrome_step` turn; the uncommitted incremental clearing in `chrome-frame-and-main-window` (setup steps 20 to 23, one `pop_first` per step) now needs several turns.
- input-1, 46 tests: `ui-doc begin refused … fault=StaleGeneration`, then `the component/tree pointer document faulted in phase fault`, in the retained UI document ingress; 1 test: `ordered-map root must be explicitly retired before drop` (`value/ordered`); the rest are the same class. All in the value ownership and retirement edits (`value/retirement`, `value-native-owned`, `store/initialization/replay` were being edited during the run).
- Transient others' breakages that cleared by themselves: `semio-framework-value-native-owned` (`statements` field), `semio-plugin` (`cold_transient_preparation_factory`), `store-initialization-replay` (`Turn::Yield`), and the source boundary guard on `semio-plugin-decl-app` (262783 of 262144 bytes).

### Files changed in this pass
`MediaTransportHost/…/🖼️presenter/🦀️.rs`, `…/🍎️audio-queue/🦀️.rs`, `ProgramBridge/…/🧩️extension-splice/🦀️.rs`, `shell-pointer/🦀️.rs`, `host-winit/📦️packages/🦀️rust/Cargo.toml`, the three `wgpu-theme-editor-and-accessibility` test files above, `quick-search-palette-keyboard/🦀️.rs`.
