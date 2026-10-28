import sys
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
F = next(Path("🎓️teaching").rglob("📉️extrema/🐍️.py"))
s = F.read_text(encoding="utf-8")
s = s.replace("        soft = _room.muted(color)\n        def table(design, found):\n            line", "        soft = _room.muted(color)\n\n        def table(design, found):\n            line")
F.write_text(s, encoding="utf-8")
C = Path(sys.argv[1])
c = C.read_text(encoding="utf-8")
a = c.index("for theta in (90, 180, 225, 282, 315, 360, 450):\n    found = at(theta)\n    for part, first")
b = c.index('print("[DEBUG] peak"')
new = '''scene = importlib.import_module(CLIP + ".📉️extrema.🐍️")
widest = {}
for theta in sorted({90, 180, 225, 282, 315, 360, 450, *range(90, 451, 5)}):
    found = at(theta)
    for part in ("heating", "cooling"):
        line = scene.balance_lines(found, part)
        digits, base, percent, top, taken = line["digits"], line["base"], line["percent"], line["top"], line["taken"]
        widest[(part, digits, line["places"])] = widest.get((part, digits, line["places"]), 0) + 1
        product = percent / 100 * base
        demand = scene.half_up(found[part + "_demand"], digits)
        check(f"{part} share line at {theta}: {percent}% x printed base rounds to printed result", scene.half_up(product, digits) == taken and (line["exact"] == (abs(product - taken) < 1e-9)),
              f"{percent}% of {base} = {product:.4f} -> printed {taken} ({digits} dec)")
        check(f"{part} balance line at {theta}: printed numbers subtract exactly", abs(top - taken - demand) < 1e-9 and abs(top - found[BAL[part][0]]) <= 10 ** -digits and abs(taken - found[BAL[part][3]]) <= 10 ** -digits and abs(base - found[BAL[part][1]]) <= 10 ** -digits,
              f"{top} - {taken} = {demand}")
print("[DEBUG] printed precision (part, kWh/a decimals, percent decimals): count", widest)

'''
c = c[:a] + new + c[b:]
c = c.replace('KEYS = ("heating", "cooling", "lighting")\n', 'KEYS = ("heating", "cooling", "lighting")\nBAL = {"heating": ("heating_losses", "heating_gains", "heating_eta", "heating_used"), "cooling": ("cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed")}\n', 1)
C.write_text(c, encoding="utf-8")
