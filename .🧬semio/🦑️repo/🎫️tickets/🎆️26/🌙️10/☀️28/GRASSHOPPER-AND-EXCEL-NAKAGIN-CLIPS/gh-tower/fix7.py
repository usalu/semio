import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)


def clause(key, en, de):
    global s
    i = s.index(f'        ["{key}", ')
    j = s.index("],\n", i) + 3
    s = s[:i] + f'        ["{key}", "{en}",\n         "{de}"],\n' + s[j:]


clause("hook", "The Nakagin Capsule Tower has 140 capsules. How do they become two towers?", "Der Nakagin Capsule Tower hat 140 Kapseln. Wie werden sie zu zwei Türmen?")
clause("items", "Two capsules sit side by side on each facade: item 0 and item 1, one step to each side along X.",
       "Auf jeder Fassade sitzen zwei Kapseln nebeneinander: Item 0 und Item 1, je einen Schritt beidseits entlang X.")
clause("mistake", "A typical mistake: without Cull Index, tower B gets 88 capsules, two of them west on the top storey, where the building has none.",
       "Typischer Fehler: Ohne Cull Index hat Turm B 88 Kapseln, zwei davon im obersten Geschoss im Westen, wo es keine gibt.")
clause("podium", "The lower building is only an extruded massing: one footprint, one height.", "Nur ein extrudierter Baukörper: Grundriss und Höhe.")
clause("why", "That is on purpose. The podium is context, not the subject: one extrusion is enough, while one capsule parameter changes every copy.",
       "Das ist Absicht. Der Sockel ist Kontext, nicht das Thema: eine Extrusion genügt, ein Kapselparameter dagegen ändert jede Kopie.")
clause("swap", "Metabolism: capsules are replaceable. Swap type B for type C, and every capsule of that type gets the larger window at once.",
       "Metabolismus: Kapseln sind austauschbar. Ersetzt man Typ B durch Typ C, bekommt jede Kapsel dieses Typs sofort das größere Fenster.")

# core: grow from a smaller start, equation on the dimension
rep("        n_t = e.ValueTracker(self.B.capsule_storeys)\n", "        n_t = e.ValueTracker(high // 2)\n")
rep("e.FadeIn(dims), n_t.animate.set_value(high // 2), seconds=6.0)\n        self.play(n_t.animate.set_value(self.B.capsule_storeys), run_time=2.5)\n",
    "e.FadeIn(dims), n_t.animate.set_value(self.B.capsule_storeys), seconds=7.0)\n")
rep('''        rise = self.dimension(self._pt(td, cx + s, cy + s, 0.0), self._pt(td, cx + s, cy + s, height), self.quantity(height, "m"), offset=0.6, color=WARN)''',
    '''        count = round(height / self.B.storey_height)
        rise = self.dimension(self._pt(td, cx + s, cy + s, 0.0), self._pt(td, cx + s, cy + s, height), f"{count} × {self.number(self.B.storey_height)} = {self.number(height)} m", offset=0.6, color=WARN)''')

# items: dimensions along X
rep('''        sx_panel = self.gh_panel(self._lines(full_solution.tree("sx")), at=(1.4, -0.8), width=1.4)''',
    '''        sx_panel = self.gh_panel(self._lines(full_solution.tree("sx")), at=(1.4, -0.8), width=1.4)
        centre = sum(np.array(first[i].origin.t) for i in range(items)) / items
        step_dims = e.VGroup(*[self.dimension(self._pt(closeup, *centre), self._pt(closeup, *self._plane(first[i]).origin),
                                              self._fmt(round(float(first[i].origin.t[0] - centre[0]), 3)) + " m", offset=-0.5 if i == 0 else 0.5, color=DATA) for i in range(items)])''')
rep("e.FadeIn(both), e.FadeIn(tags), seconds=6.0)", "e.FadeIn(both), e.FadeIn(tags), e.FadeIn(step_dims), seconds=6.0)")
rep("e.FadeOut(items_wire), e.FadeOut(tags), e.FadeIn(orient_net)", "e.FadeOut(items_wire), e.FadeOut(tags), e.FadeOut(step_dims), e.FadeIn(orient_net)")

# types: numbered planes
rep('''        legend = self._type_legend((-2.6, -2.15))
        types_net =''', '''        legend = self._type_legend((-2.6, -2.15))
        numbers = e.VGroup(*[e.VGroup(e.Circle(radius=0.14, fill_color="#FFFFFF", fill_opacity=0.95, stroke_color=TXT, stroke_width=2), self._say(str(2 * sl.facade + sl.item), (0, 0), 16)).move_to(
            self._pt(types_view, *sl.plane.origin) + np.array([0.0, 0.0, 0.0])) for sl in self._slots if sl.tower == 1 and sl.storey == 0])
        types_net =''')
rep("e.FadeIn(legend), e.FadeIn(types_view), seconds=6.0)", "e.FadeIn(legend), e.FadeIn(types_view), e.FadeIn(numbers), seconds=6.0)")
rep("e.FadeOut(types_net), e.FadeOut(legend), e.FadeOut(types_view), e.FadeIn(net_live)", "e.FadeOut(types_net), e.FadeOut(legend), e.FadeOut(types_view), e.FadeOut(numbers), e.FadeIn(net_live)")

# nowire: sweep inside the step
rep('''        self.step("nowire", e.Indicate(e.VGroup(*net_live.wires), color=GOOD, scale_factor=1.0), seconds=2.5)
        self.play(k.animate.set_value(low + 1), run_time=2.5)
        self.play(k.animate.set_value(high), run_time=2.5)''', '''        self.step("nowire", e.Indicate(e.VGroup(*net_live.wires), color=GOOD, scale_factor=1.0), k.animate.set_value(high // 2), seconds=4.0)
        self.play(k.animate.set_value(high), run_time=2.5)''')
rep("k.animate.set_value(high), seconds=8.0)", "k.animate.set_value(high), seconds=9.0)")
rep("        self.step(\"count\", e.FadeOut(types_net)", "        k.set_value(low)\n        self.step(\"count\", e.FadeOut(types_net)")

# mistake: ghost capsules
rep('''        tower_full = self.live(lambda: self._tower(k.get_value()), key=shown)''', '''        tower_full = self.live(lambda: self._tower(k.get_value()), key=shown)
        ghosts = e.VGroup(*[_nakd._capsule_lite(_nakd._cap(types["A"]), SimpleNamespace(origin=np.array(last[i].origin.t), x=np.array(last[i].x.t), y=np.array(last[i].y.t), z=np.array(last[i].z.t)),
                                                tower_full.view, "#D8362B").shift(tower_full.delta).set_opacity(0.55) for i in removed])''')
rep("e.FadeIn(cull_wrong), e.FadeIn(sum_wrong), e.FadeIn(crosses),", "e.FadeIn(cull_wrong), e.FadeIn(sum_wrong), e.FadeIn(ghosts), e.FadeIn(crosses),")
rep("e.ReplacementTransform(sum_wrong, sum_right), e.FadeOut(crosses), seconds=5.0)", "e.ReplacementTransform(sum_wrong, sum_right), e.FadeOut(crosses), e.FadeOut(ghosts), seconds=5.0)")
s = s.replace("(-2.3, -2.25), 30)", "(-2.3, -2.4), 30)")

# capital: core under the capital
rep('''        cap_live = self.live(build_capital, key=round2)''', '''        cap_live = self.live(build_capital, key=round2)
        stub = self.core_axon(self.B, at=(VX, VY), scale=0.4, storeys=2)
        self._pin(stub, (self.B.center[0], self.B.center[1], 2 * self.B.storey_height), (VX, VY - 0.7, 0.0))''')
rep("e.FadeIn(capital_net), e.FadeIn(cap_live), e.FadeIn(cap_dim), seconds=4.0)", "e.FadeIn(capital_net), e.FadeIn(stub), e.FadeIn(cap_live), e.FadeIn(cap_dim), seconds=4.0)")
rep("e.FadeOut(cap_live), e.FadeOut(cap_dim), e.FadeIn(top_net)", "e.FadeOut(cap_live), e.FadeOut(cap_dim), e.FadeOut(stub), e.FadeIn(top_net)")

# why: drawn argument instead of bars
i = s.index("        named = {oid for key in MEMBERS")
j = s.index("        per_tower = _nak.totals()")
why = '''        capsule_comps = len(_gh.canvas("capsule").objects)
        cards = []
        for key, head, body_lines in (("podium", g("Podium: context", "Sockel: Kontext"), [g(f"{len(MEMBERS['podium'])} components", f"{len(MEMBERS['podium'])} Komponenten"), g("one extrusion", "eine Extrusion")]),
                                      ("capsules", g("Capsule: the subject", "Kapsel: das Thema"), [g(f"{capsule_comps} components", f"{capsule_comps} Komponenten"), g(f"{total} copies", f"{total} Kopien")])):
            box = e.RoundedRectangle(corner_radius=0.12, width=3.6, height=1.9, fill_color=COLORS[key], fill_opacity=0.55, stroke_color=_ink(COLORS[key]), stroke_width=3)
            lines = e.VGroup(self._say(head, (0, 0), 26), *[self._say(t, (0, 0), 22) for t in body_lines]).arrange(e.DOWN, buff=0.15).move_to(box)
            cards.append(e.VGroup(box, lines))
        e.VGroup(*cards).arrange(e.DOWN, buff=0.5).move_to(np.array([-3.4, 0.2, 0.0]))
        arrows = e.VGroup(*[e.Arrow(c.get_right(), c.get_right() + np.array([0.9, 0, 0]), color=TXT, buff=0.05) for c in cards])
        capsule_lit = self.tower_axon(at=(VX, VY), scale=0.1, lit=lambda key: True, types=False)
        self._fit(capsule_lit, viewport)
        podium_outline = self._mark(capsule_lit.podium, "podium", 0.02)
        self.step("why", e.FadeOut(podium_net), e.FadeOut(podium_view), e.FadeOut(p_dims), e.FadeIn(capsule_lit), e.FadeIn(podium_outline), e.LaggedStart(*[e.FadeIn(c) for c in cards], lag_ratio=0.4), e.FadeIn(arrows), seconds=7.0)

'''
s = s[:i] + why + s[j:]
rep("e.FadeOut(*bars), e.FadeOut(unit_note), e.FadeOut(capsule_lit), e.FadeIn(sliders)", "e.FadeOut(*cards), e.FadeOut(arrows), e.FadeOut(podium_outline), e.FadeOut(capsule_lit), e.FadeIn(sliders)")

# towers / total: tinted in turn
tint_a, tint_b = '"#3FA7D6"', '"#E59A3C"'
rep('''        build_a = self.tower_axon(at=(VX, VY), scale=0.1, towers=(0,), types=False)
        self._fit(build_a, viewport)
        build_b = self.tower_axon(at=(VX, VY), scale=0.1, types=False)
        self._fit(build_b, viewport)''', f'''        tint_a, tint_b = {tint_a}, {tint_b}
        build_a = self.tower_axon(at=(VX, VY), scale=0.1, tint={{(0, 0): tint_a}}, types=False)
        self._fit(build_a, viewport)
        build_b = self.tower_axon(at=(VX, VY), scale=0.1, tint={{(0, 0): tint_a, (0, 1): tint_b}}, types=False)
        self._fit(build_b, viewport)''')
rep('''tag_a = self._say("A", build_b.cores[0].get_top() + np.array([0, 0.3, 0]), 24)
        tag_b = self._say("B", build_b.cores[1].get_top() + np.array([0, 0.3, 0]), 24)''', '''facade_count = lambda t: len(_nak.facade_storeys(t)[0])
        tag_a = self._say("A  " + g(f"{self.A.capsule_storeys} storeys, {facade_count(self.A)} facades", f"{self.A.capsule_storeys} Geschosse, {facade_count(self.A)} Fassaden"), build_b.cores[0].get_top() + np.array([-0.2, 0.35, 0]), 18, tint_a)
        tag_b = self._say("B  " + g(f"{self.B.capsule_storeys} storeys, {facade_count(self.B)} facades", f"{self.B.capsule_storeys} Geschosse, {facade_count(self.B)} Fassaden"), build_b.cores[1].get_top() + np.array([0.3, 0.35, 0]), 18, tint_b)''')
rep("self.step(\"towers\", e.FadeOut(build_a), e.FadeIn(build_b), e.FadeIn(tag_a), e.FadeIn(tag_b), seconds=4.0)", "self.step(\"towers\", e.FadeIn(tag_a), e.Indicate(build_a, color=tint_a), seconds=3.0)\n        self.play(e.FadeOut(build_a), e.FadeIn(build_b), e.FadeIn(tag_b), run_time=1.5)")
rep("at=(1.3, -1.5), width=1.6)", "at=(-1.0, -1.7), width=1.6)")
rep("lit_all = self.tower_axon(at=(VX, VY), scale=0.1, lit=lambda key: True, types=False)", "lit_all = self.tower_axon(at=(VX, VY), scale=0.1, tint={(0, 0): tint_a, (0, 1): tint_b}, lit=lambda key: True, types=False)")

# swap B -> C with real window change
rep('old_key, new_key = "B", "D"', 'old_key, new_key = "B", "C"')
rep('name="B → D")', 'name=f"{old_key} → {new_key}")')
rep('''        swap_legend = self._type_legend((-2.6, -2.15))
        recolor = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: _nakd.type_color(new_key) if kind[key] == old_key else None)
        self._fit(recolor, viewport)''', '''        swap_legend = self._type_legend((-2.6, -2.15))
        recolor = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: _nakd.type_color(new_key) if kind[key] == old_key else None)
        recolor.scale_to_fit_width(VW - 1.2).move_to(np.array([VX, VY - 0.9, 0.0]))
        pair_old = self.capsule_axon(types[old_key], at=(VX - 1.0, VY + 1.65), scale=0.3, dims=("window",))
        pair_new = self.capsule_axon(types[new_key], at=(VX + 1.0, VY + 1.65), scale=0.3, dims=("window",), highlight={"front": _nakd.type_color(new_key)})
        pair_arrow = e.Arrow(np.array([VX - 0.25, VY + 1.65, 0]), np.array([VX + 0.25, VY + 1.65, 0]), color=TXT, buff=0)
        pair = e.VGroup(pair_old, pair_new, pair_arrow)''')
rep("e.FadeOut(lit_all), e.FadeIn(recolor), seconds=5.0)", "e.FadeOut(lit_all), e.FadeIn(recolor), e.FadeIn(pair), seconds=5.0)")
rep("e.FadeOut(swap_legend), e.FadeOut(recolor), e.FadeIn(final)", "e.FadeOut(swap_legend), e.FadeOut(recolor), e.FadeOut(pair), e.FadeIn(final)")
open(p, "w", encoding="utf-8").write(s)
