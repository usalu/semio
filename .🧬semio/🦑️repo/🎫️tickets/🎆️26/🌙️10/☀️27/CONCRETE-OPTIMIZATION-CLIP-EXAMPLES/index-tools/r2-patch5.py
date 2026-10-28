import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


sub(1, "FRAME_X0, FRAME_X1 = -7.08, 0.6", "FRAME_X0, FRAME_X1 = -7.08, 0.85")
sub(1, "xl=-4.35, width=4.2,", "xl=-4.45, width=4.2,")
sub(1, """(-5.25, 0, 0), size=name_size, width=1.6)""", """(-5.3, 0, 0), size=name_size, width=1.55)""")
sub(1, "size=12, color=COLORS[criterion]) for word", "size=13, color=COLORS[criterion]) for word")
sub(1, """"~↓" if CRITERIA[criterion]["direction"] == "min" else "~↑"], size=12,""", """"~↓" if CRITERIA[criterion]["direction"] == "min" else "~↑"], size=13,""")
sub(1, "                x = xl + W / 4 * (j + .5)\n", "                x = xl + W / 2 + (j - 1.5) * 1.18\n")
sub(1, """                if (1 - compact) * self.numbers.get_value() * level > .05:
                    shown = (1 - compact) * self.numbers.get_value() * level
                    group.add(_room.text([_room.Number(s.number(values[j], 2))], (lefts[j] + slot / 2, y - .23, 0), size=14, color=INK, width=max(slot + .2, .55)).set_opacity(shown),
                              _room.text([_room.Number(s.number(raw(identifier, c), DIGITS[c]))], (lefts[j] + slot / 2, y - .41, 0), size=13, color=MUTED, width=max(slot + .2, .55)).set_opacity(shown))""",
    """                if (1 - compact) * self.numbers.get_value() * level > .05:
                    shown = (1 - compact) * self.numbers.get_value() * level
                    group.add(_room.text([_room.Number(s.number(values[j], 2))], (lefts[j] + slot / 2, y - .23, 0), size=14, color=INK, width=max(slot + .2, .55)).set_opacity(shown))
                    if slot >= .6:
                        group.add(_room.text([_room.Number(s.number(raw(identifier, c), DIGITS[c]))], (lefts[j] + slot / 2, y - .41, 0), size=13, color=MUTED, width=slot + .2).set_opacity(shown))""")

lines2 = texts[2].splitlines()
k = next(i for i, l in enumerate(lines2) if "self.footer = " in l)
lines2[k:k + 2] = ['        self.footer = _room.place(_room.text(self.tr("Simplified annual teaching model · Zurich-like climate · EN ISO 13790 style",',
                   '                                                      "Vereinfachtes Jahres-Lehrmodell · Zürich-ähnliches Klima · Stil EN ISO 13790").split(), size=13, color=MUTED), (LEFT, 3.82, 0), "left")']
texts[2] = chr(10).join(lines2) + chr(10)

sub(3, "CX = [-3.9 + 1.05 * j for j in range(4)]", "CX = [-3.95 + 1.18 * j for j in range(4)]")
sub(3, """            board.outlines""" if False else """            outlines.add(shape(mn.Rectangle(width=ws[j] * board.width, height=top - bottom,""", """            outlines.add(shape(mn.Rectangle(width=ws[j] * board.width + .1, height=top - bottom,""")
sub(3, """        was = _room.chip(self.words(self.tr("was 1st · rank change", "war 1. · Rangwechsel")), (-2.0, board.y0 - board.pitch / 2, 0), size=S0, color=WARN)
        swap = mn.DoubleArrow([-.6, board.y0, 0], [-.6, board.y0 - board.pitch, 0], color=WARN, buff=.05, stroke_width=3, tip_length=.14)""",
    """        was = _room.chip(self.words(self.tr("was 1st", "war 1.")), (-1.4, board.y0 - board.pitch - .12, 0), size=S0, color=WARN)""")
sub(3, "mn.FadeIn(was), mn.Create(swap), seconds=1.8)", "mn.FadeIn(was), seconds=1.8)")
sub(3, "mn.FadeOut(was), mn.FadeOut(swap), board.visit", "mn.FadeOut(was), board.visit")
sub(3, "x0, x1, top, step = -5.0, -1.5, 1.7, .66", "x0, x1, top, step = -4.8, -1.5, 1.7, .66")
sub(3, "pitch=.5, y0=2.1, name_size=13)", "pitch=.54, y0=2.2, name_size=13)")
sub(3, "link = mn.DoubleArrow([-.8, ya, 0], [-.8, yb, 0]", "link = mn.DoubleArrow([-.55, ya, 0], [-.55, yb, 0]")
sub(3, """            x = LEFT + 1.0 + 1.3 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, .95, 0), .11, ramp=self._ramp, value=self._ramp.low), self.name(i, size=13, width=1.2).move_to([x, .35, 0]))""",
    """            x = LEFT + .75 + 1.36 * k
            parts.add(self.thumbnail(by_id(i)["design"], (x, .95, 0), .11, ramp=self._ramp, value=self._ramp.low), self.name(i, size=13, width=1.6).move_to([x, .35, 0]))""")
sub(3, "name = self.name(i, size=13, width=1.45, limit=14).move_to([slots[i], 2.7, 0])", "name = self.name(i, size=13, width=1.6, limit=14).move_to([slots[i], 2.7, 0])")
sub(3, "flow.move_to([-3.5, 3.4, 0])", "flow.move_to([-3.5, 3.25, 0])")
sub(3, ".arrange(mn.DOWN, buff=.05, aligned_edge=mn.LEFT)\n            tags.add(_room.place(block_", ".arrange(mn.DOWN, buff=.12, aligned_edge=mn.LEFT)\n            tags.add(_room.place(block_")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
