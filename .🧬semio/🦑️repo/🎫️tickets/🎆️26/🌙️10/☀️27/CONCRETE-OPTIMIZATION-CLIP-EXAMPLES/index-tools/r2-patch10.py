import ast

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2, 3)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


# units: one notation (the toolkit's INTENSITY_UNIT) in every plain string
sub(1, '''    label = _fonts.body_text(str(value), font_size=size, color=color)
    if width is not None and label.width > width:''', '''    label = _fonts.body_text(str(value).replace("kWh/(m²·a)", _room.INTENSITY_UNIT), font_size=size, color=color)
    if width is not None and label.width > width:''')
sub(2, '''        return CRITERIA[criterion]["unit"]
''', '''        return CRITERIA[criterion]["unit"].replace("kWh/(m²·a)", _room.INTENSITY_UNIT)
''')
sub(2, '''"CHF/(m²·cm)"]''', '''"CHF/m²/cm"]''')
sub(2, '''"CO₂e/(m²·cm)"]''', '''"CO₂e/m²/cm"]''')
sub(2, '''self.n(found["intensity"], 2), "kWh/(m²·a)"]''', '''self.n(found["intensity"], 2), _room.INTENSITY_UNIT]''')
sub(3, '''self.tr("Electricity intensity, kWh/(m²·a)", "Strombedarf pro m², kWh/(m²·a)")''', '''self.tr(f"Electricity intensity, {_room.INTENSITY_UNIT}", f"Strombedarf pro m², {_room.INTENSITY_UNIT}")''')

# narration
sub(2, '''"Only window share, overhang and insulation differ.", "Nur Fensteranteil, Vordach und Dämmung unterscheiden sich."''',
    '''"Only window share, overhang and insulation are compared.", "Verglichen werden nur Fensteranteil, Vordach und Dämmung."''')
sub(2, '''"Premium: worst cost, best electricity; all other values shrink.", "Premium ist am teuersten und am sparsamsten; alle anderen Werte werden kleiner."''',
    '''"Premium is dearest and most frugal: costs shrink, electricity shifts slightly.", "Premium ist am teuersten und sparsamsten: Kosten schrumpfen, Strom verschiebt sich leicht."''')
sub(3, '''self.tr("strongest pull", "stärkster Zug") if j == wide else self.tr("weakest pull", "schwächster Zug")''',
    '''self.tr("strongest pull", "stärkster Einfluss") if j == wide else self.tr("weakest pull", "schwächster Einfluss")''')

# weights printed with the precision the index line uses
sub(1, '''                    percent = [_room.Number(s.number(round(ws[j] * 100), 0)), "%"]''',
    '''                    decimals = 1 if any(abs(w * 100 - round(w * 100)) > .05 for w in ws) else 0
                    percent = [_room.Number(s.number(round(ws[j] * 100, decimals), decimals)), "%"]''')
sub(1, '''if ws[j] * W < .45:
                        share''', '''if ws[j] * W < .5:
                        share''')

# full index table: four decimals so every product holds as printed
a = texts[2].index("            exact = table(tuple(round(w, 6) for w in weights()))[identifier][\"normalized\"]")
b = texts[2].index("            return group\n        return live(build, lambda: (getter(), tuple(round(w, 4) for w in weights())))")
texts[2] = texts[2][:a] + '''            exact = table(tuple(round(w, 6) for w in weights()))[identifier]
            norm3 = [round(exact["normalized"][c], 3) for c in ORDER]
            terms = [round(w * v, 4) for w, v in zip(printed_w, norm3)]
            for k, c in enumerate(ORDER):
                yy = 2.45 - .55 * k
                items = [([self.n(raw(identifier, c), DIGITS[c]), *self.words(self.unit_label(c))], COLORS[c]), (["→"], MUTED),
                         ([self.tr("normalized", "normiert"), self.n(norm3[k], 3), "×", self.n(printed_w[k] * 100, digits - 2), "%", "="], INK), ([self.n(terms[k], 4)], COLORS[c])]
                group.add(_room.clone(glyph(c, width=.26)).move_to([LEFT + .15, yy, 0]), self.equation(items, LEFT + .45, yy - .07, size=14, width=6.4))
            group.add(self.equation([([self.tr("Index", "Index"), "="], INK), ([self.n(terms[0], 4), "+", self.n(terms[1], 4), "+", self.n(terms[2], 4), "+", self.n(terms[3], 4), "="], INK),
                                     ([self.n(sum(terms), 4)], GOOD)], LEFT + .45, 2.45 - .55 * 4 - .07, size=16, width=6.4))
''' + texts[2][b:]

# winter: no overprint, sun disc free; the toolkit derivation sits below the sun without its head words
a = texts[3].index("            group = mn.VGroup(self.equation([([self.tr(\"Winter (Zurich-like climate)")
b = texts[3].index("            return group\n        return live(build, _room.design_key(rt))\n\n    def step_note")
texts[3] = texts[3][:a] + '''            group = mn.VGroup(self.equation([([self.tr("Winter (Zurich-like climate): sun low, heat leaves through wall and window", "Winter (Zürcher Klima): Sonne tief, Wärme geht durch Wand und Fenster verloren")], _room.HEAT)],
                                            LEFT + .4, 3.0, size=16, width=6.4),
                              self.equation([([self.tr("Conductance", "Leitwert"), "H", "=", self.n(parts[0], digits), "+", self.n(parts[1], digits), "+", self.n(parts[2], digits), "=", self.n(sum(parts), digits), "W/K"], INK)],
                                            LEFT + .4, 2.6, size=15, width=6.4),
                              self.equation([([self.tr("wall U", "Wand U"), self.n(found["u_wall"], 2), "·", self.tr("window U", "Fenster U"), self.n(constant("envelope", "u_window"), 1), "W/(m²·K)", "·",
                                               self.tr("other", "Rest"), self.n(constant("envelope", "h_other")), "W/K"], MUTED)], LEFT + .4, 2.3, size=14, width=6.4),
                              self.equation([([self.tr("heat losses H × 70,000 Kh/a − utilised free heat = heating demand; demand ÷ COP = heating", "Wärmeverlust H × 70.000 Kh/a − genutzte freie Wärme = Heizbedarf; Bedarf ÷ COP = Heizung")], MUTED)],
                                            LEFT + .4, 1.0, size=13, width=5.2))
''' + texts[3][b:]
sub(3, '''self.derivation(self._rt, "heating", (LEFT + .4, 1.5, 0), size=18, words=True, align="left", width=5.6)''', '''self.derivation(self._rt, "heating", (LEFT + .4, .45, 0), size=18, words=False, align="left", width=5.2)''')
sub(3, '''self.tr("winter loss", "Winter: Verlust")''', '''self.tr("winter heating demand", "Winter: Heizbedarf")''')

for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
s = "".join(texts[n] for n in (1, 2, 3))
ast.parse(s)
open("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py", "w", encoding="utf-8").write(s)
print("patched", count)
