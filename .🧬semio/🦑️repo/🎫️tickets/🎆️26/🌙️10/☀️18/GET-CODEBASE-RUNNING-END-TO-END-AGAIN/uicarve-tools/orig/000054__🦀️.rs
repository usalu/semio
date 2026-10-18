
use crate::tui::ansi::{AnsiParser, AnsiPatch, emit_runs, setup_sequence, teardown_sequence};
use crate::tui::cell::{Cell, CellBuffer, DiffRun, attr, diff};
use crate::tui::chrome::{ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState, mount_window_layout, shell, window_chip_layout, window_content_padding};
use crate::tui::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::layout::{Constraint, Dimension, Direction, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, create_default_layout, even_window_layout, solve, solve_window_layout};
use crate::tui::scene::{Node, NodeContent, Scene};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{Align, ChipState, DividerState, InputState, LabelState, ListState, LogScroll, LogState, SelectState, TableAlign, TableColumn, TableRow, TableState, TabsState, TerminalState, WidgetSignal, WidgetState, WizardState};
use ui_styling::appearance::AppearanceName;

include!("🪟️tui-layout-and-window-laws/🦀️.rs");
include!("📊️tui-table-diff-parser-and-theme-laws/🦀️.rs");
include!("📐️tui-geometry-text-cell-and-ansi-laws/🦀️.rs");
include!("📟️tui-vt-scene-and-layout-laws/🦀️.rs");
include!("⌨️tui-widget-key-handling-laws/🦀️.rs");
include!("🎨️tui-widget-paint-and-window-silhouette-laws/🦀️.rs");
include!("🚦️tui-chrome-engine-wasm-clipboard-and-pty-laws/🦀️.rs");
