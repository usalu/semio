import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)


rep("Center Box takes width, length and height and builds a box; the viewport previews it.", "Center Box, nickname Box, takes width, length and height and builds a box; the viewport shows it.")
rep("Center Box nimmt Breite, Länge und Höhe und baut eine Box; der Viewport zeigt sie.", "Center Box, Kurzname Box, nimmt Breite, Länge und Höhe und baut eine Box; der Viewport zeigt sie.")
rep("Each slider is one dimension of the capsule: wire and dimension line share its colour. Length and height move too.", "Each slider is one dimension: wire and dimension line share its colour. Length 5.0 and height 3.0 metres make the box grow.")
rep("Jeder Slider ist ein Maß der Kapsel: Verbindung und Maßlinie teilen seine Farbe. Auch Länge und Höhe bewegen sich.", "Jeder Slider ist ein Maß: Verbindung und Maßlinie teilen seine Farbe. Länge 5,0 und Höhe 3,0 Meter lassen die Box wachsen.")
rep("Components show states. Grey is fine. Give Z a number and a text: it turns orange, a warning. One box is built, the second slot stays null.", "Grey is fine. A number and a text in Z turn it orange, a warning: one box, one null. Replace the text by 2.5 and it is grey again.")
rep("Grau ist in Ordnung. Bekommt Z eine Zahl und einen Text, wird sie orange, eine Warnung: Eine Box entsteht, der zweite Platz bleibt null.", "Grau ist in Ordnung. Zahl und Text in Z machen sie orange, eine Warnung: eine Box, ein null. Mit 2,5 statt Text ist sie wieder grau.")
rep("With nothing usable it turns red, an error: a wire cannot make the text tall a height. Fix the first problem upstream.", "With nothing usable it turns red, an error: a wire cannot make the text tall a height. Give Z the number 2.5 and the box returns.")
rep("Ohne Brauchbares wird sie rot, ein Fehler: Aus dem Text tall macht keine Verbindung eine Höhe. Das erste Problem zuerst beheben.", "Ohne Brauchbares wird sie rot, ein Fehler: Aus dem Text tall macht keine Verbindung eine Höhe. Mit der Zahl 2,5 in Z kehrt die Box zurück.")
rep("A plugin, a .gha file, adds a tab with new components to the ribbon. Ladybug Tools can study the sun on the capsule window.", "A plugin, a .gha file, adds a tab to the ribbon. Ladybug Tools studies the sun on the capsule window: more sun, warmer tint.")
rep("Ein Plugin, eine .gha-Datei, fügt dem Ribbon einen Tab hinzu. Ladybug Tools kann die Sonne am Kapselfenster untersuchen.", "Ein Plugin, eine .gha-Datei, fügt dem Ribbon einen Tab hinzu. Ladybug Tools untersucht die Sonne am Fenster: mehr Sonne, wärmere Tönung.")
rep("Now the data. One value is an item: a single height, 0 metres above the first capsule storey, one capsule on Tower A.", "Now the data. One value is an item: the height 0 m of the first storey, one capsule on Tower A.")
rep("Nun die Daten. Ein Wert ist ein Element: eine einzelne Höhe, 0 Meter über dem ersten Kapselgeschoss, eine Kapsel an Turm A.", "Nun die Daten. Ein Wert ist ein Element: die Höhe 0 m des ersten Geschosses, eine Kapsel an Turm A.")
rep("Ändert man den Slider Anzahl von 9 auf 11", "Ändert man den Schieberegler Anzahl von 9 auf 11")
rep("Only path plus index names exactly one capsule: path {0;0;3;1}, index 1.", "Only path plus index names exactly one capsule: path {0;0;3;1}, index 1. The Panel shows both.")
rep("Erst Pfad plus Index nennt genau eine Kapsel: Pfad {0;0;3;1}, Index 1.", "Erst Pfad plus Index nennt genau eine Kapsel: Pfad {0;0;3;1}, Index 1. Das Panel zeigt beides.")
rep("The canvas terms in English and German: script, Skript; component, Komponente; input, Eingang; output, Ausgang; slider, Schieberegler.", "The canvas terms, row by row in English and German: script, Skript; component, Komponente; and six more.")

rep('''        state = self.tower_state
        return self.live(lambda: self.tower_axon(at=VIEW + np.array([0.0, -0.1, 0.0]), scale=state["scale"], lit=state["lit"](), segments=1, ground=False, types=False,
                                                 towers=state["towers"], podium=True), lambda: (state["key"](), state["towers"]))''', '''        state = self.tower_state
        slots = {(c.building, c.tower, c.storey, c.facade, c.item): c for c in _nk.slots()}

        def build():
            drawing = self.tower_axon(at=VIEW + np.array([0.0, state["dy"], 0.0]), scale=state["scale"], lit=state["lit"](), segments=1, ground=False, types=False,
                                      towers=state["towers"], podium=state["podium"])
            for text, key, dx, dy in state["marks"]():
                spot = np.array(drawing.at(*slots[key].plane.origin), dtype=float)
                spot[2] = 0.0
                label = self._note(text, spot + np.array([dx, dy, 0.0]), anchor="l" if dx > 0 else "r")
                drawing.add(self.leader(spot + np.array([dx - 0.05 * np.sign(dx), dy, 0.0]), spot, None, color=DARK, dot=True), label)
            return drawing

        return self.live(build, lambda: (state["key"](), state["towers"], tuple(m[0] for m in state["marks"]())))''')
rep('self.tower_state = {"lit": lambda: None, "key": lambda: None, "towers": (0,), "scale": 0.072}', 'self.tower_state = {"lit": lambda: None, "key": lambda: None, "towers": (0,), "scale": 0.092, "dy": -0.15, "podium": False, "marks": lambda: []}')

rep('x, y, z = self._dims_of(_gh.evaluate(state["definition"], {k: v for k, v in over.items() if k in state["definition"].objects}), state["box"])',
    'x, y, z = self._dims_of(_gh.evaluate(state["definition"], {**{k: v for k, v in over.items() if k in state["definition"].objects}, **state["overrides"]}), state["box"])')
rep('"definition": self.three, "box": "cb"}', '"definition": self.three, "box": "cb", "overrides": {}}')
rep('state["scale"], state["box"],\n                       round(', 'state["scale"], state["box"], tuple(sorted(state["overrides"])),\n                       round(')
rep('fill_color=_art.P_YELLOW, fill_opacity=0.55)', 'fill_color="#F08A00", fill_opacity=0.62)')
rep('    def construct(self):', '    def snapshot(self, tag):\n        """📸 Keyframe hook for beats that continue after their clause; the capture script saves a frame here."""\n        return None\n\n    def construct(self):')

rep('    def _states(self, kind, sol):', '    def _states(self, kind, sol, lines=None):')
rep('''        text = self.gh_panel(None, rows=[("line", "", line) for line in _gh.canvas("anatomy").objects[src].settings["lines"]], at=_at(SL_X, -0.85), max_lines=4)''',
    '''        lines = lines or _gh.canvas("anatomy").objects[src].settings["lines"]
        text = self.gh_panel(None, rows=[("line", "", line) for line in lines], at=_at(SL_X, -0.85), max_lines=4)''')
rep('''        message = sol.msgs(key)[-1] if kind == "error" else sol.msgs(key)[0]
        bubble = self.gh_bubble(message["en"], level="warning" if kind == "warning" else "error", target=box, width=3.6)''',
    '''        notes = sol.msgs(key)
        bubble = self.gh_bubble((notes[-1] if kind == "error" else notes[0])["en"], level="warning" if kind == "warning" else "error", target=box, width=3.6) if notes else _engine.VGroup()
        many = "list" if len(lines) > 1 else "item"''')
rep('self.gh_wire(text.grip_out(), box.grip_in("Z"), kind="list" if kind == "warning" else "item"), self.gh_wire(box.grip_out(), panel.grip_in(), kind="list" if kind == "warning" else "item")]',
    'self.gh_wire(text.grip_out(), box.grip_in("Z"), kind=many), self.gh_wire(box.grip_out(), panel.grip_in(), kind=many)]')

rep('''        self.step("hook", _engine.FadeIn(sheet), _engine.FadeIn(ribbon), _engine.FadeIn(viewport), _engine.FadeIn(preview), _engine.FadeIn(model_note), seconds=1.4)''',
    '''        slots_n = len(_nk.slots())
        whole = self.tower_axon(at=VIEW + np.array([0.0, -0.05, 0.0]), scale=0.05, lit=lambda k: k == (0, 0, 3, 1, 1), segments=1, ground=False, types=False, towers=(0, 1), podium=True)
        whole_note = self._note(self.tr(f"1 of {slots_n} capsules", f"1 von {slots_n} Kapseln"), VIEW + np.array([VIEW_W / 2 - 0.12, VIEW_H / 2 - 0.24, 0.0]), anchor="r")
        self.step("hook", _engine.FadeIn(sheet), _engine.FadeIn(ribbon), _engine.FadeIn(viewport), _engine.FadeIn(whole), _engine.FadeIn(whole_note), _engine.FadeIn(model_note), seconds=1.4)''')
rep('''        self.step("script", _engine.FadeIn(chip), _engine.Create(frame), _engine.FadeIn(canvas_tag), _engine.FadeOut(model_note), seconds=1.2)''',
    '''        kinds = _nk.capsule_types()
        strip = _engine.VGroup(*[self.capsule_axon(t, at=_at(-3.0 + 2.0 * i, 0.2), scale=0.2, base=_canvas.PREVIEW_FILL, window=True) for i, t in enumerate(kinds)],
                               *[self._note(t.key, _at(-3.0 + 2.0 * i, -0.45)) for i, t in enumerate(kinds)],
                               self._note(self.tr(f"one definition: {len(kinds)} types, {slots_n} placements", f"eine Definition: {len(kinds)} Typen, {slots_n} Plätze"), _at(0.0, -1.0), color=SOFT))
        self.step("script", _engine.FadeOut(whole), _engine.FadeOut(whole_note), _engine.FadeIn(preview), _engine.FadeIn(chip), _engine.Create(frame), _engine.FadeIn(canvas_tag), _engine.FadeIn(strip), _engine.FadeOut(model_note), seconds=1.2)''')
rep('self.step("slider", _engine.FadeOut(chip), _engine.FadeOut(canvas_tag),', 'self.step("slider", _engine.FadeOut(chip), _engine.FadeOut(canvas_tag), _engine.FadeOut(strip),')

rep('''        pulses = [self._pulse(''', '''        heads = _engine.VGroup()
        for slider, port, color in ((w_slider, "X", cw), (l_slider, "Y", cl), (h_slider, "Z", ch)):
            curve = _canvas.bezier(np.array(slider.grip_out().get_center()), np.array(cb.grip_in(port).get_center()))
            mid, ahead = len(curve) // 2, curve[len(curve) // 2 + 2] - curve[len(curve) // 2 - 2]
            heads.add(_engine.Triangle(fill_color=color, fill_opacity=1, stroke_width=0).scale(0.1).rotate(float(np.arctan2(ahead[1], ahead[0])) - np.pi / 2).move_to(curve[mid]).set_z_index(4))
        pulses = [self._pulse(''')
rep('self.step("flow", _engine.FadeOut(call), _engine.Succession', 'self.step("flow", _engine.FadeOut(call), _engine.FadeIn(heads), _engine.Succession')
rep('''        self.step("sliders", _engine.AnimationGroup(_engine.Succession(self._ramp(self.L, l0, l0 + 1.0, 1.0), self._ramp(self.L, l0 + 1.0, l0, 1.0)),
                                                   _engine.Succession(self._ramp(self.H, h0, h0 + 0.5, 1.0), self._ramp(self.H, h0 + 0.5, h0, 1.0))), seconds=2.0)''',
    '''        self.step("sliders", self._ramp(self.L, l0, l0 + 1.0, 2.0), self._ramp(self.H, h0, h0 + 0.5, 2.0), seconds=2.0)
        self.play(self._ramp(self.L, l0 + 1.0, l0, 1.0), self._ramp(self.H, h0 + 0.5, h0, 1.0), run_time=1.0)''')
rep('''        self.step("live", _engine.Succession(self._ramp(self.W, w0, w1, 1.6), self._ramp(self.W, w1, w0, 1.6)), seconds=3.2)''',
    '''        readout = self.live_label(lambda: f"X = {self.number(self.W.get_value(), 1)} m", _at(0.5, 1.5), size=SMALL, color=cw)
        self.step("live", _engine.FadeIn(readout), self._ramp(self.W, w0, w1, 2.0), seconds=2.0)
        self.play(self._ramp(self.W, w1, w0, 1.2), run_time=1.2)''')
rep('self.play(_engine.FadeIn(vol), _engine.FadeIn(vp),', 'self.play(_engine.FadeOut(readout), _engine.FadeIn(vol), _engine.FadeIn(vp),')
rep('main = _engine.VGroup(w_slider, l_slider, h_slider, cb, *wires)', 'main = _engine.VGroup(w_slider, l_slider, h_slider, cb, *wires, heads)')

rep('''        self.step("warning", _engine.FadeOut(main), _engine.FadeOut(extra), _engine.FadeIn(warning), seconds=1.4)
''', '''        self.step("warning", _engine.FadeOut(main), _engine.FadeOut(extra), _engine.FadeIn(warning), seconds=1.4)
        fixed_w = self._states("warning", _gh.evaluate(defn, {"wn": ["2.5"]}), lines=["2.5"])
        self.play(_engine.FadeOut(warning), _engine.FadeIn(fixed_w), run_time=1.0)
        self.snapshot("warning_fixed")
''')
rep('self.step("error", _engine.FadeOut(warning), _engine.FadeIn(error), _engine.FadeIn(first), seconds=1.4)\n        self.view_state.update(visible=True, definition=self.three, box="cb")',
    '''self.step("error", _engine.FadeOut(fixed_w), _engine.FadeIn(error), _engine.FadeIn(first), seconds=1.4)
        fixed_e = self._states("error", _gh.evaluate(defn, {"er": ["2.5"]}), lines=["2.5"])
        self.view_state.update(visible=True, box="cbe", overrides={"er": ["2.5"]})
        self.play(_engine.FadeOut(error), _engine.FadeOut(first), _engine.FadeIn(fixed_e), run_time=1.0)
        self.snapshot("error_fixed")
        self.view_state.update(definition=self.three, box="cb", overrides={})''')
rep('self.step("panel", _engine.FadeOut(error), _engine.FadeOut(first),', 'self.step("panel", _engine.FadeOut(fixed_e),')

rep('''steps = _engine.VGroup(self._tag(self.tr("Package Manager", "Paketverwaltung"), _at(-2.5, 0.2)), self._tag("Food4Rhino", _at(0.0, 0.2)), self._tag("Ladybug.gha", _at(2.4, 0.2)))
        arrows = _engine.VGroup(*[_engine.Arrow(_at(a, 0.2), _at(b, 0.2), buff=0.0, color=DARK, stroke_width=3, max_tip_length_to_length_ratio=0.3) for a, b in ((-1.4, -0.95), (0.95, 1.55))])
        fine = _engine.VGroup(self._note(self.tr("install", "installieren"), _at(-2.5, -0.15)), self._note(self.tr("download", "herunterladen"), _at(0.0, -0.15)),
                              self._note(self.tr("plugin file", "Plugin-Datei"), _at(2.4, -0.15)), self._note(self.tr("then the new tab appears in the ribbon", "dann erscheint der neue Tab im Ribbon"), _at(0.0, -0.8)))''',
    '''steps = _engine.VGroup(self._tag(self.tr("Package Manager", "Paketverwaltung"), _at(-2.5, 1.2)), self._tag("Food4Rhino", _at(0.0, 1.2)), self._tag("Ladybug.gha", _at(2.4, 1.2)))
        arrows = _engine.VGroup(*[_engine.Arrow(_at(a, 1.2), _at(b, 1.2), buff=0.0, color=DARK, stroke_width=3, max_tip_length_to_length_ratio=0.3) for a, b in ((-1.4, -0.95), (0.95, 1.55))])
        sun_spec = {"name": "LB Direct Sun Hours", "nickname": "Sun", "inputs": [{"name": "Geometry", "nickname": "G", "type": "Geometry"}, {"name": "Sun vectors", "nickname": "V", "type": "Vector"}],
                    "outputs": [{"name": "Hours", "nickname": "H", "type": "Number"}]}
        sun_param = self.gh_param("Brep", at=_at(-3.4, -0.45), label=self.tr("Window", "Fenster"))
        sun_comp = self.gh_component(sun_spec, at=_at(-1.2, -0.45))
        bar = _engine.Rectangle(width=2.0, height=0.3, stroke_color=DARK, stroke_width=2).set_fill(color=["#FFF2B0", "#F08A00"], opacity=1).set_sheen_direction(_engine.RIGHT).move_to(_at(2.5, -0.45))
        fine = _engine.VGroup(self._note(self.tr("install", "installieren"), _at(-2.5, 0.85)), self._note(self.tr("download", "herunterladen"), _at(0.0, 0.85)), self._note(self.tr("plugin file", "Plugin-Datei"), _at(2.4, 0.85)),
                              sun_param, sun_comp, bar, self.gh_wire(sun_param.grip_out(), sun_comp.grip_in("G")), self.gh_wire(sun_comp.grip_out(), _at(1.45, -0.45)),
                              self._note(self.tr("less sun", "weniger Sonne"), _at(1.7, -0.9)), self._note(self.tr("more sun", "mehr Sonne"), _at(3.3, -0.9)),
                              self._note(self.tr("qualitative, no sun-hour values", "qualitativ, keine Sonnenstunden-Werte"), _at(0.5, -1.4), color=SOFT))''')

rep('''        wires2 = [self.gh_wire(st_slider.grip_out(), series.grip_in("N")), self.gh_wire(cnt_slider.grip_out(), series.grip_in("C")),
                  self.gh_wire(series.grip_out(), _at(2.95, 0.0), kind="list")]
        self.tower_state.update(lit=''', '''        wires2 = [self.gh_wire(st_slider.grip_out(), series.grip_in("N")), self.gh_wire(cnt_slider.grip_out(), series.grip_in("C")),
                  self.gh_wire(series.grip_out(), _at(2.95, 0.0), kind="list"),
                  self._note(self.tr("S start 0 · N step · C count", "S Start 0 · N Schritt · C Anzahl"), _at(0.7, -0.5))]

        def series_marks(tower_index):
            n = count()
            values = _gh.evaluate(series_defn, {"cnt": n}).items("se", "Series")
            return [(f"{i} → {self.number(values[i], 2)} m", (0, tower_index, i, 1, 0), 0.7, 0.0) for i in sorted({0, n - 1})]

        self.tower_state.update(marks=lambda: series_marks(0), lit=''')
rep('self.tower_state.update(towers=(1,), scale=0.062, lit=lambda: (lambda k: k[0] == 0 and k[1] == 1 and k[3] == 1 and k[4] == 0 and k[2] < count()))',
    'self.tower_state.update(towers=(1,), scale=0.075, marks=lambda: series_marks(1), lit=lambda: (lambda k: k[0] == 0 and k[1] == 1 and k[3] == 1 and k[4] == 0 and k[2] < count()))')
rep('self.tower_state.update(towers=(0,), scale=0.072, lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch")',
    '''path_marks = lambda: [(self.tr("storey 3", "Geschoss 3"), (0, 0, 3, 1, 0), 0.75, 0.22), (self.tr("facade 1 · east", "Fassade 1 · Ost"), (0, 0, 3, 1, 0), 0.75, -0.22)]
        self.tower_state.update(towers=(0,), scale=0.092, marks=path_marks, lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch")''')
rep('self.tower_state.update(lit=lambda: (lambda k: k[0] == 0 and k[1] == 0 and k[4] == 1), key=lambda: "all")', 'self.tower_state.update(marks=lambda: [], lit=lambda: (lambda k: k[0] == 0 and k[1] == 0 and k[4] == 1), key=lambda: "all")')
rep('self.tower_state.update(lit=lambda: (lambda k: k == (0, 0, 3, 1, 1)), key=lambda: "one")', '''self.tower_state.update(marks=lambda: [(self.tr("path {0;0;3;1} · index 1", "Pfad {0;0;3;1} · Index 1"), (0, 0, 3, 1, 1), 0.75, 0.0)], lit=lambda: (lambda k: k == (0, 0, 3, 1, 1)), key=lambda: "one")''')
rep('''        self.step("mistake", _engine.FadeOut(band), _engine.FadeOut(explain), _engine.FadeIn(every), _engine.FadeIn(bad), _engine.FadeIn(shown), seconds=1.4)''',
    '''        expected = self._tag(self.tr("expected: 1 capsule", "erwartet: 1 Kapsel"), _at(2.3, -0.4), color=SOFT)
        self.step("mistake", _engine.FadeOut(band), _engine.FadeOut(explain), _engine.FadeIn(every), _engine.FadeIn(bad), _engine.FadeIn(expected), _engine.FadeIn(shown), seconds=1.4)''')
rep('''"{0;0;3;1}, index 1: one capsule", "{0;0;3;1}, Index 1: eine Kapsel"), _at(2.3, -0.9), color=GREEN)''', '''"{0;0;3;1}, index 1: one capsule", "{0;0;3;1}, Index 1: eine Kapsel"), _at(2.3, -0.4), color=GREEN)
        strike = _engine.Line(bad.get_left(), bad.get_right(), color=RED, stroke_width=4)''')
rep('self.step("address", _engine.FadeOut(every), _engine.FadeOut(bad), _engine.FadeOut(shown), _engine.FadeIn(one), _engine.FadeIn(good), seconds=1.4)',
    'self.step("address", _engine.FadeOut(every), _engine.FadeOut(expected), bad.animate.set_opacity(0.45), _engine.Create(strike), _engine.FadeOut(shown), _engine.FadeIn(one), _engine.FadeIn(good), seconds=1.4)')
rep('for m in (t_param, tree_panel, t_wire, one, good, ribbon)]', 'for m in (t_param, tree_panel, t_wire, one, good, bad, strike, ribbon)]')
rep('self.tower_state.update(lit=lambda: None, key=lambda: "none")', 'self.tower_state.update(marks=lambda: [], lit=lambda: None, key=lambda: "none")')
rep('self.tower_state.update(lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch2")', 'self.tower_state.update(marks=lambda: [], lit=lambda: (lambda k: k[:4] == (0, 0, 3, 1)), key=lambda: "branch2")')
open(p, "w", encoding="utf-8").write(s)
