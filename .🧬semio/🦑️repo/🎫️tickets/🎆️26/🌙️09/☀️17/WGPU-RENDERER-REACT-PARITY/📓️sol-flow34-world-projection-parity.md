# World Projection Parity

## Scope

This packet replaces the WGPU World's collapsed camera-family/orientation flags with the existing renderer-neutral `Viewport3dProjectionSpec`. It carries that complete accepted spec through Shell template selection, delivered camera snapshots, World retained state, scene publication, culling, picking, marquee projection, and the WGPU presentation pass.

## Neutral authority

- The existing viewport schema remains the sole domain model. Rust and TypeScript parsers now enforce the cross-field axonometric Corner invariant and validate the retained axonometric preference bank even while another projection kind is active.
- The existing neutral projection fixture now covers Cabinet, Cavalier, Military, two-point lens shift, Isometric and Trimetric Corner orientation, curvilinear Fisheye/Panini pointer mapping, and invalid effective axonometric angles.
- Rust Pack/scene laws consume that fixture. The React oracle consumes the same records through `worldProjectionGoalMatrix`, `worldProjectionOrientationLook`, and `worldCurvilinearUnproject` backed by Three.js.

## Runtime implementation

- Shell's static projection branches and leaves resolve to full typed specs. Dynamic `world-projection:` rows decode with the shared schema instead of a second manual parser.
- World retains the full selected spec across incoming camera snapshots. A selected default ThreePoint spec still acquires local selection authority, and a delivered projection spec owns its FOV over the separate legacy camera FOV field.
- One projection matrix path drives scene publication, frustum work, point projection, marquee selection, and pointer rays. Oblique shear and two-point vertical shift use the same matrix elements as React/Three.
- Curvilinear presentation is a real image-space pass. The prepared command follows the completed scene content, copies the accepted non-sRGB composite into a scratch texture, and remaps only the accepted viewport back into the composite with React's FOV cap, strength, aspect, black exterior, and device-scale clip.
- Pointer rays use the shader's visible-to-capture mapping. Screen-space points use the inverse capture-to-visible mapping, so retained vertex/edge picks, tutorial targets, marquees, and generic screen selection address the pixels the postpass presents.

## Evidence

- The shared matrix/orientation/curvilinear fixture is generated from and checked against the actual React/Three production functions.
- The WGPU prepared-command law requires the curvilinear scalar after all scene content and before later layer work.
- Read-only GPU review confirmed the copy-before-write ownership, world-encoded format, command order, viewport and physical scissor, and shader equation match the current React pass.
- WASM20 reached the renderer and stopped only because the renderer package had not yet declared its direct viewport dependency. The dependency declaration was added by the root owner. No postpass compiler diagnostic was emitted in that captured build.

## Runtime acceptance checklist

| Runtime boundary | Required authority | Current evidence | Remaining receipt |
| --- | --- | --- | --- |
| Neutral admission | One `Viewport3dProjectionSpec` schema in Rust and TypeScript; no Shell-local descriptor | The shared fixture covers every mode, inactive-bank retention, strict ranges, nonfinite values, and effective Corner-angle rejection. The registered React suite calls `testViewport3dProjectionValues`. | Complete: the root focused run passed 5/5 tests across projection-render-parity and native-accessibility on 2026-09-27. |
| React/Three oracle | Production `worldProjectionGoalMatrix`, `worldProjectionOrientationLook`, and `worldCurvilinearUnproject` | The registered engine suite reads the neutral fixture and compares exact matrix elements, direction/up vectors, and both Fisheye/Panini capture coordinates. | Complete in the same 5/5 receipt. |
| Shell selection | Branches resolve to React's effective leaves; dynamic rows strictly decode `world-projection:` JSON | `WORLD_PROJECTION_TEMPLATES` carries typed modes, including Axonometric 30°/30°, Oblique Cavalier, Military, TwoPoint shift, and Curvilinear parameters. | Fresh native9/WASM21 compilation is root-owned. |
| Per-window retention | A local selection owns the complete spec and survives delivered camera snapshots/remounts | `projection_selected` is acquired even for the default ThreePoint spec; selected state recomposes family and FOV while preserving shear, shift, mapping, and orientation. Delivered typed projection FOV wins over the separate camera FOV field. | Native World/Shell unit laws are in the root gate; physical remount retention remains part of the paired runtime acceptance. |
| Paint matrix | The full spec drives the one accepted `ScenePass3d.view_proj` | Oblique shear, Military orientation, two-point shift, authored FOV, and Corner look all route through `projection_spec_view_proj`. Scene culling uses that same matrix. | Rust exact fixture law is running through the scoped `@semio-tech/ui-rs:test-wgpu-engine` target. |
| Curvilinear image pass | Remap the completed accepted viewport, never a candidate scene or the full composite indiscriminately | `PassCurvilinear` follows textured/grid/opaque/line/translucent/material content. The encoder copies the accepted composite before sampling, sets the logical viewport in physical pixels, applies the physical scissor, writes the world-encoded view, and blacks samples outside the capture. Because the render pipeline viewport maps the fullscreen quad into the pane, its interpolated UV is pane-local even when the pane has a nonzero composite origin. | UI10 reached the source guard and exposed only its stale 10-pipeline count; the guard now requires the 11th `world_curvilinear_pipeline`. A fresh UI gate receipt is pending. |
| Curvilinear pointer inverse | A visible pixel unprojects through the exact postpass equation before ray construction | Neutral Fisheye and Panini rows carry visible NDC and expected React capture NDC. Rust verifies visible→capture and an eight-step capture→visible inverse. `projection_spec_ray_from_screen` applies visible→capture before the inverse view-projection. | Mathematical parity is covered; a physical WGPU click on a visibly warped off-centre target remains a runtime receipt, not a source claim. |
| Point/edge/face picking | Every direct ray and projected primitive uses the accepted spec | World direct pointer paths call `projection_spec_ray_from_screen`; retained vertex/edge point tests and tutorial geometry call `projection_spec_project_point`. No remaining World call uses `Camera3d::ray_from_screen` or bare point projection for interaction. | Fresh native interaction laws and paired physical pointer diagnostics remain root-owned. |
| Marquee/lasso | Window and crossing selection compare against presented, warped screen geometry | Both component and instance selectors receive the accepted spec; vertices, edges, faces, and AABB bounds pass through `projection_spec_project_point`. Both immediate and retained World routes pass the same spec. | Fresh native selector laws remain in native9. |
| Device scale and pane bounds | Logical scene geometry stays DPR-neutral while viewport/scissor cross once to physical pixels | The prepared unit law checks viewport/scissor at DPR 1 and 2. The curvilinear encoder scales its viewport once and calls the same physical-scissor authority. | Actual multi-pane high-DPR capture is still a physical renderer receipt. |
| Panini taxonomy | Preserve the accepted semantic even though current React renders it like Fisheye | The typed payload, schema, fixture, selection state, and transport retain `mapping: panini`. React's current fragment declares but does not read the mapping uniform; WGPU therefore intentionally matches that current output. | A visual distinction requires a joint React/WGPU product change and is outside the current parity claim. |

## Current validation boundary

Fresh WASM/native compilation and physical WGPU rendering remain owned by the root integration gates. The source and oracle claims above deliberately stop at their observed receipts; no physical WGPU curvilinear image or warped pointer result has been claimed yet.
