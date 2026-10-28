"""🖼️ Capture representative clause frames from all five styled energy series."""

import importlib
import gc
import importlib.util
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import Scene, tempconfig
from PIL import Image, ImageDraw

MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
runtime = importlib.import_module(MODULE + ".🎬️clip.⚙️runtime.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "qa"
output.mkdir(parents=True, exist_ok=True)
runtime.render_config(output)
frames = []


class PreviewComplete(Exception):
    """⏹️ End diagnostic rendering after two complete narrated clause states."""


original_wait = Scene.wait


def capture_wait(scene, *args, **kwargs):
    """📸 Record a settled narrated frame while Manim skips animation frames."""
    result = original_wait(scene, *args, **kwargs)
    count = getattr(scene, "_qa_count", 0)
    if len(scene.mobjects) and count < 2:
        scene.camera.reset()
        scene.camera.capture_mobjects(scene.mobjects)
        frame = scene.camera.get_image().copy()
        path = output / f"{scene._qa_id}-{count}.png"
        frame.save(path)
        frames.append((scene._qa_id, count, path))
        scene._qa_count = count + 1
        if scene._qa_count == 2:
            raise PreviewComplete
    return result


Scene.wait = capture_wait
for index, video in enumerate(runtime.load_manifest()):
    if len(sys.argv) > 1 and video["id"] not in sys.argv[1:]:
        continue
    segment = video["chapters"][0]["segments"][0]
    path = runtime.teaching_root(Path(runtime.__file__)) / segment["script"]
    spec = importlib.util.spec_from_file_location(f"qa_scene_{index}", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    runtime.render_config(output / video["id"])
    with tempconfig({"media_dir": str(output / str(index)), "text_dir": str(output / video["id"] / "t"), "tex_dir": str(output / "x"), "pixel_width": 1280, "pixel_height": 720, "frame_rate": 5, "skip_animations": True, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none"}):
        scene = getattr(module, segment["scene"])()
        scene._qa_id = video["id"]
        try:
            scene.render()
        except PreviewComplete:
            pass
    print(f"[DEBUG] QA captured {video['id']}: {segment['scene']}", flush=True)
    sys.modules.pop(spec.name, None)
    del scene, module
    gc.collect()
selected = []
for video in runtime.load_manifest():
    candidates = [(video["id"], count, output / f"{video['id']}-{count}.png") for count in (0, 1) if (output / f"{video['id']}-{count}.png").is_file()]
    if candidates:
        selected.extend([candidates[0], candidates[len(candidates) // 2]])
contact = Image.new("RGB", (1280, 390 * ((len(selected) + 1) // 2)), "#001117")
draw = ImageDraw.Draw(contact)
for index, (name, count, path) in enumerate(selected):
    image = Image.open(path).resize((640, 360))
    x, y = index % 2 * 640, index // 2 * 390
    contact.paste(image, (x, y + 30))
    draw.text((x + 12, y + 8), f"{name} · clause {count}", fill="#f7f3e3")
contact.save(output / "contact.png")
print(f"[DEBUG] QA contact: {output / 'contact.png'}", flush=True)
