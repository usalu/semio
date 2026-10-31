"""🈚 Render only the handcrafted vector paths for visual comparison with the primary lettering."""
import json
from pathlib import Path
from PIL import Image, ImageDraw

repository = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
root = repository / "🎓️teaching/🏛️architecture/🏢️nakagin"
data = json.loads((root / "🔣️.json").read_text(encoding="utf-8"))["capital"]["signage"]
image = Image.new("RGB", (620, 340), "#20242a")
draw = ImageDraw.Draw(image)
for index, character in enumerate(data["text"]):
    for path in data["glyphs"][character]:
        points = [(30 + index * 300 + u * 260, 35 + v * 260) for u, v in path]
        draw.line(points, fill="white", width=round(data["stroke_width"]["value"] / data["glyph_size"]["value"] * 260), joint="curve")
image.save(Path(__file__).parent / "🗑️generated/roof-glyph-preview.png")
