import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


# ---------- part 1
sub(1, "BUDGET_CHF = 21000", "BUDGET_CHF = _model.brief()[\"budget_chf\"]")
sub(1, "FRAME_X0, FRAME_X1 = -7.08, 0.85", "FRAME_X0, FRAME_X1 = -7.08, 0.1")
sub(1, """            specs.append(("off", offset, "0" if offset == 0 else ("+" if self.sign > 0 else "−") + s.number(offset, self.digits_off)))""",
    """            label = "0" if offset == 0 else ("+" if self.sign > 0 else "−") + s.number(offset, self.digits_off) + (" " + CRITERIA[self.criterion]["unit"] if abs(offset - self.step_off) < 1e-9 else "")
            specs.append(("off", offset, label))""")
sub(1, """            group.add(mn.DashedVMobject(mn.Rectangle(width=frame_w, height=tall, color=MUTED, stroke_width=2.2, stroke_opacity=self.old.get_value()).move_to([frame_x, self.y(self.old_id) - drop, 0]), num_dashes=48))""",
    """            for edge in (tall / 2, -tall / 2):
                group.add(mn.DashedVMobject(mn.Line([frame_x - frame_w / 2, self.y(self.old_id) - drop + edge, 0], [frame_x + frame_w / 2, self.y(self.old_id) - drop + edge, 0], color=MUTED, stroke_width=2.2,
                                                    stroke_opacity=self.old.get_value()), num_dashes=40))""")
sub(1, """(xl + W + .25, y, 0), "left")
                          .set_opacity(self.scores.get_value() * level))""", """(xl + W - .1, y, 0), "right")
                          .set_opacity(self.scores.get_value() * level))""")
sub(1, """ENERGY = (""", """VARIANT_COLOR = {"glass_shaded": _art.P_CYAN, "ribbon": _art.P_GREEN, "punched": _art.PASTEL_PURPLE, "punched_insulated": _art.P_YELLOW, "glass_bare": _art.P_ORANGE, "glass_premium": _art.P_RED}
ENERGY = (""")

# ---------- part 2
sub(2, '''"Which of five façade designs suits this seminar room best?", "Welcher von fünf Fassadenentwürfen passt am besten zu diesem Seminarraum?"''',
    '''"Which of five façade designs suits this seminar room best?", "Welcher von fünf Fassadenentwürfen passt am besten zum Seminarraum?"''')
sub(2, '''"Kilowatt-hours per m² and year plus francs? That sum has no unit.", "Kilowattstunden pro m² und Jahr plus Franken? Die Summe hat keine Einheit."''',
    '''"Kilowatt-hours per m² and year plus francs? That sum has no unit.", "Kilowattstunden pro m² und Jahr plus Franken? Keine Einheit."''')
sub(2, '''"Cost in kCHF, carbon in tonnes: the ranking changes.", "Kosten in kCHF, Emissionen in Tonnen: Die Rangfolge ändert sich."''',
    '''"Cost in kCHF, carbon in tonnes: the ranking changes.", "Kosten in kCHF, Graue Emissionen in Tonnen: Rangfolge ändert sich."''')
sub(2, '''"Jeder Entwurf liegt auf seinem echten Wert; der Raum zeigt, was er bedeutet."''', '''"Jeder Entwurf liegt auf seinem Wert; der Raum zeigt dessen Sinn."''')
sub(2, '''"Sonne wird zum Kühlbedarf; die Kältemaschine teilt ihn durch ihre Effizienz."''', '''"Sonne wird zum Kühlbedarf; die Kältemaschine teilt ihn durch EER."''')
sub(2, '''"In winter the room loses heat; the heat pump needs less electricity than heat."''', '''"In winter the room loses heat; the pump needs less power than heat."''')
sub(2, '''"Lampen kommen dazu, weniger bei Tageslicht; die Summe ist der Strombedarf."''', '''"Lampen kommen dazu, weniger bei Tageslicht; Summe: der Strombedarf."''')
sub(2, '''"Der Preis ist die Summe seiner Teile: Festes, Wand, Dämmung, Fenster und Vordach."''', '''"Der Preis ist die Summe seiner Teile: Fixposten, Wand, Dämmung, Fenster und Vordach."''')
sub(2, '''"Embodied carbon is the sum of the façade's materials, in kilograms per layer."''', '''"Embodied carbon is the sum of the façade's materials, per layer."''')
sub(2, '''"Each part is weight times normalized value; the four parts add up to the index.", "Jeder Teil ist Gewicht mal normierter Wert; die vier Teile ergeben den Index."''',
    '''"Weight times normalized value, summed over four criteria: the index.", "Gewicht mal normierter Wert, über vier Kriterien summiert: der Index."''')
sub(2, '''"Knappe Entscheidungen sind fragil: Prüfe die Rangfolge, bevor du entscheidest."''', '''"Knappe Entscheidungen sind fragil: Rangfolge vorher prüfen."''')
sub(2, '''"Best and worst move, so all other normalized values shrink.", "Bester und schlechtester Wert verschieben sich, alle anderen Werte schrumpfen."''',
    '''"Premium: worst cost, best electricity; all other values shrink.", "Premium ist am teuersten und am sparsamsten; alle anderen schrumpfen."''')
sub(2, '''"Die Reihenfolge der ersten fünf kann kippen, obwohl sie unverändert blieben."''', '''"Die Reihenfolge der ersten fünf kippt, obwohl sie gleich blieben."''')
sub(2, '''"Document weights, compared designs and value ranges with every decision."''', '''"Record weights, designs and value ranges with every decision."''')
sub(2, '''"The client's budget is 21,000 CHF: a mandatory limit, applied before any weights."''', '''"Client budget 21,000 CHF: a mandatory limit, applied before weights."''')
sub(2, '''"Feste Positionen", "fixed")''', '''"Fixposten", "fixed")''')
sub(2, '''self.tr("Daylit (≥ 2 %) to", "Tageshell (≥ 2 %) bis")), self.n(reach, 1), "m", *self.words(self.tr("of 6 m depth", "von 6 m Tiefe"))''',
    '''self.tr("Daylit ≥ 2 % to", "Tageslichtversorgt ≥ 2 % bis")), self.n(reach, 1), "m", *self.words(self.tr("of 6 m (linear falloff)", "von 6 m (lineares Abklingen)"))''')
sub(2, '''(LEFT + .2, 3.68, 0), "left")''', '''(LEFT + .2, 3.68, 0), "left")''')
sub(2, '''"Simplified annual teaching model · Zurich-like climate · EN ISO 13790 style",''', '''"Simplified annual teaching model · degree-hours method · EN ISO 13790 style",''')
sub(2, '''"Vereinfachtes Jahres-Lehrmodell · Zürich-ähnliches Klima · Stil EN ISO 13790")''', '''"Vereinfachtes Jahres-Lehrmodell · Gradstundenverfahren · Stil EN ISO 13790")''')

# dimension: plate above the rays
sub(2, """            return mn.VGroup(_room._dimension(a, b, color=WARN, tick=.07), _room.text(["d", self.n(depth, 1), "m"], (a + b) / 2 + np.array([0, .22, 0]), size=14, color=WARN))""",
    """            return mn.VGroup(_room._dimension(a, b, color=WARN, tick=.07), _room.chip(["d", self.n(depth, 1), "m"], (a + b) / 2 + np.array([0, .3, 0]), size=14, color=WARN)).set_z_index(8)""")
sub(2, """                return mn.VGroup(label, mn.Line(label.get_left() + np.array([-.05, 0, 0]), P(x_o + .05, HEIGHT + SLAB + .03), color=WARN, stroke_width=1.4, stroke_opacity=.8))""",
    """                return mn.VGroup(label, mn.Line(label.get_left() + np.array([-.05, 0, 0]), P(x_o + .05, HEIGHT + SLAB + .03), color=WARN, stroke_width=1.4, stroke_opacity=.8)).set_z_index(8)""")

# raws shared by blocks and strip
sub(2, """            price = lambda name: _room.constant("prices", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = [price("fixed"), wall * price("wall"), wall * thickness * price("insulation_per_cm"), window * price("window"), WIDTH * depth * price("overhang")]
            shown = _room.apportion(raws, 0)""", """            price = lambda name: _room.constant("prices", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = self.cost_raws(design, found)
            shown = _room.apportion(raws, 0)""")
sub(2, """            factor = lambda name: _room.constant("carbon", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = [wall * factor("wall"), wall * thickness * factor("insulation_per_cm"), window * factor("window"), WIDTH * depth * factor("overhang")]
            shown = _room.apportion(raws, 0)""", """            factor = lambda name: _room.constant("carbon", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = self.carbon_raws(design, found)
            shown = _room.apportion(raws, 0)""")
sub(2, """    def value_strip(self):""", """    def cost_raws(self, design, found):
        \"\"\"💰 The five cost parts in CHF: fixed items, wall, insulation, windows, overhang.\"\"\"
        price = lambda name: _room.constant("prices", name)
        return [price("fixed"), found["wall_area"] * price("wall"), found["wall_area"] * design["insulation"] * 100 * price("insulation_per_cm"), found["window_area"] * price("window"),
                WIDTH * design["overhang"] * price("overhang")]

    def carbon_raws(self, design, found):
        \"\"\"🏭 The four carbon layers in kg CO₂e: masonry, insulation, glass, overhang.\"\"\"
        factor = lambda name: _room.constant("carbon", name)
        return [found["wall_area"] * factor("wall"), found["wall_area"] * design["insulation"] * 100 * factor("insulation_per_cm"), found["window_area"] * factor("window"),
                WIDTH * design["overhang"] * factor("overhang")]

    def value_strip(self):""")
a = texts[2].index("        def build():\n            found = _room.result(_room.values(rt))\n            shown, active")
b = texts[2].index("        return live(build, lambda: (_room.design_key(rt)(), round(self.count.get_value()), round(self.active.get_value())))")
texts[2] = texts[2][:a] + '''        def build():
            design = _room.values(rt)
            found = _room.result(design)
            shown, active = int(round(self.count.get_value())), int(round(self.active.get_value()))
            parts = {"electricity": ([found[key] for key, _ in ENERGY], [colour for _, colour in ENERGY], 760),
                     "cost": (self.cost_raws(design, found), [LAYER[k] for k in ("fixed", "wall", "insulation", "window", "overhang")], 22500),
                     "daylight": ([found["daylight"]], [_room.LIGHT], 6),
                     "carbon": (self.carbon_raws(design, found), [LAYER[k] for k in ("wall", "insulation", "window", "overhang")], 5600)}
            group = mn.VGroup()
            for k, c in enumerate(ORDER[:shown]):
                x, y = 2.1 + 2.45 * (k % 2), -2.05 - .38 * (k // 2)
                label = _room.text([self.n(found[CRITERIA[c]["key"]], DIGITS[c]), *self.words(self.unit_label(c))], size=14, color=COLORS[c])
                _room.place(label, (x + .42, y, 0), "left")
                group.add(_room.clone(glyph(c, width=.26)).move_to([x + .17, y, 0]), label)
                values, colours, full = parts[c]
                cursor = x + .42
                for value, colour in zip(values, colours):
                    width = max(value / full * 1.9, 1e-3)
                    group.add(mn.Rectangle(width=width, height=.08, stroke_width=0, fill_color=colour, fill_opacity=.9).move_to([cursor + width / 2, y - .17, 0]))
                    cursor += width
                if c == "daylight":
                    group.add(mn.Line([x + .42 + 2 / 6 * 1.9, y - .22, 0], [x + .42 + 2 / 6 * 1.9, y - .12, 0], color=INK, stroke_width=1.6))
                if k == active:
                    group.add(mn.Rectangle(width=2.35, height=.4, color=COLORS[c], stroke_width=2.4).move_to([x + 1.12, y - .07, 0]))
            return group
''' + texts[2][b:]

# index table full: three decimals
a = texts[2].index("            for k, c in enumerate(ORDER):\n                yy = 2.4 - .42 * k")
b = texts[2].index("            return group\n        return live(build, lambda: (getter(), tuple(round(w, 4) for w in weights())))")
texts[2] = texts[2][:a] + '''            exact = table(tuple(round(w, 6) for w in weights()))[identifier]["normalized"]
            norm3 = [round(exact[c], 3) for c in ORDER]
            terms = [round(w * v, 3) for w, v in zip(printed_w, norm3)]
            total3 = round(sum(terms), 3)
            for k, c in enumerate(ORDER):
                yy = 2.45 - .55 * k
                items = [([self.n(raw(identifier, c), DIGITS[c]), *self.words(self.unit_label(c))], COLORS[c]), (["→"], MUTED),
                         ([self.tr("normalized", "normiert"), self.n(norm3[k], 3), "×", self.n(printed_w[k] * 100, digits - 2), "%", "="], INK), ([self.n(terms[k], 3)], COLORS[c])]
                group.add(_room.clone(glyph(c, width=.26)).move_to([LEFT + .15, yy, 0]), self.equation(items, LEFT + .45, yy - .07, size=14, width=6.4))
            group.add(self.equation([([self.tr("Index", "Index"), "="], INK), ([self.n(terms[0], 3), "+", self.n(terms[1], 3), "+", self.n(terms[2], 3), "+", self.n(terms[3], 3), "="], INK),
                                     ([self.n(total3, 3)], GOOD)], LEFT + .45, 2.45 - .55 * 4 - .07, size=16, width=6.4))
''' + texts[2][b:]

# ---------- part 3
sub(3, "-6.35, Y(k), 0), .11, ramp=self._ramp, value=self._ramp.low), self.name(item[\"id\"], size=S0, width=1.55).move_to([-5.25, Y(k), 0]))",
    "-6.3, Y(k), 0), .15, ramp=self._ramp, value=self._ramp.low), self.name(item[\"id\"], size=S0, width=1.4).move_to([-5.05, Y(k), 0]))")
sub(3, """        label = f"{sign} {self.number(value, ruler.digits_tag)} {self.unit_label(ruler.criterion)}"
        return text(label, (ruler.center, -1.15, 0), size=S1, color=ruler.color)""",
    """        label = _room.text([sign, self.n(value, ruler.digits_tag), self.unit_label(ruler.criterion)], (ruler.center, -1.15, 0), size=S1, color=ruler.color)
        return label""")
sub(3, "self.step_note(ruler, \"÷\", ruler.high - ruler.low)", "self.step_note(ruler, \"÷\", round(ruler.high, ruler.digits_tag) - round(ruler.low, ruler.digits_tag))")
# daylight legend
sub(3, """            new, new_header, new_block, run = self.next_ruler(key, ruler, header, block, [zero_one] if zero_one is not None else [])
            self.step(key, mn.Succession(*run), seconds=3.6)""", """            new, new_header, new_block, run = self.next_ruler(key, ruler, header, block, [zero_one] if zero_one is not None else [])
            sky = mn.VGroup(_room.place(_room.text(self.words(self.tr("sky light · in DF", "Himmelslicht · im TQ")), size=14, color=_room.DAYLIGHT), (2.15, 2.75, 0), "left"),
                            _room.place(_room.text(self.words(self.tr("direct sun · not in DF", "Direktsonne · nicht im TQ")), size=14, color=_room.SUN), (2.15, 2.45, 0), "left"))
            self.step(key, mn.Succession(*run), *([mn.FadeIn(sky)] if key == "daylight" else []), seconds=3.6)""")
sub(3, """        new, new_header, new_block, run = self.next_ruler("carbon", ruler, header, block, [], retire=[self.overlay])""", """        new, new_header, new_block, run = self.next_ruler("carbon", ruler, header, block, [sky], retire=[self.overlay])""")
# winter note
a = texts[3].index("    def winter_note(self):")
b = texts[3].index("    def step_note(self, ruler, sign, value):")
texts[3] = texts[3][:a] + '''    def winter_note(self):
        """❄️ What the winter picture means: conductance of wall and window, heat loss over the degree hours, the free heat that offsets it, and (toolkit derivation) demand ÷ COP."""
        rt = self._rt
        constant = _room.constant

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            conductance = found["u_wall"] * found["wall_area"] + constant("envelope", "u_window") * found["window_area"] + constant("envelope", "h_other")
            hours = constant("climate", "heating_degree_hours") / 1000
            loss = conductance * hours
            digits, _ = reproduce(lambda value: value * hours, [conductance], loss, 0, limit=3)
            parts = _room.apportion([found["u_wall"] * found["wall_area"], constant("envelope", "u_window") * found["window_area"], constant("envelope", "h_other")], digits, total=conductance)
            free = round(loss) - round(found["heating_demand"])
            group = mn.VGroup(self.equation([([self.tr("Winter (Zurich-like climate): sun low, heat leaves through wall and window", "Winter (Zürcher Klima): Sonne tief, Wärme geht durch Wand und Fenster verloren")], _room.HEAT)],
                                            LEFT + .4, 3.0, size=16, width=6.2),
                              self.equation([([self.tr("Conductance", "Leitwert"), "H", "=", self.n(parts[0], digits), "+", self.n(parts[1], digits), "+", self.n(parts[2], digits), "=", self.n(sum(parts), digits), "W/K"], INK),
                                             ([self.tr("(wall U", "(Wand U"), self.n(found["u_wall"], 2), "~,", self.tr("window U", "Fenster U"), self.n(constant("envelope", "u_window"), 1), "~,", self.tr("other", "übrige"), "~)"], MUTED)],
                                            LEFT + .4, 2.6, size=15, width=6.2),
                              self.equation([([self.n(sum(parts), digits), "W/K", "×", self.n(constant("climate", "heating_degree_hours")), "Kh/a", "=", self.n(loss), "kWh/a", self.tr("loss", "Verlust"), "−",
                                               self.n(free), self.tr("free heat", "freie Wärme"), "=", self.n(found["heating_demand"]), "kWh/a"], INK)], LEFT + .4, 2.25, size=15, width=6.2))
            return group
        return live(build, _room.design_key(rt))

''' + texts[3][b:]
sub(3, """        winter = self.winter_note()""", """        winter = mn.VGroup(self.winter_note(), self.derivation(self._rt, "heating", (LEFT + .4, 1.6, 0), size=18, words=True, align="left", width=5.6))""")
# sweep colours
sub(3, """            strip.add(mn.Line([a + .02, Y(1) + .16, 0], [b - .02, Y(1) + .16, 0], color=GOOD, stroke_width=6),
                      self.thumbnail(by_id(winner)["design"], ((a + b) / 2, Y(1) + .52, 0), .075, ramp=self._ramp, value=self._ramp.low, ring=GOOD))""",
    """            strip.add(mn.Line([a + .02, Y(1) + .16, 0], [b - .02, Y(1) + .16, 0], color=VARIANT_COLOR[winner], stroke_width=6),
                      self.thumbnail(by_id(winner)["design"], ((a + b) / 2, Y(1) + .52, 0), .075, ramp=self._ramp, value=self._ramp.low, ring=VARIANT_COLOR[winner]))""")
sub(3, """            names.add(_room.place(self.name(i, size=14, width=1.7), (x0 - .4, Y(ranks[i][0]), 0), "right"), _room.place(self.name(i, size=14, width=1.55), (x1 + .3, Y(ranks[i][-1]), 0), "left"))""",
    """            names.add(_room.place(self.name(i, size=14, width=1.7, color=VARIANT_COLOR[i] if ranks[i][0] == 1 else INK), (x0 - .4, Y(ranks[i][0]), 0), "right"),
                      _room.place(self.name(i, size=14, width=1.55, color=VARIANT_COLOR[i] if ranks[i][-1] == 1 else INK), (x1 + .3, Y(ranks[i][-1]), 0), "left"))""")
# record: names stacked on one baseline
sub(3, """self.name(i, size=13, width=1.6).move_to([x, .4 if k % 2 == 0 else -.1, 0]))""", """block("\\n".join(stack(by_id(i)["title"][self.language()])), size=13, width=1.3).move_to([x, .3, 0]))""")
for old, new in [("self.thumbnail(by_id(i)[\"design\"], (x, .95, 0), .11, ramp=self._ramp, value=self._ramp.low), block(", "self.thumbnail(by_id(i)[\"design\"], (x, 1.0, 0), .11, ramp=self._ramp, value=self._ramp.low), block(")]:
    sub(3, old, new)
sub(3, """parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), LEFT + .3, -.6))""", """parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), LEFT + .3, -.45))""")
sub(3, """parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, -1.0 - .4 * (k // 2)))""", """parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, -.9 - .4 * (k // 2)))""")
sub(3, """        parts.add(heading(self.tr("Result", "Ergebnis"), LEFT + .3, -1.8),
                  self.thumbnail(by_id(winner)["design"], (LEFT + 1.0, -2.2, 0), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), LEFT + 1.5, -2.2),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), LEFT + 4.3, -2.2))""",
    """        parts.add(heading(self.tr("Result", "Ergebnis"), LEFT + .3, -1.75),
                  self.thumbnail(by_id(winner)["design"], (LEFT + 1.0, -2.15, 0), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), LEFT + 1.5, -2.15),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), LEFT + 4.3, -2.15))""")
# range: electricity ruler of six designs
sub(3, """        span = max(ruler.values[i] for i in six) - min(ruler.values[i] for i in six)
        self.step("range_2", ruler.stretch(2.5, span=span), frames["old"].level.animate(run_time=1.2).set_value(0.0), frames["new"].level.animate(run_time=1.2).set_value(1.0), mn.FadeOut(hint), seconds=2.5)
        self.ruler = ruler""", """        span = max(ruler.values[i] for i in six) - min(ruler.values[i] for i in six)
        elec = Ruler(self, "electricity", six, six, state="normalized", name_size=13)
        elec_header = self.ruler_header(elec)
        self.add(elec.ticks)
        to_elec = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(ghosts), mn.FadeOut(header),
                                    *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("slot", "dot", "leader", "tag", "norm")], *[frame.level.animate(run_time=.5).set_value(0.0) for frame in frames.values()],
                                    mn.FadeIn(elec.line), mn.FadeIn(elec_header), *[mn.FadeIn(elec.members[i][k]) for i in six for k in ("slot", "dot", "leader", "norm")],
                                    self.active.animate(run_time=.3).set_value(0))
        self.step("range_2", mn.Succession(mn.AnimationGroup(ruler.stretch(2.5, span=span), frames["old"].level.animate(run_time=1.2).set_value(0.0), frames["new"].level.animate(run_time=1.2).set_value(1.0), mn.FadeOut(hint)),
                                           mn.Wait(.5), to_elec), seconds=6.0)
        ruler, header = elec, elec_header
        self.ruler = ruler""")
sub(3, """        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(ghosts), mn.FadeOut(header), Retire(block), mn.FadeOut(ruler.members[extra]["slot"]),
                                *[frame.level.animate(run_time=.5).set_value(0.0) for frame in frames.values()],
                                *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag", "norm")], self.active.animate(run_time=.3).set_value(0))""",
    """        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(header), Retire(block), mn.FadeOut(ruler.members[extra]["slot"]),
                                *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag", "norm")], self.active.animate(run_time=.3).set_value(0))""")
sub(3, """        link = None
        if len(flipped) >= 2:
            ya, yb = (board.y0 - (new_ranks[i] - 1) * board.pitch for i in flipped[:2])
            link = mn.DoubleArrow([-.55, ya, 0], [-.55, yb, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)
""", """        link = None
""")
sub(3, """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.54, y0=2.2, name_size=13)""", """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.54, y0=2.2, name_size=13)""")
# next: both seasons visible
sub(3, """        self.step("next", mn.FadeIn(title), mn.FadeIn(table_), mn.FadeIn(self.block_electricity()), self.active.animate(run_time=.3).set_value(0), seconds=3.0)""",
    """        loss = self.heat_loss_arrows()
        legend = mn.VGroup(_room.place(_room.text([self.tr("winter heat loss", "Wärmeverlust im Winter"), self.n(_room.result(by_id(winner)["design"])["heating_demand"]), "kWh/a"], size=14, color=_room.HEAT), (2.15, 2.75, 0), "left"),
                           _room.place(_room.text([self.tr("summer cooling load", "Kühllast im Sommer"), self.n(_room.result(by_id(winner)["design"])["cooling_demand"]), "kWh/a"], size=14, color=_room.COOL), (2.15, 2.45, 0), "left"))
        self.step("next", mn.FadeIn(title), mn.FadeIn(table_), mn.FadeIn(self.block_electricity()), mn.FadeIn(loss), mn.FadeIn(legend), self.active.animate(run_time=.3).set_value(0), seconds=3.0)""")

# part 2: facade heat loss arrows; winter section uses them
sub(2, """    def swap_season(self, season, run_time=.6):""", """    def heat_loss_arrows(self):
        \"\"\"🔴 Winter heat loss: red arrows leave through the opaque wall below the window and through the glass, weight and length bound to the heating demand.\"\"\"
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            share = found["heating_demand"] / _room.reference()["heating_demand"]
            window = min(found["window_area"] / WIDTH, HEAD - .05)
            sill, x_o = HEAD - window, -(MASONRY + design["insulation"])
            group = mn.VGroup()
            for y in (sill * .3, sill * .65, (sill + HEAD) / 2):
                start = P(.3, y)
                group.add(mn.Arrow(start, np.array([P(x_o, 0)[0] - .12 - .5 * share, start[1], 0.0]), buff=0, color=_room.HEAT, stroke_width=3 + 3 * share, max_tip_length_to_length_ratio=.25,
                                   tip_length=.12, max_stroke_width_to_length_ratio=12))
            return group
        return live(build, _room.design_key(rt))

    def swap_season(self, season, run_time=.6):""")
sub(2, """            self.winter = mn.VGroup(self.room_section(self._rt, SECTION_POINT, ROOM_SCALE, "winter", labels=False, demand=False), self.demand_arrows("winter"))""",
    """            self.winter = mn.VGroup(self.room_section(self._rt, SECTION_POINT, ROOM_SCALE, "winter", labels=False, demand=False), self.heat_loss_arrows())""")

# stack helper (module level, part 1)
sub(1, """def block(value, point=(0, 0, 0), *, size=S0, color=INK, width=None):""", """def stack(value):
    \"\"\"📚 A name with one word per line (hyphenated words break after the hyphen).\"\"\"
    return re.findall(r"[^\\s-]+-?", value)


def block(value, point=(0, 0, 0), *, size=S0, color=INK, width=None):""")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
