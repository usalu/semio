import sys
from pathlib import Path
from PIL import Image

root = Path(__file__).resolve().parent / "generated-placeholder"
src, box, out = sys.argv[1], [int(v) for v in sys.argv[2].split(",")], sys.argv[3]
image = Image.open(src).crop(box)
image = image.resize((image.width * 3, image.height * 3), Image.LANCZOS)
image.save(out)
print("[DEBUG] saved", out)
