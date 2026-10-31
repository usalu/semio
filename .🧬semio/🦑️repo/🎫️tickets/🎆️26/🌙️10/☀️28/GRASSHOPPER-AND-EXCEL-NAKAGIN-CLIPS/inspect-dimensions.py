"""🔎 Inspect primary drawing dimensions without changing or resampling any model asset."""
from pathlib import Path
from PIL import Image

output = Path(__file__).resolve().parent / "🗑️generated"
source = Image.open(output / "domus-section-full.jpg")
for name, box, turn in (("primary-core-dimension.png", (0, 1050, 335, 1540), 0), ("primary-section-chain.png", (1150, 0, 1280, 1030), 270), ("primary-total-chain.png", (630, 0, 735, 1030), 270)):
    crop = source.crop(box).rotate(turn, expand=True)
    crop.resize((crop.width * 3, crop.height * 3), Image.Resampling.NEAREST).save(output / name)
    print("[DEBUG] primary dimension crop: " + name, flush=True)

for name, box in (("primary-podium-chain.png", (1150, 740, 1280, 1030)), ("primary-b-head-chain.png", (1150, 80, 1280, 460))):
    crop = source.crop(box).rotate(270, expand=True)
    crop.resize((crop.width * 5, crop.height * 5), Image.Resampling.NEAREST).save(output / name)
    print("[DEBUG] primary focused crop: " + name, flush=True)

for name, box in (("primary-core-plan.png", (330, 1110, 720, 1440)), ("primary-b-core-plan.png", (750, 1110, 1150, 1440))):
    crop = source.crop(box)
    crop.resize((crop.width * 3, crop.height * 3), Image.Resampling.NEAREST).save(output / name)
    print("[DEBUG] primary plan crop: " + name, flush=True)
