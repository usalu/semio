import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert old in s, old[:80]
    s = s.replace(old, new)


sub('''    def _counter(self, tracker, point,''', '''    def _wrap(self, text, limit=58) -> list:
        """↩️ Break a long sentence into two balanced lines at a space."""
        if len(text) <= limit:
            return [text]
        words = text.split(" ")
        best = min(range(1, len(words)), key=lambda index: abs(len(" ".join(words[:index])) - len(" ".join(words[index:]))))
        return [" ".join(words[:best]), " ".join(words[best:])]

    def _counter(self, tracker, point,''')

sub('''spec = [("IfcColumn", ("pilotis · teaching choice, sources disagree", "Pilotis · Lehrentscheidung, Quellen uneins")),''',
    '''spec = [("IfcColumn", ("pilotis (teaching choice)", "Pilotis (Lehrentscheidung)")),''')
sub('''        chip = VGroup(frame_box(5.9, 0.46, GOOD).move_to((RIGHT, -0.55, 0)),
                      self.text(f"{pset}.LoadBearing = {'TRUE' if flag['value'] else 'FALSE'} · C3", (RIGHT, -0.55, 0), size=17.0, color=GOOD, width=5.6))''',
    '''        piloti_note = self.label("The sources disagree on the piloti count.", "Die Quellen sind uneins über die Pilotizahl.", (0.95, -0.38, 0), size=16.2, color=MUTED, anchor="l", width=5.6)
        chip = VGroup(frame_box(5.9, 0.46, GOOD).move_to((RIGHT, -0.85, 0)),
                      self.text(f"{pset}.LoadBearing = {'TRUE' if flag['value'] else 'FALSE'} · C3", (RIGHT, -0.85, 0), size=17.0, color=GOOD, width=5.6))''')
sub('''(0.95, -1.05, 0), size=16.2, color=MUTED, anchor="l"),
                         *self._lines([" · ".join(f"{name} ×{self.number(count, 0)}" for name, count in list(missing.items())[:3]),''',
    '''(0.95, -1.4, 0), size=16.2, color=MUTED, anchor="l"),
                         *self._lines([" · ".join(f"{name} ×{self.number(count, 0)}" for name, count in list(missing.items())[:3]),''')
sub('''                                      (0.95, -1.4, 0), size=17.0, color=WARN, pitch=0.32, width=5.7))
        ghost_parts''', '''                                      (0.95, -1.72, 0), size=17.0, color=WARN, pitch=0.3, width=5.7))
        ghost_parts''')
sub('''                   (0.7, [FadeIn(total)]),
                   (0.9, [FadeIn(chip), FadeIn(ring), FadeIn(leader), Indicate(struct.parts["C3"], color=DATA, scale_factor=1.6)]),''',
    '''                   (0.7, [FadeIn(total), FadeIn(piloti_note)]),
                   (0.9, [FadeIn(chip), FadeIn(ring), FadeIn(leader), Indicate(struct.parts["C3"], color=DATA, scale_factor=1.6)]),''')
sub('''self.stage = VGroup(struct, self.ghost, card, rows, total, chip, ring, leader, omitted)''', '''self.stage = VGroup(struct, self.ghost, card, rows, total, piloti_note, chip, ring, leader, omitted)''')

sub('''            note = self.text(item[language], (-2.5, y - 0.4, 0), size=15.0, color=MUTED, anchor="l", width=5.3)''',
    '''            note = VGroup(*[self.text(line, (-2.5, y - 0.36 - number * 0.23, 0), size=15.5, color=MUTED, anchor="l", width=5.3) for number, line in enumerate(self._wrap(item[language], 52))])''')
sub('''            y, tracker = 1.3 - index * 0.95, ValueTracker(0)
            origin = self._chip(item["source_class"], (-2.4, y, 0), STRUCT, 16.2, "l")''', '''            y, tracker = 1.35 - index * 1.0, ValueTracker(0)
            origin = self._chip(item["source_class"], (-2.4, y, 0), STRUCT, 16.2, "l")''')
sub('''        carry = VGroup(Arrow((-2.8, -1.85, 0), (2.8, -1.85, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.1),''',
    '''        carry = VGroup(Arrow((-2.8, -1.95, 0), (2.8, -1.95, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.1),''')
sub('''(0, -1.58, 0), size=16.2, color=MUTED, width=5.5))
        self._show("translation"''', '''(0, -1.68, 0), size=16.2, color=MUTED, width=5.5))
        self._show("translation"''')

sub('''(4.9, True, ("fixed: nothing moves", "eingespannt: nichts bewegt sich"))''', '''(4.9, True, ("fixed: nothing moves", "eingespannt: starr"))''')
sub('''mark = np.array([centre[0] - radius + 0.62, middle[1] + 0.5, 0.0])''', '''mark = np.array([centre[0] - radius + 0.5, middle[1] + 0.5, 0.0])''')
sub('''self.quantity(half, "m", 3), offset=-0.2, color=DATA, size=16.2)''', '''self.quantity(half, "m", 3), offset=-0.3, color=DATA, size=16.2)''')

sub('''left = self._property_card("architecture", 1.85,''', '''left = self._property_card("architecture", 2.05,''')
sub('''right = self._property_card("structure", -0.15,''', '''right = self._property_card("structure", -0.35,''')
sub('''ties = VGroup(segments([((RIGHT, 0.85 + 0.38, 0), (RIGHT, 1.85 - 0.62, 0)), ((RIGHT, 0.85 - 0.38, 0), (RIGHT, -0.15 + 0.62, 0))], DATA, 1.4))''',
    '''ties = VGroup(segments([((RIGHT, 0.85 + 0.4, 0), (RIGHT, 1.4, 0)), ((RIGHT, 0.85 - 0.4, 0), (RIGHT, 0.3, 0))], DATA, 1.4))''')
sub('''"ein Objekt, eine GlobalId", (RIGHT, -1.0, 0)''', '''"ein Objekt, eine GlobalId", (RIGHT, -1.3, 0)''')

sub('''            dot = Circle(radius=0.07, stroke_color=color, stroke_width=1.6, fill_color=color, fill_opacity=0.9 if color == GOOD else 0.0).move_to((1.1, y, 0))
            rows.add(VGroup(dot, shape))
            if color == WARN:
                strikes.add(segments([((1.3, y, 0), (1.4 + shape.width, y, 0))], WARN, 2.0))''',
    '''            if color == GOOD:
                dot = Circle(radius=0.07, stroke_color=color, stroke_width=1.6, fill_color=color, fill_opacity=0.9).move_to((1.1, y, 0))
            else:
                dot = segments([((1.03, y - 0.07, 0), (1.17, y + 0.07, 0)), ((1.03, y + 0.07, 0), (1.17, y - 0.07, 0))], WARN, 2.6)
            rows.add(VGroup(dot, shape))
            if color == WARN:
                strikes.add(shape)''')
sub('''FadeIn(heads[1]), FadeIn(strikes, run_time=0.8), FadeIn(same)]''', '''FadeIn(heads[1]), Indicate(strikes, color=WARN, scale_factor=1.0), FadeIn(same)]''')

sub('''struct = self.villa_axon((-3.9, 0.55, 0), 0.115, disciplines=("structure",), highlight=("C3",), ground=False)
        result = self.analysis_view(analysis, (3.9, 0.55, 0), 0.115, projection="axon", highlight=("C3",))''',
    '''struct = self.villa_axon((-3.9, 0.8, 0), 0.105, disciplines=("structure",), highlight=("C3",), ground=False)
        result = self.analysis_view(analysis, (3.9, 0.8, 0), 0.105, projection="axon", highlight=("C3",))''')
sub('''(-3.9, 2.15, 0), size=17.5, color=STRUCT, width=3.8)''', '''(-3.9, 2.5, 0), size=17.5, color=STRUCT, width=3.8)''')
sub('''(3.9, 2.15, 0), size=17.5, color=GOOD, width=3.8))''', '''(3.9, 2.5, 0), size=17.5, color=GOOD, width=3.8))''')
sub('''(-0.9, 1.5, 0), (0.9, 1.5, 0), buff=0.0''', '''(-0.9, 1.7, 0), (0.9, 1.7, 0), buff=0.0''')
sub('''(0.9, 0.95, 0), (-0.9, 0.95, 0), buff=0.0''', '''(0.9, 1.15, 0), (-0.9, 1.15, 0), buff=0.0''')
sub('''segments([((-0.2, 0.8, 0), (0.2, 1.1, 0)), ((-0.2, 1.1, 0), (0.2, 0.8, 0))], WARN, 3.0),
                         self.label("translation", "Übersetzung", (0, 1.8, 0), size=16.2, color=MUTED),
                         self.label("no automatic way back", "kein automatischer Rückweg", (0, 0.55, 0), size=15.5, color=WARN, width=2.4))''',
    '''segments([((-0.2, 1.0, 0), (0.2, 1.3, 0)), ((-0.2, 1.3, 0), (0.2, 1.0, 0))], WARN, 3.0),
                         self.label("translation", "Übersetzung", (0, 2.0, 0), size=16.2, color=MUTED),
                         self.label("no automatic way back", "kein automatischer Rückweg", (0, 0.75, 0), size=15.5, color=WARN, width=2.4))''')

p.write_text(s, encoding="utf-8")
print("ok")
