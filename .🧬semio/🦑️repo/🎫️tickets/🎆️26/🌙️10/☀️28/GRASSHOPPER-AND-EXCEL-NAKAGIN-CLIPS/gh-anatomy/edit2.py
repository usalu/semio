import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('"towers": (0,), "scale": 0.1}', '"towers": (0,), "scale": 0.072}')
rep('towers=(1,), scale=0.085,', 'towers=(1,), scale=0.062,')
rep('towers=(0,), scale=0.1, lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1))', 'towers=(0,), scale=0.072, lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1))')
rep('VIEW + np.array([VIEW_W / 2 - 0.12, -VIEW_H / 2 + 0.26, 0.0]), anchor="r")', 'VIEW + np.array([VIEW_W / 2 - 0.12, VIEW_H / 2 - 0.24, 0.0]), anchor="r")')
rep('fl = self.gh_param("Number", at=_at(0.7, 1.4),', 'fl = self.gh_param("Number", at=_at(0.7, 1.55),')
rep('at=_at(2.5, 1.4))', 'at=_at(2.5, 1.5))')
rep('at=_at(1.0, 1.4))', 'at=_at(1.0, 1.5))')
rep('''        self.step("error", _engine.FadeOut(warning), _engine.FadeIn(error), seconds=1.4)''', '''        culprit = error.obj("er")
        first = _engine.VGroup(_engine.SurroundingRectangle(culprit, buff=0.08, color="#2E8B2E", stroke_width=3),
                               self._note(self.tr("first problem", "erstes Problem"), culprit.get_bottom() + np.array([0.0, -0.22, 0.0]), color="#2E8B2E"))
        self.step("error", _engine.FadeOut(warning), _engine.FadeIn(error), _engine.FadeIn(first), seconds=1.4)''')
rep('''self.view_state.update(visible=True, definition=self.three, box="cb")''', '''self.view_state.update(visible=True, definition=self.three, box="cb")
        self.remove(first)''')
open(p, "w", encoding="utf-8").write(s)
