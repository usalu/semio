import json
import os
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(r"C:\git\semio\🧰️framework\🔨️modules")
catalog = json.loads((ROOT / "🖼️assets" / "🔤️fonts" / "📇️catalog.json").read_text(encoding="utf-8"))
paths = {}
for family in catalog["families"]:
    subset = next(part for part in family["subsets"] if part["sourceSubset"] in ("latin", None))
    paths[family["family"]] = ROOT / "🖼️assets" / "🔤️fonts" / family["directory"] / subset["directory"] / "📖️regular" / catalog["encodings"]["ttf"]
paths["Libertinus Math"] = ROOT / "📚️compiler" / "🌍️world" / "🔤️fonts" / "🧮️LibertinusMath-Regular.otf"
image = Image.new("RGB", (1400, 130 * len(paths)), (0, 20, 30))
draw = ImageDraw.Draw(image)
for row, (name, path) in enumerate(paths.items()):
    try:
        font = ImageFont.truetype(str(path), 56)
        draw.text((20, 20 + row * 130), f"{name}: (20 °C) [Vergleich] (seit 7/2026)", font=font, fill=(255, 255, 255))
    except Exception as error:
        draw.text((20, 20 + row * 130), f"{name}: {error}", fill=(255, 80, 80))
out = Path(__file__).resolve().parents[1] / "🗑️generated" / "consistency-out" / "paren-fonts.png"
image.save(out)
print("saved", os.fspath(out).encode("ascii", "replace").decode())
