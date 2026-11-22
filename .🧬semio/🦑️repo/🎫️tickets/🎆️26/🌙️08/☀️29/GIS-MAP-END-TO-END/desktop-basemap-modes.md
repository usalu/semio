# Desktop GIS2D Basemap Modes

The desktop harness now opens the GIS map window's **Window Options** and finds **Render Mode** in that pane. A selection has to reach the published control value before its rendered frame can count. The harness uses the existing surface deadline for publication, then waits for a stable canvas and completed tile requests. It checks successful vector and raster tile delivery and requires image and vector frames to differ, followed by a combined frame that differs from both.

The previous harness failed immediately because the measures rail starts folded. After opening the rail, it reached the Vector menu option but initially timed out on the five-second general interaction budget. A read-only Chrome trace observed one `setRenderMode` request to the GIS controller with `value: "vector"` and `windowId: "gis2d-main"`. The accepted value reached `vector` by the 5.5-second sample, accompanied by a new typed-operation acknowledgement sequence and UI patch. The control text had changed before the accepted measure value.

The first full rerun reached a mode screenshot after publication and lost the canvas during a Vite page navigation. The next runs freeze Vite update messages in their disposable browser contexts for a consistent test target.

With the default Colored style and Land enabled, Vector published after 5,054 ms and produced a settled frame. Image published after 1,976 ms and produced a different settled frame. Combined published after 1,440 ms, and its frame failed the comparison against both baselines within the surface budget. The browser loaded 48 distinct successful vector tile paths and one successful raster tile path.

The GIS policy explicitly enables a land wash at World, Continent, and Country bands. The renderer draws raster tiles first in Combined and can then cover the whole viewport with an opaque Colored-vector land wash. The policy tests assert that Combined enables raster and vector roles, while the default paint ordering can hide the imagery even with Raster enabled. The harness temporarily turns off Land to expose the underlay, verifies that the combined pixels differ from both baselines, and restores Land after the comparison.

## Verified Desktop Chrome Workflow

The full mode workflow passed on Desktop Chrome with WebGPU. Vector published in 2,487 ms, Image in 3,170 ms, and Combined in 2,175 ms. The combined canvas differed from both single-layer baselines, and 48 distinct vector tile paths and one raster tile path completed successfully. The same browser run passed mouse pan, wheel zoom, and keyboard pan with no console errors. Vite sent no skipped updates during the successful run. The script syntax and Git whitespace checks passed.
