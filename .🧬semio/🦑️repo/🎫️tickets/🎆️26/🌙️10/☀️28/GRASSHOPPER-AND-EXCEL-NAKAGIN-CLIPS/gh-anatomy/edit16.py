import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('cnt_slider = self.gh_slider("Count", 1,', 'cnt_slider = self.gh_slider(self.tr("Count", "Anzahl"), 1,')
rep('Ändert man Count von 9 auf 11, wächst die Liste um zwei', 'Ändert man den Slider Anzahl von 9 auf 11, wächst die Liste um zwei')
rep('        vl_wire = self.gh_wire(ref.grip_out().get_center(), _at(-0.1, 0.0))', '''        vl_wire = self.gh_wire(ref.grip_out().get_center(), _at(-0.1, 0.0))
        vl_note = self.live(lambda: _engine.VGroup(self._note(f"{self.tr('index', 'Index')} {int(self.vl_index.get_value())} → {self.tr('Type', 'Typ')} {'ABCD'[int(self.vl_index.get_value())]}", _at(0.4, -0.6), color=SOFT)),
                            lambda: int(self.vl_index.get_value()))''')
rep('_engine.FadeIn(vl_wire),\n                  _engine.FadeIn(type_note)', '_engine.FadeIn(vl_wire), _engine.FadeIn(vl_note),\n                  _engine.FadeIn(type_note)')
rep('_engine.FadeOut(vl_wire), _engine.FadeOut(type_note)', '_engine.FadeOut(vl_wire), _engine.FadeOut(vl_note), _engine.FadeOut(type_note)')
open(p, "w", encoding="utf-8").write(s)
