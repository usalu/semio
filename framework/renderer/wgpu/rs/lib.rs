//! 🧊 Raw wgpu WASM renderer for declarative framework UiNode trees.

pub mod draw;
pub mod gpu;
pub mod input;
pub mod layout_engine;
pub mod plugin_bridge;
pub mod scenes;
pub mod shaders;
pub mod shell;
pub mod text;
pub mod theme;
pub mod widgets;

use draw::DrawList;
use gpu::{schedule_frame, GpuContext};
use input::{attach_dom_listeners, InputState};
use plugin_bridge::{filter_plugins, parse_plugin_entries};
use shell::ShellState;
use std::cell::RefCell;
use std::rc::Rc;
use text::{fetch_font_bytes, FontAtlas};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

struct AppRuntime {
    gpu: GpuContext,
    atlas: FontAtlas,
    shell: ShellState,
    draw: DrawList,
    input: InputState,
}

impl AppRuntime {
    fn frame(&mut self) {
        self.input.clear_frame();
        self.draw.clear();
        self.shell
            .render_chrome(&mut self.draw, &mut self.atlas, &mut self.input);
        if let Err(err) = self.gpu.render_frame(&self.draw) {
            web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] render frame: {err}")));
        }
    }

    fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.gpu.resize(css_width, css_height, dpr);
        self.shell.screen_w = (css_width * dpr).max(1.0);
        self.shell.screen_h = (css_height * dpr).max(1.0);
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

    let font_bytes = fetch_font_bytes("/asset/font/kelly-slab/latin.woff2")
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
    }));

    start_frame_loop(runtime.clone());

    let runtime_pointer = runtime.clone();
    let runtime_keyboard = runtime.clone();
    attach_dom_listeners(
        &canvas,
        Rc::new(move |x, y, down| {
            let Ok(mut app) = runtime_pointer.try_borrow_mut() else {
                return;
            };
            app.input.pointer_x = x;
            app.input.pointer_y = y;
            app.input.pointer_down = down;
            if !down {
                return;
            }
            let hit = app.input.hit_at(x, y).cloned();
            let Some(hit) = hit else { return };
            if let Some(command) = hit.command.clone() {
                let shell_runtime = runtime_pointer.clone();
                spawn_local(async move {
                    if let Ok(mut app) = shell_runtime.try_borrow_mut() {
                        if let Err(err) = app.shell.dispatch_command(command).await {
                            web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] command failed: {err}")));
                        }
                    }
                });
            }
            if hit.kind == input::HitKind::Input {
                app.input.focused_id = hit.control_id.clone();
            }
        }),
        Rc::new(move |key| {
            let Ok(mut app) = runtime_keyboard.try_borrow_mut() else {
                return;
            };
            if app.input.focused_id.is_some() {
                app.input.text_buffer.push_str(&key);
            }
        }),
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

#[cfg(test)]
mod tests {
    use super::draw::ear_clip_polygon;

    #[test]
    fn ear_clip_produces_triangles() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let tris = ear_clip_polygon(&square);
        assert!(tris.len() >= 3);
    }
}
