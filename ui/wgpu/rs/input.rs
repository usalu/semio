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
    Generic,
}

#[derive(Clone, Debug, Default)]
pub struct PointerModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

#[derive(Clone, Debug)]
pub struct DragState {
    pub active: bool,
    pub button: i16,
    pub start_x: f32,
    pub start_y: f32,
    pub current_x: f32,
    pub current_y: f32,
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
            points: Vec::new(),
        }
    }
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
    pub hit_targets: Vec<HitTarget<E>>,
    pub pending_events: Vec<E>,
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
            hit_targets: Vec::new(),
            pending_events: Vec::new(),
        }
    }
}

impl<E: Clone> InputState<E> {
    pub fn clear_frame(&mut self) {
        self.hit_targets.clear();
        self.wheel_delta = 0.0;
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

    pub fn drain_events(&mut self) -> Vec<E> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn queue_event(&mut self, event: E) {
        self.pending_events.push(event);
    }
}

#[cfg(target_arch = "wasm32")]
pub struct PointerCallbacks {
    pub on_move: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_button: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_wheel: Rc<dyn Fn(f32, PointerModifiers)>,
    pub on_key: Rc<dyn Fn(String)>,
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
        let _ = rect;
        on_wheel(event.delta_y() as f32, PointerModifiers {
            shift: event.shift_key(),
            ctrl: event.ctrl_key(),
            alt: event.alt_key(),
        });
    }) as Box<dyn FnMut(web_sys::WheelEvent)>);
    canvas
        .add_event_listener_with_callback("wheel", wheel_cb.as_ref().unchecked_ref())
        .ok();
    wheel_cb.forget();

    let on_key = callbacks.on_key;
    let key_cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if let Some(key) = event.key().chars().next() {
            on_key(key.to_string());
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    canvas.set_tab_index(0);
    canvas
        .add_event_listener_with_callback("keydown", key_cb.as_ref().unchecked_ref())
        .ok();
    key_cb.forget();
}

#[cfg(target_arch = "wasm32")]
fn modifiers_from_event(event: &web_sys::MouseEvent) -> PointerModifiers {
    PointerModifiers {
        shift: event.shift_key(),
        ctrl: event.ctrl_key(),
        alt: event.alt_key(),
    }
}

#[cfg(target_arch = "wasm32")]
fn device_pixel_ratio() -> f32 {
    web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0)
}
