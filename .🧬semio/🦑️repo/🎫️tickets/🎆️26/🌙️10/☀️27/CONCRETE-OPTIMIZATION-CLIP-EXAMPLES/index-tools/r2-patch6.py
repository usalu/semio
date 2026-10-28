import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


sub(1, "tall, drop = self.pitch - .02 + .12 * numbers, .08 + .06 * numbers", "tall, drop = self.pitch - .02 + .12 * numbers, .08 + .08 * numbers")
sub(1, "(lefts[j] + slot / 2, y - .23, 0), size=14, color=INK, width=max(slot + .2, .55))", "(lefts[j] + slot / 2, y - .2, 0), size=14, color=INK, width=max(slot + .2, .55))")
sub(1, "(lefts[j] + slot / 2, y - .41, 0), size=13, color=MUTED, width=slot + .2)", "(lefts[j] + slot / 2, y - .43, 0), size=13, color=MUTED, width=slot + .2)")
sub(3, """        head = _room.place(text(self.tr("Sum of the printed numbers", "Summe der gedruckten Zahlen"), size=S1, color=MUTED), (LEFT, 2.95, 0), "left")""",
    """        head = _room.place(text(self.tr("Sum of the printed numbers", "Summe der gedruckten Zahlen"), size=S1, color=MUTED), (LEFT, 3.1, 0), "left")""")
sub(3, """        _room.place(unit_chip, (RIGHT, 2.95, 0), "right")""", """        _room.place(unit_chip, (RIGHT, 2.5, 0), "right")""")
sub(3, """        head2 = _room.place(text(self.tr("Still a sum of unlike units", "Immer noch eine Summe verschiedener Einheiten"), size=S1, color=MUTED), (LEFT, 2.95, 0), "left")""",
    """        head2 = _room.place(text(self.tr("Still a sum of unlike units", "Immer noch eine Summe verschiedener Einheiten"), size=S1, color=MUTED), (LEFT, 3.1, 0), "left")""")
sub(3, """        _room.place(unit_chip2, (RIGHT, 2.95, 0), "right")""", """        _room.place(unit_chip2, (RIGHT, 2.5, 0), "right")""")
sub(3, """        scale = mn.VGroup(text(self.tr("Normalized value:", "Normierter Wert:"), size=S0),
                          text(self.tr("0 = best of the set", "0 = Bester der Menge"), size=S0, color=GOOD),
                          text(self.tr("1 = worst of the set", "1 = Schlechtester der Menge"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.3)""",
    """        scale = mn.VGroup(text(self.tr("Normalized value:", "Normierter Wert:"), size=S0),
                          text(self.tr("0 = best", "0 = Bester"), size=S0, color=GOOD),
                          text(self.tr("1 = worst", "1 = Schlechtester"), size=S0, color=WARN)).arrange(mn.RIGHT, buff=.3)""")

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
