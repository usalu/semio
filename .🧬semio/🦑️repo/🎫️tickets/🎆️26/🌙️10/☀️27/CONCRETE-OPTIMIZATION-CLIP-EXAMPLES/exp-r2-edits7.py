REPLACEMENTS = [
    ('''            chips = [(_room.chip([self.tr("overhang", "Vordach"), *_room.parts(d, "m", 1, german)], size=size - 1, color=OPEN), top + slab + .35, x_tip),
                     (_room.chip([self.tr("window", "Fenster")''', '''            chips = [(_room.chip([self.tr("window", "Fenster")'''),
    ('''                     (_room.chip([self.tr("insulation", "Dämmung"), *_room.parts(t * 100, "cm", 0, german)], size=size - 1, color=_room.INSULATION), -.62, x_o)]
            lowest = math.inf''', '''                     (_room.chip([self.tr("insulation", "Dämmung"), *_room.parts(t * 100, "cm", 0, german)], size=size - 1, color=_room.INSULATION), -.62, x_o)]
            over = _room.chip([self.tr("overhang", "Vordach"), *_room.parts(d, "m", 1, german)], size=size - 1, color=OPEN)
            _room.place(over, P(.15, top + slab + .3), "left")
            notes.add(over, mn.Line(over.get_left() + np.array([-.02, 0, 0]), P(x_tip, top + slab + .35), color=OPEN, stroke_width=1.4, stroke_opacity=.8))
            lowest = math.inf'''),
    ('P(2.7, top + slab + .55), "center"))', 'P(2.7, top + slab + 1.1), "center"))'),
    ('(centre[0] + 1.62, centre[1] + .9, 0), "right")', '(centre[0] + 1.5, centre[1] + .9, 0), "right")'),
    ('point = np.array(card["centre"]) + np.array([.62, .92, 0])', 'point = np.array(card["centre"]) + np.array([1.4, .45, 0])'),
    ('_room.place(self.source, (7.0, 3.62, 0), "right")', '_room.place(self.source, (6.88, 3.62, 0), "right")'),
    ('''(1.2, 2.84, 0), 15, _room.SUN, 5.8)''', '''(1.2, 2.84, 0), 14, _room.SUN, 5.65)'''),
    ('''.arrange(RIGHT, buff=.14).move_to((PANEL_X, -2.05, 0))''', '''.arrange(RIGHT, buff=.12).scale_to_fit_width(6.2).move_to((PANEL_X, -2.05, 0))'''),
    ('''        top = self._at_plot(10, YMAX)
        legend = _mn.VGroup(self._at("● " + self.tr("no overhang", "kein Vordach"), (top[0] + .2, top[1] - .15, 0), 15, OPEN, 1.9),
                            self._at("■ " + self._depth(HIGH_D) + " " + self.tr("overhang", "Vordach"), (top[0] + .2, top[1] - .42, 0), 15, SHADED, 1.9))''',
     '''        corner = self._at_plot(50, 0)
        legend = _mn.VGroup(self._at("● " + self.tr("no overhang", "kein Vordach"), (corner[0] + .35, corner[1] + .58, 0), 15, OPEN, 1.9),
                            self._at("■ " + self._depth(HIGH_D) + " " + self.tr("overhang", "Vordach"), (corner[0] + .35, corner[1] + .36, 0), 15, SHADED, 1.9))'''),
    ('''        key_x, key_y = self._at_plot(10, YMAX)[0] + .2, self._at_plot(10, YMAX)[1] - .7''', '''        key_x, key_y = self._at_plot(50, 0)[0] + .35, self._at_plot(50, 0)[1] + .14'''),
    ('''self._at_plot(10, value) + np.array([.55, 0, 0]), order, gap=.2)''', '''self._at_plot(90, value) + np.array([-.55, 0, 0]), order, gap=.2)'''),
    ('''        gap_x = self._at_plot(70, 0)[0] + .5''', '''        gap_x = self._at_plot(70, 0)[0] + .3'''),
    ('''card_large, arrow_large = self._saving(2, 3, side="right", fraction=.78)''', '''card_large, arrow_large = self._saving(2, 3, side="right", fraction=.85)'''),
    ('''.arrange(DOWN, buff=.04)
        anchor = bottom''', '''.arrange(DOWN, buff=.07)
        anchor = bottom'''),
    ('''            return f"{self._wwr(design['wwr'])} · {self._depth(depth)} · {self.quantity(design['insulation'] * 100, 'cm').replace(SPACE, ' ')}"''',
     '''            return f"{self._wwr(design['wwr'])} · {self._depth(depth)} · {self.quantity(design['insulation'] * 100, 'cm').replace(SPACE, ' ')} {self.tr('insulation', 'Dämmung')}"'''),
    ('''        for card in cards:
            thickness = card["run"]["design"]["insulation"]
            card["group"].add(self._at(self.tr("insulation ", "Dämmung ") + self.quantity(thickness * 100, "cm").replace(SPACE, " "), (card["centre"][0] - 1.6, card["centre"][1] - 1.0, 0), 14, _room.INSULATION, 1.9))
''', ''),
    ('self._cube((5.0, -.75, 0)), self._t(self.tr("full plan: 2³ = 8 simulations · fraction: 4", "voller Plan: 2³ = 8 Simulationen · Teilplan: 4"), (3.0, -.75, 0), 17, GOOD, 3.8),',
     'self._cube((6.3, -.75, 0)), self._t(self.tr("full plan: 2³ = 8 simulations · fraction: 4", "voller Plan: 2³ = 8 Simulationen · Teilplan: 4"), (3.35, -.75, 0), 17, GOOD, 4.2),'),
    ('(PANEL_X, -1.35, 0), 16, INK, 6.4))\n        self.step("purpose_2"', '(PANEL_X, -1.6, 0), 16, INK, 6.4))\n        self.step("purpose_2"'),
    ('''self._t(self.tr("Cooling electricity", "Kühlstrom"), (BIG_BAR[0], BIG_BAR[1] - .55, 0), 14, COOL, 1.4))
        return group''', '''self._t(self.tr("Cooling electricity", "Kühlstrom"), (BIG_BAR[0], BIG_BAR[1] - .55, 0), 14, COOL, 2.1))
        return group'''),
    ('''self._t(self.tr("Cooling electricity", "Kühlstrom"), (BIG_BAR[0], BIG_BAR[1] - .55, 0), 14, COOL, 1.4))
        ghost =''', '''self._t(self.tr("Cooling electricity", "Kühlstrom"), (BIG_BAR[0], BIG_BAR[1] - .55, 0), 14, COOL, 2.1))
        ghost ='''),
    ('''self._t(self.tr("Cooling electricity", "Kühlstrom"), (base[0], floor - .55, 0), 14, COOL, 1.6))''', '''self._t(self.tr("Cooling electricity", "Kühlstrom"), (base[0], floor - .55, 0), 14, COOL, 2.1))'''),
    ('''self._at(self.tr("straight line through the two levels", "Gerade durch die zwei Stufen"), (key_x + .5, key_y, 0), 14, MUTED, 3.0))''', '''self._at(self.tr("straight line, two levels", "Gerade durch zwei Stufen"), (key_x + .5, key_y, 0), 14, MUTED, 2.6))'''),
    ('''self._at(self.tr("simulated rooms at 50 % window", "simulierte Räume bei 50 % Fenster"), (key_x + .5, key_y - .3, 0), 14, DATA, 3.0))''', '''self._at(self.tr("simulated rooms, 50 % window", "simulierte Räume, 50 % Fenster"), (key_x + .5, key_y - .3, 0), 14, DATA, 2.6))'''),
    ('''        key_x, key_y = axes.c2p(.45, 500)[0], axes.c2p(.45, 500)[1] - .12''', '''        key_x, key_y = axes.c2p(.55, 500)[0], axes.c2p(.55, 500)[1] - .12'''),
    ('''.arrange(DOWN, buff=.04)
            _room.place(ghost_top''', '''.arrange(DOWN, buff=.08)
            _room.place(ghost_top'''),
    ('''_room.place(ghost_top, base + UP * (bare["response"] * unit + .42), "center")''', '''_room.place(ghost_top, base + UP * (bare["response"] * unit + .45) + RIGHT * .3, "center")'''),
]
