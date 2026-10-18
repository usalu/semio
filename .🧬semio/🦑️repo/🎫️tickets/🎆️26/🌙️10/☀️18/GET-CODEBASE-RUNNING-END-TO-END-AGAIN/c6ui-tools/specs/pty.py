from specs.lib import *
F = U + '/⌨️tui/🚇️pty/🦀️.rs'
CH = [
    ch('🔔', 'console-events-and-spawn', 'console_events_and_spawn', [(14, 125)],
       'Windows console events and spawn: the pseudo-console constants, the console control handler and Ctrl+Break interrupts, the detached spawn and the shared pipe state.'),
    ch('🧾', 'pty-handle-and-command-line', 'pty_handle_and_command_line', [(127, 257)],
       'Windows pty handle and command line: the `Pty` handle, the wide string and batch file helpers, the command line and environment block builders and the program resolver.'),
    ch('🏃', 'pty-process-impl', 'pty_process_impl', [(259, 604)],
       'Windows pty process: the `Pty` spawn, reader, writer, resize, job object and exit handling with its write and drop implementations.'),
]
SPEC = dict(
    orig='pty.rs', file=F, nest=False, children=[], expect_missing=['mod windows_impl {', '}'],
    external=mounted(F, orig='pty.rs'),
    inner=dict(mod='windows_impl', indent=4, folder='🪟️windows-impl', nest=False, rebase_super=True,
               doc='//! 🪟️ Windows pty module: the console events and spawn, the handle with its command line builders and the process implementation, with the shared imports of the former inline module. See [`super`].',
               children=CH),
)
