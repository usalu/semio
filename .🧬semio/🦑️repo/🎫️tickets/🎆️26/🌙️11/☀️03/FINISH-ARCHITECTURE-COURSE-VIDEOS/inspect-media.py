"""🖼️ Inspect current course masters and retain representative frames for visual review."""
import importlib.util
import json
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
MEDIA = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media"
sys.path.insert(0, str(MEDIA))
import lib
from PIL import Image, ImageDraw

out = Path(__file__).parent / "🗑️generated"
out.mkdir(exist_ok=True)
folder = lib.base("en", "speaker") / "rendered"
files = [Path(name).resolve() for name in sys.argv[1:]] if sys.argv[1:] else sorted(folder.glob("*_speaker_en_480p15.mp4"))
rows = []
for file in files:
    seconds = lib.duration(file)
    frames = [lib.frame(file, at, (427, 240)) for at in (.25, .5, .75)]
    page = Image.new("RGB", (1281, 270), "white")
    for index, frame in enumerate(frames):
        page.paste(frame, (427 * index, 30))
    ImageDraw.Draw(page).text((8, 8), file.stem + "  " + str(round(seconds, 1)) + "s", fill="black")
    page.save(out / (file.stem + ".png"))
    rows.append({"file": str(file.relative_to(ROOT)), "seconds": seconds, "bytes": file.stat().st_size})
    print("[DEBUG]", file.stem, round(seconds, 1), "s", flush=True)
(out / "media-inspection.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
