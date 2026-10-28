import sys
from pathlib import Path
from PIL import Image
folder = Path(__file__).resolve().parent / "🗑️generated/frames/Beat3_FactorialExperiments"
bad, count = [], 0
for path in sorted(folder.glob("*.png")):
    count += 1
    try:
        with Image.open(path) as image:
            image.load()
            assert image.size == (1280, 720), image.size
    except Exception as error:
        bad.append((path.name, str(error)))
sys.stdout.buffer.write(f"{count} pngs, bad: {bad}\n".encode("utf-8"))
