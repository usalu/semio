import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('_at(SL_X, 0.4))', '_at(SL_X, 0.1))')
rep('"Bereich 1,0 bis 5,0 · eine Dezimalstelle"', '"Bereich 1,0 bis 5,0 · 1 Dezimalstelle"')
rep('vol = self.gh_component("Volume", at=_at(2.5, 0.35))', 'vol = self.gh_component("Volume", at=_at(2.15, 0.35))')
rep('at=_at(3.85, 0.35))', 'at=_at(3.75, 0.35), width=1.15)')
rep('f"[1] fits all {len(tower_branches)} branches · {lit_count} capsules", f"[1] passt zu allen {len(tower_branches)} Ästen · {lit_count} Kapseln"), _at(2.55, -0.9)', 'f"[1] fits {len(tower_branches)} branches · {lit_count} capsules", f"[1] passt zu {len(tower_branches)} Ästen · {lit_count} Kapseln"), _at(2.3, -0.9)')
rep('"path {0;0;3;1}, index [1]: one capsule", "Pfad {0;0;3;1}, Index [1]: eine Kapsel"), _at(2.55, -0.9)', '"{0;0;3;1}, index 1: one capsule", "{0;0;3;1}, Index 1: eine Kapsel"), _at(2.3, -0.9)')
rep('self._num(7, mini_panel.get_right() + np.array([0.4, 0.0, 0.0]))', 'self._num(7, mini_panel.get_bottom() + np.array([0.0, -0.3, 0.0]))')
rep('self._note(other, at + np.array([1.25, y, 0.0])', 'self._note(other, at + np.array([1.1, y, 0.0])')
rep('self._note(main, at + np.array([-0.3, y, 0.0])', 'self._note(main, at + np.array([-0.35, y, 0.0])')
open(p, "w", encoding="utf-8").write(s)
