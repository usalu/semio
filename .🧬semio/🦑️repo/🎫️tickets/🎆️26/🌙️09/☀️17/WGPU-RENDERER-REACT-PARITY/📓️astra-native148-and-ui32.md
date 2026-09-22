# Native 148 and UI 32

Both canonical Bun/Nx full suites completed against the coherent Rust Tree row-extent contract. Native 148 ran 1,370 tests in 34.521 seconds: 1,369 passed and one failed. UI 32 ran all 672 tests in 2.879 seconds and passed, with no skips.

The remaining native failure is `dock_stack_content_fills_full_bounds_through_one_silhouette_clip`. Its diagnostic census finds five layers, all without a clip. Only layer zero contains the full `[10, 20, 600, 400]` rectangle. The implementation owner is inspecting why the test has no admitted silhouette clip; the expected clipped content behavior is unchanged.

WASM 21 failed after 16 minutes because the renderer compiled against UI/contract artifacts produced before the required `row_extent` field was added. WASM 22 restarts the canonical producer from the now coherent source. No browser acceptance is inferred from the native tests or the failed browser build.

Native 148 still reports the same census as Native 147. The rapid popup-dismiss/close regression's source registration is being checked before claiming that new law passed.

The exact Native 149 filter confirms the rapid popup-dismiss/close law is registered: one test passed in 0.033 seconds, with 1,369 unrelated tests skipped. WASM 22 also completed successfully in 5 minutes 30 seconds. Activation and physical browser validation follow; they are separate acceptance gates.
