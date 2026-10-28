import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


sub(1, "y0=1.65, pitch=.72, xl=-4.55, width=4.0", "y0=1.7, pitch=.76, xl=-4.55, width=4.0")
sub(1, """        frame_w, frame_x = RIGHT - LEFT, (RIGHT + LEFT) / 2""", """        frame_w, frame_x = .3 + 7.08, (.3 - 7.08) / 2""")
sub(1, """        if self.scores.get_value() > .02:
            group.add(_room.text([s.tr("Index", "Index")], (xl + W + .5, strip_y, 0), size=S1, color=INK).set_opacity(self.scores.get_value()))
""", "")
sub(1, """                    share = _room.text([_room.Number(s.number(round(ws[j] * 100), 0)), "%"], (lefts[j] + ws[j] * W / 2, strip_y, 0), size=14 if ws[j] * W > .5 else 11, color="#06121a").set_opacity(header * mode)
                    group.add(split, link, share)""", """                    percent = [_room.Number(s.number(round(ws[j] * 100), 0)), "%"]
                    if ws[j] * W < .45:
                        share = _room.place(_room.text(percent, size=13, color=COLORS[c]), (lefts[j] + ws[j] * W + .08, strip_y, 0), "left").set_opacity(header * mode)
                    else:
                        share = _room.text(percent, (lefts[j] + ws[j] * W / 2, strip_y, 0), size=14, color="#06121a").set_opacity(header * mode)
                    group.add(split, link, share)""")

sub(3, """                                      text({0: "0", 1: "1"}.get(f, self.number(f, 1)), [board.xl + board.width * f, y - .27, 0], size=S0)) for f in (0, .5, 1)])
        return mn.VGroup(line, ticks)""", """                                      text({0: "0", 1: "1"}.get(f, self.number(f, 1)), [board.xl + board.width * f, y - .27, 0], size=S0)) for f in (0, .5, 1)])
        return mn.VGroup(line, ticks, _room.place(text(self.tr("Index", "Index"), size=S1, color=INK), (board.xl + board.width + .3, y, 0), "left"))""")
sub(3, """            outlines.add(shape(mn.Rectangle(width=ws[j] * board.width, height=board.pitch * 5 + .12, color=colour, stroke_width=3).move_to([centre, board.y0 - board.pitch * 2 - .02, 0])))""",
    """            top, bottom = board.y0 + .3, board.y0 - board.pitch * 4 - .7
            outlines.add(shape(mn.Rectangle(width=ws[j] * board.width, height=top - bottom, color=colour, stroke_width=3).move_to([centre, (top + bottom) / 2, 0])))""")
sub(3, """chip = _room.chip(words, (centre, -2.45, 0), size=14, color=colour)""", """chip = _room.chip(words, (centre, -2.5, 0), size=14, color=colour)""")
sub(3, """.arrange(mn.RIGHT, buff=.15).move_to([-3.4, -2.45, 0])
        self.step("winner_2\"""", """.arrange(mn.RIGHT, buff=.15).move_to([-3.4, -2.5, 0])
        self.step("winner_2\"""")
sub(3, """.arrange(mn.RIGHT, buff=.25).move_to([-3.4, -2.45, 0])
        board.old_id""", """.arrange(mn.RIGHT, buff=.25).move_to([-3.4, -2.5, 0])
        board.old_id""")
sub(3, """        formula = self.block_index(lambda: board.cursor()[0], board.weights, y=-2.2)""", """        formula = self.block_index(lambda: board.cursor()[0], board.weights, y=-2.15)""")
sub(3, """        swap = mn.DoubleArrow([-.3, board.y0, 0], [-.3, board.y0 - board.pitch, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)
        change = _room.place(_room.text([self.tr("rank change", "Rangwechsel")], size=13, color=WARN), (-.15, board.y0 - board.pitch / 2 - .3, 0), "left")""",
    """        swap = mn.DoubleArrow([.3, board.y0, 0], [.3, board.y0 - board.pitch, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)
        change = _room.place(_room.text([self.tr("rank change", "Rangwechsel")], size=13, color=WARN), (.5, board.y0 - board.pitch / 2, 0), "left")""")
sub(3, """        was = _room.chip([self.tr("was 1st", "war 1.")], (-.9, board.y0 - board.pitch / 2, 0), size=S0, color=WARN)""", """        was = _room.chip([self.tr("was 1st", "war 1.")], (-1.2, board.y0 - board.pitch / 2, 0), size=S0, color=WARN)""")
# limits
sub(3, """        for value in range(low, high + 1, 1000):
            axis.add(mn.Line([X(value), y_axis - .07, 0], [X(value), y_axis + .07, 0], color=INK, stroke_width=2), text(self.number(value), (X(value), y_axis + .3, 0), size=S0))""",
    """        for value in range(low, high + 1, 1000):
            axis.add(mn.Line([X(value), y_axis - .07, 0], [X(value), y_axis + .07, 0], color=INK, stroke_width=2))
            if value != BUDGET_CHF:
                axis.add(text(self.number(value), (X(value), y_axis + .3, 0), size=S0))""")
sub(3, """        limit = mn.VGroup(mn.DashedLine([zone_x, y_axis + .45, 0], [zone_x, y_axis - .75, 0], color=WARN, stroke_width=3),""", """        limit = mn.VGroup(mn.DashedLine([zone_x, y_axis + .45, 0], [zone_x, y_axis - .16, 0], color=WARN, stroke_width=3),""")
sub(3, """text(self.tr("not allowed", "nicht erlaubt"), ((zone_x + xr + .15) / 2, y_axis - .62, 0), size=S0, color=WARN))""", """text(self.tr("not allowed", "nicht erlaubt"), ((zone_x + xr + .15) / 2 + .35, y_axis - .62, 0), size=S0, color=WARN))""")
# range
sub(2, """self.bar_row(group, raws, [self.n(sum(shown)), "CHF"], 5.6 / 22500, colours)""", """self.bar_row(group, raws, [self.n(sum(shown)), "CHF"], 4.6 / 22500, colours)""")
sub(3, """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.5, y0=1.7)""", """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.43, y0=1.7)""")
sub(3, """            note = text(f"{before} → {new_ranks[i]}", (.95, 0, 0), size=S1, color=WARN if i in old_ranks and old_ranks[i] != new_ranks[i] else INK)
            note.identifier = i
            note.add_updater(lambda m: (m.move_to([.95, board.y(m.identifier), 0]), m.set_opacity(board.row_o[m.identifier].get_value())))""",
    """            note = text(f"{before} → {new_ranks[i]}", (-1.35, 0, 0), size=S1, color=WARN if i in old_ranks and old_ranks[i] != new_ranks[i] else INK)
            note.identifier = i
            note.add_updater(lambda m: (m.move_to([-1.35, board.y(m.identifier), 0]), m.set_opacity(board.row_o[m.identifier].get_value())))""")
sub(3, """        heading = _room.place(text(self.tr("rank old → new", "Rang alt → neu"), size=S1, color=MUTED), (.35, board.y0 + .5, 0), "left")""",
    """        heading = _room.place(text(self.tr("rank old → new (orange: changed)", "Rang alt → neu (orange: geändert)"), size=S0, color=MUTED), (LEFT + .4, -1.75, 0), "left")""")
sub(3, """            link = mn.DoubleArrow([1.55, ya, 0], [1.55, yb, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)""", """            link = mn.DoubleArrow([-.8, ya, 0], [-.8, yb, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)""")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
