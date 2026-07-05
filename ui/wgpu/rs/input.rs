//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::geometry::Rect;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct HitTarget<E> {
    pub rect: Rect,
    pub event: Option<E>,
    pub control_id: Option<String>,
    pub kind: HitKind,
    pub drag_axis: Option<DragAxis>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAxis {
    Horizontal,
    Vertical,
    Both,
    Ring,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HitKind {
    Button,
    Toggle,
    Input,
    Select,
    Slider,
    TreeItem,
    PanelTab,
    NavbarItem,
    Window,
    World3d,
    PanelResize,
    ScrollRegion,
    ContextMenu,
    DropdownItem,
    Generic,
}

#[derive(Clone, Debug, Default)]
pub struct PointerModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Clone, Debug)]
pub struct DragState {
    pub active: bool,
    pub button: i16,
    pub start_x: f32,
    pub start_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub target_id: Option<String>,
    pub axis: Option<DragAxis>,
    pub points: Vec<[f32; 2]>,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            active: false,
            button: 0,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            target_id: None,
            axis: None,
            points: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyAction {
    Char(String),
    Backspace,
    Delete,
    Enter,
    Escape,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Tab,
}

pub struct InputState<E> {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_button: i16,
    pub wheel_delta: f32,
    pub modifiers: PointerModifiers,
    pub drag: DragState,
    pub hovered_id: Option<String>,
    pub focused_id: Option<String>,
    pub text_buffer: String,
    pub cursor_pos: usize,
    pub hit_targets: Vec<HitTarget<E>>,
    pub pending_events: Vec<E>,
    pub pending_keys: Vec<KeyAction>,
    pub right_click_pos: Option<(f32, f32)>,
}

impl<E> Default for InputState<E> {
    fn default() -> Self {
        Self {
            pointer_x: 0.0,
            pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            wheel_delta: 0.0,
            modifiers: PointerModifiers::default(),
            drag: DragState::default(),
            hovered_id: None,
            focused_id: None,
            text_buffer: String::new(),
            cursor_pos: 0,
            hit_targets: Vec::new(),
            pending_events: Vec::new(),
            pending_keys: Vec::new(),
            right_click_pos: None,
        }
    }
}

impl<E: Clone> InputState<E> {
    pub fn clear_frame(&mut self) {
        self.hit_targets.clear();
        self.wheel_delta = 0.0;
        self.right_click_pos = None;
    }

    pub fn register_hit(&mut self, target: HitTarget<E>) {
        self.hit_targets.push(target);
    }

    pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget<E>> {
        self.hit_targets
            .iter()
            .rev()
            .find(|target| target.rect.contains(x, y))
    }

    pub fn update_hover(&mut self, x: f32, y: f32) {
        self.pointer_x = x;
        self.pointer_y = y;
        self.hovered_id = self
            .hit_at(x, y)
            .and_then(|hit| hit.control_id.clone());
    }

    pub fn begin_drag(&mut self, x: f32, y: f32, button: i16, target_id: Option<String>, axis: Option<DragAxis>) {
        self.drag = DragState {
            active: true,
            button,
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            target_id,
            axis,
            points: vec![[x, y]],
        };
    }

    pub fn update_drag(&mut self, x: f32, y: f32) {
        if self.drag.active {
            self.drag.current_x = x;
            self.drag.current_y = y;
            self.drag.points.push([x, y]);
        }
    }

    pub fn end_drag(&mut self) -> DragState {
        let drag = self.drag.clone();
        self.drag = DragState::default();
        drag
    }

    pub fn drain_events(&mut self) -> Vec<E> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn drain_keys(&mut self) -> Vec<KeyAction> {
        std::mem::take(&mut self.pending_keys)
    }

    pub fn queue_event(&mut self, event: E) {
        self.pending_events.push(event);
    }

    pub fn queue_key(&mut self, action: KeyAction) {
        self.pending_keys.push(action);
    }

    pub fn focus_input(&mut self, id: &str, value: &str) {
        self.focused_id = Some(id.to_string());
        self.text_buffer = value.to_string();
        self.cursor_pos = value.len();
    }

    pub fn blur_input(&mut self) {
        self.focused_id = None;
        self.text_buffer.clear();
        self.cursor_pos = 0;
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_pos <= self.text_buffer.len() {
            self.text_buffer.insert(self.cursor_pos, ch);
            self.cursor_pos += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.text_buffer.remove(self.cursor_pos);
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor_pos < self.text_buffer.len() {
            self.text_buffer.remove(self.cursor_pos);
        }
    }

    pub fn move_cursor(&mut self, delta: i32) {
        let len = self.text_buffer.len() as i32;
        self.cursor_pos = ((self.cursor_pos as i32) + delta).clamp(0, len) as usize;
    }
}

#[cfg(target_arch = "wasm32")]
pub struct PointerCallbacks {
    pub on_move: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_button: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_wheel: Rc<dyn Fn(f32, f32, f32, PointerModifiers)>,
    pub on_key: Rc<dyn Fn(KeyAction, PointerModifiers)>,
    pub on_context_menu: Rc<dyn Fn(f32, f32)>,
}

#[cfg(target_arch = "wasm32")]
pub fn attach_dom_listeners(canvas: &web_sys::HtmlCanvasElement, callbacks: PointerCallbacks) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::MouseEvent;

    let canvas_clone = canvas.clone();
    let pointer_down = Rc::new(std::cell::Cell::new(false));
    let pointer_button = Rc::new(std::cell::Cell::new(0i16));
    let pointer_down_move = pointer_down.clone();
    let pointer_button_move = pointer_button.clone();
    let on_move = callbacks.on_move.clone();

    let move_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        let rect = canvas_clone.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_move(
            x,
            y,
            pointer_down_move.get(),
            pointer_button_move.get(),
            modifiers_from_event(&event),
        );
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mousemove", move_cb.as_ref().unchecked_ref())
        .ok();
    move_cb.forget();

    let canvas_down = canvas.clone();
    let pointer_down_down = pointer_down.clone();
    let pointer_button_down = pointer_button.clone();
    let on_button_down = callbacks.on_button.clone();
    let down_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        pointer_down_down.set(true);
        pointer_button_down.set(event.button());
        let rect = canvas_down.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_button_down(x, y, true, event.button(), modifiers_from_event(&event));
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mousedown", down_cb.as_ref().unchecked_ref())
        .ok();
    down_cb.forget();

    let canvas_up = canvas.clone();
    let pointer_down_up = pointer_down.clone();
    let pointer_button_up = pointer_button.clone();
    let on_button_up = callbacks.on_button;
    let up_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        pointer_down_up.set(false);
        let rect = canvas_up.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_button_up(x, y, false, pointer_button_up.get(), modifiers_from_event(&event));
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mouseup", up_cb.as_ref().unchecked_ref())
        .ok();
    up_cb.forget();

    let canvas_wheel = canvas.clone();
    let on_wheel = callbacks.on_wheel;
    let wheel_cb = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        event.prevent_default();
        let rect = canvas_wheel.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_wheel(
            event.delta_y() as f32,
            x,
            y,
            PointerModifiers {
                shift: event.shift_key(),
                ctrl: event.ctrl_key(),
                alt: event.alt_key(),
                meta: event.meta_key(),
            },
        );
    }) as Box<dyn FnMut(web_sys::WheelEvent)>);
    canvas
        .add_event_listener_with_callback("wheel", wheel_cb.as_ref().unchecked_ref())
        .ok();
    wheel_cb.forget();

    let on_key = callbacks.on_key;
    let key_cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mods = PointerModifiers {
            shift: event.shift_key(),
            ctrl: event.ctrl_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        };
        let action = match event.key().as_str() {
            "Backspace" => KeyAction::Backspace,
            "Delete" => KeyAction::Delete,
            "Enter" => KeyAction::Enter,
            "Escape" => KeyAction::Escape,
            "ArrowLeft" => KeyAction::ArrowLeft,
            "ArrowRight" => KeyAction::ArrowRight,
            "ArrowUp" => KeyAction::ArrowUp,
            "ArrowDown" => KeyAction::ArrowDown,
            "Tab" => KeyAction::Tab,
            key if key.len() == 1 => KeyAction::Char(key.to_string()),
            _ => return,
        };
        if !matches!(action, KeyAction::Char(_)) {
            event.prevent_default();
        }
        on_key(action, mods);
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    canvas.set_tab_index(0);
    canvas
        .add_event_listener_with_callback("keydown", key_cb.as_ref().unchecked_ref())
        .ok();
    key_cb.forget();

    let canvas_ctx = canvas.clone();
    let on_context_menu = callbacks.on_context_menu;
    let ctx_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        event.prevent_default();
        let rect = canvas_ctx.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_context_menu(x, y);
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("contextmenu", ctx_cb.as_ref().unchecked_ref())
        .ok();
    ctx_cb.forget();
}

#[cfg(target_arch = "wasm32")]
fn modifiers_from_event(event: &web_sys::MouseEvent) -> PointerModifiers {
    PointerModifiers {
        shift: event.shift_key(),
        ctrl: event.ctrl_key(),
        alt: event.alt_key(),
        meta: event.meta_key(),
    }
}

#[cfg(target_arch = "wasm32")]
fn device_pixel_ratio() -> f32 {
    web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0)
}
