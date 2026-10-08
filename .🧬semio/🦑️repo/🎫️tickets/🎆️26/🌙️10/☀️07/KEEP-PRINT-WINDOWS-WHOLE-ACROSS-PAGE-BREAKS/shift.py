"""📐 Prints the first and last dark rows of a mid-page column for a baseline and a current probe page, to expose vertical drift."""
import subprocess, sys
import numpy as np
from PIL import Image

folder, page = sys.argv[1], sys.argv[2]
for name, pdf in (("before", f"{folder}/before/probe.pdf"), ("after", f"{folder}/probe/📋️zwischenbericht.pdf")):
    subprocess.run(["pdftoppm", "-r", "1200", "-f", page, "-l", page, "-x", "6000", "-W", "8", "-gray", "-singlefile", "-png", pdf, f"{folder}/shift"], check=True)
    column = np.array(Image.open(f"{folder}/shift.png").convert("L"))[:, 4]
    rows = [index for index, value in enumerate(column) if value < 190]
    edges = [row for index, row in enumerate(rows) if index == 0 or row != rows[index - 1] + 1]
    print(name, [round(edge * 72 / 1200, 2) for edge in edges])
