import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:90]
    s = s.replace(old, new)


# analysis helper: a few member labels, no origin text
sub('''    def _villa_analysis(self, point, scale, *, ids=(), nodes=False, highlight=()):''', '''    def _villa_analysis(self, point, scale, *, tags=("A1", "A5", "E1", "E5", "C3"), nodes=False, highlight=()):''')
sub('''        view = self.analysis_view(self.analysis, point, scale, projection="axon", ids=ids, highlight=highlight)
        if not nodes:
            view.remove(view.nodes, view.supports)
        base.grid.submobjects[-1].shift(_engine.RIGHT * 0.45 + _engine.UP * 0.3)
        view.submobjects.insert(0, base)''', '''        view = self.analysis_view(self.analysis, point, scale, projection="axon", highlight=highlight)
        if not nodes:
            view.remove(view.nodes, view.supports)
        base.grid.remove(base.grid.submobjects[-1])
        view.add(*[self.text(tag, view.anchors[tag] + np.array([0.09, 0.0, 0.0]), size=15.5, color=DATA if tag == "C3" else INK, anchor="l") for tag in tags])
        view.submobjects.insert(0, base)''')
sub('''        struct.grid.submobjects[-1].shift(_engine.RIGHT * 0.45 + _engine.UP * 0.3)
        result = self._villa_analysis((4.7, 0.1, 0), 0.1, ids=("members",))''', '''        struct.grid.remove(struct.grid.submobjects[-1])
        result = self._villa_analysis((4.85, 0.1, 0), 0.1)''')
sub('''struct = self.villa_axon((-4.7, 0.1, 0), 0.1, disciplines=("structure",), grid=True, shell=0.4, ground=False)''', '''struct = self.villa_axon((-4.85, 0.1, 0), 0.1, disciplines=("structure",), grid=True, shell=0.4, ground=False)''')
sub('''(-4.7, 2.1, 0), size=17.5, color=STRUCT, width=3.4)''', '''(-4.85, 2.1, 0), size=17.5, color=STRUCT, width=3.4)''')
sub('''(4.7, 2.1, 0), size=17.5, color=GOOD, width=3.4)''', '''(4.85, 2.1, 0), size=17.5, color=GOOD, width=3.4)''')
sub('''origin = self._chip(origin_name, (-2.4, y, 0), STRUCT, 15.5, "l")''', '''origin = self._chip(origin_name, (-2.2, y, 0), STRUCT, 15.5, "l")''')
sub('''rule = VGroup(self._counter(tracker, (-2.55, y, 0), size=20.0), origin, arrow, goal, note)''', '''rule = VGroup(self._counter(tracker, (-2.3, y, 0), size=20.0), origin, arrow, goal, note)''')
sub('''(-2.5, y - 0.36 - number * 0.27, 0), size=15.5, color=MUTED, anchor="l", width=5.3)''', '''(-2.2, y - 0.36 - number * 0.27, 0), size=15.5, color=MUTED, anchor="l", width=5.0)''')
sub('''        struct.grid.submobjects[-1].shift(_engine.RIGHT * 0.45 + _engine.UP * 0.3)
        result = self._villa_analysis((3.9, 1.05, 0), 0.098, ids=("members",), highlight=("C3",))''', '''        struct.grid.remove(struct.grid.submobjects[-1])
        result = self._villa_analysis((3.9, 1.05, 0), 0.098, highlight=("C3",))''')

# architecture: ramp inset label above
sub('''        ramp = VGroup(self.villa_section((-5.75, 2.4, 0), 0.095, cut="ramp", layers=("structure", "architecture"), highlight=flights, thumbnail=True),
                      self.label("ramp section", "Rampenschnitt", (-5.75, 1.78, 0), size=15.5, color=MUTED, width=1.5))''', '''        ramp = VGroup(self.villa_section((-5.75, 2.1, 0), 0.085, cut="ramp", layers=("structure", "architecture"), highlight=flights, thumbnail=True),
                      self.label("ramp section", "Rampenschnitt", (-5.75, 2.78, 0), size=15.5, color=MUTED, width=1.6))''')

# midsurface: labels, detail geometry
sub('''                    self.text(analysis["slab"]["tag"], section.at(2.0, 3.5) + np.array([0.0, 0.3, 0.0]), size=16.2, color=INK),
                    self.text("C3", section.at(piloti["y"], 0.0) + np.array([0.0, -0.25, 0.0]), size=16.2, color=DATA))''', '''                    self.leader((-5.0, -0.8, 0), section.at(3.0, 3.5), analysis["slab"]["tag"] + self.tr(" first-floor slab", " Geschossdecke"), color=INK, size=16.2, anchor="l"),
                    self.text("C3", section.at(piloti["y"], 0.0) + np.array([0.0, -0.25, 0.0]), size=16.2, color=DATA))''')
sub('''zoom, cx, cy, reach, bottom = 2.0, 3.45, 0.45, 1.0, soffit - 0.8''', '''zoom, cx, cy, reach, bottom = 1.7, 3.9, 0.45, 1.0, soffit - 0.8''')
sub('''frame = frame_box(5.4, 3.3, MUTED, fill=0.0).move_to((cx + 0.35, cy - 0.15, 0))''', '''frame = frame_box(5.75, 3.3, MUTED, fill=0.0).move_to((3.875, cy - 0.15, 0))''')
sub('''(cx - 2.35, cy + 1.5, 0)), (joint + (0.2, -0.2, 0), (cx - 2.35, cy - 1.8, 0))]''', '''(1.0, cy + 1.5, 0)), (joint + (0.2, -0.2, 0), (1.0, cy - 1.8, 0))]''')

# supports condition text
sub('''                           self.text(first["condition"] + ": " + self.tr("translations and rotations restrained", "Verschiebungen und Verdrehungen gebunden"), (RIGHT - 2.8, 1.55, 0), size=16.2, color=INK, anchor="l", width=4.3),''', '''                           self.text(first["condition"], (RIGHT - 2.8, 1.55, 0), size=17.5, color=INK, anchor="l", width=4.0),''')
sub('''                           self.text(first["label"][language], (RIGHT - 2.8, 1.17, 0), size=15.5, color=MUTED, anchor="l", width=5.6))''', '''                           self.text(self.tr("translations and rotations restrained", "Verschiebungen und Verdrehungen gebunden"), (RIGHT - 2.8, 1.17, 0), size=16.2, color=MUTED, anchor="l", width=5.6))''')
p.write_text(s, encoding="utf-8")
print("ok")
