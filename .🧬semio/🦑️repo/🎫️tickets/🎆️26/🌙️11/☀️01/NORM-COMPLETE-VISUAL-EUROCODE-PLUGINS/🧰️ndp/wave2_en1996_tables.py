import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
H = [50, 65, 100, 150, 200, 250]; W = [50, 100, 150, 200, 250]
R = [[0.85, 0.75, 0.70, 0.70, 0.70], [0.95, 0.85, 0.75, 0.70, 0.65], [1.15, 1.00, 0.90, 0.80, 0.75], [1.30, 1.20, 1.10, 1.00, 0.95], [1.45, 1.35, 1.25, 1.15, 1.10], [1.55, 1.45, 1.35, 1.25, 1.15]]
for h, row in zip(H, R):
    for w, v in zip(W, row):
        put(d, "1996", f"delta.h{h}.w{w}", "Table A.1", f"delta h={h} w={w}", v, v, "", f"Shape factor delta of EN 772-1 for unit height {h} mm and least horizontal dimension {w} mm; bilinear in the table", part="1-1", locator="EN 772-1 Table A.1")
CL = [30, 60, 90, 120, 180, 240]
F = {"clay": [100, 100, 150, 175, 240, 240], "casi": [90, 115, 140, 175, 240, 300], "aac": [115, 140, 175, 200, 250, 300], "concrete": [90, 100, 140, 170, 240, 300]}
for m, row in F.items():
    for c, v in zip(CL, row):
        put(d, "1996", f"fire.{m}.rei{c}", "Annex B", f"t_min {m} REI {c}", float(v), float(v), "mm", f"Minimum wall thickness of fire class REI {c}, unit material {m}; base row, load-ratio columns not keyed", part="1-2", locator="Annex B tabulated data")
save(d); print(len(d))
