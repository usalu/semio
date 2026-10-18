from specs.lib import *
F = FLOW + '/🕸️wasm/🎞️actions-b/🦀️.rs'
CH = [
    ch('⏪', 'undo-redo-camera-and-wheel', 'undo_redo_camera_and_wheel', [(13, 300)],
       'Undo, redo, camera and wheel actions 2561 to 2567: redo, the undo and redo availability, the world from screen point, the camera and the screen wheel.'),
    ch('🔭', 'level-of-detail', 'level_of_detail', [(302, 593)],
       'Level of detail actions 2568 to 2574: the wheel zoom flag, the level of detail scale, the automatic, proximity and forced draw levels and the label overlay paint state.'),
    ch('🖼', 'surface-theme-reorganize-and-render', 'surface_theme_reorganize_and_render', [(595, 876)],
       'Surface, theme, reorganize and render actions 2575 to 2581: the surface attach and status, readiness, size, the canvas theme, the layout reorganize and the frame render.'),
    ch('🎯', 'pointer-and-picking', 'pointer_and_picking', [(878, 1129)],
       'Pointer and picking actions 2582 to 2587: the screen pointer down and move, the pick targets, the entity screen box, the drag probe and the pointer up.'),
    ch('✅', 'selection', 'selection', [(1131, 1568)],
       'Selection actions 2588 to 2598: the selection options and preview, the union bounds, alignment, area select, delete and select all.'),
    ch('📤', 'drawing-export-and-snapshots', 'drawing_export_and_snapshots', [(1570, 2018)],
       'Drawing, export and snapshot actions 2599 to 2610: the scene render, SVG and PDF export, the bitmap trace, boolean segments, the geometry port disposal, the pointer cancel and the host snapshot JSON.'),
]
SPEC = dict(orig='actb.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='actb.rs'))
