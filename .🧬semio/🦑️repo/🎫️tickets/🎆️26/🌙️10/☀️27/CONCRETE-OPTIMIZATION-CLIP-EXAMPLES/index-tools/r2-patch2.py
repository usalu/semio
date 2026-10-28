import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


sub(1, """(0, ly + 1.5 + .45 * row, 0), size=self.name_size""", """(0, ly + 1.5 + .55 * row, 0), size=self.name_size""")
sub(1, """[0, ly + 1.72, 0], color=MUTED""", """[0, ly + 1.8, 0], color=MUTED""")
sub(1, """        self.color = COLORS[criterion]
        self.sign""", """        self.color = COLORS[criterion]
        self.digits_tag = 3 if criterion == "electricity" else DIGITS[criterion]
        self.sign""")
sub(1, """        tag = text(s.number(self.values[identifier], DIGITS[self.criterion]), (0, 0, 0), size=S0, color=shade)""", """        tag = text(s.number(self.values[identifier], self.digits_tag), (0, 0, 0), size=S0, color=shade)""")
sub(1, "y0=1.5, pitch=.66, xl=-4.55, width=4.0", "y0=1.65, pitch=.72, xl=-4.55, width=4.0")
sub(1, """(lefts[j] + slot / 2, y - .24, 0), size=14, color=INK, width=max(slot - .04, .3)""", """(lefts[j] + slot / 2, y - .23, 0), size=14, color=INK, width=max(slot - .04, .3)""")
sub(1, """(lefts[j] + slot / 2, y - .44, 0), size=13, color=MUTED, width=max(slot - .04, .3)""", """(lefts[j] + slot / 2, y - .41, 0), size=13, color=MUTED, width=max(slot - .04, .3)""")

sub(2, """                return mn.VGroup(_room.text(self.words(self.tr("no overhang", "kein Vordach")), P(3.2, HEIGHT + SLAB + .5), size=14, color=WARN))""",
     """                label = _room.text(self.words(self.tr("no overhang", "kein Vordach")), P(2.4, HEIGHT + SLAB + .6), size=14, color=WARN)
                return mn.VGroup(label, mn.Line(label.get_left() + np.array([-.05, 0, 0]), P(x_o + .05, HEIGHT + SLAB + .03), color=WARN, stroke_width=1.4, stroke_opacity=.8))""")
sub(2, """([*self.words(self.tr("no overhang hides sky", "kein Vordach verdeckt Himmel"))], MUTED)""", """([*self.words(self.tr("no overhang: full sky view", "kein Vordach: voller Himmel"))], MUTED)""")
sub(2, """    def block_index(self, getter, weights, *, full=False, y=-2.7):""", """    def block_index(self, getter, weights, *, full=False, y=-2.2):""")

sub(3, """(LEFT + .4, -1.55, 0), "left")
        self.step("shift_2\"""", """(LEFT + .4, -.36, 0), "left")
        self.step("shift_2\"""")
sub(3, """            chip = _room.chip(words, (centre, -2.4, 0), size=14, color=colour)""", """            chip = _room.chip(words, (centre, -2.45, 0), size=14, color=colour)""")
sub(3, """.arrange(mn.RIGHT, buff=.15).move_to([-3.4, -2.4, 0])
        self.step("winner_2\"""", """.arrange(mn.RIGHT, buff=.15).move_to([-3.4, -2.45, 0])
        self.step("winner_2\"""")
sub(3, """.arrange(mn.RIGHT, buff=.25).move_to([-3.4, -2.4, 0])
        board.old_id""", """.arrange(mn.RIGHT, buff=.25).move_to([-3.4, -2.45, 0])
        board.old_id""")
sub(3, """        return mn.VGroup(scale, under).arrange(mn.DOWN, buff=.1).move_to([-3.55, -2.2, 0])""", """        return mn.VGroup(scale, under).arrange(mn.DOWN, buff=.1).move_to([-3.55, -2.3, 0])""")
sub(3, """        y = board.y0 - (len(board.all) - 1) * board.pitch - .4
        line = mn.Line""", """        y = board.y0 - (len(board.all) - 1) * board.pitch - .36
        line = mn.Line""")
sub(3, """        formula = self.block_index(lambda: board.cursor()[0], board.weights, y=-2.0)""", """        formula = self.block_index(lambda: board.cursor()[0], board.weights, y=-2.2)""")
sub(3, """        label = f"{sign} {self.number(value, DIGITS[ruler.criterion])} {self.unit_label(ruler.criterion)}\"""", """        label = f"{sign} {self.number(value, ruler.digits_tag)} {self.unit_label(ruler.criterion)}\"""")
sub(3, """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.5, y0=1.6)""", """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.5, y0=1.7)""")

# winter callout
sub(3, """        self.step("ruler_4", self.ruler_away(ruler, header, True), self.set_focus("punched", .8), self.swap_season("winter"), self.row_hi.animate(run_time=.1).set_value(0), seconds=3.6)
        self.step("ruler_5", self.ruler_away(ruler, header, False),""", """        winter = self.winter_note()
        self.step("ruler_4", self.ruler_away(ruler, header, True), self.set_focus("punched", .8), self.swap_season("winter"), self.row_hi.animate(run_time=.1).set_value(0), mn.FadeIn(winter), seconds=3.6)
        self.step("ruler_5", self.ruler_away(ruler, header, False), mn.FadeOut(winter),""")
sub(3, """    def step_note(self, ruler, sign, value):""", """    def winter_note(self):
        \"\"\"❄️ What the winter picture means: U-values of wall and window, the heat pump and the heating demand it divides.\"\"\"
        rt = self._rt

        def build():
            found = _room.result(_room.values(rt))
            shown = _room.apportion([found[key] for key, _ in ENERGY], energy_digits(found))
            digits = energy_digits(found)
            precision, sign = reproduce(lambda value: value / _room.systems()["cop"], [found["heating_demand"]], shown[0], digits)
            group = mn.VGroup(self.equation([([self.tr("Winter: sun low, heat leaves through wall and window", "Winter: Sonne tief, Wärme geht durch Wand und Fenster verloren")], _room.HEAT)], LEFT + .4, 2.9, size=17, width=6.4),
                              self.equation([([self.tr("Wall", "Wand"), "U", "=", self.n(found["u_wall"], 2), "W/(m²·K)", "·", self.tr("Window", "Fenster"), "U", "=", self.n(_room.constant("envelope", "u_window"), 1), "W/(m²·K)"], INK)],
                                            LEFT + .4, 2.4, size=17, width=6.4),
                              self.equation([([self.tr("Heating demand", "Heizbedarf"), self.n(found["heating_demand"], precision), "kWh/a", "÷", "COP", self.n(_room.systems()["cop"], 1), sign], INK),
                                             ([self.n(shown[0], digits), "kWh/a"], _room.HEAT)], LEFT + .4, 1.9, size=20, width=6.4))
            return group
        return live(build, _room.design_key(rt))

    def step_note(self, ruler, sign, value):""")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
