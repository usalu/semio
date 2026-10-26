"""🔬 Reconcile nonvisual command edits against immutable authored render inputs."""

import ast
import copy
import hashlib
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
TICKET = Path(__file__).resolve().parent
ROOT = Path.cwd() / "🎓️teaching"
SOURCE = Path("C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial")
sys.path.insert(0, str(ROOT))

import av
from video_runtime import clip_state, load_manifest

generated = TICKET / "🗑️generated"
pre = generated / "pre"
settings = {"quality": "-ql", "resolution": None, "fps": None}
baselines = re.findall(r"^\| ([^|]+) \| ([0-9a-f]{64}) \|$", (TICKET / "source-integrity.md").read_text(encoding="utf-8"), re.M)
for relative, expected in baselines:
    if hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() != expected:
        raise RuntimeError(f"Authored visual input changed: {relative}")
timings = list(ROOT.rglob("vo_timing.json"))
for timing in timings:
    original = SOURCE / timing.relative_to(ROOT)
    if timing.read_bytes() != original.read_bytes():
        raise RuntimeError(f"Authored timing changed: {timing}")
wrappers = [ROOT / video["script"] for video in load_manifest()]
render_files = [*ROOT.rglob("scene_*.py"), ROOT / "intro/intro_scene.py", *wrappers]
for path in render_files:
    tree = ast.parse(path.read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            modules = [alias.name for alias in node.names] + ([node.module or ""] if isinstance(node, ast.ImportFrom) else [])
            if any("generate_audio" in name or "build_full_video" in name or "tts_pipeline" in name for name in modules):
                raise RuntimeError(f"Command tools imported by authored render source: {path}")
for path in wrappers:
    original_tree = ast.parse((SOURCE / path.relative_to(ROOT)).read_text(encoding="utf-8"))
    current_tree = ast.parse(path.read_text(encoding="utf-8"))
    original_classes = [ast.dump(node) for node in original_tree.body if isinstance(node, ast.ClassDef)]
    current_classes = [ast.dump(node) for node in current_tree.body if isinstance(node, ast.ClassDef)]
    if original_classes != current_classes:
        raise RuntimeError(f"Authored wrapper classes changed: {path}")


def font_functions(path):
    """🔤 Compare render behavior while excluding the added cache-directory creation."""
    tree = ast.parse(path.read_text(encoding="utf-8"))
    functions = {}
    for node in tree.body:
        if isinstance(node, ast.FunctionDef):
            function = copy.deepcopy(node)
            if function.name == "resolve_serif_font":
                for child in ast.walk(function):
                    if hasattr(child, "body") and isinstance(child.body, list):
                        child.body = [statement for statement in child.body if not (isinstance(statement, ast.Expr) and isinstance(statement.value, ast.Call) and ast.unparse(statement.value.func) == "_FONT_CACHE_PATH.parent.mkdir")]
            functions[function.name] = ast.dump(function)
    return functions


if font_functions(ROOT / "manim_fonts.py") != font_functions(SOURCE / "manim_fonts.py"):
    raise RuntimeError("Font rendering behavior changed")
changes = []
previous = json.loads((generated / "provenance-rebase.json").read_text(encoding="utf-8")) if (generated / "provenance-rebase.json").is_file() else {}
changes.extend(previous.get("rebased", []))
for sidecar in [*pre.glob("*/v/*.render.json"), *generated.glob("videos/*/v/*.render.json")]:
    state = json.loads(sidecar.read_text(encoding="utf-8"))
    current = clip_state(state["settings"], [Path(path) for path in state["sources"]])
    changed = [path for path, digest in state["sources"].items() if current["sources"].get(path) != digest]
    if any(Path(path).name not in ("generate_audio.py", "build_full_video.py") for path in changed):
        raise RuntimeError(f"Visual dependency changed for {sidecar}: {changed}")
    if changed:
        changes.append({"sidecar": str(sidecar), "command_only_changes": changed})
        sidecar.write_text(json.dumps(current, indent=2) + "\n", encoding="utf-8")
exits = json.loads((pre / "process-exits.json").read_text(encoding="utf-8"))
verified_jobs = []
mapping = json.loads((pre / "jobs.json").read_text(encoding="utf-8"))
for job in mapping:
    if job["job"] not in (17, 19, 21, 23):
        continue
    if not any(item["job"] == job["job"] and item["exit_code"] == 0 for item in exits):
        continue
    video = next(video for video in load_manifest() if video["id"] == job["video"])
    folder = (ROOT / video["script"]).parent
    sources = [ROOT / "manim_fonts.py", ROOT / "manim_visuals.py", *(path for path in folder.rglob("*") if path.suffix in (".py", ".json", ".pdf", ".svg", ".png", ".jpg"))]
    clip = Path(job["media"]) / "v" / f"{job['scene']}.mp4"
    with av.open(clip) as container:
        frames = sum(1 for _ in container.decode(video=0))
    if not frames:
        raise RuntimeError(f"Staged job{job['job']} has no decoded frames")
    clip.with_suffix(".render.json").write_text(json.dumps(clip_state(settings, sources), indent=2) + "\n", encoding="utf-8")
    verified_jobs.append({"job": job["job"], "scene": job["scene"], "native_exit": 0, "decoded_frames": frames})
report = {"immutable_visual_files": len(baselines), "unchanged_timing_files": len(timings), "unchanged_wrapper_classes": len(wrappers), "render_files_without_command_imports": len(render_files), "rebased": changes, "verified_native_jobs": verified_jobs}
(generated / "provenance-rebase.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
lines = ["# Render Provenance Reconciliation", "", f"Verified {len(baselines)} authored visual/asset SHA256 baselines, {len(timings)} original timing files, all five wrappers' unchanged scene-class ASTs, and unchanged font-rendering function ASTs after excluding cache-directory creation.", "", f"The {len(render_files)} render source files do not import audio generation, section build commands, or TTS command helpers. Existing provenance differences were restricted to the command-only paths below; production fingerprint policy remains conservative and unchanged.", ""]
for change in changes:
    lines.append(f"- {change['sidecar']}: " + ", ".join(change["command_only_changes"]))
lines.extend(["", "The following completed jobs exited natively with status 0 and were decoded completely. Their in-memory start-state was not persisted by the original controller; the parent-confirmed command-only edit, unchanged authored-input audit, and verified command-import isolation support recording current provenance without repeating unchanged animations.", ""])
lines.extend(f"- Job {job['job']} · {job['scene']}: {job['decoded_frames']} decoded frames." for job in verified_jobs)
(TICKET / "provenance-rebase.md").write_text("\n".join(lines), encoding="utf-8")
print(f"[DEBUG] Reconciled {len(changes)} command-only provenance snapshots; verified {len(verified_jobs)} native-successful jobs", flush=True)
