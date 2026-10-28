import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


sub(1, "(lefts[j] + slot / 2, y - .2, 0), size=14, color=INK, width=max(slot + .2, .55))", "(lefts[j] + slot / 2, y - .26, 0), size=14, color=INK, width=max(slot + .2, .55))")
sub(1, "(lefts[j] + slot / 2, y - .43, 0), size=13, color=MUTED, width=slot + .2)", "(lefts[j] + slot / 2, y - .46, 0), size=13, color=MUTED, width=slot + .2)")
sub(2, '(LEFT + .1, 3.74, 0), "left")', '(LEFT + .2, 3.68, 0), "left")')
sub(3, "[board.xl + board.width * f, y - .27, 0], size=S0))", "[board.xl + board.width * f, y - .22, 0], size=S0))")
sub(3, "board.weights, y=-2.15)", "board.weights, y=-2.24)")
sub(3, "x0, x1, top, step = -4.8, -1.5, 1.7, .66", "x0, x1, top, step = -4.8, -1.9, 1.7, .66")
sub(3, """self.name(i, size=14, width=1.65), (x1 + .3, Y(ranks[i][-1]), 0), "left"))""", """self.name(i, size=14, width=1.55), (x1 + .3, Y(ranks[i][-1]), 0), "left"))""")
sub(3, """            x = LEFT + .75 + 1.36 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, .95, 0), .11, ramp=self._ramp, value=self._ramp.low), self.name(i, size=13, width=1.6).move_to([x, .35, 0]))""",
    """            x = LEFT + .85 + 1.3 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, .95, 0), .11, ramp=self._ramp, value=self._ramp.low), self.name(i, size=13, width=1.6).move_to([x, .4 if k % 2 == 0 else -.1, 0]))""")
sub(3, """parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), LEFT + .3, -.1))""", """parts.add(heading(self.tr("Ranges: best … worst", "Bereiche: beste … schlechteste"), LEFT + .3, -.6))""")
sub(3, """parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, -.55 - .4 * (k // 2)))""", """parts.add(left(line, LEFT + .3 + (k % 2) * 3.3, -1.0 - .4 * (k // 2)))""")
sub(3, """        parts.add(heading(self.tr("Result", "Ergebnis"), LEFT + .3, -1.5),
                  self.thumbnail(by_id(winner)["design"], (LEFT + 1.0, -2.0, 0), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), LEFT + 1.5, -2.0),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), LEFT + 4.3, -2.0))""",
    """        parts.add(heading(self.tr("Result", "Ergebnis"), LEFT + .3, -1.8),
                  self.thumbnail(by_id(winner)["design"], (LEFT + 1.0, -2.2, 0), .11, ramp=self._ramp, value=self._ramp.low, ring=GOOD),
                  left(self.name(winner, size=S0, width=2.6, color=GOOD, limit=40), LEFT + 1.5, -2.2),
                  left(text(self.tr("Index", "Index") + " " + self.number(rows[winner]["score"], 2), size=S1), LEFT + 4.3, -2.2))""")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
