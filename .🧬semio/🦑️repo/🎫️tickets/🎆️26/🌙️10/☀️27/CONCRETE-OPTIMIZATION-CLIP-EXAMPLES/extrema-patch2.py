"""🩹 Patch the extrema scene: tag placement, zero strip, price rows, scope summary."""
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:70]
    text = text.replace(old, new)


swap('''axes.c2p(start, e_start) + _mn.DOWN * .3 + _mn.LEFT * .2, size=15, color=INK, width=2.4, edge="right")''',
     '''axes.c2p(start, e_start) + _mn.UP * .32 + _mn.RIGHT * .1, size=15, color=INK, width=2.4, edge="left")''')
swap('''axes.c2p(stop, e_local) + _mn.DOWN * .38, size=16, color=DATA, width=2.2)''',
     '''axes.c2p(stop, e_local) + _mn.DOWN * .3 + _mn.RIGHT * .14, size=16, color=DATA, width=2.2, edge="left")''')
swap('''glob_at + _mn.DOWN * .38, size=16, color=GOOD, width=2.4)''', '''glob_at + _mn.DOWN * .3 + _mn.RIGHT * .14, size=16, color=GOOD, width=2.4, edge="left")''')
swap('''self._build_axes((-50, 500, 100), [0, 100''', '''self._build_axes((-120, 500, 100), [0, 100''')
swap('''axes2.c2p(self.high, 0) + _mn.UP * .2, size=14, color=WARN, width=2.4, edge="right")''', '''axes2.c2p(self.low, -70) + _mn.RIGHT * .1, size=14, color=WARN, width=2.4, edge="left")''')
swap('''        band_tag = self._l("infeasible: street noise", "unzulässig: Straßenlärm", axes.c2p((self.low + lo) / 2, axes.y_range[1]) + _mn.DOWN * .3, size=15, color=WARN, width=2.6)''',
     '''        band_tag = _mn.VGroup(self._l("infeasible:", "unzulässig:", size=15, color=WARN, width=1.8), self._l("street noise", "Straßenlärm", size=15, color=WARN, width=1.8)).arrange(_mn.DOWN, buff=.05)
        band_tag.move_to(axes.c2p((self.low + lo) / 2, axes.y_range[1]) + _mn.DOWN * .4)''')
swap('''spot + _mn.DOWN * .5, size=16, color=GOOD, width=2.4)''', '''spot + _mn.DOWN * .3 + _mn.LEFT * .14, size=16, color=GOOD, width=2.4, edge="right")''')
swap('''        price = self._price([f"+{self.quantity(gap, 'kWh/a')}", f"+{self.quantity(gap * study['tariff'], 'CHF/a')}"], WARN,''',
     '''        price = self._price([self.tr("Courtyard price", "Hofpreis"), f"+{self.quantity(gap, 'kWh/a')}", f"+{self.quantity(gap * study['tariff'], 'CHF/a')}"], WARN,''')
swap('''        note = self._l("price of the courtyard", "Preis des Innenhofs", axes.c2p(stop, e_global) + _mn.DOWN * .38 + _mn.RIGHT * .6, size=15, color=WARN, width=2.4)
''', '')
swap('''_mn.FadeIn(price), _mn.FadeIn(note), seconds=1.6)''', '''_mn.FadeIn(price), seconds=1.6)''')
swap('''tag, level, bracket, price, note)''', '''tag, level, bracket, price)''')
swap('''self._t(text, (.2, -2.38, 0), size=14, color=MUTED, width=5.8)''', '''self._t(text, (-.4, -2.38, 0), size=14, color=MUTED, width=5.8)''')
swap('''{self.number(d['insulation'] * 100)} cm,",''', '''{self.number(d['insulation'] * 100)} cm",''')
swap('''Dämmung {self.number(d['insulation'] * 100)} cm,"''', '''Dämmung {self.number(d['insulation'] * 100)} cm"''')
swap('''        card = _mn.VGroup(frame, card_rows).move_to((1.0, .4, 0))
        best = {**self.fixed, "orientation": float(study["feasible_minimum"])}
        thumb = self.thumbnail(best, (-5.2, .4, 0), .5, ring=GOOD)
        self.step("scope", *[self._out(item) for item in content], _mn.FadeIn(card), _mn.FadeIn(thumb), seconds=2.2)
        smaller, larger = ({**self.fixed, "wwr": value} for value in (.3, .8))
        variants = _mn.VGroup(self.thumbnail(smaller, (-5.2, -1.6, 0), .3, state="ghost"), self._t("?", (-4.4, -1.6, 0), size=30, color=WARN), self.thumbnail(larger, (-3.6, -1.6, 0), .3, state="ghost"))
        self.step("scope_2", _mn.Indicate(card_rows[2], color=DESIGN), _mn.FadeIn(variants), seconds=1.8)''',
     '''        card = _mn.VGroup(frame, card_rows).move_to((1.6, 1.0, 0))
        plan = self.plan(self.dials, (-4.6, 1.0, 0), .27, ring_radius=6.0, sector=(study["reference"], lo, WARN), site=(180, 0))
        self.step("scope", *[self._out(item) for item in content], _mn.FadeIn(card), _mn.FadeIn(plan), seconds=2.2)
        smaller, larger = ({**self.fixed, "wwr": value} for value in (.3, .8))
        variants = _mn.VGroup(self.thumbnail(smaller, (-2.3, -1.7, 0), .28, state="ghost"), self.thumbnail(larger, (3.1, -1.7, 0), .28, state="ghost"),
                              self._l("Smaller window?", "Kleineres Fenster?", (-2.3, -2.4, 0), size=15, color=MUTED, width=2.4),
                              self._l("Larger window?", "Größeres Fenster?", (3.1, -2.4, 0), size=15, color=MUTED, width=2.4),
                              self._t("θ* = ?", (.4, -1.7, 0), size=30, color=WARN, width=2.4))
        self.step("scope_2", _mn.Indicate(card_rows[2], color=DESIGN), _mn.FadeIn(variants), seconds=1.8)''')
path.write_text(text, encoding="utf-8")
