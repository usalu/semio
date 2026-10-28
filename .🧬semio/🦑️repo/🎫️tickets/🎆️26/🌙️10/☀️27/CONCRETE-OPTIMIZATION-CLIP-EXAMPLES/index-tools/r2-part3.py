
    def criterion_chip(self, criterion, x, bottom):
        """🔖 The head of a table column: pictogram, name, unit and direction in a chip whose bottom edge sits at ``bottom``."""
        chip = _art.window_chip(head_content(self, criterion), color=COLORS[criterion])
        return chip.move_to([x, bottom + chip.height / 2, 0])

    def screen_cards(self):
        """🪪 The five variants as rows of a table next to the room: a cursor walks through them, spec columns, then the four criteria columns and the best value of each."""
        pool = variants()
        Y = lambda k: 1.4 - .8 * k
        CX = [-3.9 + 1.18 * j for j in range(4)]
        CS = [-3.65 + 1.3 * j for j in range(3)]
        base = mn.VGroup()
        for k, item in enumerate(pool):
            base.add(self.thumbnail(item["design"], (-6.3, Y(k), 0), .15, ramp=self._ramp, value=self._ramp.low), self.name(item["id"], size=S0, width=1.5).move_to([-5.15, Y(k), 0]))
        cursor = mn.Rectangle(width=FRAME_X1 - FRAME_X0, height=.7, color=INK, stroke_width=2.4)
        cursor.add_updater(lambda m: (m.move_to([(FRAME_X0 + FRAME_X1) / 2, 1.4 - .8 * self.roster_pos.get_value(), 0]), m.set_stroke(opacity=self.roster_o.get_value())))
        cursor.update()
        spec = mn.VGroup()
        for j, (en, de, unit, colour, key, digits, scale) in enumerate((("Window", "Fenster", "%", _room.GLASS, "wwr", 0, 100), ("Overhang", "Vordach", "m", WARN, "overhang", 1, 1),
                                                                         ("Insulation", "Dämmung", "cm", _room.INSULATION, "insulation", 0, 100))):
            head = mn.VGroup(_room.text([self.tr(en, de)], size=13, color=colour), _room.text([unit], size=13, color=colour)).arrange(mn.DOWN, buff=.05)
            chip = _art.window_chip(head, color=colour)
            spec.add(chip.move_to([CS[j], 2.0 + chip.height / 2, 0]))
            for k, item in enumerate(pool):
                spec.add(text(self.number(item["design"][key] * scale, digits), (CS[j], Y(k), 0), size=19, color=colour))
        cols = {}
        for j, c in enumerate(ORDER):
            group = mn.VGroup(self.criterion_chip(c, CX[j], 2.0))
            for k, item in enumerate(pool):
                group.add(text(self.value_text(c, raw(item["id"], c)), (CX[j], Y(k), 0), size=18, color=COLORS[c], width=.98))
            cols[c] = group
        frames = []
        for j, c in enumerate(ORDER):
            winner = best_id(c, ids())
            frames.append(mn.Rectangle(width=.86, height=.5, color=GOOD, stroke_width=3).move_to([CX[j], Y(ids().index(winner)), 0]))
        self.add(cursor)
        walk = []
        for position in (0, 1, 2, 3, 4, 0):
            walk += [mn.AnimationGroup(self.roster_pos.animate(run_time=.35).set_value(position), self.set_focus(ids()[position], .35)), mn.Wait(.3)]
        self.step("variants", mn.FadeIn(self.panel), mn.FadeIn(self.summer), mn.FadeIn(self.footer), mn.LaggedStart(*[mn.FadeIn(part) for part in base], lag_ratio=.12),
                  mn.Succession(mn.Wait(.8), self.roster_o.animate(run_time=.3).set_value(1.0), *walk), seconds=4.4)
        self.step("variants_2", mn.FadeIn(spec), seconds=1.8)
        self.step("criteria", mn.FadeOut(spec), mn.FadeIn(cols["electricity"]), mn.FadeIn(cols["cost"]), mn.FadeIn(self.strip), self.count.animate(run_time=.3).set_value(2), seconds=1.8)
        self.step("criteria_2", mn.FadeIn(cols["daylight"]), mn.FadeIn(cols["carbon"]), self.count.animate(run_time=.3).set_value(4), seconds=1.8)
        visits = []
        for j, c in enumerate(ORDER):
            winner = best_id(c, ids())
            visits += [mn.AnimationGroup(self.roster_pos.animate(run_time=.4).set_value(ids().index(winner)), self.set_focus(winner, .4), self.active.animate(run_time=.1).set_value(j),
                                         mn.Create(frames[j], run_time=.4)), mn.Wait(.3)]
        self.step("best", mn.Succession(*visits), seconds=3.4)
        self.a_items = [base, cursor, *cols.values(), *frames]

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
            y = 1.9 - .72 * rank
            thumb = self.thumbnail(by_id(identifier)["design"], (-6.35, y, 0), .11, ramp=self._ramp, value=self._ramp.low)
            name = self.name(identifier, size=S0, width=1.5).move_to([-5.25, y, 0])
            cursor, segments = -4.2, []
            for c in ORDER:
                value = raw(identifier, c) / (scale if c in ("cost", "carbon") else 1)
                width = max(value / longest * 3.0, .012)
                segments.append(mn.Rectangle(width=width, height=.4, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85).move_to([cursor + width / 2, y, 0]))
                cursor += width
            total = text(self.number(sums[identifier], 0 if scale == 1 else 1), (cursor + .6, y, 0), size=S1)
            total.move_to([cursor + .12 + total.width / 2, y, 0])
            badge = mn.Circle(radius=.15, color=GOOD if rank == 0 else INK, stroke_width=2).move_to([-6.9, y, 0])
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
            group.add(mn.VGroup(glyph(c, width=.28), text(unit, size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.1))
        return group.arrange(mn.RIGHT, buff=.4).move_to([-3.5, -1.95, 0])

    def screen_units(self):
        """➕ Why kWh/(m²·a) and CHF cannot be added, and why a naive sum depends on the unit."""
        first = ids()[0]
        item = by_id(first)
        chips = []
        for c in ("electricity", "cost"):
            content = mn.VGroup(glyph(c, width=.34), text(self.value_text(c, raw(first, c)), size=S2 + 2), text(self.unit_label(c), size=S1, color=COLORS[c])).arrange(mn.RIGHT, buff=.16)
            chips.append(_art.window_chip(content, color=COLORS[c]))
        plus = text("+", size=S2 + 6)
        tail = mn.VGroup(text("=", size=S2 + 6), text("?", size=S2 + 6)).arrange(mn.RIGHT, buff=.25)
        equation = mn.VGroup(chips[0], plus, chips[1], tail).arrange(mn.DOWN, buff=.18).move_to([-3.5, .5, 0])
        cross = _art.cross_mark(color=BAD, size=.28).next_to(tail, mn.RIGHT, buff=.3)
        verdict = _art.window_chip(text(self.tr("no unit", "keine Einheit"), size=S2, color=BAD), color=BAD).move_to([-3.5, -1.75, 0])
        subject = mn.VGroup(self.thumbnail(item["design"], (0, 0, 0), .26, ramp=self._ramp, value=self._ramp.low), self.name(first, size=S1, width=3.2)).arrange(mn.RIGHT, buff=.3).move_to([-3.5, 2.6, 0])
        self.step("units", *[mn.FadeOut(part) for part in self.a_items], *[mn.FadeIn(chip) for chip in chips], mn.FadeIn(subject), mn.FadeIn(plus), mn.FadeIn(tail),
                  mn.Create(cross), mn.FadeIn(verdict), self.set_focus(first, .8), self.active.animate(run_time=.1).set_value(-1), seconds=2.4)
        rows, order = self.naive_rows(1)
        legend = self.legend(1)
        head = _room.place(text(self.tr("Sum of the printed numbers", "Summe der gedruckten Zahlen"), size=S1, color=MUTED), (LEFT, 3.1, 0), "left")
        unit_chip = _art.window_chip(text(self.tr("Units: CHF · kg", "Einheiten: CHF · kg"), size=S1, color=WARN), color=WARN)
        _room.place(unit_chip, (RIGHT, 2.5, 0), "right")
        self.rows = rows
        self.step("units_2", mn.FadeOut(mn.VGroup(*chips, plus, tail)), mn.FadeOut(cross), mn.FadeOut(verdict), mn.FadeOut(subject),
                  *[mn.FadeIn(r) for r in rows.values()], mn.FadeIn(legend), mn.FadeIn(head), mn.FadeIn(unit_chip), self.set_focus(order[0], .8), seconds=2.2)
        after, order2 = self.naive_rows(1000)
        legend2 = self.legend(1000)
        head2 = _room.place(text(self.tr("Still a sum of unlike units", "Immer noch eine Summe verschiedener Einheiten"), size=S1, color=MUTED), (LEFT, 3.1, 0), "left")
        unit_chip2 = _art.window_chip(text(self.tr("Units: kCHF · t", "Einheiten: kCHF · t"), size=S1, color=GOOD), color=GOOD)
        _room.place(unit_chip2, (RIGHT, 2.5, 0), "right")
        self.step("units_3", *[mn.ReplacementTransform(rows[i], after[i]) for i in ids()], mn.ReplacementTransform(legend, legend2),
                  mn.ReplacementTransform(unit_chip, unit_chip2), mn.ReplacementTransform(head, head2), self.set_focus(order2[0], .8), seconds=2.8)
        self.naive_order = (order, order2)

    def ruler_away(self, ruler, header, away, run_time=.6):
        """🫥 Fade the ruler out while the winter picture needs the whole width, and back in afterwards."""
        level = 0.0 if away else 1.0
        parts = [ruler.line, header, *[ruler.members[i][k] for i in ruler.pool for k in ("slot", "dot", "leader")]]
        return mn.AnimationGroup(ruler.tween(run_time, o_abs=level, o_val=level, o_ring=level), *[(mn.FadeOut(part) if away else mn.FadeIn(part)) for part in parts], run_time=run_time)

    def screen_rulers(self):
        """📏 One ruler per criterion: real values, subtract the best, divide by the span; then the table."""
        pool = ids()
        self.clear_stage()
        ruler = Ruler(self, "electricity", pool, pool)
        header = self.ruler_header(ruler)
        block = self.block_electricity()
        self.add(ruler.ticks)
        first = ruler.scan[0]
        self.step("ruler", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(pool)), mn.FadeIn(block), ruler.appear(1.2), self.set_focus(first, .8),
                  self.active.animate(run_time=.3).set_value(0), seconds=2.4)
        self.add(ruler.ring)
        last = len(ruler.scan) - 1
        self.step("ruler_2", mn.Succession(ruler.tween(.3, o_ring=1.0), ruler.visit(list(range(1, last + 1)) + [0], hold=.4, travel=.6)), seconds=5.0)
        bare = ruler.scan.index("glass_bare")
        self.step("ruler_3", ruler.focus.animate(run_time=.8).set_value(bare), self.set_focus("glass_bare", .8), self.row_hi.animate(run_time=.1).set_value(1), seconds=3.6)
        winter = mn.VGroup(self.winter_note(), self.derivation(self._rt, "heating", (LEFT + .4, .4, 0), size=18, words=False, align="left", width=5.2))
        self.step("ruler_4", self.ruler_away(ruler, header, True), self.set_focus("punched", .8), self.swap_season("winter"), self.row_hi.animate(run_time=.1).set_value(0), mn.FadeIn(winter), seconds=3.6)
        self.step("ruler_5", self.ruler_away(ruler, header, False), mn.FadeOut(winter), self.swap_season("summer"), ruler.focus.animate(run_time=.1).set_value(ruler.scan.index("punched")),
                  mn.Succession(self.row_hi.animate(run_time=.1).set_value(2), mn.Wait(1.2), self.row_hi.animate(run_time=.1).set_value(3), mn.Wait(.9),
                                self.row_hi.animate(run_time=.1).set_value(4)), seconds=4.0)
        note = self.step_note(ruler, "−", ruler.best)
        self.step("shift", ruler.pan(2.0), mn.FadeIn(note), ruler.focus.animate(run_time=1.0).set_value(0), self.set_focus(ruler.scan[0], 1.0), self.row_hi.animate(run_time=.1).set_value(-1),
                  self.detail.animate(run_time=.1).set_value(0), seconds=2.2)
        note2 = self.step_note(ruler, "÷", round(ruler.high, ruler.digits_tag) - round(ruler.low, ruler.digits_tag))
        zero_one = _room.place(text(self.tr(f"0 = best, 1 = worst of the {len(pool)} compared variants", f"0 = Bester, 1 = Schlechtester der {len(pool)} verglichenen Varianten"), size=S0, color=MUTED),
                               (LEFT + .4, -.36, 0), "left")
        self.step("shift_2", ruler.stretch(2.0), mn.ReplacementTransform(note, note2), mn.FadeIn(zero_one), seconds=2.2)
        self.step("mapped", mn.FadeOut(note2), self.detail.animate(run_time=.1).set_value(1), ruler.visit([0, last], hold=.6, travel=.8), seconds=2.8)
        for key in ("cost", "daylight"):
            new, new_header, new_block, run = self.next_ruler(key, ruler, header, block, [zero_one] if zero_one is not None else [])
            sky = mn.VGroup(_room.place(_room.text(self.words(self.tr("sky light · in DF", "Himmelslicht · im TQ")), size=14, color=_room.DAYLIGHT), (2.15, 2.75, 0), "left"),
                            _room.place(_room.text(self.words(self.tr("direct sun · not in DF", "Direktsonne · nicht im TQ")), size=14, color=_room.SUN), (2.15, 2.45, 0), "left"))
            self.step(key, mn.Succession(*run), *([mn.FadeIn(sky)] if key == "daylight" else []), seconds=3.6)
            self.add(new.ring)
            ruler, header, block, zero_one = new, new_header, new_block, None
            if key == "cost":
                overlay = self.materials_overlay(("wall", "insulation", "window", "overhang"))
                self.step("cost_2", mn.FadeIn(overlay), ruler.focus.animate(run_time=.8).set_value(ruler.scan.index("ribbon")), self.set_focus("ribbon", .8),
                          mn.Succession(*[item for k in range(5) for item in (self.row_hi.animate(run_time=.1).set_value(k), mn.Wait(.7))]), seconds=4.4)
                self.row_hi.set_value(-1)
                self.overlay = overlay
        new, new_header, new_block, run = self.next_ruler("carbon", ruler, header, block, [sky], retire=[self.overlay])
        overlay = self.materials_overlay(("wall", "insulation", "window", "overhang"))
        self.step("carbon", mn.Succession(*run), mn.Succession(mn.Wait(1.2), mn.FadeIn(overlay)), seconds=4.4)
        board = Board(self, pool)
        board.cur.set_value(1.0)
        legend = self.table_legend()
        self._legend, self.board = legend, board
        moves, swaps = self.glide([new.members[i]["slot"] for i in pool], [board.labels[i] for i in pool])
        fold = mn.Succession(mn.AnimationGroup(mn.FadeOut(new.line), mn.FadeOut(new.ticks), mn.FadeOut(new_header), Retire(new_block), Retire(overlay), mn.FadeOut(new.ring),
                                               *[mn.FadeOut(new.members[i][k]) for i in pool for k in ("dot", "leader", "tag", "norm")], *moves,
                                               self.set_focus(board.scan[0], 1.2), self.active.animate(run_time=.1).set_value(-1)),
                             mn.AnimationGroup(*swaps, mn.FadeIn(board.live), mn.FadeIn(legend), board.fill.animate(run_time=2.0).set_value(1.0)))
        self.step("matrix", fold, seconds=5.0)
        self.ruler = new

    def winter_note(self):
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
                                            LEFT + .4, 3.0, size=16, width=6.4),
                              self.equation([([self.tr("Conductance", "Leitwert"), "H", "=", self.n(parts[0], digits), "+", self.n(parts[1], digits), "+", self.n(parts[2], digits), "=", self.n(sum(parts), digits), "W/K"], INK)],
                                            LEFT + .4, 2.6, size=15, width=6.4),
                              self.equation([([self.tr("wall U", "Wand U"), self.n(found["u_wall"], 2), "·", self.tr("window U", "Fenster U"), self.n(constant("envelope", "u_window"), 1), "W/m²K", "·",
                                               self.tr("other", "Rest"), self.n(constant("envelope", "h_other")), "W/K"], MUTED)], LEFT + .4, 2.3, size=14, width=6.4),
                              self.equation([([self.tr("heat losses − used free heat = heating demand", "Wärmeverlust − genutzte freie Wärme = Heizbedarf")], MUTED)], LEFT + .4, 1.3, size=14, width=5.2),
                              self.equation([([self.tr("heating demand ÷ COP = heating", "Heizbedarf ÷ COP = Heizung")], MUTED)], LEFT + .4, 1.02, size=14, width=5.2))
            return group
        return live(build, _room.design_key(rt))

    def step_note(self, ruler, sign, value):
        """🧾 The operation of a normalization step, e.g. − best value with its unit."""
        number = _room.Number(sign + self.number(value, ruler.digits_tag)) if sign == "−" else self.n(value, ruler.digits_tag)
        label = _room.text([*([] if sign == "−" else [sign]), number, self.unit_label(ruler.criterion)], (ruler.center, -1.15, 0), size=S1, color=ruler.color)
        return label

    def ruler_header(self, ruler):
        """🏷️ Criterion pictogram, title, unit and direction."""
        c = ruler.criterion
        direction = self.tr("lower is better ↓", "weniger ist besser ↓") if CRITERIA[c]["direction"] == "min" else self.tr("higher is better ↑", "mehr ist besser ↑")
        title = TITLES[c][self.language() == "de"] if c in TITLES else CRITERIA[c]["title"][self.language()]
        items = mn.VGroup(glyph(c, width=.36), text(f"{title}, {self.unit_label(c)}", size=S1, color=ruler.color), text(direction, size=S1)).arrange(mn.RIGHT, buff=.22)
        return items.move_to([LEFT + items.width / 2, 3.34, 0])

    def next_ruler(self, criterion, old, header, block, extras, retire=()):
        """🔁 Replace the ruler by the next criterion; returns the new parts and the animations (flip first for daylight)."""
        pool = ids()
        new = Ruler(self, criterion, pool, pool)
        new_header = self.ruler_header(new)
        new_block = {"cost": self.block_cost, "daylight": self.block_daylight, "carbon": self.block_carbon}[criterion]()
        for member in old.members.values():
            member["slot"].clear_updaters()
        keep = ("dot", "leader", "tag", "norm")
        index = ORDER.index(criterion)
        swap = mn.AnimationGroup(
            *[mn.ReplacementTransform(old.members[i]["slot"], new.members[i]["slot"]) for i in pool],
            mn.FadeOut(old.line), mn.FadeOut(old.ticks), mn.FadeOut(old.ring), *[mn.FadeOut(e) for e in extras], *[Retire(r) for r in retire],
            *[mn.FadeOut(old.members[i][k]) for i in pool for k in keep], mn.ReplacementTransform(header, new_header),
            Retire(block), mn.FadeIn(new_block), mn.FadeIn(new.line), *[mn.FadeIn(new.members[i][k]) for i in pool for k in keep],
            new.appear(1.0), self.active.animate(run_time=.3).set_value(index), self.row_hi.animate(run_time=.1).set_value(-1))
        self.add(new.ticks)
        target = {"cost": new.scan[0], "daylight": new.scan[0], "carbon": "ribbon"}[criterion]
        run = [swap, mn.AnimationGroup(new.pan(1.0), self.set_focus(target, 1.0), new.focus.animate(run_time=1.0).set_value(new.scan.index(target))), new.stretch(1.0)]
        if CRITERIA[criterion]["direction"] == "max":
            run.insert(1, new.flip(1.6))
        return new, new_header, new_block, run

    def glide(self, slots, labels):
        """🛫 Move ruler slots onto their table rows, then cross-fade them into the row labels."""
        moves, swaps = [], []
        for slot, label in zip(slots, labels):
            slot.clear_updaters()
            moves.append(slot.animate(run_time=1.2).scale(.5).move_to(label.get_center()))
            swaps += [mn.FadeOut(slot), mn.FadeIn(label)]
        return moves, swaps

    def table_legend(self):
        """🧭 How to read the table: 0 is the best and 1 the worst of the compared set; the arrows of the column heads give the direction."""
        scale = mn.VGroup(text(self.tr("Normalized value:", "Normierter Wert:"), size=S0),
                          text(self.tr("0 = best", "0 = Bester"), size=S0, color=GOOD),
                          text(self.tr("1 = worst", "1 = Schlechtester"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.3)
        under = text(self.tr("Under each bar: normalized value, then physical value", "Unter jedem Balken: normiert, darunter der physikalische Wert"),
                     size=14, color=MUTED, width=6.8)
        return mn.VGroup(scale, under).arrange(mn.DOWN, buff=.1).move_to([-3.55, -2.3, 0])

    def screen_board(self):
        """⚖️ Weights, the index bar, its arithmetic, the winner's room and a cost-driven re-weighting; the focus row moves the room."""
        board = self.board
        ws = self._weights
        self.step("weights", board.mode.animate(run_time=1.2).set_value(1.0), seconds=1.4)
        self.step("weights_2", board.weights_to(ws, 2.5), seconds=2.6)
        wide, narrow = max(range(4), key=lambda j: ws[j]), min(range(4), key=lambda j: ws[j])
        outlines, tags = mn.VGroup(), mn.VGroup()
        for j, (shape, colour) in ((wide, (lambda r: r, GOOD)), (narrow, (lambda r: mn.DashedVMobject(r, num_dashes=40), WARN))):
            centre = board.xl + board.width * (sum(ws[:j]) + ws[j] / 2)
            top, bottom = board.y0 + .3, board.y0 - board.pitch * 4 - .7
            outlines.add(shape(mn.Rectangle(width=ws[j] * board.width + .1, height=top - bottom, color=colour, stroke_width=3).move_to([centre, (top + bottom) / 2, 0])))
            words = [self.tr("strongest pull", "stärkster Einfluss") if j == wide else self.tr("weakest pull", "schwächster Einfluss"), self.n(ws[j] * 100), "%"]
            chip = _room.chip(words, (centre, -2.5, 0), size=14, color=colour)
            chip.shift(np.array([(LEFT + .3 - chip.get_left()[0]) if j == wide else (FRAME_X1 - chip.get_right()[0]), 0.0, 0.0]))
            tags.add(chip)
        self.step("weights_3", mn.Create(outlines), mn.FadeOut(self._legend), mn.FadeIn(tags), board.visit(list(range(1, len(board.scan))) + [0], .3), seconds=2.6)
        axis = self.index_axis(board)
        self.step("index", mn.Succession(mn.AnimationGroup(mn.FadeOut(outlines), mn.FadeOut(tags), board.compact.animate(run_time=2.0).set_value(1.0), board.numbers.animate(run_time=1.0).set_value(0.0),
                                                           board.scores.animate(run_time=1.4).set_value(1.0), mn.FadeIn(axis)),
                                         board.segments()), seconds=4.8)
        key = tuple(round(v, 6) for v in board.weights())
        ranks = {i: row["rank"] for i, row in table(key).items()}
        winner = winner_of(key)
        self.step("index_2", board.rank_to(ranks, 2.0), board.badges.animate(run_time=1.2).set_value(1.0), board.win.animate(run_time=1.2).set_value(1.0),
                  board.cur.animate(run_time=1.6).set_value(0.0), board.visit([board.scan.index(winner)], 2.0), seconds=2.4)
        formula = self.block_index(lambda: board.cursor()[0], board.weights, y=-2.2)
        self.step("index_3", mn.FadeIn(formula), seconds=2.6)
        self.step("winner", self.pulse(getattr(self, self.season)[0], color=GOOD, run_time=1.4),
                  mn.Succession(mn.Succession(board.win.animate(run_time=.4).set_value(.25), board.win.animate(run_time=.6).set_value(1.0)), board.segments(.5, .3)), seconds=4.4)
        state = table(key)[winner]["normalized"]
        weakest = max(range(4), key=lambda j: state[ORDER[j]])
        cursor = board.xl + board.width * sum(ws[j] * state[ORDER[j]] for j in range(weakest))
        width = board.width * ws[weakest] * state[ORDER[weakest]]
        mark = mn.Rectangle(width=max(width, .06) + .08, height=.46, color=WARN, stroke_width=3).move_to([cursor + width / 2, board.y0, 0])
        name = SHORT[ORDER[weakest]][1 if self.language() == "de" else 0]
        note = mn.VGroup(glyph(ORDER[weakest], width=.28, color=WARN), text(self.tr(f"weakest = highest normalized value: {name} {self.number(state[ORDER[weakest]], 2)}",
                                                                                      f"schwächstes = höchster normierter Wert: {name} {self.number(state[ORDER[weakest]], 2)}"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.15).move_to([-3.4, -2.5, 0])
        self.step("winner_2", mn.Create(mark), mn.FadeIn(note), seconds=1.8)
        persona = mn.VGroup(glyph("cost", width=.24, color=WARN), text(self.tr("cost-driven client", "kostenorientierter Bauherr"), size=S1, color=WARN)).arrange(mn.RIGHT, buff=.2).move_to([-3.4, -2.48, 0])
        board.old_id = winner
        heavy_key = tuple(round(v, 6) for v in self._heavy)
        heavy_ranks = {i: row["rank"] for i, row in table(heavy_key).items()}
        new = winner_of(heavy_key)
        was = _room.chip(self.words(self.tr("was 1st", "war 1.")), (-1.4, board.y0 - board.pitch - .12, 0), size=S0, color=WARN)
        self.step("sensitivity", mn.FadeOut(mark), mn.FadeOut(note), mn.FadeIn(persona), board.weights_to(self._heavy, 3.0),
                  mn.Succession(mn.Wait(1.0), board.rank_to(heavy_ranks, 2.0)), board.win.animate(run_time=1.0).set_value(0.0), seconds=3.2)
        self.step("sensitivity_2", board.old.animate(run_time=1.0).set_value(1.0), mn.FadeIn(was), seconds=1.8)
        self.step("sensitivity_3", board.win.animate(run_time=1.0).set_value(1.0), mn.FadeOut(was), board.visit([board.scan.index(new)], 1.5), seconds=1.8)
        self.persona = persona

    def index_axis(self, board):
        """📐 Scale under the bars: 0 is the best and 1 the worst of the compared set in every criterion."""
        y = board.y0 - (len(board.all) - 1) * board.pitch - .36
        line = mn.Line([board.xl, y, 0], [board.xl + board.width, y, 0], color=INK, stroke_width=2)
        ticks = mn.VGroup(*[mn.VGroup(mn.Line([board.xl + board.width * f, y - .06, 0], [board.xl + board.width * f, y + .06, 0], color=INK, stroke_width=2),
                                      text({0: "0", 1: "1"}.get(f, self.number(f, 1)), [board.xl + board.width * f, y - .22, 0], size=S0)) for f in (0, .5, 1)])
        return mn.VGroup(line, ticks, _room.place(text(self.tr("Index", "Index"), size=S1, color=INK), (board.xl - .15, y, 0), "right"))

    def switch_shares(self, base):
        """🔀 The cost weights (in percent) at which the winner switches, found by bisection on the model's index."""
        champion = lambda share: winner_of(tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER))
        found, previous = [], champion(0)
        for k in range(1, 401):
            current = champion(k * .25)
            if current != previous:
                low, high = (k - 1) * .25, k * .25
                for _ in range(24):
                    middle = (low + high) / 2
                    low, high = (middle, high) if champion(middle) == previous else (low, middle)
                found.append(high)
                previous = current
        return found

    def close_bands(self, base, limit=.03):
        """🤏 The cost weights (in percent) between which the top two index values lie closer than ``limit``: exact edges by bisection on the gap."""
        def gap(share):
            scores = sorted(row["score"] for row in table(tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER)).values())
            return scores[1] - scores[0] - limit
        grid = [k * .25 for k in range(401)]
        flags = [gap(share) < 0 for share in grid]
        bands, start = [], None
        for k, flag in enumerate(flags + [False]):
            if flag and start is None:
                start = k
            if not flag and start is not None:
                low, high = grid[start], grid[k - 1]
                if start > 0:
                    a, b = grid[start - 1], grid[start]
                    for _ in range(24):
                        middle = (a + b) / 2
                        a, b = (a, middle) if gap(middle) < 0 else (middle, b)
                    low = (a + b) / 2
                if k <= 400:
                    a, b = grid[k - 1], grid[k]
                    for _ in range(24):
                        middle = (a + b) / 2
                        a, b = (middle, b) if gap(middle) < 0 else (a, middle)
                    high = (a + b) / 2
                bands.append((low, high))
                start = None
        return bands

    def screen_sweep(self):
        """🔀 Rank of every variant against the cost weight, with the winner strip, the switch shares and the room of the current winner."""
        self.clear_stage()
        pool = ids()
        shares = [2.5 * k for k in range(41)]
        base = _model.load_study()["weights"]["balanced"]
        ranks = {i: [] for i in pool}
        for share in shares:
            weights = tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER)
            rows = table(weights)
            for i in pool:
                ranks[i].append(rows[i]["rank"])
        x0, x1, top, step = -4.8, -1.9, 1.7, .66
        X = lambda share: x0 + (x1 - x0) * share / 100
        Y = lambda rank: top - (rank - 1) * step
        static = mn.VGroup()
        for rank in range(1, 6):
            static.add(mn.Line([x0, Y(rank), 0], [x1, Y(rank), 0], color=MUTED, stroke_width=1, stroke_opacity=.35), text(str(rank), (x0 - .2, Y(rank), 0), size=S0, color=MUTED))
        static.add(mn.Line([x0, Y(5) - .4, 0], [x1, Y(5) - .4, 0], color=INK, stroke_width=2))
        for share in (0, 25, 50, 75, 100):
            static.add(mn.Line([X(share), Y(5) - .46, 0], [X(share), Y(5) - .34, 0], color=INK, stroke_width=2), text(str(share), (X(share), Y(5) - .64, 0), size=S0))
        static.add(text(self.tr("rank", "Rang"), (x0 - .55, Y(5) - .64, 0), size=S0, color=MUTED),
                   _room.place(text(self.tr("cost weight (%)", "Kostengewicht (%)"), size=S0, color=MUTED, width=1.5), (x1 + .3, Y(5) - .64, 0), "left"),
                   _room.place(text(self.tr("at 0 %", "bei 0 %"), size=S0, color=MUTED), (x0 - .45, Y(1) + .75, 0), "right"),
                   _room.place(text(self.tr("at 100 %", "bei 100 %"), size=S0, color=MUTED), (x1 + .3, Y(1) + .75, 0), "left"))
        lines_ = mn.VGroup(*[mn.VMobject(color=INK, stroke_width=2.4, stroke_opacity=.55).set_points_as_corners([[X(s), Y(r), 0] for s, r in zip(shares, ranks[i])]) for i in pool])
        names = mn.VGroup()
        for i in pool:
            names.add(_room.place(self.name(i, size=14, width=1.7, color=VARIANT_COLOR[i] if ranks[i][0] == 1 else INK), (x0 - .4, Y(ranks[i][0]), 0), "right"),
                      _room.place(self.name(i, size=14, width=1.55, color=VARIANT_COLOR[i] if ranks[i][-1] == 1 else INK), (x1 + .3, Y(ranks[i][-1]), 0), "left"))
        strip, winners = mn.VGroup(), [next(i for i in pool if ranks[i][k] == 1) for k in range(len(shares))]
        half = (x1 - x0) / (len(shares) - 1) / 2
        for winner, first, last in runs_of(winners):
            a, b = max(x0, X(shares[first]) - half), min(x1, X(shares[last]) + half)
            strip.add(mn.Line([a + .02, Y(1) + .16, 0], [b - .02, Y(1) + .16, 0], color=VARIANT_COLOR[winner], stroke_width=6),
                      self.thumbnail(by_id(winner)["design"], ((a + b) / 2, Y(1) + .52, 0), .075, ramp=self._ramp, value=self._ramp.low, ring=VARIANT_COLOR[winner]))
        for crossing in self.switch_shares(base):
            strip.add(mn.DashedLine([X(crossing), Y(1) + .16, 0], [X(crossing), Y(5) - .4, 0], color=GOOD, stroke_width=2, stroke_opacity=.7),
                      _room.chip([_room.Number(self.number(crossing, 0)), "%"], (X(crossing), Y(1) + .88, 0), size=S0, color=GOOD))
        close = mn.VGroup()
        bands = self.close_bands(base)
        for low, high in bands:
            close.add(mn.Rectangle(width=X(high) - X(low), height=Y(1) - Y(5) + .5, stroke_width=0, fill_color=WARN, fill_opacity=.2).move_to([(X(low) + X(high)) / 2, (Y(1) + Y(5)) / 2, 0]))
        key = mn.VGroup(mn.Rectangle(width=.3, height=.22, stroke_width=1.4, color=WARN, fill_color=WARN, fill_opacity=.3),
                        text(self.tr("close call: index gap below 0.03", "knapp: Indexabstand unter 0,03"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.15)
        key.move_to([-3.4, -2.35, 0])
        close.add(key)
        close_share = (bands[-1][0] + bands[-1][1]) / 2 if bands else 50
        share = mn.ValueTracker(self._heavy[ORDER.index("cost")] * 100)

        def champion():
            weights = tuple(_model.reweigh(base, "cost", share.get_value() / 100)[c] for c in ORDER)
            return winner_of(weights)
        for name in ("orientation", "wwr", "overhang", "insulation"):
            self._rt[name].add_updater(lambda m, name=name: m.set_value(by_id(champion())["design"][name]))
        self.focus.add_updater(lambda m: m.set_value(ids(True).index(champion())))
        self.add(share, *self._rt.values(), self.focus)

        def marker():
            weights = tuple(_model.reweigh(base, "cost", share.get_value() / 100)[c] for c in ORDER)
            rows = table(weights)
            x = X(share.get_value())
            group = mn.VGroup(mn.Line([x, Y(1) + .3, 0], [x, Y(5) - .4, 0], color=INK, stroke_width=2.4))
            for i in pool:
                group.add(mn.Dot([x, Y(rows[i]["rank"]), 0], radius=.09, color=GOOD if rows[i]["rank"] == 1 else INK))
            chip = _room.chip([self.tr("cost weight", "Kostengewicht"), _room.Number(self.number(share.get_value(), 0)), "%"], (x, Y(5) - .98, 0), size=S0, color=COLORS["cost"])
            chip.shift(mn.RIGHT * (max(0, x0 - .6 - chip.get_left()[0]) - max(0, chip.get_right()[0] - (x1 + .6))))
            group.add(chip)
            return group
        self.step("sweep", mn.FadeIn(static), mn.Create(lines_), mn.FadeIn(names), mn.FadeIn(strip), mn.FadeIn(live(marker)),
                  mn.Succession(mn.Wait(.8), share.animate(run_time=1.4).set_value(0), share.animate(run_time=2.0).set_value(100), share.animate(run_time=1.0).set_value(self._heavy[ORDER.index("cost")] * 100)), seconds=5.4)
        self.step("sweep_2", mn.FadeIn(close), share.animate(run_time=1.0).set_value(close_share), seconds=1.4)
        self.sweep_share = share

    def screen_range(self):
        """➕ A sixth variant stretches the cost ruler, squeezes the others and flips the order of two of them."""
        self.clear_stage()
        for tracker in (*self._rt.values(), self.focus):
            tracker.clear_updaters()
        five, six = ids(), ids(True)
        extra = six[-1]
        ruler = Ruler(self, "cost", six, five, state="normalized", name_size=13, rings=False)
        header = self.ruler_header(ruler)
        block = self.block_cost()
        ruler.scan = ruler.scan + [extra]
        ruler.focus.set_value(len(ruler.scan) - 1)
        worst_old = max(five, key=lambda i: ruler.values[i])
        frames = {"best": self.slot_frame(ruler, ruler.scan[0], GOOD), "old": self.slot_frame(ruler, worst_old, WARN), "new": self.slot_frame(ruler, extra, WARN)}
        ghosts = mn.VGroup(*[mn.DashedVMobject(mn.Circle(radius=.09, color=MUTED, stroke_width=2).move_to([ruler.x_of(ruler.values[i]), ruler.line_y, 0]), num_dashes=10) for i in five])
        hint = text(self.tr(f"{self.number(ruler.fraction(extra), 2)} > 1: beyond the old range", f"{self.number(ruler.fraction(extra), 2)} > 1: außerhalb des alten Bereichs"),
                    (ruler.x0 + ruler.length - 1.6, -.35, 0), size=S0, color=WARN)
        ruler.ring.update()
        self.add(ruler.ticks, ruler.ring, *frames.values())
        self.step("range", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(five)), mn.FadeIn(ghosts), frames["best"].level.animate.set_value(1.0), frames["old"].level.animate.set_value(1.0),
                  self.set_focus(extra, .8), self.active.animate(run_time=.3).set_value(1),
                  mn.Succession(mn.Wait(.8), mn.FadeIn(ruler.mobjects([extra])), mn.AnimationGroup(mn.FadeIn(block), mn.FadeIn(hint), ruler.tween(.6, o_ring=1.0))), seconds=3.0)
        span = max(ruler.values[i] for i in six) - min(ruler.values[i] for i in six)
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
        self.ruler = ruler
        order = sorted(five, key=lambda i: table(self._weights)[i]["rank"])
        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.54, y0=2.2, name_size=13)
        board.header.set_value(0.0)
        for tracker in (board.fill, board.compact, board.scores, board.badges, board.win, board.mode):
            tracker.set_value(1.0)
        board.numbers.set_value(0.0)
        new_ranks = {i: row["rank"] for i, row in table(self._weights, True).items()}
        old_ranks = {i: row["rank"] for i, row in table(self._weights).items()}
        axis = self.index_axis(board)
        notes = mn.VGroup()
        for i in six:
            before = str(old_ranks[i]) if i in old_ranks else self.tr("new", "neu")
            note = text(f"{before} → {new_ranks[i]}", (-1.6, 0, 0), size=S1, color=WARN if i in old_ranks and old_ranks[i] != new_ranks[i] else INK)
            note.identifier = i
            note.add_updater(lambda m: (m.move_to([-.98 - m.width / 2, board.y(m.identifier), 0]), m.set_opacity(board.row_o[m.identifier].get_value())))
            notes.add(note)
        flipped = [i for i in five if (sorted(five, key=old_ranks.get).index(i) != sorted([f for f in six if f in five], key=new_ranks.get).index(i))]
        heading = _room.place(text(self.tr("rank old → new (orange: changed)", "Rang alt → neu (orange: geändert)"), size=S0, color=MUTED), (LEFT + .4, -1.35, 0), "left")
        link = None
        board_parts = mn.VGroup(board.live, axis, notes, heading)
        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(header), Retire(block), mn.FadeOut(ruler.members[extra]["slot"]),
                                *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag", "norm")], self.active.animate(run_time=.3).set_value(0))
        moves, swaps = self.glide([ruler.members[i]["slot"] for i in five], [board.labels[i] for i in five])
        fold = mn.Succession(mn.AnimationGroup(*moves), mn.AnimationGroup(*swaps, mn.FadeIn(board_parts)))
        reorder = mn.AnimationGroup(board.t.animate(run_time=1.6).set_value(1.0), board.row_o[extra].animate(run_time=1.6).set_value(1.0),
                                    board.rank_to(new_ranks, 2.0), mn.FadeIn(board.labels[extra]))
        steps = [out, fold, reorder] + ([mn.Create(link, run_time=.7)] if link is not None else [])
        self.step("range_3", mn.Succession(*steps), seconds=5.4)
        self.final_board = board

    def slot_frame(self, ruler, identifier, color):
        """🔲 A frame that follows a thumbnail on the ruler; its opacity is the ``level`` tracker."""
        frame = mn.Rectangle(width=.95, height=.78, color=color, stroke_width=2.4)
        frame.level = mn.ValueTracker(0.0)
        frame.add_updater(lambda m: (m.move_to(ruler.members[identifier]["slot"][0].get_center()), m.set_stroke(opacity=m.level.get_value())))
        frame.update()
        return frame

    def screen_record(self):
        """🗂️ The decision record, the budget limit, the bridge to Pareto fronts and the answer."""
        self.clear_stage()
        pool, key = ids(), self._weights
        rows = table(key)
        winner = winner_of(key)
        left = lambda item, x, y: _room.place(item, (x, y, 0), "left")
        card = _art.window_frame(.1 - LEFT, 5.75, color=INK).move_to([(LEFT + .1) / 2, .4, 0])
        parts = mn.VGroup()
        heading = lambda label, x, y: left(text(label, size=S1, color=MUTED), x, y)
        parts.add(heading(self.tr("Weights", "Gewichte"), LEFT + .3, 3.0))
        for k, c in enumerate(ORDER):
            line = mn.VGroup(glyph(c, width=.26), text(f"{SHORT[c][1 if self.language() == 'de' else 0]} {self.number(key[k] * 100)} %", size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.12)
            parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, 2.6 - .38 * (k // 2)))
        parts.add(heading(self.tr("Compared variants", "Verglichene Varianten"), LEFT + .3, 1.65))
        for k, i in enumerate(pool):
            x = LEFT + .72 + 1.36 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, 1.0, 0), .11, ramp=self._ramp, value=self._ramp.low), block("\n".join(stack(by_id(i)["title"][self.language()])), size=13, width=1.3).move_to([x, .3, 0]))
        parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), LEFT + .3, -.45))
        for k, c in enumerate(ORDER):
            best = best_id(c, pool)
            worst = max(pool, key=lambda i: raw(i, c) * (1 if CRITERIA[c]["direction"] == "min" else -1))
            line = mn.VGroup(glyph(c, width=.26), text(f"{self.value_text(c, raw(best, c))} … {self.value_text(c, raw(worst, c))} {self.unit_label(c)}", size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.12)
            parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, -.9 - .4 * (k // 2)))
        parts.add(heading(self.tr("Result", "Ergebnis"), LEFT + .3, -1.75),
                  self.thumbnail(by_id(winner)["design"], (LEFT + 1.0, -2.15, 0), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), LEFT + 1.5, -2.15),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), LEFT + 4.3, -2.15))
        record = mn.VGroup(card, parts)
        self.step("record", mn.FadeIn(record), self.set_focus(winner, .8), self.active.animate(run_time=.3).set_value(-1), seconds=2.5)
        self.screen_limits(record, winner)
        self.clear_stage()
        pool = ids()
        costs, powers = [raw(i, "cost") for i in pool], [raw(i, "electricity") for i in pool]
        xs, ys = 500, nice_step(max(powers) - min(powers), 4)
        x_range = (math.floor(min(costs) / 1000) * 1000, math.ceil(max(costs) / 1000) * 1000 + 500, xs)
        y_range = (math.floor(min(powers) / ys - .3) * ys, math.ceil(max(powers) / ys + .3) * ys, ys)
        axes, labels = self.plot_axes(x_range=x_range, y_range=y_range, x_label=self.tr("Façade cost, CHF", "Fassadenkosten, CHF"),
                                      y_label=self.tr(f"Electricity intensity, {_room.INTENSITY_UNIT}", f"Strombedarf pro m², {_room.INTENSITY_UNIT}"), point=(-3.5, .35, 0), width=5.6, height=3.4,
                                      y_digits=0 if ys >= 1 else 1, x_ticks=[19000, 20000, 21000, 22000])
        free = [i for i in pool if not any(raw(j, "cost") <= raw(i, "cost") and raw(j, "electricity") <= raw(i, "electricity") and j != i for j in pool)]
        marks = mn.VGroup(*[self.thumbnail(by_id(i)["design"], axes.c2p(raw(i, "cost"), raw(i, "electricity")), .1, ramp=self._ramp, value=self._ramp.low, ring=GOOD if i in free else None) for i in pool])
        tags = mn.VGroup()
        dominator = by_id(winner)["title"][self.language()]
        for i in pool:
            spot = axes.c2p(raw(i, "cost"), raw(i, "electricity"))
            if i in free:
                tags.add(self.name(i, size=13, width=1.5, limit=14).move_to(spot + mn.DOWN * .55))
                continue
            tags.add(_room.place(self.name(i, size=13, width=1.5, limit=14, color=MUTED).move_to(spot), spot + mn.RIGHT * .45, "left"))
        note = _room.place(_room.text(self.words(self.tr("grey = dominated by", "grau = dominiert von")) + self.words(dominator), size=14, color=WARN), (LEFT + .3, 3.0, 0), "left")
        front = sorted((raw(i, "cost"), raw(i, "electricity")) for i in free)
        outline = mn.DashedVMobject(mn.VMobject(color=GOOD, stroke_width=3).set_points_as_corners([axes.c2p(*p) for p in front]), num_dashes=24)
        chip = _art.window_chip(text(self.tr("Pareto front", "Pareto-Front"), size=S2, color=GOOD), color=GOOD).move_to([-1.2, 3.0, 0])
        self.step("pointer", mn.FadeIn(axes), mn.FadeIn(labels), mn.FadeIn(marks), mn.FadeIn(tags), mn.Create(outline), mn.FadeIn(chip), mn.FadeIn(note), self.active.animate(run_time=.3).set_value(0), seconds=2.6)
        self.screen_answer(winner)

    def screen_answer(self, winner):
        """🏁 The answer: the winner's index written out criterion by criterion, and the electricity behind its first value, next to its room."""
        self.clear_stage()
        title = mn.VGroup(self.thumbnail(by_id(winner)["design"], (0, 0, 0), .2, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                          self.name(winner, size=S2, width=3.4, color=GOOD, limit=40),
                          _art.window_chip(text(self.tr("lowest index of five", "kleinster Index von fünf"), size=S1, color=GOOD), color=GOOD)).arrange(mn.RIGHT, buff=.3).move_to([-3.5, 3.15, 0])
        table_ = self.block_index(lambda: winner, lambda: self._weights, full=True)
        self.detail.set_value(1.0)
        loss = self.heat_loss_arrows()
        legend = mn.VGroup(_room.place(_room.text([self.tr("winter heating demand", "Winter: Heizbedarf"), self.n(_room.result(by_id(winner)["design"])["heating_demand"]), "kWh/a"], size=13, color=_room.HEAT), (2.05, 2.5, 0), "left"),
                           _room.place(_room.text([self.tr("summer cooling", "Sommer: Kühllast"), self.n(_room.result(by_id(winner)["design"])["cooling_demand"]), "kWh/a"], size=13, color=_room.COOL), (2.05, 2.2, 0), "left"))
        self.step("next", mn.FadeIn(title), mn.FadeIn(table_), mn.FadeIn(self.block_electricity()), mn.FadeIn(loss), mn.FadeIn(legend), self.active.animate(run_time=.3).set_value(0), seconds=3.0)

    def screen_limits(self, record, winner):
        """🚧 The client's budget is a mandatory limit: it removes variants before any weight is applied; the allowed ones are normalized and ranked again."""
        pool = ids()
        costs = {i: raw(i, "cost") for i in pool}
        over = [i for i in pool if costs[i] > BUDGET_CHF]
        allowed = tuple(i for i in pool if i not in over)
        rows = table(self._weights, pool=allowed)
        low, high = math.floor(min(costs.values()) / 1000) * 1000, math.ceil(max(costs.values()) / 1000) * 1000
        xl, xr, y_axis = -6.0, -.7, .7
        X = lambda value: xl + (value - low) / (high - low) * (xr - xl)
        flow = mn.VGroup(_art.window_chip(text(self.tr("Constraints: filter", "Nebenbedingungen: filtern"), size=S1, color=WARN), color=WARN), mn.Arrow(mn.LEFT * .35, mn.RIGHT * .35, buff=0, color=INK),
                         _art.window_chip(text(self.tr("Weights: rank", "Gewichte: ordnen"), size=S1, color=GOOD), color=GOOD)).arrange(mn.RIGHT, buff=.3)
        flow.move_to([-3.5, 3.25, 0])
        axis = mn.VGroup(mn.Line([xl - .15, y_axis, 0], [xr + .15, y_axis, 0], color=INK, stroke_width=2.4), text("CHF", (xr + .5, y_axis, 0), size=S0, color=COLORS["cost"]))
        for value in range(low, high + 1, 1000):
            axis.add(mn.Line([X(value), y_axis - .07, 0], [X(value), y_axis + .07, 0], color=INK, stroke_width=2))
            if value != BUDGET_CHF:
                axis.add(text(self.number(value), (X(value), y_axis + .3, 0), size=S0))
        order = sorted(pool, key=costs.get)
        gap = (xr - xl + .4) / (len(order) - 1)
        slots = {i: xl - .2 + k * gap for k, i in enumerate(order)}
        dots, groups = {}, mn.VGroup()
        for k, i in enumerate(order):
            thumb = self.thumbnail(by_id(i)["design"], (slots[i], 2.0, 0), .11, ramp=self._ramp, value=self._ramp.low)
            name = self.name(i, size=13, width=1.6, limit=14).move_to([slots[i], 2.7, 0])
            dots[i] = mn.Dot([X(costs[i]), y_axis, 0], radius=.075, color=COLORS["cost"])
            leader = mn.Line([slots[i], 1.68, 0], [X(costs[i]), y_axis + .08, 0], color=COLORS["cost"], stroke_width=1.6, stroke_opacity=.7)
            tag = text(self.number(costs[i]), (X(costs[i]), y_axis - .3 - .28 * (k % 2), 0), size=S0, color=INK)
            groups.add(thumb, name, dots[i], leader, tag)
        zone_x = X(BUDGET_CHF)
        zone = mn.Rectangle(width=xr + .15 - zone_x, height=1.15, stroke_width=0, fill_color=WARN, fill_opacity=.16).move_to([(zone_x + xr + .15) / 2, y_axis - .15, 0])
        limit = mn.VGroup(mn.DashedLine([zone_x, y_axis + .45, 0], [zone_x, y_axis - .16, 0], color=WARN, stroke_width=3),
                          _room.chip([self.tr("client budget", "Budget des Bauherrn"), _room.Number(self.number(BUDGET_CHF)), "CHF"], (zone_x, y_axis - 1.1, 0), size=14, color=WARN),
                          text(self.tr("not allowed", "nicht erlaubt"), ((zone_x + xr + .15) / 2 + .35, y_axis - .62, 0), size=S0, color=WARN))
        crosses = mn.VGroup(*[_art.cross_mark(color=BAD, size=.2).move_to([slots[i], 2.0, 0]) for i in over])
        note = _room.place(text(self.tr("normalized over the allowed designs", "normiert über die erlaubten Entwürfe"), size=14, color=MUTED), (LEFT, -1.0, 0), "left")
        rank_rows = mn.VGroup(note)
        passing = sorted(allowed, key=lambda i: rows[i]["score"])
        for k, i in enumerate(passing):
            y = -1.4 - .42 * k
            thumb = self.thumbnail(by_id(i)["design"], (-6.5, y, 0), .08, ramp=self._ramp, value=self._ramp.low)
            name = self.name(i, size=13, width=1.6, limit=18).move_to([-5.45, y, 0])
            cursor, bar = -4.2, mn.VGroup()
            for j, c in enumerate(ORDER):
                part = 1.9 * self._weights[j] * rows[i]["normalized"][c]
                bar.add(mn.Rectangle(width=max(part, 1e-3), height=.28, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85).move_to([cursor + part / 2, y, 0]))
                cursor += part
            score = _room.place(text(self.number(rows[i]["score"], 2), size=S1), (cursor + .12, y, 0), "left")
            price = _room.place(text(f"{self.number(costs[i])} CHF", size=S0, color=COLORS["cost"]), (-1.55, y, 0), "left")
            rank_rows.add(thumb, name, bar, score, price)
        frame = mn.Rectangle(width=RIGHT - LEFT, height=.4, color=GOOD, stroke_width=3).move_to([(LEFT + RIGHT) / 2, -1.4, 0])
        verdict = _room.chip([_room.Number(self.number(costs[winner])), "CHF", "<", _room.Number(self.number(BUDGET_CHF)), "CHF"], (RIGHT - 1.4, -1.0, 0), size=14, color=GOOD)
        self.step("limits", mn.FadeOut(record), mn.Succession(
            mn.AnimationGroup(mn.FadeIn(flow), mn.FadeIn(axis), mn.FadeIn(groups), self.active.animate(run_time=.3).set_value(1)),
            mn.AnimationGroup(mn.FadeIn(zone), mn.Create(limit[0]), mn.FadeIn(limit[1]), mn.FadeIn(limit[2])),
            mn.AnimationGroup(mn.Create(crosses), *[dots[i].animate.set_color(BAD) for i in over])), seconds=4.4)
        self.step("limits_2", mn.AnimationGroup(mn.FadeIn(rank_rows), mn.Create(frame), mn.FadeIn(verdict)), seconds=2.8)
