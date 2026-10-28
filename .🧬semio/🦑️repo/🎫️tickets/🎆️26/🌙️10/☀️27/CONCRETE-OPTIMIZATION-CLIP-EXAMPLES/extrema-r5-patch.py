import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()


def swap(old, new, count=1):
    global s
    assert s.count(old) == count, (s.count(old), old[:90])
    s = s.replace(old, new)


swap("AXES_AT, AXES_SIZE, Y_MAX, BAR_MAX = np.array([1.0, -.6, 0.]), (3.9, 2.1), 1700, 1500.", "AXES_AT, AXES_SIZE, Y_MAX, BAR_MAX = np.array([1.0, -.7, 0.]), (3.9, 1.9), 1700, 1500.")
# variant geometry and separator
a = s.index("    def _variant(self, part=None):")
b = s.index("    def _phase_formulate") if "    def _phase_formulate" in s else None
end = s.index("        return group", a) + len("        return group")
new_variant = '''    def _variant(self, part=None):
        """📋 The live sum above the live derivation of one part, set off by a rule: demand balance and ÷ COP or EER, or daylight factor and lamps on."""
        group = _mn.VGroup(self._sum_formula((ZONE_X, ZONE_TOP - .2, 0)))
        if part is not None:
            group.add(_mn.Line((ZONE_X, ZONE_TOP - .62, 0), (ZONE_X + 6.3, ZONE_TOP - .62, 0), color=MUTED, stroke_width=1.2, stroke_opacity=.6))
        if part in ("heating", "cooling"):
            group.add(self._demand(part, (ZONE_X, ZONE_TOP - 1.0, 0)), self._conversion(part, (ZONE_X, ZONE_TOP - 1.62, 0)))
        elif part == "lighting":
            group.add(self._daylight_chain((ZONE_X, ZONE_TOP - 1.0, 0)), self._lamps((ZONE_X, ZONE_TOP - 1.62, 0)))
        return group'''
s = s[:a] + new_variant + s[end:]

# demand line: digits that close
a = s.index("    def _demand")
b = s.index("    def _conversion")
new_demand = '''    def _demand(self, part, point, size=15):
        """🧪 The live balance behind a demand: heating = losses − η × gains, cooling = gains − η × removable heat, every term a key of the model result, printed with the digits at which the line closes."""
        winter = part == "heating"
        color = HEAT if winter else COOL
        soft = _room.muted(color)
        first, second, eta = (("heating_losses", "heating_gains", "heating_eta") if winter else ("cooling_gains", "cooling_removable", "cooling_eta"))
        half_up = lambda value, digits=0: math.floor(value * 10 ** digits + .5) / 10 ** digits

        def table(design, found):
            closing = [(a, e) for a, e in ((0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4))
                       if half_up(half_up(found[first], a) - half_up(found[eta], e) * half_up(found[second], a), a) == half_up(found[part + "_demand"], a)]
            amount, digits = closing[0] if closing else (1, 4)
            sign = "=" if closing else "≈"
            names = ((self.tr("Losses", "Verluste"), self.tr("Gains", "Gewinne")) if winter else (self.tr("Gains", "Gewinne"), self.tr("Removable", "Abführbar")))
            head = [(names[0], soft), "−", (self.tr("Useful η", "Nutzgrad η"), INK), "×", (names[1], soft), sign, (self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), color)]
            return [head, [self.live(first, soft, amount, "kWh/a"), "−", self.live(eta, INK, digits), "×", self.live(second, soft, amount, "kWh/a"), sign, self.live(part + "_demand", color, amount, "kWh/a")]]

        return self.formula(self.dials, table, point, size=size, align="left", width=6.4)

'''
s = s[:a] + new_demand + s[b:]

# season note
a = s.index("    def _season_note")
b = s.index("    def _views")
new_note = '''    def _season_note(self, season, point, color):
        """🗓️ Left of the section: hours of direct sun and the sun on the glass relative to the sunniest bearing; above its roof: the season and what the arrows mean."""
        german, s = self._german(), SECTION_SCALE
        at = lambda x, y: np.array(point, dtype=float) + s * np.array([x - 3.0, y - 1.5, 0.0])
        summer = season == "summer"
        name = self.tr("Summer" if summer else "Winter", "Sommer" if summer else "Winter")
        arrow, shade = (self.tr("heat removed →", "Wärme abgeführt →"), COOL) if summer else (self.tr("← heat lost", "← Wärmeverlust"), HEAT)
        header = _mn.VGroup(self._t(name, size=15, color=color, width=1.2), self._t(arrow, size=14, color=shade, width=1.7)).arrange(_mn.RIGHT, buff=.2)
        header = _room.place(header, at(9.0, 4.2), "right")

        def build():
            theta = round(float(self.dials["orientation"].get_value()), 2)
            found = _room.result(_room.values(self.dials))
            count = len(_model.sun_samples(theta, season))
            lines = (_room.text([self.tr("direct sun", "direkte Sonne"), _room.Number(str(count)), "h"], size=14, color=color, width=1.75),
                     _room.text([self.tr("sun on glass", "Sonne am Glas"), _room.Number(_room.number(found["sun_" + season] * 100, 0, german)), "%"], size=14, color=color, width=1.75),
                     _room.text([self.tr("of the best bearing", "der besten Lage")], size=14, color=color, width=1.75))
            return _mn.VGroup(*[_room.place(line, at(-.1, -.5 - .55 * index), "right") for index, line in enumerate(lines)])

        return _mn.VGroup(header, self._widget(build, _room.design_key(self.dials)))

'''
s = s[:a] + new_note + s[b:]

# cost tag one decimal
swap('_room.parts(found["electricity"] * tariff, "CHF/a", 0, german)', '_room.parts(found["electricity"] * tariff, "CHF/a", 1, german)')
swap("f\"≈ {self.quantity(found[0][1] * study['tariff'], 'CHF/a')}\"", "f\"≈ {self.quantity(found[0][1] * study['tariff'], 'CHF/a', 1)}\"")
# marker unchanged; negate formula 450
swap("S = {self.quantity(saving, 'kWh/a', 1)} ×", "S = {self.quantity(saving, 'kWh/a')} ×")
swap("{self.quantity(saving * tariff, 'CHF/a', 1)}\")\n        self.step(\"negate_2\"", "{self.quantity(saving * tariff, 'CHF/a', 1)}\")\n        self.step(\"negate_2\"")
# peak tag
swap("""tag = self._t(f"{self.tr('maximum', 'Maximum')} {self.peak}°", at + _mn.UP * .3 + _mn.LEFT * .5, size=14, color=WARN, width=1.5, edge="right")""",
     """tag = self._t(f"{self.tr('maximum', 'Maximum')} {self.peak}° · {self._kwh(self.peak_energy)}", at + _mn.UP * .3 + _mn.LEFT * .3, size=14, color=WARN, width=2.4, edge="right")""")
# limit line lower
swap("axes.c2p(lo, axes.y_range[1]) + _mn.DOWN * .5, color=WARN, stroke_width=3)", "axes.c2p(lo, axes.y_range[1]) + _mn.DOWN * .62, color=WARN, stroke_width=3)")
# start tags right of the ring
swap('tag = self._l("start", "Start", at_start + _mn.UP * .34, size=15, color=INK, width=1.0)', 'tag = self._l("start", "Start", at_start + _mn.RIGHT * .32 + _mn.UP * .12, size=15, color=INK, width=1.0, edge="left")')
swap('self._l("start", "Start", points[0] + _mn.UP * .34, size=15, color=INK, width=1.0))', 'self._l("start", "Start", points[0] + _mn.RIGHT * .32 + _mn.UP * .12, size=15, color=INK, width=1.0, edge="left"))')
# legend glyphs
swap('(_mn.Square(side_length=.16, color=MUTED, fill_color=MUTED, fill_opacity=.3), self.tr("excluded", "ausgeschlossen"), MUTED)', '(_mn.VGroup(_mn.Square(side_length=.16, color=MUTED, fill_color=MUTED, fill_opacity=.3), _art.cross_mark(WARN).scale(.5)), self.tr("excluded", "Ausgeschlossen"), MUTED)')
swap('self.tr("infeasible: street noise", "unzulässig: Straßenlärm")', 'self.tr("infeasible: street noise", "Unzulässig: Straßenlärm")') if False else None
open(p, "w", encoding="utf-8").write(s)
print("patched")
