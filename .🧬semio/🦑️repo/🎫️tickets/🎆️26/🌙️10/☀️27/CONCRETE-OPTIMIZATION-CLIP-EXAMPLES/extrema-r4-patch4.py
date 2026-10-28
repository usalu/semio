import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/check-extrema-r2-tables.py"
s = open(p, encoding="utf-8").read()
a = s.index('scene = importlib.import_module')
b = s.index('print("[DEBUG] peak"')
new = '''for theta in (90, 180, 282, 315, 360):
    found = at(theta)
    for part, first, second, eta, removed in (("heating", "heating_losses", "heating_gains", "heating_eta", "heating_used"), ("cooling", "cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed")):
        winter = part == "heating"
        demand = found[first] - found[removed] if winter else found[first] - found[removed]
        check(f"{part} demand balance at {theta} (model keys)", abs(found[eta] * found[second] - found[removed]) < 1e-9 and abs(demand - found[part + "_demand"]) < 1e-6,
              f"{found[first]:.0f} - {found[eta]:.2f} x {found[second]:.0f} = {demand:.1f}")
    check(f"conversion at {theta}", abs(found["heating_demand"] / found["cop"] - found["heating"]) < 1e-9 and abs(found["cooling_demand"] / found["eer"] - found["cooling"]) < 1e-9)
for theta in (90, 180, 360):
    found = at(theta)
    on = round(100 * found["lamps_on"], 1)
    check(f"lamps line at {theta}", round(found["lighting_power"] * found["lighting_hours"] * on / 100) == round(found["lighting"]) and abs(found["lamps_on"] - (1 - found["autonomy"])) < 1e-12,
          f"{found['lighting_power']:.3f} kW x {found['lighting_hours']:.0f} h/a x {on} % = {found['lighting_power'] * found['lighting_hours'] * on / 100:.1f}")

'''
s = s[:a] + new + s[b:]
open(p, "w", encoding="utf-8").write(s)
