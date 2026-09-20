# Terra Frame-Performance Audit

## Scope and conclusion

This is a static, read-only audit. It did not start Cargo, Bun/Nx, a browser, or an additional worker. It therefore does **not** attribute the measured multi-second worker steps to one source.

Checkpoint11 recorded a 2.2716 s worker tick with a nested 2.271199 s `transactionRouteIntents` observation. Checkpoint12 independently recorded a 2.4135 s maximum for that transaction stage and a 2.4156 s worker-step overrun before the later retained-tree paint crash. Those are browser-worker CPU wall scopes. They are not GPU timings, and the nested scopes cannot be added. The later reversed-paint overflow is a separate failure and cannot explain the preceding maximum without a profile tied to its timestamp.

The current `TransactionRouteIntents` maximum is not an input-routing attribution. `AppFrameTransaction::step` starts the stage timer before it pumps assets, and its `Build` branch invokes all of `frame_before_input_step`; that includes theme work, retained chrome/document work, atlas work, and resource transfer.

The strongest source-proven blocking candidates are the ready-asset appliers, specifically full raster/SVG decode after all response pages are concatenated. They run inside the first asset-pump call, before the decode-slice deadline loop, and can hold the asset-probe mutex (and, for world images, the runtime mutex) for the entire decode. Source proves that these calls are data-dependent, synchronous, and not bounded by the transaction deadline. It does not prove that either recorded maximum was an asset decode.

## Measured scope versus actual call path

`AppFrameTransaction::step` constructs its timer at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12300`, before all of the following:

1. The unconditional first `runtime.pump_renderer_asset_decode_step()` at `:12326`.
2. The clock-limited *subsequent* pump loop at `:12334-12335`.
3. The build call `app.frame_before_input_step(...)` at `:12415`.

The phase enters `Build` after brush-mesh routing (`:12387-12415`) and keeps `self.stage` as `RouteIntents`. Consequently a `TransactionRouteIntents` observation includes the first asset unit and every repeated `FrameBuildCursor` opportunity. `frame_before_input_step` reaches the retained chrome walk at `:14654-14663`; that walk invokes `render_chrome_step` (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:21795-22096`).

The deadline only guards the while condition after the first pump. A ready response therefore may enter a complete applier before either `context.deadline_exceeded()` or the asset-slice deadline is checked (`🧊️renderer/🦀️.rs:12326-12335`).

## Strongest synchronous asset candidates

### Whole-response collection plus reference-image decode

For a ready reference image, `pump_renderer_asset_decode_step` concatenates the response at `🧊️renderer/🦀️.rs:10847-10857`, locks the runtime at `:10858-10860`, then calls `apply_reference_image_bytes` at `:10865-10868`. `collect_world3d_asset_bytes` allocates to `received_bytes` and copies every sealed page in a synchronous loop (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:14933-14941`). The response pool permits 1,024 16 KiB pages, or 16 MiB total (`:14363-14365`). That is a fixed byte cap, not a per-worker-turn CPU cap.

The reference applier decodes the compressed image in one `ImageReader::decode` call and only then scales and converts it to RGBA (`🌍️world/🦀️.rs:15318-15324`). The texture-output budget limits the result after decode (`:15291-15310`); it does not limit decode CPU, decoded dimensions, or peak decode allocation. This is the most direct data-dependent blocking path in the observed scope.

### Whole-response collection plus UI raster/SVG decode

The shared ready path concatenates every response page in `take_shared_asset_bytes` (`🧊️renderer/🦀️.rs:10778-10786`) and directly calls `interpreter::apply_ui_image_bytes` (`:10805-10812`). `apply_ui_image_bytes` creates synchronous dimensions and decode closures (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1838-1850`). The raster queue executes `dimensions()` at `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5029-5040` and `decode()` at `:5057-5068` on that same call stack.

For raster input, `decode_raster_bytes` calls `image::load_from_memory` and `to_rgba8` synchronously (`🗣️Interpreter/🦀️.rs:1895-1899`). The decoded-pixel-size check occurs only after that decode (`🎞️Scenes/🦀️.rs:5069-5070`); the 2 MiB source-input guard (`:454`, `:4998-5001`) does not bound a highly compressed image's decoded size or decode time.

For SVG input, the same ready-applier path invokes `fontdb_mut().load_system_fonts()` in `svg_dimensions` (`🗣️Interpreter/🦀️.rs:1928-1936`) and again in `rasterize_svg_to_rgba` (`:1912-1925`). Thus a successful SVG runs two system-font discovery passes plus parse and rasterization inside one asset-pump call. Whether that is expensive in the actual browser-worker build is environment-dependent and unmeasured, but the duplicate synchronous traversal is concrete.

### Terrain decode is similar, but has weaker evidence for this run

Ready terrain takes the same whole-response collection path (`🧊️renderer/🦀️.rs:10829-10843`) and invokes `TerrainSession::upload_elevation_tile`. That method decodes the entire PNG synchronously before caching it (`🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs:368-377`; decoder `:155-160`). It is another possible long unit, but neither checkpoint identifies a terrain response at the recorded maxima.

### GLB decode is cursorized; publication itself is not the leading candidate

The probe reads one response page per `RendererAssetProbe::step` opportunity (`🧊️renderer/🦀️.rs:2678-2703`) and advances schema/materialization through cursor steps (`:2732-2759`). Its ready hand-off is a lease publication at `:10871-10903`; the world function only checks a revision, inserts the lease, and clears a pending URL (`🌍️world/🦀️.rs:14969-14975`). This does not rule out an expensive individual cursor unit, but it is less directly suspect than the all-pages image paths.

## Retained document, text, theme, and chrome work

The source does not support treating a whole retained document or text edit as a single unbounded frame call:

- Pre-input text advances exactly one text-buffer unit, then projects at most 4,096 bytes (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:548-570`), reached once from `drive_text_operation` (`🧊️renderer/🦀️.rs:14988-14998`).
- A document call makes a 2 ms / one-fuel `StepContext` (`🗣️Interpreter/🦀️.rs:1634-1658`). Reconciliation consumes fuel and exits when the context should yield (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1007-1025`); mounted layout gets a one-fuel 1 ms context (`🗣️Interpreter/🦀️.rs:1532-1548`).
- The production paint route progresses one retained walk, node, or scene step per `frame_into_step` call (`⚙️engine/🦀️.rs:1699-1778`; `:1803-1851`). A text node advances one glyph at a time, with a per-node byte ceiling (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:128-173`, `:843-868`).
- The shell returns to the worker after an incomplete document opportunity (`🧱️elements/🐚️Shell/🦀️.rs:22757-22763`). Its large `SHELL_WINDOW_PAINT_OPPORTUNITIES` value is a total retry ceiling, not a loop inside this call (`:2674-2677`).

Those controls make the document/text route a weaker explanation for a single 2.4 s call. They do not establish a wall-time bound for any lower-level unit that fails to observe the supplied budget, such as a glyph-cache miss, an individual widget paint, or a layout job hand-off. Actual-worker profiling is still required to clear it.

There is one separately synchronous chrome candidate. `plan_dock_windows` clones the complete dock view, generates labels, and rebuilds vectors/maps for bodies, silhouettes, tab bars, and the visible window plan in one `render_main_window_step` phase (`🧱️elements/🐚️Shell/🦀️.rs:22658-22686`; call at `:22703-22706`). The dock implementation explicitly clones in `render_view` (`🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:426-429`) and creates fresh collections in its layout helpers (`:367-369`, `:397-400`). No `StepContext` bounds that phase. It is a data-size-dependent secondary candidate; this audit found no evidence that the checkpoint's 40-node General document or normal dock size makes it a multi-second culprit.

Theme resolution is also a synchronous build phase (`🧊️renderer/🦀️.rs:14535-14541`), but document-backed themes are cached by id, appearance, and generation (`🧱️elements/🐚️Shell/🦀️.rs:28008-28026`). A cache miss can clone/parse the custom document and derive the theme; a cache hit still builds a `theme_id.to_string()` cache key (`:28012-28024`). The latter is a hot allocation worth avoiding eventually, but there is no source evidence that it explains seconds of latency.

## Instrumentation verdict and minimal attribution run

The current taxonomy cannot distinguish the candidates. `ShellRefresh` times `ShellState::refresh_ui` (`🧱️elements/🐚️Shell/🦀️.rs:6803-6809`) and `RetainedExchange` times the async `ProgramBridge::render_with_document` span (`🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:714-719`); neither wraps `frame_before_input_step` or a ready asset applier. Reusing either would produce a false label.

The existing latency mechanism can support narrow production attribution without hot allocations:

- `FrameLatencyTimer` is a four-field stack value (`⏱️frame-latency/🦀️.rs:323-343`) and, when diagnostics are disabled, does not read the clock (`:330-337`).
- On completion it uses `try_lock` and counts a refusal instead of blocking (`:345-355`).
- The hot registry is fixed-size: 256 recent summaries plus fixed arrays (`:3-7`, `:201-260`). The snapshot's `Vec`s are created only when diagnostics are requested (`:262-289`).

Add these diagnostic-only stage labels and local timers, all with the existing renderer-frame authority and no string labels:

1. `FrameAssetPump` around the first `pump_renderer_asset_decode_step` call.
2. `FrameAssetReadyBytes` around `collect_world3d_asset_bytes` / `take_shared_asset_bytes`.
3. `FrameAssetReferenceDecode`, `FrameAssetUiImageDecode`, and `FrameAssetTerrainDecode` around their respective appliers.
4. `FrameBuildTheme`, `FrameBuildText`, `FrameBuildChrome`, and `FrameBuildDockPlan` around the named leaf calls; `FrameBuildDocument` should wrap `render_ui_document_step` only.

These labels are nested and must be read as competing maxima, never summed. Extending the enum increases the fixed diagnostic aggregate arrays by a small constant, but keeps `FRAME_LATENCY_CAPACITY`, all frame fuel/deadline settings, asset-page limits, and GPU/resource budgets unchanged. If no increase in *any* static diagnostic bytes is permitted, distinct labels are impossible with the current 27-stage enum; a real worker CPU sample is then the only attribution method.

The minimum runtime experiment is one fresh actual browser-worker journey that reproduces a long worker step, followed by its frame-latency snapshot and a browser-worker CPU sample covering the same timestamp. First instrument only `FrameAssetPump` and `FrameBuildChrome`; they partition the current broad scope without changing behavior. If the former wins, enable the three ready-applier timers; if the latter wins, enable `FrameBuildDockPlan` and `FrameBuildDocument`. Preserve generation, stage maximum, work-item count, and console asset begin/done lines in the receipt. Do not claim the observed 2.27 s or 2.41 s duration belongs to any leaf until that same run contains its leaf observation or CPU stack.

## Recommended order

1. Run the root-owned fresh browser activation after the upstream paint crash is repaired, with the two partition timers or an actual Worker profile.
2. If `FrameAssetPump` is dominant, prioritize reference/UI/terrain decode. Move concatenation/decode out of the first frame transaction unit or make each step cursorized; apply pixel/dimension limits before full decode. For SVG, cache or avoid system-font discovery and avoid performing it once for dimensions and once for rasterization.
3. If `FrameBuildChrome` is dominant, split first at dock planning versus retained document paint. Preserve the existing one-step document/text cursor behavior; do not raise frame fuel or wall deadlines as an acceptance response.
