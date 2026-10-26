"""📦 Publish successful isolated chapters and assemble each ready complete lesson."""

import json
import os
from pathlib import Path
import shutil
import sys
import time

sys.dont_write_bytecode = True
TICKET = Path(__file__).resolve().parent
ROOT = Path.cwd() / "🎓️teaching"
sys.path.insert(0, str(ROOT))

import av
from video_runtime import clip_state, compose_video, load_manifest, reusable_clip

OUTPUT = TICKET / "🗑️generated/videos"
PRE = TICKET / "🗑️generated/pre"
SETTINGS = {"quality": "-ql", "resolution": None, "fps": None}
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)


def dependencies(video, intro):
    """📚 Match the product runtime's authored dependency selection exactly."""
    folder = ROOT / "intro" if intro else (ROOT / video["script"]).parent
    return [ROOT / "manim_fonts.py", ROOT / "manim_visuals.py",
            *(path for path in folder.rglob("*") if path.suffix in (".py", ".json", ".pdf", ".svg", ".png", ".jpg"))]


def validate_original(clip):
    """🔍 Decode every frame of a completed original clip before recording provenance."""
    with av.open(clip) as container:
        stream = container.streams.video[0]
        if (stream.width, stream.height, stream.base_rate) != (854, 480, 15) or abs(float(stream.average_rate) - 15) >= 0.001:
            raise RuntimeError(f"Unexpected original clip format: {clip}")
        frames = sum(1 for _ in container.decode(video=0))
        if not frames:
            raise RuntimeError(f"Original clip has no decodable frames: {clip}")
        return frames


def main():
    """🚦 Publish only successful provenance-checked jobs and assemble lessons as ready."""
    videos = load_manifest()
    mapping = json.loads((PRE / "jobs.json").read_text(encoding="utf-8"))
    staged = {job["scene"]: Path(job["media"]) / "v" / f"{job['scene']}.mp4" for job in mapping}
    publication = json.loads((PRE / "publication.json").read_text(encoding="utf-8")) if (PRE / "publication.json").is_file() else {}
    completed = publication.get("completed", {})
    original_frames = publication.get("original_frames", {})
    while len(completed) < len(videos):
        coverage = []
        for index, video in enumerate(videos, 1):
            names = [video["intro"], *video["scenes"]]
            ready = []
            for name in names:
                candidate = staged.get(name, OUTPUT / str(index) / "v" / f"{name}.mp4")
                known_original = name not in staged and name != "PhysicalFundamentals_FullSection"
                if candidate.is_file() and candidate.stat().st_size > 1000 and (known_original or candidate.with_suffix(".render.json").is_file()):
                    ready.append(name)
            coverage.append({"video": video["id"], "ready": len(ready), "expected": len(names), "assembled": video["id"] in completed})
            if video["id"] in completed or len(ready) != len(names):
                continue
            clips = []
            for name in names:
                state = clip_state(SETTINGS, dependencies(video, name == video["intro"]))
                source = staged.get(name, OUTPUT / str(index) / "v" / f"{name}.mp4")
                if name in staged or name == "PhysicalFundamentals_FullSection":
                    if not reusable_clip(source, state):
                        raise RuntimeError(f"Staged clip provenance differs: {source}")
                else:
                    original_frames[str(source)] = validate_original(source)
                destination = OUTPUT / str(index) / "v" / source.name
                destination.parent.mkdir(parents=True, exist_ok=True)
                if source != destination:
                    temporary = destination.with_suffix(".publish.mp4")
                    shutil.copy2(source, temporary)
                    temporary.replace(destination)
                destination.with_suffix(".render.json").write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")
                clips.append(destination)
            (OUTPUT / str(index) / "settings.json").write_text(json.dumps(SETTINGS, indent=2) + "\n", encoding="utf-8")
            result = compose_video(video["id"], quality_flag="-ql", concat_only=True)
            completed[video["id"]] = str(result)
            (PRE / "publication.json").write_text(json.dumps({"completed": completed, "original_frames": original_frames}, indent=2) + "\n", encoding="utf-8")
            print(f"[DEBUG] Assembled complete lesson {video['id']}: {result}", flush=True)
        (PRE / "coverage.json").write_text(json.dumps(coverage, indent=2) + "\n", encoding="utf-8")
        if len(completed) < len(videos):
            time.sleep(5)


if __name__ == "__main__":
    main()
