from specs.lib import *

WASM = '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]'
CH = [
    ch('🎨', 'canvas-theme', 'canvas_theme', [(27, 95)],
       'Canvas theme of the editor: the colour bridge to the value codec and the editor canvas theme with its defaults.'),
    ch('📐', 'viewport-and-settings-json', 'viewport_and_settings_json', [(99, 235)],
       'Viewport and wire records of the editor: the gutter and padding constants, the screen and world conversions, the settings record and the semantic token, span, range, placeholder, diagnostic and edit records.'),
    ch('🖊', 'host-state', 'host_state', [(237, 357)],
       'Host state of the editor: the editor host record, its retirement custody with the drop and default implementations.'),
    ch('⌨', 'host-impl', 'host_impl', [(359, 1132)],
       'Host implementation of the editor: the single implementation block of the editor host with its text, selection, composition, camera and projection operations.'),
    ch('🔤', 'text-geometry', 'text_geometry', [(1134, 1226)],
       'Text geometry of the editor: the range overlap test, the byte, line and column conversions and the character boundary helpers.'),
    ch('🌐', 'wasm-session', 'wasm_session', [(1243, 1572)],
       'Browser session of the editor: the canvas-bound editor session and its inner state, compiled only for the browser target.', cfg_mod=WASM),
]

SPEC = dict(
    orig='edt.rs',
    file=EDT + '/🦀️.rs',
    children=CH,
    external=mounted(EDT + '/🦀️.rs', EDT + '/📦️packages/🦀️rust/🦀️.rs'),
    wiring='each',
)
