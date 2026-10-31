"""🖼️ Contact sheet: one row of 25/50/75 percent frames per animation, three rows per sheet."""
import sys
from pathlib import Path

from PIL import Image

folder = Path(__file__).resolve().parent.parent / "🗑️generated/morph-toolkit"
names = sorted({p.name.rsplit("-", 1)[0] for p in folder.glob("*-25.png")})
if sys.argv[1:]:
    names = [n for n in names if any(k in n for k in sys.argv[1:])]
for start in range(0, len(names), 3):
    chunk = names[start : start + 3]
    sheet = Image.new("RGB", (3 * 640, len(chunk) * 360))
    for row, name in enumerate(chunk):
        for col, mark in enumerate((25, 50, 75)):
            sheet.paste(Image.open(folder / f"{name}-{mark}.png").resize((640, 360)), (col * 640, row * 360))
    out = folder / f"sheet-{start // 3:02d}.png"
    sheet.save(out)
    print("[DEBUG]", out.name, chunk)
