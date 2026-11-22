//! 🎲️ Reads the Puzzle 2D history fixture through BoardHost and the production native Vello rasterizer.

use semio_framework_canvas_scene::{native_render::CanvasRasterSession, Color, Scene};
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_os_infinite_board_normal_focus::load_board_snapshot_json;
use semio_os_infinite_board_normal_host::{BoardHost, BoardHostRetirement};
use serde_json::Value;
use std::path::PathBuf;
use wgpu::*;

const SIZE: Extent3d = Extent3d { width: 512, height: 256, depth_or_array_layers: 1 };
const FIXTURE: &str = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/🧫️history-edit-runtime/🔣️.json";
const GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 1024 };

fn main() {
    std::thread::Builder::new()
        .stack_size(134_217_728)
        .spawn(|| {
            semio_framework_async::block_on(run());
        })
        .unwrap()
        .join()
        .unwrap();
}

async fn run() {
    let output = PathBuf::from(std::env::args_os().nth(1).expect("generated output"));
    let fixture: Value = serde_json::from_str(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap();
    let snapshot = fixture["board"].to_string();
    let mut board = BoardHost::new();
    board.scene.set_size(SIZE.width, SIZE.height, 1.0);
    board.style.grid_visible = false;
    board.style.grid_snap_enabled = false;
    let mut approve = |_| true;
    let mut control = NativeDecodeControl::new(64 * 1024 * 1024, &mut approve);
    assert!(load_board_snapshot_json(&mut board.scene, &mut board.focus, &mut board.style, &mut board.outbox, &snapshot, &mut control,));
    assert!(board.scene.nodes.contains_key("mid"));
    let authored_pose = (board.scene.nodes["mid"].x, board.scene.nodes["mid"].y);
    let mid_x = SIZE.width as f64 * 0.5;
    let mid_y = SIZE.height as f64 * 0.5;

    let instance = Instance::new(InstanceDescriptor { backends: Backends::PRIMARY, ..InstanceDescriptor::new_without_display_handle() });
    let adapter = instance.request_adapter(&RequestAdapterOptions::default()).await.unwrap();
    println!("[DEBUG] Board adapter {}", adapter.get_info().name);
    let limits = Limits::default().using_resolution(adapter.limits());
    let (device, queue) = adapter.request_device(&DeviceDescriptor { required_limits: limits, ..Default::default() }).await.unwrap();
    let mut renderer = CanvasRasterSession::new(&device).unwrap();
    let baseline = frame(&mut renderer, &device, &queue, &board.build_vector_scene());
    board.pointer_down_screen(mid_x, mid_y, 0, false, false);
    let pressed = frame(&mut renderer, &device, &queue, &board.build_vector_scene());
    board.pointer_move_screen(mid_x + 64.0, mid_y, false, false, false);
    let preview = frame(&mut renderer, &device, &queue, &board.build_vector_scene());
    assert!(board.pointer_cancel_screen());
    let restored = frame(&mut renderer, &device, &queue, &board.build_vector_scene());
    assert_eq!((board.scene.nodes["mid"].x, board.scene.nodes["mid"].y), authored_pose);
    let mut retirement = BoardHostRetirement::new(board);
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(934), Generation(1), GRANT, || Some(0), u64::MAX, 1 << 44);
    assert!((0..262_144).any(|_| caller.turn(GRANT, |cx| Ok(retirement.close_step(cx))).unwrap()));
    caller.retire(GRANT);
    let changed = pressed.chunks_exact(4).zip(preview.chunks_exact(4)).filter(|(a, b)| a != b).count();
    let painted = baseline.chunks_exact(4).filter(|p| p[3] != 0).count();
    println!("[DEBUG] Puzzle Board Vello baseline {painted} painted pixels and held {changed} changed pixels");
    let offset = |x: u32, y: u32| (4 * (y * SIZE.width + x) + 3) as usize;
    let y = SIZE.height / 2;
    let center = offset(SIZE.width / 2, y);
    let moved = offset(SIZE.width / 2 + 64, y);
    assert!(pressed[center] > 0 && pressed[moved] == 0);
    assert!(preview[center] == 0 && preview[moved] > 0);
    assert!(restored[center] > 0 && restored[moved] == 0);
    for (name, bytes) in [("board", baseline), ("pressed", pressed), ("preview", preview), ("restored", restored)] {
        std::fs::write(output.join(format!("{name}.rgba")), bytes).unwrap();
    }
    assert!(painted > 100);
    assert!(changed > 100);
    println!("[DEBUG] Cancel restored the center and vacated the translated sample");
}

fn frame(renderer: &mut CanvasRasterSession, device: &Device, queue: &Queue, scene: &Scene) -> Vec<u8> {
    let target = device.create_texture(&TextureDescriptor {
        label: Some("Puzzle 2D Board readback"),
        size: SIZE,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&TextureViewDescriptor::default());
    renderer.render(device, queue, scene, &view, Color::from_rgba8(0, 0, 0, 0), SIZE.width, SIZE.height).unwrap();
    let buffer = device.create_buffer(&BufferDescriptor { label: Some("Puzzle 2D Board pixels"), size: u64::from(SIZE.width * SIZE.height * 4), usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST, mapped_at_creation: false });
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
    encoder.copy_texture_to_buffer(target.as_image_copy(), TexelCopyBufferInfo { buffer: &buffer, layout: TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(SIZE.width * 4), rows_per_image: Some(SIZE.height) } }, SIZE);
    queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::sync_channel(1);
    let slice = buffer.slice(..);
    slice.map_async(MapMode::Read, move |result| send.send(result).unwrap());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    loop {
        device.poll(PollType::Poll).unwrap();
        if let Ok(result) = receive.try_recv() {
            result.unwrap();
            break;
        }
        assert!(std::time::Instant::now() < deadline, "timed out mapping Board pixels");
        std::thread::yield_now();
    }
    let pixels = slice.get_mapped_range().to_vec();
    buffer.unmap();
    pixels
}
