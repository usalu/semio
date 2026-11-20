# Dev Shooting Drawing Controls

The shot inspector edits the camera projection, zoom, vertical field of view, eye position, target and up vector. It shows whether the shot uses a saved camera or the live camera. Editing an uncaptured shot creates a durable per-shot camera and subsequent edits address only the selected control. The shot’s Framing control follows the scene’s Center Model setting by default. Fit Model and Manual override it per shot. Edits to zoom, perspective field of view, eye or target switch only the edited shot to Manual. The preview and export use the same framing choice.

For an SVG shot the inspector exposes the background and face-fill layers, a fill color override and opacity, separate controls for visible and hidden edges, line colors and widths, merge, and corner easing in output pixels. The shot preview and export command share a render request. PNG retains its shaded render path. The inspector supports English and German. Text and number fields commit on Enter or blur so intermediate typing stays in the draft and does not trigger rendering.

The native dispatch regression drives the inspector's command payloads through a new shot, its captured camera, document edits and the exported request. The panel fixture checks both locales and every field's binding to the selected shot. The browser probe reads settings back from Save Download and inspects a styled SVG download. Execution results are tracked separately from these test definitions.

## Verification

See [the geometry report](projection-line-visibility.md) for clipping, visibility and independent Three.js comparisons. See [the drawing report](svg-drawing.md) for layer composition, easing, merge, preview invalidation and export cancellation. A headless Chromium run exercised the production icon render port with the repository base GLB. Sharp decoded the exported SVG at the requested size. The run checked that the two projections produce different pixels, hidden strokes and fill opacity reach SVG, and an edges-only export preserves both transparent and painted pixels. The full shooting workspace suite and the integrated dev-host launch are in progress.

The dev-route regression now verifies that boot receives the composed catalog’s row array. The older assertion expected the composition call itself and rejected the valid property access.

## Current acceptance

- The Chromium GLB check passed through the production React icon render port. The normal Shooting dev session still needs to exercise its Wasm editor, command dispatch and styled download in one run.
- The nine-file Shooting TypeScript run returned 30 passes and one failure. The catalog assertion was corrected after source inspection. The corrected suite passed all 42 tests across 10 files, with 6,344 assertions. It included the dev route and the camera operation text fixture. The owner’s test registration includes visibility and persistence fixtures. The SQLite run passed all 40 tests with 131 assertions, including the sparse mutation JSON schema and the shot drawing round trip against independent SQLite queries.
- The first full Rust workspace run reached an upstream PDF dependency and failed to resolve its cross-reference stream crate. The shared worktree had the manifest fix by the time the failure returned. Both Shooting locks now include the new dependency closure, and a fresh test is queued. Independent camera and drawing mutation vectors passed the concurrent-field and inverse checks. Native shot dispatch, mutation round trips and native SVG integration remain unverified by that run. Scoped Rust occlusion and line assembly checks passed independently.
- The declared dev command is live in repo’s graph queue. It activates a fresh build before starting the browser host. Memory and swap pressure are delaying admission. The ticket remains open. The bounded native SVG preview worker shares the export pipeline. The native pane now tracks requests, reports vector progress, cancels superseded requests and publishes rasters within the uploader’s byte budget. A focused Rust lifecycle harness passed. The normal native renderer build and live pane remain unverified.

### Live verification handles

- `82120` ended with the upstream PDF crate import error. `58982` is the fresh Shooting workspace test.
- `85349` is the declared React Shooting dev target. It replaces the serve-only command, which would have accepted an existing activation receipt.
- `13259` is the declared browser WGPU Shooting dev target. It waits behind the existing checkout graph load.
- `18622` exited successfully after 42 TypeScript tests.
- `27873` covers projected linework, icon-host SVG and PNG jobs, and native winit integration together. Its admission is still waiting for host memory.
- `20801` exited during native icon-host source admission. Its test closure had 4,081 active lines against a 4,000-line cap. Generic projected linework now lives in its own first-party renderer crate. The current repo scan measured 3,922 active test lines for the icon host, 576 for linework and 3,202 for winit. Full native verification still requires a retry.
- The focused Vitest run passed all eight tests covering SVG serialization, preview invalidation, request admission, asset transport and the Sharp raster oracle. The duplicate queued run `4206` was cancelled.
- `74948` exited successfully after the 40 SQLite tests. The Chromium drawing probe `94819` also exited successfully.

Browser preview and export check incoming render requests against the shared schema before handing them to the port. A direct validator probe accepted valid fill opacity and rejected out-of-range easing and invalid stroke color. The shooting operation-line grammar now describes the JSON emitted by its codec.

The browser probe now requires the first projection edit on the original demo shot to create and link a saved camera, with the other shot unchanged. Its script passed syntax validation. The scoped worktree diff passed the whitespace check.

The repository’s source closure scanner accepted all five audited Shooting owners in lib and test configurations. The largest audited closure was the editor test crate at 3,423 active lines. The parser audit script is retained under the ticket.

The browser probe now compares the live SVG preview and styled download by layer IDs, opacity, stroke, and path attributes. The standalone Rust projection checks passed 17 fixture cases after extraction.

The combined renderer probe covers the native upload budget with independent Sharp cases at 512×512, 1024×256, 724×362 and 362×724. The pure Rust extent helper also passed 294 admission-limit cases. The original-caller pane lifecycle is documented in [the native pane report](native-svg-pane.md).

Browser WGPU is a declared Shooting dev route. Its existing frame worker now shares the exact SVG pane lifecycle. The browser raster adapter now hands resized SVG to the established async bitmap decoder and page fallback. Three focused Vitest checks passed covering paged SVG custody, resize, fallback ordering and cancellation. A real Chromium worker also used the production fallback to return 512×512 straight-alpha RGBA from an 8,000×8,000 SVG inside the 1 MiB budget. The probe verified both painted and transparent pixels. A separate acceptance probe targets port 6119, edits the shot through the accessibility bridge, checks the resulting canvas pixels, and checks the saved document and fitted SVG download. Its script passed Node syntax validation. Live acceptance requires the fresh WGPU build.

The latest Shooting and bridge locks carry the PDF xref-stream dependency and their needed first-party packages. A separate offline Cargo metadata query timed out, so locked resolution still needs confirmation from the managed native test.

A static check confirmed both Shooting lockfiles contain one PDF xref-stream package entry, that graph projection depends on it, and that every named xref-stream dependency is present. A direct pinned Cargo metadata probe was refused by the repository compiler ingress. Locked compilation remains in the managed native test.

Per-shot Framing now persists as Fit Model, Manual or Follow Scene. The actual TypeScript and SQLite codecs round-tripped all three states with independent SQLite queries. The Python oracle passed all nine setting and undo pairs and an interleaved edit on the other shot. The Rust editor regression now checks that a manual zoom on one shot leaves sibling export requests fitted and that changing the perspective field of view while orthographic leaves framing alone. The browser probes also check the assigned framing value in the downloaded document. These application regressions await the full builds.

Camera undo now ignores a first-capture release if another event removed the private camera or relinked the shot. The Python oracle checked all 34 forward and inverse fixture pairs, including the stale release regression.

The workflow raster preview's hexadecimal color parser now rejects malformed UTF-8 colors before slicing the input. The native SVG conversion regression supplies malformed background and stroke values. Sharp independently rejected both values.

The combined renderer Cargo test was cancelled while its 3,584 MB request held the head of the capacity queue. The 793 MB Shooting native test then admitted one worker and reached Cargo planning. The canonical React and WGPU dev launches remain queued. The renderer suite still needs a fresh run.

The native retry resolved the previously missing PDF xref dependency and constructed the 913-unit Cargo graph. Source admission then refused the Shooting mutation owner when it reached 33 files against the 32-file limit. A complete scanner pass found 37 files in the original owner. The other 11 Shooting owners counted at most 31 files. The renderer owners scanned below both limits in native and Wasm configurations. The persistence source split is in progress.

The current React dev launch has entered the graph reader and is scanning the current file inventory. The separate WGPU launch is queued behind that operation because they share the repository fact store.

The persistence owner moved the camera and auto-fit mutation mounts into existing crates. The source scanner measured 31 files for both mutation crates involved and at most 30 in the mutation family owner, including Wasm and test configurations. The fresh managed Shooting test, session 10303, passed source admission for all 913 units and is waiting for a compiler slot.

The live fleet reported 12 machines and completed Rust unit offloads. The separate direct remote-execution command has no explicit endpoint configured, while Cargo can discover the fleet-published execution endpoint. The renderer test was relaunched through the supported offload mode as session 87798 and entered compiler-image admission. The native Shooting test and canonical browser launch sessions kept their existing positions.

The Shooting and renderer Cargo sessions both reported offload fan-out of ten jobs, then waited before starting any compiler unit. The launcher interprets the injected parallelism as an exact lease for ten local CPU slots. Nine slots were already held by other work. Both owned sessions were cancelled. The one-slot Shooting retry, session 63008, is pending the outer memory grant while the persistence agent fixes the offload path to preserve caller-selected or planned local capacity.

The Cargo offload fix now preserves an explicit local job limit or the normal plan. Both host-pressure regressions reproduced the original exact-ten request and passed with the change. All four Cargo scaling tests and the focused fleet readiness test passed. The one-slot local retry was cancelled while waiting for memory. Its replacement, session 35909, uses offload and one local slot. The live CLI printed the new admission diagnostic and advanced through build implementation loading. The renderer suite uses the same fixed route in session 94330.

The new offload launch admitted one of ten local cores, passed source admission for 914 current units and started rustc. The renderer retry is waiting for managed compiler-image headroom. The React graph reader has checked 46 owners and continues collecting source observations. WGPU waits for that fact store, and neither browser port is listening yet.

Top and bottom views now use the World3d up-vector rule in both icon camera implementations when the supplied up vector is parallel to the view. This prevents the SVG camera from rolling 90 degrees when a Z-up camera looks down the Z axis. The shared fixture raised a concrete regression in the React host, which projected a world-X point to horizontal screen coordinate zero instead of two pixels. After the change, the React camera and an independent Three camera with the expected up vector both passed the screen-coordinate fixture. All ten projection and line-visibility tests passed with 6,210 assertions. The same fixture is consumed by the pending native renderer suite.

## Fresh Verification After Resume

The previous native runs exited 101. Shooting stopped at the framework's missing controlled retirement implementation for admitted artifact genesis. Current framework source provides the implementation with bounds already required by the plugin trait. The renderer stopped at a private GLB schema intermediate crossing the decoder boundary. The GLB cursor now transfers its completed output into the materializer through a consuming method within its owner. The [native failure audit](native-failure-audit.md) and [GLB handoff report](renderer-glb-handoff.md) record the details.

All 84 tests across the 11 currently registered Shooting TypeScript entries passed, with 6,503 assertions. This run exercised the current package's exact entry list, including SQLite, the top-view projection oracle, and launch contribution resolution. All three live browser scripts passed syntax checks. The React acceptance probe now independently examines visible and hidden stroke widths and switches off all other layers before verifying a rendered hidden-lines-only SVG. Full live execution still requires the canonical hosts.

The fresh sessions are Shooting native 73915, renderer native 5237, React dev 42157 and browser WGPU dev 64538. Both old browser processes were absent before their replacements were started, with no listeners on 6019 and 6119.

The current renderer smoke suite passed all 65 browser WGPU preview and frame-transport checks. The current React suite passed eight SVG export, status and request checks. The GLB decoder owner is below the native source limits at 13 files and 3,397 active test lines.

Both fresh native runs are now compiling. React's active selected graph read advanced to 559 completed owner checks. The WGPU graph read still waits for React's fact-store request. There was no listener on either dev port on the last check, so the live editor, preview and export probes remain outstanding.

The React icon port now evicts only its own rejected cache entry and can settle an aborted consumer before the in-flight GLB request completes. Chromium reproduced both failures before the fixes. The production render-port probe then passed with one real connection reset, one successful retry, an independently aborted consumer, and reuse of the successful source by the other renders. Its real model also rendered a translucent SVG background and matched the independent Sharp RGBA oracle.

SVG background paints now preserve an authored eight-digit hexadecimal alpha in React and WGPU. Before the change the SVG fixture rasterized with alpha 255 instead of 128. Afterward all eight focused React tests passed, including the Sharp oracle tied to the shared native fixture. The native icon-host closures fit the source limits, at 3,954 active lines for the native tests and 3,743 for Wasm tests. The full native SVG assertion remains pending. All 84 current Shooting TypeScript tests passed again.

The WGPU audit found separate sticky transport failures for the SVG pane and the world asset. The existing icon token cancellation operates on its own token, with an existing sibling-cancellation test. Both WGPU recovery paths are under repair. [WGPU asset audit](wgpu-asset-audit.md)

The WGPU SVG pane now distinguishes transient transport errors from permanent decode/refusal errors and schedules at most four retries on the existing frame deadline owner. The same URL can be requested by independent previews without sharing cancellation tokens. Focused native and Wasm pane and asset probes passed. An independent RxJS virtual timer matched the five attempt times 0, 250, 750, 1,750 and 3,750 ms. The updated icon-host source boundary measured 4,000 active test lines natively and 3,789 for Wasm. The [SVG retry report](wgpu-svg-retry.md) includes the exhaustion and cancellation checks.

The world asset repair keeps its exact fetch owner across two bounded retries after network failures, selected temporary HTTP statuses or a short response. It retires partial response pages before retrying. Permanent transport statuses and decoder failures still end in the existing miss path. The 66-test browser frame and preview suite passed. The native and Wasm shared mailbox source closures fit their budgets. Full application and native renderer verification remains active.

The React application probe now also commits a manual zoom, checks exact camera persistence and sibling isolation, and restores Fit Model. Its syntax check passed. The browser launch runs still have closed ports and the two native suites are compiling.

In Chromium's Module Worker, the production world asset pump retained a real repository GLB request after a partial page and stream abort, retried an HTTP 503, then sealed the exact GLB bytes on the third attempt. A permanent HTTP 404 issued one request and a persistent 503 stopped after three. The probe retained a mailbox double for targeted stream and authority observations. [World asset retry report](wgpu-world-asset-retry.md)

## Cross-platform source admission audit

The current closures of 12 Shooting and eight renderer owners passed the repository source scanner under each of macOS ARM64, Windows x64, Linux x64 and Wasm target configurations. That covers 160 library and test closures against the 32-file, 4,000-active-line and 256 KiB limits. The fullest closure is the native icon-host test target with 30 files, 4,000 active lines and 219,070 bytes. This check covers source mounts and limits. Native executables and browser starts still need the managed build results.

The current tracked Shooting and renderer diff passed whitespace validation. Its longest checked path uses 161 UTF-16 units relative to the workspace. The [Cargo source freshness audit](cargo-source-freshness-audit.md) confirms that a build started before a concurrent edit cannot establish that edit’s compilation. Recheck any later-edited owner with fresh source admission before treating the resumed build as final verification.

Native Shooting session 73915 exited 101 after compiling the earlier failing framework and PDF graph dependencies. Its new blocker is semio-s-artifact-stdio-pdf-page-editor. The image preview imports a missing JPEG inference crate, and the TrueType scene renderer imports variants and a font-outline method missing from the package it names. Both owner issues are under repair before another native test launch. The current renderer and browser processes remain live.

The PDF page editor’s missing JPEG dependency is already resolved in the current worktree and all six applicable locks. Lockfile traversal identified both a direct Shooting-to-PDF edge and a path through Semio Drawing Export. Excluding the otherwise unused direct edge would still compile the same PDF package and page editor, so the native retry repairs their real integration. The TrueType renderer is being connected to the existing first-party PDF fonts crate that owns the outline types.

The PDF page editor now imports the outline rendering API from its actual first-party fonts owner. Its manifest and all six affected locks agree with that source. The source and whitespace audit passed. A fresh native Shooting workspace test is running as session 52393 after both PDF issues. [TrueType repair](pdf-truetype-dependency.md)

Renderer session 5237 exited 101 after the earlier GLB handoff failure was cleared. The new errors come from API changes in WGPU kernel-protocol turn results, microsecond deadlines, and the original caller source and failure routes. The renderer owner is updating the active callers and the adjacent scale, frame, and lifecycle code that used the earlier API. The canonical browser processes remain live.

The React preview frame now lets the completed SVG paint its own background. The regression reproduced duplicate translucency because the frame painted the same RGBA behind the image. The frame still shows the authored background while loading, then leaves it to the SVG. An explicitly disabled layer stays transparent in both states. The focused React run passed all eight tests after the fix.

The renderer retry, session 35799, uses the current kernel-protocol changes. They retain the real actor receipt, keep the original 100 ms budget expressed in microseconds, and retain caller-source loans in owner slots before settlement. The adjacent frame-actions and retained clipboard paths use the same ownership API. I also replaced the final stale one-argument source-loan call found in Writer’s typing fold so its live owner accepts the return directly. Full compilation of these changes is pending.

The kernel update passed Rust parsing across all 18 edited files, the scoped whitespace check and the relevant managed source limits. The native host-plumbing test closure was 259,436 bytes against the 262,144-byte limit. The [kernel API report](renderer-kernel-retained-api.md) records the original-caller and failure handling. Both fresh Cargo sessions are live in capacity admission, and the browser launch remains in graph preparation without a listening dev port.

The updated kernel, host-plumbing, runtime mailbox and worker owners fit the repository source limits in macOS, Windows, Linux and browser Wasm configurations. The source audit now also covers the component producer’s wasm32-wasip2 target. Its 38 checked closures include Shooting’s editor, mutation bridge and actual Hub component entry. Native compilation and fresh Wasm binaries remain separate verification.

The repo’s offload check reported memory pressure and a fleet path. Session 35799 was cancelled while queued and replaced with the supported one-local-slot offload retry 84059. Host pressure changed before the retry was admitted, and its real launcher remained in the normal memory queue. Shooting native 52393 and both canonical dev launch requests kept their existing positions.

## Live React mesh transport and browser readiness

The React icon renderer now keeps preview and export requests alive through a short GLB connection outage. The Chromium probe verified retry after a reset, HTTP 503 and truncated response, immediate cancellation for a single consumer, independent cancellation for a shared load, and bounded termination for persistent 503. HTTP 404 and invalid model data made one attempt. A glTF with a relative external buffer produced the same raster as the original GLB. All eight focused React tests passed with the retry change. [React GLB transport report](react-glb-retry.md)

The React export, React cancellation and browser WGPU probes now observe the app shell’s error signal as soon as it appears and verify that the ready signal identifies Shooting. All three pass syntax validation. Their production runtime runs await the canonical dev ports.
