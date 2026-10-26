"""🧪 Compare full Manim output with retained-prefix plus animation-number resume."""
import hashlib
import json
from pathlib import Path
import sys
import av

TICKET = Path(__file__).resolve().parent
ROOT = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(ROOT))
from video_runtime import concat_videos, manim_command, render_config, run_command

CASES = json.loads((TICKET / "recovery-cases.json").read_text(encoding="utf-8"))
OUTPUT = TICKET / "🗑️generated" / "recovery"
OUTPUT.mkdir(parents=True, exist_ok=True)


def hashes(path):
    with av.open(str(path)) as container:
        return [hashlib.sha256(frame.to_ndarray(format="rgb24").tobytes()).hexdigest() for frame in container.decode(video=0)]


runs = {}
for name, extra in (("reference", []), ("prefix", ["-n", "0,1"]), ("remainder", ["-n", str(CASES["resume_animation"])])):
    directory = OUTPUT / name
    config = render_config(directory)
    trace = OUTPUT / f"{name}.json"
    run_command([*manim_command(), "--config_file", str(config), "-ql", "--resolution", ",".join(map(str, CASES["resolution"])), "--fps", str(CASES["fps"]), "--disable_caching", *extra, str(TICKET / "recovery-scene.py"), CASES["scene"]], cwd=ROOT, env={"RECOVERY_TEACHING_ROOT": str(ROOT), "RECOVERY_TRACE": str(trace)})
    runs[name] = {"directory": directory, "trace": json.loads(trace.read_text(encoding="utf-8")), "frames": hashes(directory / "v" / f"{CASES['scene']}.mp4")}

prefix = runs["prefix"]["directory"] / "p" / CASES["scene"]
remainder = runs["remainder"]["directory"] / "p" / CASES["scene"]
retained = [prefix / f"uncached_{index:05}.mp4" for index in range(CASES["resume_animation"])]
resumed = sorted(remainder.glob("uncached_*.mp4"))
recovered = OUTPUT / "recovered.mp4"
concat_videos([*retained, *resumed], recovered, OUTPUT / "concat.txt")
reference_hashes = runs["reference"]["frames"]
recovered_hashes = hashes(recovered)
first_difference = next((index for index, (left, right) in enumerate(zip(reference_hashes, recovered_hashes)) if left != right), None)
boundary = len(runs["prefix"]["frames"])
result = {"manim": __import__("manim").__version__, "case": CASES, "reference_frames": len(reference_hashes), "prefix_frames": boundary, "remainder_frames": len(runs["remainder"]["frames"]), "recovered_frames": len(recovered_hashes), "exact_frames": reference_hashes == recovered_hashes, "first_differing_frame": first_difference, "boundary": boundary, "boundary_equal": {str(offset): (reference_hashes[boundary + offset] == recovered_hashes[boundary + offset]) for offset in (-1, 0, 1) if 0 <= boundary + offset < min(len(reference_hashes), len(recovered_hashes))}, "traces": {name: run["trace"] for name, run in runs.items()}}
(OUTPUT / "result.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print("[DEBUG] Recovery proof:", json.dumps(result))
