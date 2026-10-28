"""✂️ Split the existing current-source contact sheet into readable original-resolution row batches."""
from pathlib import Path

from PIL import Image

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated/iv"


def main():
    """🖼️ Crop existing pixels without decoding or changing any source movie."""
    rows = []
    with Image.open(OUTPUT / "source-progress.png") as sheet:
        total = (sheet.height + 295) // 296
        for ordinal, first in enumerate(range(0, total, 6), 1):
            last = min(total, first + 6)
            output = OUTPUT / f"source-progress-{ordinal:02d}.png"
            sheet.crop((0, first * 296, sheet.width, min(sheet.height, last * 296))).save(output)
            rows.append(f"- {output.name}: rows {first + 1}–{last}")
    with (TICKET / "inspection-source-progress.md").open("a", encoding="utf-8") as report:
        report.write("\n## Original-Resolution Inspection Crops\n\n" + "\n".join(rows) + "\n")
    print("[DEBUG] Contact sheet split into", len(rows), "row batches", flush=True)


if __name__ == "__main__":
    main()
