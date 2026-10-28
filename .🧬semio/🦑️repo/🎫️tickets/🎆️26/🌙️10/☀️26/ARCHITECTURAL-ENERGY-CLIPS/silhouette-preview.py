"""🖼️ Inspect production silhouettes at architectural and detail scales."""

import importlib
from pathlib import Path
import sys

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
import numpy as np
from manim import Camera
from PIL import Image, ImageDraw

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
A = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🐍️")
OUT = Path(__file__).resolve().parent / "🗑️generated/silhouettes"
OUT.mkdir(parents=True, exist_ok=True)
sheet = Image.new("RGB", (1800, 2000), "#171717")
draw = ImageDraw.Draw(sheet)
figures = [("Standing", V.person_glyph([0, 0, 0], color="#FFFFFF")),
           ("Seated", V.seated_person_glyph([0, 0, 0], color="#FFFFFF")["group"])]
figures += [(kind, A._activity_figure(kind, color="#FFFFFF")[0]) for kind in ("sleep", "walk", "sprint")]
figures += [(kind, V.object_silhouette(kind, color="#FFFFFF")) for kind in ("laptop", "bulb", "toaster", "vacuum", "heater")]
figures += [(kind, V.object_silhouette(kind, color="#FFFFFF")) for kind in ("tree", "factory", "flame", "sun", "moon", "monitor", "microbe")]
for i, (name, figure) in enumerate(figures):
    width, height = 360, 460
    span = max(figure.height, figure.width * height / width) * 1.25
    camera = Camera(pixel_width=width, pixel_height=height, frame_width=span * width / height,
                    frame_height=span, frame_center=figure.get_center(), background_color="#171717")
    camera.capture_mobjects([figure])
    x, y = i % 5 * 360, i // 5 * 500
    sheet.paste(camera.get_image().convert("RGB"), (x, y + 30))
    draw.text((x + 24, y + 12), name, fill="#EEEEEE")
sheet.save(OUT / "artwork.png")
print("[DEBUG] Production artwork contact sheet saved", flush=True)
