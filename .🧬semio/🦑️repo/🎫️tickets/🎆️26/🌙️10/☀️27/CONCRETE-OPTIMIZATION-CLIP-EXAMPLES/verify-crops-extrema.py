"""✂️ Zoom into the spots of the round-2 extrema frames that the full-size reading left unclear."""

import sys
from pathlib import Path

from PIL import Image

sys.stdout.reconfigure(encoding="utf-8")
FRAMES = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/frames/Beat1_LocalGlobalExtrema")
OUT = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/verify-extrema")
OUT.mkdir(parents=True, exist_ok=True)
CROPS = [
    ("en-sweep_4-summer", "en-sweep_4", (0, 180, 480, 390), 2),
    ("en-sweep_4-disc", "en-sweep_4", (400, 320, 480, 400), 5),
    ("en-sweep_4-derivation", "en-sweep_4", (470, 170, 1080, 240), 2),
    ("en-local-winter", "en-local", (20, 380, 400, 540), 2),
    ("en-compare_2-route", "en-compare_2", (700, 300, 860, 420), 4),
    ("de-maximize-ref", "de-maximize", (560, 300, 760, 360), 4),
    ("de-maximize-total", "de-maximize", (790, 100, 1060, 230), 3),
    ("en-argument-chip", "en-argument", (530, 110, 1040, 200), 2),
    ("en-constraint-chip", "en-constraint", (690, 280, 930, 350), 3),
    ("en-negate-tags", "en-negate", (520, 470, 880, 520), 3),
    ("de-response-lighting", "de-response", (470, 400, 1120, 470), 2),
    ("de-argument-chip", "de-argument", (540, 110, 1040, 200), 3),
    ("en-scope-curve", "en-scope", (380, 380, 900, 540), 2),
    ("en-negate-2-chip", "en-negate_2", (530, 130, 1020, 190), 2),
    ("en-fixed-wwr", "en-fixed", (100, 240, 580, 500), 2),
]
for name, frame, box, zoom in CROPS:
    source = FRAMES / f"Beat1_LocalGlobalExtrema-{frame.replace('-', '-')}.png"
    if not source.exists():
        source = FRAMES / f"Beat1_LocalGlobalExtrema-{frame}.png"
    image = Image.open(source).convert("RGB").crop(box)
    image = image.resize((image.width * zoom, image.height * zoom), Image.LANCZOS)
    image.save(OUT / f"{name}.png")
    print(f"[DEBUG] wrote {OUT / (name + '.png')} from {source.name}")
