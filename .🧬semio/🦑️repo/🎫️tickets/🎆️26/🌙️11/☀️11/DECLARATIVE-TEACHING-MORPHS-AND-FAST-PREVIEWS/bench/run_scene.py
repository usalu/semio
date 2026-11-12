"""📏 Instrumented single-scene Manim render for morph and preview benchmarks; edits no sources.

Usage: python run_scene.py --script <path under 🎓️teaching> --scene <Class> --out <output root> --label <name>
       [--quality l|h] [--fps N] [--language en|de] [--stop-after-morphs N] [--stop-after-seconds S] [--stop-after-plays N]
Runs exactly like the teaching renderer (manim CLI, same config file layout, SEMIO_TEACHING_* env) inside this process, with
monkeypatched timing on Scene.play/Scene.wait, constructor counters for the morph toolkit and Manim transforms, a RSS
sampler and a first-frame timestamp. Writes <out>/bench-<label>.json. Partial renders end through EndSceneEarlyException,
so Manim still finalizes the movie.
https://docs.manim.community/en/stable/reference/manim.scene.scene.Scene.html
"""
import argparse
import importlib
import json
import os
import sys
import threading
import time
from collections import Counter
from pathlib import Path

START = time.perf_counter()
ROOT = Path(os.environ["BENCH_ROOT"]) if os.environ.get("BENCH_ROOT") else next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
parser = argparse.ArgumentParser()
parser.add_argument("--script", required=True)
parser.add_argument("--scene", required=True)
parser.add_argument("--out", required=True)
parser.add_argument("--label", required=True)
parser.add_argument("--quality", default="l")
parser.add_argument("--fps", type=int)
parser.add_argument("--language", default="en")
parser.add_argument("--stop-after-morphs", type=int, default=0)
parser.add_argument("--stop-after-seconds", type=float, default=0.0)
parser.add_argument("--stop-after-plays", type=int, default=0)
args = parser.parse_args()
out = Path(args.out).resolve()
out.mkdir(parents=True, exist_ok=True)
media = out / "m"
script = Path(args.script) if Path(args.script).is_absolute() else ROOT / "🎓️teaching" / args.script
os.environ.update({"SEMIO_TEACHING_OUTPUT_ROOT": str(out), "VO_TRACE": "1", "SEMIO_TEACHING_LANGUAGE": args.language, "SEMIO_TEACHING_AUDIENCE": "speaker", "PYTHONUTF8": "1"})
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
try:
    os.environ["SEMIO_TEACHING_SPEECH_ROOT"] = str(runtime.source_speech_root(script, language=args.language).parent)
except Exception:
    pass
config = runtime.render_config(media)
t_import_done = time.perf_counter()

import psutil
import manim
from manim.scene.scene import Scene
from manim.utils.exceptions import EndSceneEarlyException
from manim.scene.scene_file_writer import SceneFileWriter

morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
proc = psutil.Process()
state = {"peak_rss": 0, "peak_tree_rss": 0, "samples": [], "first_frame": None, "first_play": None, "plays": [], "waits": [], "morph_plays": 0, "stopped": None}
counts = Counter()
stop = threading.Event()


def cpu_seconds():
    t = proc.cpu_times()
    return round(t.user + t.system, 2)


def sampler():
    while not stop.is_set():
        try:
            rss = proc.memory_info().rss
            tree = rss + sum(c.memory_info().rss for c in proc.children(recursive=True))
            state["peak_rss"] = max(state["peak_rss"], rss)
            state["peak_tree_rss"] = max(state["peak_tree_rss"], tree)
            state["samples"].append((round(time.perf_counter() - START, 2), rss, psutil.cpu_percent(None)))
        except psutil.Error:
            pass
        stop.wait(0.25)


def count_class(cls, key):
    original = cls.__init__

    def wrapped(self, *a, **k):
        counts[key] += 1
        return original(self, *a, **k)

    cls.__init__ = wrapped


MORPH_CLASSES = ("Morph", "PartMorph", "Flight", "Swarm", "Dim")
for name in MORPH_CLASSES:
    count_class(getattr(morph, name), name)
for name in ("Transform", "ReplacementTransform", "TransformMatchingShapes", "TransformFromCopy", "FadeTransform"):
    if hasattr(manim, name):
        count_class(getattr(manim, name), "manim." + name)
original_plan = morph._plan


def counted_plan(*a, **k):
    counts["_plan"] += 1
    began = time.perf_counter()
    try:
        return original_plan(*a, **k)
    finally:
        counts["_plan_seconds_x1000"] += int((time.perf_counter() - began) * 1000)


morph._plan = counted_plan
MORPHISH = {"Morph", "PartMorph", "Flight", "Swarm", "Transform", "ReplacementTransform", "TransformMatchingShapes", "TransformFromCopy", "FadeTransform"}


def names_of(animation, depth=0):
    result = [type(animation).__name__]
    if depth < 3:
        for child in getattr(animation, "animations", ()) or ():
            result += names_of(child, depth + 1)
    return result


original_write = SceneFileWriter.write_frame


def timed_write(self, *a, **k):
    if state["first_frame"] is None:
        state["first_frame"] = time.perf_counter() - START
        state["cpu_first_frame"] = cpu_seconds()
    return original_write(self, *a, **k)


SceneFileWriter.write_frame = timed_write
original_play = Scene.play
original_wait = Scene.wait


def timed_play(self, *animations, **kwargs):
    now = time.perf_counter()
    state["first_play"] = state["first_play"] or now - START
    state.setdefault("cpu_first_play", cpu_seconds())
    names = [n for a in animations for n in names_of(a)]
    morphish = any(n in MORPHISH for n in names)
    before_time = self.renderer.time
    result = original_play(self, *animations, **kwargs)
    elapsed = time.perf_counter() - now
    index = len(state["plays"])
    state["plays"].append({"i": index, "kinds": sorted(set(names)), "morph": morphish, "wall_s": round(elapsed, 4), "scene_s": round(self.renderer.time - before_time, 4), "at_scene_s": round(before_time, 4)})
    print(f"[DEBUG] play {index} {names[:3]} morph={morphish} wall={elapsed:.2f}s scene_t={self.renderer.time:.2f}s t={time.perf_counter() - START:.0f}s", flush=True)
    if morphish:
        state["morph_plays"] += 1
    check_stop(self)
    return result


def timed_wait(self, *a, **k):
    now = time.perf_counter()
    before_time = self.renderer.time
    result = original_wait(self, *a, **k)
    state["waits"].append({"wall_s": round(time.perf_counter() - now, 4), "scene_s": round(self.renderer.time - before_time, 4)})
    check_stop(self)
    return result


def check_stop(scene):
    reasons = []
    if args.stop_after_morphs and state["morph_plays"] >= args.stop_after_morphs:
        reasons.append(f"morphs>={args.stop_after_morphs}")
    if args.stop_after_plays and len(state["plays"]) >= args.stop_after_plays:
        reasons.append(f"plays>={args.stop_after_plays}")
    if args.stop_after_seconds and scene.renderer.time >= args.stop_after_seconds:
        reasons.append(f"scene_s>={args.stop_after_seconds}")
    if reasons:
        state["stopped"] = ",".join(reasons)
        raise EndSceneEarlyException()


Scene.play = timed_play
Scene.wait = timed_wait
quality = {"l": "-ql", "m": "-qm", "h": "-qh"}[args.quality]
argv = ["manim", "--config_file", str(config), quality, "--media_dir", str(media)]
if args.fps:
    argv += ["--fps", str(args.fps)]
argv += ["--disable_caching", str(script), args.scene]
sys.argv = argv
psutil.cpu_percent(None)
threading.Thread(target=sampler, daemon=True).start()
t_render = time.perf_counter()
code = 0
try:
    from manim.__main__ import main
    main()
except SystemExit as exit_:
    code = exit_.code or 0
stop.set()
end = time.perf_counter()
movie = next(iter(sorted((media / "v").glob(f"{args.scene}.mp4"))), None)
info = {}
if movie and movie.is_file():
    import av
    with av.open(str(movie)) as container:
        stream = container.streams.video[0]
        frames = sum(1 for _ in container.demux(stream) if _.size)
        info = {"path": str(movie), "bytes": movie.stat().st_size, "width": stream.codec_context.width, "height": stream.codec_context.height,
                "fps": float(stream.average_rate), "frames": frames, "duration_s": float(container.duration / 1e6) if container.duration else None}
cpu = [s[2] for s in state["samples"]]
result = {
    "label": args.label, "script": args.script, "scene": args.scene, "quality": args.quality, "fps_arg": args.fps, "language": args.language,
    "stop": {"morphs": args.stop_after_morphs, "plays": args.stop_after_plays, "scene_s": args.stop_after_seconds, "hit": state["stopped"]},
    "exit": code,
    "seconds": {"imports_and_config": round(t_import_done - START, 2), "to_first_play": round(state["first_play"] or 0, 2),
                "to_first_frame": round(state["first_frame"] or 0, 2), "render_total": round(end - t_render, 2), "process_total": round(end - START, 2)},
    "morph_constructor_counts": dict(counts), "plan_seconds": counts.get("_plan_seconds_x1000", 0) / 1000,
    "plays": len(state["plays"]), "morph_plays": state["morph_plays"],
    "play_wall_total_s": round(sum(p["wall_s"] for p in state["plays"]), 2), "wait_wall_total_s": round(sum(w["wall_s"] for w in state["waits"]), 2),
    "peak_rss_mb": round(state["peak_rss"] / 2**20), "peak_tree_rss_mb": round(state["peak_tree_rss"] / 2**20),
    "mean_cpu_pct_system": round(sum(cpu) / len(cpu), 1) if cpu else None,
    "process_cpu_seconds": {"to_first_play": state.get("cpu_first_play"), "to_first_frame": state.get("cpu_first_frame"), "total": cpu_seconds()},
    "movie": info, "play_log": state["plays"],
}
(out / f"bench-{args.label}.json").write_text(json.dumps(result, indent=1, ensure_ascii=False), encoding="utf-8")
print("[DEBUG] bench", json.dumps({k: result[k] for k in ("label", "seconds", "morph_constructor_counts", "plays", "morph_plays", "peak_tree_rss_mb", "movie", "stop")}, ensure_ascii=False), flush=True)
sys.exit(code)
