# Procedural World Grid Implementation Packet

## Decision

Replace only the LOD-grid lane with one procedural triangle draw per visible ScenePass3d. Keep reference outlines, component overlays, engagement previews, pick-target marks, attractions, volumes, and gumball wireframes on LineDraw3d. The current React grid is Drei's derivative-antialiased fragment program over a camera-following plane. A thicker or lighter LineList cannot reproduce it.

This is source-only research. No source was changed and no Cargo or browser test was run.

## Confirmed Current Route

1. React mounts the installed Drei Grid with plane position [0, 0, datum[2] + 0.001], args [2,2], one cellSize, cellThickness 0.6, sectionThickness 0, fadeStrength 1.5, followCamera, and infiniteGrid. It sets depthTest true, depthWrite false, and renderOrder -5. Source: framework/product/os/module/infinite/world/r3f/index.tsx lines 934-985.

2. Installed Drei expands its plane by 1 + fadeDistance, projects the camera position onto the world plane, and translates the plane by that projection. Its fragment shader uses fwidth coverage and radial fade. With section thickness zero g2 is zero, so alpha is exactly 0.75 * g1 * pow(d, 1.5), followed by discard and Three colour chunks. Source: node_modules/@react-three/drei/core/Grid.js lines 5-117.

3. Native calls append_lod_grid_lines with camera.target and emits target-anchored LineVertex3d pairs into the shared overlay vector. Source: framework/product/os/module/infinite/world/🦀️.rs lines 12799-12818 and 7731-7777.

4. Those vertices reach world3d_line_pipeline, a LineList. The prepared cursor measures every vertex, and the actual GPU arm submits each completed two-vertex segment. Sources: framework/ui/targets/wgpu/draw/🦀️.rs lines 3167-3191 and 4324-4365; framework/ui/targets/wgpu/prepared/🦀️.rs lines 2407-2426 and 2718-2732; framework/ui/targets/wgpu/gpu/🦀️.rs lines 879-895.

The source establishes a geometric and raster-policy mismatch. It does not quantify final pixels; the pixel law below does that.

## Minimal Retained Schema and Producer

Add one private, ephemeral record next to ScenePass3d in framework/ui/scene/math/🦀️.rs:

    pub struct ProceduralGrid3d {
        pub plane_z: f32,
        pub camera_plane_projection: [f32; 3],
        pub cell_size: f32,
        pub fade_distance: f32,
        pub cell_color: [f32; 3],
    }

    pub struct ScenePass3d {
        // existing fields
        pub procedural_grid: Option<ProceduralGrid3d>,
    }

There is deliberately no document JSON field and no list: all values derive on every retained paint, while React mounts at most one Grid per world host. Fixed policy must be named shared constants: cell thickness 0.6, section thickness 0, fade strength 1.5, infinite grid true, follow camera true.

render_world_3d creates Some only if show_grid is true and lod_grid_step_world returns a positive finite step:

    datum_z                 = state.lod.grid_datum.unwrap_or([0,0,0])[2]
    plane_z                 = datum_z + 0.001
    cell_size               = lod_grid_step_world(current_lod, grid_factor)
    fade_distance           = camera_grid_fade_distance(camera, datum_z,
                              cell_size, inner.w, inner.h)
    camera_plane_projection = [camera.position.x, camera.position.y, plane_z]
    cell_color              = theme.text_element.rgb

The fade helper receives datum_z, as React does; the rendered plane alone gets +0.001. The projection uses camera.position, never camera.target. Camera3d owns both and computes its matrices from logical viewport dimensions in framework/ui/scene/math/🦀️.rs lines 239-295.

Remove only append_lod_grid_lines from render_world_3d. Leave all following append_*_lines calls untouched. This removes grid vertices and grid line-cursor work without changing interaction overlays.

## WGSL, Pipeline, and Buffer Ownership

Add WORLD3D_GRID_SHADER in both the WGPU target shader source and the shared shader contract, then extend the existing shader-contract parity unit. It uses:

- group 0: the existing dynamic World3dGlobals binding;
- group 1 binding 0: one fixed-size World3dGridUniforms uniform holding plane Z, projected camera position, linear cell RGB, cell size, cell thickness, fade distance, and fade strength;
- vertex location 0: the existing WORLD_PLANE_VERTICES position attribute.

The existing buffer is a centred [-0.5,+0.5] XY unit plane: framework/ui/targets/wgpu/draw/🦀️.rs lines 400-410. Reuse it. The grid vertex shader multiplies XY by 2 * (1 + fade_distance), exactly matching Drei PlaneGeometry(2,2) followed by its 1 + fadeDistance scale. It then translates XY by camera_plane_projection.xy and sets Z to plane_z. The native XY/Z-normal CAD plane is the equivalent of Drei's rotated XZ local plane.

The effective zero-section fragment branch is:

    r       = local_xy / cell_size
    grid    = abs(fract(r - 0.5) - 0.5) / fwidth(r)
    g1      = 1 - min(min(grid.x, grid.y) + 1 - 0.6, 1)
    d       = 1 - min(distance(camera_plane_projection, world_position)
                      / fade_distance, 1)
    alpha   = 0.75 * g1 * pow(d, 1.5)
    discard alpha <= 0

The pipeline uses triangle-list, alpha blending, LessEqual depth, depth writes disabled, no culling, and world-content stencil state.

RGB is already linear. Theme::text_element lifts the generated linear border_element token in framework/ui/targets/wgpu/theme/🦀️.rs lines 22-44 and 289-294. React passes an sRGB hex to THREE.Color, which decodes it into its Linear-sRGB working space; see node_modules/three/src/math/Color.js lines 47-63 and 204-214. The grid fragment must pass the existing linear theme RGB unchanged to world3d_attachment_output once. Do not decode Theme again and do not use the textured MeshBasic transfer.

UiPipelines owns one persistent 256-byte grid-uniform buffer and bind group. Each PassGrid scalar overwrites it before encoding, using the existing World3dGlobals ring for group 0. The existing six-vertex plane buffer is also pipeline-owned. This is constant ownership: no per-frame plane allocation, no per-line GrowBuffer upload, and no retirement work. Serial scalar queue operations make rewriting this one buffer safe.

encode_prepared_world_grid must reuse the textured encoder's logical ScenePass3d viewport, physical_scissor, and surface_scale path. The existing encoder is framework/ui/targets/wgpu/draw/🦀️.rs lines 4297-4316. Fade and projection use logical dimensions; DPR applies only to WGPU viewport/scissor. This is the renderer's documented invariant at draw/🦀️.rs lines 2612-2617.

## Bounded Prepared-Frame Integration and Order

Add DrawMeasureCursor::PassGrid { pass } with one item and size_of::<ProceduralGrid3d>() measurement.

1. PassHeader and shadows remain unchanged.
2. Textured references remain first.
3. PassGrid runs when procedural_grid is Some.
4. Existing opaque material/draw, line, translucent, and translucent material lanes follow.

That preserves React reference content -10, grid -5, normal meshes 0. The current cursor already orders textured before opaque content: framework/ui/targets/wgpu/prepared/🦀️.rs lines 2874-2989. Insert the grid cursor from next_after_textured, then continue into the present opaque path.

One cursor means one six-vertex draw. A missing grid creates no cursor, upload, or command. Prepared packets remain immutable, so cancellation and abandonment occur between whole grid scalars. No division loop, extra cursor state, or partial-grid state is introduced.

Update both world-encoded/layer-index classifiers and the real GPU match in framework/ui/targets/wgpu/gpu/🦀️.rs; otherwise the new cursor would select the UI attachment or be silently unencoded.

## Fail-First Laws

### Native producer and prepared-frame

1. A World unit invokes actual render_world_3d with showGrid true and a perspective camera whose camera XY differs from target XY. It requires exactly one procedural grid, no grid vertices in line_draws, plane Z datum+0.001, Lod step, React fade-helper result, linear text_element RGB, and camera-projected XY. Mutants using target XY, +0.002, or append_lod_grid_lines fail.

2. A prepared-WGPU unit walks PassHeader to completion and requires one measured and encoded PassGrid after a textured reference and before an opaque instance. It also proves ordinary LineDraw3d overlays still receive PassLineVertex. Cancellation or abandonment before PassGrid leaves no partial grid admission.

3. A DPR unit uses one logical viewport at scales 1 and 2. It requires identical grid uniform/view-projection/fade inputs, with viewport/scissor alone multiplied at the WGPU encoder boundary.

### Three and production-WGSL pixels

Add framework.world3d.grid-visual/v1 next to the existing reference-visual schema and fixture. Extend framework/product/os/module/renderer/engine/tests/world3d-scene-shading/script.ts. Its reference-visual and reference-visual-wgpu commands already validate JSON, render Three, load target WGSL constants, and read Chromium WebGPU pixels.

The Three branch must mount installed Drei Grid with the production props, not copy its GLSL. The WGPU branch loads WORLD3D_GRID_SHADER from framework/ui/targets/wgpu/shaders/🦀️.rs, compiles it in Chromium WebGPU, and constructs the declared group layouts, blend/depth state, static plane, and uniforms. It must not copy WGSL to test code.

Record and compare these rows at DPR 1 and 2:

| Case | Fails when |
| --- | --- |
| Orthographic cell centre and line centre | LineList, wrong 0.6 coverage, wrong 0.75 alpha |
| One physical-pixel neighbour of a line | fwidth antialiasing is absent |
| Mid-fade and beyond-fade samples | fade is non-radial, exponent wrong, or discard missing |
| Perspective camera move with target fixed | native anchors to target instead of projected camera |
| Light and dark element token samples | Theme is double-decoded or output transform drifts |
| Logical viewport at DPR 1 and 2 | DPR leaks into camera/fade math |

Use robust line-centre and interior samples rather than exact every-edge equality across DPR, where derivative coverage necessarily changes.

## Reference Visual Law Route

No new reference fixture or independent matrix oracle is needed. The appropriate real-producer route now exists as reference_visual_geometry_reaches_the_actual_textured_scene_pass in the World unit module. It puts the shared fixture JSON through render_world_3d, supplies its decoded raster descriptor, and inspects emitted TexturedInstance3d model corners. See the unrun law at framework/product/os/module/infinite/world/tests/unit/🦀️.rs lines 5088-5130 and ticket report 📓️astra-reference-scene-pass-law.md.

Keep that structural gate for reference placement while grid-visual covers pixels. It is unrun, therefore fail-first rather than passing evidence. Do not add orientation or scale to the native reference record for this task: the real World3dHost mapping does not publish those generic R3F fields, as documented in 📓️terra-checkpoint17-reference-grid-owner-reconciliation.md.

## Confidence

High: native is a finite target-anchored LineList and React is a camera-following derivative fragment grid. The separate record, one cursor, persistent uniform, and shared plane are the minimal owner boundary.

High: Theme::text_element is already linear and must remain unchanged before the world attachment output transform.

High: the new reference scene-pass law is the proper actual render_world_3d structural boundary.

Medium: exact recorded RGBA values until installed-Drei and production-WGSL browser pixel rows run. This packet does not assign visible darkness to a particular colour-transform mutation before that evidence exists.
