"""🏋️ Round 5 root fix of components/steel-column: the rig mass and `dimensions.massKg` become the welded assembly (shaft, plates, gussets, anchor heads, welds) instead of the bare shaft formula.

Without a flag it prints the breakdown; with `--apply` it re-reads the catalog file and replaces only the two mass values and the centre of mass in the file's own text.

@see ./r3-components-steel-column.md
@see ./r5-brief.md"""
import math
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/steel-column/🔣️.json"
RHO = 7850.0


def shoelace(points):
    """📐 Area of a closed polygon."""
    return abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1]))) / 2


def breakdown():
    """🧮 Parts of the column as built: (name, kg, height of the centre of mass in m)."""
    section = 2 * 0.2 * 0.015 + 0.17 * 0.009 + 4 * 0.018 ** 2 * (1 - math.pi / 4)
    hole = math.pi * 0.009 ** 2 * 0.02
    gusset = shoelace([(0.02, 0.115), (0.02, 0.15), (0.045, 0.15), (0.11, 0.11), (0.11, 0.1), (0.035, 0.1)]) * 0.016 * RHO
    nut = (3 * math.sqrt(3) / 2 * (0.024 / math.sqrt(3)) ** 2 - math.pi * 0.008 ** 2) * 0.0148 * RHO
    washer = math.pi * (0.015 ** 2 - 0.0085 ** 2) * 0.003 * RHO
    stud = math.pi * 0.008 ** 2 * (0.003 + 0.0148 + 0.037) * RHO
    weld = 1.15 * 0.0085 ** 2 / 2 * RHO
    return [
        ("shaft HEB 200, 2.96 m", section * 2.96 * RHO, 1.5),
        ("base plate 300 x 260 x 20, four holes", (0.3 * 0.26 * 0.02 - 4 * hole) * RHO, 0.01),
        ("head plate 240 x 220 x 20, four holes", (0.24 * 0.22 * 0.02 - 4 * hole) * RHO, 2.99),
        ("two gussets 16 mm", 2 * gusset, 0.065),
        ("four anchor heads (nut, washer, rod above the plate)", 4 * (nut + washer + stud), 0.045),
        ("fillet welds a 6 at both ends", 2 * weld, 1.5),
    ]


def main():
    """🚀 Print the breakdown and optionally write it."""
    parts = breakdown()
    kg = sum(m for _, m, _ in parts)
    com = sum(m * h for _, m, h in parts) / kg
    for name, m, h in parts:
        print(f"{name}: {m:.2f} kg at {h:.3f} m")
    print(f"total {kg:.2f} kg, centre of mass {com:.4f} m")
    if "--apply" not in sys.argv:
        return
    text = open(CATALOG, encoding="utf-8").read()
    new, n1 = re.subn(r'"massKg": 183\.93\b', f'"massKg": {kg:.2f}', text)
    new, n2 = re.subn(r'"kg": 183\.93,(\s*)"comX": 1\.5,', lambda m: f'"kg": {kg:.2f},{m.group(1)}"comX": {com:.4f},', new)
    if (n1, n2) != (1, 1):
        sys.exit(f"refused: massKg {n1}, rig mass {n2}")
    open(CATALOG, "w", encoding="utf-8").write(new)
    print("written", CATALOG)


if __name__ == "__main__":
    main()
