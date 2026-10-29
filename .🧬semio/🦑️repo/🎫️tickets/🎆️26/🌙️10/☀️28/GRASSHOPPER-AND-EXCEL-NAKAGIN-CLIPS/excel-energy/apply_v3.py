import re
from pathlib import Path

target = Path("C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py")
s = target.read_text(encoding="utf-8")


def function_span(name):
    start = s.index(f"    def {name}(")
    match = re.search(r"\n    def |\nclass ", s[start + 10:])
    end = start + 10 + match.start() + 1 if match else len(s)
    return start, end


def replace_function(name, code):
    global s
    a, b = function_span(name)
    s = s[:a] + code + "\n" + s[b:]


def add_before(name, code):
    global s
    a, _ = function_span(name)
    s = s[:a] + code + "\n" + s[a:]


def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)


rep('"air": (0, 1, 0), "window"', '"air": (-0.3, 0.7, 0.6), "window"')
rep('BLUE = "#6aa8ff"\n', 'BLUE = "#6aa8ff"\nARROW_PER_UA, ARROW_WIDTH = 0.17, 1.4\n')

rep('vent=_engine.ValueTracker(0.0),\n', 'vent=_engine.ValueTracker(0.0), vol=_engine.ValueTracker(0.0),\n')
rep('round(state.vent.get_value(), 3), tuple(state.at), state.scale)', 'round(state.vent.get_value(), 3), round(state.vol.get_value(), 3), tuple(state.at), state.scale)')

rep('''        group = _engine.VGroup(behind, drawing)
        if ring is not None:
            group.add(ring)
        group.add(front)''', '''        group = _engine.VGroup(behind, drawing)
        if ring is not None:
            group.add(ring)
        if state.vol.get_value() > 0.01:
            group.add(self._volume_box(drawing, state.vol.get_value()))
        group.add(front)
        if grow > 0.6 and state.mode == "ua" and state.scale > 0.4:
            group.add(self._scale_key(at, scale))''')

replace_function("_arrow", '''    def _arrow(self, drawing, row, grow, state):
        """➡️ One heat-flow arrow out of a surface (row 99: the air): length and width are proportional to its U·A (H_V for the air); hidden faces start outside the body on a dashed lead."""
        air = row == 99
        face = "air" if air else ROWS[row]
        ua = self._num(self.T, "B12" if air else f"F{row}")
        value = ua if state.mode == "ua" else ua * (self._num(self.CL, "B2") - self._num(self.CL, "B3"))
        unit = "W/K" if state.mode == "ua" else "W"
        color = BLUE if air else SERV
        point, normal = np.array(POINT[face], dtype=float), np.array(NORMAL[face], dtype=float)
        start0 = drawing.at(*point)
        step = drawing.at(*(point + normal)) - start0
        direction = step / max(np.linalg.norm(step), 1e-6)
        start = start0 + direction * HIDDEN.get(face, 0.0)
        end = start + direction * ARROW_PER_UA * ua * grow
        group = _engine.VGroup()
        if face in HIDDEN:
            group.add(_draft.polyline([start0, start], color, _draft.W_THIN, 0.7, dash=0.06))
        group.add(_engine.Arrow(start, end, buff=0, color=color, stroke_width=ARROW_WIDTH * ua, max_tip_length_to_length_ratio=0.5, max_stroke_width_to_length_ratio=40))
        if grow > 0.6:
            group.add(self._say((self.tr("air ", "Luft ") if air else "") + self.quantity(value, unit, 2 if unit == "W/K" else 0), end + direction * 0.14, size=SMALL, color=INK, anchor=self._side(direction)))
        group.add(_engine.Dot(start0, radius=0.05, color=color))
        return group

    def _scale_key(self, at, scale):
        """🔑 The arrow scale: one arrow of 1 W/K."""
        base = np.array([at[0] + 1.5, at[1] - 2.0, 0.0])
        return _engine.VGroup(_engine.Arrow(base, base + np.array([ARROW_PER_UA, 0.0, 0.0]), buff=0, color=SERV, stroke_width=ARROW_WIDTH, max_tip_length_to_length_ratio=0.5),
                              self._say("= 1 W/K", base + np.array([ARROW_PER_UA + 0.15, 0.0, 0.0]), size=SMALL, color=MUTED, anchor="l"))

    def _volume_box(self, drawing, amount):
        """🧊 The interior air volume as a translucent box inside the capsule: right wall, front wall and ceiling."""
        t = self.cap
        x, y0, y1, z0, z1 = t.interior_width / 2, (t.length - t.interior_length) / 2, (t.length + t.interior_length) / 2, (t.height - t.interior_height) / 2, (t.height + t.interior_height) / 2
        faces = [[(x, y0, z0), (x, y1, z0), (x, y1, z1), (x, y0, z1)], [(-x, y1, z0), (x, y1, z0), (x, y1, z1), (-x, y1, z1)], [(-x, y0, z1), (x, y0, z1), (x, y1, z1), (-x, y1, z1)]]
        return _engine.VGroup(*[_draft.polygon([drawing.at(*p) for p in face], BLUE, 0.22 * amount, BLUE, _draft.W_THIN, 0.9 * amount) for face in faces])
''')

add_before("_sumproduct", '''    def _hdd_chart(self, at):
        """📈 Heating degree days as an area: twelve monthly Tokyo means against the heating limit, the shortfall hatched, summing to the engine value."""
        lang = self.language()
        months = _nakagin.climate()["months"]
        limit = self.vals_base[(self.CL, "B4")]
        w, h, ymax = 4.9, 2.0, 30.0
        x0, y0 = at[0] - w / 2, at[1] - h / 2
        px = lambda i: x0 + (i + 0.5) * w / len(months)
        py = lambda t: y0 + h * t / ymax
        bar = w / len(months) * 0.62
        group = _engine.VGroup(_draft.segments([((x0, y0, 0), (x0 + w, y0, 0))], MUTED, _draft.W_EDGE))
        for i, month in enumerate(months):
            mean = month["mean"].value if hasattr(month["mean"], "value") else month["mean"]["value"]
            name = month["name"][lang]
            group.add(_draft.polygon([(px(i) - bar / 2, y0), (px(i) + bar / 2, y0), (px(i) + bar / 2, py(mean)), (px(i) - bar / 2, py(mean))], BLUE, 0.75))
            if mean < limit:
                ring = [(px(i) - bar / 2, py(mean)), (px(i) + bar / 2, py(mean)), (px(i) + bar / 2, py(limit)), (px(i) - bar / 2, py(limit))]
                group.add(_draft.polygon(ring, SERV, 0.18), _draft.hatch([ring], SERV, 0.07, 45, _draft.W_HAIR, 0.95))
            group.add(self._say(name[0], (px(i), y0 - 0.2), size=SMALL, color=MUTED))
        group.add(_draft.polyline([(x0 - 0.1, py(limit)), (x0 + w + 0.1, py(limit))], DATA, _draft.W_EDGE, 1.0, dash=0.1),
                  self._say(self.quantity(limit, "°C", 0), (x0 - 0.15, py(limit)), size=SMALL, color=DATA, anchor="r"),
                  self._readout(lambda: "Σ = " + self._show(self.CL, "B5"), (at[0], y0 + h + 0.4), color=SERV),
                  self._say(self.tr("monthly mean, Tokyo", "Monatsmittel, Tokio"), (at[0], y0 - 0.6), size=SMALL, color=MUTED))
        return group
''')

add_before("_sumproduct", '''    def _annual_tiles(self, at):
        """🧱 Annual demand as a product of tiles: loss per kelvin times degree days times 24 h per day over 1000, then per floor area."""
        T, CL = self.T, self.CL

        def tile(x, y, label, fn, color, width=1.15):
            box = _engine.RoundedRectangle(corner_radius=0.08, width=width, height=0.78, stroke_color=color, stroke_width=_draft.W_THIN, fill_color=color, fill_opacity=0.14).move_to((x, y, 0))
            return _engine.VGroup(box, self._say(label, (x, y + 0.2), size=SMALL, color=MUTED, width=width - 0.1), self._readout(fn, (x, y - 0.13), size=SMALL, color=INK))

        group = _engine.VGroup()
        xs = [at[0] - 2.05, at[0] - 0.65, at[0] + 0.75, at[0] + 2.15]
        y = at[1]
        loss = lambda: self.xl_display(self._num(T, "B11") + self._num(T, "B12"), '0.0 "W/K"')
        group.add(tile(xs[0], y, "H", loss, SERV), tile(xs[1], y, self.tr("degree days", "Gradtage"), lambda: self._show(CL, "B5"), BLUE),
                  tile(xs[2], y, self.tr("hours per day", "Stunden je Tag"), lambda: "24 h/d ÷ 1000", MUTED), tile(xs[3], y, "Q", lambda: self._show(T, "B14"), DATA))
        for x, sign in ((at[0] - 1.35, "×"), (at[0] + 0.05, "×"), (at[0] + 1.45, "=")):
            group.add(self._say(sign, (x, y), size=LABEL, color=INK))
        group.add(tile(xs[0], y - 1.1, self.tr("floor area", "Grundfläche"), lambda: self._show(T, "B9"), GOOD), tile(xs[3], y - 1.1, "Q/A", lambda: self._show(T, "B15"), DATA),
                  self._say("Q ÷", (xs[1] - 0.2, y - 1.1), size=LABEL, color=INK), self._say("=", (at[0] + 1.45, y - 1.1), size=LABEL, color=INK))
        return group
''')

add_before("_sumproduct", '''    def _types_row(self, at):
        """🧩 The four capsule types side by side: A, B and D share one shell and window, C has the larger window."""
        group = _engine.VGroup()
        for i, t in enumerate(_nakagin.capsule_types()):
            x = at[0] - 1.35 + i * 0.9
            group.add(self.capsule_axon(t, at=(x, at[1], 0.0), scale=0.14, window=True),
                      self.badge(t.key, (x - 0.22, at[1] - 0.52), color=_drawing.type_color(t.key), size=SMALL),
                      self._say(self.quantity(t.window_diameter, "m", 1), (x + 0.2, at[1] - 0.52), size=SMALL, color=WARN if t.window_diameter > 1.4 else INK, width=0.7))
        group.add(self._say(self.tr("same shell, same demand · only C has the larger window", "gleiche Hülle, gleicher Bedarf · nur C hat das größere Fenster"), (at[0], at[1] - 0.95), size=SMALL, color=MUTED, width=3.7))
        return group
''')

add_before("_sumproduct", '''    def _tower_counter(self, at, kinds):
        """🏙️ The building as a small tinted tower with the live building total and its change against the start."""
        EN = self.EN
        base = self.vals_base[(EN, "D8")]
        group = _engine.VGroup(self.tower_axon(at=(at[0] - 1.1, at[1], 0.0), scale=0.045, tint=lambda key: _drawing.type_color(kinds[key]), lit=None))
        group.add(self._readout(lambda: self._show(EN, "D8"), (at[0] + 0.9, at[1] + 0.3), color=DATA),
                  self._readout(lambda: self.xl_display(self._num(EN, "D8") / base - 1.0, '+0.0%;-0.0%;0.0%'), (at[0] + 0.9, at[1] - 0.1), size=SMALL, color=GOOD))
        return group
''')

# hook
rep('''        mark = self.badge("?", (1.7, 1.4), color=DATA, size=26)
        note = self._note()
        self.step("hook", _engine.FadeIn(tower), _engine.FadeIn(mark), _engine.FadeIn(note), seconds=2.5)''', '''        mark = self.badge("?", (1.7, 1.4), color=DATA, size=26)
        outside = _engine.VGroup(self._say("θe = " + self.xl_display(self.vals_base[(CL, "B3")], '0 "°C"'), (3.4, 0.6), size=LABEL, color=BLUE),
                                 self._say("θi = " + self.xl_display(self.vals_base[(CL, "B2")], '0 "°C"'), (3.4, 0.15), size=LABEL, color=SERV))
        note = self._note()
        self.step("hook", _engine.FadeIn(tower), _engine.FadeIn(mark), _engine.FadeIn(outside), _engine.FadeIn(note), seconds=2.5)''')
rep('self.step("sheet", _engine.FadeOut(tower), _engine.FadeOut(mark), _engine.FadeIn(self.win)', 'self.step("sheet", _engine.FadeOut(tower), _engine.FadeOut(mark), _engine.FadeOut(outside), _engine.FadeIn(self.win)')

# climate + degreedays
rep('''        self.step("climate", *self._swap(new), _engine.FadeOut(card), _engine.FadeOut(capsule), _engine.FadeIn(names), seconds=3.0)
''', '''        self.step("climate", *self._swap(new), _engine.FadeOut(card), _engine.FadeOut(capsule), _engine.FadeIn(names), seconds=3.0)

        hdd = self._hdd_chart((self.dx, 0.2))
        self.step("degreedays", self.win.select("B5"), _engine.FadeOut(names), _engine.FadeIn(hdd), seconds=3.0)
''')

# mistake with formula
rep('''        moving = self._begin(self.books["back_wall_exposed"][1])
        self.step("mistake", moving, self.win.select("B6"), *self._recalc(*self.books["back_wall_exposed"]), _engine.FadeIn(self._frame("B6", SERV)), _engine.FadeOut(names), _engine.FadeIn(capsule), _engine.FadeIn(delta), seconds=3.0)''', '''        formula_card = _engine.VGroup(self._say(self.tr("Type A · F9", "Typ A · F9"), (self.dx - 2.3, 3.0), size=SMALL, color=MUTED, anchor="l"),
                                      self.xl_formula_text("=D9*E9*attached_factor", at=(self.dx - 1.2, 3.0, 0), size=SMALL, ink=INK),
                                      self._readout(lambda: "= " + self._show(T, "F9"), (self.dx + 1.6, 3.0), size=SMALL, color=SERV, anchor="l"))
        moving = self._begin(self.books["back_wall_exposed"][1])
        self.step("mistake", moving, self.win.select("B6"), *self._recalc(*self.books["back_wall_exposed"]), _engine.FadeIn(self._frame("B6", SERV)), _engine.FadeOut(hdd), _engine.FadeIn(capsule), _engine.FadeIn(delta),
                  _engine.FadeIn(formula_card), seconds=3.0)''')
rep('''        self.step("fix", moving, self.win.select("B6"), *self._recalc(self.book, self.vals_base), seconds=3.0)''', '''        self.step("fix", moving, self.win.select("B6"), *self._recalc(self.book, self.vals_base), _engine.FadeOut(formula_card), seconds=3.0)''')

# volume
rep('''        self.step("volume", *self._swap(new), _engine.FadeOut(delta), ramp(cs.vent, 0, 1), seconds=3.0)''', '''        vol_label = self._readout(lambda: "V = " + self._show(T, "B10") + " · n = " + self._show(CL, "B7"), (self.dx, -2.05), size=LABEL, color=BLUE)
        self.step("volume", *self._swap(new), _engine.FadeOut(delta), ramp(cs.vent, 0, 1), ramp(cs.vol, 0, 1), _engine.FadeIn(vol_label), seconds=3.0)''')
rep('''self.step("total", self.win.select("B11:B12"), _engine.FadeIn(self._frame("B11", SERV)), _engine.FadeIn(self._frame("B12", BLUE)), _engine.FadeIn(bar), ramp(cs.grow, 1, 0, 1.5),
                  ramp(cs.vent, 1, 0, 1.5), seconds=2.5)''', '''self.step("total", self.win.select("B11:B12"), _engine.FadeIn(self._frame("B11", SERV)), _engine.FadeIn(self._frame("B12", BLUE)), _engine.FadeIn(bar), ramp(cs.grow, 1, 0, 1.5),
                  ramp(cs.vent, 1, 0, 1.5), ramp(cs.vol, 1, 0, 1.5), _engine.FadeOut(vol_label), seconds=2.5)''')

# annual
rep('''        units = self._say(self.tr("W/K · K·d · 24 h/d ÷ 1000 = kWh/a", "W/K · K·d · 24 h/d ÷ 1000 = kWh/a"), (self.dx, -2.1), size=LABEL, color=DATA)
        plan = self._plan((self.dx + 2.2, 1.7))
        self.step("annual", *self._swap(new), ramp(cs.grow, 1, 0, 2.0), ramp(cs.vent, 1, 0, 2.0), _engine.FadeOut(temps), _engine.FadeIn(units), _engine.FadeIn(plan), seconds=3.0)''', '''        units = self._annual_tiles((self.dx, 0.5))
        plan = self._plan((self.dx + 2.2, 2.0))
        self.step("annual", *self._swap(new), ramp(cs.grow, 1, 0, 2.0), ramp(cs.vent, 1, 0, 2.0), _engine.FadeOut(temps), _engine.FadeOut(capsule), _engine.FadeIn(units), _engine.FadeIn(plan), seconds=3.0)''')
rep('_engine.FadeOut(capsule), _engine.FadeOut(units), _engine.FadeOut(plan), _engine.FadeIn(tower), _engine.FadeIn(counts), seconds=3.0)', '_engine.FadeOut(units), _engine.FadeOut(plan), _engine.FadeIn(tower), _engine.FadeIn(counts), seconds=3.0)')

# compare
rep('''chart = self.xl_bar_chart(rows, at=(self.dx, -0.1, 0.0), width=3.7, height=3.6,''', '''types_row = self._types_row((self.dx, 2.1))
        chart = self.xl_bar_chart(rows, at=(self.dx, -0.6, 0.0), width=3.7, height=2.9,''')
rep('_engine.FadeIn(badges), _engine.FadeOut(tower), _engine.FadeOut(counts), _engine.FadeIn(chart), seconds=3.0)', '_engine.FadeIn(badges), _engine.FadeOut(tower), _engine.FadeOut(counts), _engine.FadeIn(chart), _engine.FadeIn(types_row), seconds=3.0)')
rep('self.step("shape", _engine.FadeOut(chart), _engine.FadeIn(shape)', 'self.step("shape", _engine.FadeOut(chart), _engine.FadeOut(types_row), _engine.FadeIn(shape)')

# trace / whatif tower
rep('''        small_state = self._cap_state(rows=(8,), grow=_engine.ValueTracker(1.0))
        new = self._window(EN, (3, 14), "B14", "A:D")
        small = self._capsule((self.dx - 0.6, 1.6, 0.0), 0.34, small_state)
        chain = self._chain((self.dx, -0.95), [(EN, "B14", self.tr("Window U-value", "Fenster U-Wert")), (T, "F8", self.tr("Type A · U·A window", "Typ A · U·A Fenster")),
                                               (T, "B11", self.tr("Type A · H_T", "Typ A · H_T")), (T, "B14", self.tr("Type A · demand", "Typ A · Bedarf")),
                                               (EN, "D8", self.tr("Building total", "Gebäudesumme"))])
        count = len(self._closure((EN, "B14")))
        counted = self._say(self.tr(f"{count} dependent cells", f"{count} abhängige Zellen"), (self.dx, 2.5), size=LABEL, color=DATA)''', '''        new = self._window(EN, (3, 14), "B14", "A:D")
        small = self._tower_counter((self.dx, 1.55), kinds)
        chain = self._chain((self.dx, -1.0), [(EN, "B14", self.tr("Window U-value", "Fenster U-Wert")), (T, "B11", self.tr("Type A · H_T", "Typ A · H_T")),
                                              (T, "B14", self.tr("Type A · demand", "Typ A · Bedarf")), (EN, "D8", self.tr("Building total", "Gebäudesumme"))])
        count = len(self._closure((EN, "B14")))
        counted = self._say(self.tr(f"{count} dependent cells", f"{count} abhängige Zellen"), (self.dx, 0.35), size=SMALL, color=MUTED, width=3.4)''')
rep('(self.dx, 0.85), size=SMALL, color=SERV)', '(self.dx, 0.7), size=SMALL, color=SERV)')

# recap
rep('''        legend.add(self.tower_axon(at=(4.9, -0.1, 0.0), scale=0.085, tint=lambda key: _drawing.type_color(kinds[key]), lit=None))''', '''        legend.add(self.tower_axon(at=(4.9, -0.45, 0.0), scale=0.085, tint=lambda key: _drawing.type_color(kinds[key]), lit=None))
        EN, T = self.EN, self.T
        a, b = self.vals_base, self.vals_better
        lines = [(self.tr("Type A: ", "Typ A: ") + self.xl_display(a[(T, "B14")], '#,##0') + " → " + self.xl_display(b[(T, "B14")], '#,##0 "kWh/a"')),
                 (self.xl_display(a[(T, "B15")], '0') + " → " + self.xl_display(b[(T, "B15")], '0 "kWh/(m²·a)"')),
                 (self.tr("Building: ", "Gebäude: ") + self.xl_display(a[(EN, "D8")], '#,##0') + " → " + self.xl_display(b[(EN, "D8")], '#,##0 "kWh/a"'))]
        for i, line in enumerate(lines):
            legend.add(self._say(line, (4.9, 2.75 - i * 0.36), size=SMALL, color=DATA, width=4.4))''')
rep('"Recap: U·A per surface, H_T and H_V, load from the temperature difference, demand from degree days, times the capsule count."', '"Recap: U·A per surface, H_T and H_V, load from the temperature difference, demand from degree days, times the capsule count."')

# narration
rep('''        ["mistake",''', '''        ["degreedays", "Heating degree days are an area: each month's shortfall below the 18 degree line, times its days, adds up over the year.",
         "Heizgradtage sind eine Fläche: Der Fehlbetrag jedes Monats unter der 18-Grad-Linie mal seine Tage, summiert über das Jahr."],
        ["mistake",''')
rep('"Per square metre the four types are close. Only type C, with its larger window, needs a little more."', '"A, B and D share one shell, and fittings do not change the heat loss. Only C, with its larger window, needs more."')
rep('"Je Quadratmeter liegen die vier Typen dicht beieinander. Nur Typ C mit seinem größeren Fenster braucht etwas mehr."', '"A, B und D teilen eine Hülle, Einbauten ändern den Verlust nicht. Nur C mit dem größeren Fenster braucht mehr."')
rep('"Why so much for so little floor? Five exposed faces wrap a small room. Heated neighbours above and below would cut the demand."', '"Why so much for so little floor? Five faces and a window wrap a small room. Heated neighbours above and below would cut the demand."')
target.write_text(s, encoding="utf-8")
print("patched v3")
