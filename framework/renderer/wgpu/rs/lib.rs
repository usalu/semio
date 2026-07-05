//! 🧊 Raw wgpu WASM renderer for declarative framework UiNode trees.

pub mod interpreter;
pub mod plugin_bridge;
pub mod scenes;
pub mod shell;
pub mod world3d;

use plugin_bridge::{filter_plugins, parse_plugin_entries};
use semio_framework_core::CommandDescriptor;
use shell::ShellState;
use std::cell::RefCell;
use std::rc::Rc;
use ui_wgpu::{
    attach_dom_listeners, fetch_font_bytes, schedule_frame, DrawList, FontAtlas, GpuContext,
    HitKind, InputState, PointerCallbacks, PointerModifiers, Theme,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

struct AppRuntime {
    gpu: GpuContext,
    atlas: FontAtlas,
    shell: ShellState,
    draw: DrawList,
    input: InputState<CommandDescriptor>,
    theme: Theme,
    last_pointer_x: f32,
    last_pointer_y: f32,
    pointer_down: bool,
    pointer_button: i16,
    modifiers: PointerModifiers,
    wheel_delta: f32,
    asset_poll_pending: bool,
    self_weak: std::rc::Weak<RefCell<AppRuntime>>,
}

impl AppRuntime {
    fn frame(&mut self) {
        self.input.clear_frame();
        self.draw.clear();
        let wheel_delta = self.wheel_delta;
        self.wheel_delta = 0.0;
        if wheel_delta.abs() > 0.0 {
            let x = self.last_pointer_x;
            let y = self.last_pointer_y;
            let shell = &mut self.shell;
            for state in shell.world3d_states.values_mut() {
                if state.bounds.inset(8.0).contains(x, y) {
                    world3d::handle_world3d_wheel(state, wheel_delta);
                }
            }
        }
        self.shell.render_chrome(
            &mut self.draw,
            &mut self.atlas,
            &mut self.input,
            &self.theme,
            &mut self.gpu,
        );
        if let Err(err) = self.gpu.render_frame(&self.draw) {
            web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] render frame: {err}")));
        }
        if !self.asset_poll_pending {
            self.asset_poll_pending = true;
            let runtime = self.self_weak.clone();
            spawn_local(async move {
                let Some(runtime) = runtime.upgrade() else {
                    return;
                };
                let pending = {
                    let Ok(app) = runtime.try_borrow() else {
                        return;
                    };
                    world3d::collect_pending_glb_fetches(&app.shell.world3d_states)
                };
                let mut fetched = Vec::new();
                for item in pending {
                    if let Some(bytes) = world3d::fetch_url_bytes(&item.url).await {
                        fetched.push((item.surface_id, item.url, bytes));
                    }
                }
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    for (surface_id, url, bytes) in fetched {
                        if let Some(state) = app.shell.world3d_states.get_mut(&surface_id) {
                            world3d::apply_glb_bytes(state, &url, &bytes);
                        }
                    }
                    app.asset_poll_pending = false;
                };
            });
        }
    }

    fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.gpu.resize(css_width, css_height, dpr);
        self.shell.screen_w = (css_width * dpr).max(1.0);
        self.shell.screen_h = (css_height * dpr).max(1.0);
    }

    async fn dispatch_world3d_commands(&mut self, commands: Vec<CommandDescriptor>) {
        for command in commands {
            if let Err(err) = self.shell.dispatch_command(command).await {
                web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] command failed: {err}")));
            }
        }
    }

    async fn handle_pointer_button(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        modifiers: PointerModifiers,
    ) {
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        let mut world_commands = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.inset(8.0).contains(x, y) {
                continue;
            }
            if let Some(command) = world3d::handle_world3d_pointer_button(
                state,
                x,
                y,
                down,
                button,
                modifiers.shift,
                modifiers.ctrl,
            ) {
                world_commands.push(command);
            }
        }
        if !world_commands.is_empty() {
            self.dispatch_world3d_commands(world_commands).await;
            return;
        }
        if !down {
            return;
        }
        let hit = self.input.hit_at(x, y).cloned();
        let Some(hit) = hit else { return };
        if let Some(command) = hit.event.clone() {
            if let Err(err) = self.shell.dispatch_command(command).await {
                web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] command failed: {err}")));
            }
        }
        if hit.kind == HitKind::Input {
            self.input.focused_id = hit.control_id.clone();
        }
    }

    async fn handle_pointer_move(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        modifiers: PointerModifiers,
    ) {
        let drag_dx = x - self.last_pointer_x;
        let drag_dy = y - self.last_pointer_y;
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        if down && (button == 2 || button == 1) {
            for state in self.shell.world3d_states.values_mut() {
                if state.bounds.inset(8.0).contains(x, y) {
                    world3d::handle_world3d_pointer_drag(state, drag_dx, drag_dy, button, modifiers.shift);
                }
            }
        }
        let mut world_commands = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.inset(8.0).contains(x, y) {
                continue;
            }
            if let Some(command) = world3d::handle_world3d_pointer_move(state, x, y, down, button) {
                world_commands.push(command);
            }
        }
        if !world_commands.is_empty() {
            self.dispatch_world3d_commands(world_commands).await;
        }
    }
}

fn start_frame_loop(runtime: Rc<RefCell<AppRuntime>>) {
    let next = runtime.clone();
    schedule_frame(move || {
        if let Ok(mut app) = next.try_borrow_mut() {
            app.frame();
        }
        start_frame_loop(next.clone());
    });
}

#[wasm_bindgen(js_name = semioRendererBoot)]
pub async fn semio_renderer_boot(
    canvas: web_sys::HtmlCanvasElement,
    plugins: JsValue,
    plugin_filter: String,
) -> Result<(), JsValue> {
    let dpr = web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0);
    let css_width = canvas.client_width().max(1) as f32;
    let css_height = canvas.client_height().max(1) as f32;
    canvas.set_width((css_width * dpr) as u32);
    canvas.set_height((css_height * dpr) as u32);

    let font_bytes = fetch_font_bytes("/asset/font/kelly-slab/latin.ttf")
        .await
        .unwrap_or_default();
    let atlas = FontAtlas::from_bytes(&font_bytes)
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] atlas failed: {err}")))?;

    let mut gpu = GpuContext::from_canvas(canvas.clone(), dpr)
        .await
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] gpu init failed: {err}")))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);

    let entries = parse_plugin_entries(plugins)
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] plugin parse failed: {err}")))?;
    let filtered = filter_plugins(entries, &plugin_filter);
    let mut shell = ShellState::new(filtered, plugin_filter.clone());
    shell.screen_w = css_width * dpr;
    shell.screen_h = css_height * dpr;
    shell
        .boot()
        .await
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] shell boot failed: {err}")))?;

    let runtime = Rc::new(RefCell::new(AppRuntime {
        gpu,
        atlas,
        shell,
        draw: DrawList::default(),
        input: InputState::default(),
        theme: Theme::default(),
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        modifiers: PointerModifiers::default(),
        wheel_delta: 0.0,
        asset_poll_pending: false,
        self_weak: std::rc::Weak::new(),
    }));
    runtime.borrow_mut().self_weak = Rc::downgrade(&runtime);

    start_frame_loop(runtime.clone());

    let runtime_pointer = runtime.clone();
    let runtime_move = runtime.clone();
    let runtime_wheel = runtime.clone();
    let runtime_keyboard = runtime.clone();

    attach_dom_listeners(
        &canvas,
        PointerCallbacks {
            on_move: Rc::new(move |x, y, down, button, modifiers| {
                let runtime = runtime_move.clone();
                spawn_local(async move {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.handle_pointer_move(x, y, down, button, modifiers).await;
                    }
                });
            }),
            on_button: Rc::new(move |x, y, down, button, modifiers| {
                let runtime = runtime_pointer.clone();
                spawn_local(async move {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.handle_pointer_button(x, y, down, button, modifiers).await;
                    }
                });
            }),
            on_wheel: Rc::new(move |delta, _modifiers| {
                if let Ok(mut app) = runtime_wheel.try_borrow_mut() {
                    app.wheel_delta += delta;
                }
            }),
            on_key: Rc::new(move |key| {
                let Ok(mut app) = runtime_keyboard.try_borrow_mut() else {
                    return;
                };
                if app.input.focused_id.is_some() {
                    app.input.text_buffer.push_str(&key);
                }
            }),
        },
    );

    let runtime_resize = runtime.clone();
    let canvas_resize = canvas.clone();
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let Ok(mut app) = runtime_resize.try_borrow_mut() else {
            return;
        };
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio() as f32)
            .unwrap_or(1.0);
        let w = canvas_resize.client_width().max(1) as f32;
        let h = canvas_resize.client_height().max(1) as f32;
        canvas_resize.set_width((w * dpr) as u32);
        canvas_resize.set_height((h * dpr) as u32);
        app.resize(w, h, dpr);
    }) as Box<dyn FnMut()>);
    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
    }
    closure.forget();

    web_sys::console::log_1(&JsValue::from_str("[DEBUG] wgpu renderer booted"));
    Ok(())
}
