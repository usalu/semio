# React Tile Refresh

The React map starts rendering as soon as its session attaches. Each raster and vector visibility snapshot owns one cancellable request group. Visible requests run first. A newer snapshot cancels the old requests and starts immediately after the existing viewport debounce without waiting for obsolete prefetch. Late completions cannot update the browser caches or upload into the session.

Visible tiles retry network failures, request timeouts reported by the server, rate limits and server errors after 250 ms and then 1,000 ms. The retry bypasses the browser's cached response. A successful upload requests a frame, including when the camera is idle. HTTP 404 and 410 remain bounded misses for the current visibility revision. Disposal cancels the active requests and the retry wait before freeing the Wasm session.

Each raster or vector lane starts the next tile in request order as soon as one of its 12 slots opens. A stalled tile leaves the other slots available. Requests expire after 12 seconds, including any wait for a response body. A visible tile can then retry using the same backoff. Replacing the viewport or disposing the renderer cancels pending request deadlines.

## Verification

The language-neutral fixture specifies tile coordinates, HTTP response sequences and expected uploads. Vitest checks viewport changes during pending visible and prefetch requests, ignores completions after cancellation and verifies disposal during a retry wait. Each HTTP sequence also runs through RxJS retry for an independent attempt count and outcome.

Seven new regression cases failed before implementation. The final React map component suite passed all 33 tests using the checked-in React renderer long Vitest configuration through `repo tools exec`. The project-level `repo run` was cancelled after it waited over four minutes behind a shared graph inventory scan.

The package-wide TypeScript check exited with diagnostics in other renderer and framework modules, including diagram attributes, WGPU frame transport and NodeGraph fixtures.

Before the scheduling change, the newly added cases reproduced both the blocked request slot and the missing deadline for stalled response headers and bodies. The language-neutral fixture and RxJS concurrency and timeout operators provide the reference outcomes. All 49 React TiledMapHost tests passed. The focused TypeScript check reported zero diagnostics in the renderer and tile tests.
