#![allow(dead_code)]

#[path = "../../../../../../../../🧰️framework/🔨️modules/🌉️abi/🦀️component.rs"]
mod abi;

extern crate self as ui_render;
pub use ui_render_stub::*;

mod ui_render_stub {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CursorRequest {
        Default,
        Pointer,
        Text,
        Grab,
        Grabbing,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PhysicalSize {
        pub width: u32,
        pub height: u32,
    }
    impl PhysicalSize {
        pub const fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct InvalidationReason(u8);
    impl InvalidationReason {
        pub const PAINT: Self = Self(1);
        pub const ANIMATION: Self = Self(2);
    }

    pub struct FrameScheduler {
        dirty: Option<InvalidationReason>,
        deadline: Option<(f64, InvalidationReason)>,
        visible: bool,
    }
    impl FrameScheduler {
        pub const fn new() -> Self {
            Self { dirty: None, deadline: None, visible: true }
        }
        pub fn invalidate(&mut self, reason: InvalidationReason) {
            self.dirty = Some(reason);
        }
        pub fn request_deadline(&mut self, at: f64, reason: InvalidationReason) {
            self.deadline = Some((at, reason));
        }
        pub fn should_render(&mut self, now: f64) -> Option<InvalidationReason> {
            if !self.visible {
                return None;
            }
            self.dirty.take().or_else(|| self.deadline.filter(|(at, _)| *at <= now).map(|(_, reason)| reason).inspect(|_| self.deadline = None))
        }
        pub fn set_visible(&mut self, visible: bool) {
            self.visible = visible;
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rect {
        pub x: f32,
        pub y: f32,
        pub w: f32,
        pub h: f32,
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ImeDirective {
        Enable { cursor_bounds: Rect },
        Disable,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PointerId(pub u64);
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PointerKind {
        Mouse,
        Touch,
        Pen,
        Eraser,
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PointerInfo {
        pub id: PointerId,
        pub kind: PointerKind,
        pub pressure: Option<f32>,
        pub tilt: Option<(f32, f32)>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PointerButton {
        Primary,
        Secondary,
        Middle,
    }
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct EventModifiers {
        pub shift: bool,
        pub ctrl: bool,
        pub alt: bool,
        pub meta: bool,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum ImeEvent {
        Start,
        Update { text: String, cursor: usize },
        Commit { text: String },
        Cancel,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum DispatchEvent {
        PointerDown { pointer: PointerInfo, x: f32, y: f32, button: PointerButton },
        PointerUp { pointer: PointerInfo, x: f32, y: f32, button: PointerButton },
        PointerMove { pointer: PointerInfo, x: f32, y: f32 },
        Scroll { x: f32, y: f32, delta_x: f32, delta_y: f32 },
        KeyDown { key: String, modifiers: EventModifiers },
        KeyUp { key: String, modifiers: EventModifiers },
        TextInput { text: String },
        Paste { text: String },
        Ime(ImeEvent),
    }
}

mod enqueue {
    pub struct UiThreadToken;
    impl UiThreadToken {
        pub(crate) fn mint() -> Self {
            Self
        }
    }
}

#[path = "../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️event.rs"]
mod event;
#[path = "../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs"]
mod window;

fn main() {}
