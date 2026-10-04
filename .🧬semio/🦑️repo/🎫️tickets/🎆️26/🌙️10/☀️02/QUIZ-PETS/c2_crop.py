"""🔪️ Ticket tool of work package C2: cuts every PNG of a folder that is taller than a height (default 1400 px) into numbered parts `<name>-part1.png`, … so each can be looked at without being shrunk. `.venv/Scripts/python.exe c2_crop.py <folder> [--height 1400] [--match species-]`."""

import sys
from pathlib import Path

from PIL import Image


def main() -> None:
    """✂️ Cuts the tall PNGs of the folder named first on the command line."""
    folder = Path(sys.argv[1])
    height = int(sys.argv[sys.argv.index("--height") + 1]) if "--height" in sys.argv else 1400
    match = sys.argv[sys.argv.index("--match") + 1] if "--match" in sys.argv else ""
    for path in sorted(folder.glob("*.png")):
        if "-part" in path.stem or not path.name.startswith(match):
            continue
        with Image.open(path) as image:
            if image.height <= height:
                continue
            for index, top in enumerate(range(0, image.height, height), start=1):
                image.crop((0, top, image.width, min(top + height, image.height))).save(path.with_name(f"{path.stem}-part{index}.png"))
                print(path.with_name(f"{path.stem}-part{index}.png").name)


main()
