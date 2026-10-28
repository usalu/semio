"""🩹 Patch the extrema scene: fixed readout chip, larger section, taller bar closer to the plot, coloured scope pictograms."""
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:70])
    text = text.replace(old, new)


swap("AXES_AT, AXES_SIZE = np.array([.2, .4, 0.]), (4.2, 3.1)", "AXES_AT, AXES_SIZE = np.array([-.05, .4, 0.]), (4.2, 3.1)")
swap("BAR_AT, BAR_HEIGHT = np.array([4.6, -2.0, 0.]), 3.4", "BAR_AT, BAR_HEIGHT = np.array([4.35, -2.1, 0.]), 4.4")
swap("SECTION_BOX = (-7.0, -2.55, -3.0, -.95)", "SECTION_BOX = (-7.0, -2.55, -2.95, -.85)")
swap("self.room_section(dials, point, scale, kind, labels=False)", "self.room_section(dials, point, scale, kind, labels=False, demand=False)")
swap("self.room_section(self.dials, point, scale, season, labels=False)", "self.room_section(self.dials, point, scale, season, labels=False, demand=False)")
swap("(-.4, -2.38, 0), size=14, color=MUTED, width=5.8)", "(0, -2.38, 0), size=14, color=MUTED, width=5.6)")
swap('''        marker = self.live_marker(axes, self.dials, "orientation", "electricity", x_label="θ", x_unit="°", y_unit="kWh/a")
        trace''', '''        marker = self._marker(axes, lambda d: _room.result(d)["electricity"], "electricity")
        trace''')
swap('''        marker2 = self.live_marker(axes2, self.dials, "orientation", lambda d: self.saving(d["orientation"]), x_label="θ", x_unit="°", y_label="S", y_unit="kWh/a", color=INK)''',
     '''        marker2 = self._marker(axes2, lambda d: self.saving(d["orientation"]), lambda d: self.saving(d["orientation"]), "S")''')
swap('''        marker = self.live_marker(axes, self.dials, "orientation", "electricity", x_label="θ", x_unit="°", y_unit="kWh/a")
        band''', '''        marker = self._marker(axes, lambda d: _room.result(d)["electricity"], "electricity")
        band''')
swap('''(1.3, 1.65, 0), size=20, color=GOOD, width=2.0)''', '''(.95, 1.05, 0), size=20, color=GOOD, width=2.0)''')
swap('''    def _legend(self, entries, left=None):''', '''    def _marker(self, axes, y_of, value_fn, label=""):
        """📍 Live marker with drop lines and a readout chip pinned to the upper right corner of the plot, clear of every other mark."""
        german = self._german()
        corner = np.array([axes.c2p(self.high, 0)[0], axes.c2p(0, axes.y_range[1])[1] - .22, 0.])
        dot = self.live_marker(axes, self.dials, "orientation", value_fn, readout=False)

        def build():
            design = _room.values(self.dials)
            chip = _room.chip(["θ", *_room.parts(design["orientation"], "°", 0, german), "·", *([label] if label else []), *_room.parts(y_of(design), "kWh/a", 0, german)], size=15, color=INK)
            return _mn.VGroup(_room.place(chip, corner, "right"))

        return _mn.VGroup(dot, self._live(build))

    def _legend(self, entries, left=None):''')

start = text.index("        best = {**self.fixed") if "        best = {**self.fixed" in text else None
a = text.index('        plan = self.plan(self.dials, (-4.6, 1.0, 0), .27')
b = text.index('        self.step("scope_2"')
b_end = text.index("\n", b) + 1
block = '''        plan = self.plan(self.dials, (-4.6, 1.0, 0), .27, ring_radius=6.0, sector=(study["reference"], lo, WARN), site=(180, 0))
        rated = {**self.fixed, "orientation": float(study["feasible_minimum"])}
        values = [_room.result({**rated, "wwr": wwr})["electricity"] for wwr in (.3, d["wwr"], .8)]
        colour = self.ramp(min(values), max(values))
        tiles = [(.3, -2.9), (d["wwr"], .5), (.8, 3.9)]
        pictos = [self.thumbnail({**rated, "wwr": wwr}, (x, -1.45, 0), .26, ramp=colour, ring=GOOD if wwr == d["wwr"] else None) for wwr, x in tiles]
        names = [self._t(self.tr(f"window {self.number(wwr * 100)} %", f"Fenster {self.number(wwr * 100)} %") + (self.tr(" · this design", " · dieser Entwurf") if wwr == d["wwr"] else ""),
                         (x, -2.3, 0), size=15, color=GOOD if wwr == d["wwr"] else MUTED, width=2.6) for wwr, x in tiles]
        strip = self.ramp_strip(colour, (5.0, -2.45, 0), width=2.2)
        self.step("scope", *[self._out(item) for item in content], _Appear(card), _Appear(plan), _Appear(pictos[1]), _Appear(names[1]), seconds=2.2)
        unknown = self._t("θ* = ?", (-1.2, -1.45, 0), size=28, color=WARN, width=1.8)
        unknown2 = self._t("θ* = ?", (2.2, -1.45, 0), size=28, color=WARN, width=1.8)
        self.step("scope_2", _mn.Indicate(card_rows[2], color=DESIGN), _Appear(pictos[0]), _Appear(pictos[2]), _Appear(names[0]), _Appear(names[2]), _Appear(unknown), _Appear(unknown2), _Appear(strip), seconds=1.8)
'''
text = text[:a] + block + text[b_end:]
path.write_text(text, encoding="utf-8")
