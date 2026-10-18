from specs.lib import *
F = U + '/⌨️tui/🔌️backend/🦀️.rs'
INC_UNIX = 'include!("../🧪️tests/🔬️backend-native-unix-unit/🦀️.rs");'
INC_WIN = 'include!("../🧪️tests/🔬️backend-native-windows-unit/🦀️.rs");'
TOP = [
    ch('📋', 'clipboard-and-osc52', 'clipboard_and_osc52', [(7, 219), (348, 360)],
       'Clipboard and osc52: the backend error, the clipboard trait and results, the base64 and osc52 copy sequences, the native copy and paste workers, the host and memory clipboards and the osc52 payload check.'),
    ch('🔎', 'capabilities-and-frame-presenter', 'capabilities_and_frame_presenter', [(225, 342)],
       'Capabilities and frame presenter: the colour depth and unicode level, the capabilities, the waker, the terminal backend trait, the capability detection and the frame presenter.'),
]
UNIX = [
    ch('🌬', 'wake-pipe-and-signals', 'wake_pipe_and_signals', [(12, 230)],
       'Unix wake pipe and signals: the read burst, the mode restoration, the wake pipe, the saved termios and restore arming, the resize and terminate handlers, the signal guard and the readiness waits.'),
    ch('🖥', 'unix-native-terminal', 'unix_native_terminal', [(232, 477)],
       'Unix native terminal: the `NativeTerminal` with its setup, leave, size and event reads, the terminal backend implementation and the drop.'),
]
WIN = [
    ch('🧰', 'wake-event-and-console-state', 'wake_event_and_console_state', [(13, 160)],
       'Windows wake event and console state: the record batch and size poll, the wake event, the wait outcome, the utf16 decoder, the input record conversion and the saved console restore.'),
    ch('🎚', 'native-terminal-and-console-modes', 'native_terminal_and_console_modes', [(162, 310)],
       'Windows native terminal and console modes: the `NativeTerminal` state, the console mode ownership and the `NativeTerminal` setup and leave.'),
    ch('📟', 'terminal-backend-and-drop', 'terminal_backend_and_drop', [(312, 502)],
       'Windows terminal backend and drop: the terminal backend implementation, the remaining `NativeTerminal` operations, the unit tests include and the drop.'),
]
SPEC = dict(
    orig='backend.rs', file=F, children=TOP, wiring='block',
    expect_missing=['mod native_unix {', 'mod native_windows {', '}', '}'],
    external=mounted(F, orig='backend.rs'),
    patch={
        '🐧️native-unix/🖥️unix-native-terminal': [(INC_UNIX, INC_UNIX.replace('"../', '"../../../'))],
        '🪟️native-windows/📟️terminal-backend-and-drop': [(INC_WIN, INC_WIN.replace('"../', '"../../../'))],
    },
    inners=[
        dict(mod='native_unix', indent=4, folder='🐧️native-unix', nest=True, rebase_super=True,
             doc='//! 🐧️ Unix native terminal module: the wake pipe and signals and the native terminal, with the shared imports of the former inline module. See [`super`].', children=UNIX),
        dict(mod='native_windows', indent=4, folder='🪟️native-windows', nest=True, rebase_super=True,
             doc='//! 🪟️ Windows native terminal module: the wake event and console state, the console modes and the terminal backend, with the shared imports of the former inline module. See [`super`].', children=WIN),
    ],
)
