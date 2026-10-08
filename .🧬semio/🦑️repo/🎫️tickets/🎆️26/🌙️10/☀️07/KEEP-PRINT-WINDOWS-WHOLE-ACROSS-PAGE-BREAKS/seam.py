"""📏 Measures, in points, the title tab top rule and the first dark body row of a page-top window in a rendered probe page."""
import subprocess, sys
import numpy as np
from PIL import Image

pdf, page, left, dpi = sys.argv[1], sys.argv[2], sys.argv[3], 1200
subprocess.run(["pdftoppm", "-r", str(dpi), "-f", page, "-l", page, "-x", left, "-y", "950", "-W", "40", "-H", "1100", "-gray", "-singlefile", "-png", pdf, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️07/KEEP-PRINT-WINDOWS-WHOLE-ACROSS-PAGE-BREAKS/🗑️generated/seam-col"], check=True)
column = np.array(Image.open("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️07/KEEP-PRINT-WINDOWS-WHOLE-ACROSS-PAGE-BREAKS/🗑️generated/seam-col.png").convert("L"))[:, 20]
dark = [index for index, value in enumerate(column) if value < 190]
runs, start = [], dark[0]
for previous, current in zip(dark, dark[1:] + [None]):
    if current is None or current != previous + 1:
        runs.append((start, previous))
        start = current
print([(round(a * 72 / dpi, 2), round((b - a + 1) * 72 / dpi, 2)) for a, b in runs[:4]])
