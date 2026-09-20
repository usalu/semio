# Reference Plane Parity Packet

## Measured problem

Checkpoint14 has resident references in both panes, but WGPU paints a gray opaque-looking rectangle and darker walls where React produces a muted composition. This is an independent consumer/material issue; full-resolution decode ownership is assigned to SolChrome.

## Current executable authority

The actual renderer's World3dHost reference record has id, url, origin, widthWorld, locked, hidden and opacity plus marker interaction fields. Its WorldReferenceLayer mapping forwards id, origin, widthWorld, locked and opacity and sets source.mediaKind to image for every record. The lower-level ReferenceMediaPort has SVG/PDF rasterizers, but that does not establish their reachability through this host.

React loadReferenceImageTexture uses THREE.TextureLoader without setting colorSpace. Ordinary image references therefore retain Three's NoColorSpace. Only referenceCanvasTexture, used by the explicit SVG/PDF paths, assigns SRGBColorSpace. A single blanket sRGB reference decode profile would not match this implementation.

The React plane uses MeshBasicMaterial, DoubleSide, transparent opacity from worldReferenceAppearance, depthWrite false and toneMapped false. Its content renderOrder is -10; optional hover/selected background is -11 and outline is -9. Authored opacity defaults to one, locked dimming applies the shared locked opacity multiplier, and selected content opacity is multiplied by 0.5. Hover and selection resolve semantic background/outline colors.

WGPU WorldReferenceRecord currently reads only url, origin, widthWorld and hidden. The draw emits tint [1,1,1,0.85] for every visible plane, omitting authored opacity and locked/selected/hovered appearance. The texture table currently allocates Rgba8UnormSrgb. The reference interaction registry and hit helpers use URL because the record has no separate id, while React selection uses the authored reference id.

## Required bounded execution

1. Extend the canonical neutral reference fixture/schema to cover authored id, opacity, locked/hidden and hover/selected states, with current React host semantics and actual material pixel reference. Verify default width against the exact host/content-bounds implementations rather than assuming a global lower-layer default.
2. Preserve authored reference id through retained publication, interaction registry, hover/context selection and emitted actions. URL remains the resource key.
3. Resolve ordinary image reference transfer exactly like current TextureLoader/Basic material. Keep image and explicitly rasterized SVG/PDF decode profiles distinct. Coordinate identity and pool metadata with SolChrome; do not modify its full-resolution ownership implementation.
4. Replace the unconditional 0.85 tint with authored and interaction-derived opacity; implement background/outline layers and depth/order/culling policies matching the actual React material.
5. Use an actual Three/WebGPU pixel oracle for neutral, authored opacity, locked, hovered and selected cases. Add native normal publication and interaction laws, including distinct authored id versus URL.
6. Rebuild and compare both panes at matched camera, theme, locale and device scale. Pixel-unit agreement is not a substitute for the full application screenshot.

Do not add unsupported pose, page or media-kind APIs solely because the lower-level R3F library exposes them. Reachability from the renderer contract must be demonstrated first.

## Source anchors

- Renderer engine: World3dHost/🟦️.tsx, WorldReferenceRecord and WorldReferenceLayer mapping.
- Infinite world R3F: WorldReferencePlane, worldReferenceAppearance and reference selection callbacks.
- UI React target: loadReferenceImageTexture and referenceCanvasTexture.
- Infinite world Rust: WorldReferenceRecord, reference interaction registry and textured_instances draw.
- UI WGPU draw: raster texture format and textured world pipeline.

This is a source-backed execution packet. No new production reference appearance implementation or runtime acceptance is claimed here.

