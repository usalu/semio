use super::encode::{encode_focus, encode_key, encode_keypad, encode_mouse, encode_paste, encode_wheel_as_arrows, InputModes, KeypadKey, MouseEncoding, MouseReporting};
use super::pane::{SelectionMode, TerminalState};
use super::{CellPoint, Palette, VtScreen};
use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::theme::Theme;
use crate::tui::widget::{CursorShape, WidgetSignal};
use ui_styling::appearance::AppearanceName;

include!("🧫️vt-shared-fixtures-and-corpus-laws/🦀️.rs");
include!("⌨️vt-key-mouse-paste-and-focus-encoding-laws/🦀️.rs");
include!("📟️vt-screen-sequence-laws/🦀️.rs");
include!("📜️vt-history-reflow-and-selection-text-laws/🦀️.rs");
include!("🖥️vt-pane-input-search-and-browse-laws/🦀️.rs");
include!("🖱️vt-pane-pointer-selection-and-paint-laws/🦀️.rs");
include!("🎲️vt-robustness-law/🦀️.rs");
