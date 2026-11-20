# PDF page viewport

The PDF 1.7 page editor calculates visible page bounds from the intersection of the CropBox and MediaBox and uses MediaBox when CropBox is absent. UserUnit scales page dimensions, object positions and PDF strokes. Rotate changes the origin and axes before page stacking. A first draft is placed inside the effective bounds. [ISO 32000-1 §14.11.2](https://udp.adobe.io/document-services/docs/assets/35e4369068f86065372c18787171a17e/PDF_ISO_32000-1.pdf)

The page supplies a scene-space clip to all of its paint layers, including images, selections and nested forms. The browser scopes the clip before the node transform. The native renderer converts it through the camera and scopes its draw-list scissor. Pointer hit tests apply the same page rectangle, so cropped marks cannot select themselves through neighboring pages.

The language-neutral fixture checks all four rotations, a nonzero origin, double-size UserUnit, a partly cropped red path and a wholly cropped blue path, and a second page that must not receive the first page’s cropped content or hits. PyMuPDF 1.24.14 independently matched all four viewport sizes and raster probes for the conforming CropBox. The Rust test also exercises an oversized CropBox against the standard’s MediaBox intersection rule. The pinned MuPDF binding reported the oversized crop directly, so that variant is checked against the specification rather than that oracle.

All 12 browser Canvas2d paint tests passed, including clipping order and scope restoration. Native editor and wgpu runtime verification were pending when this report was written.
