from specs.lib import *

WASM = '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]'
CH = [
    ch('📄', 'document-json', 'document_json', [(38, 236)],
       'Document records of surface paint: the layer, transform, mask and document wire records, the layer node and mask state and the document parser.'),
    ch('⚠', 'errors-and-pixels', 'errors_and_pixels', [(240, 345)],
       'Errors and pixels of surface paint: the paint error with its conversions and the checkerboard, image and test-only brightness and blur pixel helpers.'),
    ch('🖥', 'raster-host', 'raster_host', [(349, 705)],
       'Raster host of surface paint: the layer buffers, the raster host record and its compositing, painting and document operations.'),
    ch('🎯', 'picking', 'picking', [(709, 963)],
       'Picking of surface paint: the screen rectangle, the pick and query wire records and the raster host hit-testing and marquee operations.', reexport=False),
    ch('🧹', 'host-retirement', 'host_retirement', [(967, 1054)],
       'Host retirement of surface paint: the raster host retirement custody with its drop guard.'),
    ch('🌐', 'wasm-session', 'wasm_session', [(1072, 1276)],
       'Browser session of surface paint: the canvas-bound raster session and its inner state, compiled only for the browser target.', cfg_mod=WASM),
]

SPEC = dict(
    orig='pnt.rs',
    file=PNT + '/🦀️.rs',
    children=CH,
    external=mounted(PNT + '/🦀️.rs'),
    patch_parent=[('mod picking;', 'mod picking;\n#[cfg(test)]\nuse picking::*;')],
)
