import importlib
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(root))
posters = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖼️posters.🐍️")
rows = []
for scene in posters.chapters():
    image, _ = posters.asset(scene)
    if not image.is_file():
        rows.append((scene, None, None))
        continue
    pixels = np.asarray(Image.open(image).convert("RGB"), dtype=int)
    height, width = pixels.shape[:2]
    area = pixels[int(height * 0.15):int(height * 0.85), int(width * 0.05):int(width * 0.95)]
    flat = area.reshape(-1, 3)
    keys, counts = np.unique(flat // 8, axis=0, return_counts=True)
    background = keys[counts.argmax()] * 8 + 4
    share = float((np.abs(flat - background).max(axis=1) > 24).mean())
    rows.append((scene, round(share, 4), image))
for scene, share, _ in sorted(rows, key=lambda row: (row[1] is None, row[1] or 0)):
    print(f"{share!s:8} {scene}")
sheet = Image.new("RGB", (6 * 320, 8 * 190), (30, 30, 30))
for index, (scene, share, image) in enumerate(rows):
    if image is None:
        continue
    tile = Image.open(image).convert("RGB").resize((320, 180))
    ImageDraw.Draw(tile).text((4, 2), f"{scene} {share}", fill=(255, 255, 0))
    sheet.paste(tile, ((index % 6) * 320, (index // 6) * 190))
sheet.save(Path(__file__).resolve().parent / "🗑️generated" / "contact-sheet.png")
