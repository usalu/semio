import sys
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
F = next(Path("🎓️teaching").rglob("📉️extrema/🐍️.py"))
s = F.read_text(encoding="utf-8")
old_start = s.index("        first, second, eta, used = ((\"heating_losses\"")
old_end = s.index("        return self.formula(self.dials, table, point, size=size, align=\"left\", width=6.4)\n\n    def _conversion")
new = '''
        def table(design, found):
            line = balance_lines(found, part)
            digits = line["digits"]
            names = ((self.tr("Useful gains", "Nutzbare Gewinne"), self.tr("Losses", "Verluste"), self.tr("Heating demand", "Heizbedarf")) if winter else
                     (self.tr("Removed heat", "Abgeführte Wärme"), self.tr("Gains", "Gewinne"), self.tr("Cooling demand", "Kühlbedarf")))
            share = [(names[0], soft), "=", self.live(line["percent"], INK, line["places"], "%"), self.tr("of", "von"), self.live(line["base"], soft, digits, "kWh/a"), "=" if line["exact"] else "≈", self.live(line["taken"], soft, digits, "kWh/a")]
            balance = [(names[1], soft), self.live(line["top"], soft, digits), "−", (names[0].lower() if self.language() == "en" else names[0], soft), self.live(line["taken"], soft, digits), "=", (names[2], color), self.live(part + "_demand", color, digits, "kWh/a")]
            return [share, balance]

'''
s = s[:old_start] + new.lstrip("\n") + s[old_end:]
s = s.replace('''        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(half_up(found[part + "_demand"]) / found[factor]) == half_up(found[part]) else "≈"''', '''        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            digits = balance_lines(found, part)["digits"]
            sign = "=" if half_up(half_up(found[part + "_demand"], digits) / found[factor]) == half_up(found[part]) else "≈"''')
s = s.replace('''self.live(part + "_demand", soft, 0, "kWh/a"), "÷", (label, INK)''', '''self.live(part + "_demand", soft, digits, "kWh/a"), "÷", (label, INK)''')
s = s.replace('''        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(found["lighting_power"]''', '''        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(found["lighting_power"]''')
F.write_text(s, encoding="utf-8")
