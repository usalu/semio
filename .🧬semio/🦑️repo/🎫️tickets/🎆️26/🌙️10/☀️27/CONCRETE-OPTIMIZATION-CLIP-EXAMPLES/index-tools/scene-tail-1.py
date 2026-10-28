class Beat4_NormalizedIndex(_visuals.ClipScene):
    """📚 Present authored bilingual narration with model-bound animation."""

    TITLE = {"en": "Multiple Criteria and a Normalized Index", "de": "Mehrere Kriterien und ein Normierter Index"}
    NARRATION = [
        ["variants", "Five façade variants for the same seminar room.", "Fünf Fassadenvarianten für denselben Seminarraum."],
        ["variants_2", "Only window share, overhang and insulation differ.", "Nur Fensteranteil, Vordach und Dämmung unterscheiden sich."],
        ["criteria", "Electricity and cost: lower is better.", "Strom und Kosten: weniger ist besser."],
        ["criteria_2", "Daylight: higher is better. Carbon: lower is better.", "Tageslicht: mehr ist besser. CO₂: weniger ist besser."],
        ["best", "No variant is best everywhere.", "Keine Variante ist überall die beste."],
        ["units", "Kilowatt-hours per square metre plus francs? That sum has no unit.", "Kilowattstunden pro Quadratmeter plus Franken? Die Summe hat keine Einheit."],
        ["units_2", "Added anyway, the biggest numbers decide, not the importance.", "Trotzdem addiert, entscheiden die größten Zahlen, nicht die Wichtigkeit."],
        ["units_3", "Count in thousands (kCHF, tonnes) instead and the ranking changes.", "Zähle stattdessen in Tausendern (kCHF, Tonnen), und die Rangfolge ändert sich."],
        ["ruler", "Give each criterion its own ruler, starting with electricity.", "Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strom."],
        ["ruler_2", "Each variant sits at its real value; the room shows what it means.", "Jede Variante liegt auf ihrem echten Wert; der Raum zeigt die Bedeutung."],
        ["shift", "Step one: subtract the best value of the set.", "Schritt eins: Ziehe den besten Wert der Menge ab."],
        ["shift_2", "Step two: divide by the span from best to worst.", "Schritt zwei: Teile durch die Spanne vom besten zum schlechtesten Wert."],
        ["mapped", "Zero is the best of these five, one the worst of these five.", "Null ist die beste dieser fünf, eins die schlechteste dieser fünf."],
        ["cost", "Cost gets the same ruler: the cheapest façade becomes zero.", "Die Kosten erhalten dasselbe Lineal: Die günstigste Fassade wird null."],
        ["daylight", "Daylight: more is better, so the ruler is flipped.", "Tageslicht: mehr ist besser, also wird das Lineal umgedreht."],
        ["matrix", "Carbon follows the same rule: four comparable values between zero and one.", "CO₂ folgt derselben Regel: vier vergleichbare Werte zwischen null und eins."],
        ["weights", "Weights say how much each criterion matters; together they make 100 %.", "Gewichte sagen, wie wichtig jedes Kriterium ist; zusammen ergeben sie 100 %."],
        ["weights_2", "Here: electricity 40 %, cost 30 %, daylight 20 %, carbon 10 %.", "Hier: Strom 40 %, Kosten 30 %, Tageslicht 20 %, CO₂ 10 %."],
        ["weights_3", "A wider column means a stronger pull on the result.", "Eine breitere Spalte zieht das Ergebnis stärker an."],
        ["index", "Slide the filled parts together: their length is the index.", "Schiebe die gefüllten Teile zusammen: Ihre Länge ist der Index."],
        ["index_2", "The shortest bar has the lowest index and ranks first.", "Der kürzeste Balken hat den kleinsten Index und steht vorn."],
        ["winner", "Its room: not best everywhere, but best overall.", "Sein Raum: nicht überall der beste, aber insgesamt der beste."],
        ["winner_2", "Strengths elsewhere outweigh its weakest criterion.", "Stärken anderswo gleichen sein schwächstes Kriterium aus."],
        ["sensitivity", "A cost-driven client puts more weight on cost.", "Ein kostenorientierter Bauherr gewichtet die Kosten stärker."],
        ["sensitivity_2", "The bars change and the ranking reorders.", "Die Balken ändern sich, und die Rangfolge ordnet sich neu."],
        ["sensitivity_3", "Another façade wins, though no variant changed.", "Eine andere Fassade gewinnt, obwohl sich keine Variante geändert hat."],
        ["sweep", "Sweep the cost weight to see where the winner switches.", "Verschiebe das Kostengewicht und sieh, wo der Sieger wechselt."],
        ["sweep_2", "Close calls are fragile: test the ranking before you decide.", "Knappe Entscheide sind fragil: Prüfe die Rangfolge, bevor du entscheidest."],
        ["range", "The rulers depend on the variants compared: add a sixth façade.", "Die Lineale hängen von der Vergleichsmenge ab: Füge eine sechste Fassade hinzu."],
        ["range_2", "Best and worst move, so all other normalized values shrink.", "Bester und schlechtester Wert verschieben sich, alle anderen Werte schrumpfen."],
        ["range_3", "The order of the original five can flip, though they did not change.", "Die Reihenfolge der ersten fünf kann kippen, obwohl sie gleich blieben."],
        ["record", "Document weights, compared variants and ranges with every decision.", "Dokumentiere Gewichte, Varianten und Wertebereiche bei jeder Entscheidung."],
        ["limits", "Strengths can offset weaknesses; mandatory limits stay constraints.", "Stärken gleichen Schwächen aus; zwingende Grenzen bleiben Nebenbedingungen."],
        ["next", "To see trade-offs before weighting, look at Pareto fronts.", "Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten."]
    ]

    def construct(self):
        """▶️ Present this independent architecture optimization chapter."""
        self.present("index")

    def prepare(self):
        """🧰 Clip-wide colour ramp, weight sets, panel scale and design trackers."""
        every = [_room.result(item["design"])["electricity"] / FLOOR_AREA for item in variants(True)]
        self._ramp = self.ramp(min(every), max(every))
        study = _model.load_study()["weights"]
        self._weights = tuple(study["balanced"][c] for c in ORDER)
        self._heavy = tuple(_model.reweigh(study["balanced"], "cost", .6)[c] for c in ORDER)
        self._maximum = max(_room.result(v["design"])["electricity"] for v in variants(True)) * 1.1
        self._rt = self.study_trackers(variants()[0]["design"])

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

    def name(self, identifier, size=S0, width=1.5, color=INK, limit=14):
        """🏷️ The variant's model title in the scene language, on one or two lines."""
        return block(lines(by_id(identifier)["title"][self.language()], limit), size=size, color=color, width=width)

    def value_text(self, criterion, value, digits=None):
        """🔢 A raw value with the model's digits for its criterion."""
        return self.number(value, DIGITS[criterion] if digits is None else digits)

    def unit_label(self, criterion):
        """🏷️ The unit of a criterion as the model gives it."""
        return CRITERIA[criterion]["unit"]

    def unit_note(self, criterion):
        """📝 The unit with what kind of quantity it counts: one-off cost, embodied carbon."""
        notes = {"cost": ("CHF · one-off", "CHF · einmalig"), "carbon": ("kg CO₂e · embodied", "kg CO₂e · grau")}
        return self.tr(*notes[criterion]) if criterion in notes else self.unit_label(criterion)

    def spec_line(self, design, point=(0, 0, 0), *, size=S0, color=MUTED, width=None):
        """📐 Window share, overhang depth and insulation thickness of a design in one label."""
        number = lambda value, digits=0: _room.Number(self.number(value, digits))
        return _room.text([self.tr("Window", "Fenster"), number(design["wwr"] * 100), "%", "·", self.tr("overhang", "Vordach"), number(design["overhang"], 1), "m", "·",
                           self.tr("insulation", "Dämmung"), number(design["insulation"] * 100), "cm"], point, size=size, color=color, width=width)

    def slot_frame(self, ruler, identifier, color):
        """🔲 A frame that follows a thumbnail on the ruler; its opacity is the ``level`` tracker."""
        frame = mn.Rectangle(width=.95, height=.78, color=color, stroke_width=2.4)
        frame.level = mn.ValueTracker(0.0)
        frame.add_updater(lambda m: (m.move_to(ruler.members[identifier]["slot"][0].get_center()), m.set_stroke(opacity=m.level.get_value())))
        frame.update()
        return frame

    def variant_panel(self, rt, getter, *, color=GOOD, x=5.25):
        """🏠 The room of the variant ``getter()`` names: title, section, window / overhang / insulation line and the four raw values."""
        title = live(lambda: mn.VGroup(_room.text(by_id(getter())["title"][self.language()].split(), (x, 2.4, 0), size=S1, width=3.0, color=color)), getter)
        section = self.room_section(rt, (x, 1.2, 0), .27, "summer", labels=False, demand=False)
        spec = live(lambda: mn.VGroup(self.spec_line(by_id(getter())["design"], (x, .12, 0), width=3.0)), getter)

        def values():
            rows = mn.VGroup()
            for k, c in enumerate(ORDER):
                label = _room.text([_room.Number(self.number(raw(getter(), c), DIGITS[c])), *self.unit_label(c).split()], size=S0, color=COLORS[c])
                rows.add(mn.VGroup(_room.clone(glyph(c, width=.28)).move_to([4.0, -.3 - .4 * k, 0]), _room.place(label, (4.3, -.3 - .4 * k, 0), "left")))
            return rows
        return mn.VGroup(title, section, spec, live(values, getter))

    def elevation(self, rt, point, scale=.4):
        """🧱 The 8 × 3 m façade in elevation: punched windows as separate openings, a ribbon or glass wall as one band, the overhang edge only when it exists, and the model's summer shadow."""
        s = scale
        origin = np.array(point, dtype=float) - s * np.array([4.0, 1.5, 0.0])
        P = lambda x, y: origin + s * np.array([x, y, 0.0])

        def rect(x0, y0, x1, y1, **style):
            return mn.Rectangle(width=s * (x1 - x0), height=s * (y1 - y0), **style).move_to(P((x0 + x1) / 2, (y0 + y1) / 2))

        def build():
            design = _room.values(rt)
            found = _room.result(design)
            area, depth, shade = found["window_area"], max(design["overhang"], 0.0), found["shading_summer"]
            group = mn.VGroup(rect(0, 0, 8, 3, color=INK, stroke_width=2, fill_color="#1c2b2d", fill_opacity=.9))
            if design["wwr"] <= .3:
                high = area / 4 / 1.5
                boxes = [(.4 + 1.9 * k, 2.7 - high, 1.9 + 1.9 * k, 2.7) for k in range(4)]
            else:
                high = min(area / 8, 2.65)
                boxes = [(0, 2.7 - high, 8, 2.7)]
            for x0, y0, x1, y1 in boxes:
                group.add(rect(x0, y0, x1, y1, color=INK, stroke_width=1.6, fill_color=_room.GLASS, fill_opacity=.55))
                if shade > .01:
                    group.add(rect(x0, y1 - shade * (y1 - y0), x1, y1, stroke_width=0, fill_color="#000000", fill_opacity=.55))
            if len(boxes) == 1:
                group.add(*[mn.Line(P(x, boxes[0][1]), P(x, boxes[0][3]), color=INK, stroke_width=1.4) for x in (2, 4, 6)])
            if depth > .01:
                group.add(mn.Line(P(-.15, 3.0), P(8.15, 3.0), color=WARN, stroke_width=5))
            group.add(mn.DoubleArrow(P(0, -.4), P(8, -.4), buff=0, color=MUTED, stroke_width=1.6, tip_length=.08, max_tip_length_to_length_ratio=.2),
                      _room.text(["8", "m"], P(4, -.85), size=14, color=MUTED),
                      mn.DoubleArrow(P(-.4, 0), P(-.4, 3), buff=0, color=MUTED, stroke_width=1.6, tip_length=.08, max_tip_length_to_length_ratio=.2),
                      _room.text(["3", "m"], P(-1.0, 1.5), size=14, color=MUTED))
            number = lambda value, digits=0: _room.Number(self.number(value, digits))
            group.add(_room.chip([self.tr("Window", "Fenster"), number(design["wwr"] * 100), "%", "·", number(area, 1), "m²"], P(1.5, 3.9), size=S0, color=_room.GLASS),
                      _room.chip([self.tr("Overhang", "Vordach"), number(depth, 1), "m"] if depth > .01 else self.tr("no overhang", "kein Vordach").split(), P(6.6, 3.9), size=S0, color=WARN))
            return group
        return self._live(build, _room.design_key(rt))

    def screen_cards(self):
        """🪪 Five variant cards: the room chip, big section pictograms that shrink to make room for the spec lines, raw values and best frames."""
        pool = variants()
        row_y = {"electricity": -.95, "cost": -1.42, "daylight": -1.89, "carbon": -2.36}
        room = _art.window_chip(text(self.tr("Seminar room 8 × 6 × 3 m · 48 m² · Zurich · simplified annual teaching model", "Seminarraum 8 × 6 × 3 m · 48 m² · Zürich · vereinfachtes Jahresmodell"),
                                     size=S0, color=MUTED), color=MUTED).move_to([0, 2.3, 0])
        cards = []
        for index, item in enumerate(pool):
            x = -5.4 + 2.7 * index
            frame = _art.window_frame(2.5, 4.55, color=MUTED).move_to([x, -.28, 0])
            title = self.name(item["id"], size=S1, width=2.2, limit=16).move_to([x, 1.55, 0])
            thumb = self.thumbnail(item["design"], (x, .45, 0), .4, ramp=self._ramp, value=self._ramp.low)
            design = item["design"]
            spec = mn.VGroup(*[text(label, (x, y, 0), size=S0, color=MUTED) for label, y in (
                (self.tr(f"Window {self.number(design['wwr'] * 100)} %", f"Fenster {self.number(design['wwr'] * 100)} %"), -.02),
                (self.tr(f"Overhang {self.number(design['overhang'], 1)} m", f"Vordach {self.number(design['overhang'], 1)} m"), -.23),
                (self.tr(f"Insulation {self.number(design['insulation'] * 100)} cm", f"Dämmung {self.number(design['insulation'] * 100)} cm"), -.44))])
            rows = {}
            for criterion in ORDER:
                y = row_y[criterion]
                value = text(self.value_text(criterion, raw(item["id"], criterion)), (0, 0, 0), size=19)
                value.move_to([x - .6 + value.width / 2, y + .1, 0])
                unit = text(self.unit_note(criterion), (0, 0, 0), size=14, color=MUTED, width=1.5)
                unit.move_to([x - .6 + unit.width / 2, y - .13, 0])
                rows[criterion] = mn.VGroup(glyph(criterion, (x - .9, y, 0), .3), value, unit, arrow(criterion, (x + .98, y, 0), .28))
            cards.append({"id": item["id"], "x": x, "base": mn.VGroup(frame, title, thumb), "thumb": thumb, "spec": spec, "rows": rows})
        self.cards, self.room_chip = cards, room
        self.step("variants", mn.FadeIn(room), mn.LaggedStart(*[mn.FadeIn(card["base"]) for card in cards], lag_ratio=.15), seconds=2.0)
        self.step("variants_2", *[mn.FadeIn(card["spec"]) for card in cards], *[card["thumb"].animate.scale(.6).move_to([card["x"], .72, 0]) for card in cards], seconds=1.6)
        for key, group in (("criteria", ORDER[:2]), ("criteria_2", ORDER[2:])):
            self.step(key, *[mn.LaggedStart(*[mn.FadeIn(card["rows"][c], shift=mn.UP * .1) for card in cards], lag_ratio=.12) for c in group], seconds=1.8)
        frames = []
        for criterion in ORDER:
            winner = next(card for card in cards if card["id"] == best_id(criterion, ids()))
            frames.append(mn.Rectangle(width=2.3, height=.44, color=GOOD, stroke_width=3).move_to([winner["x"], row_y[criterion] - .02, 0]))
        self.step("best", mn.LaggedStart(*[mn.Create(f) for f in frames], lag_ratio=.4), seconds=2.4)
        self.best_frames = frames

    def sums(self, scale):
        """➕ The naive sum of the printed raw numbers with cost and carbon divided by a scale."""
        return {i: sum(raw(i, c) / (scale if c in ("cost", "carbon") else 1) for c in ORDER) for i in ids()}

    def naive_rows(self, scale):
        """📶 One row per variant: thumbnail, name, a bar of four coloured segments and the sum, sorted by the sum."""
        sums = self.sums(scale)
        order = sorted(ids(), key=sums.get)
        longest = max(sums.values())
        rows = {}
        for rank, identifier in enumerate(order):
            y = 1.55 - .72 * rank
            thumb = self.thumbnail(by_id(identifier)["design"], (-5.7, y, 0), .13, ramp=self._ramp, value=self._ramp.low)
            name = self.name(identifier, size=S0, width=1.5).move_to([-4.45, y, 0])
            cursor, segments = -3.2, []
            for c in ORDER:
                value = raw(identifier, c) / (scale if c in ("cost", "carbon") else 1)
                width = max(value / longest * 6.4, .012)
                segments.append(mn.Rectangle(width=width, height=.4, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85).move_to([cursor + width / 2, y, 0]))
                cursor += width
            total = text(self.number(sums[identifier], 0 if scale == 1 else 1), (cursor + .6, y, 0), size=S1)
            total.move_to([cursor + .12 + total.width / 2, y, 0])
            badge = mn.Circle(radius=.16, color=GOOD if rank == 0 else INK, stroke_width=2).move_to([-6.45, y, 0])
            number = text(str(rank + 1), badge.get_center(), size=S0, color=GOOD if rank == 0 else INK)
            rows[identifier] = mn.VGroup(thumb, name, *segments, total, badge, number)
        return rows, order

    def legend(self, scale):
        """🏷️ Four coloured pictograms with the unit each segment is counted in."""
        group = mn.VGroup()
        for c in ORDER:
            unit = self.unit_label(c)
            if scale != 1 and c == "cost":
                unit = "kCHF"
            if scale != 1 and c == "carbon":
                unit = "t CO₂e"
            item = mn.VGroup(glyph(c, width=.3), text(unit, size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.12)
            group.add(item)
        return group.arrange(mn.RIGHT, buff=.55).move_to([0, -2.28, 0])

    def screen_units(self):
        """➕ Why kWh/(m²·a) and CHF cannot be added, and why a naive sum depends on the unit."""
        first = self.cards[0]
        item = by_id(first["id"])
        chips = []
        for c in ("electricity", "cost"):
            content = mn.VGroup(glyph(c, width=.36), text(self.value_text(c, raw(first["id"], c)), size=S2 + 4), text(self.unit_label(c), size=S1, color=COLORS[c])).arrange(mn.RIGHT, buff=.18)
            chips.append(_art.window_chip(content, color=COLORS[c]))
        plus, equals, question = (text(s, size=S2 + 8) for s in ("+", "=", "?"))
        equation = mn.VGroup(chips[0], plus, chips[1], equals, question).arrange(mn.RIGHT, buff=.32).move_to([0, .3, 0])
        cross = _art.cross_mark(color=BAD, size=.3).next_to(question, mn.RIGHT, buff=.35)
        verdict = _art.window_chip(text(self.tr("no unit", "keine Einheit"), size=S2, color=BAD), color=BAD).move_to([0, -.9, 0])
        subject = mn.VGroup(self.thumbnail(item["design"], (0, 0, 0), .3, ramp=self._ramp, value=self._ramp.low),
                            self.name(first["id"], size=S1, width=3.4)).arrange(mn.RIGHT, buff=.3).move_to([0, 1.75, 0])
        self.step("units", *[mn.FadeOut(card["base"]) for card in self.cards], *[mn.FadeOut(card["spec"]) for card in self.cards], mn.FadeOut(self.room_chip),
                  *[mn.FadeOut(r) for card in self.cards for r in card["rows"].values()], *[mn.FadeOut(f) for f in self.best_frames],
                  *[mn.FadeIn(chip) for chip in chips], mn.FadeIn(subject), mn.FadeIn(plus), mn.FadeIn(equals), mn.FadeIn(question),
                  mn.Create(cross), mn.FadeIn(verdict), seconds=2.2)
        rows, order = self.naive_rows(1)
        legend = self.legend(1)
        head = text(self.tr("Sum of the printed numbers", "Summe der gedruckten Zahlen"), (0, 2.3, 0), size=S1, color=MUTED)
        unit_chip = _art.window_chip(text(self.tr("Units: CHF · kg", "Einheiten: CHF · kg"), size=S1, color=WARN), color=WARN).move_to([4.9, 2.3, 0])
        self.rows = rows
        self.step("units_2", mn.FadeOut(equation), mn.FadeOut(cross), mn.FadeOut(verdict), mn.FadeOut(subject),
                  *[mn.FadeIn(r) for r in rows.values()], mn.FadeIn(legend), mn.FadeIn(head), mn.FadeIn(unit_chip), seconds=2.2)
        after, order2 = self.naive_rows(1000)
        legend2 = self.legend(1000)
        head2 = text(self.tr("Still a sum of unlike units", "Immer noch eine Summe verschiedener Einheiten"), (0, 2.3, 0), size=S1, color=MUTED)
        unit_chip2 = _art.window_chip(text(self.tr("Units: kCHF · t", "Einheiten: kCHF · t"), size=S1, color=GOOD), color=GOOD).move_to([4.9, 2.3, 0])
        self.step("units_3", *[mn.ReplacementTransform(rows[i], after[i]) for i in ids()], mn.ReplacementTransform(legend, legend2),
                  mn.ReplacementTransform(unit_chip, unit_chip2), mn.ReplacementTransform(head, head2), seconds=2.8)
        self.naive_order = (order, order2)
