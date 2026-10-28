"""🩹 Patch the extrema scene: probes chip, tags and price brackets."""
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:60]
    text = text.replace(old, new)


swap('''        local_tag = self._l("local minimum", "lokales Minimum", axes.c2p(stop, e_local) + _mn.DOWN * .5, size=16, color=DATA, width=2.2)
        probes = _mn.VGroup()
        for item in neighbours:
            spot = axes.c2p(item["theta"], item["electricity"])
            side = _mn.LEFT if item["theta"] < stop else _mn.RIGHT
            chip = _room.chip([f"{item['theta']}° ·", self._kwh(item["electricity"] - e_local, signed=True)], spot + _mn.UP * .75 + side * .75, size=13, color=WARN)
            probes.add(_mn.Dot(spot, radius=.06, color=WARN), _mn.Line(spot, chip.get_bottom(), color=WARN, stroke_width=1.4), chip)
''', '''        local_tag = self._l("local minimum", "lokales Minimum", axes.c2p(stop, e_local) + _mn.DOWN * .38, size=16, color=DATA, width=2.2)
        rows = _mn.VGroup(*[_room.text([f"{item['theta']}°", self._kwh(item["electricity"] - e_local, signed=True)], size=14, color=WARN)
                            for item in neighbours]).arrange(_mn.DOWN, aligned_edge=_mn.LEFT, buff=.08)
        chip = _art.window_chip(rows, color=WARN).move_to(axes.c2p(stop, e_local) + _mn.DOWN * 1.0)
        probes = _mn.VGroup(chip)
        for item in neighbours:
            spot = axes.c2p(item["theta"], item["electricity"])
            probes.add(_mn.Dot(spot, radius=.06, color=WARN), _mn.Line(spot, chip.get_top() + _mn.RIGHT * (.3 if item["theta"] > stop else -.3), color=WARN, stroke_width=1.4))
''')
swap('''        square_tag = self._l("global minimum", "globales Minimum", glob_at + _mn.DOWN * .5, size=16, color=GOOD, width=2.4)''',
     '''        square_tag = self._l("global minimum", "globales Minimum", glob_at + _mn.DOWN * .38, size=16, color=GOOD, width=2.4)''')
swap('''self._out(probes), self._out(m["start"]),''', '''self._out(probes), self._out(m["start"]), self._out(local_tag),''')
swap('''        price = _art.window_chip(_mn.VGroup(self._t(f"+{self.quantity(gap, 'kWh/a')} · +{self.number(gap / e_global * 100, 1)} %", size=15, color=WARN),
                                            self._t(f"+{self.quantity(chf, 'CHF/a')}", size=15, color=WARN)).arrange(_mn.DOWN, buff=.06), color=WARN)
        price.move_to(axes.c2p(stop, e_global) + _mn.RIGHT * 0 + _mn.UP * 1.45 + _mn.RIGHT * .1)
        price.move_to(axes.c2p(stop + 55, (e_local + e_global) / 2 + 10))
''', '''        price = self._price([f"+{self.quantity(gap, 'kWh/a')}", f"+{self.number(gap / e_global * 100, 1)} %", f"+{self.quantity(chf, 'CHF/a')}"], WARN,
                            (axes.c2p(stop, 0)[0] - .15, axes.c2p(stop, (e_local + e_global) / 2)[1], 0))
''')
swap('''        price = _art.window_chip(_mn.VGroup(self._t(f"+{self.quantity(gap, 'kWh/a')}", size=15, color=WARN), self._t(f"+{self.quantity(gap * study['tariff'], 'CHF/a')}", size=15, color=WARN)).arrange(_mn.DOWN, buff=.06), color=WARN)
        price.move_to(axes.c2p(stop + 55, (e_local + e_global) / 2 + 10))
        note = self._l("the price of the courtyard", "der Preis des Innenhofs", axes.c2p(stop + 5, e_global) + _mn.DOWN * .45, size=15, color=WARN, width=3, edge="center")
''', '''        price = self._price([f"+{self.quantity(gap, 'kWh/a')}", f"+{self.quantity(gap * study['tariff'], 'CHF/a')}"], WARN,
                            (axes.c2p(stop, 0)[0] - .15, axes.c2p(stop, (e_local + e_global) / 2)[1], 0))
        note = self._l("price of the courtyard", "Preis des Innenhofs", axes.c2p(stop, e_global) + _mn.DOWN * .38 + _mn.RIGHT * .6, size=15, color=WARN, width=2.4)
''')
swap('''        chip = _art.window_chip(_mn.VGroup(self._t(self.quantity(self.saving(study["global_minimum"]), "kWh/a"), size=15, color=GOOD),
                                           self._t(self.quantity(chf, "CHF/a"), size=15, color=GOOD)).arrange(_mn.DOWN, buff=.06), color=GOOD)
''', '''        chip = self._price([self.quantity(self.saving(study["global_minimum"]), "kWh/a"), self.quantity(chf, "CHF/a")], GOOD, (0, 0, 0))
''')
swap('''    def _bearing_name_any(self, theta):''', '''    def _price(self, rows, color, point, edge="right"):
        """🏷️ A boxed stack of value rows aligned at ``point``."""
        chip = _art.window_chip(_mn.VGroup(*[self._t(row, size=15, color=color) for row in rows]).arrange(_mn.DOWN, buff=.06), color=color)
        return _room.place(chip, point, edge)

    def _bearing_name_any(self, theta):''')
swap('''design = _art.window_chip(self._t(f"θ* = {theta_g}° · {self._bearing_name(theta_g)}", size=17''', '''design = _art.window_chip(self._t(f"θ* = {theta_g}° ({self.tr('south', 'Süd')})", size=17''')
swap('''        start_tag = self._t(f"{self.tr('start', 'Start')} {self._bearing_name_any(start)}", axes.c2p(start, e_start) + _mn.UP * .5 + _mn.LEFT * .3, size=15, color=INK, width=2.4, edge="right")''',
     '''        start_tag = self._t(f"{self.tr('start', 'Start')} {self._bearing_name_any(start)}", axes.c2p(start, e_start) + _mn.DOWN * .3 + _mn.LEFT * .2, size=15, color=INK, width=2.4, edge="right")''')
swap('''clear = [m["cards"], m["route"], m["route_start"], m["local"], m["local_tag"], m["global"]''', '''clear = [m["cards"], m["route"], m["route_start"], m["local"], m["global"]''')
swap('''        return self._t(text, (.6, -2.38, 0), size=14, color=MUTED, width=8.6)''', '''        return self._t(text, (.2, -2.38, 0), size=14, color=MUTED, width=5.8)''')
swap('''cm · simplified annual teaching model",''', '''cm",''')
swap('''cm · vereinfachtes Jahresmodell")''', '''cm")''')
path.write_text(text, encoding="utf-8")
