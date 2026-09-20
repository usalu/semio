# Checkpoint 15 Build Boundary

Canonical WGPU activation15 and Native25 are running in separate isolated Cargo build directories. The renderer source includes pointer modifier snapshots, root fullscreen ownership, orthographic aspect correction, compact panel preflight, retained Select option routing, async reference codec/orientation/SVG handling, per-instance material scalars, painted and conic material paths, stable projected transparent ordering, and the document-retirement bitmap index.

Before this cohort, UI32 passed 622/622, generic shader/render passed 136/136, scene math passed 141/141, and World21 passed 233 selected tests with 243 filtered by its target. Actual pixel gates pass Standard 20/20, painted 3/3, conic 4/4, and ordered translucency 3/3. These are complementary gates and are not full application acceptance.

The previous checkpoint14 browser generators were repaired and both generators have passed in canonical activation15. The renderer WASM target passed and the five renderer artifacts were sealed in `checkpoint-15-renderer-artifacts.json`. The WASM is 127,293,339 bytes with SHA-256 `2bb05d8ba115c603762529af4d5222223d2308e7946ed7efecb0ce758c78d9ab`. All eleven selected source hashes remained unchanged. The final component and activation steps are still ongoing. Native25 reached 1,196 tests and reported three General layout integration failures: missing compact first-walk geometry, missing published Select trigger, and missing rendered General panel tab. SolWindow is diagnosing those failures. Native25 exited with code 1 after 13m 46s. Its captured log reports those three failures and no aggregate pass count; do not infer an all-suite pass. Activation15 remains ongoing and has no accepted final activation result.

Selected source hashes were recorded in `🗑️generated/astra-runtime/checkpoint-15-selected-source.json`; this is an eleven-file boundary snapshot, not a complete input manifest. Final WGPU artifact hashes will be recorded only after compilation completes and will be checked again after the browser journey. React activation8 must complete before the next paired acceptance run.

Full-resolution raster pooling and the reference visual/material/id/opacity packet are later work. Their current schema and oracle preparation must not be credited to this build. Their shared production wiring was released after the renderer WASM artifact boundary. Later source changes are not included in checkpoint15 artifacts.

Logs: `🗑️generated/astra-runtime/native-25.log` and `activation-15.log`. Goal and ticket remain active.

## Canonical WGPU activation result

Activation15 completed successfully in 17m15s: all 20 Nx tasks succeeded, with three cached. It prepared one completed component, browser support, session, and 17 fonts; the activation receipt reports the changed cohort activated. Canonical React activation8 is now running before any paired application measurement.

## Native follow-up boundaries

Native26 stopped before tests on three borrow errors in the newly registered raster pool. Native27 stopped before tests on the in-flight S3 World/draw tint API. Both compiler issues were repaired before Native28.

Native28 compiled the coherent pool and S3 API, then executed six General laws:five passed,one failed,1190 filtered. The compact first-layout and footer ownership laws pass. The remaining Select law proves an overlay-hit defect: the center of the painted Dark option is picked as the underlying Layout Select, producing zero setting actions. SolWindow owns the generic hit-precedence repair. UI33 is now running the complete UI-WGPU suite, including the new pool core laws.

UI35 passed632/632 tests with zero skips in1.952s (19.0s Nx), including the new raster pool core and S3 instance/shader layout laws. It precedes SolWindow’s latest overlay-priority/action-scope/captured-owner reconcile packet. That packet is source-coherent and Native29 is now testing all six General laws, including the interleaved option press/release regression. UI36 and World22 follow.
