
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

    def screen_sweep(self):
        """🔀 Rank of every variant against the cost weight, with the winner strip, the switch shares and the room of the current winner."""
        self.clear_stage()
        pool = ids()
        shares = [2.5 * k for k in range(41)]
        base = _model.load_study()["weights"]["balanced"]
        ranks = {i: [] for i in pool}
        gaps = []
        for share in shares:
            weights = tuple(_model.reweigh(base, "cost", share / 100)[c] for c in ORDER)
            rows = table(weights)
            for i in pool:
                ranks[i].append(rows[i]["rank"])
            scores = sorted(row["score"] for row in rows.values())
            gaps.append(scores[1] - scores[0])
        x0, x1, top, step = -4.4, 0.7, 1.55, .72
        X = lambda share: x0 + (x1 - x0) * share / 100
        Y = lambda rank: top - (rank - 1) * step
        static = mn.VGroup()
        for rank in range(1, 6):
            static.add(mn.Line([x0, Y(rank), 0], [x1, Y(rank), 0], color=MUTED, stroke_width=1, stroke_opacity=.35), text(str(rank), (x0 - .25, Y(rank), 0), size=S0, color=MUTED))
        static.add(mn.Line([x0, Y(5) - .4, 0], [x1, Y(5) - .4, 0], color=INK, stroke_width=2))
        for share in (0, 25, 50, 75, 100):
            static.add(mn.Line([X(share), Y(5) - .46, 0], [X(share), Y(5) - .34, 0], color=INK, stroke_width=2), text(str(share), (X(share), Y(5) - .66, 0), size=S0))
        static.add(text(self.tr("rank", "Rang"), (x0 - .25, Y(5) - .45, 0), size=S0, color=MUTED),
                   _room.place(text(self.tr("cost weight (%)", "Kostengewicht (%)"), size=S0, color=MUTED), (x1 + .3, Y(5) - .66, 0), "left"),
                   _room.place(text(self.tr("at 0 %", "bei 0 %"), size=S0, color=MUTED), (x0 - .55, Y(1) + .62, 0), "right"),
                   _room.place(text(self.tr("at 100 %", "bei 100 %"), size=S0, color=MUTED), (x1 + .3, Y(1) + .62, 0), "left"))
        lines_ = mn.VGroup(*[mn.VMobject(color=INK, stroke_width=2.4, stroke_opacity=.55).set_points_as_corners([[X(s), Y(r), 0] for s, r in zip(shares, ranks[i])]) for i in pool])
        names = mn.VGroup()
        for i in pool:
            names.add(_room.place(self.name(i, size=14, width=1.7), (x0 - .55, Y(ranks[i][0]), 0), "right"),
                      _room.place(self.name(i, size=14, width=1.7), (x1 + .3, Y(ranks[i][-1]), 0), "left"))
        strip, winners = mn.VGroup(), [next(i for i in pool if ranks[i][k] == 1) for k in range(len(shares))]
        half = (x1 - x0) / (len(shares) - 1) / 2
        for winner, first, last in runs_of(winners):
            a, b = max(x0, X(shares[first]) - half), min(x1, X(shares[last]) + half)
            strip.add(mn.Line([a + .03, Y(1) + .16, 0], [b - .03, Y(1) + .16, 0], color=GOOD, stroke_width=6),
                      self.thumbnail(by_id(winner)["design"], ((a + b) / 2, Y(1) + .5, 0), .075, ramp=self._ramp, value=self._ramp.low, ring=GOOD))
            if b - a > 1.5:
                strip.add(self.name(winner, size=13, width=b - a - .1, limit=40).move_to([(a + b) / 2, Y(1) + .88, 0]))
        for crossing in self.switch_shares(base):
            strip.add(mn.DashedLine([X(crossing), Y(1) + .16, 0], [X(crossing), Y(5) - .4, 0], color=GOOD, stroke_width=2, stroke_opacity=.7),
                      _room.chip([_room.Number(self.number(crossing, 0)), "%"], (X(crossing), Y(1) + .16, 0), size=S0, color=GOOD))
        close = mn.VGroup()
        tight = [gap < .03 for gap in gaps]
        for k, flag in enumerate(tight):
            if flag:
                close.add(mn.Rectangle(width=X(2.5) - x0, height=Y(1) - Y(5) + .5, stroke_width=0, fill_color=WARN, fill_opacity=.2).move_to([X(shares[k]), (Y(1) + Y(5)) / 2, 0]))
        key = mn.VGroup(mn.Rectangle(width=.3, height=.22, stroke_width=1.4, color=WARN, fill_color=WARN, fill_opacity=.3),
                        text(self.tr("close call: index gap below 0.03", "knapp: Indexabstand unter 0,03"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.15)
        key.move_to([x1 + .3 + key.width / 2, Y(5) - 1.05, 0])
        close.add(key)
        run = max(runs_of(tight), key=lambda item: (item[0], item[2] - item[1]), default=(True, 0, 0))
        close_share = shares[(run[1] + run[2]) // 2]
        share = mn.ValueTracker(self._heavy[ORDER.index("cost")] * 100)

        def champion():
            weights = tuple(_model.reweigh(base, "cost", share.get_value() / 100)[c] for c in ORDER)
            return winner_of(weights)
        rt = self.study_trackers(by_id(champion())["design"])
        for name in ("orientation", "wwr", "overhang", "insulation"):
            rt[name].add_updater(lambda m, name=name: m.set_value(by_id(champion())["design"][name]))
        self.add(share, *rt.values())

        def marker():
            weights = tuple(_model.reweigh(base, "cost", share.get_value() / 100)[c] for c in ORDER)
            rows = table(weights)
            x = X(share.get_value())
            group = mn.VGroup(mn.Line([x, Y(1) + .3, 0], [x, Y(5) - .4, 0], color=INK, stroke_width=2.4))
            for i in pool:
                group.add(mn.Dot([x, Y(rows[i]["rank"]), 0], radius=.09, color=GOOD if rows[i]["rank"] == 1 else INK))
            chip = _room.chip([self.tr("cost weight", "Kostengewicht"), _room.Number(self.number(share.get_value(), 0)), "%"], (x, Y(5) - 1.05, 0), size=S0, color=COLORS["cost"])
            chip.shift(mn.RIGHT * (max(0, x0 - 1.0 - chip.get_left()[0]) - max(0, chip.get_right()[0] - (x1 + 1.0))))
            group.add(chip)
            return group
        panel = self.variant_panel(rt, champion)
        self.step("sweep", mn.FadeIn(static), mn.Create(lines_), mn.FadeIn(names), mn.FadeIn(strip), mn.FadeIn(live(marker)), mn.FadeIn(panel),
                  mn.Succession(mn.Wait(.8), share.animate(run_time=1.4).set_value(0), share.animate(run_time=2.0).set_value(100), share.animate(run_time=1.0).set_value(self._heavy[ORDER.index("cost")] * 100)), seconds=5.4)
        self.step("sweep_2", mn.FadeIn(close), share.animate(run_time=1.0).set_value(close_share), seconds=1.2)
        self.sweep_share = share

    def screen_range(self):
        """➕ A sixth variant stretches the cost ruler, squeezes the others and flips the order of two of them."""
        self.clear_stage()
        five, six = ids(), ids(True)
        extra = six[-1]
        ruler = Ruler(self, "cost", six, five, state="normalized", name_size=13, rings=False)
        self._rt = self.study_trackers(by_id(extra)["design"])
        header = self.ruler_header(ruler)
        panel = self.panel("cost", ruler)
        ruler.scan = ruler.scan + [extra]
        ruler.focus.set_value(len(ruler.scan) - 1)
        worst_old = max(five, key=lambda i: ruler.values[i])
        frames = {"best": self.slot_frame(ruler, ruler.scan[0], GOOD), "old": self.slot_frame(ruler, worst_old, WARN), "new": self.slot_frame(ruler, extra, WARN)}
        ghosts = mn.VGroup(*[mn.DashedVMobject(mn.Circle(radius=.09, color=MUTED, stroke_width=2).move_to([ruler.x_of(ruler.values[i]), ruler.line_y, 0]), num_dashes=10) for i in five])
        hint = text(self.tr(f"{self.number(ruler.fraction(extra), 2)} > 1: beyond the old range", f"{self.number(ruler.fraction(extra), 2)} > 1: außerhalb des alten Bereichs"),
                    (ruler.x0 + ruler.length - 1.6, ruler.line_y - 1.55, 0), size=S0, color=WARN)
        ruler.ring.update()
        self.add(ruler.ticks, ruler.ring, *frames.values())
        self.step("range", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(five)), mn.FadeIn(ghosts), frames["best"].level.animate.set_value(1.0), frames["old"].level.animate.set_value(1.0),
                  mn.Succession(mn.Wait(.8), mn.FadeIn(ruler.mobjects([extra])), mn.AnimationGroup(mn.FadeIn(panel), mn.FadeIn(hint), ruler.tween(.6, o_ring=1.0))), seconds=3.0)
        span = max(ruler.values[i] for i in six) - min(ruler.values[i] for i in six)
        self.step("range_2", ruler.stretch(2.5, span=span), frames["old"].level.animate(run_time=1.2).set_value(0.0), frames["new"].level.animate(run_time=1.2).set_value(1.0), mn.FadeOut(hint), seconds=2.5)
        self.ruler = ruler
        order = sorted(five, key=lambda i: table(self._weights)[i]["rank"])
        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.58)
        for tracker in (board.fill, board.compact, board.scores, board.badges, board.win, board.mode):
            tracker.set_value(1.0)
        board.numbers.set_value(0.0)
        new_ranks = {i: row["rank"] for i, row in table(self._weights, True).items()}
        old_ranks = {i: row["rank"] for i, row in table(self._weights).items()}
        axis = self.index_axis(board)
        notes = mn.VGroup()
        for i in six:
            before = str(old_ranks[i]) if i in old_ranks else self.tr("new", "neu")
            note = text(f"{before} → {new_ranks[i]}", (5.9, 0, 0), size=S1, color=WARN if i in old_ranks and old_ranks[i] != new_ranks[i] else INK)
            note.identifier = i
            note.add_updater(lambda m: (m.move_to([5.9, board.y(m.identifier), 0]), m.set_opacity(board.row_o[m.identifier].get_value())))
            cost = text(f"{self.number(raw(i, 'cost'))} CHF", (4.0, 0, 0), size=S0, color=COLORS["cost"])
            cost.identifier = i
            cost.add_updater(lambda m: (m.move_to([4.0, board.y(m.identifier), 0]), m.set_opacity(board.row_o[m.identifier].get_value())))
            notes.add(note, cost)
        flipped = [i for i in five if (sorted(five, key=old_ranks.get).index(i) != sorted([f for f in six if f in five], key=new_ranks.get).index(i))]
        heading = text(self.tr("rank 5 → 6", "Rang 5 → 6"), (5.9, 1.9, 0), size=S1, color=MUTED)
        cost_heading = text(self.tr("façade cost", "Fassadenkosten"), (4.0, 1.9, 0), size=S1, color=COLORS["cost"])
        link = None
        if len(flipped) >= 2:
            ya, yb = (board.y0 - (new_ranks[i] - 1) * board.pitch for i in flipped[:2])
            link = mn.DoubleArrow([6.7, ya, 0], [6.7, yb, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)
        board_parts = mn.VGroup(board.live, axis, notes, heading, cost_heading)
        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(ghosts), mn.FadeOut(header), mn.FadeOut(panel), mn.FadeOut(ruler.members[extra]["slot"]),
                                *[frame.level.animate(run_time=.5).set_value(0.0) for frame in frames.values()],
                                *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag", "norm")])
        moves, swaps = self.glide([ruler.members[i]["slot"] for i in five], [board.labels[i] for i in five])
        fold = mn.Succession(mn.AnimationGroup(*moves), mn.AnimationGroup(*swaps, mn.FadeIn(board_parts)))
        reorder = mn.AnimationGroup(board.t.animate(run_time=1.6).set_value(1.0), board.row_o[extra].animate(run_time=1.6).set_value(1.0),
                                    board.rank_to(new_ranks, 2.0), mn.FadeIn(board.labels[extra]))
        steps = [out, fold, reorder] + ([mn.Create(link, run_time=.7)] if link is not None else [])
        self.step("range_3", mn.Succession(*steps), seconds=5.2)
        self.final_board = board

    def screen_record(self):
        """🗂️ The decision record, the compensation limit and the bridge to Pareto fronts."""
        self.clear_stage()
        german = self.language() == "de"
        pool, key = ids(), self._weights
        rows = table(key)
        winner = winner_of(key)
        left = lambda item, x, y: _room.place(item, (x, y, 0), "left")
        card = _art.window_frame(12.6, 4.9, color=INK)
        parts = mn.VGroup()
        heading = lambda label, x, y: left(text(label, size=S1, color=MUTED), x, y)
        parts.add(heading(self.tr("Weights", "Gewichte"), -5.9, 2.0))
        for k, c in enumerate(ORDER):
            line = mn.VGroup(glyph(c, width=.28), text(f"{SHORT[c][1 if german else 0]} {self.number(key[k] * 100)} %", size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.15)
            parts.add(left(line, -5.9, 1.5 - .4 * k))
        parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), -5.9, -.2))
        for k, c in enumerate(ORDER):
            best = best_id(c, pool)
            worst = max(pool, key=lambda i: raw(i, c) * (1 if CRITERIA[c]["direction"] == "min" else -1))
            line = mn.VGroup(glyph(c, width=.28), text(f"{self.value_text(c, raw(best, c))} … {self.value_text(c, raw(worst, c))} {self.unit_label(c)}", size=S0, color=COLORS[c])).arrange(mn.RIGHT, buff=.15)
            parts.add(left(line, -5.9, -.7 - .4 * k))
        parts.add(heading(self.tr("Compared variants", "Verglichene Varianten"), -1.2, 2.0))
        for k, i in enumerate(pool):
            x = 1.4 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, 1.2, 0), .13, ramp=self._ramp, value=self._ramp.low), self.name(i, size=13, width=1.3).move_to([x, .6, 0]))
        parts.add(heading(self.tr("Result", "Ergebnis"), -1.2, -.2),
                  self.thumbnail(by_id(winner)["design"], (-.2, -1.25, 0), .22, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), 1.2, -1.0),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), 1.2, -1.6))
        record = mn.VGroup(card, parts)
        self.step("record", mn.FadeIn(record), seconds=2.5)
        self.screen_limits(record, winner)
        self.clear_stage()
        costs, powers = [raw(i, "cost") for i in pool], [raw(i, "electricity") for i in pool]
        xs, ys = nice_step(max(costs) - min(costs), 4), nice_step(max(powers) - min(powers), 4)
        x_range = (math.floor(min(costs) / xs - .3) * xs, math.ceil(max(costs) / xs + .3) * xs, xs)
        y_range = (math.floor(min(powers) / ys - .3) * ys, math.ceil(max(powers) / ys + .3) * ys, ys)
        axes, labels = self.plot_axes(x_range=x_range, y_range=y_range, x_label=self.tr("Façade cost, CHF", "Fassadenkosten, CHF"),
                                      y_label=self.tr("Electricity, kWh/(m²·a)", "Strombedarf, kWh/(m²·a)"), point=(-.4, -.1, 0), width=8.4, height=3.4,
                                      y_digits=0 if ys >= 1 else 1)
        free = [i for i in pool if not any(raw(j, "cost") <= raw(i, "cost") and raw(j, "electricity") <= raw(i, "electricity") and j != i for j in pool)]
        marks = mn.VGroup(*[self.thumbnail(by_id(i)["design"], axes.c2p(raw(i, "cost"), raw(i, "electricity")), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD if i in free else None) for i in pool])
        tags = mn.VGroup()
        for k, i in enumerate(sorted(pool, key=lambda j: raw(j, "cost"))):
            spot = axes.c2p(raw(i, "cost"), raw(i, "electricity"))
            tag = self.name(i, size=13, width=1.5, limit=14)
            tags.add(tag.move_to(spot + mn.UP * (.62 if k % 2 == 0 else -.62) + mn.RIGHT * .1))
        front = sorted((raw(i, "cost"), raw(i, "electricity")) for i in free)
        outline = mn.DashedVMobject(mn.VMobject(color=GOOD, stroke_width=3).set_points_as_corners([axes.c2p(*p) for p in front]), num_dashes=24)
        chip = _art.window_chip(text(self.tr("Pareto front", "Pareto-Front"), size=S2, color=GOOD), color=GOOD).move_to([4.9, 1.9, 0])
        self.step("next", mn.FadeIn(axes), mn.FadeIn(labels), mn.FadeIn(marks), mn.FadeIn(tags), mn.Create(outline), mn.FadeIn(chip), seconds=2.4)

    def screen_limits(self, record, winner):
        """🚧 A mandatory cap on the façade cost removes variants before any weight is applied; the rest are ranked and the winner's room is shown."""
        pool, key = ids(), self._weights
        rows = table(key)
        left = lambda item, x, y: _room.place(item, (x, y, 0), "left")
        costs = {i: raw(i, "cost") for i in pool}
        cap = math.ceil(costs[winner] / CAP_STEP) * CAP_STEP
        over = [i for i in pool if costs[i] > cap]
        low, high = math.floor(min(costs.values()) / 1000) * 1000, math.ceil(max(costs.values()) / 1000) * 1000
        xl, xr, y_axis = -6.2, 1.6, .2
        X = lambda value: xl + (value - low) / (high - low) * (xr - xl)
        flow = mn.VGroup(_art.window_chip(text(self.tr("Constraints: filter", "Nebenbedingungen: filtern"), size=S1, color=WARN), color=WARN), mn.Arrow(mn.LEFT * .4, mn.RIGHT * .4, buff=0, color=INK),
                         _art.window_chip(text(self.tr("Weights: rank", "Gewichte: ordnen"), size=S1, color=GOOD), color=GOOD)).arrange(mn.RIGHT, buff=.3)
        flow.move_to([-2.4, 2.4, 0])
        axis = mn.VGroup(mn.Line([xl - .15, y_axis, 0], [xr + .15, y_axis, 0], color=INK, stroke_width=2.4),
                         text("CHF", (xr + .5, y_axis, 0), size=S0, color=COLORS["cost"]))
        for value in range(low, high + 1, 1000):
            axis.add(mn.Line([X(value), y_axis - .07, 0], [X(value), y_axis + .07, 0], color=INK, stroke_width=2), text(self.number(value), (X(value), y_axis - .28, 0), size=S0))
        order = sorted(pool, key=costs.get)
        gap = (xr - xl - 1.2) / (len(order) - 1)
        slots = {i: xl + .6 + k * gap for k, i in enumerate(order)}
        dots, groups = {}, mn.VGroup()
        for k, i in enumerate(order):
            thumb = self.thumbnail(by_id(i)["design"], (slots[i], 1.3, 0), .13, ramp=self._ramp, value=self._ramp.low)
            name = self.name(i, size=13, width=1.45, limit=14).move_to([slots[i], 1.88, 0])
            dots[i] = mn.Dot([X(costs[i]), y_axis, 0], radius=.075, color=COLORS["cost"])
            leader = mn.Line([slots[i], .98, 0], [X(costs[i]), y_axis + .08, 0], color=COLORS["cost"], stroke_width=1.6, stroke_opacity=.7)
            tag = text(self.number(costs[i]), (X(costs[i]), y_axis - .58 - .28 * (k % 2), 0), size=S0, color=INK)
            groups.add(thumb, name, dots[i], leader, tag)
        zone_x = X(cap)
        zone = mn.Rectangle(width=xr + .15 - zone_x, height=1.1, stroke_width=0, fill_color=WARN, fill_opacity=.16).move_to([(zone_x + xr + .15) / 2, y_axis + .55 - .05, 0])
        limit = mn.VGroup(mn.DashedLine([zone_x, y_axis - .12, 0], [zone_x, y_axis + 1.05, 0], color=WARN, stroke_width=3),
                          _room.chip([self.tr("limit", "Grenze"), _room.Number(self.number(cap)), "CHF"], (zone_x, -1.0, 0), size=S0, color=WARN),
                          text(self.tr("not allowed", "nicht erlaubt"), ((zone_x + xr) / 2, y_axis + .74, 0), size=S0, color=WARN))
        crosses = mn.VGroup(*[_art.cross_mark(color=BAD, size=.2).move_to([slots[i], 1.3, 0]) for i in over])
        rank_rows = mn.VGroup()
        passing = sorted([i for i in pool if i not in over], key=lambda i: rows[i]["score"])
        for k, i in enumerate(passing):
            y = -1.5 - .45 * k
            thumb = self.thumbnail(by_id(i)["design"], (-6.0, y, 0), .09, ramp=self._ramp, value=self._ramp.low)
            name = self.name(i, size=13, width=1.6, limit=40).move_to([-4.95, y, 0])
            cursor, bar = -3.7, mn.VGroup()
            for j, c in enumerate(ORDER):
                part = 3.0 * self._weights[j] * rows[i]["normalized"][c]
                bar.add(mn.Rectangle(width=max(part, 1e-3), height=.3, stroke_width=0, fill_color=COLORS[c], fill_opacity=.85).move_to([cursor + part / 2, y, 0]))
                cursor += part
            score = left(text(self.number(rows[i]["score"], 2), size=S1), cursor + .15, y)
            price = left(text(f"{self.number(costs[i])} CHF", size=S0, color=COLORS["cost"]), 1.0, y)
            rank_rows.add(thumb, name, bar, score, price)
        frame = mn.Rectangle(width=9.5, height=.42, color=GOOD, stroke_width=3).move_to([-1.75, -1.5, 0])
        rt = self.study_trackers(by_id(winner)["design"])
        panel = self.variant_panel(rt, lambda: winner)
        verdict = _room.chip([_room.Number(self.number(costs[winner])), "CHF", "<", _room.Number(self.number(cap)), "CHF"], (5.25, -2.25, 0), size=S0, color=GOOD)
        self.step("limits", mn.FadeOut(record), mn.Succession(
            mn.AnimationGroup(mn.FadeIn(flow), mn.FadeIn(axis), mn.FadeIn(groups)),
            mn.AnimationGroup(mn.FadeIn(zone), mn.Create(limit[0]), mn.FadeIn(limit[1]), mn.FadeIn(limit[2])),
            mn.AnimationGroup(mn.Create(crosses), *[dots[i].animate.set_color(WARN) for i in over]),
            mn.AnimationGroup(mn.FadeIn(rank_rows), mn.Create(frame), mn.FadeIn(panel), mn.FadeIn(verdict))), seconds=6.0)
