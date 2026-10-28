"""🩹 Round 4 patch of the algorithms scene: one replacement per entry, each must match exactly once."""
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
path = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py"
text = path.read_text(encoding="utf-8")
edits = []


def edit(old, new, count=1):
    edits.append((old, new, count))


edit('''class Beat6_EvolutionarySearch(_visuals.ClipScene):''', '''def _half(value, digits=1):
    """➗ Round half up, the rounding of the printed numbers."""
    return math.floor(value * 10 ** digits + .5 + 1e-9) / 10 ** digits


@lru_cache(maxsize=8192)
def _figures_of(key):
    """🧾 Every printed figure of one design, each derived from the printed figures before it, so every row on screen closes."""
    design = dict(key)
    found = _room.result(design)
    systems = _room.systems()
    kilowatt = _half(_room.constant("lighting", "power_density") * _room.constant("room", "width") * _room.constant("room", "depth") / 1000, 3)
    share = _half(100 * (1 - found["autonomy"]), 3)
    loss = _half(_loss_of(found))
    heat_demand, cold_demand = _half(found["heating_demand"]), _half(found["cooling_demand"])
    heat, cold = _half(heat_demand / systems["cop"]), _half(cold_demand / systems["eer"])
    lamps = _half(kilowatt * _room.constant("lighting", "hours") * share / 100)
    return {"loss": loss, "gains": _half(loss - heat_demand), "heat_demand": heat_demand, "cold_demand": cold_demand, "heat": heat, "cold": cold, "share": share, "lamps": lamps,
            "total": _half(heat + cold + lamps)}


def _loss_of(found):
    """🌡️ Winter transmission loss in kWh/a: (U_wall · wall + U_window · window + other) · degree hours."""
    conductance = found["u_wall"] * found["wall_area"] + _room.constant("envelope", "u_window") * found["window_area"] + _room.constant("envelope", "h_other")
    return conductance * _room.constant("climate", "heating_degree_hours") / 1000


def _figures(design):
    """🧾 Printed figures of a design (rounded cache key, so live trackers hit the cache)."""
    return _figures_of(tuple((name, round(float(design[name]), 5)) for name in ("orientation", "wwr", "overhang", "insulation")))


class Beat6_EvolutionarySearch(_visuals.ClipScene):''')

# narration
edit('''"Pattern search first reaches within 1 % of the minimum after 16 simulations, with no slope.", "Die Mustersuche erreicht innerhalb von 1 % des Minimums nach 16 Simulationen, ganz ohne Steigung."''',
     '''"Pattern search gets within 1 % of the minimum first, after 16 simulations, with no slope.", "Die Mustersuche kommt als erste auf 1 % ans Minimum heran, nach 16 Simulationen, ganz ohne Steigung."''')
edit('''"Die Mustersuche ist zuerst am Ziel; alle drei enden innerhalb von 3,3 kWh/a des Kartenminimums."''', '''"Die Mustersuche ist zuerst am Ziel; alle drei enden höchstens 3,3 kWh/a über dem Kartenminimum."''')
edit('''"Die Toleranz greift zuerst: Der genetische Lauf stoppte nach 15 Simulationen, noch weit vom Besten."''', '''"Die Toleranz greift zuerst: Der genetische Lauf stoppte schon nach 15 Simulationen, noch weit vom Besten."''')
edit('''"None of them proves the global optimum; only the full map of 1,681 simulations could show it."''', '''"None of them proves the global optimum; only the full map with 1,681 simulations could show it."''')
edit('''"Keines beweist das globale Optimum; nur die volle Karte mit 1.681 Simulationen könnte es zeigen."''', '''"Keines beweist das globale Optimum; nur die komplette Karte (1.681 Simulationen) könnte es zeigen."''')

# ledger on printed figures
a = text.index("    def _loss(self, design, found):")
b = text.index("    def _seasons(self):")
text = text[:a] + '''    def _loss(self, design, found):
        """🌡️ Winter transmission loss in kWh/a."""
        return _loss_of(found)

    def sum_formula(self, point, size=14, trackers=None, width=5.6, unit="kWh/a"):
        """➕ The three electricity parts and their total at one decimal; every part is derived from its printed factors and the total is their printed sum."""
        pick = lambda key: (lambda design, found: _figures(design)[key])
        row = [self.live(pick("heat"), HEAT, 1), "+", self.live(pick("cold"), COOL, 1), "+", self.live(pick("lamps"), LIGHT, 1), "=", self.live(pick("total"), INK, 1, unit)]
        return self.formula(trackers or self.focus, [row], point, size=size, width=width)

    def _ledger(self):
        """🧾 How heating, cooling and lighting electricity of the focus design come about; factors and results are printed at one precision and every row closes."""
        tr = self.tr
        systems, hours = _room.systems(), _room.constant("lighting", "hours")
        kilowatt = _half(_room.constant("lighting", "power_density") * _room.constant("room", "width") * _room.constant("room", "depth") / 1000, 3)
        soft = _room.muted
        pick = lambda key: (lambda design, found: _figures(design)[key])
        balance = [[(tr("Loss", "Verlust"), soft(HEAT)), self.live(pick("loss"), soft(HEAT), 1), "−", (tr("useful gains", "nutzbare Gewinne"), soft(HEAT)), self.live(pick("gains"), soft(HEAT), 1), "=",
                    (tr("heat", "Wärme"), soft(HEAT)), self.live(pick("heat_demand"), HEAT, 1, "kWh/a")]]
        heating = [[(tr("Heat", "Wärme"), soft(HEAT)), self.live(pick("heat_demand"), soft(HEAT), 1, "kWh/a"), "÷", ("COP", INK), self.live(systems["cop"], INK, 1), "=", self.live(pick("heat"), HEAT, 1, "kWh/a")]]
        cooling = [[(tr("Cold", "Kälte"), soft(COOL)), self.live(pick("cold_demand"), soft(COOL), 1, "kWh/a"), "÷", ("EER", INK), self.live(systems["eer"], INK, 1), "=", self.live(pick("cold"), COOL, 1, "kWh/a")]]
        lamps = [[(tr("Lamps", "Lampen"), soft(LIGHT)), self.live(kilowatt, INK, 3, "kW"), "·", self.live(hours, INK, 0, "h/a"), "·", (tr("on", "an"), soft(LIGHT)), self.live(pick("share"), LIGHT, 3, "%"), "=",
                  self.live(pick("lamps"), LIGHT, 1, "kWh/a")]]
        return mn.VGroup(*[self.formula(self.focus, rows, (LEDGER_X, y, 0), size=16, align="left", width=6.7) for rows, y in zip((balance, heating, cooling, lamps), LEDGER_Y)])

    def _bar(self, point=BAR_AT, height=BAR_H, maximum=900, width=.8):
        """📊 Stacked electricity bar of the focus design at the fixed scale; the total above it is the printed sum of the ledger, at one decimal like every other number on the frame."""
        base, unit = np.array(point, dtype=float), height / maximum
        static = mn.VGroup(mn.Line(base + np.array([-.6, 0, 0]), base + np.array([.6, 0, 0]), color=MUTED, stroke_width=2),
                           mn.Line(base + np.array([-width / 2 - .3, 0, 0]), base + np.array([-width / 2 - .3, height, 0]), color=MUTED, stroke_width=1.2, stroke_opacity=.6),
                           mn.Line(base + np.array([-width / 2 - .3, height, 0]), base + np.array([-width / 2 - .2, height, 0]), color=MUTED, stroke_width=1.2, stroke_opacity=.6))

        def build():
            design = _room.values(self.focus)
            found = _room.result(design)
            group, cursor = mn.VGroup(), 0.0
            for key, colour in (("heating", HEAT), ("cooling", COOL), ("lighting", LIGHT)):
                span = found[key] * unit
                group.add(mn.Rectangle(width=width, height=span, color=colour, stroke_width=1.2, fill_color=colour, fill_opacity=.78).move_to(np.array([base[0], base[1] + cursor + span / 2, 0])))
                cursor += span
            group.add(_room.text(self._p(_figures(design)["total"], "kWh/a", 1), np.array([base[0], base[1] + cursor + .3, 0]), size=17, color=INK))
            return group
        return mn.VGroup(static, self._live(build, _room.design_key(self.focus)))

''' + text[b:]
edit("        self.bar = self.energy_bars(self.focus, BAR_AT, BAR_H, 900, compact=True, intensity=False)", "        self.bar = self._bar()")
edit("BAR_AT, BAR_H = (6.25, -2.0, 0), 2.6", "BAR_AT, BAR_H = (6.15, -2.0, 0), 2.6")

# task: the bar appears with its parts
edit('''self.reveal(*self.rooms, self.bar, self.note_chip, self.design_chip), seconds=1.4)
        self.step("task_2", self.reveal(*self.ledger), seconds=1.6)''', '''self.reveal(*self.rooms, self.note_chip, self.design_chip), seconds=1.4)
        self.step("task_2", self.reveal(*self.ledger, self.bar), seconds=1.6)''')

# cards: printed figures, one decimal, exact values for the gradient probes
edit('''    def _card(self, item, point, *, width=1.4, label=None, note=None, digits=(0, 2), power=1, ghost=False, ring=None, color=None, value=False, tail=()):''',
     '''    def _card(self, item, point, *, width=1.46, label=None, note=None, digits=(1, 2), power=1, ghost=False, ring=None, color=None, value=False, tail=(), exact=False):''')
edit('''_room.text([*self._p(_room.result(design)["electricity"], "kWh/a", power), *tail]''', '''_room.text([*self._p(item["electricity"] if exact else _figures(design)["total"], "kWh/a", power), *tail]''')
edit('''                edge = search and (abs(probe["design"][axis] * scale - BOUNDS[axis][0]) < 1e-6 or abs(probe["design"][axis] * scale - BOUNDS[axis][1]) < 1e-6)''',
     '''                at = probe["design"][axis] * scale
                edge = search and (abs(at - BOUNDS[axis][0]) < 1e-6 or abs(at - BOUNDS[axis][1]) < 1e-6)''')
edit('''                note = [self.tr("limit: no move", "Rand: kein Zug")] if edge else None''',
     '''                note = None if not edge else [self.tr("limit: no move", "Rand: kein Zug")] if abs(delta) < 1e-9 else [self.tr("at limit", "am Rand"), self.quantity(at, "%" if axis == "wwr" else "m", 0 if axis == "wwr" else 2)]''')
edit('''width=1.68, label=words, note=note, digits=(1, 3) if not search else (1, 2), power=1,
                                     ring=GOOD if winner else None, color=WINDOWS if axis == "wwr" else OVERHANG))''',
     '''width=1.68, label=words, note=note, digits=(1, 3) if not search else (1, 2),
                                     power=1 if search or axis == "wwr" else 3, exact=not search, ring=GOOD if winner else None, color=WINDOWS if axis == "wwr" else OVERHANG))''')
edit('''(CARD_X + CARD_PITCH * (slot - 1.5), y, 0), width=1.68''', '''(CARD_X + CARD_PITCH * (slot - 1.5), y, 0), width=1.68''')
edit("CARD_X, CARD_PITCH = 1.9, 1.72", "CARD_X, CARD_PITCH = 1.9, 1.72")
edit('''SLOTS = [(CARD_X + 1.5 * (i % 4 - 1.5)''', '''SLOTS = [(1.5 + 1.52 * (i % 4 - 1.5)''') if False else None

# lens label
edit('''_room.text(["×", self._n(LENS_ZOOM)]''', '''_room.text([self.tr("zoom", "Zoom"), "×", self._n(LENS_ZOOM)]''')
edit('''lens + mn.DOWN * .5, size=14, color=INK))''', '''lens + mn.DOWN * .82, size=14, color=INK))''')

# step rule: define eta
edit('''self.tr("step = −η · slope: move against the slope", "Schritt = −η · Steigung: gegen die Steigung gehen")''',
     '''self.tr("step = −η · slope, η: one fixed step factor", "Schritt = −η · Steigung, η: ein fester Schrittfaktor")''')

# step rows from printed totals
edit('''        old, new = round(before["electricity"], 1), round(after["electricity"], 1)''', '''        old, new = _figures(before["design"])["total"], _figures(after["design"])["total"]''')

# change row
edit('''            delta = rows[index]["electricity"] - rows[index - 1]["electricity"]
            colour = GOOD if delta < 0 else BAD
            return mn.VGroup(self._chip([self.tr("Change", "Änderung"), self._signed(round(delta), "kWh/a", 0)], (CARD_X, ROW_Y - .36, 0), color=colour, width=2.4, size=15))''',
     '''            delta = _figures(rows[index]["design"])["total"] - _figures(rows[index - 1]["design"])["total"]
            colour = GOOD if delta < 0 else BAD
            return mn.VGroup(self._chip([self.tr("change to the previous iterate", "Änderung zum vorigen Iterat"), self._signed(_half(delta), "kWh/a", 1)], (CARD_X, ROW_Y - .36, 0), color=colour, width=4.2, size=15))''')

# pattern band: before and after
a = text.index("    def _pattern_band(self, rows, reader):")
b = text.index("    def _card_live(self, item, point):")
text = text[:a] + '''    def _pattern_band(self, rows, reader):
        """⚖️ The decision of the current compass round: the design before the move and the design after it, both with their values, or the halved step."""
        def build():
            index = reader()
            item, previous = rows[index], rows[max(index - 1, 0)]
            if item["moved"]:
                return mn.VGroup(self._line([(self.tr("before", "vorher"), MUTED), (self._p(_figures(previous["design"])["total"], "kWh/a", 1), INK), "→", (self.tr("after", "nachher"), GOOD),
                                             (self._p(_figures(item["design"])["total"], "kWh/a", 1), GOOD), (self.tr("(moved to the best neighbour)", "(zum besten Nachbarn gezogen)"), MUTED)],
                                            (CARD_X, -.8, 0), size=18, width=6.7))
            step = item["step"]["wwr"] * 100
            digits = lambda value: 0 if abs(value - round(value)) < 1e-6 else 1
            return mn.VGroup(self._line([self.tr("no neighbour below", "kein Nachbar unter"), (self._p(_figures(previous["design"])["total"], "kWh/a", 1), INK), "→", (self.tr("no improvement", "keine Besserung"), BAD)],
                                        (CARD_X, -.8, 0), size=18, width=6.7),
                             self._line([(self.tr("step ÷ 2:", "Schritt ÷ 2:"), BAD), (self._p(step, "", digits(step)), INK), "→", (self._p(step / 2, "%", digits(step / 2)), BAD), "·",
                                         (self._p(item["step"]["overhang"], "", 2), INK), "→", (self._p(item["step"]["overhang"] / 2, "m", 2), BAD)], (CARD_X, -1.1, 0), size=18, width=6.7))
        return self._live(build, lambda: (reader(),))

''' + text[b:]
edit('''(self._p(round(rows[reader()]["electricity"], 1) - round(minimum, 1), "kWh/a", 1), BAD),''', '''(self._p(_figures(rows[reader()]["design"])["total"] - round(minimum, 1), "kWh/a", 1), BAD),''')
edit('''(self.tr("still", "noch"), INK)''', '''(self.tr("after: still", "nachher: noch"), INK)''')

# results: one decimal like the design chip, start from printed total
edit('''*self._p(design["wwr"] * 100, "%", 0), "·", self.tr("overhang", "Vordach"), *self._p(design["overhang"], "m", 2), "·",
                        *self._p(electricity, "kWh/a", 1)''', '''*self._p(design["wwr"] * 100, "%", 1), "·", self.tr("overhang", "Vordach"), *self._p(design["overhang"], "m", 2), "·",
                        *self._p(electricity, "kWh/a", 1)''')
edit('''        start = _room.result(self.start)["electricity"]''', '''        start = _figures(self.start)["total"]''')

# map minimum chip and star label
edit('''found = self._chip([self.tr("map minimum", "Kartenminimum"), *self._p(self.optimum["electricity"], "kWh/a", 1)], (spot[0] - .55, spot[1] - .45, 0), color=GOOD, width=2.3).set_z_index(8)''',
     '''found = self._chip([*self._p(self.optimum["electricity"], "kWh/a", 1), "·", *self._p(self.optimum["design"]["wwr"] * 100, "%", 0), "·", *self._p(self.optimum["design"]["overhang"], "m", 2)],
                           (spot[0] - .45, spot[1] - .45, 0), color=GOOD, width=3.1).set_z_index(8)''')
edit('''        self.heat["star"].set_z_index(5)
''', '''        self.heat["star"].set_z_index(5)
        self.heat["star"].add(self._chip([self.tr("map minimum", "Kartenminimum")], self.heat["star"].get_center() + mn.UP * .3, size=14, color=GOOD, width=1.7))
''')

# band label of the 1 % band
edit('''        band_label = self._chip([self.tr("within 1 %: up to", "innerhalb 1 %: bis"), *self._p(limit, "kWh/a", 1)], ax.c2p(budget * .25, low + (minimum - low) * .33), color=GOOD, size=14, width=2.6)''',
     '''        band_label = self._chip([self.tr("1 % above:", "1 % darüber:"), *self._p(minimum, "", 2)[:1], "× 1.01 =" if not self._german() else "× 1,01 =", *self._p(limit, "kWh/a", 1)], ax.c2p(budget * .25, low + (minimum - low) * .33), color=GOOD, size=14, width=3.2)''')
edit('''ax.c2p(budget * .72, low + (minimum - low) * .33), color=GOOD, width=2.4)''', '''ax.c2p(budget * .8, low + (minimum - low) * .33), color=GOOD, width=2.3)''')

# stopping: if stopped, bracket legend, counter
edit('''*self._p(electricity, "kWh/a", 1), self.tr("after", "nach"), self._n(run), self.tr("simulations", "Simulationen")], (3.9, y + .15, 0), color=colour, size=14, width=3.7)''',
     '''*self._p(electricity, "kWh/a", 1), self.tr("if stopped after", "bei Stopp nach"), self._n(run), self.tr("simulations", "Simulationen")], (3.9, y + .15, 0), color=colour, size=14, width=3.9)''')
edit('''        self.step("stopping", mn.Create(line), mn.FadeIn(flagged), *self._fade([rows, links, gaps, marks]), mn.FadeIn(panel))''',
     '''        brackets = self._chip([self.tr("bracket: the 5 simulations the tolerance rule compares", "Klammer: die 5 Simulationen, die die Toleranz vergleicht")], (2.65, -.25, 0), size=14, width=5.9)
        self.step("stopping", mn.Create(line), mn.FadeIn(flagged), *self._fade([rows, links, gaps, marks, self.counter]), mn.FadeIn(panel), mn.FadeIn(brackets))''')
edit('''        self.step("stopping_2", mn.FadeOut(panel), mn.FadeIn(results))''', '''        self.step("stopping_2", mn.FadeOut(panel), mn.FadeOut(brackets), mn.FadeIn(results))''')
edit('''        self.later = mn.VGroup(''', '''        self.later = mn.VGroup(''')

# what-if marker wording
edit('''tail=[self.tr("(what-if)", "(fiktiv)")]''', '''tail=[self.tr("(what-if, not simulated)", "(nicht simuliert)")]''')

# rule chips without spaces before commas
edit('''        return self._chip([self.tr("mutate:", "Mutation:"), self.tr("windows", "Fenster"), "±" + self.number(settings["wwr"] * 100, 0) + " %", ",", self.tr("overhang", "Vordach"),
                           "±" + self.quantity(settings["overhang"], "m", 1), ",", "p = " + self.number(settings["probability"], 1)], (CARD_X, -.55, 0), size=18, width=6.7)''',
     '''        return self._chip([self.tr("mutate:", "Mutation:"), self.tr("windows", "Fenster") + " ±" + self.number(settings["wwr"] * 100, 0) + " %,", self.tr("overhang", "Vordach") + " ±" + self.quantity(settings["overhang"], "m", 1) + ",",
                           "p = " + self.number(settings["probability"], 1)], (CARD_X, -.72, 0), size=18, width=6.7)''')
edit('''(self.tr("child = (windows of", "Kind = (Fenster von"), INK), (letter_of(child["genes"]["wwr"]), colour_of(child["genes"]["wwr"])),
                           (self.tr(", overhang of", ", Vordach von"), INK), (letter_of(child["genes"]["overhang"]), colour_of(child["genes"]["overhang"])), (")", INK)]''',
     '''(self.tr("child = (windows of", "Kind = (Fenster von"), INK), (letter_of(child["genes"]["wwr"]) + ",", colour_of(child["genes"]["wwr"])),
                           (self.tr("overhang of", "Vordach von"), INK), (letter_of(child["genes"]["overhang"]) + ")", colour_of(child["genes"]["overhang"]))]''')

# survival legend for the green frame
edit('''self._chip([self.tr("kept: best two parents of generation 0", "behalten: die zwei besten Eltern der Generation 0")], (CARD_X, -.2, 0), color=STAR, width=4.6)).set_z_index(9)''',
     '''self._chip([self.tr("yellow: kept, the best two parents of generation 0", "gelb: behalten, die zwei besten Eltern der Generation 0")], (CARD_X, -.2, 0), color=STAR, width=6.4),
                            self._chip([self.tr("green: best card of generation 1", "grün: beste Karte der Generation 1")], (CARD_X, -.55, 0), color=GOOD, width=6.4)).set_z_index(9)''')

# design chip hidden while the genetic population is not simulated
edit('''self.heat["star"], *self.rooms, self.seasons, *self.ledger, self.bar, self.note_chip]),''', '''self.heat["star"], *self.rooms, self.seasons, *self.ledger, self.bar, self.note_chip, self.design_chip]),''')

# close: cost explained
edit('''self.tr("façade cost", "Fassade"), *self._p(found["cost"], "CHF", 0)]''', '''self.tr("one-off façade cost", "Fassade einmalig"), *self._p(found["cost"], "CHF", 0)]''')

for old, new, count in [item for item in edits if item]:
    assert text.count(old) == count, (text.count(old), old[:90])
    text = text.replace(old, new)
Path("C:/Users/Ueli/AppData/Local/Temp/claude/C--git-semio/c6859e80-281a-48bf-9c1f-0d7da30016af/scratchpad/patched.py").write_text(text, encoding="utf-8")
print("patched", len([item for item in edits if item]))
