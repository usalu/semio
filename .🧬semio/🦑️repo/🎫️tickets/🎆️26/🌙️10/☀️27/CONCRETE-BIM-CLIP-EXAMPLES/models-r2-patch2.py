import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert old in s, old[:80]
    s = s.replace(old, new)


sub('''villa = self.villa_axon((-3.9, 0.3, 0), 0.15, colors="discipline", ground=False)''', '''villa = self.villa_axon((-4.55, 0.3, 0), 0.12, colors="discipline", ground=False)''')
sub('''            row = VGroup(self.text(head, (0.85, y, 0), size=16.2, color=WARN, anchor="l", width=2.35),
                         self.text(sentence, (3.3, y, 0), size=16.2, color=INK, anchor="l", width=3.4))''',
    '''            row = VGroup(self.text(head, (-2.0, y, 0), size=16.2, color=WARN, anchor="l", width=2.45),
                         self.text(sentence, (0.6, y, 0), size=16.2, color=INK, anchor="l", width=6.0))''')
sub('''line = Line((0.78, y, 0), target, stroke_width=0.8, color=WARN, stroke_opacity=0.5)''', '''line = Line((-2.08, y, 0), target, stroke_width=0.8, color=WARN, stroke_opacity=0.5)''')
sub('''        footer = VGroup(self._counter(gone, (0.85, -0.95, 0), size=22.0, color=WARN, anchor="r"),
                        self.label("objects are not in the analysis", "Objekte fehlen in der Analyse", (1.0, -0.95, 0), size=17.0, color=INK, anchor="l", width=5.4),
                        self._counter(stays, (0.85, -1.35, 0), size=22.0, color=GOOD, anchor="r"),
                        self.label(f"objects are kept as {self.number(analysis['counts']['members'], 0)} members", f"Objekte bleiben als {self.number(analysis['counts']['members'], 0)} Elemente erhalten", (1.0, -1.35, 0), size=17.0, color=INK, anchor="l", width=5.4),
                        self.label("together: the unique objects of the three models", "zusammen: die eindeutigen Objekte der drei Modelle", (1.0, -1.75, 0), size=16.2, color=MUTED, anchor="l", width=5.4))
        legend = self.label("Orange: not in the analysis", "Orange: nicht in der Analyse", (-3.9, -2.3, 0), size=16.2, color=WARN)''',
    '''        footer = VGroup(self._counter(gone, (-1.5, -0.95, 0), size=22.0, color=WARN, anchor="r"),
                        self.label("objects are not in the analysis", "Objekte fehlen in der Analyse", (-1.35, -0.95, 0), size=17.0, color=INK, anchor="l", width=6.0),
                        self._counter(stays, (-1.5, -1.4, 0), size=22.0, color=GOOD, anchor="r"),
                        self.label(f"objects are kept as {self.number(analysis['counts']['members'], 0)} members", f"Objekte bleiben als {self.number(analysis['counts']['members'], 0)} Elemente erhalten", (-1.35, -1.4, 0), size=17.0, color=INK, anchor="l", width=6.0),
                        self.label("together: the unique objects of the three models", "zusammen: die eindeutigen Objekte der drei Modelle", (-1.35, -1.85, 0), size=16.2, color=MUTED, anchor="l", width=6.0))
        legend = self.label("Orange: not in the analysis", "Orange: nicht in der Analyse", (-4.55, -2.0, 0), size=16.2, color=WARN)''')

sub('''layers["structure"].grid.submobjects[-1].shift(_engine.RIGHT * 0.55 + _engine.UP * 0.12)''', '''layers["structure"].grid.submobjects[-1].shift(_engine.LEFT * 0.35 + _engine.DOWN * 0.25)''')

sub('''        section = VGroup(polygon(strip, STRUCT, 0.3, STRUCT, 1.8), self.hatch(strip, STRUCT, 0.07, 45, 0.8, 0.8),''',
    '''        finish = [(x0, y0 + slab["thickness"] * zoom / 2, 0), (x0 + 2.8, y0 + slab["thickness"] * zoom / 2, 0), (x0 + 2.8, y0 + slab["thickness"] * zoom / 2 + 0.09, 0), (x0, y0 + slab["thickness"] * zoom / 2 + 0.09, 0)]
        section = VGroup(polygon(strip, STRUCT, 0.3, STRUCT, 1.8), self.hatch(strip, STRUCT, 0.07, 45, 0.8, 0.8), polygon(finish, GOOD, 0.8, GOOD, 1.2),''')
p.write_text(s, encoding="utf-8")
print("ok")
