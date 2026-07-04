//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::theme::Rect;
use semio_framework_core::CommandDescriptor;

#[derive(Clone, Debug)]
pub struct HitTarget {
    pub rect: Rect,
    pub command: Option<CommandDescriptor>,
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
    Generic,
}

pub struct InputState {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub hovered_id: Option<String>,
    pub focused_id: Option<String>,
    pub text_buffer: String,
    pub hit_targets: Vec<HitTarget>,
    pub pending_commands: Vec<CommandDescriptor>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            pointer_x: 0.0,
            pointer_y: 0.0,
            pointer_down: false,
            hovered_id: None,
            focused_id: None,
            text_buffer: String::new(),
            hit_targets: Vec::new(),
            pending_commands: Vec::new(),
        }
    }
}

impl InputState {
    pub fn clear_frame(&mut self) {
        self.hit_targets.clear();
    }

    pub fn register_hit(&mut self, target: HitTarget) {
        self.hit_targets.push(target);
    }

    pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget> {
        self.hit_targets
            .iter()
            .rev()
            .find(|target| target.rect.contains(x, y))
    }

    pub fn drain_commands(&mut self) -> Vec<CommandDescriptor> {
        std::mem::take(&mut self.pending_commands)
    }

    pub fn queue_command(&mut self, command: CommandDescriptor) {
        self.pending_commands.push(command);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_dom_listeners(
    canvas: &web_sys::HtmlCanvasElement,
    on_pointer: impl Fn(f32, f32, bool) + 'static,
    on_key: impl Fn(String) + 'static,
) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{Event, KeyboardEvent, MouseEvent};

    let canvas_clone = canvas.clone();
    let pointer_down = std::rc::Rc::new(std::cell::Cell::new(false));
    let pointer_down_move = pointer_down.clone();

    let move_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        let rect = canvas_clone.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_pointer(x, y, pointer_down_move.get());
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mousemove", move_cb.as_ref().unchecked_ref())
        .ok();
    move_cb.forget();

    let canvas_down = canvas.clone();
    let pointer_down_down = pointer_down.clone();
    let down_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        pointer_down_down.set(true);
        let rect = canvas_down.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_pointer(x, y, true);
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mousedown", down_cb.as_ref().unchecked_ref())
        .ok();
    down_cb.forget();

    let canvas_up = canvas.clone();
    let pointer_down_up = pointer_down.clone();
    let up_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        pointer_down_up.set(false);
        let rect = canvas_up.get_bounding_client_rect();
        let x = (event.client_x() as f32 - rect.left() as f32) * device_pixel_ratio();
        let y = (event.client_y() as f32 - rect.top() as f32) * device_pixel_ratio();
        on_pointer(x, y, false);
    }) as Box<dyn FnMut(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mouseup", up_cb.as_ref().unchecked_ref())
        .ok();
    up_cb.forget();

    let key_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if let Some(key) = event.key().chars().next() {
            on_key(key.to_string());
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);
    canvas.set_tab_index(0);
    canvas
        .add_event_listener_with_callback("keydown", key_cb.as_ref().unchecked_ref())
        .ok();
    key_cb.forget();
}

#[cfg(target_arch = "wasm32")]
fn device_pixel_ratio() -> f32 {
    web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0)
}
