from specs.lib import *
F = FLOW + '/🖍️drawing/🦀️.rs'
CH = [
    ch('🧭', 'kernel-types-and-path-segments', 'kernel_types_and_path_segments', [(29, 285)],
       'Kernel types and path segments of the drawing kernel: the drawing kind, handle, styles, affine transform, node, scene and the JSON form of path segments.'),
    ch('🧰', 'kernel-trait-and-engine', 'kernel_trait_and_engine', [(289, 434)],
       'Kernel trait and engine of the drawing kernel: the `DrawingKernel` operations, the content-addressed drawing engine, its cache helpers and the stored node.'),
    ch('🗄', 'store-and-geometry', 'store_and_geometry', [(436, 634)],
       'Store and geometry of the drawing kernel: the drawing store with its node operations plus the primitive segment builders and the scene bounds.'),
    ch('🧾', 'svg-and-pdf-export', 'svg_and_pdf_export', [(638, 778)],
       'SVG and PDF export of the drawing kernel: the color, path data, SVG serialization and PDF operations.'),
    ch('🧱', 'kernel-impl', 'kernel_impl', [(782, 931)],
       'Kernel impl of the drawing kernel: the `DrawingKernel` implementation of the drawing store.'),
    ch('🌉', 'json-bridge', 'json_bridge', [(935, 1093)],
       'JSON bridge of the drawing kernel: the shared kernel instance, the kernel error and the JSON entry points of the flow `draw` operator extension.'),
]
SPEC = dict(orig='draw.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='draw.rs'))
