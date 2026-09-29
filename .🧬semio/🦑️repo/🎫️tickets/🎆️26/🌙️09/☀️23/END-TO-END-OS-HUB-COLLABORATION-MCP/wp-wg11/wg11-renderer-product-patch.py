#!/usr/bin/env python3
"""🧊️ WG11 session 14d — set for the first train after the chain (T6): renderer-wgpu product roots behind four red laws.

Measured (LW1 `wg11-laws-5.txt`, WG11 isolated re-run `.🧬semio/🌐hub/s14-wg11-captures/isolated-1.tsv`):

1. `directory_door::tests::the_directory_command_queue_compiles_on_the_browser_target` — the spawned hub workspace task
   (`ShellHubTask` + its answers and sign-in deadline, session 12) was written INSIDE `//#region 🎮️DirectoryCommandQueue`, and its
   native pool handle is `cfg(not(wasm32))`-gated. It is not the directory command FIFO: it moves to its own region
   `🔐️HubWorkspaceTask`, byte-identical, directly after the queue region.
2. `shell_chrome_parity_tests::no_capitalised_chrome_phrase_escapes_the_bilingual_tables` — the empty-dock glyph fault is an internal
   `Shell …` diagnostic like its eleven siblings (`Shell navbar title exceeded …`) and now says so.
3. `production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only` — `apply_presented_scene_caret` cloned the
   whole accepted `UiComponentSceneNode` (every scene payload) per caret phase to hand three identity fields to
   `component_scene_set_caret_visible`; the engine call now takes exactly `host_id`, `surface_id` and `kind`.
4. `async_boundary_tests::product_library_has_no_executor_bridge` — `compile_serving_others` parked a pool worker in
   `block_on(runtime.compile(..))`; the compile now runs as a `KernelPoolFuture` on the Background lane (the product's own
   pool-future owner), answering through the same handoff slot.
5. `icon_gpu_export_*` ×2 (and every headless `GpuContext`): `Device::create_render_pipeline` refuses `world3d_shadow_pipeline` —
   `vs_shadow` takes `InstanceInput`, whose `@location(10) emissive_cutoff` the shadow pipeline's hand-copied instance layout never
   declared (the main/translucent copies did). Native panics in wgpu's default error handler; WebGPU leaves the shadow pipeline
   invalid. The three hand-copied layouts become the ONE `world_vertex_layout()`/`world_instance_layout()` pair the material
   pipelines already share (crate semio-framework-ui).
6. …and past that, `Device::create_shader_module` refuses `world3d_painted_shader`: its `.replacen` chain anchored on the
   lit shader's emissive line WITHOUT the `+ in.emissive_cutoff.rgb` term the lit shader gained, so the edit that declares
   `lit_color` silently matched nothing and the next edit referenced an undefined identifier. The edits become ONE ordered
   table `WORLD3D_PAINTED_SHADER_EDITS` whose every anchor a CPU law finds exactly once in `WORLD3D_SHADER`.
7. `native_accessibility::tests::every_platform_action_is_the_shell_event_the_fixture_names` ("blur-blurs"): the 09-27 gate that
   refuses an action a node does not advertise (so Expand never activates a leaf) also refused Blur, which no node advertises —
   a node gives focus back exactly when it can take it. Blur is admitted on every node that advertises Focus.
8. `diff_view_tests::neutral_presentation_fixture_drives_mono_geometry_color_and_pre_wrap`: `FontAtlas::pre_wrap_lines` priced
   the spaces ending a line into the overflow test, so `a  b  c  d` in a 7.7-column box broke BEFORE `c` (`["a  b  ", "c  d"]`).
   CSS `pre-wrap` hangs those spaces (CSS Text 3 §4.1.3; React's DOM line `["a  b  c  ", "d"]` in the shared fixture): a space
   never triggers the overflow break. Law in the ui text unit (`pre_wrap_keeps_repeated_spaces_and_hangs_the_ones_ending_a_line`).

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/renderer-product/` and applies;
`--revert` restores the backups.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = ENGINE / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
INTERPRETER = ENGINE / "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"
ENGINE_CANVAS = ENGINE / "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"
RENDERER = ENGINE / "🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
DRAW = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs"
DRAW_LAWS = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs"
SHADERS = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs"
NATIVE_AX = ENGINE / "🎯️targets/🧊️wgpu/♿️native-accessibility/🦀️.rs"
TEXT = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs"
TEXT_LAWS = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs"

NATIVE_AX_EDITS = [
    (
        '''/// 🎯️ The shell event one assistive-technology action request is: `Focus`, `Blur`, `Click` (activate) and `SetValue`
/// on a node this tree addresses. Anything else — another action, a node the tree does not know — answers nothing.
fn action_dispatch(tree: &NativeAccessibilityTree, request: &accesskit::ActionRequest) -> Option<ui_render::DispatchEvent> {
    let target = tree.addresses.get(&request.target_node)?.clone();
    let supported = tree.update.nodes.iter().find(|(id, _)| *id == request.target_node).is_some_and(|(_, node)| node.supports_action(request.action));''',
        '''/// 🎯️ The shell event one assistive-technology action request is: `Focus`, `Blur`, `Click` (activate) and `SetValue`
/// on a node this tree addresses and that advertises the action — `Blur` being `Focus`'s inverse, a node that can take focus
/// can give it back. Anything else — another action, an unadvertised one, a node the tree does not know — answers nothing.
fn action_dispatch(tree: &NativeAccessibilityTree, request: &accesskit::ActionRequest) -> Option<ui_render::DispatchEvent> {
    let target = tree.addresses.get(&request.target_node)?.clone();
    let supported = tree.update.nodes.iter().find(|(id, _)| *id == request.target_node).is_some_and(|(_, node)| node.supports_action(request.action) || (request.action == accesskit::Action::Blur && node.supports_action(accesskit::Action::Focus)));''',
    )
]

TEXT_EDITS = [
    (
        '''    /// ↩️ Returns CSS `white-space: pre-wrap` line ranges while retaining repeated and trailing spaces.
    pub fn pre_wrap_lines(''',
        '''    /// ↩️ Returns CSS `white-space: pre-wrap` line ranges: every space is kept, and the spaces ending a line HANG (CSS Text 3
    /// §4.1.3) — they never push the line over its box, so the break falls after them and only a word opens the next line.
    pub fn pre_wrap_lines(''',
    ),
    (
        '''                let advance = self.ensure_glyph_for(face, ch, size).advance;
                if pen + advance > limit + LINE_BREAK_FIT_EPSILON && byte > line_start {
                    if let Some(split) = last_break.filter(|split| *split > line_start) {''',
        '''                let advance = self.ensure_glyph_for(face, ch, size).advance;
                if !is_wrap_space(ch) && pen + advance > limit + LINE_BREAK_FIT_EPSILON && byte > line_start {
                    if let Some(split) = last_break.filter(|split| *split > line_start) {''',
    ),
]

TEXT_LAW_EDITS = [
    (
        '''    let (width, _) = atlas.measure_text_wrapped("one two ", exact, size);
    assert!((width - exact).abs() < 0.001, "a hanging space is not priced into the line box, got {width}");
}
''',
        '''    let (width, _) = atlas.measure_text_wrapped("one two ", exact, size);
    assert!((width - exact).abs() < 0.001, "a hanging space is not priced into the line box, got {width}");
}

/// ⚖️ LAW (ticket 26/09/23 session 14d, WG11): `white-space: pre-wrap` keeps every space, and the spaces ending a line hang —
/// a line whose last word fits breaks AFTER its trailing spaces, never before that word (the DiffView presentation fixture's
/// `preWrap` vector, `🧬️contract/🧫️fixtures/🆚️diff-view-presentation/🔣️.json`). A hard newline still breaks, empty lines stay.
#[test]
fn pre_wrap_keeps_repeated_spaces_and_hangs_the_ones_ending_a_line() {
    use super::TextFace;
    let size = ui_styling::metrics::typography::TEXT_XS_PX as f32;
    let mut atlas = FontAtlas::builtin();
    let text = "a  b  c  d";
    let fits = atlas.measure_range_face(TextFace::Mono, text, 0, "a  b  c".len(), size);
    let lines = atlas.pre_wrap_lines(TextFace::Mono, text, fits + 0.5, size).into_iter().map(|range| &text[range]).collect::<Vec<_>>();
    assert_eq!(lines, ["a  b  c  ", "d"], "the spaces after `c` hang; `d` opens the next line");
    assert_eq!(atlas.pre_wrap_lines(TextFace::Mono, "one\\n\\ntwo", 1_000.0, size).len(), 3, "a hard newline breaks and an empty line is kept");
}
''',
    ),
]

SHADER_EDITS = [
    (
        '''pub fn world3d_painted_shader() -> String {
    WORLD3D_SHADER
        .replacen(
            "@group(1) @binding(1) var shadow_sampler: sampler_comparison;",
            "@group(1) @binding(1) var shadow_sampler: sampler_comparison;\\n@group(2) @binding(0) var paint_map: texture_2d<f32>;\\n@group(2) @binding(1) var paint_sampler: sampler;",
            1,
        )
        .replacen("@location(2) color: vec4<f32>,\\n}", "@location(2) color: vec4<f32>,\\n@location(9) uv: vec2<f32>,\\n}", 1)
        .replacen("@location(4) emissive_cutoff: vec4<f32>,\\n}", "@location(4) emissive_cutoff: vec4<f32>,\\n@location(5) uv: vec2<f32>,\\n}", 1)
        .replacen("out.emissive_cutoff = instance.emissive_cutoff;\\nreturn out;", "out.emissive_cutoff = instance.emissive_cutoff;\\nout.uv = vertex.uv;\\nreturn out;", 1)
        .replacen(
            "let emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0);",
            "let sampled = textureSample(paint_map, paint_sampler, in.uv);\\nlet authored_texture = (u32(in.flags.x) & 4u) != 0u;\\nlet paint_color = select(world3d_linear_to_srgb(sampled.rgb), sampled.rgb, authored_texture);\\nlet lit_color = in.color.rgb * paint_color;\\nif (in.emissive_cutoff.w >= 0.0 && sampled.a * in.color.a < in.emissive_cutoff.w) { discard; }\\nlet emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0) + in.emissive_cutoff.rgb;",
            1,
        )
        .replacen(
            "let color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, shadow_visibility) + emissive;\\nreturn vec4<f32>(world3d_attachment_output(color), in.color.a);",
            "let color = world3d_lighting(n, v, lit_color, metalness, roughness, shadow_visibility) + emissive;\\nreturn vec4<f32>(world3d_attachment_output(color), sampled.a * in.color.a);",
            1,
        )
}''',
        '''pub fn world3d_painted_shader() -> String {
    WORLD3D_PAINTED_SHADER_EDITS.iter().fold(WORLD3D_SHADER.to_string(), |shader, (anchor, edit)| shader.replacen(anchor, edit, 1))
}

/// 🎨️ The ordered edits that turn [`WORLD3D_SHADER`] into [`world3d_painted_shader`]: the paint map bindings, the UV
/// varyings, the sampled albedo with its mask test, and the sampled alpha. Every anchor occurs EXACTLY once in the lit
/// shader (law `every_painted_shader_edit_finds_its_anchor_in_the_lit_shader_once`) — an anchor that stopped matching once
/// dropped the `lit_color` declaration silently and every GPU context refused the painted shader module.
pub const WORLD3D_PAINTED_SHADER_EDITS: [(&str, &str); 6] = [
    (
        "@group(1) @binding(1) var shadow_sampler: sampler_comparison;",
        "@group(1) @binding(1) var shadow_sampler: sampler_comparison;\\n@group(2) @binding(0) var paint_map: texture_2d<f32>;\\n@group(2) @binding(1) var paint_sampler: sampler;",
    ),
    ("@location(2) color: vec4<f32>,\\n}", "@location(2) color: vec4<f32>,\\n@location(9) uv: vec2<f32>,\\n}"),
    ("@location(4) emissive_cutoff: vec4<f32>,\\n}", "@location(4) emissive_cutoff: vec4<f32>,\\n@location(5) uv: vec2<f32>,\\n}"),
    ("out.emissive_cutoff = instance.emissive_cutoff;\\nreturn out;", "out.emissive_cutoff = instance.emissive_cutoff;\\nout.uv = vertex.uv;\\nreturn out;"),
    (
        "let emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0) + in.emissive_cutoff.rgb;",
        "let sampled = textureSample(paint_map, paint_sampler, in.uv);\\nlet authored_texture = (u32(in.flags.x) & 4u) != 0u;\\nlet paint_color = select(world3d_linear_to_srgb(sampled.rgb), sampled.rgb, authored_texture);\\nlet lit_color = in.color.rgb * paint_color;\\nif (in.emissive_cutoff.w >= 0.0 && sampled.a * in.color.a < in.emissive_cutoff.w) { discard; }\\nlet emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0) + in.emissive_cutoff.rgb;",
    ),
    (
        "let color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, shadow_visibility) + emissive;\\nreturn vec4<f32>(world3d_attachment_output(color), in.color.a);",
        "let color = world3d_lighting(n, v, lit_color, metalness, roughness, shadow_visibility) + emissive;\\nreturn vec4<f32>(world3d_attachment_output(color), sampled.a * in.color.a);",
    ),
];''',
    )
]
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/renderer-product"

HUB_TASK_START = "/// ⏳️ The credential sign-in's own deadline (the mint and the `me` read that follows it)."
HUB_TASK_END = "/// ⏳️ Browser identity retry floor: a hub that refuses `/auth/sessions/me` must not be re-asked on"
QUEUE_END = "//#endregion 🎮️DirectoryCommandQueue\n"

SHELL_EDITS = [
    (
        '''                        self.error = Some("Empty dock notice exceeded the retained glyph boundary".into());''',
        '''                        self.error = Some("Shell empty dock notice exceeded the retained glyph boundary".into());''',
    )
]

INTERPRETER_EDITS = [
    (
        '''        let UiNode::ComponentScene(scene) = &tree.node(node)?.spec.0 else { return None };
        Some(scene.clone())
    });
    scene.as_ref().is_some_and(|scene| crate::engine_canvas::component_scene_set_caret_visible(scene, caret.visible))''',
        '''        let UiNode::ComponentScene(scene) = &tree.node(node)?.spec.0 else { return None };
        Some((scene.host_id.clone(), scene.surface_id.clone(), scene.component_kind))
    });
    scene.is_some_and(|(host_id, surface_id, kind)| crate::engine_canvas::component_scene_set_caret_visible(&host_id, &surface_id, kind, caret.visible))''',
    )
]

ENGINE_CANVAS_EDITS = [
    (
        '''/// ✍️ Applies one accepted component scene's caret phase to its exact mounted engine surface.
pub fn component_scene_set_caret_visible(scene: &UiComponentSceneNode, visible: bool) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let changed = {
            let Some(entry) = surfaces.get_mut(&scene.host_id) else { return false };
            if entry.surface_id.as_ref().is_none_or(|surface| surface.as_str() != scene.surface_id) {
                return false;
            }
            match scene.component_kind {''',
        '''/// ✍️ Applies one accepted component scene's caret phase to its exact mounted engine surface — addressed by the scene's
/// identity alone (`host_id`, `surface_id`, `kind`), so a caret phase never copies the scene's payload.
pub fn component_scene_set_caret_visible(host_id: &str, surface_id: &str, kind: SurfaceKind, visible: bool) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let changed = {
            let Some(entry) = surfaces.get_mut(host_id) else { return false };
            if entry.surface_id.as_ref().is_none_or(|surface| surface.as_str() != surface_id) {
                return false;
            }
            match kind {''',
    ),
    (
        '''                    host.set_note_caret_visible(visible);
                    true
                }
                _ => false,
            }
        };
        if changed {
            mark_engine_scene_repaint(&mut surfaces, &scene.host_id);
        }
        changed''',
        '''                    host.set_note_caret_visible(visible);
                    true
                }
                _ => false,
            }
        };
        if changed {
            mark_engine_scene_repaint(&mut surfaces, host_id);
        }
        changed''',
    ),
]

RENDERER_EDITS = [
    (
        '''            crate::renderer_worker_pool().submit(
                semio_framework_async::Lane::Background,
                Box::new(move || {
                    let result = semio_framework_async::block_on(runtime.compile(&key, &bytes)).map_err(|error| error.to_string());
                    let waker = {
                        let mut slot = worker.lock().expect("compile handoff lock");
                        slot.0 = Some(result);
                        slot.1.take()
                    };
                    if let Some(waker) = waker {
                        waker.wake();
                    }
                }),
            );''',
        '''            let _ = KernelPoolFuture::spawn(crate::renderer_worker_pool(), semio_framework_async::Lane::Background, async move {
                let result = runtime.compile(&key, &bytes).await.map_err(|error| error.to_string());
                let waker = {
                    let mut slot = worker.lock().expect("compile handoff lock");
                    slot.0 = Some(result);
                    slot.1.take()
                };
                if let Some(waker) = waker {
                    waker.wake();
                }
            });''',
    )
]


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def shell_after(source: str) -> str:
    if "//#region 🔐️HubWorkspaceTask" in source:
        sys.exit("🔐️HubWorkspaceTask region already present — landed already")
    for anchor in (HUB_TASK_START, HUB_TASK_END, QUEUE_END):
        if source.count(anchor) != 1:
            sys.exit(f"shell anchor occurs {source.count(anchor)}x: {anchor[:80]!r}")
    start, end, queue_end = source.index(HUB_TASK_START), source.index(HUB_TASK_END), source.index(QUEUE_END)
    if not start < end < queue_end:
        sys.exit("the hub task block no longer sits inside the directory command queue region")
    block = source[start:end]
    if "struct ShellHubTask<T>" not in block or "impl<T: 'static> ShellHubTask<T>" not in block or not block.endswith("}\n"):
        sys.exit("the hub task block changed shape")
    moved = source[:start] + source[end:]
    queue_end = moved.index(QUEUE_END) + len(QUEUE_END)
    region = "\n//#region 🔐️HubWorkspaceTask\n" + block + "//#endregion 🔐️HubWorkspaceTask\n"
    return replaced(SHELL, moved[:queue_end] + region + moved[queue_end:], SHELL_EDITS)


LAYOUTS_START = "        let world_vertex_layout = || wgpu::VertexBufferLayout {\n"
LAYOUTS_END = "        let celebration_instance_layout = || wgpu::VertexBufferLayout {\n"
FIRST_WORLD_PIPELINE = "        let world_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {\n            label: Some(\"world3d_pipeline\"),\n"
LOCATION_10 = "                            wgpu::VertexAttribute { offset: 96, shader_location: 10, format: wgpu::VertexFormat::Float32x4 },\n"
SHARED_BUFFERS = "                buffers: &[world_vertex_layout(), world_instance_layout()],\n"


def draw_after(source: str) -> str:
    for anchor in (LAYOUTS_START, LAYOUTS_END, FIRST_WORLD_PIPELINE):
        if source.count(anchor) != 1:
            sys.exit(f"draw anchor occurs {source.count(anchor)}x: {anchor[:80]!r}")
    blocks = {}
    for label in ("world3d_pipeline", "world3d_pipeline_translucent", "world3d_shadow_pipeline"):
        at = source.index(f'label: Some("{label}")')
        start = source.index("                buffers: &[\n", at)
        end = source.index("                ],\n                compilation_options", start) + len("                ],\n")
        blocks[label] = (start, end, source[start:end])
    main = blocks["world3d_pipeline"][2]
    if blocks["world3d_pipeline_translucent"][2] != main or main.count(LOCATION_10) != 1:
        sys.exit("the main/translucent hand-copied layouts drifted from each other")
    if blocks["world3d_shadow_pipeline"][2] != main.replace(LOCATION_10, ""):
        sys.exit("the shadow layout is not the main layout minus @location(10) — re-measure")
    layouts_start, layouts_end = source.index(LAYOUTS_START), source.index(LAYOUTS_END)
    layouts = source[layouts_start:layouts_end]
    if layouts.count("wgpu::VertexAttribute { offset: 96, shader_location: 10, format: wgpu::VertexFormat::Float32x4 }") != 1 or "let world_instance_layout = || wgpu::VertexBufferLayout {" not in layouts:
        sys.exit("the shared world layouts changed shape")
    for start, end, _ in sorted(blocks.values(), reverse=True):
        source = source[:start] + SHARED_BUFFERS + source[end:]
    for label in blocks:
        at = source.index(f'label: Some("{label}")')
        start = source.index("            vertex: wgpu::VertexState {\n", at)
        end = source.index("            },\n", start) + len("            },\n")
        fields = [line.strip().rstrip(",") for line in source[start:end].splitlines()[1:-1]]
        if len(fields) != 4:
            sys.exit(f"the {label} vertex state changed shape: {fields}")
        source = source[:start] + "            vertex: wgpu::VertexState { " + ", ".join(fields) + " },\n" + source[end:]
    source = source.replace(layouts, "", 1)
    at = source.index(FIRST_WORLD_PIPELINE)
    return source[:at] + layouts + source[at:]


PIPELINE_LAW = '''
/// 🎨️ LAW (ticket 26/09/23 session 14d, WG11): every edit that turns the lit mesh shader into the painted one finds its anchor
/// EXACTLY once — the emissive anchor once lost its match when the lit shader gained `+ in.emissive_cutoff.rgb`, the
/// `lit_color` declaration was silently dropped, and every GPU context refused `world3d_painted_shader`.
#[test]
fn every_painted_shader_edit_finds_its_anchor_in_the_lit_shader_once() {
    for (anchor, _) in crate::wgpu::shaders::WORLD3D_PAINTED_SHADER_EDITS {
        assert_eq!(crate::wgpu::shaders::WORLD3D_SHADER.matches(anchor).count(), 1, "painted shader anchor: {anchor}");
    }
    let painted = crate::wgpu::shaders::world3d_painted_shader();
    assert!(painted.contains("let lit_color = in.color.rgb * paint_color;") && painted.contains("out.uv = vertex.uv;"), "every painted lane is applied");
}
/// 🧊️ LAW (ticket 26/09/23 session 14d, WG11): a native `GpuContext` creates every render pipeline without a validation error.
/// `world3d_shadow_pipeline` once declared its own copy of the instance layout without `@location(10)` of the `InstanceInput` its
/// `vs_shadow` reads, and wgpu's default error handler panicked inside every native `GpuContext::from_device` (both GPU icon export
/// laws, every native renderer boot). A machine with no adapter at all is the only accepted absence.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_native_gpu_context_creates_every_pipeline_without_a_validation_error() {
    match semio_framework_async::block_on(crate::wgpu::gpu::GpuContext::headless(1, 1)) {
        Ok(_) => {}
        Err(error) => assert!(error.starts_with("offscreen adapter:"), "a native GPU context refused past adapter selection: {error}"),
    }
}
'''

def draw_laws_after(source: str) -> str:
    if "fn a_native_gpu_context_creates_every_pipeline_without_a_validation_error" in source:
        sys.exit("pipeline-creation law already present — landed already")
    return source.rstrip("\n") + "\n" + PIPELINE_LAW


FILES = [SHELL, INTERPRETER, ENGINE_CANVAS, RENDERER, DRAW, DRAW_LAWS, SHADERS, NATIVE_AX, TEXT, TEXT_LAWS]


def plans():
    read = {path: path.read_text(encoding="utf-8") for path in FILES}
    return [
        (SHELL, read[SHELL], shell_after(read[SHELL])),
        (INTERPRETER, read[INTERPRETER], replaced(INTERPRETER, read[INTERPRETER], INTERPRETER_EDITS)),
        (ENGINE_CANVAS, read[ENGINE_CANVAS], replaced(ENGINE_CANVAS, read[ENGINE_CANVAS], ENGINE_CANVAS_EDITS)),
        (RENDERER, read[RENDERER], replaced(RENDERER, read[RENDERER], RENDERER_EDITS)),
        (DRAW, read[DRAW], draw_after(read[DRAW])),
        (DRAW_LAWS, read[DRAW_LAWS], draw_laws_after(read[DRAW_LAWS])),
        (SHADERS, read[SHADERS], replaced(SHADERS, read[SHADERS], SHADER_EDITS)),
        (NATIVE_AX, read[NATIVE_AX], replaced(NATIVE_AX, read[NATIVE_AX], NATIVE_AX_EDITS)),
        (TEXT, read[TEXT], replaced(TEXT, read[TEXT], TEXT_EDITS)),
        (TEXT_LAWS, read[TEXT_LAWS], replaced(TEXT_LAWS, read[TEXT_LAWS], TEXT_LAW_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in FILES:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(FILES)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-os-renderer-wgpu, semio-framework-ui)")


if __name__ == "__main__":
    main()
