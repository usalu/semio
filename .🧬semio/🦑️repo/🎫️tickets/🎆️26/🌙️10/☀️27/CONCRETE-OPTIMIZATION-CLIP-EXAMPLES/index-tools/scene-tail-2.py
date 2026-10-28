
    def screen_rulers(self):
        """📏 One ruler per criterion: real values, subtract the best, divide by the span; then the table."""
        pool = ids()
        self.clear_stage()
        ruler = Ruler(self, "electricity", pool, pool)
        self._rt = self.study_trackers(by_id(ruler.scan[0])["design"])
        header = self.ruler_header(ruler)
        panel = self.panel("electricity", ruler)
        self.add(ruler.ticks)
        self.step("ruler", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(pool)), mn.FadeIn(panel), ruler.appear(1.2), seconds=2.2)
        self.add(ruler.ring)
        last = len(ruler.scan) - 1
        self.step("ruler_2", mn.Succession(ruler.tween(.3, o_ring=1.0), ruler.visit(list(range(1, last + 1)) + [0], hold=.4, travel=.6, rt=self._rt)), seconds=5.0)
        note = self.step_note(ruler, "−", ruler.best)
        self.step("shift", ruler.pan(2.0), mn.FadeIn(note), seconds=2.0)
        note2 = self.step_note(ruler, "÷", ruler.high - ruler.low)
        zero_one = text(self.tr("0 = best of the set · 1 = worst of the set", "0 = Bester der Menge · 1 = Schlechtester der Menge"), (ruler.center, -1.72, 0), size=S0, color=MUTED)
        self.step("shift_2", ruler.stretch(2.0), mn.ReplacementTransform(note, note2), mn.FadeIn(zero_one), seconds=2.0)
        brace = _art.window_chip(text(self.tr(f"compared set: {len(pool)} variants", f"Vergleichsmenge: {len(pool)} Varianten"), size=S1), color=INK).move_to([ruler.center, -2.28, 0])
        self.step("mapped", mn.FadeOut(note2), mn.FadeIn(brace), ruler.visit([0, last], hold=.5, travel=.8, rt=self._rt), seconds=2.4)
        for key in ("cost", "daylight"):
            new, new_header, new_panel, run = self.next_ruler(key, ruler, header, panel, [brace])
            self.step(key, mn.Succession(*run), seconds=3.6)
            self.add(new.ring)
            ruler, header, panel, brace = new, new_header, new_panel, None
        new, new_header, new_panel, run = self.next_ruler("carbon", ruler, header, panel, [])
        board = Board(self, pool)
        board.cur.set_value(1.0)
        legend = self.table_legend()
        self._legend, self.board = legend, board
        board_panel = self.variant_panel(self._rt, lambda: board.cursor()[0])
        self.board_panel = board_panel
        moves, swaps = self.glide([new.members[i]["slot"] for i in pool], [board.labels[i] for i in pool])
        fold = mn.Succession(mn.AnimationGroup(mn.FadeOut(new.line), mn.FadeOut(new.ticks), mn.FadeOut(new_header), mn.FadeOut(new_panel), mn.FadeOut(zero_one),
                                               *[mn.FadeOut(new.members[i][k]) for i in pool for k in ("dot", "leader", "tag", "norm")], *moves),
                             mn.AnimationGroup(*swaps, mn.FadeIn(board.live), mn.FadeIn(legend), mn.FadeIn(board_panel), board.fill.animate(run_time=2.0).set_value(1.0)))
        self.step("matrix", mn.Succession(*run, fold), seconds=5.6)
        self.ruler = new

    def step_note(self, ruler, sign, value):
        """🧾 The operation of a normalization step, e.g. − best value with its unit."""
        label = f"{sign} {self.number(value, DIGITS[ruler.criterion])} {self.unit_label(ruler.criterion)}"
        return text(label, (ruler.center, -2.05, 0), size=S1, color=ruler.color)

    def ruler_header(self, ruler):
        """🏷️ Criterion pictogram, title, unit and direction."""
        c = ruler.criterion
        direction = self.tr("lower is better ↓", "weniger ist besser ↓") if CRITERIA[c]["direction"] == "min" else self.tr("higher is better ↑", "mehr ist besser ↑")
        title = CRITERIA[c]["title"][self.language()]
        items = mn.VGroup(glyph(c, width=.36), text(f"{title}, {self.unit_label(c)}", size=S1, color=ruler.color), text(direction, size=S1)).arrange(mn.RIGHT, buff=.25)
        return items.move_to([-6.7 + items.width / 2, 2.38, 0])

    def next_ruler(self, criterion, old, header, panel, extras):
        """🔁 Replace the ruler by the next criterion; returns the new parts and the animations (flip first for daylight)."""
        pool = ids()
        new = Ruler(self, criterion, pool, pool)
        new_header, new_panel = self.ruler_header(new), self.panel(criterion, new)
        for member in old.members.values():
            member["slot"].clear_updaters()
        keep = ("dot", "leader", "tag", "norm")
        swap = mn.AnimationGroup(
            *[mn.ReplacementTransform(old.members[i]["slot"], new.members[i]["slot"]) for i in pool],
            mn.FadeOut(old.line), mn.FadeOut(old.ticks), mn.FadeOut(old.ring), *[mn.FadeOut(e) for e in extras],
            *[mn.FadeOut(old.members[i][k]) for i in pool for k in keep], mn.ReplacementTransform(header, new_header),
            mn.FadeOut(panel), mn.FadeIn(new_panel), mn.FadeIn(new.line), *[mn.FadeIn(new.members[i][k]) for i in pool for k in keep],
            new.appear(1.0))
        self.add(new.ticks)
        run = [swap, mn.AnimationGroup(new.pan(1.0), self.set_design(self._rt, by_id(new.scan[0])["design"], 1.0)), new.stretch(1.0)]
        if CRITERIA[criterion]["direction"] == "max":
            run.insert(1, new.flip(1.6))
        return new, new_header, new_panel, run

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
        words = mn.VGroup(text(self.tr("Normalized value:", "Normierter Wert:"), size=S0),
                          text(self.tr("0 = best of the set", "0 = Bester der Menge"), size=S0, color=GOOD),
                          text(self.tr("1 = worst of the set", "1 = Schlechtester der Menge"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.3)
        return words.move_to([-.7, -1.75, 0])

    def panel(self, kind, ruler):
        """🪟 The room panel: what the focused variant means for the criterion."""
        rt, german = self._rt, self.language() == "de"
        maximum = {"carbon": max(raw(i, "carbon") for i in ids(True))}
        current = ruler.current
        group = mn.VGroup(live(lambda: mn.VGroup(_room.text(by_id(current())["title"][self.language()].split(), (5.15, 2.38, 0), size=S1, width=2.9)), current))
        found = lambda: _room.result(_room.values(rt))
        design_key = _room.design_key(rt)
        if kind == "electricity":
            def conversion():
                result = found()
                return mn.VGroup(_room.text([_room.Number(self.number(result["electricity"], 0)), "kWh/a", "÷", str(FLOOR_AREA), "m²", "=",
                                             _room.Number(self.number(result["intensity"], 2)), "kWh/(m²·a)"], (4.9, -2.62, 0), size=S0, color=INK, width=3.8))
            group.add(self.room_section(rt, (4.75, 1.15, 0), .3, "summer", labels=False, demand=False),
                      self.energy_bars(rt, (3.25, -2.0, 0), 1.7, self._maximum, width=.65, labels=True, size=14), live(conversion, design_key))
        elif kind == "cost":
            def price():
                result = found()
                row = mn.VGroup(glyph("cost", width=.4), _room.text([_room.Number(self.number(result["cost"], 0)), "CHF"], size=24, color=COLORS["cost"])).arrange(mn.RIGHT, buff=.2)
                area = _room.text([*self.tr("façade 24 m² · window", "Fassade 24 m² · Fenster").split(), _room.Number(self.number(result["window_area"], 1)), "m²"], size=S0, color=INK)
                return mn.VGroup(row.move_to([4.75, -.6, 0]), area.move_to([4.75, -1.3, 0]))
            group.add(self.elevation(rt, (4.75, .85, 0), .4), live(price, design_key))
        elif kind == "daylight":
            def levels():
                result = found()
                big = mn.VGroup(glyph("daylight", width=.4), _room.text([self.tr("factor", "Quotient"), _room.Number(self.number(result["daylight"], DIGITS["daylight"])), "%"], size=24, color=COLORS["daylight"])).arrange(mn.RIGHT, buff=.2)
                words = ["Autonomie:", "Lampen", "aus"] if german else ["Autonomy:", "lamps", "off"]
                cover = _room.text([*words, _room.Number(self.number(result["autonomy"] * 100, 0)), "%", *(["der", "Lichtstunden"] if german else ["of", "lighting", "hours"])], size=S0, color=INK, width=3.8)
                return mn.VGroup(big.move_to([4.75, -.55, 0]), cover.move_to([4.75, -1.2, 0]))
            group.add(self.room_section(rt, (4.75, 1.15, 0), .3, "summer", labels=False, demand=False), live(levels, design_key))
        else:
            def cloud():
                result = found()
                share = result["carbon"] / maximum["carbon"]
                puffs = mn.VGroup(*[mn.Circle(radius=.1 + .32 * share * (1 + .25 * k), stroke_width=0, fill_color=COLORS["carbon"], fill_opacity=.5)
                                    .move_to([3.55 + .3 * k, -.9 + .35 * k + .1 * share, 0]) for k in range(3)])
                number = _room.text([_room.Number(self.number(result["carbon"], 0)), "kg", "CO₂e", self.tr("embodied", "grau")], size=22, color=COLORS["carbon"])
                return mn.VGroup(puffs, number.move_to([5.2, -1.7, 0]))
            group.add(self.elevation(rt, (4.75, .85, 0), .4), live(cloud, design_key), glyph("carbon", (3.2, -1.7, 0), .9))
        return group

    def screen_board(self):
        """⚖️ Weights, the index bar, the winner's room and a cost-driven re-weighting; the focus row moves the room."""
        board, rt = self.board, self._rt
        ws = self._weights
        self.step("weights", board.mode.animate(run_time=1.2).set_value(1.0), seconds=1.4)
        self.step("weights_2", board.weights_to(ws, 2.5), seconds=2.6)
        wide, narrow = max(range(4), key=lambda j: ws[j]), min(range(4), key=lambda j: ws[j])
        outlines = mn.VGroup(*[shape(mn.Rectangle(width=ws[j] * board.width, height=3.2, color=INK, stroke_width=3).move_to(
            [board.xl + board.width * (sum(ws[:j]) + ws[j] / 2), .1, 0])) for j, shape in ((wide, lambda r: r), (narrow, lambda r: mn.DashedVMobject(r, num_dashes=40)))])
        self.step("weights_3", mn.Create(outlines), board.visit(list(range(1, len(board.scan))) + [0], .3, rt), seconds=2.2)
        axis = self.index_axis(board)
        self.step("index", mn.FadeOut(outlines), board.compact.animate(run_time=2.0).set_value(1.0), board.numbers.animate(run_time=1.0).set_value(0.0),
                  board.scores.animate(run_time=1.4).set_value(1.0), mn.FadeIn(axis), mn.FadeOut(self._legend), seconds=2.2)
        key = tuple(round(v, 6) for v in board.weights())
        ranks = {i: row["rank"] for i, row in table(key).items()}
        winner = winner_of(key)
        self.step("index_2", board.rank_to(ranks, 2.0), board.badges.animate(run_time=1.2).set_value(1.0), board.win.animate(run_time=1.2).set_value(1.0),
                  board.cur.animate(run_time=1.6).set_value(0.0), board.visit([board.scan.index(winner)], 2.0, rt), seconds=2.4)
        panel = self.board_panel
        self.step("winner", mn.Indicate(panel[0], scale_factor=1.12, color=GOOD),
                  mn.Succession(board.win.animate(run_time=.4).set_value(.25), board.win.animate(run_time=.6).set_value(1.0)), seconds=1.6)
        state = table(key)[winner]["normalized"]
        weakest = max(range(4), key=lambda j: state[ORDER[j]])
        cursor = board.xl + board.width * sum(ws[j] * state[ORDER[j]] for j in range(weakest))
        width = board.width * ws[weakest] * state[ORDER[weakest]]
        mark = mn.Rectangle(width=max(width, .06) + .08, height=.46, color=WARN, stroke_width=3).move_to([cursor + width / 2, board.y0, 0])
        name = SHORT[ORDER[weakest]][1 if self.language() == "de" else 0]
        note = mn.VGroup(glyph(ORDER[weakest], width=.28, color=WARN), text(self.tr(f"weakest: {name} {self.number(state[ORDER[weakest]], 2)}", f"schwächstes: {name} {self.number(state[ORDER[weakest]], 2)}"),
                                                                           size=S0, color=WARN)).arrange(mn.RIGHT, buff=.15).move_to([5.25, -2.25, 0])
        self.step("winner_2", mn.Create(mark), mn.FadeIn(note), seconds=1.6)
        persona = mn.VGroup(glyph("cost", width=.4, color=WARN), text(self.tr("cost-driven client", "kostenorientierter Bauherr"), size=S1, color=WARN)).arrange(mn.RIGHT, buff=.25).move_to([5.25, -2.25, 0])
        board.old_id = winner
        self.step("sensitivity", mn.FadeOut(mark), mn.FadeOut(note), mn.FadeIn(persona), board.weights_to(self._heavy, 3.0),
                  board.win.animate(run_time=1.0).set_value(0.0), board.old.animate(run_time=1.0).set_value(1.0), seconds=3.0)
        heavy_key = tuple(round(v, 6) for v in board.weights())
        heavy_ranks = {i: row["rank"] for i, row in table(heavy_key).items()}
        self.step("sensitivity_2", board.rank_to(heavy_ranks, 2.0), board.win.animate(run_time=1.4).set_value(1.0), seconds=2.0)
        new = winner_of(heavy_key)
        self.step("sensitivity_3", board.visit([board.scan.index(new)], 1.5, rt), seconds=1.6)

    def index_axis(self, board):
        """📐 Scale under the bars: 0 is the best and 1 the worst of the compared set in every criterion."""
        y = board.y0 - (len(board.all) - 1) * board.pitch - .42
        line = mn.Line([board.xl, y, 0], [board.xl + board.width, y, 0], color=INK, stroke_width=2)
        ticks = mn.VGroup(*[mn.VGroup(mn.Line([board.xl + board.width * f, y - .06, 0], [board.xl + board.width * f, y + .06, 0], color=INK, stroke_width=2),
                                      text({0: "0", 1: "1"}.get(f, self.number(f, 1)), [board.xl + board.width * f, y - .27, 0], size=S0)) for f in (0, .5, 1)])
        label = text(self.tr("Index: 0 = best of the set in every criterion, 1 = worst of the set", "Index: 0 = Bester der Menge bei jedem Kriterium, 1 = Schlechtester der Menge"),
                     (board.xl + board.width / 2, y - .5, 0), size=S0, color=MUTED, width=6)
        return mn.VGroup(line, ticks, label)
