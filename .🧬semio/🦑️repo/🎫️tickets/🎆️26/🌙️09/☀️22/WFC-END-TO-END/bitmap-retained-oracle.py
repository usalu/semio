"""🧪️ Independent Pillow oracle for the portable retained bitmap cases."""
import json
from pathlib import Path
from PIL import Image

root = Path(__file__).resolve().parents[7]
fixture = root / "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/retained.json"
law = json.loads(fixture.read_text(encoding="utf-8"))
cases = law["cases"]
for case in cases:
    image = Image.new("P", (2, 1))
    image.putdata([0, 1])
    valid = len({tuple(pin[:2]) for pin in case["afterPins"]}) == len(case["afterPins"])
    for write in case.get("writes", []):
        x, y, width, height = (write[key] for key in ("x", "y", "width", "height"))
        if x + width > 2 or y + height > 1 or len(write["pixels"]) != width * height:
            valid = False
            break
        region = Image.new("P", (width, height))
        region.putdata(write["pixels"])
        image.paste(region, (x, y))
    pixels = list(image.tobytes()) if valid else [0, 1]
    assert valid == case["admitted"], case["id"]
    assert pixels == case["expectedPixels"], case["id"]
for case in law["allocationCases"]:
    image = Image.new("P", (2, 1))
    image.putdata(case.get("pixels", [0, 1]))
    assert list(image.tobytes()) == case["expectedPixels"], case["id"]
    assert [case.get("paletteR", 0), 255] == case["expectedPaletteR"], case["id"]
    assert case.get("seed", 0) == case["expectedSeed"], case["id"]
print(f"Pillow matched {len(cases) + len(law['allocationCases'])} retained bitmap fixture cases")
