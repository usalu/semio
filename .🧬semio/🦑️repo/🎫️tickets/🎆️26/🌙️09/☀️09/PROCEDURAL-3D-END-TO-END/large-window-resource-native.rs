extern crate self as semio_ui_contract;
extern crate self as semio_ui_declaration;
extern crate self as semio_ui_wgpu_hosting;

extern crate self as semio_framework_async;
extern crate self as semio_framework_io_base64;
extern crate self as semio_framework_pixels;
extern crate self as semio_renderer_wgpu_host_base;

pub fn request_retained_image_raster_frame() {}
pub enum Lane {
    Maintenance,
}
pub struct Pool;
pub fn renderer_worker_pool() -> Pool {
    Pool
}
impl Pool {
    pub fn try_submit(&self, _: Lane, job: Box<dyn FnOnce() + Send>) -> Result<(), ()> {
        job();
        Ok(())
    }
}

pub fn base64_standard_decode(text: &str) -> Result<Vec<u8>, ()> {
    Ok(text.as_bytes().to_vec())
}
pub mod image_decoding {
    pub struct EncodedDecodeLimits;
    impl EncodedDecodeLimits {
        pub const MAXIMUM: Self = Self;
    }
    pub struct Image {
        pub pixels: Vec<u8>,
        pub width: u32,
        pub height: u32,
    }
    pub struct Decoded {
        pub image: Image,
    }
    pub fn decode_encoded_blocking(data: Vec<u8>, _: EncodedDecodeLimits, cancelled: &dyn Fn() -> bool) -> Result<Decoded, ()> {
        assert!(!cancelled());
        let mut decoded = super::DECODED.lock().unwrap();
        *decoded = Some(String::from_utf8(data).unwrap());
        let pixels = std::fs::read(std::env::args().nth(2).unwrap()).unwrap();
        Ok(Decoded { image: Image { pixels, width: 1024, height: 768 } })
    }
}
fn parse_svg_data_url_bytes(_: &str) -> Option<Vec<u8>> {
    None
}
fn rasterize_svg_to_rgba(_: &str) -> Option<(Vec<u8>, u32, u32)> {
    None
}
type Tile = (u32, u32, Vec<u8>);
static UPLOADS: std::sync::LazyLock<Mutex<std::collections::HashMap<String, Tile>>> = std::sync::LazyLock::new(Default::default);
fn queue_decoded_raster_upload_retaining_source(_owner: &str, key: String, width: u32, height: u32, tile: &mut Vec<u8>) -> Result<String, RasterUploadRefusal> {
    assert_eq!(tile.len(), width as usize * height as usize * 4);
    assert!(tile.len() <= 1024 * 1024);
    UPLOADS.lock().unwrap().insert(key.clone(), (width, height, std::mem::take(tile)));
    Ok(key)
}

use std::{
    cell::RefCell,
    sync::{Mutex, OnceLock},
};

struct WorkerCell<T> {
    new_value: fn() -> T,
    value: OnceLock<Mutex<RefCell<T>>>,
}

impl<T> WorkerCell<T> {
    const fn new(new_value: fn() -> T) -> Self {
        Self { new_value, value: OnceLock::new() }
    }

    fn with<R>(&self, f: impl FnOnce(&RefCell<T>) -> R) -> R {
        let value = self.value.get_or_init(|| Mutex::new(RefCell::new((self.new_value)())));
        f(&value.lock().unwrap())
    }
}

mod ui {
    pub struct UiImageNode {
        pub id: String,
        pub src: String,
        pub source_page: Option<String>,
    }
}
mod scene_slots {
    #[derive(Debug, PartialEq)]
    pub enum ScenePaintStep {
        Complete,
        Pending,
        Fault,
    }
    pub struct ScenePaintCursor(pub usize);
    impl ScenePaintCursor {
        pub fn item(&mut self) -> usize {
            self.0
        }
        pub fn advance_item(&mut self) -> Result<(), ()> {
            self.0 += 1;
            Ok(())
        }
        pub fn finish(&mut self) -> ScenePaintStep {
            self.0 = 0;
            ScenePaintStep::Complete
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}
fn object_contain_rect(bounds: Rect, _width: f32, _height: f32) -> Rect {
    bounds
}
enum RasterUploadRefusal {
    Invalid(&'static str),
    Busy,
}
static DECODED: Mutex<Option<String>> = Mutex::new(None);
fn queue_canvas_image_upload_sized(_: &str, _: &str, text: &str) -> (Result<String, RasterUploadRefusal>, Option<(u32, u32)>) {
    if !text.starts_with("data:image/png;") {
        return (Err(RasterUploadRefusal::Invalid("format")), None);
    }
    *DECODED.lock().unwrap() = Some(text.into());
    (Ok("raster".into()), Some((1024, 768)))
}
fn resolve_ui_image_svg(_: &str, _: &str) -> (Result<String, RasterUploadRefusal>, Option<(u32, u32)>) {
    (Err(RasterUploadRefusal::Invalid("fixture format")), None)
}

#[derive(Default)]
struct Request {
    window: String,
    generation: u64,
    image: String,
    key: String,
    page: u32,
}
struct Input(Vec<Request>);
impl Input {
    fn publish_action(&mut self, controller: &str, name: &str, _: usize, build: impl FnOnce(&mut Request, &str) -> Result<(), ()>) -> Result<(), ()> {
        assert_eq!((controller, name), ("framework", "setImageSourcePage"));
        let mut request = Request::default();
        build(&mut request, "")?;
        self.0.push(request);
        Ok(())
    }
}
impl Request {
    fn begin_object(&mut self, _: Option<&str>) -> Result<(), ()> {
        Ok(())
    }
    fn end_container(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn string(&mut self, key: Option<&str>, value: &str) -> Result<(), ()> {
        if key == Some("windowGeneration") {
            self.generation = value.parse().unwrap();
            return Ok(());
        }
        *match key {
            Some("windowId") => &mut self.window,
            Some("imageId") => &mut self.image,
            Some("resourceKey") => &mut self.key,
            _ => return Err(()),
        } = value.into();
        Ok(())
    }
    fn number(&mut self, key: Option<&str>, value: f64) -> Result<(), ()> {
        assert_eq!(key, Some("page"));
        self.page = value as u32;
        Ok(())
    }
}
type Quad = (String, [f32; 4], [f32; 4]);
struct Draw {
    progress: usize,
    rasters: Vec<Quad>,
}
impl Draw {
    fn try_reserve_retained_items(&mut self, _: usize) -> Result<(), ()> {
        Ok(())
    }
    fn push_raster_quad(&mut self, key: &str, rect: [f32; 4], uv: [f32; 4], _: f32) {
        self.rasters.push((key.into(), rect, uv));
    }
    fn push_rounded(&mut self, _: [f32; 4], _: u8, _: f32) {
        self.progress += 1;
    }
}
struct Theme {
    accent: u8,
    muted: u8,
    border_radius: f32,
}
struct FrameworkWidgetContext<'a> {
    draw: &'a mut Draw,
    input: &'a mut Input,
    theme: &'a Theme,
}

fn verify_native(source: &str, window: &str, question: &str, resource: WindowResource) {
    use native_image::{close_hidden_window_image_resource, close_window_image_resource_step, render_window_image_resource, replace_window_image_resource};
    use scene_slots::ScenePaintCursor;
    use ui::UiImageNode;
    let mut ctx = FrameworkWidgetContext { draw: &mut Draw { progress: 0, rasters: Vec::new() }, input: &mut Input(Vec::new()), theme: &Theme { accent: 1, muted: 2, border_radius: 1.0 } };
    let mut cursor = ScenePaintCursor(0);
    let bounds = Rect { x: 0.0, y: 0.0, w: 1024.0, h: 768.0 };
    let image = |index| {
        let page = WindowResourcePage { resource, index };
        UiImageNode { id: question.into(), src: page.key(), source_page: page.text(source).map(str::to_owned) }
    };
    render_window_image_resource(&image(1), bounds, &mut ctx, &mut cursor, "foreign", 1, 100);
    assert!(ctx.input.0.is_empty());
    render_window_image_resource(&image(1), bounds, &mut ctx, &mut cursor, window, 1, 100);
    assert_eq!(ctx.input.0.last().unwrap().page, 2);
    assert_eq!(ctx.draw.progress, 2);
    replace_window_image_resource(&image(1), &mut ctx, window, 1);
    assert_eq!(ctx.input.0.last().unwrap().page, 0);
    render_window_image_resource(&image(1), bounds, &mut ctx, &mut cursor, window, 1, 100);
    assert_eq!(ctx.input.0.last().unwrap().page, 2);
    assert!(!close_hidden_window_image_resource(&mut ctx, window, 1, 100));
    assert!(close_hidden_window_image_resource(&mut ctx, window, 1, 101));
    assert_eq!(ctx.input.0.last().unwrap().page, 0);
    assert!(!close_window_image_resource_step(window, 1));
    ctx.input.0.clear();
    for index in 1..=resource.pages() {
        assert_eq!(render_window_image_resource(&image(index), bounds, &mut ctx, &mut cursor, window, 2, 100), scene_slots::ScenePaintStep::Complete);
    }
    for _ in 0..32 {
        let was_ready = ctx.draw.rasters.len() > 0;
        let result = render_window_image_resource(&image(1), bounds, &mut ctx, &mut cursor, window, 2, 100);
        if was_ready && result == scene_slots::ScenePaintStep::Complete {
            break;
        }
    }
    let requests = &ctx.input.0;
    assert!(requests.iter().all(|request| request.window == window && request.generation == 2 && request.image == question && request.key == resource.request_key()));
    assert_eq!(requests.iter().map(|request| request.page).collect::<Vec<_>>(), (2..=resource.pages()).chain([0]).collect::<Vec<_>>());
    assert_eq!(ctx.draw.rasters.len(), 6);
    assert_eq!(DECODED.lock().unwrap().take().as_deref(), Some(source.split_once(',').unwrap().1));
    let expected = std::fs::read(std::env::args().nth(2).unwrap()).unwrap();
    let mut actual = vec![0; expected.len()];
    let uploads = UPLOADS.lock().unwrap();
    for (key, [left, top, width, height], [u, v, max_u, max_v]) in &ctx.draw.rasters {
        let &(tile_width, tile_height, ref data) = uploads.get(key).unwrap();
        let offset_x = (u * tile_width as f32).round() as usize;
        let offset_y = (v * tile_height as f32).round() as usize;
        let left = *left as usize;
        let top = *top as usize;
        let width = *width as usize;
        let height = *height as usize;
        assert_eq!((max_u * tile_width as f32).round() as usize - offset_x, width);
        assert_eq!((max_v * tile_height as f32).round() as usize - offset_y, height);
        for y in 0..height {
            let from = ((y + offset_y) * tile_width as usize + offset_x) * 4;
            let to = ((y + top) * 1024 + left) * 4;
            actual[to..to + width * 4].copy_from_slice(&data[from..from + width * 4]);
        }
    }
    assert_eq!(actual, expected);
    assert!(!close_window_image_resource_step(window, 1));
    assert!(close_window_image_resource_step(window, 2));
}
