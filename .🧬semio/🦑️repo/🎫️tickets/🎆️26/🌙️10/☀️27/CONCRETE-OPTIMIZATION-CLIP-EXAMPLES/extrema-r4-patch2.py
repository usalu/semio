import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
a = s.index("    def _demand")
b = s.index("    def _daylight_chain")
new = '''    def _demand(self, part, point, size=15):
        """🧪 The live balance behind a demand: heating = losses − η × gains, cooling = gains − η × removable heat, every term a key of the model result."""
        winter = part == "heating"
        color = HEAT if winter else COOL
        soft = _room.muted(color)
        first, second, eta = (("heating_losses", "heating_gains", "heating_eta") if winter else ("cooling_gains", "cooling_removable", "cooling_eta"))
        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits

        def table(design, found):
            sign = "=" if half_up(half_up(found[first]) - half_up(found[eta], 2) * half_up(found[second])) == half_up(found[part + "_demand"]) else "≈"
            names = ((self.tr("Losses", "Verluste"), self.tr("Gains", "Gewinne")) if winter else (self.tr("Gains", "Gewinne"), self.tr("Removable", "Abführbar")))
            head = [(names[0], soft), "−", (self.tr("Useful η", "Nutzgrad η"), INK), "×", (names[1], soft), sign, (self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), color)]
            return [head, [self.live(first, soft, 0, "kWh/a"), "−", self.live(eta, INK, 2), "×", self.live(second, soft, 0, "kWh/a"), sign, self.live(part + "_demand", color, 0, "kWh/a")]]

        return self.formula(self.dials, table, point, size=size, align="left", width=6.4)

    def _conversion(self, part, point, size=15, width=6.4):
        """🪫 The last step to electricity: heating demand ÷ COP, cooling demand ÷ EER (German: annual performance factor, JAZ)."""
        winter = part == "heating"
        color, factor = (HEAT, "cop") if winter else (COOL, "eer")
        soft = _room.muted(color)
        label = self.tr("COP" if winter else "EER", "JAZ" if winter else "Kälte-JAZ")
        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(half_up(found[part + "_demand"]) / found[factor]) == half_up(found[part]) else "≈"
            head = [(self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), soft), "÷", (label, INK), sign, (self.tr(*PARTS[0 if winter else 1][1]), color)]
            return [head, [self.live(part + "_demand", soft, 0, "kWh/a"), "÷", self.live(factor, INK, 1), sign, self.live(part, color, 0, "kWh/a", parts_of=keys)]]

        return self.formula(self.dials, table, point, size=size, align="left", width=width)

    def _lamps(self, point, size=15, width=6.4):
        """💡 The live lighting product: installed power × hours × share of the hours the lamps burn, printed so that it closes."""
        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits
        keys = ("heating", "cooling", "lighting")

        def table(design, found):
            sign = "=" if half_up(found["lighting_power"] * found["lighting_hours"] * half_up(100 * found["lamps_on"], 1) / 100) == half_up(found["lighting"]) else "≈"
            head = [(self.tr("Installed power", "Leistung"), INK), "×", (self.tr("Hours", "Stunden"), INK), "×", (self.tr("Lamps on", "Lampen an"), LIGHT), sign, (self.tr("Lighting", "Beleuchtung"), LIGHT)]
            return [head, [self.live("lighting_power", INK, 3, "kW"), "×", self.live("lighting_hours", INK, 0, "h/a"), "×", self.live("lamps_on", LIGHT, 1, "%", scale=100), sign, self.live("lighting", LIGHT, 0, "kWh/a", parts_of=keys)]]

        return self.formula(self.dials, table, point, size=size, align="left", width=width)

'''
open(p, "w", encoding="utf-8").write(s[:a] + new + s[b:])
