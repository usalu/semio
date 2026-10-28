import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:90]
    s = s.replace(old, new)


def replace_method(name, source):
    global s
    start = s.index(f"    def {name}(")
    end = s.index("\n    def ", start + 10)
    s = s[:start] + source.rstrip("\n") + "\n" + s[end:]


# owners: the toolkit ground-floor plan as in the information clip, each object in its model colour
sub('''        plan = self.villa_plan("ground", (-3.3, 0.05, 0), 0.2, layers=DISCIPLINES, colors="discipline", dims=("grid",))
        self.plan = plan''',
    '''        owner = {tag: COLORS[name] for name in DISCIPLINES for tag in self.info[name]["element_tags"]}
        plan = self.villa_plan("ground", (-3.3, 0.05, 0), 0.2, layers=DISCIPLINES, dims=("grid",), north=True, halo=0.07, highlight=list(owner), highlight_colors=owner)
        plan.add(self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        self.plan = plan''')

# check: larger inset, label clear of the ring, bigger marker, no brackets
sub('''marker = Circle(radius=0.17, stroke_color=WARN, stroke_width=2.4).move_to(spot)
        centre, radius = np.array([3.6, 1.0, 0.0]), 1.45''', '''marker = Circle(radius=0.26, stroke_color=WARN, stroke_width=2.4).move_to(spot)
        centre, radius = np.array([3.6, 1.05, 0.0]), 1.6''')
sub('''mark = np.array([centre[0] - radius + 0.37, middle[1] + 0.45, 0.0])''', '''mark = np.array([centre[0] - radius + 0.5, middle[1] + 0.4, 0.0])''')
sub('''(Ø {self.number(piloti['radius'] * 2)} m, assumed)",''', '''(Ø {self.number(piloti['radius'] * 2)} m assumed)",''')
sub('''(Ø {self.number(piloti['radius'] * 2)} m, angenommen)"), (RIGHT, -0.8, 0)''', '''(Ø {self.number(piloti['radius'] * 2)} m angenommen)"), (RIGHT, -0.95, 0)''')
sub('''short by {short} m", f"Abstand {clearance} m < gefordert {required} m · es fehlen {short} m"), (RIGHT, -1.15, 0)''', '''short by {short} m", f"Abstand {clearance} m < gefordert {required} m · es fehlen {short} m"), (RIGHT, -1.28, 0)''')
sub('''(RIGHT, -1.47, 0), size=16.2, color=MUTED, width=5.9))
        chips = VGroup(self.guid_chip(clash["a_tag"], (1.95, -2.1, 0)''', '''(RIGHT, -1.58, 0), size=16.2, color=MUTED, width=5.9))
        chips = VGroup(self.guid_chip(clash["a_tag"], (1.95, -2.15, 0)''')
sub('''self.guid_chip(clash["b_tag"], (5.15, -2.1, 0), color=STRUCT))''', '''self.guid_chip(clash["b_tag"], (5.15, -2.15, 0), color=STRUCT))''')

# translation
replace_method("_translation", '''    def _translation(self):
        """🔁 Beat 11: the structure model on the left, the analysis model on the villa on the right, the three mapping rules in between."""
        analysis, language = self.analysis, self.language()
        surfaces, shells = len({item["source_guid"] for item in analysis["members"] if item["kind"] == "surface"}), analysis["counts"]["shells"]
        struct = self.villa_axon((-4.7, 0.1, 0), 0.1, disciplines=("structure",), grid=True, shell=0.4, ground=False)
        result = self._villa_analysis((4.7, 0.1, 0), 0.1, ids=("members",))
        source = self.text(f"{self.info['structure']['name'][language]} · {self._revision(analysis['source_revision'])}", (-4.7, 2.1, 0), size=17.5, color=STRUCT, width=3.4)
        target = self.label("Structural analysis model", "Tragwerksanalysemodell", (4.7, 2.1, 0), size=17.5, color=GOOD, width=3.4)
        schema = VGroup(self._chip(analysis["target_schema"], (0, 2.65, 0), GOOD, 18.0),
                        self.label("the analysis domain of the same schema", "die Analysedomäne desselben Schemas", (0, 2.27, 0), size=16.2, color=MUTED, width=5.4))
        columns, slab_tag = self._tags("structure", "IfcColumn"), analysis["slab"]["tag"]
        one = self.number(surfaces, 0)
        slab_note = self.tr(f"{one} IfcSlab ({slab_tag}) → {one} IfcStructuralSurfaceMember, meshed into {self.number(shells, 0)} shell elements",
                            f"{one} IfcSlab ({slab_tag}) → {one} IfcStructuralSurfaceMember, vernetzt in {self.number(shells, 0)} Schalenelemente")
        mapping = analysis["mapping"]
        spec = [(mapping[0]["count"], mapping[0]["source_class"], mapping[0]["target_class"], mapping[0][language], lambda: self._lit(struct, columns), [result.parts[tag] for tag in columns]),
                (surfaces, f"IfcSlab · {slab_tag}", mapping[1]["target_class"], slab_note, lambda: self._lit(struct, [slab_tag]), [result.shells]),
                (mapping[2]["count"], mapping[2]["source_class"], mapping[2]["target_class"], mapping[2][language], lambda: self._lit(struct, columns, GOOD), [result.parts[tag] for tag in columns])]
        rules, steps = VGroup(), []
        for index, (count, origin_name, goal_name, text, light, lit_result) in enumerate(spec):
            y, tracker = 1.4 - index * 1.0, ValueTracker(0)
            origin = self._chip(origin_name, (-2.4, y, 0), STRUCT, 16.2, "l")
            arrow = Arrow((-0.95, y, 0), (-0.65, y, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.5)
            goal = self._chip(goal_name, (-0.55, y, 0), GOOD, 16.2, "l")
            note = VGroup(*[self.text(line, (-2.5, y - 0.36 - number * 0.27, 0), size=15.5, color=MUTED, anchor="l", width=5.3) for number, line in enumerate(self._wrap(text, 52))])
            rule = VGroup(self._counter(tracker, (-2.55, y, 0), size=20.0), origin, arrow, goal, note)
            rules.add(rule)
            steps.append((0.9, [FadeIn(rule), tracker.animate(rate_func=LINEAR).set_value(count), light(),
                                LaggedStart(*[Indicate(part, color=DATA, scale_factor=1.0) for part in lit_result], lag_ratio=min(0.5, 2.0 / len(lit_result)))]))
        carry = VGroup(Arrow((-2.8, -1.95, 0), (2.8, -1.95, 0), buff=0.0, color=MUTED, stroke_width=2.0, max_tip_length_to_length_ratio=0.1),
                       self.text(self.tr("translation of ", "Übersetzung von ") + f"{self.info['structure']['name'][language]} {self._revision(analysis['source_revision'])}", (0, -1.68, 0), size=16.2, color=MUTED, width=5.5))
        self._show("translation", (0.8, [self._leave(), FadeIn(struct), FadeIn(source)]), *steps,
                   (1.1, [FadeIn(schema), FadeIn(carry), FadeIn(result, shift=_engine.RIGHT * 0.3), FadeIn(target)]))
        self.stage = VGroup(struct, source, rules, schema, carry, result, target)
''')

# members
replace_method("_members", '''    def _members(self):
        """🧮 Beat 12: the analysis mesh on the real first-floor slab outline: line members on the piloti positions, one surface member in shells between the grid lines, nodes at the crossings."""
        analysis, language = self.analysis, self.language()
        mesh_spec, counts = analysis["mesh"], analysis["counts"]
        columns, rows_n = len(mesh_spec["x_lines"]) - 1, len(mesh_spec["y_lines"]) - 1
        nodes_n = len(mesh_spec["x_lines"]) * len(mesh_spec["y_lines"])
        if columns * rows_n != counts["shells"] or nodes_n != counts["slab_nodes"]:
            raise ValueError("The mesh lines must give the shells and slab nodes of the translation")
        surfaces = len({item["source_guid"] for item in analysis["members"] if item["kind"] == "surface"})
        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=("grid",), ramp_arrow=False)
        base.add(self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan", supports=False)
        groups = {"members": list(mesh.members.submobjects), "shells": list(mesh.shells.submobjects), "nodes": list(mesh.nodes.submobjects)}
        groups["members"] = [VGroup(member, Circle(radius=0.11, stroke_color=STRUCT, stroke_width=1.6).move_to(member.get_center())) for member in groups["members"]]
        mesh.remove(mesh.members, mesh.shells, mesh.nodes, mesh.outline)
        nx, ny = len(mesh_spec["x_lines"]), len(mesh_spec["y_lines"])
        slab_tag = analysis["slab"]["tag"]
        spec = [("IfcStructuralCurveMember", counts["curve_members"], ("line members: one per piloti, on the piloti positions", "Stabelemente: eines je Pilotistütze, auf den Pilotipositionen"), groups["members"], STRUCT),
                ("IfcStructuralSurfaceMember", surfaces, (f"one surface member: the first-floor slab {slab_tag}", f"ein Flächenelement: die Obergeschossdecke {slab_tag}"), [mesh.outline], STRUCT),
                (self.tr("shell elements", "Schalenelemente"), counts["shells"], (f"{columns} × {rows_n} between the grid lines and the façade edges", f"{columns} × {rows_n} zwischen Rasterlinien und Fassadenkanten"), groups["shells"], GOOD),
                ("IfcStructuralPointConnection", counts["slab_nodes"], (f"slab nodes: {nx} × {ny} at the line crossings", f"Deckenknoten: {nx} × {ny} an den Linienkreuzungen"), groups["nodes"], DATA)]
        rows, steps = VGroup(), []
        for index, (name, value, text, parts, color) in enumerate(spec):
            y, tracker = 2.45 - index * 0.68, ValueTracker(0)
            rows.add(VGroup(self._counter(tracker, (0.95, y, 0), size=21.0), self.text(name, (1.1, y, 0), size=17.5, color=color, anchor="l", width=5.5),
                            self.label(*text, (1.1, y - 0.28, 0), size=15.5, color=MUTED, anchor="l", width=5.5)))
            reveal = LaggedStart(*[FadeIn(part) for part in parts], lag_ratio=min(0.6, 1.0 / max(len(parts), 1)))
            steps.append((1.0, [FadeIn(rows[-1], run_time=0.2), tracker.animate(rate_func=LINEAR).set_value(value), reveal]))
        void, gross, net = analysis["slab"]["void_area"], analysis["slab"]["gross_area"], analysis["slab"]["net_area"]
        notes = VGroup(*self._lines([self.tr("line member ≠ column solid", "Stabelement ≠ Stützenkörper"), self.tr("surface member ≠ slab solid", "Flächenelement ≠ Deckenkörper")],
                                    (0.95, -0.35, 0), size=17.0, color=INK, pitch=0.32, width=5.6),
                       *self._lines([self.tr(f"The mesh ignores the ramp void of {self.quantity(void, 'm²')}:", f"Das Netz ignoriert die Rampenöffnung von {self.quantity(void, 'm²')}:"),
                                     self.tr(f"analysis slab {self.quantity(gross, 'm²')}, model slab {self.quantity(net, 'm²')}.", f"Analysedecke {self.quantity(gross, 'm²')}, Modelldecke {self.quantity(net, 'm²')}.")],
                                    (0.95, -1.25, 0), size=16.2, color=WARN, pitch=0.28, width=5.7))
        self._show("members", (0.9, [self._leave(), FadeIn(base)]), *steps, (0.8, [FadeIn(notes)]))
        self.stage = VGroup(base, rows, notes, mesh, *[VGroup(*items) for items in groups.values()], mesh.outline)
''')

# midsurface
sub('''        section = self.villa_section((-3.3, 0.55, 0), 0.3, cut=("x", piloti["x"]), layers=("structure",), highlight=("C3",), levels=False, level_lines=False, labels=False)
        joint = section.at(piloti["y"], mid_z)''',
    '''        section = self.villa_section((-4.0, 0.6, 0), 0.27, cut=("x", piloti["x"]), layers=("structure", "architecture"), highlight=("C3",), level_lines=False, labels=False)
        section.add(self.scale_bar((-6.4, -1.3, 0), 0.27, 5.0, steps=2),
                    self.text(analysis["slab"]["tag"], section.at(2.0, 3.5) + np.array([0.0, 0.3, 0.0]), size=16.2, color=INK),
                    self.text("C3", section.at(piloti["y"], 0.0) + np.array([0.0, -0.25, 0.0]), size=16.2, color=DATA))
        joint = section.at(piloti["y"], mid_z)''')
sub('''zoom, cx, cy, reach, bottom = 2.0, 3.6, 0.45, 1.0, soffit - 0.8''', '''zoom, cx, cy, reach, bottom = 2.0, 3.45, 0.45, 1.0, soffit - 0.8''')
sub('''frame = frame_box(4.5, 3.3, MUTED, fill=0.0).move_to((cx, cy - 0.15, 0))''', '''frame = frame_box(5.4, 3.3, MUTED, fill=0.0).move_to((cx + 0.35, cy - 0.15, 0))''')
sub('''(cx - 2.25, cy + 1.5, 0)), (joint + (0.2, -0.2, 0), (cx - 2.25, cy - 1.8, 0))]''', '''(cx - 2.35, cy + 1.5, 0)), (joint + (0.2, -0.2, 0), (cx - 2.35, cy - 1.8, 0))]''')

# supports
sub('''        condition = VGroup(frame_box(5.9, 0.9, WARN).move_to((RIGHT, 1.35, 0)), self.text(first["condition"] + " = " + first["fixity"], (RIGHT - 2.8, 1.55, 0), size=17.5, color=INK, anchor="l", width=4.0),''',
    '''        condition = VGroup(frame_box(5.9, 0.9, WARN).move_to((RIGHT, 1.35, 0)),
                           self.text(first["condition"] + ": " + self.tr("translations and rotations restrained", "Verschiebungen und Verdrehungen gebunden"), (RIGHT - 2.8, 1.55, 0), size=16.2, color=INK, anchor="l", width=4.3),''')
sub('''        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=("grid",), ramp_arrow=False)
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan")''', '''        base = self.villa_plan("first", LEFT_PLAN, 0.2, layers=("structure",), dims=("grid",), ramp_arrow=False)
        base.add(self.scale_bar((-1.05, -2.1, 0), 0.2, 5.0, steps=2))
        mesh = self.analysis_view(analysis, LEFT_PLAN, 0.2, projection="plan")''')

# loads
sub('''        axon = self.analysis_view(analysis, (-3.4, 0.2, 0), 0.17, projection="axon")''', '''        axon = self._villa_analysis((-3.4, 0.2, 0), 0.17, nodes=True)''')

# loss
replace_method("_loss", '''    def _loss(self):
        """🗑️ Beat 16: what the analysis does not keep, one row per kind, each tied to the elements it came from; rows with an object count add up to the headline."""
        analysis, language = self.analysis, self.language()
        villa = self.villa_axon((-4.55, 0.3, 0), 0.12, colors="discipline", ground=False)
        services = self.info["services"]["element_tags"]
        void_area = analysis["slab"]["void_area"]
        groups = []
        for item in analysis["loss_groups"]:
            if item["class"] == "IfcColumn":
                continue
            kind = KIND_LABELS.get(item["predefined_type"] or item["property"].split(".")[-1])
            counted = item["property"] == "element"
            head = f"{item['class']}" + (" · " + kind[0 if language == "en" else 1] if kind and counted else "") + (f" ×{self.number(item['count'], 0)}" if counted else "")
            if item["property"] == "voids":
                head = self.tr("ramp void", "Rampenöffnung") + " · " + self.quantity(void_area, "m²")
            elif item["property"].startswith("material.layer"):
                head = self.tr(f"render layer · {self.number(item['count'], 0)} façades", f"Putzschicht · {self.number(item['count'], 0)} Fassaden")
            groups.append((head, item["source_tags"], item[language].split(": ", 1)[-1], counted, item["property"] == "voids"))
        groups.append((self.tr("services", "Haustechnik") + f" ×{self.number(len(services), 0)}", services, self.tr("boiler, radiators, drains, pipes: no structural meaning", "Kessel, Heizkörper, Abläufe, Rohre: ohne statische Bedeutung"), True, False))
        table, steps, links = VGroup(), [], VGroup()
        for index, (head, tags, sentence, counted, void) in enumerate(groups):
            y = 2.62 - index * 0.262
            row = VGroup(self.text(head, (-2.0, y, 0), size=16.2, color=WARN, anchor="l", width=2.45), self.text(sentence, (0.6, y, 0), size=16.2, color=INK, anchor="l", width=6.0))
            table.add(row)
            parts = [villa.parts[key] for tag in tags for key in self._keys(villa, tag)] if counted else []
            keys = [key for tag in tags for key in self._keys(villa, tag)]
            target = villa.at(8.8, 8.65, self.geometry["storeys"]["first"]) if void else np.mean([villa.parts[key].get_center() for key in keys], axis=0)
            line = Line((-2.08, y, 0), target, stroke_width=0.8, color=WARN, stroke_opacity=0.5)
            links.add(line, Dot(target, radius=0.04, color=WARN))
            steps.append(AnimationGroup(FadeIn(row, run_time=0.2), FadeIn(VGroup(line, links[-1]), run_time=0.2), *[part.animate.set_stroke(WARN, 2.0, 1.0).set_fill(WARN, 0.2) for part in parts]))
        dropped = len({tag for _, tags, _, counted, _ in groups if counted for tag in tags})
        kept = len({item["source_tag"] for item in analysis["members"]})
        if dropped + kept != len(set().union(*[set(info["element_guids"]) for info in self.info.values()])):
            raise ValueError("Dropped and kept objects must add up to the unique objects of the three models")
        if sum(len(set(tags)) for _, tags, _, counted, _ in groups if counted) != dropped:
            raise ValueError("Counted loss rows must not share objects")
        curve, surfaces = analysis["counts"]["curve_members"], len({item["source_guid"] for item in analysis["members"] if item["kind"] == "surface"})
        gone, stays = ValueTracker(0), ValueTracker(0)
        footer = VGroup(self._counter(gone, (-1.5, -0.95, 0), size=22.0, color=WARN, anchor="r"),
                        self.label("objects are not in the analysis: the counted rows above", "Objekte fehlen in der Analyse: die gezählten Zeilen oben", (-1.35, -0.95, 0), size=17.0, color=INK, anchor="l", width=6.0),
                        self._counter(stays, (-1.5, -1.4, 0), size=22.0, color=GOOD, anchor="r"),
                        self.label(f"objects are kept: {self.number(curve, 0)} line members, {self.number(surfaces, 0)} surface member", f"Objekte bleiben: {self.number(curve, 0)} Stabelemente, {self.number(surfaces, 0)} Flächenelement", (-1.35, -1.4, 0), size=17.0, color=INK, anchor="l", width=6.0),
                        self.label("together: the unique objects of the three models", "zusammen: die eindeutigen Objekte der drei Modelle", (-1.35, -1.85, 0), size=16.2, color=MUTED, anchor="l", width=6.0))
        legend = self.label("Orange: not in the analysis", "Orange: nicht in der Analyse", (-4.55, -2.0, 0), size=16.2, color=WARN)
        self._show("loss", (0.8, [self._leave(), FadeIn(villa)]), (2.8, [LaggedStart(*steps, lag_ratio=0.35), FadeIn(legend)]),
                   (0.7, [FadeIn(footer), gone.animate(rate_func=LINEAR).set_value(dropped), stays.animate(rate_func=LINEAR).set_value(kept)]))
        self.stage = VGroup(villa, table, links, footer, legend)
''')

# properties: assumed tags
sub('''(self.tr("section Ø ", "Querschnitt Ø ") + self.quantity(member["diameter"], "m"), GOOD),''', '''(self.tr("section Ø ", "Querschnitt Ø ") + self.quantity(member["diameter"], "m") + self.tr(" · assumed", " · angenommen"), GOOD),''')
sub('''(self.tr("reinforced concrete ", "Stahlbeton ") + self.quantity(analysis["loads"]["self_weight"]["density"], "kN/m³", 0), GOOD),''', '''(self.tr("reinforced concrete ", "Stahlbeton ") + self.quantity(analysis["loads"]["self_weight"]["density"], "kN/m³", 0) + self.tr(" · assumed", " · angenommen"), GOOD),''')

# trace
replace_method("_trace_footer", '''    def _trace_footer(self, records, kinds):
        """📏 Reserve measured live-counter widths and center the localized loss-record row."""
        labels = [self.label("loss records in", "Verlustvermerke in", (0, 0, 0), size=17.0, color=INK, anchor="l"),
                  self.label("kinds: the only trace of what was dropped", "Arten: die einzige Spur des Weggelassenen", (0, 0, 0), size=17.0, color=INK, anchor="l", width=5.5)]
        totals = [len(self.analysis["losses"]), len(self.analysis["loss_groups"])]
        widths = [max(self.text(self.number(value, 0), (0, 0, 0), size=21.0, color=WARN).width for value in range(total + 1)) for total in totals]
        gap, footer = 0.15, VGroup()
        x = -(sum(widths) + sum(label.width for label in labels) + 3 * gap) / 2
        for tracker, label, width in zip((records, kinds), labels, widths):
            x += width
            counter = self._counter(tracker, (x, -2.1, 0), size=21.0, color=WARN)
            label.shift((x + gap - label.get_left()[0], -2.1 - label.get_center()[1], 0))
            footer.add(counter, label)
            x += 2 * gap + label.width
        return footer

    def _records(self):
        """🟧 One orange square per loss record, grouped by kind, so the 82 records and 13 kinds are drawn."""
        squares, groups, x = [], VGroup(), -4.9
        for group in self.analysis["loss_groups"]:
            row = VGroup()
            for _ in range(group["count"]):
                row.add(Rectangle(width=0.08, height=0.1, stroke_width=0, fill_color=WARN, fill_opacity=0.9).move_to((x, -2.42, 0)))
                x += 0.1
            x += 0.12
            groups.add(row)
        return groups
''')
sub('''        struct = self.villa_axon((-3.9, 0.8, 0), 0.105, disciplines=("structure",), highlight=("C3",), ground=False)
        result = self.analysis_view(analysis, (3.9, 0.8, 0), 0.105, projection="axon", highlight=("C3",))
        heads = VGroup(self.text(f"{self.info['structure']['name'][language]} · {self._revision(analysis['source_revision'])}", (-3.9, 2.5, 0), size=17.5, color=STRUCT, width=3.8),
                       self.label("Structural analysis model", "Tragwerksanalysemodell", (3.9, 2.5, 0), size=17.5, color=GOOD, width=3.8))''',
    '''        struct = self.villa_axon((-3.9, 1.05, 0), 0.098, disciplines=("structure",), grid=True, highlight=("C3",), ground=False)
        result = self._villa_analysis((3.9, 1.05, 0), 0.098, ids=("members",), highlight=("C3",))
        heads = VGroup(self.text(f"{self.info['structure']['name'][language]} · {self._revision(analysis['source_revision'])}", (-3.9, 2.75, 0), size=17.5, color=STRUCT, width=3.8),
                       self.label("Structural analysis model", "Tragwerksanalysemodell", (3.9, 2.75, 0), size=17.5, color=GOOD, width=3.8))''')
sub('''        one_way = VGroup(Arrow((-0.9, 1.7, 0), (0.9, 1.7, 0), buff=0.0''', '''        one_way = VGroup(Arrow((-0.9, 1.9, 0), (0.9, 1.9, 0), buff=0.0''')
sub('''Arrow((0.9, 1.15, 0), (-0.9, 1.15, 0), buff=0.0''', '''Arrow((0.9, 1.35, 0), (-0.9, 1.35, 0), buff=0.0''')
sub('''segments([((-0.2, 1.0, 0), (0.2, 1.3, 0)), ((-0.2, 1.3, 0), (0.2, 1.0, 0))], WARN, 3.0),
                         self.label("translation", "Übersetzung", (0, 2.0, 0), size=16.2, color=MUTED),
                         self.label("no automatic way back", "kein automatischer Rückweg", (0, 0.75, 0), size=15.5, color=WARN, width=2.4))''',
    '''segments([((-0.2, 1.2, 0), (0.2, 1.5, 0)), ((-0.2, 1.5, 0), (0.2, 1.2, 0))], WARN, 3.0),
                         self.label("translation", "Übersetzung", (0, 2.2, 0), size=16.2, color=MUTED),
                         self.label("no automatic way back", "kein automatischer Rückweg", (0, 0.95, 0), size=15.5, color=WARN, width=2.4))''')
sub('''        chips = VGroup(self.guid_chip("C3", (-3.9, -1.75, 0), color=DATA),
                       self.label("source GlobalId of member " + member["id"], "Quell-GlobalId des Elements " + member["id"], (3.9, -1.18, 0), size=15.5, color=MUTED, width=3.9),
                       self.guid_chip("C3", (3.9, -1.75, 0), color=DATA, label=False),
                       self.text("=", (0, -1.75, 0), size=30.0, color=GOOD))''',
    '''        chips = VGroup(self.guid_chip("C3", (-3.9, -1.5, 0), color=DATA),
                       self.label("source GlobalId of member " + member["id"], "Quell-GlobalId des Elements " + member["id"], (3.9, -0.98, 0), size=15.5, color=MUTED, width=3.9),
                       self.guid_chip("C3", (3.9, -1.5, 0), color=DATA, label=False),
                       self.text("=", (0, -1.5, 0), size=30.0, color=GOOD))''')
sub('''        footer = self._trace_footer(records, kinds)
        self._show("trace", (0.9, [self._leave(), FadeIn(struct), FadeIn(result), FadeIn(heads)]),
                   (1.2, [FadeIn(c3), FadeIn(chips)]), (1.0, [FadeIn(one_way)]),
                   (0.8, [FadeIn(footer), records.animate(rate_func=LINEAR).set_value(len(analysis["losses"])), kinds.animate(rate_func=LINEAR).set_value(len(analysis["loss_groups"]))]))
        self.stage = VGroup(struct, result, heads, c3, one_way, chips, footer)''',
    '''        footer, squares = self._trace_footer(records, kinds), self._records()
        self._show("trace", (0.9, [self._leave(), FadeIn(struct), FadeIn(result), FadeIn(heads)]),
                   (1.2, [FadeIn(c3), FadeIn(chips)]), (1.0, [FadeIn(one_way)]),
                   (1.2, [FadeIn(footer), records.animate(rate_func=LINEAR).set_value(len(analysis["losses"])), kinds.animate(rate_func=LINEAR).set_value(len(analysis["loss_groups"])),
                          LaggedStart(*[FadeIn(row) for row in squares], lag_ratio=0.12)]))
        self.stage = VGroup(struct, result, heads, c3, one_way, chips, footer, squares)''')

p.write_text(s, encoding="utf-8")
print("part 2 ok")
