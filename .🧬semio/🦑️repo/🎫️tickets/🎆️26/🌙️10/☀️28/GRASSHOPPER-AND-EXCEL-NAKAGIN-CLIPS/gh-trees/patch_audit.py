import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def sub(old, new, count=1):
    global s
    assert old in s, old[:70]
    s = s.replace(old, new) if count == 0 else s.replace(old, new, count)


sub('INK, MUTED, ARCH, DATA, GOOD, WARN = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "DATA", "GOOD", "WARN"))\nROLE_COLORS = (INK, ARCH, DATA, GOOD)',
    '''INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "DATA", "GOOD", "WARN"))
PINK, ITEM_C = "#ff9db0", "#9fb4ff"
TOWER_B, STOREY_LIGHT, PEACH = _draft.mix(STRUCT, INK, 0.55), _draft.mix(GOOD, INK, 0.55), "#ffcf9f"
ROLE_COLORS = (INK, STRUCT, GOOD, PINK)
LEVELS = ((INK, ("Building", "Gebäude")), (STRUCT, ("Tower", "Turm")), (GOOD, ("Storey", "Geschoss")), (PINK, ("Facade", "Fassade")), (ITEM_C, ("Index", "Index")), (INK, ("Branch", "Ast")))
FULL = (0, 1, 2, 3)''')
sub('SHEET_AT, SHEET_W, SHEET_H = (-3.65, 2.4, 0), 6.8, 2.9\nSRC_AT, COMP_AT, PANEL_AT = (-6.2, 2.4, 0), (-4.0, 2.4, 0), (-1.45, 2.4, 0)',
    'SHEET_AT, SHEET_W, SHEET_H = (-3.55, 2.4, 0), 6.5, 2.9\nSRC_AT, COMP_AT, PANEL_AT = (-5.95, 2.4, 0), (-3.9, 2.4, 0), (-1.45, 2.4, 0)\nHEAD_X = -6.8')
sub("FAN_LEFT, FAN_RIGHT, GROUND, CROWN = -5.45,", "FAN_LEFT, FAN_RIGHT, GROUND, CROWN = -5.35,")
s = s.replace("(-6.95, HEAD_Y)", "(HEAD_X, HEAD_Y)").replace("(6.95, 3.65)", "(6.8, 3.65)").replace("edge = _engine.VGroup(), 6.95", "edge = _engine.VGroup(), 6.8")

a = s.index("    def _diagram(")
b = s.index("    def _flat_rows(")
s = s[:a] + '''    def _diagram(self, tree, levels, *, lit=None, labels=True, leaf_color=None, big=False):
        """🌳 Draw a data tree as a real tree: trunk at the bottom (or several roots when the first level is not shared), a level per path segment, one leaf per item, all read from the engine tree.
        ``levels`` names the meaning of each path segment (indices into ``LEVELS``) for colour and label."""
        entries = [(path, index) for path in tree.paths() for index in range(len(tree.branch(path)))]
        depth = len(tree.paths()[0])
        count = len(entries)
        step = (FAN_RIGHT - FAN_LEFT) / count
        ys = [GROUND + (CROWN - GROUND) * level / (depth + 1) for level in range(depth + 2)]
        xs = {}
        for slot, (path, index) in enumerate(entries):
            x = FAN_LEFT + (slot + 0.5) * step
            for level in range(1, depth + 1):
                xs.setdefault(path[:level], []).append(x)
        centre = {prefix: sum(values) / len(values) for prefix, values in xs.items()}
        roots = [prefix for prefix in centre if len(prefix) == 1]
        base = np.array([centre[roots[0]] if len(roots) == 1 else 0.0, ys[0], 0.0])
        bright = lambda prefix: lit is None or any(prefix[:len(item)] == item[:len(prefix)] for item in lit)
        colors = [*[LEVELS[i][0] for i in levels], ITEM_C]
        buckets, dots = {}, _engine.VGroup()
        position = lambda prefix: base if not prefix else np.array([centre[prefix], ys[len(prefix)], 0.0])
        for prefix in sorted(centre, key=lambda p: (len(p), p)):
            level = len(prefix)
            if level > 1 or len(roots) == 1:
                key = (colors[level - 1], 1.0 if bright(prefix) else 0.16, 2.6 if level == 1 else 1.6 if level < 3 else 1.1)
                buckets.setdefault(key, []).append((position(prefix[:-1]), position(prefix)))
            if level <= 2 or count <= 40:
                dots.add(_engine.Dot(position(prefix), radius=0.075 if level == 1 else 0.05 if level == 2 else 0.035, color=colors[level - 1],
                                     fill_opacity=1.0 if bright(prefix) else 0.16))
        radius = min(0.07 if big else 0.045, step * 0.4)
        for slot, (path, index) in enumerate(entries):
            x = FAN_LEFT + (slot + 0.5) * step
            full = (*path, index)
            tone = leaf_color(path, index) if leaf_color else ITEM_C
            on = lit is None or any(full[:len(item)] == item[:len(full)] for item in lit)
            buckets.setdefault((tone, 1.0 if on else 0.16, 0.9), []).append((position(path), np.array([x, ys[depth + 1], 0.0])))
            dots.add(_engine.Dot(np.array([x, ys[depth + 1], 0.0]), radius=radius, color=tone, fill_opacity=1.0 if on else 0.16))
        group = _engine.VGroup(*[_draft.segments(pairs, color, width, opacity) for (color, opacity, width), pairs in buckets.items()], dots)
        if labels:
            names = [*[self._word(LEVELS[i][1]) for i in levels], self._word(ITEM_WORD)]
            for level in range(1, depth + 2):
                group.add(self._say(names[level - 1], (FAN_LEFT - 0.18, ys[level]), size=SMALL, color=colors[level - 1], edge="r"))
        group.ys = ys
        return group

    def _head(self, text):
        """📏 The count line above the tree diagram."""
        return self._say(text, (HEAD_X, HEAD_Y), edge="l")

    def _small(self, tree, prefix):
        """🔬 The branches of ``tree`` under a path prefix as a tree of their own, the smallest example of an operation."""
        return _model.Tree({path: tree.branch(path) for path in tree.paths() if path[:len(prefix)] == prefix})

''' + s[b:]

sub('''    def _pipe(self, op, tree, *, rows=None, count=3, lines=7, extra=()):''', '''    def _pipe(self, op, tree, *, rows=None, count=3, lines=7, mapping=None):''')
sub('''        comp = self.gh_component(op, at=COMP_AT)
        group = _engine.VGroup(comp, panel, self.gh_wire(self._src.grip_out(), comp.grip_in(0), kind="tree"), self.gh_wire(comp.grip_out(0), panel.grip_in(), kind="tree"))
        group.comp, group.panel = comp, panel''', '''        comp = self.gh_component(op, at=COMP_AT)
        group = _engine.VGroup(comp, panel, self.gh_wire(self._src.grip_out(), comp.grip_in(0), kind="tree"), self.gh_wire(comp.grip_out(0), panel.grip_in(), kind="tree"))
        if mapping:
            text = self.gh_panel(None, rows=[("line", "", mapping)], at=(-4.9, 3.5), max_lines=1, max_width=3.1)
            group.add(text, self.gh_wire(text.grip_out(), comp.grip_in(1), kind="item"))
        group.comp, group.panel = comp, panel''')
sub('letters = {"A": INK, "B": ARCH, "C": DATA, "D": GOOD, "i": WARN}', 'letters = {"A": INK, "B": STRUCT, "C": GOOD, "D": PINK, "i": ITEM_C}')
sub('dots.add(_engine.Dot(top, radius=0.045, color=WARN))', 'dots.add(_engine.Dot(top, radius=0.045, color=ITEM_C))') if False else None
s = s.replace("group.add(_engine.Dot(top, radius=0.045, color=WARN))", "group.add(_engine.Dot(top, radius=0.045, color=ITEM_C))")

a = s.index("        full_diag = self._diagram(self._tree, ROLE_COLORS)")
b = s.index('        self.step("recap"')
new = '''        full_diag = self._diagram(self._tree, FULL)
        head = self._head(self._count_text(self._tree))
        lit_all = self._tower(lit=[(0,)])
        self.step("tree", _engine.FadeOut(frame), _engine.FadeOut(question), self._swap("pipe", self._pipe(None, self._tree, count=2)), self._swap("tower", lit_all), self._swap("head", head),
                  _engine.FadeIn(full_diag, lag_ratio=0.01), seconds=2.4)
        self._slots["diag"] = full_diag

        mini = self._diagram(branch_tree, FULL, big=True)
        branch_pipe = self._pipe(None, branch_tree, rows=self._rows(branch_tree, 1))
        branch_tower = self._tower(lit=[(0, 0, 0, 0)])
        self.step("branch", self._swap("diag", mini), self._swap("pipe", branch_pipe), self._swap("tower", branch_tower),
                  self._swap("head", self._head(self.tr("1 branch · 2 items", "1 Ast · 2 Elemente"))), seconds=1.4)

        tower_key = towers[1].key
        segments = (0, 1, 4, 2)
        names = [self._word(LEVELS[0][1]), self.tr(f"Tower {tower_key}", f"Turm {tower_key}"), self.tr("Storey 4", "Geschoss 4"),
                 self.tr(f"Facade 2 ({FACADES[2][0]})", f"Fassade 2 ({FACADES[2][1]})")]

        def shown(stage):
            """🎯 Path text and the words of its first ``stage`` segments on a dark plate."""
            group = _engine.VGroup(self._plate((-2.75, 2.25), 4.6, 2.4))
            parts = [("{", MUTED)]
            for number, value in enumerate(segments):
                parts += [(str(value), ROLE_COLORS[number] if number < stage else MUTED), (";" if number < 3 else "}", MUTED)]
            group.add(self._tokens(parts, (-2.75, 3.05), size=40))
            for number in range(stage):
                y = 2.52 - 0.4 * number
                group.add(self._mono(str(segments[number]), (-4.7, y), size=LABEL, color=ROLE_COLORS[number], edge="l"),
                          self._say(names[number], (-4.25, y), size=LABEL, color=ROLE_COLORS[number], edge="l"))
            return group

        def scope(stage, zoom=1.0):
            """🔭 Capsules lit by the first ``stage`` path segments in their segment colours, magnified by ``zoom`` about the lit region."""
            prefix = segments[:stage]
            colour = lambda key: next((ROLE_COLORS[n] for n in range(stage - 1, -1, -1) if key[:n + 1] == segments[:n + 1]), None)
            if zoom == 1.0:
                return self._tower(lit=[prefix], tint=lambda key: colour(key))
            return self._detail([prefix], lambda key: colour(key), *zoom)

        def route(stage):
            """🧭 Tree diagram with the path prefix lit."""
            return self._diagram(self._tree, FULL, lit=[segments[:stage]])

        stages = [_engine.AnimationGroup(self._swap("diag", route(1)), self._swap("tower", scope(1)), self._swap("pipe", shown(1)))]
        for stage in (2, 3, 4):
            stages.append(_engine.AnimationGroup(self._swap("diag", route(stage)), self._swap("tower", scope(stage, {2: 1.0, 3: (0.2, 2), 4: (0.3, 1)}[stage])), self._swap("pipe", shown(stage))))
        self.step("path", self._swap("head", self._head(self.tr("path = address of a branch", "Pfad = Adresse eines Asts"))), _engine.Succession(*stages), seconds=7.0)

        leaf = self._diagram(self._tree, FULL, lit=[(0, 1, 4, 2, 1)], big=True)
        item_rows = [("path", "", "{0;1;4;2}"), ("item", "0", "Brep"), ("item", "1", "Brep")]
        item_pipe = _engine.VGroup(self.gh_panel(None, rows=item_rows, at=(-3.9, 2.4), max_lines=4, max_width=2.0), self._plate((-1.55, 2.4), 2.2, 0.8),
                                   self._tokens([("{0;1;4;2}", MUTED), ("[1]", ITEM_C)], (-1.55, 2.4), size=20))
        right_capsule = self._detail(lambda key: key == (0, 1, 4, 2, 1), lambda key: ITEM_C if key == (0, 1, 4, 2, 1) else None, 0.3, 1)
        self.step("item", self._swap("diag", leaf), self._swap("pipe", item_pipe), self._swap("tower", right_capsule),
                  self._swap("head", self._head(self.tr("index = position in the branch", "Index = Position im Ast"))), seconds=1.4)

        viewer_tree = _model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:4]})
        viewer = self.gh_param_viewer(viewer_tree, at=(-3.7, 2.45), max_items=2)
        panel_tree = self.gh_panel(None, rows=self._rows(self._tree, 3), at=(-1.3, 2.4), max_lines=8, max_width=1.7)
        viewer_pipe = _engine.VGroup(viewer, panel_tree, self.gh_wire(self._src.grip_out(), viewer.grip_in(), kind="tree"),
                                     self.gh_wire(self._src.grip_out(), panel_tree.grip_in(), kind="tree"),
                                     self._say(self.tr("Param Viewer: 70 branches", "Param Viewer: 70 Äste"), (-3.7, 1.15), size=SMALL, color=SHEET_INK))
        colour_towers = self._tower(tint=lambda key: STRUCT if key[1] == 0 else TOWER_B)
        self.step("panel", self._swap("pipe", viewer_pipe), self._swap("diag", self._diagram(self._tree, FULL)), self._swap("tower", colour_towers),
                  self._swap("head", self._head(self._count_text(self._tree))), seconds=1.4)

        stats = self.gh_component("Tree Statistics", at=(-3.9, 2.4))
        count_panel = self.gh_panel(None, rows=[("line", "", str(self._sol.tree("stc").branch((0,))[0]))], at=(-1.45, 3.1), max_lines=1, max_width=1.4)
        length_panel = self.gh_panel(None, rows=self._rows(self._sol.tree("stl"), 1)[1:5] + [("line", "", "...")], at=(-1.45, 1.75), max_lines=5, max_width=1.4)
        stats_pipe = _engine.VGroup(stats, count_panel, length_panel, self.gh_wire(self._src.grip_out(), stats.grip_in(0), kind="tree"),
                                    self.gh_wire(stats.grip_out("Count"), count_panel.grip_in(), kind="tree"), self.gh_wire(stats.grip_out("Length"), length_panel.grip_in(), kind="tree"))
        top = lambda key: key[1] == 1 and key[2] == 10
        below = lambda key: key[1] == 1 and key[2] == 9
        uneven = self._tower(lit=lambda key: top(key) or below(key), tint=lambda key: GOOD if top(key) else STOREY_LIGHT if below(key) else None)
        tags = self._chips((self.tr("tower B, storey 9: 4 facades", "Turm B, Geschoss 9: 4 Fassaden"), STOREY_LIGHT), (self.tr("tower B, storey 10: 3 facades", "Turm B, Geschoss 10: 3 Fassaden"), GOOD))
        self.step("stats", self._swap("pipe", stats_pipe), self._swap("tower", uneven), self._swap("tags", tags), seconds=1.4)

        def derived(op, tree, levels, mapping=None):
            """🧬 One restructuring result: component, Panel, diagram and count line all read from the evaluated tree."""
            return self._pipe(op, tree, mapping=mapping), self._diagram(tree, levels, big=len(tree.paths()) < 5), self._head(self._count_text(tree))

        def operate(key, op, small, big, levels, small_lit, seconds, tower_big=None, tags=None):
            """🔬 Introduce an operation on the smallest example first, then run it on the whole tower."""
            first, second = derived(op, small, levels[0]), derived(op, big, levels[1])
            stage = lambda parts, tower: _engine.AnimationGroup(self._swap("pipe", parts[0]), self._swap("diag", parts[1]), self._swap("head", parts[2]), self._swap("tower", tower))
            self.step(key, self._swap("tags", tags), _engine.Succession(stage(first, self._tower(lit=[small_lit])), stage(second, tower_big or self._tower())), seconds=seconds)

        one_storey = self._small(self._tree, (0, 0, 0))
        sweep = self._tower(lit=self._lit_all(total), tint=lambda key: INK)
        operate("flatten", "Flatten Tree", one_storey.flatten(), flat_tree, ((5,), (5,)), (0, 0, 0), 4.0, sweep)
        operate("graft", "Graft Tree", one_storey.graft(), graft_tree, ((0, 1, 2, 3, 4), (0, 1, 2, 3, 4)), (0, 0, 0), 4.0)
        operate("simplify", "Simplify Tree", one_storey.simplify(), simple_tree, ((3,), (1, 2, 3)), (0, 0, 0), 4.0)

        map_pipe = self._pipe("Path Mapper", storey_tree, mapping="{A;B;C;D}(i) -> {A;B;C}(i)")
        map_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B;C}(i)", (-3.9, 1.25)))
        storey_diag = self._diagram(storey_tree, (0, 1, 2), big=True)
        self.step("mapper", self._swap("pipe", map_pipe), self._swap("diag", storey_diag), self._swap("head", self._head(self._count_text(storey_tree))), self._swap("tower", self._tower()), seconds=1.6)

        stages = []
        sizes = {(0, 0, 0): per_storey[0], (0, 1, 4): per_storey[9 + 4], (0, 1, 10): per_storey[-1]}
        names_s = {(0, 0, 0): ("tower A, storey 0", "Turm A, Geschoss 0"), (0, 1, 4): ("tower B, storey 4", "Turm B, Geschoss 4"), (0, 1, 10): ("tower B, top storey", "Turm B, oberstes Geschoss")}
        for prefixes in (((0, 0, 0),), ((0, 1, 10),), ((0, 1, 4), (0, 1, 10))):
            chips = self._chips(*[(self.tr(f"{names_s[p][0]}: {width(sizes[p])} capsules", f"{names_s[p][1]}: {width(sizes[p])} Kapseln"), GOOD) for p in prefixes])
            lit = self._tower(lit=list(prefixes), tint=lambda key, ps=prefixes: GOOD if any(key[:3] == p for p in ps) else None)
            stages.append(_engine.AnimationGroup(self._swap("tower", lit), self._swap("tags", chips)))
        self.step("perstorey", _engine.Succession(*stages), seconds=4.0)

        tower_pipe = self._pipe("Path Mapper", tower_tree, count=2, mapping="{A;B;C;D}(i) -> {A;B}(i)")
        tower_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B}(i)", (-3.9, 1.25)))
        tower_diag = self._diagram(tower_tree, (0, 1), big=True)
        tone = lambda i: STRUCT if i == 0 else TOWER_B
        stages = []
        for group in ((0,), (1,), (0, 1)):
            chips = self._chips(*[(self.tr(f"tower {towers[i].key}: {width(per_tower[i])} capsules", f"Turm {towers[i].key}: {width(per_tower[i])} Kapseln"), tone(i)) for i in group])
            drawing = self._tower(lit=[(0, i) for i in group], tint=lambda key, g=group: tone(key[1]) if key[1] in g else None)
            stages.append(_engine.AnimationGroup(self._swap("tower", drawing), self._swap("tags", chips)))
        self.step("pertower", self._swap("pipe", tower_pipe), self._swap("diag", tower_diag), self._swap("head", self._head(self._count_text(tower_tree))), _engine.Succession(*stages), seconds=3.2)

        flip_pipe, flip_diag, flip_head = derived("Flip Matrix", flip_tree, (5,))
        item_tone = (ITEM_C, PEACH)
        halves = [self._tower(lit=lambda key, i=i: key[4] == i, tint=lambda key, i=i: item_tone[i] if key[4] == i else None) for i in (0, 1)]
        item_chip = lambda i: self._chips((self.tr(("left capsules: 70", "right capsules: 70")[i], ("linke Kapseln: 70", "rechte Kapseln: 70")[i]), item_tone[i]))
        self.step("flip", self._swap("pipe", flip_pipe), self._swap("diag", flip_diag), self._swap("head", flip_head), self._swap("tags", None),
                  _engine.Succession(*[_engine.AnimationGroup(self._swap("tower", halves[i]), self._swap("tags", item_chip(i))) for i in (0, 1)]), seconds=3.2)

        words = (self.tr("teal", "türkis"), self.tr("orange", "orange"))
        preview = self.gh_component("Custom Preview", at=(-2.9, 2.4))
        colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-5.3, 1.42), max_lines=2, max_width=1.4)
        match_pipe = _engine.VGroup(preview, colors, self.gh_wire(self._src.grip_out(), preview.grip_in(0), kind="tree"), self.gh_wire(colors.grip_out(), preview.grip_in(1), kind="list"))
        intent = self._chips((self.tr("tower A: teal", "Turm A: türkis"), ARCH), (self.tr("tower B: orange", "Turm B: orange"), WARN))
        self.step("match", self._swap("pipe", match_pipe), self._swap("diag", self._diagram(self._tree, FULL)), self._swap("tower", self._tower()),
                  self._swap("head", self._head(self._count_text(self._tree))), self._swap("tags", intent), seconds=1.6)

        stripes = self._tower(lit=None, tint=lambda key: MATERIALS[paint["cp1"][key]])
        warn_note = self._chips((self.tr("no error: 70 teal, 70 orange", "kein Fehler: 70 türkis, 70 orange"), INK))
        mistake_diag = self._diagram(self._tree, FULL, leaf_color=lambda path, index: MATERIALS[paint["cp1"][(*path, index)]])
        self.step("mistake", self._swap("tower", stripes), self._swap("tags", warn_note), self._swap("diag", mistake_diag), seconds=1.6)

        first_path = path_text_of(self._tree.paths()[0])
        viewer_a = self.gh_param_viewer(_model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:3]}), at=(-3.85, 2.4), max_items=2)
        viewer_b = self.gh_param_viewer(_model.Tree({(0,): [None, None]}), at=(-1.4, 2.4), max_items=2)
        depth_chips = _engine.VGroup(self._say(self.tr("70 branches, depth 4", "70 Äste, Tiefe 4"), (-3.85, 1.2), size=SMALL, color=SHEET_INK),
                                     self._say(self.tr("1 branch, depth 1", "1 Ast, Tiefe 1"), (-1.4, 1.2), size=SMALL, color=SHEET_INK))
        why_pipe = _engine.VGroup(viewer_a, viewer_b, depth_chips)
        self.step("why", self._swap("pipe", why_pipe), self._swap("diag", _engine.VGroup(self._diagram(self._tree, FULL, lit=[(0, 0, 0, 0)]))), self._swap("tags", None), seconds=1.6)

        fix_map = self.gh_component("Path Mapper", at=(-3.0, 3.0), scale=0.8)
        fix_text = self.gh_panel(None, rows=[("line", "", "{A;B;C;D}(i) -> {A;B}(i)")], at=(-5.2, 3.55), max_lines=1, max_width=2.9)
        fix_colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-5.3, 1.42), max_lines=2, max_width=1.4)
        fix_graft = self.gh_component("Graft Tree", at=(-3.3, 1.45), scale=0.8)
        fix_preview = self.gh_component("Custom Preview", at=(-1.2, 2.3), scale=0.8)
        fix_pipe = _engine.VGroup(fix_map, fix_text, fix_colors, fix_graft, fix_preview,
                                  self.gh_wire(self._src.grip_out(), fix_map.grip_in(0), kind="tree"), self.gh_wire(fix_text.grip_out(), fix_map.grip_in(1), kind="item"),
                                  self.gh_wire(fix_colors.grip_out(), fix_graft.grip_in(0), kind="list"), self.gh_wire(fix_map.grip_out(0), fix_preview.grip_in(0), kind="tree"),
                                  self.gh_wire(fix_graft.grip_out(0), fix_preview.grip_in(1), kind="tree"))
        fix_diag = self._diagram(tower_tree, (0, 1), big=True, leaf_color=lambda path, index: MATERIALS[paint["cp2"][(*path, index)]] if (*path, index) in paint["cp2"] else ITEM_C)
        self.step("fix", self._swap("pipe", fix_pipe), self._swap("diag", fix_diag), self._swap("tower", self._tower()), self._swap("tags", None),
                  self._swap("head", self._head(self.tr("{0;0} meets {0;0} · {0;1} meets {0;1}", "{0;0} trifft {0;0} · {0;1} trifft {0;1}"))), seconds=2.4)

        counts = {name: sum(1 for value in paint["cp2"].values() if value == name) for name in ("teal", "orange")}
        done = self._tower(tint=lambda key: MATERIALS[paint["cp2"][key]])
        result_tags = self._chips((self.tr(f"tower A: {width(counts['teal'])} teal", f"Turm A: {width(counts['teal'])} türkis"), ARCH),
                                  (self.tr(f"tower B: {width(counts['orange'])} orange", f"Turm B: {width(counts['orange'])} orange"), WARN))
        self.step("result", self._swap("tower", done), self._swap("tags", result_tags), seconds=1.6)

        check = self.badge(self.tr("read both paths first", "erst beide Pfade lesen"), (HEAD_X, HEAD_Y), color=GOOD, size=LABEL, filled=False, anchor="l")
        self.step("rule", self._swap("head", check), seconds=1.2)

'''
s = s[:a] + new + s[b:]
s = s.replace("        first_path = path_text_of(self._tree.paths()[0])\n", "")
s = s.replace('self._mono(result, (x0 + 3.9, y), size=SMALL, color=DATA, edge="l")', 'self._mono(result, (x0 + 3.9, y), size=SMALL, color=DATA, edge="l")')

# narration and German grammar
sub("but which sits where?", "but which one sits where?")
sub("Eine Liste enthält alle, aber welche sitzt wo?", "Eine Liste enthält alle, aber welche Kapsel sitzt wo?")
sub("listet jeden Ast mit Pfad.", "listet jeden Ast mit seinem Pfad.")
sub("Die Regel {A;B;C;D}(i) nach {A;B;C} benennt", "Die Regel {A;B;C;D}(i) wird zu {A;B;C}: Sie benennt")
sub("Der Param Viewer zeigt warum:", "Der Param Viewer zeigt, warum:")
sub("Links wird türkis, rechts orange, in beiden Türmen.", "Die linken Kapseln werden türkis, die rechten orange, in beiden Türmen.")
sub("verschiedene Strukturen überraschen still.", "verschiedene Strukturen führen zu stillen Fehlern.")
sub("Zusammenfassung: Baum, Ast,", "Zusammenfassung: Datenbaum, Ast,")
open(p, "w", encoding="utf-8").write(s)
