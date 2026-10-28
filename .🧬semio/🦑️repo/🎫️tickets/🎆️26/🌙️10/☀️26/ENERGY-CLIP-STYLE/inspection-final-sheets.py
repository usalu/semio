"""✂️ Split independently verified final contact sheets into readable row batches."""
import json
from pathlib import Path

from PIL import Image

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated/iv"


def main():
    """🖼️ Crop existing contact pixels without any extra movie decoding."""
    measurements = json.loads((OUTPUT / "measurements.json").read_text(encoding="utf-8"))
    rows = []
    for video in measurements["videos"]:
        root = OUTPUT / video["id"]
        for kind in ("coverage", "navigation"):
            with Image.open(root / f"{kind}.png") as sheet:
                total = (sheet.height + 295) // 296
                for ordinal, first in enumerate(range(0, total, 6), 1):
                    last = min(total, first + 6)
                    path = root / f"{kind}-{ordinal:02d}.png"
                    sheet.crop((0, first * 296, sheet.width, min(sheet.height, last * 296))).save(path)
                    rows.append(f"- {video['id']}/{path.name}: rows {first + 1}–{last}")
    (TICKET / "inspection-final-sheets.md").write_text("# Inspection Final Sheet Crops\n\nThese disposable row batches preserve the original 1440-pixel contact-sheet width for visual audit without additional movie decoding. Full contact sheets are retained with the final deliveries.\n\n" + "\n".join(rows) + "\n", encoding="utf-8")
    print("[DEBUG] Final sheet row batches", len(rows), flush=True)


if __name__ == "__main__":
    main()
