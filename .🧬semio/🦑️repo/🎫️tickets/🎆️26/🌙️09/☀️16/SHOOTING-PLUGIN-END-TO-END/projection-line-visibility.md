# SVG Projection and Line Visibility

The native wgpu and browser React icon hosts project the camera's saved position, target, up vector, zoom, and perspective field of view into centered SVG coordinates. The native SVG and World3d preview and raster paths now resolve the same camera pose. When fit is enabled, perspective framing uses the zoom-adjusted field of view and orthographic framing keeps the fitted mesh clear of the near plane.

SVG linework clips to the camera frustum before the perspective divide. A screen-space spatial index supplies candidate opaque faces for each edge. Each candidate limits the edge to its projected triangle and the portion farther from the camera than the face. The exporters union those intervals and emit the visible and hidden spans to their respective layers. The fill controls affect paint independently of mesh occlusion. The native exporter steps scan, publication, and retirement incrementally. The browser renderer yields during face collection, edge projection, and large occluder scans and accepts the render port's abort signal. It materializes instanced geometry and active morph and skin positions before painting SVG fills and edges. The native export consumes the native decoder's static mesh geometry.

The shared `line-visibility` fixture exercises cameras with a translated target and rolled up vector, partially occluded edges, overlapping occluders, depth crossings, point contact, coplanar edges, and fully hidden edges. A separate fixture scene exercises the exported layer paths with fill disabled. The browser suite uses Three.js cameras and ray casting as independent oracles and checks actual SVG groups in jsdom. The targeted command ran ten tests with 6,186 assertions. It checks homogeneous clipping and verifies browser instance and morph materialization.

```text
repo tools exec bun --light -- test './🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧪️tests/🕶️line-visibility/🟦️.ts'
```

At the time of this report, repository-wide React typechecking and the native icon host test were still waiting for the shared build scheduler. Browser geometry tests exercised the actual SVG layering helper directly.
