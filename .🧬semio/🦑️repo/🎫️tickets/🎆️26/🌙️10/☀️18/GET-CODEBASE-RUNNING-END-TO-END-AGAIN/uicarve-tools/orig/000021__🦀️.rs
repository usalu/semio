//! 🪟️ Windows native terminal module: the wake event and console state, the console modes and the terminal backend, with the shared imports of the former inline module. See [`super`].

use super::native_common::*;
use super::*;
use crate::tui::ansi::{setup_sequence, teardown_sequence};
use crate::tui::component::windows_abi::{
    CreateEventW, GetConsoleCP, GetConsoleMode, GetConsoleOutputCP, GetConsoleScreenBufferInfo, GetNumberOfConsoleInputEvents, OwnedHandle, ReadConsoleInputW, SetConsoleCP, SetConsoleCtrlHandler, SetConsoleMode, SetConsoleOutputCP, SetEvent, WaitForMultipleObjects, WriteFile,
    CONSOLE_SCREEN_BUFFER_INFO, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT, ENABLE_EXTENDED_FLAGS, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_QUICK_EDIT_MODE, ENABLE_VIRTUAL_TERMINAL_INPUT,
    ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_WINDOW_INPUT, HANDLE, INFINITE, INPUT_RECORD, KEY_EVENT, VK_MENU, VK_SPACE, VK_2, CONTROL_KEYS, WAIT_OBJECT_0, WAIT_TIMEOUT, WINDOW_BUFFER_SIZE_EVENT,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[path = "🧰️wake-event-and-console-state/🦀️.rs"]
mod wake_event_and_console_state;
use wake_event_and_console_state::{
    arm_restore, disarm_restore, err, on_console_control, restore_for_panic, SavedConsole,
    WakeEvent, WakeState, RECORD_BATCH, SIZE_POLL,
};
pub(super) use wake_event_and_console_state::{
    records_to_bytes, wait_signalled, Utf16Decoder, WaitOutcome,
};

#[path = "🎚️native-terminal-and-console-modes/🦀️.rs"]
mod native_terminal_and_console_modes;
pub use native_terminal_and_console_modes::NativeTerminal;
#[cfg(test)]
use native_terminal_and_console_modes::ConsoleModeOwnership;

#[path = "📟️terminal-backend-and-drop/🦀️.rs"]
mod terminal_backend_and_drop;
