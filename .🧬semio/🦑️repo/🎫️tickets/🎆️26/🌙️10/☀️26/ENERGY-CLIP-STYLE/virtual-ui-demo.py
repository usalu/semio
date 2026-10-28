"""🎥 Preview a physical-to-ventilation handoff with current authored clause frames."""

import gc
import importlib
import importlib.util
import os
import shutil
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import Scene, tempconfig

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
presentation = importlib.import_module(PACKAGE + ".🎬️clip.🪟️presentation.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_AUDIENCE"] = "spectator"
settings = {"quality": "-ql", "resolution": "960,540", "fps": 6, "language": "de", "audience": "spectator"}


class Captured(Exception):
    """⏹️ End the diagnostic scene after two settled narrated states."""


def capture(segment, folder):
    """📸 Render authored source states without changing its geometry or narration."""
    folder.mkdir(parents=True, exist_ok=True)
    runtime.render_config(folder)
    source = runtime.teaching_root(Path(runtime.__file__)) / segment["script"]
    spec = importlib.util.spec_from_file_location("virtual_demo_" + segment["scene"], source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    original_wait = Scene.wait
    images = []

    def settled(scene, *args, **kwargs):
        result = original_wait(scene, *args, **kwargs)
        if scene.mobjects:
            scene.camera.reset()
            scene.camera.capture_mobjects(scene.mobjects)
            path = folder / f"{len(images)}.png"
            scene.camera.get_image().save(path)
            images.append(path)
            if len(images) == 2:
                raise Captured
        return result

    try:
        Scene.wait = settled
        with tempconfig({"media_dir": str(folder), "text_dir": str(folder / "t"), "pixel_width": 1280,
                         "pixel_height": 720, "frame_rate": 6, "skip_animations": True,
                         "write_to_movie": False, "save_last_frame": False, "disable_caching": True,
                         "verbosity": "ERROR", "progress_bar": "none"}):
            try:
                getattr(module, segment["scene"])().render()
            except Captured:
                pass
    finally:
        Scene.wait = original_wait
        sys.modules.pop(spec.name, None)
        gc.collect()
    if len(images) != 2:
        raise RuntimeError("Expected two complete narrated states")
    return images


movies = []
videos = runtime.load_manifest()
for index, (video, chapter) in enumerate(((videos[0], 0), (videos[1], 1))):
    folder = output / str(index)
    body = folder / "body.mp4"
    if "--reuse-bodies" not in sys.argv or not body.is_file():
        images = capture(video["chapters"][chapter]["segments"][0], folder)
        runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-y",
                         "-loop", "1", "-i", str(images[0]), "-loop", "1", "-i", str(images[1]),
                         "-filter_complex", "[0:v]fps=6,trim=duration=2,setpts=PTS-STARTPTS[a];[1:v]fps=6,trim=duration=2,setpts=PTS-STARTPTS[b];[a][b]concat=n=2:v=1:a=0[v]",
                         "-map", "[v]", "-an", "-r", "6", "-fps_mode", "cfr", "-c:v", "libx264", "-pix_fmt", "yuv420p", str(body)],
                        log=folder / "body.log")
    movie = folder / "window.mp4"
    presentation.render_presentation(video["id"], body, movie, settings)
    movies.append(movie)
    print(f"[DEBUG] Virtual UI preview complete: {video['id']}", flush=True)
joined = output / "virtual-ui.mp4"
runtime.concat_videos(movies, joined, output / "join.txt")
final = Path(__file__).with_name("virtual-ui.mp4")
shutil.copy2(joined, final)
presentation.render_overview(output / "overview.png", settings)
shutil.copy2(output / "overview.png", Path(__file__).with_name("virtual-ui.png"))
print(f"[DEBUG] Virtual UI review film: {final}", flush=True)
