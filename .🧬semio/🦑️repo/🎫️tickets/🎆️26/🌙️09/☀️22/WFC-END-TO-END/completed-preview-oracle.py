"""🎨️ Independently rasterize the completed-preview palette and discard-marker contract."""
import json
from pathlib import Path
from PIL import Image, ImageDraw

root = Path(__file__).resolve()
while not (root / ".git").exists():
    root = root.parent
fixture = json.loads((root / "✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️completed-preview/🔣️.json").read_text(encoding="utf-8"))
palette = [(0, 0, 0), (255, 255, 255)]
discard = (255, 40, 160)
for case in fixture["cases"]:
    selected = case["assigned"] if case["done"] else case["traced"]
    marker = not case["done"] and case["discarded"]
    assert selected == case["expected"] and marker == case["showDiscard"], case["id"]
    image = Image.new("RGB", (10, 10), palette[selected])
    if marker:
        ImageDraw.Draw(image).rectangle((2, 2, 7, 7), fill=discard)
    assert image.getpixel((0, 0)) == palette[case["expected"]], case["id"]
    assert image.getpixel((5, 5)) == (discard if case["showDiscard"] else palette[case["expected"]]), case["id"]
print("validated 4 completed-preview palette/marker cases with Pillow")
