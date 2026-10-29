"""📑 Contact sheets of the Beat1_Anatomy keyframes: four frames per PNG, in narration order."""
import sys
from pathlib import Path
from PIL import Image

T = Path(__file__).resolve().parent.parent
KEYS = "hook workbook sheets grid address range bars constant display formula frames hatch precedence function locale recalc trace div0 name ref value repair legend".split()
for lang in ("en", "de"):
    src = T / "🗑️generated/frames/📊️excel/Beat1_Anatomy" / lang
    out = T / "🗑️generated/excel-anatomy-sheets"
    out.mkdir(parents=True, exist_ok=True)
    for index in range(0, len(KEYS), 4):
        sheet = Image.new("RGB", (1920, 1080))
        for slot, key in enumerate(KEYS[index:index + 4]):
            image = Image.open(src / f"{key}.png").resize((960, 540))
            sheet.paste(image, ((slot % 2) * 960, (slot // 2) * 540))
        sheet.save(out / f"{lang}-{index // 4}.png")
print("[DEBUG] done")
