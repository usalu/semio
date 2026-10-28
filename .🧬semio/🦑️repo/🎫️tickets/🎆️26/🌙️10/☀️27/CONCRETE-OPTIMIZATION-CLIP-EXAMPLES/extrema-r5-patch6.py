import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
def swap(old, new):
    global s
    assert s.count(old) == 1, old[:70]
    s = s.replace(old, new)
swap("header = _room.place(header, at(9.0, 4.2), \"right\")", "header = _room.place(header, at(7.7, 3.9), \"right\")")
swap('_room.text([self.tr("sun on glass", "Sonne am Glas"), _room.Number(_room.number(found["sun_" + season] * 100, 0, german)), "%"], size=14, color=color, width=1.75)',
     '_room.text([self.tr("sun on glass", "Glasstrahlung"), _room.Number(_room.number(found["sun_" + season] * 100, 0, german)), "%"], size=14, color=color, width=1.75)')
swap("winter, summer = self._focus((-4.45, -1.44, 0), (5.0, 2.28), WARN)", "winter, summer = self._focus((-4.45, -1.39, 0), (5.0, 2.38), WARN)")
swap("big = self.room_section(self.dials, (-2.45, -.15, 0), .5, \"winter\", demand=False)", "big = self.room_section(self.dials, (-2.2, -.15, 0), .5, \"winter\", demand=False)")
a = s.index("        tag = self._t(f\"{self.tr('maximum', 'Maximum')} {self.peak}°")
b = s.index("\n", a)
s = s[:a] + """        tag = _mn.VGroup(self._t(f"{self.tr('maximum', 'Maximum')} {self.peak}°", size=14, color=WARN, width=1.5), self._t(self._kwh(self.peak_energy), size=14, color=WARN, width=1.5)).arrange(_mn.DOWN, buff=.04)
        _room.place(tag, (at[0] - .55 - tag.width / 2, at[1] + .22, 0), "center")""" + s[b:]
swap("rows_y = (-.35, -1.0, -1.65)", "rows_y = (-.2, -.85, -1.5)")
swap("strip = self.ramp_strip(colour, (5.3, -2.15, 0)", "strip = self.ramp_strip(colour, (5.3, -2.25, 0)")
swap("(3.75, -.82, 0), size=15, color=WARN, width=2.0, edge=\"left\")", "(3.75, -.5, 0), size=15, color=WARN, width=2.0, edge=\"left\")")
open(p, "w", encoding="utf-8").write(s)
print("ok")
