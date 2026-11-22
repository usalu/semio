# Puzzle Browser Verification Audit

The 3D product route is `repo run @semio-tech/puzzle-3d-rs:test-browser-lifecycle`. It runs the time-travel scenario in `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️3d-lifecycle/🟦️.ts` with the React renderer and English and German locales. Its default server is `http://127.0.0.1:6013/`. The scenario creates an object in an attached local archive, picks it, checks keyboard copy, relative paste, cut and undo, moves the object along a projected transform axis, checks a fresh peer, and verifies reload, undo and redo. This route still needs browser execution and a separate elapsed-latency measurement.

The registered Storybook route is `repo run repo-storybook:test-storybook`. Its 2D browser suite covers fixture loading, selection, zoom, delete, select all, lasso, brush placement, and keyboard focus. Assertions inspect the CPU session debug readout, not GPU pixels. The spatial suite checks real 2D and 3D fixture loading and 3D selection. Neither suite covers clipboard, dragging preview, relative placement, or latency.

## Baseline attempts

- The repo MCP `show_projects` and `run repo-storybook:test-storybook` calls did not return after more than a minute. A later dry run of the exact 3D lifecycle target timed out after 300 seconds. No server listened on ports 6010 or 6013.
- The CLI lifecycle command initially failed because agent-session and capacity directories under `/home/deck/.cache` are read-only. Redirecting those directories and the lease registry to this ticket allowed admission, but the graph service then failed creating its lock under read-only `/run/user/1000`.
- Redirecting `XDG_RUNTIME_DIR` into this ticket failed because the graph socket path is 175 bytes and the service permits at most 96. A short `/tmp` symlink to the ticket runtime got past that limit, but the graph service then died with `EPERM` while writing an owned auxiliary publication on descriptor 3. Redirecting the whole `XDG_CACHE_HOME` instead caused first-use Bun provisioning, which did not finish during the attempt. The existing Bun runtime is in `/home/deck/.cache/semio/runtimes/bun/`.
- Calling the OS development script directly through `repo` failed with `Original parent Script process invocation required`; it cannot be used as an unwrapped graph bypass. No browser smoke result was obtained.

The browser suites should be rerun in an environment with writable short runtime and cache directories and pre-provisioned tools. Add product-level assertions for clipboard, placement preview, drag-before-release, relative transforms, and a measured input-to-render latency budget; existing state-only checks do not establish those behaviors.

## Independent browser and artifact probe

The Puzzle composition has a generated `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle_bg.wasm` and its JavaScript bindings; the binary is 278 MiB. No other WASM binary was found under the Puzzle plugin and composition trees outside dependency and target directories. The root `node_modules` and pinned Chromium are present. No `storybook-static` directory is present at the repository root.

An independent minimal Playwright clipboard smoke is retained as `clipboard-browser-smoke.ts` and `clipboard-browser-smoke.html`. It uses real copy and paste keyboard chords, checks the browser clipboard and the destination input, and does not depend on Puzzle or WASM. Run it with `SEMIO_LANGUAGE=en repo tools exec --light bun -- <ticket>/clipboard-browser-smoke.ts`. It did not reach the test: Chromium launched and immediately exited with `SIGTRAP`, reporting `sandbox_host_linux.cc:41` and `shutdown: Operation not permitted`, even with `--no-sandbox`. A separate initial attempt to serve the page through Bun on loopback failed with `EPERM` on `listen`. These restrictions independently block a direct Vite or Storybook browser fallback in this environment. The existing `/tmp/semio` and `/tmp/semio-footprint` contents were inspected without modification.

## Fill and brush preview routes

2D's Storybook test exercises hover and Alt-leave commit, checking node and edge counts, but does not assert the visible ghost, its relative pose, or its disappearance. Native brush tests cover retained candidate stages, preview state and proximity, but are not a browser-to-renderer test. Verify both renderer paths with a real hover, preview pose, candidate cycle, commit and cancellation.

3D brush suggestions deliberately publish candidate previews through the tool-run trace, rather than the `brushPreviewJson` lane: the suggestion scene law explicitly asserts that lane is absent. WGPU consumes the trace in `infinite/world/🎛️host/🔄️sync-and-overlay/🦀️.rs` and makes translucent draws; its older brush ghost draw exists separately. The native suggestion tests cover a settled popup, hover index and acceptance. A full interaction test still needs to verify actual visible trace, candidate selection and accepted placement in both React and WGPU.

3D fill native tests verify provisional objects in the composite, one-edit finalization, undo/redo and cancellation. The Nakagin interactive ceiling test measures deterministic work units per turn, not elapsed input-to-frame latency. Run targeted non-browser tests through `repo tools exec --light cargo -- test --manifest-path ✏️s/🔌️plugins/🧩️puzzle/Cargo.toml -p semio-s-artifact-puzzle-3d-precompute-fill fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin` and similarly package `semio-s-artifact-puzzle-3d-editor` with filter `fill_run_finalize_publishes_one_edit_with_every_provisional_placement` when the toolchain and build capacity are available. These do not establish browser rendering or wall-clock responsiveness. No new concrete fill or brush bug was established by this read-only audit.

## Clipboard cross-check

React stores the fragment in `clipboardFragmentRef` and serializes copy, cut and paste through `ClipboardActionQueue`. WGPU stores it in `DocumentState.workspace_clipboard` and injects it into paste. The scope is the running shell. The 2D and 3D reserved jobs still compute the selected clipboard result and paste mutations in one step. Puzzle 3D now projects only selected objects and their internal attractions. Large-document latency and cancellation need a representative browser test. The framework prunes post-document-change topology selections, and the authored 3D browser scenario exercises cut and reselection after undo.

## WGPU browser input ordering

The following findings describe the original ordering failure. The transport and Rust queue now preserve discrete boundaries, with focused verification recorded in [WGPU input ordering](./wgpu-input-order.md).

The browser transport keeps only the latest move per pointer, separately from the FIFO lossless queue. `flush` sends every retained move with at most 16 lossless events; the native `BrowserRenderer::enqueue_batch` applies all replaceable events before all lossless events. Thus a single DOM burst `down → move → up` reaches the host as `move → down → up`. A lossless backlog also permits a newer move to overtake older discrete events across batches. The host `EventQueue` stamps the order it receives; `RuntimeDispatchCursor::take_next` merges its retained move and discrete page by those generations, so it cannot recover the lost DOM order. Even with native sorting alone, the transport already overwrote moves across discrete barriers; the downstream queue's own single move slot can likewise overwrite a move before an up with one after it.

This loses an intervening drag preview and can lose gesture state transitions that depend on movement. Some 2D release handlers recompute the final pose from the up coordinates, so final mutation failure is not established for every gesture. Test actual drag output and intermediate preview for both dimensions, including a burst in one flush, a move after release, multiple pointers, a backlog exceeding one lossless page, and overflow/retry.

Preserve a monotonic input sequence and coalesce each pointer only within a bounded segment between ordered barriers. Drain segments and lossless events in sequence, retaining newer samples while an earlier lossless page remains; do not order by timestamps. The downstream queue must preserve those segment boundaries too, for example by promoting each pending move into bounded ordered storage before admitting a barrier. Charge those retained samples to explicit capacity and preflight each batch atomically, retaining existing observable overflow/backpressure and stale-generation protections. Resize may remain independently replaceable only if its effect on coordinate interpretation is explicitly ordered or captured per pointer event.

## 3D browser parity hooks

WGPU's browser mount with `pageBindings` publishes `window.semioWgpuIntrospection.dumpMeshStats(windowId?)`. Its parsed `surfaces` rows carry `surfaceId`, `paneId`, page CSS `rect`, instance and interaction IDs, camera, selection, and hover. A keyboard clipboard test can use the real focused surface and compare instance IDs and selection before and after paste. The dump still omits projected gumball hit positions. `dumpChrome` supplies chrome hit rectangles, and `dumpAccessibility` and the DOM mirror expose accessible controls. The existing React 3D lifecycle relies on its own DOM attributes.

React's `data-gumball-hits` provides projected axis and origin targeting. The live gumball preview goes directly to Three.js instance roots, while `data-instances-json` continues to report the published scene. The React lifecycle now compares canvas pixels before and during a held drag, checks the published instances stay stable until release, and then checks the committed movement. The pixel check observes a change during the drag. The post-release position assertion checks the axis and direction.

The WGPU mesh dump now includes published instance transforms. The separate 3D WGPU clipboard case uses the real keyboard and tests relative placement, preserved orientation and scale, selection, and undo. It still needs browser execution. WGPU pre-release transform verification needs a live pose or an independent visual oracle. [WGPU 3D browser clipboard](./wgpu-puzzle3d-browser-clipboard.md)

A later fleet probe returned host capacity in under one second. The sealed-graph project query and exact WGPU browser dry run still returned no result before cancellation. Native work continued while those queries ran.

The standalone clipboard smoke also exercised the installed full Chromium binary in headless mode. Chrome stopped in Crashpad's socket setup with `setsockopt: Operation not permitted`. With its testing flag to disable Crashpad, it reached the same denied `shutdown` call as the headless shell. Both failures occurred before creating a browser context or loading a page.

The repeated exact WGPU browser dry run and fleet status query timed out at the MCP tool boundary after 300 seconds each. The separate remote execution status query returned successfully and reported remote execution unconfigured for this workspace. Native checks continued on the local machine.

The 19:49 CET single-process Chromium check used the pinned headless shell, disabled its zygote and supplied a ticket-local profile. It still stopped at `sandbox_host_linux.cc:41` with `shutdown: Operation not permitted` before loading the local fixture. The fresh remote execution status also returned `configured: false`.

The 21:27 CET remote execution status again returned `configured: false`. The locally cached Chromium distribution supplied SwiftShader for native Vulkan readback despite the browser process startup restriction. The native Puzzle 3D probe rendered the registered artifact and checked a held relocate preview and cancellation. [Native Puzzle scene ingress](./wgpu-puzzle3d-scene-ingress.md)
