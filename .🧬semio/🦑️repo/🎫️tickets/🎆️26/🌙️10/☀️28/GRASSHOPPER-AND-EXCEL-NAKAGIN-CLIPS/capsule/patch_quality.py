import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    if a not in s:
        print("MISSING", a[:70])
        return
    s = s.replace(a, b, 1)


a = s.index("        tower = self.tower_axon(at=VP")
b = s.index("        sliders = {key: self._node(key")
new = '''        slider_w = self.gh_slider(NAMES["w"], value("w"), d.obj("w").settings["min"], d.obj("w").settings["max"], at=(-5.2, 0.6, 0), tracker=vals["w"], width=SLIDER_W)
        center = self.gh_component("Center Box", at=(-1.7, 0.6, 0))
        wire_a = self.gh_wire(slider_w.grip_out(), center.grip_in("X"))
        outline = self.gh_group([slider_w, center], self.tr("One definition", "Eine Definition"), color="#9FD0F2")
        tower = self.tower_axon(at=VP + np.array([0.0, -0.1, 0.0]), scale=0.12, view="axon", lit=[(0, 1, 4, 1, 0)])
        lit = tower.capsules[(0, 1, 4, 1, 0)].get_center()
        callout = _engine.VGroup(_draft.polyline([lit, lit + np.array([-0.9, 0.55, 0.0])], DARK, 2.0, 0.9), self._tag(self.tr("one capsule", "eine Kapsel"), lit + np.array([-1.5, 0.6, 0.0])))
        counts = self.gh_panel([self.tr("Capsules ", "Kapseln ") + str(len(slots)), self.tr("Types ", "Typen ") + str(len(types)), self.tr("Definitions ", "Definitionen ") + "1"],
                               at=(-2.6, -1.4, 0), width=2.7)
        self.step("hook", _engine.AnimationGroup(_engine.FadeIn(tower), _engine.FadeIn(callout), _engine.FadeIn(counts), _engine.FadeIn(slider_w), _engine.FadeIn(center), _engine.FadeIn(wire_a),
                                                 _engine.FadeIn(outline)), seconds=4.0)

        rv_set = lambda **kw: _engine.AnimationGroup(*[self._to(rv[k], v, 1.2) for k, v in kw.items()])
        for name in ("grid", "lift"):
            rv[name].set_value(1.0)
        head_p = self._say(self.tr("Parameter", "Parameter"), (-5.2, 1.85), color="#E08A1E")
        head_d = self._say(self.tr("Dependency", "Abhängigkeit"), (-1.7, 1.85), color=DARK)
        head_g = self._tag(self.tr("Geometry", "Geometrie"), (VP[0], VP[1] + VP_H / 2 - 0.65), color="#1E7A3A", size=LABEL)
        wire_b = _draft.polyline([center.grip_out().get_center(), (2.45, 0.6, 0)], "#3F6FB5", 2.6, 0.95, dash=0.14)
        self.add(stage)
        self.step("parametric", _engine.Succession(
            _engine.AnimationGroup(*self._gone(tower), _engine.FadeOut(callout), _engine.FadeOut(counts), _engine.FadeIn(head_p), _engine.FadeIn(head_d), _engine.FadeIn(head_g), _engine.FadeIn(wire_b),
                                   self._to(rv["body"], 1.0, 1.5), self._to(rv["dw"], 1.0, 1.5)),
            self._ramp(vals["w"], value("w"), 3.1, 1.6), self._ramp(vals["w"], 3.1, value("w"), 1.4)), seconds=5.0)
        self.wait(0.2)

'''
s = s[:a] + new + s[b:]
rep('self.step("sliders", _engine.AnimationGroup(*self._gone(head_p, head_d, head_g, center, wire_a, wire_b, slider_w),', 'self.step("sliders", _engine.AnimationGroup(*self._gone(head_p, head_d, head_g, center, wire_a, wire_b, slider_w, outline),')
rep("rv_set(dl=1.0, dh=1.0, dd=1.0, dr=1.0, du=1.0)), seconds=5.0)", "rv_set(dl=1.0, dh=1.0, dd=1.0, dr=1.0, du=1.0, round=1.0, window=1.0)), seconds=5.0)")

a = s.index("        table = self._table((-0.85, 1.25))")
b = s.index("        for key in IDS:\n            vals[key].set_value(value(key))")
new = '''        table = self._table((-0.85, 1.25))
        minis = _engine.VGroup()
        for index, t in enumerate(types):
            x = -2.85 + 1.3 * index
            minis.add(self.capsule_axon({"key": t.key}, at=(x, -1.5, 0), scale=0.2, base=_nkd.type_color(t.key)), self._say(self.tr("Type ", "Typ ") + t.key, (x, -2.2), size=SMALL, color=_nkd.type_color(t.key)))
        self.step("types", _engine.AnimationGroup(*fout([card, limit_badge]), _engine.FadeIn(table), _engine.FadeIn(minis), self._ramp(vals["w"], width_hi, value("w"), 1.5), self._to(rv["env"], 0.0, 1.0),
                                                  self._to(rv["hit"], 0.0, 0.5)), seconds=5.0)

        order = [types[0], types[1], types[3], types[2]]
        ty = _engine.ValueTracker(0.0)

        def highlight():
            t = order[int(round(ty.get_value()))]
            rect = table.rows[t.key][0]
            return _engine.VGroup(_engine.Rectangle(width=rect.width, height=rect.height, stroke_color=_nkd.type_color(t.key), stroke_width=3, fill_color=_nkd.type_color(t.key),
                                                    fill_opacity=0.18).move_to(rect.get_center()))

        marker = self.live(highlight, key=lambda: int(round(ty.get_value())))
        typetag = self.live(lambda: _engine.VGroup(self._tag(self.tr("Same definition · type ", "Dieselbe Definition · Typ ") + order[int(round(ty.get_value()))].key,
                                                              (VP[0], VP[1] + VP_H / 2 - 0.75, 0.0), color=DARK, size=LABEL)), key=lambda: int(round(ty.get_value())))
        steps = [_engine.AnimationGroup(self._ramp(ty, float(i) - 1.0 if i else 0.0, float(i), 0.2), *[self._to(vals[k], v, 2.0) for k, v in (
            ("w", t.width), ("l", t.length), ("h", t.height), ("r", t.corner_radius), ("d", t.window_diameter), ("wu", t.window_u))]) for i, t in enumerate(order)]
        self.step("four", _engine.AnimationGroup(_engine.FadeIn(marker), _engine.FadeIn(typetag), self._to(rv["dd"], 1.0, 1.0), self._to(rv["du"], 1.0, 1.0), _engine.Succession(*steps)), seconds=9.0)

'''
s = s[:a] + new + s[b:]
rep("cur = col + [table, minis]", "cur = col + [table, minis, marker, typetag]")

rep('''_engine.Succession(self._to(cv["cap"], 1.0, 1.0), self._to(cv["arrow"], 1.0, 0.8), self._to(cv["glide"], 1.0, 3.0))), seconds=6.0)''',
    '''_engine.FadeIn(move_panel), _engine.FadeIn(move_wire),
                                                   _engine.Succession(self._to(cv["cap"], 1.0, 1.0), self._to(cv["arrow"], 1.0, 0.8), self._to(cv["glide"], 0.55, 2.4))), seconds=6.0)''')
rep('''        direct = self.gh_wire(apl["tpl"].anchor.grip_out(), self._grip(capn["ori"], "Target", "in"))''', '''        direct = self.gh_wire(apl["tpl"].anchor.grip_out(), self._grip(capn["ori"], "Target", "in"))
        goal = self._solve_place().items("rot")[0].origin
        move_vector = _gh.Vector(goal.x - self._plane_a["origin"][0], goal.y - self._plane_a["origin"][1], goal.z - self._plane_a["origin"][2])
        move_panel = self.gh_panel([self.tr("Translation [m]", "Verschiebung [m]"), self._show(move_vector)], at=(0.3, -0.35, 0), width=3.0)
        move_wire = self.gh_wire(self._grip(capn["ori"], "X", "out"), move_panel.grip_in())''')
rep('''*fout([direct, apl["pa"].view, apl["pb"].view, *apl_wires]),''', '''*fout([direct, move_panel, move_wire, apl["pa"].view, apl["pb"].view, *apl_wires]),''')
rep('''_engine.Succession(_engine.Wait(1.2), *[self._to(cv["angle"], a, 1.6) for a in (-90.0, -180.0, -270.0, -90.0)])), seconds=9.0)''',
    '''_engine.Succession(self._to(cv["glide"], 1.0, 1.2), *[self._to(cv["angle"], a, 1.8) for a in (-90.0, -180.0, -270.0)])), seconds=9.0)''')
rep("        notice = self.live(", "        wv[\"rev\"].set_value(4.0)\n        notice = self.live(")
rep('self.step("wrap", self._to(wv["wrap"], 1.0, 0.4), seconds=6.0)', 'self.step("wrap", _engine.AnimationGroup(self._to(wv["wrap"], 1.0, 0.4), _engine.Succession(_engine.Wait(0.6), self._to(wv["rev"], 8.0, 3.2))), seconds=6.0)')
rep('"Jede Slider-Domain ist eine Entwurfsgrenze; hier sind die Grenzen Lehrannahmen, zum Beispiel eine Transporthülle."', '"Jede Slider-Domain ist eine Entwurfsgrenze; hier sind die Grenzen Annahmen, zum Beispiel eine Transporthülle."')
rep('"Each slider domain is a design constraint; here the limits are teaching assumptions, for example a transport envelope."', '"Each slider domain is a design constraint; here the limits are assumptions, for example a transport envelope."')
rep('v(-1.5, -1.0, 0.0), color="#B3261E")', 'v(-2.2, -1.9, 0.0), color="#B3261E")')
open(p, "w", encoding="utf-8").write(s)
