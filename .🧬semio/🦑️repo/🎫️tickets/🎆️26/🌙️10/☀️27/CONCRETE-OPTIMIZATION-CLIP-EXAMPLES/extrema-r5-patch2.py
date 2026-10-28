import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()


def swap(old, new, count=1):
    global s
    assert s.count(old) == count, (s.count(old), old[:90])
    s = s.replace(old, new)


swap('''    def _price_row(self, parts, point=(1.5, ZONE_MID, 0)):
        """🪙 One boxed row of value text above the plot."""
        return _art.window_chip(self._t(parts, size=16, color=WARN, width=6.0), color=WARN).move_to(point)''',
     '''    def _price_row(self, *lines, point=(1.5, ZONE_MID, 0)):
        """🪙 One boxed block of one or two value lines above the plot."""
        block = _mn.VGroup(*[self._t(line, size=16, color=WARN, width=6.0) for line in lines]).arrange(_mn.DOWN, buff=.08)
        return _art.window_chip(block, color=WARN).move_to(point)''')
swap('''row =self._price_row(f"{self.number(e_local)} − {self.number(e_global)} = +{self.quantity(gap, 'kWh/a')}  ·  +{self.number(gap / e_global * 100, 1)} %  ·  +{self.quantity(gap * study['tariff'], 'CHF/a')}")''',
     '''row = self._price_row(f"{self.tr('North 360° − South 180°', 'Nord 360° − Süd 180°')}: {self.number(e_local)} − {self.number(e_global)} kWh/a",
                              f"= +{self.quantity(gap, 'kWh/a')}  ·  +{self.number(gap / e_global * 100, 1)} % {self.tr('of South', 'vom Süden')}  ·  +{self.quantity(gap * study['tariff'], 'CHF/a')}")''')
swap('''price =self._price_row(f"{self.tr('Courtyard price', 'Hofpreis')}:  {self.number(e_local)} − {self.number(e_global)} = +{self.quantity(gap, 'kWh/a')}  ·  +{self.quantity(gap * study['tariff'], 'CHF/a')}")''',
     '''price = self._price_row(f"{self.tr('Courtyard price', 'Hofpreis')}, {self.tr('North 360° − South 180°', 'Nord 360° − Süd 180°')}: {self.number(e_local)} − {self.number(e_global)} kWh/a",
                                f"= +{self.quantity(gap, 'kWh/a')}  ·  +{self.number(gap / e_global * 100, 1)} % {self.tr('of South', 'vom Süden')}  ·  +{self.quantity(gap * study['tariff'], 'CHF/a')}")''')
swap('''pay = self._price_row(f"S = ''', '''pay = self._price_row(f"S = ''')
# table header unit
swap('head = [[(title, MUTED, 6.0), ("", MUTED), ("", MUTED)]]', 'head = [[(title, MUTED, 4.4), ("", MUTED), ("kWh/a", MUTED)]]')
# goal and objective: define E
swap('self._l("Goal: minimize E", "Ziel: E minimieren", size=22, color=GOOD, width=2.4)', 'self._l("Goal: minimize electricity E", "Ziel: Strombedarf E minimieren", size=20, color=GOOD, width=3.2)')
swap('self._l("lower is better", "kleiner ist besser", size=17, color=GOOD, width=1.7)', 'self._l("lower is better", "Kleiner ist besser", size=17, color=GOOD, width=1.7)')
swap('self._l("Objective: electricity per year", "Ziel: Strombedarf pro Jahr"', 'self._l("Objective E: electricity per year", "Ziel E: Strombedarf pro Jahr"')
# compare cards: words not letters
swap("f\"{self.tr('N', 'N')} {study['local_minima'][1]}°", "f\"{self.tr('North', 'Nord')} {study['local_minima'][1]}°")
swap("f\"{self.tr('S', 'S')} {study['local_minima'][0]}°", "f\"{self.tr('South', 'Süd')} {study['local_minima'][0]}°")
swap("self.tr('best', 'beste')}\", size=16, color=GOOD, width=3.3)", "self.tr('best', 'Beste')}\", size=15, color=GOOD, width=3.5)")
swap("size=16, color=LOCAL, width=2.6), color=LOCAL)", "size=15, color=LOCAL, width=2.8), color=LOCAL)")
# German capitals in chips
swap('self.tr("local minimum", "lokales Minimum")', 'self.tr("local minimum", "Lokales Minimum")')
swap('self.tr("global minimum", "globales Minimum")', 'self.tr("global minimum", "Globales Minimum")')
swap('self.tr("feasible minimum", "zulässig")', 'self.tr("feasible minimum", "Zulässig")')
swap('"unzulässig: Straßenlärm"', '"Unzulässig: Straßenlärm"')
# zero tag position
swap('left, zero = axes2.c2p(self.low, 0)[0] + .17, axes2.c2p(0, 0)[1]', 'left, zero = axes2.c2p(self.low, 0)[0] + .12, axes2.c2p(0, 0)[1]')
swap('("S = 0:", "S = 0:", (left, zero - .16, 0), size=14, color=WARN, width=.7, edge="left")', '("S = 0:", "S = 0:", (left, zero - .16, 0), size=14, color=WARN, width=.7, edge="left")') if False else None
swap('self._l("east 90°", "Ost 90°", (left, zero - .36, 0), size=14, color=WARN, width=.8, edge="left")', 'self._l("east 90°", "Ost 90°", (left, zero - .36, 0), size=14, color=WARN, width=.72, edge="left")')
# scope: strip title and wiggle note
swap('strip = self.ramp_strip(colour, (5.3, -2.2, 0), width=2.4, size=14)', 'strip = self.ramp_strip(colour, (5.3, -2.15, 0), width=2.4, size=14, title=self.tr("electricity at θ*", "Strombedarf bei θ*"))')
open(p, "w", encoding="utf-8").write(s)
print("patched2")
