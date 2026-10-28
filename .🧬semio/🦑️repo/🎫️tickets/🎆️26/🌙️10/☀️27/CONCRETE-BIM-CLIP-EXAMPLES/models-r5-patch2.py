import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (s.count(old), old[:100])
    s = s.replace(old, new)


# helpers: grid chains and ground plan
sub('''    def _timeline(self, *stages):''', '''    def _chains(self, plan):
        """📏 Grid dimension chains on top and right of a plan; the short façade bays carry their number above the extension lines."""
        box, grid = self.geometry["box"], self.geometry["grid"]
        xs, ys, group = [0.0, *grid["x"], box["length"]], list(grid["y"]), VGroup()
        for first, second in zip(xs, xs[1:]):
            a, b = plan.to_screen(first, box["width"]), plan.to_screen(second, box["width"])
            group.add(self.dimension(a, b, None, offset=0.45, color=MUTED), self.text(self.number(second - first, 2), (a + b) / 2 + np.array([0.0, 0.67, 0.0]), size=15.5, color=MUTED))
        for first, second in zip(ys, ys[1:]):
            a, b = plan.to_screen(box["length"], first), plan.to_screen(box["length"], second)
            group.add(self.dimension(a, b, self.number(second - first, 2), offset=-0.45, color=MUTED, size=15.5))
        return group

    def _ground_plan(self, scale=0.22):
        """🗺️ The toolkit ground-floor plan in model colours with heavy poché on the cut walls, the double-line glass wall and filled pilotis."""
        owner = {tag: COLORS[name] for name in DISCIPLINES for tag in self.info[name]["element_tags"]}
        plan = self.villa_plan("ground", (-3.3, 0.1, 0), scale, layers=DISCIPLINES, dims=(), north=True, halo=0.0, highlight=list(owner), highlight_colors=owner)
        walls = _sheet.ground_walls(self.geometry)
        cut = VGroup()
        for a, b, _ in walls["outer"] + walls["inner"]:
            ring = [plan.to_screen(point[0], point[1])[:2] for point in _plan.quad(a, b, max(walls["thickness"] / 2, 0.05 / scale))]
            cut.add(polygon(ring, mix(BACKGROUND, ARCH, 0.55), 1.0, ARCH, 1.6), hatch(ring, ARCH, 0.05, 45, 0.7, 0.9))
        for item in self.geometry["pilotis"]:
            cut.add(Circle(radius=0.07, stroke_color=INK, stroke_width=1.0, fill_color=STRUCT, fill_opacity=1.0).move_to(P(plan.to_screen(item["x"], item["y"]))))
        plan.add(cut, self._chains(plan), self.scale_bar((-1.65, -2.4, 0), scale, 5.0, steps=2))
        plan.walls = cut
        return plan

    def _timeline(self, *stages):''')

# schema
sub('''self.live_label(lambda drive=drive, value=value: self.number(round(drive.get_value() * value), 0), (x, y, 0), size=15.5, color=INK))''',
    '''self.live_label(lambda drive=drive, value=value, star=(ifc_class == "IfcColumn"): self.number(round(drive.get_value() * value), 0) + (STAR if star else ""), (x, y, 0), size=15.5, color=INK))''')
sub('''        self._show("schema", (0.7, [self._leave(), FadeIn(bar), FadeIn(labels)]), *columns)
        self.stage = VGroup(bar, labels, parts)''', '''        key_note = self._legend((-6.6, 2.15, 0), piloti=True, width=2.0)
        self._show("schema", (0.7, [self._leave(), FadeIn(bar), FadeIn(labels), FadeIn(key_note)]), *columns)
        self.stage = VGroup(bar, labels, parts, key_note)''')

# slab cards with stars, no basis column words
sub('''        left = self._property_card("architecture", 2.05, [(self.tr("Net floor area", "Netto-Geschossfläche"), self.quantity(slab["net_area"], "m²"), "assumed"),
                                                         ("IsExternal", self._flag("SL-1", "IsExternal"), "assumed")])
        right = self._property_card("structure", -0.35, [(self.tr("Thickness", "Dicke"), self.quantity(slab["thickness"], "m"), "assumed"),
                                                        ("LoadBearing", self._flag("SL-1", "LoadBearing"), "sourced")])''',
    '''        left = self._property_card("architecture", 2.05, [("Qto_SlabBaseQuantities.NetArea", self.quantity(slab["net_area"], "m²") + STAR),
                                                         ("Pset_SlabCommon.IsExternal", self._flag("SL-1", "IsExternal") + STAR)])
        right = self._property_card("structure", -0.35, [(self.tr("Thickness", "Dicke"), self.quantity(slab["thickness"], "m") + STAR),
                                                        ("Pset_SlabCommon.LoadBearing", self._flag("SL-1", "LoadBearing") + STAR)])''')
sub('''self.quantity(slab["thickness"], "m"), offset=0.15, color=DATA, size=16.2),
                         self.label("slab section", "Deckenschnitt"''', '''self.quantity(slab["thickness"], "m") + STAR, offset=0.15, color=DATA, size=16.2),
                         self.label("slab section", "Deckenschnitt"''')
sub('''self._roof_note((0.95, -2.2, 0)))
        spacing = _model.clearance()["grid_spacing"]''', '''self._roof_note((0.95, -2.2, 0)), self._legend((0.95, -2.45, 0), assumed=True))
        spacing = _model.clearance()["grid_spacing"]''')
sub('''        card = self.property_table([(label, value, "", basis) for label, value, basis in rows], (RIGHT, y, 0), width=5.9,
                                   title=self.info[name]["name"][self.language()] + " · " + self._revision(self.info[name]["revision"]), color=COLORS[name])''',
    '''        card = self.property_table([(label, value, "", "assumed") for label, value in rows], (RIGHT, y, 0), width=5.9, basis=False,
                                   title=self.info[name]["name"][self.language()] + " · " + self._revision(self.info[name]["revision"]), color=COLORS[name])''')

# owners
sub('''        owner = {tag: COLORS[name] for name in DISCIPLINES for tag in self.info[name]["element_tags"]}
        plan = self.villa_plan("ground", (-3.3, 0.05, 0), 0.2, layers=DISCIPLINES, dims=("grid",), north=True, halo=0.07, highlight=list(owner), highlight_colors=owner)
        plan.add(self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        self.plan = plan''', '''        plan = self._ground_plan()
        self.plan = plan''')
sub('''self.text(" · ".join(f"{ifc_class} ×{self.number(count, 0)}" for ifc_class, count in classes.items()), (RIGHT - 2.8, y - 0.14, 0), size=15.0, color=INK, anchor="l", width=5.6))''',
    '''self.text(" · ".join(f"{ifc_class} ×{self.number(count, 0)}" + (STAR if ifc_class == "IfcColumn" else "") for ifc_class, count in classes.items()), (RIGHT - 2.8, y - 0.14, 0), size=15.0, color=INK, anchor="l", width=5.6))''')
sub('''self.label(f"Structural grid {self.quantity(spacing, 'm')} (sourced)", f"Tragraster {self.quantity(spacing, 'm')} (belegt)", (RIGHT, -1.95, 0), size=17.0, color=STRUCT, width=5.6)''',
    '''VGroup(self.label(f"Structural grid {self.quantity(spacing, 'm')}, sourced", f"Tragraster {self.quantity(spacing, 'm')}, Quelle", (RIGHT, -1.95, 0), size=17.0, color=STRUCT, width=5.6),
                     self._legend((RIGHT - 2.9, -2.3, 0), piloti=True))''')

# check
sub('''        mark = np.array([centre[0] - radius + 0.37, middle[1] + 0.45, 0.0])''', '''        mark = np.array([centre[0] - radius + 0.5, middle[1] + 0.4, 0.0])''') if False else None
sub('''segments([(mark + (0.3, -0.08, 0), middle + (0.0, 0.0, 0))], WARN, 1.0, 0.8))''', '''segments([(mark + (0.4, -0.12, 0), middle + (0.0, 0.0, 0))], WARN, 1.0, 0.8))''')
sub('''(Ø {self.number(piloti['radius'] * 2)} m assumed)",''', '''(Ø {self.number(piloti['radius'] * 2)} m{STAR})",''')
sub('''(Ø {self.number(piloti['radius'] * 2)} m angenommen)"), (RIGHT, -0.95, 0)''', '''(Ø {self.number(piloti['radius'] * 2)} m{STAR})"), (RIGHT, -0.95, 0)''')
sub('''clearance {clearance} m < required {required} m · short by {short} m", f"Abstand {clearance} m < gefordert {required} m · es fehlen {short} m"''', '''clearance {clearance} m < required {required} m{STAR} · short by {short} m", f"Abstand {clearance} m < gefordert {required} m{STAR} · es fehlen {short} m"''')
sub('''self.label("assumed rule · issue (BCF) refers to both GlobalIds", "angenommene Regel · Meldung (BCF) verweist auf beide GlobalIds", (RIGHT, -1.58, 0), size=16.2, color=MUTED, width=5.9))''',
    '''self.label("issue (BCF) refers to both GlobalIds", "Meldung (BCF) verweist auf beide GlobalIds", (RIGHT, -1.58, 0), size=16.2, color=MUTED, width=5.9),
                      self._legend((RIGHT - 2.95, -2.47, 0), assumed=True))''')
sub('''self.guid_chip(clash["a_tag"], (1.95, -2.15, 0), color=SERV)''', '''self.guid_chip(clash["a_tag"], (1.95, -2.0, 0), color=SERV)''')
sub('''self.guid_chip(clash["b_tag"], (5.15, -2.15, 0), color=STRUCT)''', '''self.guid_chip(clash["b_tag"], (5.15, -2.0, 0), color=STRUCT)''')

# translation
sub('''        schema = VGroup(self._chip(analysis["target_schema"], (0, 2.65, 0), GOOD, 18.0),''', '''        schema = VGroup(self._chip(self.tr(analysis["target_schema"], f"{analysis['target_schema'].split(' ')[0]} · Strukturanalyse-Domäne"), (0, 2.65, 0), GOOD, 18.0),''')
sub('''            rule = VGroup(self._counter(tracker, (-2.3, y, 0), size=20.0), origin, arrow, goal, note)''', '''            rule = VGroup(self._counter(tracker, (-2.3, y, 0), size=20.0, unit=STAR if "IfcColumn" in origin_name else ""), origin, arrow, goal, note)''')
sub('''        self._show("translation", (0.8, [self._leave(), FadeIn(struct), FadeIn(source)]), *steps,
                   (1.1, [FadeIn(schema), FadeIn(carry), FadeIn(result, shift=_engine.RIGHT * 0.3), FadeIn(target)]))
        self.stage = VGroup(struct, source, rules, schema, carry, result, target)''', '''        key_note = self._legend((-2.2, -2.3, 0), piloti=True)
        self._show("translation", (0.8, [self._leave(), FadeIn(struct), FadeIn(source)]), *steps,
                   (1.1, [FadeIn(schema), FadeIn(carry), FadeIn(result, shift=_engine.RIGHT * 0.3), FadeIn(target), FadeIn(key_note)]))
        self.stage = VGroup(struct, source, rules, schema, carry, result, target, key_note)''')

# members and supports: own chains
sub('''        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=("grid",), ramp_arrow=False)
        base.add(self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan", supports=False)''', '''        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=(), ramp_arrow=False)
        base.add(self._chains(base), self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan", supports=False)''')
sub('''            rows.add(VGroup(self._counter(tracker, (0.95, y, 0), size=21.0), self.text(name, (1.1, y, 0), size=17.5, color=color, anchor="l", width=5.5),''',
    '''            rows.add(VGroup(self._counter(tracker, (0.95, y, 0), size=21.0, unit=STAR if value == counts["curve_members"] else ""), self.text(name, (1.1, y, 0), size=17.5, color=color, anchor="l", width=5.5),''')
sub('''        roof = self._roof_note((0.95, -2.05, 0))
        self._show("members", (0.9, [self._leave(), FadeIn(base)]), *steps, (0.8, [FadeIn(notes), FadeIn(roof)]))
        self.stage = VGroup(base, rows, notes, roof, mesh,''', '''        roof = VGroup(self._roof_note((0.95, -2.05, 0)), self._legend((0.95, -2.35, 0), piloti=True))
        self._show("members", (0.9, [self._leave(), FadeIn(base)]), *steps, (0.8, [FadeIn(notes), FadeIn(roof)]))
        self.stage = VGroup(base, rows, notes, roof, mesh,''')
sub('''        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=("grid",), ramp_arrow=False)
        base.add(self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan")''', '''        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=(), ramp_arrow=False)
        base.add(self._chains(base), self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan")''')
sub('''        row = VGroup(self._counter(count, (0.95, 2.4, 0), size=21.0), self.text("IfcStructuralPointConnection"''', '''        row = VGroup(self._counter(count, (0.95, 2.4, 0), size=21.0, unit=STAR), self.text("IfcStructuralPointConnection"''')
sub('''        roof = self._roof_note((0.95, -2.3, 0))
        chosen = VGroup(roof,''', '''        roof = VGroup(self._roof_note((0.95, -2.3, 0)), self._legend((0.95, -2.03, 0), piloti=True))
        chosen = VGroup(roof,''')

p.write_text(s, encoding="utf-8")
print("part 2 ok")
