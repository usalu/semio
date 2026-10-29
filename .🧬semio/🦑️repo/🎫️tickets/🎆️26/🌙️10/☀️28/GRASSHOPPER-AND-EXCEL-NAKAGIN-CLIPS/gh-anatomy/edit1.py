import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('def _sheet_view(self, definition, solution, view, shift=(0.0, 0.0), gap=(0.9, 0.8)):', 'def _sheet_view(self, definition, solution, view, shift=(0.0, 0.0), gap=(0.7, 0.6)):')
rep('wp = self.gh_panel(None, rows=self._rows(full.tree("wp")), at=_at(0.6, 1.4))', 'wp = self.gh_panel(None, rows=self._rows(full.tree("wp")), at=_at(1.0, 1.4))')
rep('fl = self.gh_param("Number", at=_at(-0.15, 1.4), label=self.tr("Width", "Breite"))', 'fl = self.gh_param("Number", at=_at(0.7, 1.4), label=self.tr("Width", "Breite"))')
rep('flp = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(1.7, 1.4))', 'flp = self.gh_panel(None, rows=self._rows(full.tree("flp")), at=_at(2.5, 1.4))')
rep('x_tag = self._tag(self.tr("input X · Number", "Eingang X · Number"), _at(-1.0, -1.0), color="#D8362B")', 'x_tag = self._tag(self.tr("input X · Number", "Eingang X · Number"), _at(-2.5, 1.45), color="#D8362B")')
# value list block
a = s.index('        names = [f"{self.tr(\'Type\'')
b = s.index('        sun = self.gh_ribbon(')
new = '''        names = [f"{self.tr('Type', 'Typ')} {t.key}" for t in _nk.capsule_types()]
        ref = self.gh_value_list(names, 0, at=_at(-1.9, 0.0), name="Value List", is_open=True)
        vlist = self.live(lambda: self.gh_value_list(names, int(self.vl_index.get_value()), at=_at(-1.9, 0.0), name="Value List", is_open=True), lambda: int(self.vl_index.get_value()))
        vlp = self.live(lambda: self.gh_panel(None, rows=self._rows(_gh.evaluate(defn, {"vl": int(self.vl_index.get_value())}).tree("vlp")), at=_at(0.4, 0.0)), lambda: int(self.vl_index.get_value()))
        vl_wire = self.gh_wire(ref.grip_out().get_center(), _at(-0.1, 0.0))
        type_note = self._type_note()
        self.vl_index.set_value(0)
        self.view_state.update(window=True, dims=(), scale=0.5)
        self.step("valuelist", _engine.FadeOut(toggle_on), _engine.FadeOut(tg_panel), _engine.FadeOut(tg_wire), _engine.FadeIn(vlist), _engine.FadeIn(vlp), _engine.FadeIn(vl_wire),
                  _engine.FadeIn(type_note), self._ramp_index(self.vl_index, 3, 2.4), self._type_follow(), seconds=2.4)

'''
s = s[:a] + new + s[b:]
rep('''self.step("plugin", _engine.FadeOut(toggle_on), _engine.FadeOut(tg_panel), _engine.FadeOut(tg_wire), _engine.FadeOut(vlist), _engine.FadeOut(vlp), _engine.FadeOut(type_note),''', '''self.step("plugin", _engine.FadeOut(vlist), _engine.FadeOut(vlp), _engine.FadeOut(vl_wire), _engine.FadeOut(type_note),''')
rep('''fine = self._note(self.tr("install, then the new tab appears", "installieren, dann erscheint der neue Tab"), _at(0.0, -0.7))''', '''fine = _engine.VGroup(self._note(self.tr("install", "installieren"), _at(-2.5, -0.15)), self._note(self.tr("download", "herunterladen"), _at(0.0, -0.15)),
                              self._note(self.tr("plugin file", "Plugin-Datei"), _at(2.4, -0.15)), self._note(self.tr("then the new tab appears in the ribbon", "dann erscheint der neue Tab im Ribbon"), _at(0.0, -0.8)))''')
rep('text = f"{self.tr(\'Type\', \'Typ\')} {t.key} · {t.name[self.language()]} · Ø {self.number(t.window_diameter, 1)} m"\n            return _engine.VGroup(self._note(text, VIEW + np.array([0.0, -VIEW_H / 2 + 0.26, 0.0])))',
    'text = f"{self.tr(\'Type\', \'Typ\')} {t.key} · {t.name[self.language()]} · Ø {self.number(t.window_diameter, 1)} m"\n            return _engine.VGroup(self._note(text, VIEW + np.array([VIEW_W / 2 - 0.12, -VIEW_H / 2 + 0.26, 0.0]), anchor="r"))')
# tree section
rep('t_param = self.gh_param("Brep", at=_at(-2.6, 0.0), label=self.tr("Capsules", "Kapseln"))', 't_param = self.gh_param("Brep", at=_at(-3.2, 0.0), label=self.tr("Capsules", "Kapseln"))')
rep('tree_panel = self.gh_panel(None, rows=self._tower_rows(branches), at=_at(0.4, 0.0), width=2.6)', 'tree_panel = self.gh_panel(None, rows=self._tower_rows(branches), at=_at(-0.9, 0.0), width=2.4)')
rep('''explain = _engine.VGroup(self._note(self.tr("branch = one facade of a storey", "Ast = eine Fassade eines Geschosses"), _at(2.6, 0.55), anchor="c"),
                                 self._note(self.tr("path = its address", "Pfad = seine Adresse"), _at(2.6, 0.1), anchor="c"),
                                 self._note(self.tr("index = place in the branch", "Index = Platz im Ast"), _at(2.6, -0.35), anchor="c"))''',
    '''explain = _engine.VGroup(self._note(self.tr("branch = one facade", "Ast = eine Fassade"), _at(1.1, 0.55), anchor="l"),
                                 self._note(self.tr("path = its address", "Pfad = seine Adresse"), _at(1.1, 0.1), anchor="l"),
                                 self._note(self.tr("index = place in branch", "Index = Platz im Ast"), _at(1.1, -0.35), anchor="l"))''')
rep('self.tower_state.update(lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch")\n        self.step("tree", *[_engine.FadeOut(m) for m in (st_slider, cnt_slider, series, list_panel, series_ribbon, *wires2)],',
    'self.tower_state.update(towers=(0,), scale=0.1, lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch")\n        self.step("tree", _engine.FadeIn(ribbon), *[_engine.FadeOut(m) for m in (st_slider, cnt_slider, series, list_panel, series_ribbon, *wires2)],')
rep('bad = self._tag(self.tr(f"[1] fits {tower_count} branches of Tower A", f"[1] passt zu {tower_count} Ästen von Turm A"), _at(2.6, -1.1), color="#D8362B")', 'bad = self._tag(self.tr(f"[1] fits {tower_count} branches", f"[1] passt zu {tower_count} Ästen"), _at(2.7, -0.9), color="#D8362B")')
rep('good = self._tag(self.tr("{0;0;3;1}[1] = one capsule", "{0;0;3;1}[1] = eine Kapsel"), _at(2.2, -1.1), color="#2E8B2E")', 'good = self._tag(self.tr("{0;0;3;1}[1] = one capsule", "{0;0;3;1}[1] = eine Kapsel"), _at(2.7, -0.9), color="#2E8B2E")')
rep('pitch = 0.4\n        rows', 'pitch = 0.34\n        rows')
rep('table = self._table(self.TOOL_TERMS, _at(0.0, -0.1))', 'table = self._table(self.TOOL_TERMS, _at(0.0, -0.2))\n        self.tower_state.update(lit=lambda: None, key=lambda: "none")')
open(p, "w", encoding="utf-8").write(s)
