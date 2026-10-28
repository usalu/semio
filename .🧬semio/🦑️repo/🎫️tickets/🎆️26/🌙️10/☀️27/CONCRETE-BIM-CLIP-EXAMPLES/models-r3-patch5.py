import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:90]
    s = s.replace(old, new)


sub('''tags=("A1", "A5", "E1", "E5", "C3"), nodes=False, highlight=()):''', '''tags=(), nodes=False, highlight=()):''')
sub('''        view.add(*[self.text(tag, view.anchors[tag] + np.array([0.09, 0.0, 0.0]), size=15.5, color=DATA if tag == "C3" else INK, anchor="l") for tag in tags])''',
    '''        view.add(*[self.leader((view.anchors[tag][0], view.get_top()[1] + 0.3, 0), view.anchors[tag], tag, color=DATA, size=16.2, anchor="c") for tag in tags])''')
sub('''        result = self._villa_analysis((3.9, 1.05, 0), 0.098, highlight=("C3",))''', '''        result = self._villa_analysis((3.9, 1.05, 0), 0.098, highlight=("C3",), tags=("C3",))''')
sub('''        axon = self._villa_analysis((-3.4, 0.2, 0), 0.17, nodes=True)''', '''        axon = self._villa_analysis((-3.4, 0.1, 0), 0.17, nodes=True)''')

# schema header widths per column
sub('''            head = VGroup(self.text(info["name"][language], (x + 0.2, 1.95, 0), size=17.0, color=tone, anchor="l", width=1.5),''', '''            head = VGroup(self.text(info["name"][language], (x + 0.2, 1.95, 0), size=17.0, color=tone, anchor="l", width=(1.95, 1.55, 1.7)[DISCIPLINES.index(name)]),''')

# midsurface
sub('''self.leader((-5.0, -0.8, 0), section.at(3.0, 3.5),''', '''self.leader((-5.0, -1.1, 0), section.at(3.0, 3.5),''')
sub('''self.scale_bar((-6.5, -1.3, 0), 0.24, 5.0, steps=2)''', '''self.scale_bar((-6.5, -1.55, 0), 0.24, 5.0, steps=2)''')
sub('''zoom, cx, cy, reach, bottom = 1.7, 3.9, 0.45, 1.0, soffit - 0.8''', '''zoom, cx, cy, reach, bottom = 1.6, 3.9, 0.45, 1.0, soffit - 0.8''')
sub('''self.quantity(half, "m", 3), offset=-0.3, color=DATA, size=16.2)''', '''self.quantity(half, "m", 3), offset=-0.12, color=DATA, size=16.2)''')
sub('''"t = " + self.quantity(slab["thickness"], "m"), offset=0.2, color=DATA, size=16.2)''', '''self.quantity(slab["thickness"], "m"), offset=0.15, color=DATA, size=16.2)''')
p.write_text(s, encoding="utf-8")
print("ok")
