# Procedural World Grid

## Fail-first boundary

Three native laws are registered before production changes:

- `an_actual_world_grid_is_one_camera_projected_procedural_scalar_without_grid_lines` in `semio-framework-os-infinite:test-wgpu-world-terrain` invokes the actual `render_world_3d` producer with a moved perspective camera and currently requires the finite grid LineList to be absent.
- `a_procedural_grid_is_one_prepared_scalar_between_textures_and_opaque_geometry` in `@semio-tech/ui-rs:test-wgpu-engine` requires one measured `PassGrid` cursor after textured references and before opaque geometry, retention of `PassLineVertex` for overlays, and a real GPU cursor arm.
- `procedural_grid_uniforms_are_dpr_invariant_while_viewport_and_scissor_scale_physically` in `@semio-tech/ui-rs:test-wgpu-engine` requires a dedicated grid encoder that reads the retained logical viewport and uses the existing surface-scale and physical-scissor boundary.

The latter two are temporary source-shape RED assertions because the production grid record/cursor does not exist yet. They must become behavioral cursor, usage, cancellation, uniform, viewport and scissor assertions after the RED receipt and before acceptance.

## Language-neutral browser authority

Added schema `framework.world3d.grid-visual/v1` and its fixture at the World3d domain boundary. It pins installed Three 182 and Drei 10.7.7, the production Grid props, both element-token colors, orthographic/perspective cameras, DPR 1/2, line, neighbor, cell, mid-fade and beyond-fade physical samples.

The existing permanent `@semio-tech/framework-renderer-react:scene-shading-pixel-check` target now has `grid-visual` and `grid-visual-wgpu` modes. `grid-visual` mounts the installed `@react-three/drei/core/Grid.js` component through actual React Three Fiber in Chromium. `grid-visual-wgpu` loads the exact `WORLD3D_GRID_SHADER` Rust constant, compiles it on actual browser WebGPU, supplies the declared two uniform groups and shared plane vertex layout, and compares its pixels with the recorded Drei rows. Both modes are registered in `.vscode/launch.json`.

### Receipts

- Installed Drei unrecorded capture: `grid-drei-red-1`, 20 browser pixel rows, Nx success in 2.9 seconds.
- Installed Drei recorded replay: `grid-drei-green-2`, 20/20 exact rows, Nx success in 2.4 seconds.
- Production WGSL fail-first: `grid-wgpu-red-1`, Nx reached the oracle and failed exactly because `WORLD3D_GRID_SHADER` does not exist. No browser WGSL program ran. This is the intended pre-production RED.

The recorded rows demonstrate derivative coverage, a transparent cell centre, a nonzero mid-fade sample, discard beyond the fade radius, both token inputs, a camera-moved perspective case and stable logical samples at DPR 1/2.

## Production implementation

The retained scene now publishes one `ProceduralGrid3d` per visible grid pass. World derives its plane at datum Z plus 0.001, projects from camera position rather than orbit target, retains the actual LOD cell step, calls the existing fade helper with the unshifted datum and logical viewport, and publishes the linear `text_element` token. The finite grid call was removed from the World paint route while every overlay line producer remains unchanged.

The prepared ladder inserts one `PassGrid` after textured references and before opaque material/mesh work. Its measured ownership is one item and exactly `size_of::<ProceduralGrid3d>()`; ordinary overlay vertices still traverse `PassLineVertex`. Both retained layer/attachment classifiers and the actual GPU match recognize the scalar.

`UiPipelines` owns one 256-byte `World3dGridUniforms` buffer, its persistent bind group, a triangle-list pipeline over the existing six-vertex plane, and a dedicated encoder. The shader and target-neutral contract are byte-identical. The pipeline alpha-blends into the world-encoded attachment, tests `LessEqual` without writing depth, uses content stencil, and has no culling. The encoder writes logical plane/camera/cell/fade/theme values and applies surface scale only to viewport and scissor.

The temporary source-presence laws were replaced with behavior. The cursor law walks an actual retained pass, observes exactly one grid scalar between a textured instance and opaque instance, checks its exact byte usage, and observes both overlay line vertices. The DPR law compares the actual 256-byte uniform value at scales one and two, then proves only viewport and scissor double.

The World producer law now checks the actual retained scalar's plane Z, camera projection, LOD step, fade result and theme RGB in addition to zero grid line vertices.

## Production browser receipt

The first shader execution compiled and recorded 20 pixels, but the orthographic cases were completely clipped and the perspective cases read neighboring rows. This was an oracle setup defect rather than a shader defect. Unlike the existing production-WGPU reference oracle, the new grid oracle had omitted `camera.coordinateSystem = THREE.WebGPUCoordinateSystem`, so its default WebGL NDC Z was invalid for WebGPU. It also indexed WebGPU's top-left copied rows with WebGL's bottom-left `readPixels` Y. Setting the actual WebGPU camera coordinate system and flipping copied row Y made the same production shader byte-exact against every installed-Drei row.

`grid-wgpu-green-7` ran:

```text
NX_DAEMON=false SEMIO_TEST_ARTIFACT_DIR=.../grid-wgpu-green-7 bun nx run @semio-tech/framework-renderer-react:scene-shading-pixel-check --skip-nx-cache -- grid-visual-wgpu
```

It passed 20/20 actual Chromium WebGPU samples against installed Drei in 2.2 seconds through Nx. Orthographic light/dark, DPR one/two, derivative neighbor, transparent cell, radial mid/beyond fade and moved perspective rows all matched RGBA8 exactly. No Cargo command was run by this agent; the native green compile/run remains root-owned.

The root-owned full UI census then passed 653/653 with zero skipped in 1.672 seconds (Nx 26.8 seconds), receipt `ui-grid-green-4/metadata/semio-nextest-D0DlLE`. It includes both procedural-grid prepared/DPR laws and the exact ten-pipeline encoded-attachment inventory, including `world3d_grid_pipeline`.

The focused World producer and actual reference-scene laws passed 2/2 with 479 outside the filter in 230 milliseconds (Nx 37.7 seconds), receipt `world-grid-green-2/metadata/semio-nextest-92TZmI`. Together with the UI and Chromium receipts, this closes the prepared cursor, uniform/DPR, producer, reference-scene and installed-Drei shader boundaries. A full main-scene browser run has not yet been recorded.
