import sys

base = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/index-tools/"
texts = {n: open(base + f"r2-part{n}.py", encoding="utf-8").read() for n in (1, 2)}
count = 0


def sub(part, old, new):
    global count
    assert texts[part].count(old) == 1, f"part{part}: {texts[part].count(old)} matches: " + old[:90]
    texts[part] = texts[part].replace(old, new)
    count += 1


# part 1: ruler and board geometry
sub(1, "x0=-6.6, length=6.0, line_y=.45,", "x0=-6.6, length=6.0, line_y=.85,")
sub(1, """name = block(lines(item["title"][s.language()], 14), (0, ly + 1.55 + .5 * row, 0), size=self.name_size, color=shade, width=1.9)""",
    """name = block(lines(item["title"][s.language()], 14), (0, ly + 1.5 + .45 * row, 0), size=self.name_size, color=shade, width=1.9)""")
sub(1, """parts.append(mn.Line([0, ly + 1.18, 0], [0, ly + 1.78, 0], color=MUTED, stroke_width=1.2, stroke_opacity=.6))""", """parts.append(mn.Line([0, ly + 1.18, 0], [0, ly + 1.72, 0], color=MUTED, stroke_width=1.2, stroke_opacity=.6))""")
sub(1, "y0=1.1, pitch=.66, xl=-4.6, width=4.0", "y0=1.5, pitch=.66, xl=-4.55, width=4.0")
sub(1, """thumb = scene.thumbnail(item["design"], (-6.4, 0, 0), .095, ramp=scene._ramp, value=scene._ramp.low)
            name = block(lines(item["title"][scene.language()], 15), (-5.3, 0, 0), size=14, width=1.55)""",
    """thumb = scene.thumbnail(item["design"], (-6.45, 0, 0), .095, ramp=scene._ramp, value=scene._ramp.low)
            name = block(lines(item["title"][scene.language()], 15), (-5.4, 0, 0), size=14, width=1.4)""")

# part 2
sub(2, """        self.season = "summer"
        self.keep = (self.panel, self.strip, self.footer)
""", """        self.season = "summer"
""")
sub(2, """        content = [item for item in self.mobjects if item is not self._title and item is not self._caption and not any(item is kept for kept in getattr(self, "keep", ()))]""",
    """        kept = (self._caption, getattr(self, "panel", None), getattr(self, "strip", None), getattr(self, "footer", None), getattr(self, getattr(self, "season", "summer"), None))
        content = [item for item in self.mobjects if not any(item is other for other in kept)]""")
sub(2, '''"""🧹 Fade out everything but title, caption and the persistent room panel; live widgets go through static twins."""''', '''"""🧹 Fade out everything but the caption and the persistent room panel; live widgets go through static twins."""''')
sub(2, """        group = mn.VGroup(live(dimension, key), live(specs, key), title, section, self.demand_arrows("summer"))
        self.summer = mn.VGroup(section, group[4])
        return group""", """        self.summer = mn.VGroup(section, self.demand_arrows("summer"))
        return mn.VGroup(live(dimension, key), live(specs, key), title)""")
sub(2, """                return mn.VGroup(_room.text(self.words(self.tr("no overhang", "kein Vordach")), b + np.array([-.6, .22, 0]), size=14, color=WARN))""",
    """                return mn.VGroup(_room.text(self.words(self.tr("no overhang", "kein Vordach")), P(3.2, HEIGHT + SLAB + .5), size=14, color=WARN))""")
sub(2, """group.add(mn.Rectangle(width=width, height=.24, stroke_width=0, fill_color=colour, fill_opacity=.88).move_to([cursor + width / 2, -1.28, 0]))""",
    """group.add(mn.Rectangle(width=width, height=.22, stroke_width=0, fill_color=colour, fill_opacity=.88).move_to([cursor + width / 2, -.62, 0]))""")
sub(2, """        group.add(self.equation([(total_parts, INK)], cursor + .15, -1.28 - .07, size=15))""", """        group.add(self.equation([(total_parts, INK)], cursor + .15, -.62 - .07, size=15))""")
sub(2, """            ys = [-1.64 - .29 * k for k in range(5)]""", """            ys = [-1.0 - .3 * k for k in range(5)]""")
sub(2, """                y = -1.64 - .29 * k
                items = [([self.tr(en, de)], LAYER[layer])]""", """                y = -1.0 - .3 * k
                items = [([self.tr(en, de)], LAYER[layer])]""")
sub(2, """                y = -1.64 - .29 * k
                items = [([self.tr(en, de)], LAYER[layers[k]])""", """                y = -1.0 - .3 * k
                items = [([self.tr(en, de)], LAYER[layers[k]])""")
sub(2, """x, y = 2.1 + 2.45 * (k % 2), -2.38 - .36 * (k // 2)""", """x, y = 2.1 + 2.45 * (k % 2), -2.14 - .33 * (k // 2)""")
sub(2, """        return mn.Square(.14, stroke_width=0, fill_color=colour, fill_opacity=.9).move_to([LEFT + .12, y + .07, 0])""", """        return mn.Square(.14, stroke_width=0, fill_color=colour, fill_opacity=.9).move_to([LEFT + .12, y, 0])""")
sub(2, """            group.add(mn.Rectangle(width=6.8, height=.3, color=INK, stroke_width=2, stroke_opacity=.8).move_to([(LEFT + RIGHT) / 2, y + .06, 0]))""",
    """            group.add(mn.Rectangle(width=6.8, height=.27, color=INK, stroke_width=2, stroke_opacity=.8).move_to([(LEFT + RIGHT) / 2, y, 0]))""")
sub(2, """(([self.n(shown[k], digits)], colour), (["+" if k < 2 else "="], INK))][:-1]""", """(([self.n(shown[k], digits)], colour), (["+" if k < 2 else "="], INK))]""")
text = texts[2]
start, stop = text.index("            group, x0, scale = mn.VGroup(), LEFT + .4, 4.8 / 6"), text.index("            area = round(found[\"window_area\"], 1)")
new = '''            group, x0, scale = mn.VGroup(), LEFT + .4, 4.8 / 6
            group.add(mn.Rectangle(width=max(factor * scale, 1e-3), height=.22, stroke_width=0, fill_color=_room.LIGHT, fill_opacity=.88).move_to([x0 + factor * scale / 2, -.62, 0]),
                      mn.Line([x0, -.62 - .16, 0], [x0 + 6 * scale, -.62 - .16, 0], color=MUTED, stroke_width=1.4))
            for mark in (0, 2, 4, 6):
                group.add(mn.Line([x0 + mark * scale, -.62 - .16, 0], [x0 + mark * scale, -.62 - .22, 0], color=MUTED, stroke_width=1.4),
                          _room.text([self.n(mark), "%"], (x0 + mark * scale, -.62 - .36, 0), size=13, color=MUTED))
            group.add(_room.dashed([x0 + 2 * scale, -.62 + .15, 0], [x0 + 2 * scale, -.62 - .16, 0], color=INK, stroke_width=2),
                      self.equation([([self.n(factor, 2), "%"], _room.LIGHT)], x0 + 6 * scale + .2, -.62 - .07, size=15))
            ys = [-1.4 - .3 * k for k in range(4)]
            sky = [([self.tr("Sky angle", "Himmelswinkel"), "θ", "=", self.n(theta, 1), "~°"], _room.LIGHT)]
            sky.append(([*self.words(self.tr("with the overhang; 75° without", "mit Vordach; 75° ohne"))], MUTED) if depth > .01 else ([*self.words(self.tr("no overhang hides sky", "kein Vordach verdeckt Himmel"))], MUTED))
            group.add(self.marker(_room.LIGHT, ys[0]), self.equation(sky, LEFT + .4, ys[0] - .07, size=14, width=6.6))
'''
texts[2] = text[:start] + new + text[stop:]
sub(2, """self.n(theta, precision), "°", "÷",""", """self.n(theta, precision), "~°", "÷",""")
sub(2, '''"Sun through the window becomes cooling demand; the chiller divides it by its efficiency.", "Sonne durch das Fenster wird zum Kühlbedarf; die Kältemaschine teilt ihn durch ihre Effizienz."''',
    '''"Sun becomes cooling demand; the chiller divides it by its efficiency.", "Sonne wird zum Kühlbedarf; die Kältemaschine teilt ihn durch ihre Effizienz."''')
sub(2, """        title = live(lambda: mn.VGroup(_room.place(_room.text(self.words(by_id(self.focus_id())["title"][language]), size=21, color=INK), (RIGHT + 7.15, 3.3, 0), "right")), focus_key)""",
    """        title = live(lambda: mn.VGroup(_room.place(_room.text(self.words(by_id(self.focus_id())["title"][language]), size=21, color=INK), (6.95, 3.3, 0), "right")), focus_key)""")
for n, text in texts.items():
    open(base + f"r2-part{n}.py", "w", encoding="utf-8").write(text)
print("patched", count)
