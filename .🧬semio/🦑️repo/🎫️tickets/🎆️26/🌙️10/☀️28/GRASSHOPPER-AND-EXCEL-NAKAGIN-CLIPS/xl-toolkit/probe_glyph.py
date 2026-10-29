import importlib, sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
t = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
img = Image.new("RGB", (900, 300), "white"); d = ImageDraw.Draw(img); y = 5
for name, paths in t.font_sources().items():
    if name == "Noto Emoji": continue
    font = ImageFont.truetype(str(paths[0]), 44)
    d.text((10, y), f"{name[:8]} =SUM(B2:B4)-B5 [x] U-v", font=font, fill="black"); y += 58
img.save(str(Path(sys.argv[1])))
