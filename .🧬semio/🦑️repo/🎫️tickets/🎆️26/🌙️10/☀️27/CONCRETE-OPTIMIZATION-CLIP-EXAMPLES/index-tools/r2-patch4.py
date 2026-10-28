import ast
import re

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new, many=False):
    global count
    found = texts[part].count(old)
    assert found == 1 or (many and found > 1), f"part{part}: {found} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


# part 1
sub(1, "LEFT, RIGHT = -6.95, -0.2\n", "LEFT, RIGHT = -6.95, -0.2\nFRAME_X0, FRAME_X1 = -7.08, 0.3\n")
sub(1, "y0=1.7, pitch=.76, xl=-4.55, width=4.0, weights=None, order=None):", "y0=1.7, pitch=.76, xl=-4.35, width=4.0, weights=None, order=None, name_size=14):")
sub(1, """            name = block(lines(item["title"][scene.language()], 15), (-5.4, 0, 0), size=14, width=1.4)""", """            name = block(lines(item["title"][scene.language()], 15), (-5.25, 0, 0), size=name_size, width=1.6)""")
sub(1, """        frame_w, frame_x = .3 + 7.08, (.3 - 7.08) / 2""", """        frame_w, frame_x = FRAME_X1 - FRAME_X0, (FRAME_X1 + FRAME_X0) / 2""")
sub(1, """        tall, drop = self.pitch - .06 + .12 * numbers, .08 + .06 * numbers""", """        tall, drop = self.pitch - .02 + .12 * numbers, .08 + .06 * numbers""")
sub(1, """size=14, color=INK, width=max(slot - .04, .3)""", """size=14, color=INK, width=max(slot + .2, .55)""")
sub(1, """size=13, color=MUTED, width=max(slot - .04, .3)""", """size=13, color=MUTED, width=max(slot + .2, .55)""")
sub(1, """(xl + W + .3, y, 0), "left")
                          .set_opacity(self.scores.get_value() * level))""", """(xl + W + .25, y, 0), "left")
                          .set_opacity(self.scores.get_value() * level))""")

# part 2
sub(2, """(7.0, 3.78, 0), "right")""", """(6.8, 3.65, 0), "right")""")
sub(2, """            group.add(mn.Rectangle(width=6.8, height=.27, color=INK, stroke_width=2, stroke_opacity=.8).move_to([(LEFT + RIGHT) / 2, y, 0]))""",
    """            group.add(mn.Rectangle(width=6.8, height=.31, color=INK, stroke_width=2, stroke_opacity=.8).move_to([(LEFT + RIGHT) / 2, y, 0]))""")
for old in ("ys = [-1.0 - .3 * k for k in range(5)]",):
    sub(2, old, "ys = [-1.0 - .32 * k for k in range(5)]")
text = texts[2]
assert text.count("y = -1.0 - .3 * k") == 2
texts[2] = text.replace("y = -1.0 - .3 * k", "y = -1.0 - .32 * k")
sub(2, "ys = [-1.4 - .3 * k for k in range(4)]", "ys = [-1.4 - .32 * k for k in range(4)]")

# part 3
sub(3, "CX = [-3.8 + 1.05 * j for j in range(4)]", "CX = [-3.9 + 1.05 * j for j in range(4)]")
sub(3, """        cursor = mn.Rectangle(width=RIGHT - LEFT, height=.7, color=INK, stroke_width=2.4)
        cursor.add_updater(lambda m: (m.move_to([(LEFT + RIGHT) / 2, 1.4 - .8 * self.roster_pos.get_value(), 0])""", """        cursor = mn.Rectangle(width=FRAME_X1 - FRAME_X0, height=.7, color=INK, stroke_width=2.4)
        cursor.add_updater(lambda m: (m.move_to([(FRAME_X0 + FRAME_X1) / 2, 1.4 - .8 * self.roster_pos.get_value(), 0])""")
sub(3, """        under = text(self.tr("Under each bar: normalized value, then the physical value in the unit of the column head", "Unter jedem Balken: normierter Wert, darunter der physikalische Wert in der Einheit des Spaltenkopfs"),
                     size=14, color=MUTED, width=6.8)""", """        under = text(self.tr("Under each bar: normalized value, then physical value", "Unter jedem Balken: normiert, darunter der physikalische Wert"),
                     size=14, color=MUTED, width=6.8)""")
sub(3, """self.name(i, size=14, width=1.55), (x0 - .4, Y(ranks[i][0]), 0), "right"), _room.place(self.name(i, size=14, width=1.3), (x1 + .3, Y(ranks[i][-1]), 0), "left"))""",
    """self.name(i, size=14, width=1.7), (x0 - .4, Y(ranks[i][0]), 0), "right"), _room.place(self.name(i, size=14, width=1.65), (x1 + .3, Y(ranks[i][-1]), 0), "left"))""")
sub(3, """            name = self.name(i, size=13, width=1.3, limit=14).move_to([slots[i], 2.7, 0])""", """            name = self.name(i, size=13, width=1.45, limit=14).move_to([slots[i], 2.7, 0])""")
sub(3, """            block_ = mn.VGroup(self.name(i, size=13, width=1.4, limit=14, color=MUTED), _room.text(self.words(self.tr("dominated by", "dominiert von")) + self.words(dominator), size=13, color=WARN, width=1.7)).arrange(mn.DOWN, buff=.06, aligned_edge=mn.LEFT)""",
    """            block_ = mn.VGroup(self.name(i, size=13, width=1.5, limit=14, color=MUTED), _room.text(self.words(self.tr("dominated by", "dominiert von")), size=13, color=WARN),
                               _room.text(self.words(dominator), size=13, color=WARN)).arrange(mn.DOWN, buff=.05, aligned_edge=mn.LEFT)""")
# sensitivity labels
sub(3, """        was = _room.chip([self.tr("was 1st", "war 1.")], (-1.2, board.y0 - board.pitch / 2, 0), size=S0, color=WARN)
        swap = mn.DoubleArrow([.3, board.y0, 0], [.3, board.y0 - board.pitch, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)
        change = _room.place(_room.text([self.tr("rank change", "Rangwechsel")], size=13, color=WARN), (.5, board.y0 - board.pitch / 2, 0), "left")""",
    """        was = _room.chip(self.words(self.tr("was 1st · rank change", "war 1. · Rangwechsel")), (-2.0, board.y0 - board.pitch / 2, 0), size=S0, color=WARN)
        swap = mn.DoubleArrow([-.6, board.y0, 0], [-.6, board.y0 - board.pitch, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)""")
sub(3, """mn.FadeIn(was), mn.Create(swap), mn.FadeIn(change), seconds=1.8)""", """mn.FadeIn(was), mn.Create(swap), seconds=1.8)""")
sub(3, """mn.FadeOut(was), mn.FadeOut(swap), mn.FadeOut(change), board.visit""", """mn.FadeOut(was), mn.FadeOut(swap), board.visit""")
# index axis label above its right end
sub(3, """        return mn.VGroup(line, ticks, _room.place(text(self.tr("Index", "Index"), size=S1, color=INK), (board.xl + board.width + .3, y, 0), "left"))""",
    """        return mn.VGroup(line, ticks, _room.place(text(self.tr("Index", "Index"), size=S1, color=INK), (board.xl + board.width, y + .2, 0), "right"))""")
# range board
sub(3, """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.43, y0=1.7)""", """        board = Board(self, five, [extra], weights=self._weights, order=order, pitch=.5, y0=2.1, name_size=13)
        board.header.set_value(0.0)""")
sub(3, """(LEFT + .4, -1.75, 0), "left")
        link = None""", """(LEFT + .4, -1.35, 0), "left")
        link = None""")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
