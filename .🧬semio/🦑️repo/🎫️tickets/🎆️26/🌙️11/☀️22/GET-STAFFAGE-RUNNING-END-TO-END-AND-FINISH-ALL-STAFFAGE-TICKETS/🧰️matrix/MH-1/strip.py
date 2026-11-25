"""🎞️ Cuts the solid row (lower half, without the caption block) out of product sheets for a before/after reading: `python3 strip.py <out folder> <sheet.png>…` writes one 1500 px wide row per sheet under its own name (uses `sips`)."""
import os, subprocess, sys

folder, sheets = sys.argv[1], sys.argv[2:]
os.makedirs(folder, exist_ok=True)
for sheet in sheets:
    row = os.path.join(folder, os.path.basename(sheet))
    subprocess.run(["sips", "-c", "460", "2400", "--cropOffset", "715", "0", sheet, "--out", row], capture_output=True, check=True)
    subprocess.run(["sips", "-Z", "1500", row], capture_output=True, check=True)
    print(row)
