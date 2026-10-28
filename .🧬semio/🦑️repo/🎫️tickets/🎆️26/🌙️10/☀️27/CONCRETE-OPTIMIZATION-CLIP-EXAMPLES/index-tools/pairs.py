"""Stack the keyframes of one language in pairs (two frames per image) for reading: python pairs.py <en|de> <source dir> <out dir>."""

import sys
from pathlib import Path

from PIL import Image

language, source, target = sys.argv[1], Path(sys.argv[2]), Path(sys.argv[3])
target.mkdir(parents=True, exist_ok=True)
keys = ["variants", "variants_2", "criteria", "criteria_2", "best", "units", "units_2", "units_3", "ruler", "ruler_2", "ruler_3", "ruler_4", "ruler_5", "shift", "shift_2", "mapped", "cost", "cost_2",
        "daylight", "carbon", "matrix", "weights", "weights_2", "weights_3", "index", "index_2", "index_3", "winner", "winner_2", "sensitivity", "sensitivity_2", "sensitivity_3", "sweep", "sweep_2",
        "range", "range_2", "range_3", "record", "limits", "limits_2", "pointer", "next"]
for start in range(0, len(keys), 2):
    images = [Image.open(source / f"Beat4_NormalizedIndex-{language}-{key}.png") for key in keys[start:start + 2]]
    width, height = images[0].size
    sheet = Image.new("RGB", (width, height * len(images)))
    for index, image in enumerate(images):
        sheet.paste(image, (0, index * height))
    sheet.save(target / f"{language}-{start // 2:02d}-{'+'.join(keys[start:start + 2])}.png")
print("pairs", len(keys) // 2)
