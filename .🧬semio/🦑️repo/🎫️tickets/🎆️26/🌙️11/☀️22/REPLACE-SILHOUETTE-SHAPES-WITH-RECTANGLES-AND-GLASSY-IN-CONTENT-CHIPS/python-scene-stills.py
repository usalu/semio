"""🖼️ Low-quality last-frame still of one authored chapter scene, rendered by the poster route's own command (`manim --save_last_frame`, 854x480 at 15 fps) and written only into the ticket's generated folder; never stores a poster asset."""
import importlib
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
OUT = TICKET / "🗑️generated" / "stills"
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"


def still(scene: str, language: str = "en") -> Path:
    """📸 Render one chapter's poster-segment last frame at low quality and return the PNG path."""
    posters = importlib.import_module(PACKAGE + ".🪟️presentation.🖼️posters.🐍️")
    runtime = importlib.import_module(PACKAGE + ".⚙️runtime.🐍️")
    profile = importlib.import_module(PACKAGE + ".🎚️profile.🐍️")
    info = posters.chapters()[scene]
    segment, animation = info["segment"], info["declaration"]["animation"]
    teaching = runtime.teaching_root(Path(runtime.__file__))
    work = OUT / scene
    media = work / "m"
    resolution, fps = "854,480", "15"
    command = [*runtime.manim_command(), "-ql", "-s", "--config_file", str(runtime.render_config(media)), "--media_dir", str(media),
               "--resolution", resolution, "--fps", fps, "--disable_caching", "--save_last_frame",
               *([] if animation is None else ["--from_animation_number", f"0,{animation}"]),
               str(teaching / segment["script"]), segment["scene"]]
    environment = {**os.environ, "SEMIO_TEACHING_OUTPUT_ROOT": str(work),
                   "SEMIO_TEACHING_SPEECH_ROOT": str(runtime.source_speech_root(teaching / segment["script"], language=language).parent),
                   "SEMIO_TEACHING_LANGUAGE": language, "SEMIO_TEACHING_AUDIENCE": "speaker", "SEMIO_TEACHING_WORK": str(media / "work.json"),
                   **profile.environment("final", job_id=f"still-{scene}", fps=float(fps), resolution=resolution)}
    print(f"[DEBUG] still {scene}: {segment['script']} {segment['scene']}", flush=True)
    subprocess.run(command, cwd=teaching, env=environment, check=True)
    frames = sorted((path for path in media.rglob("*.png") if "t" not in path.relative_to(media).parts[:1]), key=lambda path: path.stat().st_mtime)
    if not frames:
        raise FileNotFoundError(f"[DEBUG] no frame produced for {scene} under {media}")
    return frames[-1]


if __name__ == "__main__":
    print(still(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else "en"), flush=True)
