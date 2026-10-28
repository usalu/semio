import sys
from pathlib import Path

path = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py")
text = path.read_text(encoding="utf-8")
start = text.index("        self.rooms = [self.room_section(self.focus, at, ROOM_SCALE, season, labels=False) for at, season in zip(ROOMS_AT, (\"winter\", \"summer\"))]\n        self.bar = self.energy_bars(self.focus, BAR_AT, 3.0, 900, compact=True, intensity=False)\n        self.ledger = self._ledger()\n        self.design_chip")
end = text.index("        self.pointer = self._pointer()")
text = text[:start] + "        self._mode_a()\n" + text[end:]
text = text.replace("self.energy_bars(self.focus, BAR_AT, 3.0, 900, compact=True, intensity=False)", "self.energy_bars(self.focus, BAR_AT, BAR_H, 900, compact=True, intensity=False)")
text = text.replace("BAR_AT = (6.1, -2.0, 0)\n", "BAR_AT, BAR_H = (6.1, -2.0, 0), 2.6\n")
anchor = "    def _card(self, item, point,"
helpers = '''    def _seasons(self):
        """☀️ Season tags under the two rooms with the noon sun altitude drawn in them."""
        return mn.VGroup(*[self._line([(self.tr(word, german), colour), self.tr("noon, sun altitude", "mittags, Sonnenhöhe") + " " + self.quantity(_room.sun_state(self.start["orientation"], season)["altitude"], "°", 0)],
                                      (at[0], -.33, 0), size=12, width=3.3) for at, season, word, german, colour in zip(ROOMS_AT, ("winter", "summer"), ("Winter", "Summer"), ("Winter", "Sommer"), (HEAT, COOL))])

    def _design_chip(self):
        """🪪 The design on the map as numbers: window share and overhang depth of the focus design."""
        return self._live_chip(lambda: ([self.tr("windows", "Fenster"), *self._p(self.focus["wwr"].get_value() * 100, "%", 1), "·", self.tr("overhang", "Vordach"),
                                         *self._p(self.focus["overhang"].get_value(), "m", 2)], (MAP_AT[0], 1.74, 0)), width=3.3, key=_room.design_key(self.focus), z=8)

'''
text = text.replace(anchor, helpers + anchor, 1)
text = text.replace("    def _card(self, item, point, *, width=1.22, label=None, digits=(0, 2), power=0, ghost=False, ring=None, color=None):", "    def _card(self, item, point, *, width=1.22, label=None, digits=(0, 2), power=0, ghost=False, ring=None, color=None, value=False):")
text = text.replace("        if not ghost:\n            group.add(_room.text(self._p(item[\"electricity\"], \"kWh/a\", power)", "        if value or not ghost:\n            group.add(_room.text(self._p(item[\"electricity\"], \"kWh/a\", power)")
text = text.replace("    def _result(self, name, design, electricity, runs, colour, strip, *, top=-1.45, extra=()):", "    def _result(self, name, design, electricity, runs, colour, strip, *, top=-1.62, extra=()):")
text = text.replace("(CARD_X, top - .46, 0), color=colour, width=3.9)", "(CARD_X, top - .4, 0), color=colour, width=3.9)").replace("(CARD_X, top - .92, 0), color=GOOD, width=3.9))", "(CARD_X, top - .8, 0), color=GOOD, width=3.9))")
text = text.replace("        if ghost is not None:\n            self.ghosts.append(ghost)", "        if ghost is not None:\n            self.ghosts.append(ghost)\n            self.trails[name] = ghost")
text = text.replace("self.path, self.colour, self.painted, self.ghosts, self.result_chips = [], INK, [], [], mn.VGroup()", "self.path, self.colour, self.painted, self.ghosts, self.result_chips, self.trails = [], INK, [], [], mn.VGroup(), {}")
path.write_text(text, encoding="utf-8")
print("ok")
