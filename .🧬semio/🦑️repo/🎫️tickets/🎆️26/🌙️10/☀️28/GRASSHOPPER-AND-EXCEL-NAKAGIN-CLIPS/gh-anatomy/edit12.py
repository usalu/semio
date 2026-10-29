import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('dim_offset=0.62)', 'dim_offset=0.4)')
rep('at=VIEW + np.array([-0.15, -0.05, 0.0]), scale=state["scale"]', 'at=VIEW + np.array([-0.2, 0.12, 0.0]), scale=state["scale"]')
rep('"scale": 0.44,\n                           "definition"', '"scale": 0.42,\n                           "definition"')
rep('self.view_state.update(mode="box", window=False, dims=("width", "length", "height"), scale=0.5)', 'self.view_state.update(mode="box", window=False, dims=("width", "length", "height"), scale=0.46)')
rep('w_slider = self._slider("w", _at(SL_X, 1.0), color=DIMCOL["width"])\n        l_slider = self._slider("bl", _at(SL_X, 0.2), color=DIMCOL["length"])', 'w_slider = self._slider("w", _at(SL_X, 0.85), color=DIMCOL["width"])\n        l_slider = self._slider("bl", _at(SL_X, 0.05), color=DIMCOL["length"])')
rep('at=_at(SL_X, -0.75), max_lines=4)', 'at=_at(SL_X, -0.85), max_lines=4)')
rep('box = self.gh_component("Center Box", at=_at(*CB_AT), state=sol.state(key), nickname=True)', 'box = self.gh_component("Center Box", at=_at(CB_AT[0], -0.2), state=sol.state(key), nickname=True)')
rep('at=_at(2.6, CB_AT[1]), max_lines=4)', 'at=_at(2.6, -0.2), max_lines=4)')
rep('x_tag = self._tag(self.tr("input X · Number", "Eingang X · Number"), _at(-0.9, -0.95), color=RED)', 'x_tag = self._tag(self.tr("input X · Number", "Eingang X · Number"), _at(-1.3, 1.5), color=RED)')
rep('"a list sits in {0}, this tree deeper", "eine Liste liegt in {0}, dieser Baum tiefer"', '"a list: {0} · a tree: deeper", "Liste: {0} · Baum: tiefer"')
rep('f"[1] fits all {len(tower_branches)} branches: {lit_count} capsules lit", f"[1] passt zu allen {len(tower_branches)} Ästen: {lit_count} Kapseln leuchten"), _at(2.3, -0.9)', 'f"[1] fits all {len(tower_branches)} branches · {lit_count} capsules", f"[1] passt zu allen {len(tower_branches)} Ästen · {lit_count} Kapseln"), _at(2.55, -0.9)')
rep('"path {0;0;3;1}, index [1] = one capsule", "Pfad {0;0;3;1}, Index [1] = eine Kapsel"), _at(2.3, -0.9)', '"path {0;0;3;1}, index [1]: one capsule", "Pfad {0;0;3;1}, Index [1]: eine Kapsel"), _at(2.55, -0.9)')
rep('self._note(self.tr("Name", "Name"), _at(-4.35, 1.5)), self._note(self.tr("Value", "Wert"), _at(-3.35, 1.5))', 'self._note(self.tr("Name", "Name"), _at(-3.95, 1.4)), self._note(self.tr("Value", "Wert"), _at(-2.95, 1.4))')
rep('self._note(self.tr("Domain 1.0 to 5.0 · one decimal", "Bereich 1,0 bis 5,0 · eine Dezimalstelle"), _at(SL_X, 0.45))', 'self._note(self.tr("Domain 1.0 to 5.0 · one decimal", "Bereich 1,0 bis 5,0 · eine Dezimalstelle"), _at(SL_X, 0.4))')
# legend mini canvas
rep('mini_slider = self._slider("w", _at(-3.05, 0.75), width=2.6, color=cw)\n        mini_box = self.gh_component("Center Box", at=_at(-0.55, 0.75), nickname=True)\n        mini_param = self.gh_param("Number", at=_at(-3.05, -0.6), label=self.tr("Width", "Breite"))\n        mini_panel = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(-0.55, -0.6))',
    'mini_slider = self._slider("w", _at(-3.2, 0.75), width=2.3, color=cw)\n        mini_box = self.gh_component("Center Box", at=_at(-0.7, 0.75), nickname=True)\n        mini_param = self.gh_param("Number", at=_at(-1.5, -0.65), label=self.tr("Width", "Breite"))\n        mini_panel = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(0.1, -0.65))')
rep('self._num(7, mini_panel.get_top() + np.array([0.0, 0.3, 0.0]))', 'self._num(7, mini_panel.get_right() + np.array([0.4, 0.0, 0.0]))')
rep('self._num(3, mini_param.get_left() + np.array([-0.35, 0.0, 0.0]))', 'self._num(3, mini_param.get_left() + np.array([-0.4, 0.0, 0.0]))')
rep('table = self._table(self.TOOL_TERMS, _at(2.4, -0.05))', 'table = self._table(self.TOOL_TERMS, _at(2.55, -0.05))')
open(p, "w", encoding="utf-8").write(s)
