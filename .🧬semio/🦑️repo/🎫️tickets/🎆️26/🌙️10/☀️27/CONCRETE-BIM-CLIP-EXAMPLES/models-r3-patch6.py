import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:90]
    s = s.replace(old, new)


sub('''    def _timeline(self, *stages):''', '''    def _roof_note(self, point):
        """⬚ Legend for the dashed outline on first-floor plans: the roof garden opening above."""
        x, y, _ = P(point)
        return VGroup(polyline([(x, y - 0.07), (x + 0.3, y - 0.07), (x + 0.3, y + 0.07), (x, y + 0.07)], MUTED, 1.2, closed=True, dash=0.06),
                      self.label("dashed: roof garden opening above", "gestrichelt: Dachgartenöffnung darüber", (x + 0.42, y, 0), size=15.5, color=MUTED, anchor="l", width=5.0))

    def _timeline(self, *stages):''')
sub('''slab_note = self.tr(f"{one} IfcSlab ({slab_tag}) → {one} IfcStructuralSurfaceMember, meshed into {self.number(shells, 0)} shell elements",
                            f"{one} IfcSlab ({slab_tag}) → {one} IfcStructuralSurfaceMember, vernetzt in {self.number(shells, 0)} Schalenelemente")''',
    '''slab_note = self.tr(f"{one} IfcSlab · {slab_tag} → {one} IfcStructuralSurfaceMember, meshed into {self.number(shells, 0)} shell elements",
                            f"{one} IfcSlab · {slab_tag} → {one} IfcStructuralSurfaceMember, vernetzt in {self.number(shells, 0)} Schalenelemente")''')
sub('''result = self._villa_analysis((4.85, 0.1, 0), 0.1)''', '''result = self._villa_analysis((4.85, 0.0, 0), 0.1, tags=("C3",))''')
sub('''(-4.85, 2.1, 0), size=17.5, color=STRUCT, width=3.4)''', '''(-4.85, 2.45, 0), size=17.5, color=STRUCT, width=3.4)''')
sub('''(4.85, 2.1, 0), size=17.5, color=GOOD, width=3.4)''', '''(4.85, 2.45, 0), size=17.5, color=GOOD, width=3.4)''')
sub('''axon = self._villa_analysis((-3.4, 0.3, 0), 0.17, nodes=True)''', '''axon = self._villa_analysis((-3.4, 0.2, 0), 0.17, nodes=True, tags=("C3",))''')

# dashed legends
sub('''        chip = VGroup(frame_box(5.9, 0.95, GOOD).move_to((RIGHT, -0.72, 0)),''', '''        roof = self._roof_note((1.2, -1.75, 0))
        chip = VGroup(frame_box(5.9, 0.95, GOOD).move_to((RIGHT, -0.72, 0)),''')
sub('''(0.6, [FadeIn(chip)]))
        self.stage = VGroup(plan, mark, link, sides, live, names, chip)''', '''(0.6, [FadeIn(chip), FadeIn(roof)]))
        self.stage = VGroup(plan, mark, link, sides, live, names, chip, roof)''')
sub('''        self._show("members", (0.9, [self._leave(), FadeIn(base)]), *steps, (0.8, [FadeIn(notes)]))
        self.stage = VGroup(base, rows, notes, mesh,''', '''        roof = self._roof_note((0.95, -2.05, 0))
        self._show("members", (0.9, [self._leave(), FadeIn(base)]), *steps, (0.8, [FadeIn(notes), FadeIn(roof)]))
        self.stage = VGroup(base, rows, notes, roof, mesh,''')
sub('''        chosen = VGroup(self.label("chosen assumption"''', '''        roof = self._roof_note((0.95, -2.3, 0))
        chosen = VGroup(roof, self.label("chosen assumption"''')
sub('''(1.0, [FadeIn(condition), FadeIn(sketches), FadeIn(chosen)]))''', '''(1.0, [FadeIn(condition), FadeIn(sketches), FadeIn(chosen)]))''')
sub('''(RIGHT, -1.75, 0), size=15.5, color=MUTED, width=5.8))
        spacing = _model.clearance()["grid_spacing"]''', '''(RIGHT, -1.75, 0), size=15.5, color=MUTED, width=5.8), self._roof_note((0.95, -2.2, 0)))
        spacing = _model.clearance()["grid_spacing"]''')
p.write_text(s, encoding="utf-8")
print("ok")
