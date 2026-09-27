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

## Current validation boundary

Fresh WASM/native compilation and physical WGPU rendering remain owned by the root integration gates. Panini intentionally matches Fisheye because the current React shader declares its mapping uniform but does not read it; the typed payload retains the mapping without claiming a visual distinction that React does not provide.
