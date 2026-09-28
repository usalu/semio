# Prepared Target Readback Contract

The neutral fixture covers unaligned two-row RGBA/BGRA bytes, partial and zero alpha, allocation limits, and a nontrivial sRGB view clear. Its schema and oracle are co-located under UI. The existing React Nx test target registers the new suite.

The first run passed schema and Sharp PNG roundtrip tests but could not find the browser binary in the repository cache. Rerunning with the installed user Playwright cache passed all three tests (696 ms test time; Vitest 1.58 seconds; Nx 2.4 seconds, exit 0). Chromium WebGPU returned exact expected row bytes and the expected sRGB color [137, 188, 225, 255]. Sharp independently encoded and decoded the same neutral RGBA bytes without losing alpha.

This proves the fixture and external oracle. Production Rust readback and export integration remain pending; these tests do not yet validate the first-party encoder or export delivery. Logs: `🗑️generated/astra-runtime/gate34/prepared-readback-oracle36b.log`.

## Production Source Checkpoint

The UI GPU context now supports an independent request-sized offscreen target on its existing device/queue. Prepared rendering skips surface acquisition only for that context; normal presentation retains its current path. A completed offscreen packet may begin async GPU mapping, normalize at most 4096 RGBA bytes per step, and publish the complete buffer once. Cancellation destroys the staging buffer and discards candidate bytes. Admission checks dimensions and device buffer/texture limits before allocation.

Three Rust laws are authored for dimension admission, bounded padding/channel normalization, and actual production composite-target readback with cancellation. They have not run yet. The export effect, first-party PNG encoder, SVG serializer, progress UI, and delivery are still outstanding.

The offscreen target clears transparently and records the exact completed `(scene_revision, preview_generation)` before readback. Starting a new packet or resizing invalidates that witness; beginning readback consumes it, preventing duplicate or unrelated cursor publication. Surface presentation keeps its existing opaque scene clear. Mapping failure cancels the candidate rather than returning to a perpetual Pending state.

The actual-GPU Rust law now also feeds accepted RGBA into the existing bounded first-party PngEncodeJob and decodes the PNG back to the neutral expected pixels/dimensions. Sharp supplies the independent expected PNG roundtrip. This extended native law remains unrun. Allocation limits now reference the pixels module’s exported constants directly.
