import json
from pathlib import Path

from PIL import Image

root = Path.cwd()
fixture = json.loads((root / "✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️bitmap-pinned-output/🔣️.json").read_text(encoding="utf-8"))
case = next(row for row in fixture["cases"] if row["id"] == "complete-unordered")
image = Image.new("P", (fixture["width"], fixture["height"]))
for x, y, color in case["pins"]:
    image.putpixel((x, y), color)
assert list(image.tobytes()) == case["indices"]
print("Pillow independently confirms the complete pinned output's row-major pixel indices.")
