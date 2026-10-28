from pathlib import Path
from PIL import Image

ROOT = Path(r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES")
FRAMES = ROOT / "🗑️generated" / "frames" / "Beat4_PartialModels"
OUT = ROOT / "🗑️generated" / "verify-models"
CROPS = {
    "en-check-inset": ("en-check", (820, 130, 1250, 410)),
    "en-federation-leaders": ("en-federation", (20, 110, 1230, 560)),
    "en-schema-headers": ("en-schema", (40, 130, 1250, 230)),
    "en-midsurface-left": ("en-midsurface", (40, 170, 660, 470)),
    "en-services-drawing": ("en-services", (50, 130, 610, 550)),
    "en-rooms-first-floor": ("en-rooms", (480, 120, 800, 510)),
    "en-loss-leaders": ("en-loss", (30, 100, 700, 410)),
    "en-trace-analysis": ("en-trace", (800, 180, 1180, 440)),
    "en-translation-analysis": ("en-translation", (890, 250, 1240, 480)),
    "de-check-inset": ("de-check", (820, 400, 1250, 600)),
    "en-slab-plan": ("en-slab", (90, 100, 640, 600)),
}
for name, (frame, box) in CROPS.items():
    image = Image.open(FRAMES / f"Beat4_PartialModels-{frame}.png").convert("RGB")
    crop = image.crop(box)
    crop = crop.resize((crop.width * 2, crop.height * 2), Image.LANCZOS)
    crop.save(OUT / f"{name}.png")
    print(name, crop.size)
