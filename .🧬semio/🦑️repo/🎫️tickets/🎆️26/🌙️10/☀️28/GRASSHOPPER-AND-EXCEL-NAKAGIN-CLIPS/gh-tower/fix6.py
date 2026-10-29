import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)


def clause(key, en, de):
    global s
    i = s.index(f'        ["{key}", ')
    j = s.index("],\n", i) + 3
    s = s[:i] + f'        ["{key}", "{en}",\n         "{de}"],\n' + s[j:]


clause("count", "Now the capsule storeys slider: the count is storeys times facades times items, and the drawing grows with it.",
       "Nun der Slider Kapselgeschosse: Die Anzahl ist Geschosse mal Fassaden mal Items, und die Zeichnung wächst mit.")
clause("mistake", "A typical mistake: without Cull Index, tower B gets 88 capsules, including two west capsules on the top storey that the building does not have.",
       "Typischer Fehler: Ohne Cull Index hat Turm B 88 Kapseln, darunter zwei westliche im obersten Geschoss, die es nicht gibt.")
clause("fix", "Cull Index removes exactly those two planes, indices 6 and 7. Now the definition gives 86, the number the building really has.",
       "Cull Index entfernt genau diese zwei Ebenen, die Indizes 6 und 7. Nun liefert die Definition 86, die Zahl des Gebäudes.")
rep("    def _custom(self, defn, sol, layout, group, extra=()):", "    def _custom(self, defn, sol, layout, group, extra=(), widths=None):")
rep("self._widget(oid, at, sol, {}, None) for oid, at in layout.items()}", "self._widget(oid, at, sol, {}, (widths or {}).get(oid)) for oid, at in layout.items()}")
rep('"planes": len(self._defn.objects) - len(named)}', '"planes": len(plane_ids)}')
rep("        named = {oid for key in MEMBERS for oid in MEMBERS[key]}\n", '''        named = {oid for key in MEMBERS for oid in MEMBERS[key]}
        plane_ids = {oid for view in ("levels", "facades", "stagger", "rotate", "centre", "storeys", "yaw", "clearance") for oid in self._defn.views[view]["objects"]} - named
''')
i = s.index("        old_key, new_key = ")
j = s.index("        swap_legend = ")
new = '''        old_key, new_key = "B", "D"
        flag = lambda key: [str(sl.type == key) for sl in self._slots]
        swap_spec = lambda on: {"objects": [{"id": "all", "component": "Panel", "settings": {"lines": [sl.type for sl in self._slots]}},
                                            {"id": "pb", "component": "Panel", "settings": {"lines": flag(old_key)}}, {"id": "pd", "component": "Panel", "settings": {"lines": flag(new_key)}},
                                            {"id": "db", "component": "Dispatch", "settings": {}}, {"id": "dd", "component": "Dispatch", "settings": {}},
                                            {"id": "cb", "component": "List Length", "settings": {}}, {"id": "cd", "component": "List Length", "settings": {}},
                                            {"id": "sw", "component": "Boolean Toggle", "settings": {"value": on}},
                                            {"id": "mul", "component": "Multiplication", "settings": {}}, {"id": "add", "component": "Addition", "settings": {}},
                                            {"id": "sub", "component": "Subtraction", "settings": {}},
                                            {"id": "rb", "component": "Panel", "settings": {"lines": [""]}}, {"id": "rd", "component": "Panel", "settings": {"lines": [""]}}],
                                "wires": ["all.Stream>db.List", "pb.Stream>db.Pattern", "all.Stream>dd.List", "pd.Stream>dd.Pattern", "db.List A>cb.List", "dd.List A>cd.List",
                                          "cb.Length>mul.A", "sw.Boolean>mul.B", "cd.Length>add.A", "mul.Result>add.B", "cb.Length>sub.A", "mul.Result>sub.B", "sub.Result>rb.Stream", "add.Result>rd.Stream"]}
        swap_layout = {"sw": (-5.3, 1.7), "all": (-5.9, -0.5), "cb": (-3.9, 0.5), "cd": (-3.9, -1.4), "mul": (-2.0, 1.3), "sub": (-0.2, 1.7), "add": (-0.2, -0.5), "rb": (1.6, 1.7), "rd": (1.6, -0.5)}
        before_defn, before_sol = self._define(swap_spec(False), "swap")
        after_defn, after_sol = self._define(swap_spec(True), "swap")

        def swap_net(defn, sol):
            net = self._custom(defn, sol, swap_layout, "capsules", widths={"all": 0.8})
            net.add(*[self.gh_wire(net.made["all"].grip_out(), net.made[oid].grip_in(), kind="list") for oid in ("cb", "cd")])
            return net

        before_net, after_net = swap_net(before_defn, before_sol), swap_net(after_defn, after_sol)
'''
s = s[:i] + new + s[j:]
# cull values
rep('''        cull_wrong = self._net(cull_layout, over=empty, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),))
        cull_right = self._net({**cull_layout, "cullp": (-5.4, -1.0)}, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),))''',
    '''        cull_tree = full_solution.tree("cullp")
        cull_lines = lambda: {path: [str(v) for v in cull_tree.branch(path)] for path in cull_tree.paths() if cull_tree.branch(path)}
        panel_wrong, panel_right = self.gh_panel(cull_lines(), at=(-5.4, -1.1), width=1.5), self.gh_panel(cull_lines(), at=(-5.4, -1.1), width=1.5)
        cull_wrong = self._net(cull_layout, over=empty, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),), extra=(panel_wrong,))
        cull_wrong.add(panel_wrong)
        cull_right = self._net(cull_layout, widths={"stl": 1.0}, group="capsules", bridges=(("cull", "List", "st", "Tree"),), extra=(panel_right,))
        cull_right.add(panel_right, self.gh_wire(panel_right.grip_out(), cull_right.made["cull"].grip_in("Indices"), kind="tree"))''')
# recap legend
rep('''        self.step("recap", e.FadeOut(after_net),''', '''        pairs = [("Plane", "Ebene"), ("Facade", "Fassade"), ("Storey", "Geschoss"), ("Group", "Gruppe"), ("Capital", "Kopf"), ("Podium", "Sockel")]
        words = [f"{a} = {b}" if lang == "en" else f"{b} = {a}" for a, b in pairs]
        terms = e.VGroup(self._say("  ·  ".join(words[:3]), (-2.35, -2.0), 19), self._say("  ·  ".join(words[3:]), (-2.35, -2.3), 19))
        recap_cards = part_cards.copy().scale(0.82).shift(np.array([0.0, 0.3, 0.0]))
        self.step("recap", e.FadeOut(after_net),''')
rep("e.FadeIn(final), e.FadeIn(part_cards),", "e.FadeIn(final), e.FadeIn(recap_cards), e.FadeIn(terms),")
open(p, "w", encoding="utf-8").write(s)
