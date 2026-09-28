# Native Icon SVG Export Implementation Audit

## Scope and conclusion

This is a read-only audit of the current React Icon SVG authority, the accepted native `World3dMeshAsset`, the installed Three `SVGRenderer`, and the smallest native integration seam. No production or test source was changed and no build was run. The installed oracle is `three@0.182.0` (`node_modules/three/package.json:2-4`).

The native SVG route should branch immediately after `IconExportAssetRequest::take_ready` returns the request-owned `World3dMeshAsset`, before `IconExportScenePreparation::new` publishes that asset into a `World3dState`. At that point the batch owns exactly the two inputs a CPU serializer needs: the validated Icon request and the accepted mesh/appearance carrier. Building the World scene and prepared GPU packet for SVG is unnecessary work and currently destroys the convenient ownership seam.

The accepted asset already contains substantially more parity data than the older export audit assumed:

- GLB node transforms and React's fixed `+π/2` X-axis frame are baked into positions and normals.
- primitive index ranges and authored material values are public through `World3dMeshAppearance::primitives()`;
- the public mesh read API exposes positions, indices, colors, edges, schemas, bounds, random typed reads, item cursors, and 16 KiB page cursors;
- the decoder's `Edges` field already implements Three `EdgesGeometry`'s 1° crease rule, four-decimal coordinate weld, boundary edges, degenerate rejection, and the React outline's local `1.001` scale before node/world transforms.

Those inputs are enough for a bounded first-party serializer for the ordinary Icon path. They are not enough for byte-for-byte or accidental-bug parity for every GLB that both loaders currently accept. The current carrier loses source RGB-versus-RGBA color arity, per-outline-object edge ranges, original transform handedness, and whether a material came from `KHR_materials_unlit`. These losses matter to the installed Projector. The clean long-term choice is to preserve the small missing semantic metadata in the accepted primitive carrier. If implementation intentionally limits the parity domain instead, that limit must be admitted before asset acceptance and covered by refusal tests; it cannot be inferred reliably after flattening.

## Current authority and exact handoff

### React Icon path

The current authority is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:264-550`:

1. `loadGlbGroup` wraps the loaded scene in a group rotated `Math.PI / 2` about X (`:268`, `:311-332`).
2. `buildIconScene` clones the model, optionally replaces every mesh material, adds borders, then adds one ambient and one directional light (`:334-374`).
3. `iconRenderCameraPose` fits an AABB-derived sphere when requested. `Box3.getBoundingSphere` means its center is the AABB center and its radius is half the AABB diagonal (`:382-422`). The unbordered loaded model is passed to fitting, so the `1.001` outline does not enlarge the fit bounds (`:544-548`).
4. `buildIconCamera` constructs a symmetric perspective or orthographic camera with near `0.1`, far `10_000`, request up/zoom, and Three `lookAt` (`:425-442`).
5. `renderIconSvg` creates a fresh `SVGRenderer`, disables auto-clear for a transparent request, renders, serializes through `XMLSerializer`, and returns UTF-8 SVG markup/data URL (`:445-463`). It never calls `setPrecision`.
6. `clipIconSvgMarkupToEllipse` post-processes the finished markup. It inserts a fixed-id ellipse clip, moves a root CSS background into a clipped `<rect>`, and wraps the body in the clipped group (`:476-505`).

Material replacement is all-or-nothing. If `request.material` exists, each GLTF mesh receives a new `MeshStandardMaterial` with color default `#9aa0ab`, metalness default `0`, roughness default `1`, optional emissive, and default front side; GLTF primitive materials and vertex colors no longer participate (`:334-350`). If it is absent, each GLTF primitive keeps its loaded material. Stroke is independent: absent or empty means black, trimmed case-insensitive `none`/`transparent` disables borders, and every other string is passed to `THREE.Color` (`:278-301`, `:352-355`).

### Native batch gap

The current batch is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📤️icon-export/🦀️.rs`:

- `Phase::Load` obtains `Result<World3dMeshAsset, String>` from `IconExportAssetRequest::take_ready`, clears the asset request owner, and immediately constructs `IconExportScenePreparation` (`:83-93`).
- scene preparation consumes/publishes the asset into an isolated World and eventually builds a GPU packet.
- only afterward, in `device`, the batch notices `IconExportFormat::Svg` and fails with `SVG icon export is not yet connected to the vector serializer` (`:153-177`).
- delivery is currently hard-coded to `image/png` (`:229-235`).

`IconExportAssetRequest::take_ready` in `…/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📥️asset/🦀️.rs:33-51` is the exact ownership seam. It returns the accepted asset without publishing it elsewhere and already participates in bounded cancellation/close.

`IconExportScenePreparation::new` in `…/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🎬️scene/🦀️.rs:145-204` re-parses the request, reads revision/AABB, and consumes the asset through `publish_world3d_asset_mesh`. The SVG serializer must be chosen before that publication. Factoring the private `ValidatedIconExportRequest` and request resolver out of this file avoids parsing the same JSON in `next`, `Load`, preview construction, and SVG construction.

## Accepted asset: exact public read surface

### Mesh owner and schema

`World3dMeshAsset` publicly owns `mesh: Mesh3dLease` and `appearance: World3dMeshAppearance` (`♾️infinite/🌍️world/🦀️.rs:1646-1669`). Its `close_step` first retires appearance records/textures and then closes the paged mesh lease. A cancelled/rejected SVG job can therefore retain the whole asset and release it through the existing bounded owner ladder.

`Mesh3dSchema` and `Mesh3dField` are in `🖱️ui/🎬️scene/📐️math/🦀️.rs:746-846`:

| Field | Public count | Item representation | SVG use |
|---|---:|---|---|
| `Positions` | `schema.vertices` | `[f32; 3]` | required for projection, culling, flat face normal, centroid |
| `Normals` | `schema.vertices` | `[f32; 3]` | not used by installed SVG face lighting; useful only as metadata/diagnostic |
| `Indices` | `schema.indices`, divisible by 3 | `u32` | required; primitive ranges address this stream |
| `Edges` | `schema.edges` | `[[f32; 3]; 2]` | required when stroke is enabled; already scaled/derived for Icon parity |
| `Colors` | zero or `schema.vertices` | `[f32; 4]` | required only for inherited materials with `preserve_vertex_color` |
| `Uvs` | zero or `schema.vertices` | `[f32; 2]` | not needed because installed SVGRenderer ignores maps |
| IDs | optional | `u32` | not used by React Icon SVG |

The whole owner is bounded to `1_024 × 16 KiB = 16 MiB` (`:747-751`). Schema validation requires a nonempty triangle mesh, complete optional per-vertex fields, and exact edge/id cardinalities (`:822-844`).

`Mesh3dLease` is a small copyable generation/revision witness. Its public reads are (`:1272-1415`):

- `generation`, `revision`, `schema`, and `aabb`;
- `vec2`, `vec3`, `vec4`, `edge`, and `u32` random typed item reads;
- `cursor(field).read_next()` for typed sequential items;
- `page_cursor(field).next(|bytes| …)` for bounded contiguous page slices.

No new mesh byte accessor is needed. A simple retained triangle cursor can read three indices and at most three positions/one color per step. Page cursors are preferable for throughput, but random typed reads are already sufficient and bounded for an initial implementation.

### Primitive materials

`World3dMeshAppearance::primitives()` is public (`♾️infinite/🌍️world/🦀️.rs:1580-1626`). Each `World3dPrimitiveMaterial` exposes:

- `first_index`;
- `index_count`;
- `SceneAuthoredMaterial3d`.

The material record (`🖱️ui/🎬️scene/📐️math/🦀️.rs:1549-1561`) contains base RGBA, emissive RGB, metalness, roughness, alpha mode/cutoff, double-sided, vertex-color preservation, optional base-color texture key, and sampler. For the installed SVG renderer the meaningful subset is:

| Carrier member | Three SVG interpretation |
|---|---|
| `base_color.rgb` | Standard diffuse in linear-sRGB |
| `base_color.a` | `fill-opacity`; exactly zero drops the face before rendering |
| `emissive` | added directly in linear space |
| `double_sided` | `DoubleSide` bypasses projected-area culling; otherwise front side |
| `preserve_vertex_color` | multiply diffuse by the Projector face color |
| `metalness`, `roughness` | deliberately ignored |
| `alpha`, `alpha_cutoff` | alpha mode/test are ignored; base alpha still becomes opacity |
| texture/sampler/UV | deliberately ignored |

Texture storage itself is private and only `texture_count()` is readable. That is not a blocker: `SVGRenderer` does not sample base-color maps, alpha maps, normal maps, or environment maps.

When `request.material` exists, construct the one override material once and apply it to every primitive. It must force vertex colors off, `double_sided = false`, opacity `1`, default emissive black, and default stroke black. When it is absent, use each primitive carrier material. This is the same precedence already used by the World material draw path.

### Decoder guarantees already matching React

The retained GLB decoder in `…/wgpu/🧊️renderer/🦀️.rs` provides three valuable guarantees:

1. `glb_world_frame` is the exact `+90°` X frame; `(x,y,z)` becomes `(x,-z,y)` (`:2373-2381`). Node TRS/matrices and that frame are baked into the accepted position stream (`:2180-2361`, `:2760-2770`). The SVG serializer must not rotate the asset again.
2. one appearance primitive is emitted per reachable node × GLTF primitive, with its output index range and material (`:2990-3025`). Triangle strips/fans have already been expanded.
3. `GlbOutlineAccumulator` uses the same four-decimal endpoint hash and `cos(1°)` threshold as installed `EdgesGeometry`, skips degenerate welded triangles, emits boundary/crease edges, multiplies primitive-local endpoints by `1.001`, then applies the node/world matrix (`:2453-2568`). The sealed `Edges` field is therefore the correct geometry source; regenerating ordinary edges in the serializer would be duplicate work and would apply translated-node scaling around the wrong origin.

## Information lost before the serializer

The following are real limits of the current accepted carrier, not missing serializer algorithms.

### Vertex-color source arity

The decoder expands every `COLOR_0` to `[f32; 4]`; VEC3 sources receive alpha `1` (`…/wgpu/🧊️renderer/🦀️.rs:2798-2828`). The primitive record only says whether colors exist.

Installed `Projector.js:717-728` incorrectly walks `geometry.attributes.color.array` in raw groups of three and later reads `a * 3`, ignoring the BufferAttribute `itemSize`. VEC3 colors therefore behave normally, while VEC4 colors are misaligned across vertices. The flattened native carrier cannot tell which behavior the React oracle used, especially when all source alpha values were one.

For intentional semantic behavior, define the native contract as RGB per vertex and add a fixture proving it. For exact installed-Three behavior, preserve `color_components: 3 | 4` per primitive and reproduce the raw-array indexing quirk. Silent guessing is not exact.

### Outline object grouping

React creates one `LineSegments` child per mesh. Projector gives every line its owning object id, which is a final tie-breaker. The native `Edges` stream contains correct endpoints but no edge range per primitive/object. Boundary edges are also flushed after topology traversal, so primitive ownership cannot be reconstructed from contiguity.

Most scenes are unaffected because line depth differs and the final paint is visually identical. Exact equal-depth ordering requires `first_edge`/`edge_count` (or an edge-object ordinal per edge) in the primitive carrier and decoder emission grouped by primitive. Treating the whole stream as one synthetic line object is a documented approximation.

### Flattened transform handedness and precision

For positive-determinant transforms, the normalized cross product of baked positions matches Three's local face normal transformed by the normal matrix. For a negative-determinant node scale, the baked-position cross product gains the determinant sign while Three's inverse-transpose face normal does not. The original matrix/determinant is no longer carried per primitive. Preserve one handedness bit or a per-primitive face-normal orientation if mirrored nodes are in scope.

Node transforms are also evaluated/stored as `f32` by the native decoder, while React keeps GLTF buffer values as floats but performs object-matrix math in JavaScript doubles. Tests should compare projected coordinates with an admitted tolerance and compare structural/style semantics exactly; raw path-number byte identity is not a defensible cross-implementation contract without changing the carrier precision.

### Unlit material kind

The native material schema records PBR factors but not `KHR_materials_unlit`. GLTFLoader maps that extension to `MeshBasicMaterial`, while the current native carrier looks like an ordinary authored material. SVGRenderer lights Standard/Lambert/Phong but does not light Basic. Preserve an `unlit`/material-kind flag or reject the extension upstream for exact parity.

Morph/skin animation is likewise not represented by the flattened accepted mesh. The bounded serializer should state that it serializes the accepted static asset, or the decoder must reject/preserve those features rather than promising original GLTFLoader parity.

## Installed Three 0.182.0 semantics to reproduce

### Camera and projection

`SVGRenderer` delegates to `Projector`; clip transform is `camera.projectionMatrix * camera.matrixWorldInverse * worldPosition`.

- perspective: `top = near * tan(fov/2) / zoom`; symmetric width uses aspect; WebGL clip Z is `[-1,+1]`;
- orthographic: the React camera starts with `left/right = ±width/2`, `top/bottom = ±height/2`, then divides the centered bounds by zoom; one world unit maps to `zoom` SVG pixels;
- the root viewBox is centered: `-width/2 -height/2 width height`;
- NDC maps as `x * width/2`, `-y * height/2`;
- near/far are exactly `0.1` and `10_000` for this Icon path.

Do not reuse `Camera3d::projection_matrix` directly: the native World matrix intentionally emits WebGPU `z ∈ [0,1]` (`🖱️ui/🎬️scene/📐️math/🦀️.rs:172-189`), while Projector tests and clips `[-1,+1]`. A dedicated Three-compatible SVG projection matrix is required.

Do not assume the existing native `stable_up` is byte/geometry exact for degenerate cameras either. Three `Matrix4.lookAt` uses a specific `0.0001` perturbation when up is parallel and substitutes local +Z when eye equals target; `Camera3d::stable_up` selects a different canonical axis. Ordinary cameras can share a resolved pose, but SVG needs the Three look-at rule or explicit validation that refuses degeneracy.

Fit behavior may be shared after it is factored from JSON construction: AABB center/half-diagonal, padding clamped to at least one, orthographic zoom from the smaller dimension, perspective distance from the smaller horizontal/vertical half-FOV, fallback direction `[1,-1,0.85]`, and perspective zoom preserved. The existing `icon_render_camera_json` already mirrors these ordinary cases.

### Triangle clipping, culling, and overdraw

Projector behavior is intentionally incomplete and must be copied, not replaced with a conventional vector pipeline:

- object-level frustum rejection uses the transformed bounding sphere. Per-triangle rejection can produce the same output without retaining that optimization.
- vertices are perspective-divided and marked visible only when x, y, and z are all within `[-1,1]`.
- triangles are polygon-clipped only against homogeneous near `z+w>=0` and far `-z+w>=0`. A clipped triangle can become a convex polygon of at most five vertices, fan-triangulated into at most three output faces.
- x/y are not geometrically clipped. A face survives when any vertex is inside the NDC cube or its NDC AABB intersects the cube; SVGRenderer then intersects its pixel AABB with the viewport. The root viewport performs final clipping.
- backface admission is projected signed area. The implementation checks `side === DoubleSide || frontFacing`; `BackSide` accidentally behaves like `FrontSide`. Double-sided faces keep the original normal rather than flipping it.
- partially clipped output copies the first original vertex's world position into every new vertex. The lighting centroid is therefore that first world position, while normal/color stay from the original triangle. This is an installed-oracle quirk.
- after viewport scaling, each accepted face is expanded by `overdraw = 0.5`: each of its three directed edges moves both endpoints outward in sequence. The mutated coordinates then feed the viewport AABB and path.

Lines use a separate homogeneous near/far parametric clip, no x/y geometric clip, no overdraw, then a pixel AABB test.

### Painter ordering

There is no depth buffer or hidden-edge removal.

Projector first stably sorts objects by `(renderOrder ascending, projected object-origin z descending, object id ascending)`. It expands objects into elements, then stably sorts all elements by `(renderOrder ascending, element z descending, object id ascending)`:

- face z is average NDC z;
- line z is maximum endpoint NDC z;
- source order survives exact ties.

SVGRenderer combines projected elements with SVGObject nodes and performs one more stable sort by `(renderOrder ascending, z descending)`, preserving Projector's id/source order for equal keys. Icon meshes and borders all have `renderOrder = 0`; the effective native key is therefore `z descending, object ordinal, source ordinal`. Far elements are serialized first, near elements later and visually on top.

An unstable native sort is acceptable only if the record contains the full deterministic key including original sequence. A stable bounded merge sort, or an unstable sort with sequence as the final key, avoids Three's quadratic insertion-sort cost while preserving its result.

### Flat lighting and colors

For `MeshStandardMaterial`, `MeshPhongMaterial`, and `MeshLambertMaterial`, installed SVGRenderer uses one flat equation in Three's linear-sRGB working space:

```text
light = ambientColor
      + max(0, faceNormal · normalize(directionalWorldPosition)) * directionalColor * directionalIntensity
      + supported point-light terms

rgbLinear = light * diffuseLinear + emissiveLinear
```

For this Icon scene there is one directional light and one ambient light. Critical quirks:

- ambient intensity is completely ignored; ambient contributes only its color;
- directional target is ignored; the normalized world position is the direction. The Icon sun is placed on a radius-120 sphere about origin, so only its normalized azimuth/elevation direction matters;
- material emissive intensity is ignored; emissive RGB is added unscaled;
- metalness, roughness, shadows, textures, environment, alpha test, depth test/write, and blending mode are ignored;
- the flat normal is computed from triangle positions and transformed by the object normal matrix. Vertex normals are populated but never used by face lighting;
- when vertex colors are active, Projector copies the color addressed by the triangle's first index only; it does not average or interpolate. Vertex alpha has no supported opacity role;
- `MeshBasicMaterial` is diffuse/vertex color without lighting; `MeshNormalMaterial` maps its view-space normal to RGB;
- opacity exactly zero skips the element; any other value is emitted directly as `fill-opacity`, even when material `transparent` is false.

After linear math, `Color.getStyle(SRGBColorSpace)` uses Three's `LinearToSRGB` transfer (`0.41666` exponent) and `Math.round(channel * 255)`, producing `rgb(r,g,b)`. Existing `ui_styling::color::linear_to_rgba8` is close but uses exponent `1/2.4`; use an explicit Three-compatible formatter in the serializer and freeze boundary values in the oracle.

### Paths and edges

Face path syntax is exactly `M{x1},{y1}L{x2},{y2}L{x3},{y3}z`. A normal filled face style is `fill:rgb(...);fill-opacity:{opacity}`. Consecutive render elements with exactly equal style concatenate their path strings into one `<path>` node. A style change flushes the current path.

Outline behavior:

- default `EdgesGeometry` threshold is 1°;
- coordinates are welded after `Math.round(component * 10_000)`;
- degenerate welded triangles are skipped;
- unmatched boundary edges and shared edges with normal dot `<= cos(1°)` are emitted;
- every surviving segment remains visible; there is no hidden-line removal;
- the React outline scales geometry by `1.001` in mesh-local space;
- `LineBasicMaterial` defaults used here are opacity `1`, linewidth `1`, linecap `round`;
- line path syntax is `M{x1},{y1}L{x2},{y2}` with `fill:none;stroke:rgb(...);stroke-opacity:1;stroke-width:1;stroke-linecap:round`;
- consecutive same-style segments merge into one path.

The accepted native edge stream already supplies the correct endpoints and topology. The serializer only projects/clips/sorts/styles them.

### Background, transparency, ellipse, and serialization

`SVGRenderer` starts with a white clear color. With auto-clear true and no scene background it removes children and sets root CSS background to white. A `Color` scene background similarly sets root CSS. The React Icon path uses a fresh renderer and sets `autoClear = false` for absent or exactly lowercase `transparent`, so transparent rectangle output has no root background style.

For a rectangle with explicit background, keep `style="background-color: rgb(r, g, b);"` on the root. For an ellipse:

- derive `(x,y,w,h)` from the centered viewBox;
- emit `<clipPath id="semio-icon-ellipse-clip"><ellipse cx="x+w/2" cy="y+h/2" rx="w/2" ry="h/2"/></clipPath>`;
- remove the root background declaration and, when present, insert `<rect x="x" y="y" width="w" height="h" fill="…"/>` first inside the clipped group;
- wrap all rendered paths in `<g clip-path="url(#semio-icon-ellipse-clip)">`;
- transparent ellipses contain no background rect.

For the centered Three viewBox the ellipse center is `(0,0)`, not `(width/2,height/2)`.

No precision is configured, so Three converts coordinates through JavaScript's ordinary number-to-string conversion. Native serialization should at minimum normalize signed zero to `0`, reject nonfinite projected values, and use deterministic shortest finite numbers. Exact raw bytes across JS-double versus accepted-f32 geometry should not be the primary oracle; compare parsed path/style semantics and separately raster-probe the resulting SVG.

## Smallest clean implementation seams

### 1. One shared validated request packet

Move/factor the private request validation from `📤️export/🎬️scene/🦀️.rs` into the Icon export module as a `pub(crate)` packet. It should retain resolved values rather than forcing SVG to parse JSON emitted for World:

- transported asset URL and original request URL;
- dimensions, format, shape, transparent/explicit background;
- resolved fitted pose, Three-compatible camera parameters, near/far;
- ambient/sun colors and normalized sun direction;
- optional resolved override material and resolved stroke;
- shadow flag retained for PNG but explicitly irrelevant to SVG.

`asset_url` should parse this packet once. `IconExportScenePreparation::new` should accept the packet for PNG. The SVG job should accept the same packet plus the still-owned asset. This keeps schema-first request interpretation one authority.

The current private `parse_color` in World supports short/long hex and otherwise silently substitutes a fallback. `THREE.Color` accepts a wider CSS grammar. Either make the request schema explicitly admit a bounded canonical color grammar and reject outside it in both renderers, or implement that admitted grammar in the shared packet. Silent fallback is not parity. Current Shooting emits hex colors, so hex-first is a valid tested slice, but the public TypeScript contract presently says arbitrary `string`.

### 2. A retained CPU SVG job

Add a domain file such as `…/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🖼️svg/🦀️.rs`, registered by the existing export `🦀️.rs`. It should own `World3dMeshAsset` until success/cancellation retirement and advance in explicit bounded phases:

1. **Admit**: validate schema, primitive ranges, bounds, finite values, record-count arithmetic, and an explicit SVG output byte ceiling before large allocation.
2. **Resolve camera/materials**: compute fitted pose and Three WebGL view/projection matrices.
3. **Project faces**: one source triangle or a small fixed fuel batch per `advance`; near/far clip to at most three face records, compute normal/centroid/style/depth/object/source keys.
4. **Project edges**: one accepted edge per step when stroke is enabled; near/far clip and create line records.
5. **Order**: bounded stable merge passes, or deterministic full-key sort within the admitted fixed record allocation.
6. **Serialize**: emit root/defs/group and coalesce adjacent equal styles incrementally, checking the byte ceiling on every append.
7. **Complete**: publish bytes only after the closing `</svg>` exists.
8. **Close**: discard candidate bytes/records and call `asset.close_step()` until terminal empty.

Checked capacity can derive from the schema: source triangles are `indices / 3`; near/far clipping emits at most `3 × sourceTriangles`; line records are at most `schema.edges`. Primitive count is already bounded by `WORLD_DYNAMIC_MESH_CAPACITY`, and mesh bytes by 16 MiB. The SVG byte cap must still be explicit because textual expansion and path coordinates are not bounded by the mesh byte count alone.

Progress should report admitted source triangles/edges, sort passes, and serialized records/bytes. Cancellation must be observed between every expensive phase and must never expose a partial string.

### 3. Branch the batch before World/GPU preparation

At `IconExportBatch::Phase::Load`, after `take_ready`:

- SVG: construct the retained CPU job directly from the validated packet and asset, then advance it in a dedicated render/serialize phase;
- PNG: pass the packet and asset to existing `IconExportScenePreparation`, then continue through device/readback/PNG.

Add the SVG job/rejection owners to `retire`, `terminal_is_empty`, progress, and cancellation. Delivery must carry a per-result MIME: `image/svg+xml` for SVG and `image/png` for PNG. No `GpuContext` should be initialized for an SVG item.

### 4. Preserve only metadata proven necessary

No new general mesh read API is required. For full installed-oracle parity, the smallest carrier extension is primitive-level metadata:

- source vertex-color component count;
- edge range/object ordinal, with decoder output grouped or tagged by primitive;
- transform handedness for flat face-normal orientation;
- lit versus unlit material kind.

These belong beside `first_index/index_count/material` because they describe the same node × primitive render object. They should not become SVG-only side tables. If the chosen product contract intentionally corrects Three's VEC4 bug or excludes unlit/mirrored assets, encode that in neutral schema/decoder acceptance and tests instead of carrying unused compatibility data.

## Test plan

### Existing seed proof

There is already a useful new seed:

- `…/🖼️IconRenderHost/🧫️fixtures/📤️svg-export/🔣️.json`;
- matching JSON schema;
- `…/🧪️tests/📤️svg-export/🟦️.ts`.

It runs the installed Three `SVGRenderer`, `EdgesGeometry`, the production ellipse helper, and Sharp. It proves one perspective triangle's fill, black outline, explicit background, ellipse clip, path count, and three raster probes. Keep it, but it is not yet a native parity test and it does not constrain clipping, ordering, orthographic projection, transparency, inherited primitive materials, vertex colors, or cancellation.

The existing `🎨️icon-svg-lighting` neutral fixture/oracle separately proves the important ignored-parameter behavior: changing ambient intensity, metalness, roughness, and emissive intensity does not change SVG flat-light output.

### One shared semantic oracle matrix

Evolve the neutral SVG fixture to a versioned array of compact cases and consume it from both:

- a TypeScript oracle that uses the actual installed Three renderer/Projector and records normalized SVG semantics plus Sharp probes;
- a Rust unit test for the first-party serializer that reads the same geometry/request cases and compares those normalized semantics.

Normalize into an explicit record rather than snapshotting XMLSerializer whitespace: root size/viewBox/background, clip/rect geometry, ordered path `{d, style}` values, and selected raster probes. Coordinate comparisons may use a small tolerance; path order, style, color, opacity, clip structure, and MIME are exact.

Required cases:

1. perspective and orthographic projection, zoom, fit center/radius, non-square aspect, and the Three degenerate-look-at rule;
2. triangle wholly inside, near-plane split, near-and-far slab split, wholly rejected, and x/y partial viewport clipping;
3. front-side rejection, double-sided admission without normal flip, and an explicit oracle case freezing installed BackSide-as-FrontSide behavior even though GLTF does not normally request BackSide;
4. overlapping near/far faces, equal-depth faces from separate primitive objects, source-order ties, and adjacent same-style path coalescing;
5. request material override versus inherited primitive materials, base alpha/zero opacity, double-sided, first-index vertex color, VEC3/VEC4 source distinction, unlit, and mapped material proving maps are ignored;
6. ambient intensity ignored, sun direction/intensity, back-facing sun, emissive intensity ignored, and exact sRGB rounding boundaries;
7. square split into coplanar triangles (diagonal omitted), hard crease (shared edge included), boundary edges, degenerate triangle, `1.001` local scale under translated node, line clipping, and same-style line coalescing;
8. transparent rectangle, explicit rectangle background, transparent ellipse, explicit ellipse background moved to clipped rect, centered non-square ellipse;
9. record-cap refusal, byte-cap refusal, cancellation during face projection/sort/serialization, and terminal-empty asset retirement.

### Batch integration laws

Add focused native tests at the existing Icon export/batch seams:

- an SVG item never initializes `GpuContext` or constructs `IconExportScenePreparation`;
- exact bytes are delivered once with `image/svg+xml`;
- PNG remains `image/png` and follows the existing packet route;
- sequential mixed PNG/SVG items retain order;
- cancelling before asset readiness, during projection, during sort, during serialization, and during save yields no partial delivery and closes every owner;
- one item fault does not leak owners or publish bytes and preserves the batch's current failure accounting.

No broad build was run during this audit, so this report makes no passing-test claim.

## Implementation checklist

- [ ] Factor a shared validated Icon export packet and camera/material/color resolvers.
- [ ] Keep SVG ownership before World publication; do not construct a prepared GPU packet.
- [ ] Use Three WebGL `[-1,+1]` projection and exact `lookAt`, not the WGPU camera matrix.
- [ ] Read `Positions`, `Indices`, optional `Colors`, and accepted `Edges` through existing `Mesh3dLease` APIs.
- [ ] Respect primitive ranges/material precedence and base alpha/double side.
- [ ] Copy Projector's near/far-only clipping, signed-area cull, clipped-centroid quirk, overdraw, depth keys, and stable order.
- [ ] Copy SVGRenderer's flat-light equation, ignored parameters, sRGB conversion, path syntax, and equal-style coalescing.
- [ ] Emit root background versus ellipse background rect exactly; use centered viewBox coordinates.
- [ ] Admit record and byte ceilings before allocation; advance/cancel/close incrementally.
- [ ] Deliver `image/svg+xml`; never initialize GPU for SVG.
- [ ] Decide and encode the four lost-metadata semantics instead of guessing.
- [ ] Make the installed Three oracle and native serializer consume the same language-neutral cases.

## Files inspected

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/…`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📤️icon-export/🦀️.rs`
- `node_modules/three/examples/jsm/renderers/SVGRenderer.js`
- `node_modules/three/examples/jsm/renderers/Projector.js`
- `node_modules/three/src/geometries/EdgesGeometry.js`
- `node_modules/three/src/cameras/{PerspectiveCamera,OrthographicCamera}.js`
- `node_modules/three/src/math/{Matrix4,Color,ColorManagement,Frustum}.js`
- `node_modules/three/examples/jsm/loaders/GLTFLoader.js`

