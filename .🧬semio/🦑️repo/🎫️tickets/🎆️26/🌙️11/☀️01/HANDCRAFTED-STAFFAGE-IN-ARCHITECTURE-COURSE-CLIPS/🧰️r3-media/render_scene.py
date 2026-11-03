"""🎞️ Render one source scene of a manifest video through the canonical segment renderer (narration, guard, success-only provenance) into the course slot.

Usage: python render_scene.py <en|de> <video-id> <Scene>... [--force]
The clip lands at videos/<lang>/speaker/s/<source_ordinal>/v/<Scene>.mp4 exactly as `script.ts render` would write it, so a later
`render_video.sh` reuses it. Run through `run.sh`; one scene after the other, never in parallel for the same language.
"""
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib

language, video, *scenes = [argument for argument in sys.argv[1:] if not argument.startswith("--")]
force = "--force" in sys.argv
runtime = lib.runtime()
config = lib.settings(language, "speaker", lib.QUALITY, lib.FPS)
folder = lib.base(language, "speaker")
plan = {job["segment"]["scene"]: job for job in runtime.segment_plan() if job["video_id"] == video}
for scene in scenes:
    job = plan[scene]
    started = time.time()
    clip = runtime.render_segment(job["segment"], folder / "s" / str(job["source_ordinal"]), config, force=force, output_root=folder, log_output=True)
    fresh = runtime.reusable_clip(clip, runtime.segment_state({key: job["segment"][key] for key in ("script", "scene")}, config))
    print(f"[DEBUG] {video}/{scene} -> {clip} fresh={fresh} in {time.time() - started:.0f} s", flush=True)
