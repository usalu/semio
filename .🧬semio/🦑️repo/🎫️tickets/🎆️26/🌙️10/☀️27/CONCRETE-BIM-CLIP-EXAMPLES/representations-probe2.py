"""Probe background colour and widget availability."""
import importlib, os, sys
from pathlib import Path
from PIL import Image
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
D = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
print("[DEBUG] BACKGROUND", D.BACKGROUND, "ARCH", D.ARCH, "DATA", D.DATA, "GOOD", D.GOOD, "WARN", D.WARN, "MUTED", D.MUTED, "CONCRETE", D.CONCRETE)
img = Image.open(Path(sys.argv[1]))
print("[DEBUG] pixel", img.getpixel((5, 300)), img.getpixel((640, 200)))
