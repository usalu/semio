import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/check-extrema-r2-tables.py"
s = open(p, encoding="utf-8").read()
a = s.index("half_up = lambda value, digits=0: float(__import__")
b = s.index('print("[DEBUG] peak"')
new = '''for theta in (90, 180, 225, 282, 315, 360, 450):
    found = at(theta)
    for part, first, second, eta, used in (("heating", "heating_losses", "heating_gains", "heating_eta", "heating_used"), ("cooling", "cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed")):
        top, taken = room.apportion([found[first], -found[used]], 0, found[part + "_demand"])
        shown_used, demand = -taken, round(found[part + "_demand"])
        share = f"{found[eta] * 100:.1f} % of {round(found[second]):,} kWh/a = {found[eta] * 100 / 100 * round(found[second]):.1f} (printed {shown_used:,.0f})"
        check(f"{part} balance at {theta}: whole numbers subtract exactly", top - shown_used == demand and abs(top - found[first]) <= 1 and abs(shown_used - found[used]) <= 1,
              f"{top:,.0f} - {shown_used:,.0f} = {demand:,}")
        check(f"{part} useful share line at {theta}: result within one kWh/a of share x printed base", abs(found[eta] * round(found[second]) - shown_used) <= 1.5, share)

'''
open(p, "w", encoding="utf-8").write(s[:a] + new + s[b:])
