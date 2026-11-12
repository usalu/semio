# Model Viewer — Report

Scope: `🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/` (V below). Owned, dependency-free WebGL2 renderer/viewer for `semio.model/v1`. Domain-neutral, no React, no runtime library, browser platform APIs only. Normative source: `design.md`.

Import path (repo-relative): `🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts`. Everything below is exported from there.

## Files

| File | Content |
|---|---|
| `V/🟦️.ts` | public entry (re-exports) |
| `V/🧬️contract/🟦️.ts` | `Scene`, `Frame`, `Draw`, `Light`, `RenderBackend`, `RenderStyle`, strides |
| `V/🧱️scene/🟦️.ts` | `sceneOf`, `sceneOfAsync`, `buildScene`, `SceneError` |
| `V/📷️camera/🟦️.ts` | orbit camera, presets, fit, matrices, rays |
| `V/🎞️frame/🟦️.ts` | `frameOf`, `createFrame`, `posedPositions`, `boundsOf` |
| `V/🎯️picking/🟦️.ts` | `pick`, `pickAt` |
| `V/🖼️software/🟦️.ts` | `SoftwareBackend` (reference rasteriser) |
| `V/🔶️webgl2/🟦️.ts`, `V/🔶️webgl2/🎨️shaders/🟦️.ts` | `WebGl2Backend`, `createWebGl2Backend`, GLSL ES 3.00 sources |
| `V/🕹️controller/🟦️.ts`, `V/🕹️controller/🎨️color/🟦️.ts` | `ModelViewer`, `parseCssColor` |
| `V/🧩️element/🟦️.ts` | `defineModelViewerElement` (custom element) |
| `V/🧫️fixtures/{🧱️rig,📷️camera,🎞️frame,🎯️picking,🖼️raster}/🔣️.json` | language-agnostic vectors (rig document; camera matrices/fits/rays recorded from three; palettes, skinned vertices, colours; picks; per-pixel coverage masks and depths from a three Raycaster) |
| `V/🧪️tests/{📷️camera,🧱️scene,🎞️frame,🎯️picking,🖼️raster,🕹️controller,🔶️webgl2,🔶️gpu}/🟦️.ts` | Vitest suites |
| `V/🧪️tests/🧰️support/🟦️.ts` | three.js oracle helpers (test only) |
| `V/🧪️tests/🧰️support/🖥️harness/🟦️.ts` | browser-side GPU harness (WebGL2 vs software, context loss) |
| `T/gen-viewer-fixtures.ts`, `T/probe-viewer-raster.ts`, `T/build-gpu-harness.ts`, `T/vitest.viewer.config.ts` | fixture generator (procedural rig -> kernel `tessellate` + `featureEdges` -> resolved JSON; expectations from three), mask-mismatch probe, esbuild harness bundler, direct vitest config |
| `T/kernel-requests.md` | one non-blocking kernel request (solid ids in resolved primitives) |

The shared test config (`🧊️3d/🧪️tests/🎚️config/🟦️.ts`) already globs `🗿️model/*/🧪️tests/*/🟦️.ts`; no config edit was needed. The jsdom suite selects its environment by docblock.

## Public API

```ts
// contract
type Bounds = { min: V3; max: V3 };
const VERTEX_STRIDE = 10, SKIN_STRIDE = 8, MATERIAL_STRIDE = 12;
type Scene = { document; nodeIds; nodeIndex; materials: SceneMaterial[]; materialIndex; variants: Map<Slug, Int32Array>;
  vertices: Float32Array /* pos3 normal3 rgba4 */; skinning: Float32Array | null /* joints4 weights4 */; indices: Uint32Array /* global */;
  primitives: ScenePrimitive[]; skins: SceneSkin[]; jointCount: number };
type Frame = { view; projection; viewProjection; eye; matrices /* (nodes+1)*16, last = identity */; palette /* jointCount*16 */;
  materials /* MATERIAL_STRIDE per material: base rgba, emissive rgb, unlit, metallic, roughness, doubleSided, blend */; materialMap: Int32Array;
  draws: Draw[]; drawCount: number; light: Light; issues: Issue[] };
type Draw = { primitive; matrixOffset; material; paletteOffset /* -1 rigid */; pass: 0 | 1 | 2 /* opaque, blend back-to-front, lines */; depth };
interface RenderBackend { readonly kind: string; load(scene: Scene): void; resize(cssWidth: number, cssHeight: number, pixelRatio: number): void;
  render(frame: Frame, style: { background: V4 /* straight sRGB rgba */ }): void; readPixels(): Uint8ClampedArray /* RGBA, top row first */; dispose(): void }

// core
sceneOf(document: ModelDocument, operation?: Operation): Scene            // resolved documents only; throws SceneError / CancelledError
sceneOfAsync(document, operation?): Promise<Scene>                          // yields to the event loop every ~12 ms
createFrame(scene): Frame
frameOf(scene, input: { pose?: PoseSelector; variant?: string | null; camera: CameraState; aspect: number; lighting?: Lighting }, into?: Frame): Frame
boundsOf(scene, frame): Bounds;  posedPositions(scene, frame, primitiveIndex): Float64Array   // CPU skinning, shared scratch
type Lighting = { mode?: "camera" | "world"; direction?: V3; intensity?: number; ambient?: number }

// camera
type CameraPreset = "elevation" | "front" | "plan" | "axonometric" | "perspective";
type CameraState = { projection; target: V3; yaw; pitch; distance; fovY; orthoHeight; extent };
cameraOf(preset?), withPreset(camera, preset), fitCamera(camera, bounds, aspect, margin = 1.1), orbit(camera, dYaw, dPitch), pan(camera, dx, dy, aspect),
zoom(camera, factor), viewMatrix(camera, out?), projectionMatrix(camera, aspect, out?), rayOf(camera, aspect, ndcX, ndcY) -> { origin, direction }, basisOf, clipPlanes, visibleHeight, PRESETS

// picking
pick(scene, frame, ray): Hit | null;  pickAt(scene, frame, camera, aspect, ndcX, ndcY): Hit | null
type Hit = { primitive; node; mesh; material /* variant-resolved */; tags; triangle; distance; point: V3; normal: V3 }

// backends
class SoftwareBackend implements RenderBackend { constructor(width?, height?); width; height; pixels; depth /* NDC z, Infinity empty */; present(canvas2d, ImageDataClass) }
createWebGl2Backend(canvas: HTMLCanvasElement, onStatus?: (s: "ready" | "lost" | "restored") => void): RenderBackend   // throws when WebGL2 is unavailable

// controller
new ModelViewer(options: ModelViewerOptions)
type ModelViewerOptions = { canvas; strings: ViewerStrings; backend?: BackendFactory; fallback?: boolean; background?: V4 | null; lighting?; quality?: Quality;
  autoplay?; reducedMotion?; controls?: ViewerControls; preset?: CameraPreset; fitMargin?; maxPixelRatio? }
type ViewerStrings = { label: string; description: string; roleDescription?: string; status?: (s: ViewerStatus) => string }   // no built-in language
viewer.on(type, handler): () => void    // progress, ready, error, camera, time, playing, pick, backend, issues
viewer.setDocument(doc, signal?): Promise<void>; cancelPreparation(); setPose(id|null); setAnimation(id|null, timeS?); play(); pause(); seek(t); setRate(r); setLoop(bool|null);
setVariant(id|null); setPreset(p); fit(); getCamera(); setCamera(c); orbitBy(dYaw, dPitch); panBy(dx, dy); zoomBy(f); pickAt(clientX, clientY): Hit | null;
readPixels(); render(); requestRender(); get clock; get backendKind; dispose()
defineModelViewerElement(tagName = "semio-model-viewer"): element with properties strings, options, model, viewer; attributes preset, variant, animation, playing;
  events `semio-<event>` (CustomEvent, payload in detail); CSS custom properties --semio-viewer-background, --semio-viewer-focus, --semio-viewer-min-height
parseCssColor(text): V4 | null     // hex, rgb()/rgba() (comma/space, %, / alpha), transparent
```

Behaviour contract for a React wrapper: create the `ModelViewer` in an effect on a `<canvas>`, call `dispose()` on cleanup, forward props to the setters, subscribe to events (`pick`, `time`, `playing`, `camera`, `progress`, `error`); the viewer holds no application state beyond what it shows. A procedural document is accepted when `quality` is given (kernel `tessellate` runs first, progress 0..0.5 then 0.5..1).

Conventions: camera at `target + distance * (cos p sin y, sin p, cos p cos y)`; `front` from +Z; `elevation` from -X (ahead = +Z runs to the right of the picture); `plan` from above (ahead to the right, asset's left +X at the top); `axonometric` isometric corner, orthographic. Shading in linear space, output gamma 2.2 (WebGL2 and software alike), colours of the document are linear (glTF convention), the background option is sRGB.

## Test command and result

- Project target: `repo run @semio-tech/framework-3d-js:test --output=stream` (config glob already covers V). Result: see "Project target" below.
- Direct fallback: `bun node_modules/vitest/vitest.mjs run --config T/vitest.viewer.config.ts` from the repo root (bun from `repo tools list`).

Direct result: 8 files, 138 tests: 137 passed, 0 failed, 1 skipped (the Playwright WebGL2 test; no Chromium installed here). Per file: camera 44, controller (jsdom) 31, frame 17, raster 16, webgl2 stub 10, scene and picking 19 together, gpu 1 skipped.

## Oracles (all test-only)

- three 0.182: view/projection matrices and rays of every preset (recorded into the vectors and re-checked live); fit tightness by projecting the box corners; `Skeleton`/`SkinnedMesh.getVertexPosition` for palettes and skinned vertices (bent pose and a clip time); `Raycaster` for picks; a per-pixel `Raycaster` over the posed meshes for the software rasteriser's coverage masks and depths in all five presets (0 mismatching pixels of 1280 in each; the test allows 0.5%; depths within 0.2%).
- Hand-computed expectations: analytic orthographic fits, animated/variant colours, draw order and blend sorting.

## What was verified on a real GPU

- Manually, in the desktop app's browser pane (Chromium, ANGLE Metal, Apple GPU): the harness bundle rendered the rig with `WebGl2Backend` and `SoftwareBackend` for 5 presets x 2 variants (160x128). Shader compile/link OK; coverage agreement 98.8–99.9 % (differences are MSAA edges and 1 px line rasterisation); mean absolute colour difference over covered pixels 4–31 of 255 (smooth per-vertex normals and the GPU's lines versus flat facets and a DDA line in the reference); the picture is visually the same (side-by-side and difference images inspected). Context loss via `WEBGL_lose_context` and restore: status `ready, lost, restored`, the re-rendered image is byte-identical. `ModelViewer` on that canvas: backend `webgl2`, progress reached 1, variant/animation/keyboard calls and ARIA attributes worked.
- Automated: `🧪️tests/🔶️gpu` would repeat this in headless Chromium through Playwright (SwiftShader) with tolerances (coverage > 97 %, colour < 40); it skips without a browser and was skipped here, so the automated GPU path has not run. The WebGL2 backend is otherwise tested against a recording stub context (compile/link, uniforms looked up equal those declared in the sources, buffer layout, palette texture, draw modes/ranges/uniform offsets, pass state, culling, DPR sizing, loss/restore, dispose).
- Not verified on a real browser: the `requestAnimationFrame` loop (the pane was hidden, so frames were suspended); animation timing, visibility pause, pointer/touch/keyboard handling, `ResizeObserver`/`IntersectionObserver`, the custom element and `prefers-reduced-motion` are covered in jsdom with a manual frame clock and stubs only. No touch device, no Firefox/Safari run, no WebGL2-less browser run of the software fallback.

## Behaviour decisions

- Skinned primitives use the identity model matrix and palette `world(joint) * inverseBind`; skin palettes of all skins are one float texture (4 texels per joint, one joint per row, `RGBA32F`, `texelFetch`), so joint count is limited only by the texture size.
- One vertex buffer, one skin buffer, one global index buffer (32-bit) and one VAO for the whole scene; geometry shared by several nodes is stored once. Per-frame draw loop: uniforms by offset into frame arrays (`uniformMatrix4fv` with source offset), tracked state; no allocation in the backend. `frameOf` allocates only inside the kernel's `poseModel`/`sampleMaterials`; a static viewer renders on demand and does not call it idly.
- Lit shading: base x vertex colour, hemisphere ambient, one directional light (camera-relative by default), Blinn specular scaled by `(1 - roughness)^2`, emissive; double-sided flips the back-face normal; unlit materials and lines show colour + emissive. Blended materials: premultiplied alpha, no depth write, back to front by view depth of the bounds centre (of the rest bounds for skinned primitives). Lines: depth `LEQUAL` after triangles drawn with polygon offset (1, 1); width is always 1 device pixel (WebGL does not guarantee more).
- Animated material colour channels apply to the material id they name (so they colour the variant-bound material), independent of which variant is chosen.
- Animation clock: `requestAnimationFrame`; suspended while `document.hidden` or the canvas is outside the viewport; resumes without a time jump; `setLoop` overrides the clip's flag; negative rates play backwards; `reducedMotion` (option or media query) blocks autoplay only.
- The software rasteriser shades flat per face and has no specular; it blends in sRGB space like the GPU path.

## Deviations and open issues

- Solid ids are not recoverable from a resolved document (tessellation merges solids per node and material); picking reports node, mesh, primitive, material, triangle and tags. Request appended to `T/kernel-requests.md`.
- `lines` are 1 device pixel; thicker outlines would need quad expansion in a later iteration.
- Skinned blended primitives are depth-sorted with their rest bounds centre, which can mis-order after large poses.
- CSS colour parsing covers hex and `rgb()/rgba()` only; other syntaxes (named, `hsl()`, `color-mix()`) fall back to white unless `background` is passed.
- Lighting is not shadowed and has no environment map; PBR is "lite" (metallic only dims diffuse and tints specular).
- The project target `repo run @semio-tech/framework-3d-js:test` was queued behind the host's memory gate for the whole session (see "Project target"); the direct vitest run is the evidence. `tsc` over V reports no errors in V's own sources; the test support file only lacks `node:*` typings, like the kernel's.
- Ticket scratch: bundle, rig copy and `repo-test.log` live in `T/🗑️generated/viewer/`; delete that folder when the ticket closes.

## Project target

Not obtained: queued by the capacity gate ("host is swapping out too fast"); see `T/🗑️generated/viewer/repo-test.log`.

## Maintenance update (A5, `kernel-maintenance.md`)

- Solid identity is now in the resolved document: `ScenePrimitive.solids`, `Hit.solid` (`null` without provenance), `solidOf(solids, index)` (binary search) exported from the barrel. The rig and picking vectors were regenerated with `T/gen-viewer-fixtures.ts` (expected solid per pick case from the rig's solid list); viewer suite: 8 files, 141 tests, 140 passed, 1 skipped (GPU). The limitation "Solid ids are not recoverable" above is resolved.

## Follow camera and root motion (react-site request)

Module `V/🎥️follow/🟦️.ts`, wired through `frameOf` (`FrameInput.rootMotion`, `rootNode`, `follow`), the `ModelViewer` controller and the custom element. Defaults are unchanged: `follow: "off"`, `rootMotion: "travel"`.

- **`follow`**: `"off"` leaves the camera target alone; `{ node }` tracks the world translation of that node; `"bounds"` the centre of the exact world bounds of the posed geometry (skinning included). The tracked point is smoothed per frame: `target += (point - target) * (1 - exp(-dtS / smoothingS))`, `dtS` = animation time elapsed since the previous frame (`|dt * rate|` of the frame clock), `smoothingS` = option `followSmoothingS` (default 0.25 s, 0 snaps). The first frame, a seek, a new animation or pose, a new document and any change of `follow` / `rootMotion` snap onto the point; a repeated render at the same time does not move the target. While following, the frame looks at the smoothed target and `camera.target` of the request is ignored (panning has no lasting effect); `getCamera()` returns the live camera whose `target` array is the follow state, updated in place (no allocation per frame); `follow: "off"` afterwards keeps the last target as a copy. Unknown node or empty scene: nothing to track, the camera is left alone.
- **`rootMotion`**: `"travel"` keeps the pose as sampled. `"in-place"` subtracts, from the root node and every node below it, `(dx, dz)` = horizontal (x and z, y is up) change of the root node's local translation against its rest translation at the current time, so the figure stays at its rest horizontal position while y keeps moving. Root node = option `rootNode` if it has no parent, else the first node without parent. In-place applies to poses and clips alike; follow, bounds, picking and rasterising see the in-place pose.
- **Controller**: options `follow`, `followSmoothingS`, `rootMotion`, `rootNode`; `setFollow(follow)`, `setRootMotion(mode)`, getters `follow`, `rootMotion`. Events: `follow { follow }`, `rootMotion { rootMotion }`, `camera` (also whenever the followed target moved), `preset { preset }` (null when the camera is set freely).
- **Element**: attributes `follow` (`off` | `bounds` | `node:<id>`) and `root-motion` (`travel` | `in-place`); events `semio-follow`, `semio-rootMotion`, `semio-preset`; style via an adopted constructable sheet.
- **Tests**: `V/🧪️tests/🎥️follow` + fixture `V/🧫️fixtures/🎥️follow` (a walker whose root goes from [1,0,1] to [5,0.5,3] in 2 s): three `VectorKeyframeTrack` interpolant for the root translation, `Matrix4` world positions, `Box3` bounds centre, `Vector3.lerp` with the exponential alpha for the smoothing, determinism, no new target array, controller options/setters/events and element attributes (jsdom).
