from specs.lib import *
F = U + '/🖥️host/🪟️window/🦀️.rs'
H = '    impl<P: BrowserHostPort, D: BrowserWindowDelegate> CanvasHost<P, D> {'
B = [906, 1104]
TOP = [
    ch('🖱', 'metrics-cursor-and-clipboard', 'metrics_cursor_and_clipboard', [(28, 256)],
       'Window metrics, cursor and clipboard: the window metrics, the redraw gating, the cursor icons and css, the clipboard host, the native clipboard job and the browser clipboard.'),
    ch('📁', 'file-dialog-ime-and-window-delegate', 'file_dialog_ime_and_window_delegate', [(258, 377)],
       'File dialog, ime and window delegate: the native file dialog requests, the ime directives, the redraw outcome and the `WindowDelegate`.'),
]
INNER = [
    ch('🔌', 'browser-host-port-and-types', 'browser_host_port_and_types', [(4, 103)],
       'Browser host port and types: the browser host byte limits and operations, the host port, the unavailable and error enums, the clipboard request, the scheduler envelope, the progress and step records and the browser window delegate.'),
    ch('🎞', 'canvas-host-lifecycle', 'canvas_host_lifecycle', [(105, 240)],
       'Canvas host lifecycle: the `CanvasHost` state, its construction, wake and clipboard requests, the close start and the delegate access.'),
    ch('⏩', 'canvas-host-step-and-events', 'canvas_host_step_and_events', [(242, 441)],
       'Canvas host step and events: the host step, the message and reply acceptance and the event classification.'),
    ch('🔚', 'canvas-host-staging-and-close', 'canvas_host_staging_and_close', [(443, 612)],
       'Canvas host staging and close: the event acknowledgement, the cursor, clipboard and request staging and the close step.'),
    ch('🧰', 'browser-host-helpers-and-linear-memory-port', 'browser_host_helpers_and_linear_memory_port', [(614, 792)],
       'Browser host helpers and linear memory port: the request, payload and listener helpers, the budget and progress helpers and the linear memory browser host port.'),
]
SPEC = dict(
    orig='window.rs', file=F, children=TOP, wiring='block', pre=split_impl(H, B, ind='    '), expect_missing=['mod browser {', '}'], expect_extra=split_extra(H, B),
    external=mounted(F, orig='window.rs'),
    patch={'🌐️browser-host/🧰️browser-host-helpers-and-linear-memory-port': [('include!("../🧪️tests/🔬️window-browser-linear-memory-test-import/🦀️.rs");', 'include!("../../../🧪️tests/🔬️window-browser-linear-memory-test-import/🦀️.rs");')]},
    inner=dict(mod='browser', indent=4, folder='🌐️browser-host', nest=False, rebase_super=True,
               doc='//! 🌐️ Browser host module: the host port and types, the canvas host lifecycle, step, staging and close and the helpers with the linear memory port, with the shared imports of the former inline module. See [`super`].',
               children=INNER),
)
