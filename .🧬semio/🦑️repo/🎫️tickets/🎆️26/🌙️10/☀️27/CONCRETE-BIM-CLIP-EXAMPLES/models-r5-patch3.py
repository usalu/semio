import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, (s.count(old), old[:100])
    s = s.replace(old, new)


# midsurface
sub('''self.quantity(half, "m", 3), offset=-0.12, color=DATA, size=16.2)
        thick = self.dimension(at(-reach - 0.1, soffit), at(-reach - 0.1, soffit + slab["thickness"]), self.quantity(slab["thickness"], "m"), offset=0.15, color=DATA, size=16.2)''',
    '''self.quantity(half, "m", 3) + STAR, offset=-0.12, color=DATA, size=16.2)
        thick = self.dimension(at(-reach - 0.1, soffit), at(-reach - 0.1, soffit + slab["thickness"]), self.quantity(slab["thickness"], "m") + STAR, offset=0.15, color=DATA, size=16.2)''')
sub('''        terms = [self.quantity(value, "m", 3) for value in (member["physical_height"], half, member["length"])]''', '''        terms = [self.quantity(value, "m", 3) + STAR for value in (member["physical_height"], half, member["length"])]''')
sub('''        title = self.label("line member length", "Länge des Stabelements", (cx, -1.78, 0), size=16.2, color=MUTED)
        self._show("midsurface",''', '''        title = VGroup(self.label("line member length", "Länge des Stabelements", (cx, -1.78, 0), size=16.2, color=MUTED), self._legend((cx - 1.6, -2.45, 0), assumed=True))
        self._show("midsurface",''')
sub('''                    self.text("C3", section.at(piloti["y"], 0.0) + np.array([0.0, -0.25, 0.0]), size=16.2, color=DATA))''', '''                    self.text("C3", section.at(piloti["y"], 0.0) + np.array([0.0, -0.25, 0.0]), size=16.2, color=DATA),
                    self.leader((-3.2, -0.8, 0), section.parts["RF-1A"].get_center(), self.tr("ramp flights", "Rampenläufe"), color=MUTED, size=16.2, anchor="l"))''')

# supports: star instead of the word badge
sub('''                           self.text(first["condition"], (RIGHT - 2.8, 1.55, 0), size=17.5, color=INK, anchor="l", width=4.0),
                           self.badge(self.tr(first["basis"], "angenommen"), (RIGHT + 2.8, 1.55, 0), color=WARN, anchor="r"),''',
    '''                           self.text(first["condition"] + STAR, (RIGHT - 2.8, 1.55, 0), size=17.5, color=INK, anchor="l", width=5.4),''')
sub('''        roof = VGroup(self._roof_note((0.95, -2.3, 0)), self._legend((0.95, -2.03, 0), piloti=True))''', '''        roof = VGroup(self._roof_note((0.95, -2.42, 0)), self._legend((0.95, -2.0, 0), assumed=True, piloti=True))''')

# loads
sub('''(lambda: self.quantity(q(), "kN/m²"), INK, self.quantity(sum(values), "kN/m²"))], (RIGHT, 0.7, 0), size=19.0)''', '''(lambda: self.quantity(q(), "kN/m²") + STAR, INK, self.quantity(sum(values), "kN/m²") + STAR)], (RIGHT, 0.7, 0), size=19.0)''')
sub('''        equation = self._equation([("q =", INK, "q ="), (lambda: self.number(share(0)), STRUCT, self.number(values[0])), ("+", MUTED, "+"),
                                   (lambda: self.number(share(1)), GOOD, self.number(values[1])), ("+", MUTED, "+"), (lambda: self.number(share(2)), DATA, self.number(values[2])),''',
    '''        equation = self._equation([("q =", INK, "q ="), (lambda: self.number(share(0)) + STAR, STRUCT, self.number(values[0]) + STAR), ("+", MUTED, "+"),
                                   (lambda: self.number(share(1)) + STAR, GOOD, self.number(values[1]) + STAR), ("+", MUTED, "+"), (lambda: self.number(share(2)) + STAR, DATA, self.number(values[2]) + STAR),''')
sub('''self.live_label(lambda: "q = " + self.quantity(q(), "kN/m²"), (-6.3, 2.35, 0)''', '''self.live_label(lambda: "q = " + self.quantity(q(), "kN/m²") + STAR, (-6.3, 2.35, 0)''')
sub('''self.quantity(loads["self_weight"]["density"], "kN/m³", 0) + self.tr(" (assumed)", " (angenommen)")], (x0 + 3.0, y0 + 0.15, 0)''', '''self.quantity(loads["self_weight"]["density"], "kN/m³", 0) + STAR], (x0 + 3.0, y0 + 0.15, 0)''')
sub('''self.dimension(strip[0], strip[3], self.quantity(slab["thickness"], "m"), offset=0.15, color=STRUCT, size=16.2),''', '''self.dimension(strip[0], strip[3], self.quantity(slab["thickness"], "m") + STAR, offset=0.15, color=STRUCT, size=16.2),''')
sub('''        sub = self.text(f"{self.number(values[0])} = {self.quantity(loads['self_weight']['thickness'], 'm')} × {self.quantity(loads['self_weight']['density'], 'kN/m³', 0)}"''', '''        sub = self.text(f"{self.number(values[0])}{STAR} = {self.quantity(loads['self_weight']['thickness'], 'm')}{STAR} × {self.quantity(loads['self_weight']['density'], 'kN/m³', 0)}{STAR}"''')
sub('''(self.quantity(sum(values), "kN/m²"), GOOD, self.quantity(sum(values), "kN/m²")), ("=", MUTED, "="),
                                   (lambda: self.quantity(q() * area, "kN", 0)''', '''(self.quantity(sum(values), "kN/m²") + STAR, GOOD, self.quantity(sum(values), "kN/m²") + STAR), ("=", MUTED, "="),
                                   (lambda: self.quantity(q() * area, "kN", 0)''')
sub('''        note = self.label("all three values are assumed: no source gives them", "alle drei Werte sind angenommen: keine Quelle nennt sie", (RIGHT, -2.0, 0), size=16.2, color=WARN, width=5.8)''',
    '''        note = self._legend((RIGHT - 2.9, -2.1, 0), assumed=True)''')

# loss
sub('''        curve, surfaces = analysis["counts"]["curve_members"]''', '''        curve, surfaces = analysis["counts"]["curve_members"]''')
sub('''self.label(f"objects are kept: {self.number(curve, 0)} line members, {self.number(surfaces, 0)} surface member", f"Objekte bleiben: {self.number(curve, 0)} Stabelemente, {self.number(surfaces, 0)} Flächenelement"''',
    '''self.label(f"objects are kept: {self.number(curve, 0)}{STAR} line members, {self.number(surfaces, 0)} surface member", f"Objekte bleiben: {self.number(curve, 0)}{STAR} Stabelemente, {self.number(surfaces, 0)} Flächenelement"''')
sub('''        legend = self.label("Orange: not in the analysis", "Orange: nicht in der Analyse", (-4.55, -2.0, 0), size=16.2, color=WARN)
        self._show("loss",''', '''        legend = VGroup(self.label("Orange: not in the analysis", "Orange: nicht in der Analyse", (-4.55, -2.0, 0), size=16.2, color=WARN), self._legend((-1.35, -2.25, 0), piloti=True))
        self._show("loss",''')

# properties
sub('''self.dimension((x - 0.5, z(0), 0), (x - 0.5, z(piloti["top"]), 0), self.quantity(piloti["top"], "m"), offset=0.2, color=MUTED, size=15.5))''', '''self.dimension((x - 0.5, z(0), 0), (x - 0.5, z(piloti["top"]), 0), self.quantity(piloti["top"], "m") + STAR, offset=0.2, color=MUTED, size=15.5))''')
sub('''self.tr("line ", "Stab ") + self.quantity(member["length"], "m", 3), offset=-0.2''', '''self.tr("line ", "Stab ") + self.quantity(member["length"], "m", 3) + STAR, offset=-0.2''')
sub('''"Ø " + self.quantity(piloti["radius"] * 2, "m"), offset=-(radius + 0.2)''', '''"Ø " + self.quantity(piloti["radius"] * 2, "m") + STAR, offset=-(radius + 0.2)''')
sub('''        keep = [(self.tr("line member ", "Stabelement ") + self.quantity(member["length"], "m", 3), GOOD), (self.tr("section Ø ", "Querschnitt Ø ") + self.quantity(member["diameter"], "m") + self.tr(" · assumed", " · angenommen"), GOOD),
                (self.tr("reinforced concrete ", "Stahlbeton ") + self.quantity(analysis["loads"]["self_weight"]["density"], "kN/m³", 0) + self.tr(" · assumed", " · angenommen"), GOOD),''',
    '''        keep = [(self.tr("line member ", "Stabelement ") + self.quantity(member["length"], "m", 3) + STAR, GOOD), (self.tr("section Ø ", "Querschnitt Ø ") + self.quantity(member["diameter"], "m") + STAR, GOOD),
                (self.tr("reinforced concrete ", "Stahlbeton ") + self.quantity(analysis["loads"]["self_weight"]["density"], "kN/m³", 0) + STAR, GOOD),''')
sub('''        span = f"{tags[0]}–{tags[-1]} ×{self.number(len(tags), 0)}"
        same = self.label(f"the same on all pilotis: {span}", f"gleich bei allen Pilotis: {span}", (1.1, -0.85, 0), size=16.2, color=MUTED, anchor="l", width=5.6)''',
    '''        span = f"{tags[0]}–{tags[-1]} ×{self.number(len(tags), 0)}{STAR}"
        same = VGroup(self.label(f"the same on all pilotis: {span}", f"gleich bei allen Pilotis: {span}", (1.1, -0.85, 0), size=16.2, color=MUTED, anchor="l", width=5.6),
                      self._legend((1.1, -1.4, 0), assumed=True, piloti=True))''')

# trace: kind chart instead of unlabeled squares
start = s.index("    def _records(self):")
end = s.index("    def _trace(self):")
s = s[:start] + '''    def _kind_chart(self, fill):
        """📊 One bar per kind of loss: its height is the number of loss records, its name stands under the bar; the 13 bars sum to the 82 records."""
        groups = self.analysis["loss_groups"]
        if {group["id"] for group in groups} != set(KIND_NAMES):
            raise ValueError("Every loss kind needs a name")
        base, pitch, unit = -0.5, 4.5 / len(groups), 0.02
        names, counts, centres = VGroup(), VGroup(), []
        for index, group in enumerate(groups):
            x = -2.2 + pitch * (index + 0.5)
            centres.append(x)
            name = self.text(self.tr(*KIND_NAMES[group["id"]]), (0, 0, 0), size=15.0, color=INK)
            name.rotate(np.pi / 2)
            name.scale_to_fit_height(min(name.height, 0.8))
            names.add(name.move_to((x, base - 0.07 - name.height / 2, 0)))
            counts.add(self.live_label(lambda fill=fill, value=group["count"]: self.number(round(fill.get_value() * value), 0), (x, base + group["count"] * unit + 0.15, 0), size=15.0, color=WARN))

        def bars():
            return VGroup(*[Rectangle(width=0.2, height=max(group["count"] * unit * fill.get_value(), 1e-3), stroke_width=0, fill_color=WARN, fill_opacity=0.9).move_to(
                (x, base + group["count"] * unit * fill.get_value() / 2, 0)) for x, group in zip(centres, groups)])

        return VGroup(self.live(bars, key=lambda: round(fill.get_value(), 3)), names, counts)

''' + s[end:]
sub('''        records, kinds = ValueTracker(0), ValueTracker(0)
        footer, squares = self._trace_footer(records, kinds), self._records()''', '''        records, kinds, fill = ValueTracker(0), ValueTracker(0), ValueTracker(0)
        footer, squares = self._trace_footer(records, kinds), self._kind_chart(fill)''')
sub('''                          LaggedStart(*[FadeIn(row) for row in squares], lag_ratio=0.12)]))''', '''                          FadeIn(squares), fill.animate(rate_func=LINEAR).set_value(1.0)]))''')

p.write_text(s, encoding="utf-8")
print("part 3 ok")
