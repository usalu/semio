"""🌐 One static browser still of the opened video window: a red-bordered body image at the body box size shows that it fills the frame edge to edge below the chip row. Usage: run through pyrun.sh from the repo root."""
import importlib
import json
import os
import sys
from pathlib import Path

from PIL import Image, ImageDraw

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
synthetic = importlib.import_module(PACKAGE + ".🖼️posters.🧪️tests.🧰️synthetic.🐍️")

out = Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
contract = synthetic.catalog()
viewport = contract["focus"]["viewport"]
body = out / "body.png"
size = (round(viewport["width"]), round(viewport["height"]))
image = Image.new("RGB", size, "#2a6f7a")
draw = ImageDraw.Draw(image)
draw.rectangle([0, 0, size[0] - 1, size[1] - 1], outline="#ff3b30", width=3)
draw.line([0, 0, size[0], size[1]], fill="#fccf05", width=2)
draw.line([0, size[1], size[0], 0], fill="#fccf05", width=2)
image.save(body)
video = hierarchy.ordered_chapters()[0]
frame = {"output": str(out / "browser-focus.png"), "media": [{"id": video["id"], "body_image": str(body), "body_opacity": 1}],
         "hierarchy": {"opened_path": hierarchy.path(video["id"]), "revealed_path": [], "camera": {"x": 0, "y": 0}},
         "cursor": {"point": None, "kind": "arrow", "pressed": False}}
batch = {"schema": "semio-presentation-browser/v1", "catalog": contract, "work_directory": str(out), "language": "en",
         "viewport": {"width": 1600, "height": 900}, "frames": [frame]}
result = browser.render_batch(batch)
window = next(window for window in result["metadata"][0]["windows"] if window["id"] == video["id"])
print(json.dumps({"rect": window["rect"], "content_rect": window["content_rect"], "title_rect": window["title_rect"], "viewport": viewport}))
