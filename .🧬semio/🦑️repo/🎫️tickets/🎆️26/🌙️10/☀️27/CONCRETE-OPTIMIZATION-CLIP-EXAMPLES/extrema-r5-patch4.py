import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/check-extrema-r2-tables.py"
s = open(p, encoding="utf-8").read()
marker = 'print("[DEBUG] peak"'
add = '''half_up = lambda value, digits=0: float(__import__("decimal").Decimal(repr(float(value))).quantize(__import__("decimal").Decimal(1).scaleb(-digits), rounding="ROUND_HALF_UP"))
for theta, part, first, second, eta in ((180, "heating", "heating_losses", "heating_gains", "heating_eta"), (180, "cooling", "cooling_gains", "cooling_removable", "cooling_eta"),
                                       (282, "cooling", "cooling_gains", "cooling_removable", "cooling_eta"), (90, "heating", "heating_losses", "heating_gains", "heating_eta"),
                                       (315, "heating", "heating_losses", "heating_gains", "heating_eta")):
    found = at(theta)
    closing = [(a, e) for a, e in ((0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4)) if half_up(half_up(found[first], a) - half_up(found[eta], e) * half_up(found[second], a), a) == half_up(found[part + "_demand"], a)]
    a, e = closing[0] if closing else (1, 4)
    check(f"printed {part} balance closes at {theta}", bool(closing), f"{half_up(found[first], a):,.{a}f} - {half_up(found[eta], e):.{e}f} x {half_up(found[second], a):,.{a}f} = {half_up(found[part + '_demand'], a):,.{a}f}")
'''
open(p, "w", encoding="utf-8").write(s.replace(marker, add + marker, 1))
