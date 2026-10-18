from specs.lib import *
F = U + '/⌨️tui/🔡️ansi/🦀️.rs'
H = 'impl AnsiParser {'
B = [739]
CH = [
    ch('📤', 'sgr-emit-and-session-modes', 'sgr_emit_and_session_modes', [(9, 123)],
       'Sgr emit and session modes: the ansi patch, the sgr state and run emission, the session mode table and the setup and teardown sequences.'),
    ch('🌈', 'cursor-emitter-and-colour-quantizing', 'cursor_emitter_and_colour_quantizing', [(127, 332)],
       'Cursor emitter and colour quantizing: the cursor emitter with its style codes, the colour cube and palette conversions and the sgr quantizing.'),
    ch('🔤', 'parser-state-and-feeding', 'parser_state_and_feeding', [(339, 739)],
       'Parser state and feeding: the paste and timeout limits, the parser state, the key tables and the `AnsiParser` constructor, byte feeding, escape and ss3 handling.'),
    ch('🔣', 'csi-mouse-and-paste-decoding', 'csi_mouse_and_paste_decoding', [(741, 994)],
       'Csi, mouse and paste decoding: the csi feeding and finishing, the tilde and x10 sequences, mouse decoding and the bracketed paste.'),
    ch('🖱', 'click-counter', 'click_counter', [(998, 1031)],
       'Click counter: the multi-click window and the `ClickCounter`.'),
]
SPEC = dict(orig='ansi.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='ansi.rs'))
