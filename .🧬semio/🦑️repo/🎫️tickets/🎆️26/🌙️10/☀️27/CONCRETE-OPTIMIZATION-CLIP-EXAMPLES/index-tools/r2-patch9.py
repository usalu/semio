import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


sub(3, "self.name(item[\"id\"], size=S0, width=1.4).move_to([-5.05, Y(k), 0]))", "self.name(item[\"id\"], size=S0, width=1.5).move_to([-5.1, Y(k), 0]))")
sub(3, "CX = [-3.95 + 1.18 * j for j in range(4)]", "CX = [-3.9 + 1.18 * j for j in range(4)]")
sub(2, "group.add(mn.Rectangle(width=2.35, height=.4, color=COLORS[c], stroke_width=2.4).move_to([x + 1.12, y - .07, 0]))", "group.add(mn.Rectangle(width=2.35, height=.42, color=COLORS[c], stroke_width=2.4).move_to([x + 1.12, y - .03, 0]))")
sub(3, """(board.xl + board.width, y + .2, 0), "right"))""", """(board.xl - .15, y, 0), "right"))""")
sub(3, "m.move_to([-1.35, board.y(m.identifier), 0])", "m.move_to([-.92 - m.width / 2, board.y(m.identifier), 0])")
a = texts[3].index("            group = mn.VGroup(self.equation([([self.tr(\"Winter (Zurich-like climate)")
b = texts[3].index("            return group\n        return live(build, _room.design_key(rt))\n\n    def step_note")
texts[3] = texts[3][:a] + '''            group = mn.VGroup(self.equation([([self.tr("Winter (Zurich-like climate): sun low, heat leaves through wall and window", "Winter (Zürcher Klima): Sonne tief, Wärme geht durch Wand und Fenster verloren")], _room.HEAT)],
                                            LEFT + .4, 3.0, size=16, width=6.4),
                              self.equation([([self.tr("Conductance", "Leitwert"), "H", "=", self.n(parts[0], digits), "+", self.n(parts[1], digits), "+", self.n(parts[2], digits), "=", self.n(sum(parts), digits), "W/K"], INK)],
                                            LEFT + .4, 2.65, size=15, width=6.4),
                              self.equation([([self.tr("wall U", "Wand U"), self.n(found["u_wall"], 2), "·", self.tr("window U", "Fenster U"), self.n(constant("envelope", "u_window"), 1), "W/(m²·K)", "·",
                                               self.tr("other", "übrige"), self.n(constant("envelope", "h_other")), "W/K"], MUTED)], LEFT + .4, 2.38, size=14, width=6.4),
                              self.equation([([self.n(sum(parts), digits), "W/K", "×", self.n(constant("climate", "heating_degree_hours")), "Kh/a", "=", self.n(loss), "kWh/a", self.tr("loss", "Verlust"), "−",
                                               self.n(free), self.tr("free heat", "freie Wärme"), "=", self.n(found["heating_demand"]), "kWh/a"], INK)], LEFT + .4, 2.05, size=15, width=6.4))
''' + texts[3][b:]
sub(3, "(LEFT + .4, 1.6, 0), size=18, words=True, align=\"left\", width=5.6))", "(LEFT + .4, 1.5, 0), size=18, words=True, align=\"left\", width=5.6))")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
