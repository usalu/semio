# SVG Drawing Layers and Linework

The renderer accepts an optional drawing object shared by native export and the React preview and export path. SVG output uses named groups for the background, fills, hidden lines and visible lines in that paint order. The ellipse mask encloses all four groups. A disabled layer contributes no markup, and a supported line layer only appears if it contains geometry.

Fill color overrides shading when supplied. Fill opacity applies to the composited fill group. The source material geometry remains eligible for line occlusion when the fill is disabled or transparent. Visible and hidden line styles have independent colors and widths. A hidden stroke uses a repeating four-width dash and three-width gap.

Line merging joins projected edges at degree-two vertices, preserves branch points, removes reversed duplicates and closes cycles. Easing rounds those same corners with quadratic Bézier curves. The radius is in output pixels, and the corner uses the same trim on its two legs, limited by half the shorter leg. Easing also works with merging switched off, in which case each original edge receives the adjacent halves of its corner curves.

React shot preview and export share the icon render port. A newer preview request cancels the previous one. Cancelling an export aborts the in-flight request as well as withholding delivery. React projection and stroke assembly yield between bounded batches, and native stroke assembly participates in the export lifecycle.

## Preview Lifecycle

The React pane test changes a shot's drawing settings while the previous preview is pending. The old signal aborts, the new request receives the drawing object, and the frame remains transparent when its background layer is off.

The native pane currently paints the shared World3D scene and owns only a borrowed mesh lease. Native SVG export has a separate cancellable owner, retires the decoded model at completion and publishes SVG bytes to the save flow. The reusable native preview owner now drives the same vector job, retires mesh and vector storage, then rasterizes the completed markup on the renderer's existing shared worker. The renderer pane can stage the returned owned RGBA image with the existing Canvas raster upload lane.

## Verification

- Seven neutral drawing fixture cases passed on the browser serializer and an independent Three.js and Sharp oracle. The cases cover group enablement and order, custom line and fill styles, fill opacity, merging, easing and easing without merging.
- Eight Vitest checks passed across SVG serialization, preview status, request schema and forwarding, and the Sharp oracle for capped preview resolution.
- Eleven Bun export lifecycle checks passed, including delivery suppression and abort signal propagation.
- The catalogue native test spent over six minutes waiting for another checkout graph load. It was cancelled and retried with the catalogue's owner-only selection.
- Three isolated Rust checks passed against the actual linework module. They cover closed loops, a short adjacent edge, collinear continuation, branch points and reversed duplicates.
- The owner-only native selection encountered the same project graph wait. The targeted crate test was retried through the repository's shared Cargo ingress with one compiler job.

## Native Preview Raster

The preview preserves export view box and viewport clipping. It caps the longest side at 1024 pixels, fits the caller's RGBA upload byte budget and preserves the aspect ratio within whole-pixel rounding and converts the existing rasterizer's premultiplied output to the texture pipeline's straight-alpha RGBA. Cancellation prevents publication and gives the owner one bounded retirement step at a time. The preview tests reuse the independent Three and Sharp fixture. The Sharp raster oracle passed the 1 MiB cases for square, shallow, landscape and portrait output. An isolated Rust probe passed those same dimension cases plus 294 combinations of source dimensions and byte budgets, including the minimum complete-pixel budget.

## Renderer Source Boundary

The icon host originally exceeded its 4,000-active-line test closure. The standard-library-only projected-linework crate now owns frustum clipping, depth-aware hidden-line classification and connected SVG path assembly. Both the renderer integration and the shared tests use the same methods. The new crate has no production dependency, and its test closure shares the renderer's neutral geometry fixture.

The repository source scanner measured 2,855 active library lines and 3,853 active test lines in the icon host, and 493 library lines and 576 test lines in projected-linework. A standalone Rust harness linked the relocated source and passed five frustum, six visibility and six linework fixture cases. The owner-only Cargo invocation was cancelled while awaiting host capacity after the combined native build was started.

The native preview raster worker wakes its caller upon sending the result, so the pane can wait for the asset and raster completions without scheduling idle redraws.
