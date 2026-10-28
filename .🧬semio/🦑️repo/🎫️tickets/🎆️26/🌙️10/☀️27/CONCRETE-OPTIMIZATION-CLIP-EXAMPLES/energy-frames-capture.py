"""📸 Keyframes of the energy teaching clips: replay each beat with skipped animations and save the frame at chosen clause keys.

Usage: python energy-frames-capture.py  (writes to 🗑️generated/energy-frames/<SceneClass>-<key>.png)
Replaces hold_for in the shared visuals module before the clip modules import it, so every clause hold saves a still.
"""
import importlib
import importlib.util
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
OUT = TICKET / "🗑️generated/energy-frames"
OUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
ENERGY = ROOT / "🎓️teaching/🏛️architecture/⚡️energy"
CLIPS = {
    "demand": ENERGY / "📊️demand/🎬️clip/⚡️demand-power/🐍️.py",
    "solar": ENERGY / "🔥️heating/🎬️clip/☀️solar-gains/🐍️.py",
    "summer": ENERGY / "❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py",
    "sizing": ENERGY / "❄️cooling/🎬️clip/📐️system-sizing/🐍️.py",
    "calculation": ENERGY / "🔥️heating/🎬️clip/🧮️calculation/🐍️.py",
}
JOBS = [
    ("demand", "Beat1_LeistungUndEnergie", ["jahr", "summe", "trennung"]),
    ("solar", "Beat1_VerlustZuGewinn", ["gain"]),
    ("solar", "Beat5_Verschattung", ["raffstore"]),
    ("solar", "Beat8_Hauptgleichung", ["chain"]),
    ("summer", "Beat1_LastZuerstSenken", ["envelope", "natural", "reserve"]),
    ("sizing", "Beat2_VolumeFlowEquation", ["formula"]),
    ("sizing", "Beat3_IsolateAirflow", ["compare", "duct"]),
    ("calculation", "AnlagenVerluste", ["chain", "d", "sum"]),
]
WANTED: dict[str, set[str]] = {}
LAYOUT: list[str] = []
_original_hold = art.hold_for


def _snap(scene, key: str) -> None:
    """📸 Save the current frame of a scene under its class name and clause key."""
    name = type(scene).__name__
    scene.renderer.static_image = None
    scene.renderer.update_frame(scene, mobjects=scene.mobjects, ignore_skipping=True)
    scene.renderer.get_frame()
    scene.renderer.camera.get_image().save(OUT / f"{name}-{key}.png")
    conflicts = art.layout_conflicts(scene)
    LAYOUT.append(f"## {name} {key}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")
    print("[DEBUG] saved", name, key, flush=True)


def _hold_and_snap(scene, narration, key, **kwargs):
    """📸 Clause hold that first saves the still of the clause it is about to play."""
    if key in WANTED.get(type(scene).__name__, set()):
        _snap(scene, key)
    return _original_hold(scene, narration, key, **kwargs)


art.hold_for = _hold_and_snap


def _load(path: Path, name: str):
    """📦 Import one energy clip module from its file, so the emoji folder names stay out of import paths."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


modules = {key: _load(path, "energy_clip_" + key) for key, path in CLIPS.items()}
for clip, class_name, keys in JOBS:
    WANTED[class_name] = set(keys)
    scene_cls = getattr(modules[clip], class_name)
    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False,
                           "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                           "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
        scene_cls().render()
    print("[DEBUG] rendered", class_name, flush=True)
(OUT / "layout-conflicts.md").write_text("# Layout conflicts in captured energy frames\n\n" + "\n".join(LAYOUT), encoding="utf-8")
print("[DEBUG] done", len(LAYOUT), "frames", flush=True)
