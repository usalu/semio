

class Beat4_NormalizedIndex(_visuals.ClipScene):
    """📚 Present authored bilingual narration with model-bound animation."""

    TITLE = {"en": "Multiple Criteria and a Normalized Index", "de": "Mehrere Kriterien und ein normierter Index"}
    NARRATION = [
        ["variants", "Which of five façade designs suits this seminar room best?", "Welcher von fünf Fassadenentwürfen passt hier am besten?"],
        ["variants_2", "Only window share, overhang and insulation are compared.", "Verglichen werden nur Fensteranteil, Vordach und Dämmung."],
        ["criteria", "Electricity intensity and cost: lower is better.", "Strombedarf pro m² und Fassadenkosten: kleiner ist besser."],
        ["criteria_2", "Daylight factor: higher is better. Embodied carbon: lower is better.", "Tageslichtquotient: mehr ist besser. Graue Emissionen: kleiner ist besser."],
        ["best", "No design is best everywhere.", "Kein Entwurf ist überall der beste."],
        ["units", "Kilowatt-hours per m² and year plus francs? That sum has no unit.", "Kilowattstunden pro m² und Jahr plus Franken? Keine Einheit."],
        ["units_2", "Added anyway, the biggest numbers decide, not the importance.", "Trotzdem addiert, entscheiden die größten Zahlen, nicht die Wichtigkeit."],
        ["units_3", "Cost in kCHF, carbon in tonnes: the ranking changes.", "Kosten in kCHF, Graue Emissionen in Tonnen: Rangfolge ändert sich."],
        ["ruler", "Give each criterion its own ruler, starting with electricity.", "Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strombedarf pro m²."],
        ["ruler_2", "Each design sits at its real value; the room shows what the value means.", "Jeder Entwurf liegt auf seinem Wert; der Raum zeigt dessen Sinn."],
        ["ruler_3", "Sun becomes cooling demand; the chiller divides it by its efficiency.", "Sonne wird zum Kühlbedarf; die Kältemaschine teilt ihn durch EER."],
        ["ruler_4", "In winter the room loses heat; the pump needs less power than heat.", "Im Winter verliert der Raum Wärme; die Wärmepumpe braucht weniger Strom als Wärme."],
        ["ruler_5", "Lamps add the rest, less where daylight helps; the sum is the electricity.", "Lampen kommen dazu, weniger bei Tageslicht; Summe: der Strombedarf."],
        ["shift", "Step one: subtract the best of these five values.", "Schritt eins: Ziehe den besten dieser fünf Werte ab."],
        ["shift_2", "Step two: divide by the span from best to worst.", "Schritt zwei: Teile durch die Spanne vom besten zum schlechtesten Wert."],
        ["mapped", "Zero is the best of these five, one the worst of these five.", "Null ist die beste dieser fünf, eins die schlechteste dieser fünf."],
        ["cost", "Façade cost gets the same ruler: the cheapest façade becomes zero.", "Die Fassadenkosten erhalten dasselbe Lineal: Die günstigste Fassade wird null."],
        ["cost_2", "The price is the sum of its parts: fixed items, wall, insulation, windows and overhang.", "Der Preis ist die Summe seiner Teile: Fixposten, Wand, Dämmung, Fenster und Vordach."],
        ["daylight", "Daylight factor: more is better, so the ruler is mirrored.", "Tageslichtquotient: mehr ist besser, also wird das Lineal gespiegelt."],
        ["carbon", "Embodied carbon is the sum of the façade's materials, per layer.", "Graue Emissionen sind die Summe der Fassadenmaterialien, in Kilogramm je Schicht."],
        ["matrix", "Embodied carbon follows the same rule: its five values now lie between zero and one.", "Graue Emissionen folgen derselben Regel: ihre fünf Werte liegen nun zwischen null und eins."],
        ["weights", "Weights say how much each criterion matters; they add up to 100 %.", "Gewichte sagen, wie wichtig jedes Kriterium ist; zusammen 100 %."],
        ["weights_2", "Here: electricity intensity 40 %, cost 30 %, daylight factor 20 %, embodied carbon 10 %.", "Hier: Strombedarf pro m² 40 %, Kosten 30 %, Tageslichtquotient 20 %, Graue Emissionen 10 %."],
        ["weights_3", "A wider column means a stronger pull on the result.", "Eine breitere Spalte zieht das Ergebnis stärker an."],
        ["index", "Slide the filled parts together: their length is the index.", "Schiebe die gefüllten Teile zusammen: ihre Länge ist der Index."],
        ["index_2", "The shortest bar has the lowest index and ranks first.", "Der kürzeste Balken hat den kleinsten Index und steht vorn."],
        ["index_3", "Weight times normalized value, summed over four criteria: the index.", "Gewicht mal normierter Wert, über vier Kriterien summiert: der Index."],
        ["winner", "Its room: not best everywhere, but best overall.", "Sein Raum: nicht überall der beste, aber insgesamt der beste."],
        ["winner_2", "Strengths elsewhere outweigh its weakest criterion.", "Stärken anderswo gleichen sein schwächstes Kriterium aus."],
        ["sensitivity", "A cost-driven client puts more weight on cost.", "Ein kostenorientierter Bauherr gewichtet die Kosten stärker."],
        ["sensitivity_2", "The bars change and the ranking reorders.", "Die Balken ändern sich, und die Rangfolge ordnet sich neu."],
        ["sensitivity_3", "Another façade wins, though no design changed.", "Eine andere Fassade gewinnt, obwohl sich kein Entwurf geändert hat."],
        ["sweep", "Sweep the cost weight to see where the winner switches.", "Verschiebe das Kostengewicht und sieh, wo der Sieger wechselt."],
        ["sweep_2", "Close calls are fragile: test the ranking before you decide.", "Knappe Entscheidungen sind fragil: Rangfolge vorher prüfen."],
        ["range", "The rulers depend on the designs compared: add a sixth façade.", "Die Lineale hängen von der Vergleichsmenge ab: Füge eine sechste Fassade hinzu."],
        ["range_2", "Premium is dearest and most frugal: costs shrink, electricity values shift a little.", "Premium ist am teuersten und sparsamsten: Kosten schrumpfen, Stromwerte verschieben sich."],
        ["range_3", "The order of the original five can flip, though they did not change.", "Die Reihenfolge der ersten fünf kippt, obwohl sie gleich blieben."],
        ["record", "Record weights, designs and value ranges with every decision.", "Dokumentiere Gewichte, verglichene Entwürfe und Wertebereiche bei jeder Entscheidung."],
        ["limits", "Client budget 21,000 CHF: a mandatory limit, applied before weights.", "Das Budget des Bauherrn beträgt 21.000 CHF: eine zwingende Grenze, vor jeder Gewichtung."],
        ["limits_2", "Within the budget, strengths still offset weaknesses; the limit itself is never traded off.", "Im Budget gleichen Stärken noch Schwächen aus; die Grenze selbst wird nie verrechnet."],
        ["pointer", "To see trade-offs before weighting, look at Pareto fronts.", "Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten."],
        ["next", "For this room at these weights, the winner has the lowest index.", "Für diesen Raum gewinnt bei diesen Gewichten der kleinste Index."]
    ]

    def construct(self):
        """▶️ Present this independent architecture optimization chapter."""
        self.present("index")

    def prepare(self):
        """🧰 Clip-wide colour ramp, weight sets, the shared design trackers and the persistent room panel."""
        every = [_room.result(item["design"])["electricity"] / FLOOR_AREA for item in variants(True)]
        self._ramp = self.ramp(min(every), max(every))
        study = _model.load_study()["weights"]
        self._weights = tuple(study["balanced"][c] for c in ORDER)
        self._heavy = tuple(_model.reweigh(study["balanced"], "cost", .6)[c] for c in ORDER)
        self._maximum = max(_room.result(v["design"])["electricity"] for v in variants(True)) * 1.1
        self._rt = self.study_trackers(variants()[0]["design"])
        self.focus, self.active, self.count, self.detail, self.row_hi = mn.ValueTracker(0.0), mn.ValueTracker(-1.0), mn.ValueTracker(0.0), mn.ValueTracker(1.0), mn.ValueTracker(-1.0)
        self.roster_pos, self.roster_o = mn.ValueTracker(0.0), mn.ValueTracker(0.0)
        self.panel = self.build_panel()
        self.strip = self.value_strip()
        self.footer = _room.place(_room.text(self.tr("Simplified annual teaching model · degree-hours method · EN ISO 13790 style",
                                                      "Vereinfachtes Jahres-Lehrmodell · Gradstundenverfahren · Stil EN ISO 13790").split(), size=13, color=MUTED), (LEFT + .2, 3.68, 0), "left")
        self.season = "summer"

    def present_index(self):
        """🎞️ Cards, units, rulers, the weighted table, sensitivity, range dependence, the decision record and its limit."""
        self.prepare()
        self.screen_cards()
        self.screen_units()
        self.screen_rulers()
        self.screen_board()
        self.screen_sweep()
        self.screen_range()
        self.screen_record()

    def clear_stage(self):
        """🧹 Fade out everything but the caption and the persistent room panel; live widgets go through static twins."""
        kept = (self._caption, getattr(self, "panel", None), getattr(self, "strip", None), getattr(self, "footer", None), getattr(self, getattr(self, "season", "summer"), None))
        content = [item for item in self.mobjects if not any(item is other for other in kept)]
        if content:
            self.play(*[Retire(item) if any(isinstance(member, _room.LiveGroup) for member in item.get_family()) else mn.FadeOut(item) for item in content], run_time=.5)

    def n(self, value, digits=0):
        """🔢 A live number part of a label."""
        return _room.Number(self.number(value, digits))

    def words(self, value):
        """💬 The words of a phrase as label parts."""
        return str(value).split()

    def gap(self, size):
        """↔️ The space between two runs of a row."""
        return _room._glyphs()[1] * size / _room.REFERENCE_SIZE * 1.8

    def equation(self, items, x, y, *, size=14, edge="left", width=None):
        """🧾 One row of coloured runs on a common baseline: ``items`` are ``(parts, colour)``; ``x`` anchors the left, centre or right edge."""
        labels, cursor = [], 0.0
        for parts, colour in items:
            label = _room.row(size, colour, *parts)
            label.shift(np.array([cursor - label.get_left()[0], 0.0, 0.0]))
            labels.append(label)
            cursor = label.get_right()[0] + self.gap(size)
        span = labels[-1].get_right()[0] - labels[0].get_left()[0]
        start = {"left": x, "center": x - span / 2, "right": x - span}[edge]
        group = mn.VGroup(*labels)
        group.shift(np.array([start - labels[0].get_left()[0], y, 0.0]))
        if width is not None and span > width:
            group.scale(width / span, about_point=np.array([start, y, 0.0]) if edge == "left" else None)
        return group

    def name(self, identifier, size=S0, width=1.5, color=INK, limit=14):
        """🏷️ The variant's model title in the scene language, on one or two lines."""
        return block(lines(by_id(identifier)["title"][self.language()], limit), size=size, color=color, width=width)

    def value_text(self, criterion, value, digits=None):
        """🔢 A raw value with the model's digits for its criterion."""
        return self.number(value, DIGITS[criterion] if digits is None else digits)

    def unit_label(self, criterion):
        """🏷️ The unit of a criterion as the model gives it."""
        return CRITERIA[criterion]["unit"].replace("kWh/(m²·a)", _room.INTENSITY_UNIT)

    def focus_id(self):
        """📌 The variant the room panel currently shows."""
        return ids(True)[int(round(min(max(self.focus.get_value(), 0), len(ids(True)) - 1)))]

    def set_focus(self, identifier, run_time=1.0):
        """🎯 Move the room panel to a variant: the design trackers and the focus index move together."""
        return mn.AnimationGroup(self.set_design(self._rt, by_id(identifier)["design"], run_time), self.focus.animate(run_time=run_time).set_value(ids(True).index(identifier)))

    def build_panel(self):
        """🏠 The persistent room panel: title, section with sun, sky light, daylight zone and lamps, spec chips, overhang dimension and the heat arrows of the season."""
        rt, language = self._rt, self.language()
        key, focus_key = _room.design_key(rt), (lambda: round(self.focus.get_value()))
        title = live(lambda: mn.VGroup(_room.place(_room.text(self.words(by_id(self.focus_id())["title"][language]), size=21, color=INK), (6.95, 3.3, 0), "right")), focus_key)

        def specs():
            design = _room.values(rt)
            found = _room.result(design)
            chips = [_room.chip([self.tr("Window", "Fenster"), self.n(design["wwr"] * 100), "%", "·", self.n(found["window_area"], 1), "m²"], size=14, color=_room.GLASS),
                     _room.chip([self.tr("Overhang", "Vordach"), self.n(design["overhang"], 1), "m"], size=14, color=WARN),
                     _room.chip([self.tr("Insulation", "Dämmung"), self.n(design["insulation"] * 100), "cm"], size=14, color=_room.INSULATION)]
            group = mn.VGroup()
            for k, chip in enumerate(chips):
                group.add(_room.place(chip, (6.95, 2.8 - .42 * k, 0), "right"))
            return group

        def dimension():
            design = _room.values(rt)
            depth, thickness = design["overhang"], design["insulation"]
            x_o = -(MASONRY + thickness)
            a, b = P(x_o - depth, HEIGHT + SLAB + .35), P(x_o, HEIGHT + SLAB + .35)
            if depth < .01:
                label = _room.text(self.words(self.tr("no overhang", "kein Vordach")), P(2.4, HEIGHT + SLAB + .6), size=14, color=WARN)
                return mn.VGroup(label, mn.Line(label.get_left() + np.array([-.05, 0, 0]), P(x_o + .05, HEIGHT + SLAB + .03), color=WARN, stroke_width=1.4, stroke_opacity=.8)).set_z_index(8)
            return mn.VGroup(_room._dimension(a, b, color=WARN, tick=.07), _room.chip(["d", self.n(depth, 1), "m"], (a + b) / 2 + np.array([0, .3, 0]), size=14, color=WARN)).set_z_index(8)

        section = self.room_section(rt, SECTION_POINT, ROOM_SCALE, "summer", labels=False, demand=False)
        self.summer = mn.VGroup(section, self.demand_arrows("summer"))
        return mn.VGroup(live(dimension, key), live(specs, key), title)

    def demand_arrows(self, season):
        """➡️ Heat arrows leaving through the back wall: red for the winter heating demand, blue for the summer cooling demand; length and weight follow the model."""
        rt, heating = self._rt, season == "winter"

        def build():
            found = _room.result(_room.values(rt))
            key = "heating_demand" if heating else "cooling_demand"
            share = found[key] / _room.reference()[key]
            wall = P(DEPTH + MASONRY, 0)[0]
            group = mn.VGroup()
            for y in (1.1, 1.6, 2.1):
                start = P(DEPTH - .5, y)
                group.add(mn.Arrow(start, np.array([wall + .12 + .36 * share, start[1], 0.0]), buff=0, color=_room.HEAT if heating else _room.COOL, stroke_width=3 + 3 * share,
                                   max_tip_length_to_length_ratio=.35, tip_length=.12, max_stroke_width_to_length_ratio=12))
            return group
        return live(build, _room.design_key(rt))

    def heat_loss_arrows(self):
        """🔴 Winter heat loss: red arrows leave through the opaque wall below the window and through the glass, weight and length bound to the heating demand."""
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

    def swap_season(self, season, run_time=.6):
        """🍂 Cross-fade the room panel's section between its summer and winter picture."""
        if season == self.season:
            return mn.AnimationGroup()
        if season == "winter":
            self.winter = mn.VGroup(self.room_section(self._rt, SECTION_POINT, ROOM_SCALE, "winter", labels=False, demand=False), self.heat_loss_arrows())
        old, new = (self.summer, self.winter) if season == "winter" else (self.winter, self.summer)
        self.season = season
        return mn.AnimationGroup(Retire(old, run_time=run_time), mn.FadeIn(new, run_time=run_time))

    def cost_raws(self, design, found):
        """💰 The five cost parts in CHF: fixed items, wall, insulation, windows, overhang."""
        price = lambda name: _room.constant("prices", name)
        return [price("fixed"), found["wall_area"] * price("wall"), found["wall_area"] * design["insulation"] * 100 * price("insulation_per_cm"), found["window_area"] * price("window"),
                WIDTH * design["overhang"] * price("overhang")]

    def carbon_raws(self, design, found):
        """🏭 The four carbon layers in kg CO₂e: masonry, insulation, glass, overhang."""
        factor = lambda name: _room.constant("carbon", name)
        return [found["wall_area"] * factor("wall"), found["wall_area"] * design["insulation"] * 100 * factor("insulation_per_cm"), found["window_area"] * factor("window"),
                WIDTH * design["overhang"] * factor("overhang")]

    def value_strip(self):
        """🔢 The four raw values of the room on show, each with its pictogram and unit; the active criterion is framed."""
        rt = self._rt

        def build():
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
                    group.add(mn.Rectangle(width=2.35, height=.42, color=COLORS[c], stroke_width=2.4).move_to([x + 1.12, y - .03, 0]))
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.count.get_value()), round(self.active.get_value())))

    def materials_overlay(self, layers):
        """🧱 Tint the façade layers of the room section: insulation green, masonry cream, glass cyan and the overhang slab orange, each in its layer colour."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            depth, thickness = max(design["overhang"], 0.0), design["insulation"]
            window = min(found["window_area"] / WIDTH, HEAD - .05)
            sill, x_o = HEAD - window, -(MASONRY + thickness)
            boxes = {"insulation": [(x_o, 0, -MASONRY, sill), (x_o, HEAD, -MASONRY, HEIGHT)], "wall": [(-MASONRY, 0, 0, sill), (-MASONRY, HEAD, 0, HEIGHT)],
                     "window": [(x_o, sill, x_o + .2, HEAD)], "overhang": [(x_o - depth, HEIGHT, x_o, HEIGHT + SLAB)] if depth > .01 else []}
            group = mn.VGroup()
            for layer in layers:
                for x0, y0, x1, y1 in boxes[layer]:
                    a, b = P(x0, y0), P(x1, y1)
                    group.add(mn.Rectangle(width=max(b[0] - a[0], 1e-3), height=max(b[1] - a[1], 1e-3), color=LAYER[layer], stroke_width=3, fill_color=LAYER[layer],
                                           fill_opacity=.5).move_to((a + b) / 2))
            return group
        return live(build, _room.design_key(rt))

    def marker(self, colour, y):
        """⏹️ A small square in front of a row of a build-up."""
        return mn.Square(.14, stroke_width=0, fill_color=colour, fill_opacity=.9).move_to([LEFT + .12, y, 0])

    def lighting_row(self, found, digits, shown, y):
        """💡 Installed power · hours · share of the hours the lamps burn = lighting electricity, with the share printed as precisely as the result needs."""
        power = _room.constant("lighting", "power_density") * FLOOR_AREA
        hours = _room.constant("lighting", "hours")
        on = 100 * (1 - found["autonomy"])
        precision, sign = reproduce(lambda value: power * hours / 1000 * value / 100, [on], shown, digits)
        items = [([self.tr("Lighting", "Beleuchtung")], _room.LIGHT),
                 ([self.n(power), "W", "·", self.n(hours), "h/a", "·", *self.words(self.tr("lamps on", "Lampen an")), self.n(on, precision), "%", sign], INK),
                 ([self.n(shown, digits), "kWh/a"], _room.LIGHT)]
        return self.equation(items, LEFT + .4, y - .07, size=14, width=6.6)

    def bar_row(self, group, parts, total_parts, unit, colour_of):
        """📊 A horizontal stacked bar of the parts (length ∝ value) with the total in front of its end."""
        cursor = LEFT + .4
        for value, colour in zip(parts, colour_of):
            width = max(value * unit, 1e-3)
            group.add(mn.Rectangle(width=width, height=.22, stroke_width=0, fill_color=colour, fill_opacity=.88).move_to([cursor + width / 2, -.62, 0]))
            cursor += width
        group.add(self.equation([(total_parts, INK)], cursor + .15, -.62 - .07, size=15))

    def highlight(self, group, k, y):
        """🔦 Frame row ``k`` of a build-up while the clause talks about it."""
        if int(round(self.row_hi.get_value())) == k:
            group.add(mn.Rectangle(width=6.8, height=.31, color=INK, stroke_width=2, stroke_opacity=.8).move_to([(LEFT + RIGHT) / 2, y, 0]))

    def block_electricity(self):
        """⚡ Heating, cooling and lighting electricity of the room on show: stacked bar and the arithmetic behind every part, down to kWh/(m²·a)."""
        rt = self._rt

        def build():
            found = _room.result(_room.values(rt))
            digits = energy_digits(found)
            shown = _room.apportion([found[key] for key, _ in ENERGY], digits)
            total = round(sum(shown), digits)
            group, detail = mn.VGroup(), self.detail.get_value() > .5
            self.bar_row(group, [found[key] for key, _ in ENERGY], [self.n(total, digits), "kWh/a"], 4.4 / 760, [colour for _, colour in ENERGY])
            ys = [-1.0 - .32 * k for k in range(5)]
            names = (self.tr("Heating", "Heizung"), self.tr("Cooling", "Kühlung"))
            if detail:
                for k, (key, colour) in enumerate(ENERGY[:2]):
                    factor = _room.systems()["cop" if k == 0 else "eer"]
                    precision, sign = reproduce(lambda value: value / factor, [found[key + "_demand"]], shown[k], digits)
                    items = [([names[k]], colour), ([self.n(found[key + "_demand"], precision), "kWh/a", *self.words(self.tr("heat" if k == 0 else "cold", "Wärme" if k == 0 else "Kälte")), "÷", "COP" if k == 0 else "EER",
                                                     self.n(factor, 1), sign], INK), ([self.n(shown[k], digits), "kWh/a"], colour)]
                    group.add(self.marker(colour, ys[k]), self.equation(items, LEFT + .4, ys[k] - .07, size=14, width=6.6))
                group.add(self.marker(_room.LIGHT, ys[2]), self.lighting_row(found, digits, shown[2], ys[2]))
                for k in range(3):
                    self.highlight(group, k, ys[k])
            sums = [([self.tr("Total", "Summe")], INK)] + [item for k, (_, colour) in enumerate(ENERGY) for item in (([self.n(shown[k], digits)], colour), (["+" if k < 2 else "="], INK))]
            group.add(self.equation(sums + [([self.n(total, digits), "kWh/a"], INK)], LEFT + .4, ys[3] - .07, size=14, width=6.6))
            group.add(self.equation([([self.n(total, digits), "kWh/a", "÷", self.n(FLOOR_AREA), "m²", "="], INK), ([self.n(found["intensity"], 2), _room.INTENSITY_UNIT], COLORS["electricity"])],
                                    LEFT + .4, ys[4] - .07, size=14, width=6.6))
            for k in (3, 4):
                self.highlight(group, k, ys[k])
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.detail.get_value()), round(self.row_hi.get_value())))

    def block_cost(self):
        """💰 The façade price of the room on show as its parts: fixed items, wall, insulation, windows and overhang, each with area × price."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            price = lambda name: _room.constant("prices", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = self.cost_raws(design, found)
            shown = _room.apportion(raws, 0)
            colours = [LAYER[k] for k in ("fixed", "wall", "insulation", "window", "overhang")]
            group = mn.VGroup()
            self.bar_row(group, raws, [self.n(sum(shown)), "CHF"], 4.6 / 22500, colours)
            cost = lambda name, rest: ([self.tr(*name)], colours[("fixed", "wall", "insulation", "window", "overhang").index(name[2])])
            rows = [(("Fixed items", "Fixposten", "fixed"), []),
                    (("Wall", "Wand", "wall"), [self.n(wall, 1), "m²", "×", self.n(price("wall")), "CHF/m²"]),
                    (("Insulation", "Dämmung", "insulation"), [self.n(wall, 1), "m²", "×", self.n(thickness), "cm", "×", self.n(price("insulation_per_cm")), "CHF/m²/cm"]),
                    (("Windows", "Fenster", "window"), [self.n(window, 1), "m²", "×", self.n(price("window")), "CHF/m²"]),
                    (("Overhang", "Vordach", "overhang"), [self.n(WIDTH), "m", "×", self.n(depth, 1), "m", "×", self.n(price("overhang")), "CHF/m²"])]
            for k, ((en, de, layer), formula) in enumerate(rows):
                y = -1.0 - .32 * k
                items = [([self.tr(en, de)], LAYER[layer])] + ([(formula + ["="], INK)] if formula else []) + [([self.n(shown[k]), "CHF"], LAYER[layer])]
                group.add(self.marker(LAYER[layer], y), self.equation(items, LEFT + .4, y - .07, size=14, width=6.6))
                self.highlight(group, k, y)
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.row_hi.get_value())))

    def block_carbon(self):
        """🏭 The embodied carbon of the room on show as material layers: masonry, insulation, glass and the concrete overhang, in kg CO₂e per layer."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            factor = lambda name: _room.constant("carbon", name)
            wall, window, thickness, depth = found["wall_area"], found["window_area"], design["insulation"] * 100, design["overhang"]
            raws = self.carbon_raws(design, found)
            shown = _room.apportion(raws, 0)
            layers = ("wall", "insulation", "window", "overhang")
            group = mn.VGroup()
            self.bar_row(group, raws, [self.n(sum(shown)), "kg", "CO₂e"], 4.6 / 5600, [LAYER[k] for k in layers])
            rows = [(("Masonry", "Mauerwerk"), [self.n(wall, 1), "m²", "×", self.n(factor("wall")), "kg", "CO₂e/m²"]),
                    (("Insulation", "Dämmung"), [self.n(wall, 1), "m²", "×", self.n(thickness), "cm", "×", self.n(factor("insulation_per_cm"), 1), "kg", "CO₂e/m²/cm"]),
                    (("Glass", "Verglasung"), [self.n(window, 1), "m²", "×", self.n(factor("window")), "kg", "CO₂e/m²"]),
                    (("Concrete overhang", "Vordach aus Beton"), [self.n(WIDTH), "m", "×", self.n(depth, 1), "m", "×", self.n(factor("overhang")), "kg", "CO₂e/m²"])]
            for k, ((en, de), formula) in enumerate(rows):
                y = -1.0 - .32 * k
                items = [([self.tr(en, de)], LAYER[layers[k]]), (formula + ["="], INK), ([self.n(shown[k]), "kg", "CO₂e"], LAYER[layers[k]])]
                group.add(self.marker(LAYER[layers[k]], y), self.equation(items, LEFT + .4, y - .07, size=14, width=6.6))
                self.highlight(group, k, y)
            return group
        return live(build, lambda: (_room.design_key(rt)(), round(self.row_hi.get_value())))

    def block_daylight(self):
        """☀️ The daylight factor of the room on show: gauge against the 2 % daylit limit, the sky angle behind the overhang, the substituted formula and the lamps it saves."""
        rt = self._rt

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            const = lambda name: _room.constant("daylight", name)
            depth = design["overhang"]
            theta = const("sky_angle") * (1 - const("sky_angle_loss") * depth ** 2 / (depth ** 2 + const("overhang_half_depth") ** 2))
            factor = found["daylight"]
            group, x0, scale = mn.VGroup(), LEFT + .4, 4.8 / 6
            group.add(mn.Rectangle(width=max(factor * scale, 1e-3), height=.22, stroke_width=0, fill_color=_room.LIGHT, fill_opacity=.88).move_to([x0 + factor * scale / 2, -.62, 0]),
                      mn.Line([x0, -.62 - .16, 0], [x0 + 6 * scale, -.62 - .16, 0], color=MUTED, stroke_width=1.4))
            for mark in (0, 2, 4, 6):
                group.add(mn.Line([x0 + mark * scale, -.62 - .16, 0], [x0 + mark * scale, -.62 - .22, 0], color=MUTED, stroke_width=1.4),
                          _room.text([self.n(mark), "%"], (x0 + mark * scale, -.62 - .36, 0), size=13, color=MUTED))
            group.add(_room.dashed([x0 + 2 * scale, -.62 + .15, 0], [x0 + 2 * scale, -.62 - .16, 0], color=INK, stroke_width=2),
                      self.equation([([self.n(factor, 2), "%"], _room.LIGHT)], x0 + 6 * scale + .2, -.62 - .07, size=15))
            ys = [-1.4 - .32 * k for k in range(4)]
            sky = [([self.tr("Sky angle", "Himmelswinkel"), "θ", "=", self.n(theta, 1), "~°"], _room.LIGHT)]
            sky.append(([*self.words(self.tr("with the overhang; 75° without", "mit Vordach; 75° ohne"))], MUTED) if depth > .01 else ([*self.words(self.tr("no overhang: full sky view", "kein Vordach: voller Himmel"))], MUTED))
            group.add(self.marker(_room.LIGHT, ys[0]), self.equation(sky, LEFT + .4, ys[0] - .07, size=14, width=6.6))
            area = round(found["window_area"], 1)
            precision, sign = reproduce(lambda angle: const("transmittance") * area * angle / (const("surface_area") * (1 - const("reflectance") ** 2)), [theta], factor, 2)
            group.add(self.marker(_room.LIGHT, ys[1]), self.equation([([self.tr("DF", "TQ"), "="], _room.LIGHT),
                                                                      ([self.n(const("transmittance"), 1), "·", self.n(area, 1), "m²", "·", self.n(theta, precision), "~°", "÷", "(" + self.number(const("surface_area")), "m²", "·",
                                                                        self.number(1 - const("reflectance") ** 2, 2) + ")", sign], INK), ([self.n(factor, 2), "%"], _room.LIGHT)], LEFT + .4, ys[1] - .07, size=14, width=6.6))
            reach = _room.daylit_depth(factor)
            group.add(self.marker(_room.DAYLIGHT, ys[2]), self.equation([([*self.words(self.tr("Daylit ≥ 2 % to", "Tageslichtversorgt ≥ 2 % bis")), self.n(reach, 1), "m", *self.words(self.tr("of 6 m (linear falloff)", "von 6 m (lineares Abklingen)"))], INK)],
                                                                      LEFT + .4, ys[2] - .07, size=14, width=6.6))
            digits = energy_digits(found)
            shown = _room.apportion([found[key] for key, _ in ENERGY], digits)
            group.add(self.marker(_room.LIGHT, ys[3]), self.lighting_row(found, digits, shown[2], ys[3]))
            return group
        return live(build, _room.design_key(rt))

    def index_terms(self, identifier, weights):
        """🧮 Weights, normalized values and the four weighted parts as printed: the parts add up to the printed index and every product holds as printed."""
        row = table(tuple(round(w, 6) for w in weights))[identifier]
        for digits in (2, 3, 4):
            printed_w = [round(w, digits) for w in weights]
            norm = [round(row["normalized"][c], digits) for c in ORDER]
            if round(sum(w * v for w, v in zip(printed_w, norm)), 2) == round(row["score"], 2) and abs(sum(printed_w) - 1) < 1e-9:
                break
        parts = _room.apportion([w * v for w, v in zip(printed_w, norm)], 2, total=row["score"])
        return printed_w, norm, parts, digits

    def block_index(self, getter, weights, *, full=False, y=-2.2):
        """⚖️ The index of one variant written out: ``w × n`` per criterion, the four parts and their sum; ``full`` adds the raw value behind every normalized value."""
        def build():
            ws = weights()
            identifier = getter()
            printed_w, norm, parts, digits = self.index_terms(identifier, ws)
            group = mn.VGroup()
            if not full:
                items = [([self.tr("Index", "Index"), "="], INK)]
                for k, c in enumerate(ORDER):
                    items += [([self.n(printed_w[k], digits), "×", self.n(norm[k], digits)], COLORS[c]), (["+" if k < 3 else "="], INK)]
                items.append(([self.n(sum(parts), 2)], GOOD))
                group.add(self.equation(items, LEFT + .1, y - .07, size=14, width=6.8))
                return group
            exact = table(tuple(round(w, 6) for w in weights()))[identifier]
            norm3 = [round(exact["normalized"][c], 3) for c in ORDER]
            terms = [round(w * v, 4) for w, v in zip(printed_w, norm3)]
            for k, c in enumerate(ORDER):
                yy = 2.45 - .55 * k
                items = [([self.n(raw(identifier, c), DIGITS[c]), *self.words(self.unit_label(c))], COLORS[c]), (["→"], MUTED),
                         ([self.tr("normalized", "normiert"), self.n(norm3[k], 3), "×", self.n(printed_w[k] * 100, digits - 2), "%", "="], INK), ([self.n(terms[k], 4)], COLORS[c])]
                group.add(_room.clone(glyph(c, width=.26)).move_to([LEFT + .15, yy, 0]), self.equation(items, LEFT + .45, yy - .07, size=14, width=6.4))
            group.add(self.equation([([self.tr("Index", "Index"), "="], INK), ([self.n(terms[0], 4), "+", self.n(terms[1], 4), "+", self.n(terms[2], 4), "+", self.n(terms[3], 4), "="], INK),
                                     ([self.n(sum(terms), 4)], GOOD)], LEFT + .45, 2.45 - .55 * 4 - .07, size=16, width=6.4))
            return group
        return live(build, lambda: (getter(), tuple(round(w, 4) for w in weights())))
