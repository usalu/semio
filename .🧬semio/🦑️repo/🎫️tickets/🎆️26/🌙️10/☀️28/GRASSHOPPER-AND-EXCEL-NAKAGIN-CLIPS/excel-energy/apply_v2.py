import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
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


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


a, b = function_span("present_energy")
c, d = function_span("_sumproduct")
s = s[:a] + (HERE / "present_v2.txt").read_text(encoding="utf-8") + s[d:]

rep('COLS = {"A:B": 6.2, "C:F": 7.6, "A:D": 10.0, "A:B+": 6.6, "A:F": 9.0}', 'COLS = {"A:B": 6.2, "C:F": 7.6, "A:D": 10.0, "A:B+": 8.0, "D:F": 6.4}')
rep('"front": (0.9, 4.0, 0.3),', '"front": (0.9, 4.0, 0.3), "air": (0.75, 4.0, 2.1),')
rep('"front": (0, 1, 0), "window": (0, 1, 0),', '"front": (0, 1, 0), "air": (0, 1, 0), "window": (0, 1, 0),')

replace_function("_window", '''    def _window(self, sheet, rows, selected, cols, left=LEFT):
        """🪟 The workbook window over one sheet with live numeric cells; the drawing column to its right follows its width."""
        width = COLS[cols]
        self.dx = (left + width + 0.3 + 7.0) / 2
        window = self.xl_window(self.book, sheet, at=(left + width / 2, -0.05, 0.0), width=width, height=WIN_H, cols=cols.rstrip("+"), rows=rows, selected=selected, values=self.vs.a)
        return self._live(window, sheet)
''')

replace_function("_arrow", '''    def _arrow(self, drawing, row, grow, state):
        """➡️ One heat-flow arrow out of a surface (row 99: the air), as long and thick as its U·A (H_V for the air)."""
        air = row == 99
        face = "air" if air else ROWS[row]
        ua = self._num(self.T, "B12" if air else f"F{row}")
        value = ua if state.mode == "ua" else ua * (self._num(self.CL, "B2") - self._num(self.CL, "B3"))
        unit = "W/K" if state.mode == "ua" else "W"
        color = BLUE if air else SERV
        point, normal = np.array(POINT[face], dtype=float), np.array(NORMAL[face], dtype=float)
        start = drawing.at(*point)
        step = drawing.at(*(point + normal)) - start
        direction = step / max(np.linalg.norm(step), 1e-6)
        end = start + direction * (HIDDEN.get(face, 0.0) + 0.16 * ua) * grow
        group = _engine.VGroup(_engine.Arrow(start, end, buff=0, color=color, stroke_width=1.4 * ua, max_tip_length_to_length_ratio=0.5, max_stroke_width_to_length_ratio=40))
        if grow > 0.6:
            group.add(self._say(self.quantity(value, unit, 2 if unit == "W/K" else 0), end + direction * 0.14, size=SMALL, color=INK, anchor=self._side(direction)))
        group.add(_engine.Dot(start, radius=0.05, color=color))
        return group
''')

rep('''        if state.vent.get_value() > 0.01:
            group.add(self._vent(drawing, state.vent.get_value()))''', '''        if state.vent.get_value() > 0.01:
            group.add(self._arrow(drawing, 99, state.vent.get_value(), state))''')
replace_function("_vent", "")

replace_function("_chain", '''    def _chain(self, at, rows):
        """⛓️ A framed list of dependent cells: the label on one line and the live value under it."""
        pitch, width = 0.56, 3.7
        height = pitch * len(rows) + 0.2
        frame = _engine.RoundedRectangle(corner_radius=0.08, width=width, height=height, stroke_color=MUTED, stroke_width=_draft.W_THIN, fill_color=BACKGROUND, fill_opacity=0.9)
        frame.move_to(np.array([at[0], at[1], 0.0]))
        panel = _engine.VGroup(frame)
        top = at[1] + height / 2 - 0.1 - pitch / 2
        for i, (sheet, a1, label) in enumerate(rows):
            y = top - i * pitch
            panel.add(self._say(label, (at[0] - width / 2 + 0.15, y + 0.12), size=SMALL, color=INK, anchor="l", width=3.4),
                      self._readout(lambda s=sheet, a=a1: self._show(s, a), (at[0] + width / 2 - 0.15, y - 0.13), size=SMALL, color=DATA, anchor="r"))
        return panel
''')

replace_function("_prepare", '''    def _prepare(self):
        """🧮 Load the workbook, the model and every alternative workbook state of this chapter from the engine; what-if values are the model's named variants."""
        self.book = _xl.workbook(language=self.language())
        order = self.book.order()
        self.PR, self.T, self.CL, self.EN = order[0], order[1], order[-2], order[-1]
        self.cap = _nakagin.capsule_type("A")
        base = _xl.evaluate(self.book)
        self.vs = SimpleNamespace(a=base, b=base, t=_engine.ValueTracker(0.0))
        self.vals_base = base
        self.books = {}
        for name in ("window_upgrade", "back_wall_exposed", "shared_roof_floor"):
            events = []
            for key, override in _nakagin.variant(name)["overrides"].items():
                sheet, cell = self.book.names[key].replace("$", "").split("!")
                events.append({"type": "set-cell", "sheet": sheet, "cell": cell, "value": override.value})
            book = _xl.apply(self.book, events)
            self.books[name] = (book, _xl.evaluate(book))
        better_book, better_vals = self.books["window_upgrade"]
        frozen = _xl.apply(better_book, [{"type": "set-cell", "sheet": self.EN, "cell": "D8", "value": base[(self.EN, "D8")]}])
        self.books["frozen"] = (frozen, _xl.evaluate(frozen))
        self.vals_stacked = self.books["shared_roof_floor"][1]
        self.win, self.frames = None, []
''')

replace_function("_names", '''    def _names(self, at):
        """🏷️ The defined names of the Climate sheet with their live values, and the climate source and period."""
        CL = self.CL
        rows = [("theta_i", "B2"), ("theta_e", "B3"), ("heating_degree_days", "B5"), ("attached_factor", "B6"), ("air_change", "B7"), ("air_heat_capacity", "B8")]
        group = _engine.VGroup()
        for i, (name, a1) in enumerate(rows):
            y = at[1] + 1.75 - i * 0.5
            group.add(self.mono(name, (at[0] - 2.6, y), size=SMALL, color=DATA, anchor="l"), self._readout(lambda a=a1: self._show(CL, a), (at[0] + 2.6, y), size=SMALL, color=INK, anchor="r"))
        station = _nakagin.climate()["station"]
        station = station[self.language()] if isinstance(station, dict) else station
        group.add(self._say(station + " · " + self.tr("HDD base ", "Gradtage Basis ") + self.xl_display(self.vals_base[(CL, "B4")], '0 "°C"'), (at[0], at[1] - 1.55), size=SMALL, color=MUTED, width=5.2),
                  self._chip("assumed", (at[0] - 2.6, at[1] - 2.05)))
        return group
''')

replace_function("_recap", '''    def _recap(self, window, small, chain, counted):
        """🔁 The closing legend of terms in the language of the clip."""
        terms = [("U", self.tr("U-value", "U-Wert"), "W/(m²·K)"), ("U·A", self.tr("heat loss of one surface", "Wärmeverlust einer Fläche"), "W/K"),
                 ("b", self.tr("attached factor of the back wall", "Anbindungsfaktor der Rückwand"), "1"), ("H_T", self.tr("transmission loss", "Transmissionswärmeverlust"), "W/K"),
                 ("n", self.tr("air change rate", "Luftwechselrate"), "1/h"), ("H_V", self.tr("ventilation loss", "Lüftungswärmeverlust"), "W/K"),
                 ("Φ", self.tr("heating load", "Heizlast"), "W"), ("HDD", self.tr("heating degree days", "Heizgradtage"), "K·d"),
                 ("Q", self.tr("annual heating demand", "Jahresheizwärmebedarf"), "kWh/a"), ("Q/A", self.tr("demand per floor area", "Bedarf je Grundfläche"), "kWh/(m²·a)"),
                 ("×", self.tr("capsule count per type", "Kapselzahl je Typ"), "1"), (self.xl_localize("=SUMPRODUCT()").lstrip("=").rstrip("()"), self.tr("sum of products", "Summe der Produkte"), "")]
        legend = _engine.VGroup()
        for i, (symbol, name, unit) in enumerate(terms):
            y = 2.4 - i * 0.42
            legend.add(self._say(symbol, (-6.6, y), size=LABEL, color=DATA, anchor="l"), self._say(name, (-3.9, y), size=LABEL, color=INK, anchor="l", width=3.5),
                       self._say(unit, (-0.2, y), size=LABEL, color=MUTED, anchor="l"))
        kinds = self._slot_types()
        legend.add(self.tower_axon(at=(4.9, -0.1, 0.0), scale=0.085, tint=lambda key: _drawing.type_color(kinds[key]), lit=None))
        self.step("recap", _engine.FadeOut(window), *[_engine.FadeOut(m) for m in self.frames], _engine.FadeOut(chain), _engine.FadeOut(counted), _engine.FadeOut(small), _engine.FadeIn(legend), seconds=3.0)
''')

# shape labels
rep('self._say(self.tr("5 exposed faces", "5 Außenflächen"), (at[0] - 0.95, at[1] - 1.7), size=SMALL, color=INK, width=1.8)', 'self._say(self.tr("5 faces + window", "5 Flächen + Fenster"), (at[0] - 0.95, at[1] - 1.7), size=SMALL, color=INK, width=1.65)')
rep('self._say(self.tr("roof, floor shared", "Dach, Boden geteilt"), (at[0] + 0.95, at[1] - 1.7), size=SMALL, color=INK, width=1.8)', 'self._say(self.tr("roof, floor shared", "Dach, Boden geteilt"), (at[0] + 0.95, at[1] - 1.7), size=SMALL, color=INK, width=1.65)')
rep('(at[0] + 0.95, (5, 6, 7, 8), ("top",), alt)', '(at[0] + 0.95, (5, 6, 7, 8), ("top", "bottom"), alt)')

# narration
rep('''        ["ua", "Area times U-value is U·A in watts per kelvin. Each arrow is as long and thick as the U·A of its surface.",
         "Fläche mal U-Wert ergibt U·A in Watt je Kelvin. Jeder Pfeil ist so lang und dick wie das U·A seiner Fläche."],''', '''        ["ua", "Area times U-value is U·A in watts per kelvin. The larger the U·A of a surface, the longer and thicker its arrow.",
         "Fläche mal U-Wert ergibt U·A in Watt je Kelvin. Je größer das U·A einer Fläche, desto länger und dicker ihr Pfeil."],''')
rep('''"The sum of all U·A values is the transmission loss H_T. SUMPRODUCT gives the same result straight from areas and U-values."''', '''"H_T is the sum of all U·A values. SUMPRODUCT does it in one formula: area times U-value times temperature factor, back wall included."''')
rep('''"Die Summe aller U·A-Werte ist der Transmissionswärmeverlust H_T. SUMMENPRODUKT liefert dasselbe direkt aus Flächen und U-Werten."''', '''"H_T ist die Summe aller U·A-Werte. SUMMENPRODUKT rechnet es in einer Formel: Fläche mal U-Wert mal Temperaturfaktor, Rückwand eingeschlossen."''')
rep('"H_T plus H_V is the heat loss of the whole capsule per kelvin. Every later result is built on these two cells."', '"H_T plus H_V is the heat loss of the whole capsule per kelvin. Every later result is built on these two cells."')
rep('"H_T plus H_V ist der Wärmeverlust der ganzen Kapsel je Kelvin. Alle weiteren Ergebnisse bauen auf diesen beiden Zellen auf."', '"H_T und H_V zusammen sind der Wärmeverlust der ganzen Kapsel je Kelvin. Alle weiteren Ergebnisse bauen auf diesen beiden Zellen auf."')
rep('Beheizte Nachbarn oben und unten sparten viel.', 'Beheizte Nachbarn oben und unten würden den Bedarf deutlich senken.')
rep('"The factor lives in one cell on the Climate sheet. Set it back, and the back-wall arrow and H_T return to their values."', '"The factor is one cell on the Climate sheet. Set it back, and the back-wall arrow and H_T return to their values."')

# reorder climate after ht, new climate text
m = re.search(r'        \["climate", .*?\],\n(?=        \["load")', s, re.S)
climate_block = m.group(0)
s = s.replace(climate_block, "")
new_climate = '''        ["climate", "The Climate sheet stores Tokyo and the shared factors under defined names: temperatures, degree days, b, air change, air heat capacity.",
         "Das Blatt Klima speichert Tokio und die gemeinsamen Faktoren unter definierten Namen: Temperaturen, Gradtage, b, Luftwechsel, Luftwärmekapazität."],
'''
rep('        ["mistake",', new_climate + '        ["mistake",')
target.write_text(s, encoding="utf-8")
print("patched")
