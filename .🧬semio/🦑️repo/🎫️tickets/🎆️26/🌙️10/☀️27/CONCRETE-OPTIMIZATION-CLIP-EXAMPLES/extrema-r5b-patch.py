import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
a = s.index("    def _demand")
b = s.index("    def _lamps")
new = '''    def _demand(self, part, point, size=15):
        """🧪 The balance behind a demand as two short exact lines: useful share of the gains (or removable heat) in %, then the whole-number subtraction that closes as printed."""
        winter = part == "heating"
        color = HEAT if winter else COOL
        soft = _room.muted(color)
        first, second, eta, used = (("heating_losses", "heating_gains", "heating_eta", "heating_used") if winter else ("cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed"))

        def table(design, found):
            top, taken = self.parts([found[first], -found[used]], 0, found[part + "_demand"])
            names = ((self.tr("Useful gains", "Nutzbare Gewinne"), self.tr("Losses", "Verluste"), self.tr("Heating demand", "Heizbedarf")) if winter else
                     (self.tr("Removed heat", "Abgeführte Wärme"), self.tr("Gains", "Gewinne"), self.tr("Cooling demand", "Kühlbedarf")))
            share = [(names[0], soft), "=", self.live(eta, INK, 1, "%", scale=100), self.tr("of", "von"), self.live(second, soft, 0, "kWh/a"), "≈", self.live(-taken, soft, 0, "kWh/a")]
            balance = [(names[1], soft), self.live(top, soft, 0), "−", (names[0].lower() if self.language() == "en" else names[0], soft), self.live(-taken, soft, 0), "=", (names[2], color), self.live(part + "_demand", color, 0, "kWh/a")]
            return [share, balance]

        return self.formula(self.dials, table, point, size=size, align="left", width=6.4)

    def _conversion(self, part, point, size=15, width=6.4):
        """🪫 The last step to electricity as one line: heating demand ÷ COP, cooling demand ÷ EER (German: annual performance factor, JAZ)."""
        winter = part == "heating"
        color, factor = (HEAT, "cop") if winter else (COOL, "eer")
        soft = _room.muted(color)
        label = self.tr("COP" if winter else "EER", "JAZ" if winter else "Kälte-JAZ")
        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(half_up(found[part + "_demand"]) / found[factor]) == half_up(found[part]) else "≈"
            return [[(self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), soft), self.live(part + "_demand", soft, 0, "kWh/a"), "÷", (label, INK), self.live(factor, INK, 1), sign,
                     (self.tr(*PARTS[0 if winter else 1][1]), color), self.live(part, color, 0, "kWh/a", parts_of=keys)]]

        return self.formula(self.dials, table, point, size=size, align="left", width=width)

'''
s = s[:a] + new + s[b:]
s = s.replace("self._demand(part, (ZONE_X, ZONE_TOP - 1.0, 0)), self._conversion(part, (ZONE_X, ZONE_TOP - 1.62, 0))", "self._demand(part, (ZONE_X, ZONE_TOP - 1.0, 0)), self._conversion(part, (ZONE_X, ZONE_TOP - 1.6, 0))")
open(p, "w", encoding="utf-8").write(s)
print("ok")
