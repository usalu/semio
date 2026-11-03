import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
TABLE = [(12, 15, 27), (16, 20, 29), (20, 25, 30), (25, 30, 31), (30, 37, 33), (35, 45, 34), (40, 50, 35), (45, 55, 36), (50, 60, 37), (55, 67, 38), (60, 75, 39), (70, 85, 41), (80, 95, 42), (90, 105, 44)]
d = load()
for cyl, cube, ecm in TABLE:
    i = put(d, "1992", f"t3-1.ecm.c{cyl}-{cube}", "Table 3.1", f"E_cm C{cyl}/{cube}", float(ecm), float(ecm), "GPa", f"Secant modulus of elasticity E_cm of concrete C{cyl}/{cube}, EN 1992-1-1 Table 3.1; shared by the pile elastic shortening of EN 1997 and the EN 1992 catalogue.", part="1-1", keep=True)
    d[i]["evidence"] = "Value of the EN 1992 catalogue table (materials-and-time conc) and equal to the Table 3.1 formula 22 (f_cm/10)^0.3 GPa rounded to 1 GPa (f_cm = f_ck + 8); the German annex is assumed to adopt it; not checked against the licensed print."
save(d)
