from specs.lib import *

HOST = '#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]'
CH = [
    ch('🔧', 'kurbo-boundary', 'kurbo_boundary', [(48, 110)],
       'Kurbo boundary of the scene: the conversions from the first-party geometry to the kurbo affine, point, path element and shape, compiled only for the host and browser targets.', cfg_mod=HOST),
    ch('🖊', 'stroke-style', 'stroke_style', [(112, 205)],
       'Stroke style of the scene: the cap, the join and the stroke record with its peniko conversions.'),
    ch('🎨', 'paint-values', 'paint_values', [(207, 382)],
       'Paint values of the scene: the colour, the fill rule, the blend mode, the paint and the raster image with their peniko conversions.'),
    ch('📦', 'shape-and-command', 'shape_and_command', [(392, 628)],
       'Shape and command of the scene: the recorded shape and the scene command with its replay into a vello scene.'),
    ch('🎬', 'scene-and-retirement', 'scene_and_retirement', [(630, 890), (915, 941)],
       'Scene and retirement: the recorded scene, the opaque scene retirement registry and its entry points, the canvas scene retirement custody and the parsed SVG document.'),
]

SPEC = dict(
    orig='rnd.rs',
    file=RND + '/🦀️.rs',
    children=CH,
    external=[M + '/🖼️canvas/🧪️tests/🔬️renderer-color-stroke-raster/🦀️.rs', M + '/🖼️canvas/🧪️tests/🔬️renderer-opaque-scene-retirement/🦀️.rs'],
    external_ranges=[(893, 1205)],
    expect_missing=['pub mod draw_list {', '}', 'fn vello_scene(&self) -> backend::Scene {'],
    expect_extra=['pub(crate) fn vello_scene(&self) -> backend::Scene {'],
    inner=dict(mod='draw_list', indent=4, folder='🗒' + V + 'draw-list',
               doc='//! 🗒' + V + ' Draw list of the scene: the portable replay encoding of a scene command list as numbers a 2D canvas context can replay verb for verb. See [`super`].',
               children=[]),
)
