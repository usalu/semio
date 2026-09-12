#!/usr/bin/env python3
"""🔁️ Baseline toggle (2026-09-12 ticket boxed-fixed-slots: the helper now lives in
`semio_framework_async::boxed_fixed_slots`; the call sites below were re-pointed accordingly) — reverses/re-applies the four `boxed_fixed_slots` call sites by exact string
replacement (never a file snapshot), so a concurrent peer's unrelated edits in the same files survive.
Diagnostic only: `off` reproduces the pre-fix tree for a baseline suite run, `on` restores it.
"""
import io, sys

SCENES = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"
CANVAS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"

HELPER_HEAD = None  # 🧹️ the private helper moved to `semio-framework-async`; nothing to (un)gate here any more
PAIRS = [
    (SCENES, "slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; SCENE_SURFACE_CAPACITY]",
             "slots: Box::new([const { None }; SCENE_SURFACE_CAPACITY]), epochs: [0; SCENE_SURFACE_CAPACITY]"),
    (SCENES, "slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; RASTER_UPLOADS_PER_SURFACE_CAPACITY]",
             "slots: Box::new([const { None }; RASTER_UPLOADS_PER_SURFACE_CAPACITY]), epochs: [0; RASTER_UPLOADS_PER_SURFACE_CAPACITY]"),
    (CANVAS, "slots: semio_framework_async::boxed_fixed_slots(|| EngineSurfaceSlot { id: None, generation: 0, exhausted: false, value: None, retirement: None })",
             "slots: Box::new(std::array::from_fn(|_| EngineSurfaceSlot { id: None, generation: 0, exhausted: false, value: None, retirement: None }))"),
    (CANVAS, "slots: ManuallyDrop::new(Some(semio_framework_async::boxed_fixed_slots(EngineGpuSlot::new)))",
             "slots: ManuallyDrop::new(Some(Box::new(std::array::from_fn(|_| EngineGpuSlot::new()))))"),
]

mode = sys.argv[1]
for path, fixed, legacy in PAIRS:
    text = io.open(path, encoding="utf-8").read()
    src, dst = (fixed, legacy) if mode == "off" else (legacy, fixed)
    assert text.count(src) == 1, (path, src[:60], text.count(src))
    io.open(path, "w", encoding="utf-8").write(text.replace(src, dst))
print(f"toggled {mode}")
