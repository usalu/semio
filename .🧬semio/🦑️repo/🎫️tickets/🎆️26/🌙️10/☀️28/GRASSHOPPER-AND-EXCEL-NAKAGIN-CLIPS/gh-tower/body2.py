    def _define(self, spec, key):
        """🧪 Solve a small custom definition on the engine and return it with its solution."""
        defn = _gh.define(spec, key)
        return defn, _gh.evaluate(defn, dict(spec.get("over", {})))

    def _custom(self, defn, sol, layout, group, extra=()):
        """🕸️ Draw a custom engine definition at given positions with its wires and one group."""
        saved = self._defn
        self._defn = defn
        try:
            made = {oid: self._widget(oid, at, sol, {}, None) for oid, at in layout.items()}
            wires = [self._wire(made, sol, w) for w in defn.wires if w.src in made and w.dst in made]
        finally:
            self._defn = saved
        net = _engine.VGroup(*made.values(), *wires)
        net.made, net.wires = made, wires
        frame = self.gh_group(_engine.VGroup(*made.values(), *extra), NAMES[group][0 if self.language() == "en" else 1], color=COLORS[group])
        net.add(frame)
        return net

    def _type_legend(self, at):
        """🎨 Colour chips of the four capsule types."""
        chips = [_engine.VGroup(_engine.Square(0.26, fill_color=_nakd.type_color(t.key), fill_opacity=1, stroke_color=TXT, stroke_width=1.5),
                                self._say(t.key, (0, 0), 20, anchor="l")).arrange(_engine.RIGHT, buff=0.12) for t in _nak.capsule_types()]
        return _engine.VGroup(*chips).arrange(_engine.RIGHT, buff=0.45).move_to(np.array([at[0], at[1], 0.0]))

    def present_tower(self):
        """🎞️ The whole animation: one capsule, the five parts, the count rising with the storeys, the Cull Index mistake, both towers, the swap of a type."""
        lang = self.language()
        self._defn, self._cat, self._cache = _gh.canvas("tower"), _gh.catalogue(), {}
        towers, self._slots = _nak.towers(), _nak.slots()
        self.A, self.B = towers[0], towers[1]
        self._pods = int(_nak.value("core.podium_storeys").value)
        types = {t.key: t for t in _nak.capsule_types()}
        kind = {(s.building, s.tower, s.storey, s.facade, s.item): s.type for s in self._slots}
        total = len(self._slots)
        e, g = _engine, self.tr
        name = lambda key: NAMES[key][0 if lang == "en" else 1]
        sep = "; " if lang == "de" else ", "
        full_solution = self._solve()
        lengths = full_solution.items("st", "Length")
        rotated = full_solution.tree("rot", "Geometry")
        facades, items = len(rotated.paths()), len(rotated.branch(rotated.paths()[0]))
        high = sum(1 for v in lengths if v == lengths[0])
        low = max(1, high // 3)
        settings = lambda oid: self._defn.objects[oid].settings
        self.k = k = e.ValueTracker(low)
        shown = lambda: int(round(k.get_value()))
        self.add(self.gh_background(at=LC, width=LW, height=LH), _fonts.section_heading(self.TITLE[lang]))
        viewport = self.gh_viewport(at=(VX, VY), width=VW, height=VH, label="Perspective")
        self.add(viewport)

        cap_a = self.capsule_axon(types["A"], at=(-4.6, 0.3), scale=0.72)
        one = self._say(g("1 capsule", "1 Kapsel"), (-4.6, -1.5), 26)
        full = self.tower_axon(at=(VX, VY), scale=0.1, lit={(0, 1, 4, 0, 0)}, types=False)
        self._fit(full, viewport)
        counter = e.ValueTracker(1)
        number = self.live_label(lambda: self.number(round(counter.get_value()), 0), (-1.0, 0.3), size=96, color=TXT)
        unit = self._say(g("capsules in the towers", "Kapseln in den Türmen"), (-1.0, -0.6), 26)
        self.step("hook", e.FadeIn(cap_a), e.FadeIn(one), e.FadeIn(full), e.FadeIn(number), e.FadeIn(unit), counter.animate.set_value(total), seconds=6.0)

        full_plain = self.tower_axon(at=(VX, VY), scale=0.1, types=False, tint=lambda key: COLORS["capsules"])
        self._fit(full_plain, viewport)
        marks = self._marks(full_plain)
        samples = {"core": "Center Box", "planes": "Rotate", "capsules": "Orient", "capital": "Center Box", "podium": "Extrude"}
        spots = {"core": (-5.2, 1.45), "planes": (-2.5, 1.45), "capsules": (0.3, 1.45), "capital": (-3.6, -1.1), "podium": (-0.8, -1.1)}
        boxes = []
        for key in GROUP_KEYS:
            part = self.gh_component(samples[key], at=spots[key])
            boxes.append(e.VGroup(part, self.gh_group(part, name(key), color=COLORS[key])))
        part_cards = e.VGroup(*boxes)
        self.step("parts", e.FadeOut(cap_a), e.FadeOut(one), e.FadeOut(number), e.FadeOut(unit), e.FadeOut(full), e.FadeIn(full_plain),
                  e.LaggedStart(*[e.FadeIn(b) for b in boxes], lag_ratio=0.35), e.LaggedStart(*[e.FadeIn(m) for m in marks], lag_ratio=0.3), seconds=6.0)

        n_t = e.ValueTracker(self.B.capsule_storeys)
        step_n = lambda: int(round(n_t.get_value()))
        layout_core = {"side": (-4.7, 1.45), "sh": (-4.7, 0.5), "n": (-4.7, -0.45), "ps": (-4.7, -1.4), "tn": (-1.9, -1.0), "tot": (-0.5, -0.1), "core": (1.2, 0.8)}
        core_net = self.live(lambda: self._net(layout_core, over={"n": step_n()}, group="core"), key=step_n)

        def build_shaft():
            drawing = self.core_axon(self.B, at=(VX, VY), scale=S_T, storeys=step_n() + self._pods)
            return self._pin(drawing, (self.B.center[0], 0.0, 0.0), KBASE)

        shaft = self.live(build_shaft, key=step_n)
        dims = self.live(lambda: self._shaft_dims(shaft, self.B.core_side, (step_n() + self._pods) * self.B.storey_height), key=step_n)
        self.step("core", e.FadeOut(part_cards), *[e.FadeOut(m) for m in marks], e.FadeOut(full_plain), e.FadeIn(core_net), e.FadeIn(shaft), e.FadeIn(dims), n_t.animate.set_value(high // 2), seconds=6.0)
        self.play(n_t.animate.set_value(self.B.capsule_storeys), run_time=2.5)

        levels = e.VGroup(*[self.level_mark(self._pt(shaft, self.B.center[0] - self.B.core_side / 2, self.B.center[1] + self.B.core_side / 2, i * self.B.storey_height),
                                            self.number(i, 0), size=16, length=0.4, side=-1, color=TXT) for i in (self.B.storeys,)])
        tn_box = self._mark(core_net.made["tn"], "core", 0.04)
        self.step("shaft", e.FadeIn(tn_box), e.Indicate(shaft, color=DATA), e.FadeIn(levels), seconds=5.0)

        plane_net = self._net({"ps": (-4.7, 1.55), "sh": (-4.7, 0.5), "n": (-4.7, -0.9), "z0": (-1.8, 1.3), "ser": (-0.5, 0.15), "gp": (1.5, 0.0)}, widths={"gp": 1.4}, group="planes",
                              bridges=(("ser", "Series", "gp", "Stream"),))
        series_axes = e.VGroup(*[self._axes(shaft, self._plane(b[0]), length=2.2, scale=S_T) for b in self._storey_planes()])
        self.step("series", e.FadeOut(core_net), e.FadeOut(tn_box), e.FadeOut(levels), e.FadeOut(dims), e.FadeIn(plane_net),
                  e.LaggedStart(*[e.FadeIn(a) for a in series_axes], lag_ratio=0.1), seconds=6.0)

        closeup = self.tower_axon(at=(VX, VY), scale=S_C, towers=(1,), storeys=3, where=lambda s: False, podium=False, capital=False)
        self._pin(closeup, (self.B.center[0], 0.0, 0.0), CLOSE)
        angle_net = self._net({"p0s": (-5.0, 1.2), "ang": (-5.4, -1.1), "rad": (-3.6, -1.1), "ga": (-1.9, -1.1), "rot": (0.7, 0.2)}, widths={"ang": 1.2}, group="planes")
        first = self._storey_planes()[0]
        facade_planes = [dataclasses.replace(first[items * f], origin=_gh.Point(*(sum(np.array(first[items * f + i].origin.t) for i in range(items)) / items))) for f in range(facades)]
        facade_axes = e.VGroup(*[self._axes(closeup, self._plane(p), length=2.2, labels=False) for p in facade_planes])
        self.step("facades", e.FadeOut(plane_net), e.FadeOut(series_axes), e.FadeOut(shaft), e.FadeIn(closeup), e.FadeIn(angle_net),
                  e.LaggedStart(*[e.FadeIn(a) for a in facade_axes], lag_ratio=0.4), seconds=7.0)

        triple = lambda v, d=3: sep.join(self._fmt(round(float(c), d)) for c in v.t)
        plane = first[0]
        panel = self.gh_panel(["Plane", "O  " + triple(plane.origin) + " m", "X  " + triple(plane.x), "Y  " + triple(plane.y), "Z  " + triple(plane.z)], at=(-2.4, 0.2), width=4.6)
        panel_group = self.gh_group(panel, name("planes"), color=COLORS["planes"])
        big = self._axes(closeup, self._plane(plane), length=3.4, labels=True)
        self.step("axes", e.FadeOut(angle_net), e.FadeOut(facade_axes), e.FadeIn(panel), e.FadeIn(panel_group), e.FadeIn(big), seconds=6.0)

        both = e.VGroup(*[self._axes(closeup, self._plane(first[i]), length=2.6, labels=False) for i in range(items)])
        tags = e.VGroup(*[self._say(f"[{i}]", self._pt(closeup, *self._plane(first[i]).origin) + np.array([0, -0.45, 0]), 20, "#FFFFFF") for i in range(items)])
        sx_panel = self.gh_panel(self._lines(full_solution.tree("sx")), at=(1.4, -0.8), width=1.4)
        items_net = self._net({"pitch": (-4.7, 1.4), "hp": (-1.9, 1.4), "nhp": (-1.9, -0.3), "sx": (0.0, 0.7)}, group="planes", extra=(sx_panel,))
        items_wire = self.gh_wire(items_net.made["sx"].grip_out(), sx_panel.grip_in(), kind="list")
        self.step("items", e.FadeOut(panel), e.FadeOut(panel_group), e.FadeOut(big), e.FadeIn(items_net), e.FadeIn(sx_panel), e.FadeIn(items_wire), e.FadeIn(both), e.FadeIn(tags), seconds=6.0)

        orient_net = self._net({"capp": (-5.0, 1.2), "xy": (-5.4, -0.8), "slots": (-2.3, -1.0), "ori": (0.6, 0.2)}, group="capsules", bridges=(("xy", "Plane", "ori", "Source"),))
        origin = self._plane(first[0])
        source = SimpleNamespace(origin=origin.origin + np.array([6.0, 0.0, 0.0]), x=origin.x, y=origin.y, z=origin.z)
        source_axes = self._axes(closeup, source, length=2.6, labels=False)
        half = np.array([0.0, types["A"].length / 2, types["A"].height / 2])
        start = self.capsule_axon(types["A"], at=self._pt(closeup, *(source.origin + half)), scale=S_C)
        shift = self._pt(closeup, *(origin.origin + half)) - self._pt(closeup, *(source.origin + half))
        slot_cap = self._tower(1, types=False, capital=False, scale=S_C, storeys=3, base=CLOSE)
        self.play(e.FadeOut(items_net), e.FadeOut(sx_panel), e.FadeOut(items_wire), e.FadeOut(tags), e.FadeIn(orient_net), e.FadeIn(source_axes), e.FadeIn(start), run_time=1.5)
        self.step("orient", start.animate.shift(shift), seconds=4.0)
        self.play(e.FadeIn(slot_cap), e.FadeOut(start), e.FadeOut(source_axes), e.FadeOut(both), e.FadeOut(closeup), run_time=1.5)

        types_view = self._tower(low, types=True, capital=False, scale=S_K, storeys=low + self._pods + 1, base=KBASE)
        kinds = [s.type for s in self._slots if s.tower == 1 and s.storey == 0]
        types_spec = {"objects": [{"id": "pat", "component": "Panel", "settings": {"lines": kinds}}, {"id": "cnt", "component": "Panel", "settings": {"lines": [str(len(kinds))]}},
                                  {"id": "ser", "component": "Series", "settings": {}}, {"id": "li", "component": "List Item", "settings": {}}, {"id": "out", "component": "Panel", "settings": {"lines": [""]}}],
                      "wires": ["pat.Stream>li.List", "cnt.Stream>ser.Count", "ser.Series>li.Index", "li.Item>out.Stream"]}
        types_defn, types_sol = self._define(types_spec, "types")
        legend = self._type_legend((-2.6, -2.15))
        types_net = self._custom(types_defn, types_sol, {"pat": (-5.6, 0.2), "cnt": (-5.6, -1.5), "ser": (-3.5, -1.2), "li": (-1.7, 0.3), "out": (0.7, 0.2)}, "capsules")
        self.step("types", e.FadeOut(orient_net), e.FadeOut(slot_cap), e.FadeIn(types_net), e.FadeIn(legend), e.FadeIn(types_view), seconds=6.0)

        count_layout = {"n": (-4.7, 1.4), "ser": (-1.9, 1.4), "ori": (0.3, 1.4), "st": (-2.0, -0.6), "stc": (1.2, -0.3), "stl": (1.2, -1.3)}
        net_live = self.live(lambda: self._net(count_layout, over={"n": shown()}, widths={"stl": 1.2}, group="capsules", bridges=(("ser", "Series", "ori", "Target"),),
                                                panels={"stc": {(0,): [str(shown())]}, "stl": {(0,): [str(v) for v in lengths[:shown()]]}}), key=shown)
        tower_live = self.live(lambda: self._tower(k.get_value()), key=shown)
        formula = self.live_label(lambda: f"{shown()} × {facades} × {items} = {sum(lengths[:shown()])}", (-4.5, -1.6), size=30, color=TXT)
        self.step("count", e.FadeOut(types_net), e.FadeOut(legend), e.FadeOut(types_view), e.FadeIn(net_live), e.FadeIn(tower_live), e.FadeIn(formula), k.animate.set_value(high), seconds=8.0)

        self.step("nowire", e.Indicate(e.VGroup(*net_live.wires), color=GOOD, scale_factor=1.0), seconds=2.5)
        self.play(k.animate.set_value(low + 1), run_time=2.5)
        self.play(k.animate.set_value(high), run_time=2.5)

        empty = {"cullp": {"tree": {"{0;%d}" % i: [] for i in range(len(lengths))}}}
        wrong = self._solve(empty)
        wrong_lengths = wrong.items("st", "Length")
        cull_layout = {"mv": (-5.2, 0.8), "cull": (-2.6, 0.0), "st": (-0.3, 0.1), "stc": (1.7, -0.9), "stl": (1.7, 0.7)}
        cull_wrong = self._net(cull_layout, over=empty, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),))
        cull_right = self._net({**cull_layout, "cullp": (-5.2, -1.0)}, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),))
        sum_wrong = self._say(f"Σ = {sum(wrong_lengths)}", (-4.5, -1.75), 30)
        sum_right = self._say(f"Σ = {sum(lengths)}", (-4.5, -1.75), 30)
        top_planes = self._solve().tree("mv", "Geometry")
        last = top_planes.branch(top_planes.paths()[-1])
        removed = [i for i in range(len(last)) if wrong_lengths[-1] - lengths[-1] and i >= lengths[-1]]
        tower_full = self.live(lambda: self._tower(k.get_value()), key=shown)
        crosses = e.VGroup(*[e.VGroup(e.Circle(radius=0.12, color="#D8362B", stroke_width=4), e.Cross(scale_factor=0.09, stroke_color="#D8362B", stroke_width=4)).move_to(
            self._pt(tower_full, *self._plane(last[i]).origin)) for i in removed])
        self.step("mistake", e.FadeOut(net_live), e.FadeOut(formula), e.FadeIn(cull_wrong), e.FadeIn(sum_wrong), e.FadeIn(crosses), k.animate.set_value(high + 1), seconds=5.0)
        self.step("fix", e.ReplacementTransform(cull_wrong, cull_right), e.ReplacementTransform(sum_wrong, sum_right), e.FadeOut(crosses), seconds=5.0)

        ow_t, ch_t = e.ValueTracker(self.B.capital_overhang), e.ValueTracker(self.B.capital_height)
        round2 = lambda: (round(ch_t.get_value(), 2), round(ow_t.get_value(), 2))
        capital_net = self.live(lambda: self._net({"ow": (-4.7, 1.3), "ch": (-4.7, 0.2), "cw": (-1.9, 1.0), "capital": (1.0, 0.6)}, over={"ow": round2()[1], "ch": round2()[0]},
                                                   group="capital", bridges=(("ow", "Number", "cw", "B"), ("ch", "Number", "capital", "Z"))), key=round2)

        def build_capital():
            shaped = dataclasses.replace(self.B, capital_height=round2()[0], capital_overhang=round2()[1])
            drawing = self.capital_axon(shaped, at=(VX, VY), scale=0.4)
            return self._pin(drawing, (self.B.center[0], self.B.center[1], shaped.height), (VX, VY - 0.7, 0.0))

        cap_live = self.live(build_capital, key=round2)

        def build_cap_dim():
            b = self.B.core_side / 2 + round2()[1]
            x, y = self.B.center[0] + b, self.B.center[1] + b
            return self.dimension(self._pt(cap_live, x, y, self.B.height), self._pt(cap_live, x, y, self.B.height + round2()[0]), self.quantity(round2()[0], "m"), offset=-0.35, color=DATA)

        cap_dim = self.live(build_cap_dim, key=round2)
        self.step("capital", e.FadeOut(cull_right), e.FadeOut(sum_right), e.FadeOut(tower_live), e.FadeIn(capital_net), e.FadeIn(cap_live), e.FadeIn(cap_dim), seconds=4.0)
        self.play(ch_t.animate.set_value(settings("ch")["max"]), ow_t.animate.set_value(settings("ow")["max"]), run_time=2.5)
        self.play(ch_t.animate.set_value(self.B.capital_height), ow_t.animate.set_value(self.B.capital_overhang), run_time=2.0)

        k.set_value(low + 1)
        top_net = self.live(lambda: self._net({"n": (-4.7, 1.5), "tot": (-1.7, 1.5), "chh": (-1.7, 0.0), "czz": (-0.3, 0.75), "capital": (1.3, 0.75)}, over={"n": shown()},
                                               group="capital", bridges=(("n", "Number", "tot", "B"), ("czz", "Result", "capital", "B"))), key=shown)
        height_label = self.live_label(lambda: g("Capital base Z", "Kopf Unterkante Z") + f" = {self._fmt(self._solve({'n': shown()}).value('tot'))} m", (-3.4, -1.5), size=26, color=TXT)
        rising = self.live(lambda: self._tower(k.get_value()), key=shown)
        self.step("ontop", e.FadeOut(capital_net), e.FadeOut(cap_live), e.FadeOut(cap_dim), e.FadeIn(top_net), e.FadeIn(rising), e.FadeIn(height_label), k.animate.set_value(high), seconds=6.0)

        podium_net = self._net({"pw": (-4.7, 1.7), "pd": (-4.7, 0.8), "ps": (-4.7, -0.1), "sh": (-4.7, -1.0), "rect": (-1.7, 1.2), "z0": (-1.7, -0.5), "uzp": (-0.2, -0.5), "ext": (1.3, 0.4)}, group="podium")
        podium_view = self.podium_axon(at=(VX, VY), scale=0.1)
        self._fit(podium_view, viewport, 0.5)
        length, width, height = _nak.value("podium.length").value, _nak.value("podium.width").value, _nak.podium_height()
        p_dims = self._say(f"{self.number(length, 1)} × {self.number(width, 1)} × {self.number(height, 1)} m", (VX, VY - 2.2), 22)
        self.step("podium", e.FadeOut(top_net), e.FadeOut(rising), e.FadeOut(height_label), e.FadeIn(podium_net), e.FadeIn(podium_view), e.FadeIn(p_dims), seconds=5.0)

        named = {oid for key in MEMBERS for oid in MEMBERS[key]}
        sizes = {**{key: len(MEMBERS[key]) for key in MEMBERS}, "planes": len(self._defn.objects) - len(named)}
        extra = {"capsules": len(_gh.canvas("capsule").objects)}
        top_count = max(sizes[key] + extra.get(key, 0) for key in GROUP_KEYS)
        bars = []
        for i, key in enumerate(GROUP_KEYS):
            y = 1.7 - i * 0.8
            main = e.Rectangle(width=max(sizes[key] / top_count * 4.4, 0.1), height=0.42, fill_color=COLORS[key], fill_opacity=0.95, stroke_color=_ink(COLORS[key]), stroke_width=2)
            main.move_to(np.array([-5.0 + main.width / 2, y, 0.0]))
            bar = e.VGroup(main)
            right = main.get_right()[0]
            label = str(sizes[key])
            if key in extra:
                more = e.Rectangle(width=extra[key] / top_count * 4.4, height=0.42, fill_color=COLORS[key], fill_opacity=0.45, stroke_color=_ink(COLORS[key]), stroke_width=2)
                more.next_to(main, e.RIGHT, buff=0)
                bar.add(more)
                right = more.get_right()[0]
                label = f"{sizes[key]} + {extra[key]}"
            bars.append(e.VGroup(self._say(name(key), (-6.8, y), 22, anchor="l"), bar, self._say(label, (right + 0.3, y), 22, anchor="l")))
        unit_note = self._say(g("components in the definition (capsule definition lighter)", "Komponenten in der Definition (Kapseldefinition heller)"), (-3.2, -2.2), 20, "#444444")
        capsule_lit = self.tower_axon(at=(VX, VY), scale=0.1, lit=lambda key: True, types=False)
        self._fit(capsule_lit, viewport)
        self.step("why", e.FadeOut(podium_net), e.FadeOut(podium_view), e.FadeOut(p_dims), e.FadeIn(capsule_lit), e.LaggedStart(*[e.FadeIn(b) for b in bars], lag_ratio=0.3), e.FadeIn(unit_note), seconds=7.0)

        per_tower = _nak.totals()["per_tower"]
        sliders = e.VGroup(*[self.gh_slider(g("Capsule storeys ", "Kapselgeschosse ") + t.key, t.capsule_storeys, 1, 13, at=(-4.7, 1.0 - 1.2 * i), decimals=0, width=3.6) for i, t in enumerate((self.A, self.B))])
        generic = {t.key: t.capsule_storeys * len(_nak.facade_storeys(t)[0]) * items for t in (self.A, self.B)}
        formula_lines = [f"A  {self.A.capsule_storeys} × {len(_nak.facade_storeys(self.A)[0])} × {items} = {generic['A']}",
                         f"B  {self.B.capsule_storeys} × {len(_nak.facade_storeys(self.B)[0])} × {items} = {generic['B']}",
                         f"Cull Index  {MINUS}{generic['B'] - per_tower['B']['count']}", f"B  = {per_tower['B']['count']}"]
        a_panel = self.gh_panel(formula_lines, at=(-0.9, 0.3), width=3.3)
        a_group = self.gh_group(e.VGroup(sliders, a_panel), name("capsules"), color=COLORS["capsules"])
        build_a = self.tower_axon(at=(VX, VY), scale=0.1, towers=(0,), types=False)
        self._fit(build_a, viewport)
        build_b = self.tower_axon(at=(VX, VY), scale=0.1, types=False)
        self._fit(build_b, viewport)
        tag_a = self._say("A", build_b.cores[0].get_top() + np.array([0, 0.3, 0]), 24)
        tag_b = self._say("B", build_b.cores[1].get_top() + np.array([0, 0.3, 0]), 24)
        self.play(e.FadeOut(*bars), e.FadeOut(unit_note), e.FadeOut(capsule_lit), e.FadeIn(sliders), e.FadeIn(a_panel), e.FadeIn(a_group), e.FadeIn(build_a), run_time=1.5)
        self.step("towers", e.FadeOut(build_a), e.FadeIn(build_b), e.FadeIn(tag_a), e.FadeIn(tag_b), seconds=4.0)

        sum_panel = self.gh_panel([f"A  {per_tower['A']['count']}", f"B  {per_tower['B']['count']}", f"Σ  {total}"], at=(1.3, -1.5), width=1.6)
        lit_all = self.tower_axon(at=(VX, VY), scale=0.1, lit=lambda key: True, types=False)
        self._fit(lit_all, viewport)
        self.step("total", e.FadeIn(sum_panel), e.FadeOut(build_b), e.FadeOut(tag_a), e.FadeOut(tag_b), e.FadeIn(lit_all), seconds=5.0)

        old_key, new_key = "B", "D"
        swap_spec = lambda on: {"objects": [{"id": "sw", "component": "Boolean Toggle", "settings": {"value": on}},
                                            {"id": "cb", "component": "Panel", "settings": {"lines": [str(types[old_key].count)]}},
                                            {"id": "cd", "component": "Panel", "settings": {"lines": [str(types[new_key].count)]}},
                                            {"id": "mul", "component": "Multiplication", "settings": {}}, {"id": "add", "component": "Addition", "settings": {}},
                                            {"id": "sub", "component": "Subtraction", "settings": {}},
                                            {"id": "rb", "component": "Panel", "settings": {"lines": [""]}}, {"id": "rd", "component": "Panel", "settings": {"lines": [""]}}],
                                "wires": ["cb.Stream>mul.A", "sw.Value>mul.B", "cd.Stream>add.A", "mul.Result>add.B", "cb.Stream>sub.A", "mul.Result>sub.B", "sub.Result>rb.Stream", "add.Result>rd.Stream"]}
        swap_layout = {"sw": (-5.3, 1.5), "cb": (-5.5, 0.2), "cd": (-5.5, -1.1), "mul": (-3.3, 1.0), "sub": (-1.6, 1.6), "add": (-1.6, -0.5), "rb": (0.5, 1.6), "rd": (0.5, -0.5)}
        before_defn, before_sol = self._define(swap_spec(False), "swap")
        after_defn, after_sol = self._define(swap_spec(True), "swap")
        before_net = self._custom(before_defn, before_sol, swap_layout, "capsules")
        after_net = self._custom(after_defn, after_sol, swap_layout, "capsules")
        swap_legend = self._type_legend((-2.6, -2.15))
        recolor = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: _nakd.type_color(new_key) if kind[key] == old_key else None)
        self._fit(recolor, viewport)
        self.play(e.FadeOut(sum_panel), e.FadeOut(sliders), e.FadeOut(a_panel), e.FadeOut(a_group), e.FadeIn(before_net), e.FadeIn(swap_legend), run_time=1.5)
        self.step("swap", e.ReplacementTransform(before_net, after_net), e.FadeOut(lit_all), e.FadeIn(recolor), seconds=5.0)

        final = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: COLORS["capsules"])
        self._fit(final, viewport)
        outlines = self._marks(final)
        self.step("recap", e.FadeOut(after_net), e.FadeOut(swap_legend), e.FadeOut(recolor), e.FadeIn(final), e.FadeIn(part_cards), e.LaggedStart(*[e.FadeIn(m) for m in outlines], lag_ratio=0.3), seconds=5.0)
