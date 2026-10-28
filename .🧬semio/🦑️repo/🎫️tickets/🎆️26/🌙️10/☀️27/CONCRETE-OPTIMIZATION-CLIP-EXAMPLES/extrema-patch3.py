"""🩹 Patch the extrema scene: plot legend instead of point tags, header row in the neighbour chip."""
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:70]
    text = text.replace(old, new)


swap('''        start_tag = self._t(f"{self.tr('start', 'Start')} {self._bearing_name_any(start)}", axes.c2p(start, e_start) + _mn.UP * .32 + _mn.RIGHT * .1, size=15, color=INK, width=2.4, edge="left")''',
     '''        start_tag = self._l("start", "Start", axes.c2p(start, e_start) + _mn.UP * .3 + _mn.RIGHT * .4, size=15, color=INK, width=1.2)''')
swap('''        local_tag = self._l("local minimum", "lokales Minimum", axes.c2p(stop, e_local) + _mn.DOWN * .3 + _mn.RIGHT * .14, size=16, color=DATA, width=2.2, edge="left")
        rows = _mn.VGroup(*[''', '''        rows = _mn.VGroup(self._l("local minimum", "lokales Minimum", size=15, color=DATA, width=2.2), *[''')
swap('''        chip = _art.window_chip(rows, color=WARN).move_to(axes.c2p(stop, e_local) + _mn.DOWN * 1.0)''', '''        chip = _art.window_chip(rows, color=WARN).move_to(axes.c2p(stop, e_local) + _mn.DOWN * 1.1)''')
swap('''            probes.add(_mn.Dot(spot, radius=.06, color=WARN), _mn.Line(spot, chip.get_top() + _mn.RIGHT * (.3 if item["theta"] > stop else -.3), color=WARN, stroke_width=1.4))
        self.step("local_2", _mn.FadeIn(local), _mn.FadeIn(local_tag), _mn.FadeIn(probes), seconds=1.4)
        m["local"], m["local_tag"], m["probes"] = local, local_tag, probes''', '''            probes.add(_mn.Dot(spot, radius=.06, color=WARN))
        self.step("local_2", _mn.FadeIn(local), _mn.FadeIn(probes), seconds=1.4)
        m["local"], m["probes"] = local, probes''')
swap('''        square_tag = self._l("global minimum", "globales Minimum", glob_at + _mn.DOWN * .3 + _mn.RIGHT * .14, size=16, color=GOOD, width=2.4, edge="left")
''', '''        legend = self._legend([(_mn.Circle(radius=.08, color=DATA, stroke_width=3), self.tr("local minimum", "lokales Minimum"), DATA),
                               (_mn.Square(side_length=.16, color=GOOD, fill_color=GOOD, fill_opacity=1), self.tr("global minimum", "globales Minimum"), GOOD)])
''')
swap('''self._out(m["start"]), self._out(local_tag),''', '''self._out(m["start"]),''')
swap('''_mn.FadeIn(square), _mn.FadeIn(square_tag), _mn.Create(level), seconds=3)
        m["bars"], m["global"], m["global_tag"], m["level"] = compare_bars, square, square_tag, level''', '''_mn.FadeIn(square), _mn.FadeIn(legend), _mn.Create(level), seconds=3)
        m["bars"], m["global"], m["legend"], m["level"] = compare_bars, square, legend, level''')
swap('''    def _price(self, rows, color, point, edge="right"):''', '''    def _legend(self, entries):
        """🗝️ Marker legend in the free lower right corner of the plot."""
        axes = self.m["axes"]
        rows = [_mn.VGroup(shape, self._t(label, size=15, color=color, width=2.1)).arrange(_mn.RIGHT, buff=.15) for shape, label, color in entries]
        group = _mn.VGroup(*rows).arrange(_mn.DOWN, aligned_edge=_mn.LEFT, buff=.1)
        return _room.place(group, (axes.c2p(self.high, 0)[0] - .05, axes.c2p(0, axes.y_range[0])[1] + .5, 0), "right")

    def _price(self, rows, color, point, edge="right"):''')
swap('''clear = [m["cards"], m["route"], m["route_start"], m["local"], m["global"], m["global_tag"],''', '''clear = [m["cards"], m["route"], m["route_start"], m["local"], m["global"], m["legend"],''')
swap('''axes2.c2p(self.low, -70) + _mn.RIGHT * .1, size=14, color=WARN, width=2.4, edge="left")''', '''axes2.c2p(200, -70), size=14, color=WARN, width=2.4, edge="left")''')
swap('''        tag = self._l("feasible optimum", "zulässiges Optimum", spot + _mn.DOWN * .3 + _mn.LEFT * .14, size=16, color=GOOD, width=2.4, edge="right")
        self.step("feasible", _mn.FadeIn(optimum), _mn.FadeIn(tag), seconds=1.4)''', '''        self.step("feasible", _mn.FadeIn(optimum), _mn.FadeIn(legend), seconds=1.4)''')
swap('''tag, level, bracket, price)''', '''legend, level, bracket, price)''')
swap('''        cross = _art.cross_mark(WARN).move_to(glob_at)''', '''        cross = _art.cross_mark(WARN).move_to(glob_at)
        legend = self._legend([(_mn.Square(side_length=.16, color=GOOD, fill_color=GOOD, fill_opacity=1), self.tr("feasible optimum", "zulässiges Optimum"), GOOD),
                               (_mn.Square(side_length=.16, color=MUTED, fill_color=MUTED, fill_opacity=.3), self.tr("excluded", "ausgeschlossen"), MUTED)])''')
path.write_text(text, encoding="utf-8")
