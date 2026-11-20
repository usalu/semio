# Compound PDF Path Clips

The PDF 1.7 page scene preserves clipping from `W` and `W*` when the path ends. It retains cubic segments and all subpaths, applies the nonzero or even-odd rule, and intersects nested clips. Graphics-state restoration removes clips introduced inside `q` and `Q`. Existing page rectangles and transformed Form XObject bounds share the same clipping stack. Pointer hit testing checks the effective path clips before selecting an object.

Canvas2D clips the actual Path2D in scene coordinates before the layer transform. The native painter reuses its path flattener and fill-rule tessellator, produces disjoint scissor spans for the resulting clip, and intersects the spans with the existing rectangle and oriented quadrilateral clips. Disjoint spans preserve holes within a single row group.

## Verification

- The shared JSON fixture contains PDF content operators, corresponding scene layers, and ten color and selection probes. It covers cubic boundaries, a same-winding hole under even-odd, the same interior filled under nonzero, nested intersection, and two levels of graphics-state restoration.
- `📜️path-clips-oracle.py` created a PDF from the shared operators. PyMuPDF independently rendered the page and matched all ten color probes. The Chromium harness `📜️path-clips-chromium.ts` bundled and ran the production canvas painter on the fixture’s scene layers, matched all ten probes, and produced the same visible holes, intersections, and outer silhouette in a visual comparison with MuPDF.
- The isolated Rust harness `📜️path-clips-native.py` compiled the production native flattening, fill tessellation and scanline clipping modules. All eight tests passed, including the ten-probe comparison for the shared fixture. Canvas2D paint Vitest passed 14 tests using the ticket’s `🎚️canvas-paint.ts` config.
- The editor fixture asserts scene layer geometry, clip wire values and hit targets. Full editor compilation and native GPU integration require the repo package checks. The isolated native check exercises geometry and span generation.

## Rendering limits

The native clip mask uses logical pixel centers and binary scissors, so its edge antialiasing differs from Chromium and MuPDF. Hit testing uses flattened contours. PDF text rendering modes that add glyph outlines to the clipping path remain open.
