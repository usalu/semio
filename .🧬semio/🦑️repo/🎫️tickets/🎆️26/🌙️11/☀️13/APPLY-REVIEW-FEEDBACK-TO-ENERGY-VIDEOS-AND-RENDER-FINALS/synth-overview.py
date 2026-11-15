import hashlib
import importlib
import os
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(root))
out = Path(__file__).resolve().parent / "🗑️generated" / "synth"
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(out)
package = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
hierarchy = importlib.import_module(package + ".🥞️hierarchy.🐍️")
posters = importlib.import_module(package + ".🖼️posters.🐍️")
presentation = importlib.import_module(package + ".🐍️")
from PIL import Image, ImageDraw

mode = sys.argv[1]
chapter = sys.argv[2]
settings = {"language": "de", "profile": "preview", "resolution": None, "fps": None, "audience": "speaker"}
directory = out / "posters"
directory.mkdir(parents=True, exist_ok=True)
scenes = list(posters.chapters()) if mode == "all" else posters.required(hierarchy.path(chapter))
print("scenes", len(scenes))
made = {}
for scene in scenes:
    path = directory / f"{scene}.png"
    image = Image.new("RGB", (854, 480), (40 + hash(scene) % 150, 90, 120))
    ImageDraw.Draw(image).text((20, 200), scene, fill=(255, 255, 255))
    image.save(path)
    made[scene] = {"file": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
with hierarchy.capture_catalog():
    hierarchy.capture_posters(made)
    for t in (0, 0.4):
        result = presentation.render_interaction_frame(chapter, "focus", t, out / f"frame-{mode}-{t}.png", settings)
        print(result["path"], [layer["id"] + ":" + ",".join(card["id"] for card in layer["cards"]) for layer in result["browser"]["layers"]])
