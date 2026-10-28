import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:90]
    s = s.replace(old, new)


sub('''            origin = self._chip(origin_name, (-2.4, y, 0), STRUCT, 16.2, "l")
            arrow = Arrow((-0.95, y, 0), (-0.65, y, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.5)
            goal = self._chip(goal_name, (-0.55, y, 0), GOOD, 16.2, "l")''',
    '''            origin = self._chip(origin_name, (-2.4, y, 0), STRUCT, 15.5, "l")
            start = origin.get_right()[0] + 0.05
            arrow = Arrow((start, y, 0), (start + 0.28, y, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.5)
            goal = self._chip(goal_name, (start + 0.36, y, 0), GOOD, 15.5, "l")''')
sub('''        view.submobjects.insert(0, base)
        view.base = base
        return view''', '''        base.grid.submobjects[-1].shift(_engine.RIGHT * 0.45 + _engine.UP * 0.3)
        view.submobjects.insert(0, base)
        view.base = base
        return view''')
sub('''        struct = self.villa_axon((-4.7, 0.1, 0), 0.1, disciplines=("structure",), grid=True, shell=0.4, ground=False)
        result = self._villa_analysis((4.7, 0.1, 0), 0.1, ids=("members",))''', '''        struct = self.villa_axon((-4.7, 0.1, 0), 0.1, disciplines=("structure",), grid=True, shell=0.4, ground=False)
        struct.grid.submobjects[-1].shift(_engine.RIGHT * 0.45 + _engine.UP * 0.3)
        result = self._villa_analysis((4.7, 0.1, 0), 0.1, ids=("members",))''')
sub('''        struct = self.villa_axon((-3.9, 1.05, 0), 0.098, disciplines=("structure",), grid=True, highlight=("C3",), ground=False)''', '''        struct = self.villa_axon((-3.9, 1.05, 0), 0.098, disciplines=("structure",), grid=True, highlight=("C3",), ground=False)
        struct.grid.submobjects[-1].shift(_engine.RIGHT * 0.45 + _engine.UP * 0.3)''')
sub('''section = self.villa_section((-4.0, 0.6, 0), 0.27,''', '''section = self.villa_section((-4.45, 0.6, 0), 0.24,''')
sub('''self.scale_bar((-6.4, -1.3, 0), 0.27, 5.0, steps=2)''', '''self.scale_bar((-6.5, -1.3, 0), 0.24, 5.0, steps=2)''')
p.write_text(s, encoding="utf-8")
print("ok")
