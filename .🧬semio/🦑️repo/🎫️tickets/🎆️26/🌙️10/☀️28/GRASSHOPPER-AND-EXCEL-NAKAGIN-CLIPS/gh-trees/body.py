    def _rule_tokens(self, text, point, size=SMALL):
        """🎨 A Path Mapper expression with the letters in the colours of their levels."""
        letters = {"A": INK, "B": TOWER_GREY, "C": GOOD, "D": PINK, "i": ITEM_C}
        return self._tokens([(c, letters.get(c, MUTED)) for c in text], point, size=size)

    def _path_parts(self, path, index, colors):
        """🎨 Coloured tokens of ``{p;q;…}[i]``."""
        parts = [("{", MUTED)]
        for number, value in enumerate(path):
            parts += [(str(value), colors[number]), (";" if number < len(path) - 1 else "}", MUTED)]
        return [*parts, (f"[{index}]", ITEM_C)]

    def _has(self, tree):
        """❓ Whether capsule 57 is part of an engine tree."""
        try:
            self._locate(tree)
        except KeyError:
            return False
        return True

    def _swatches(self, panel, colors):
        """🟦 Real colour swatches at the right edge of the rows of a colour Panel."""
        squares = _engine.VGroup()
        for row, color in zip(panel.parts["rows"], colors):
            squares.add(_engine.Square(side_length=0.17, fill_color=color, fill_opacity=1, stroke_color=SHEET_INK, stroke_width=1.2).move_to((panel.body.get_right()[0] - 0.24, row.get_center()[1], 0)))
        return squares

    def present_trees(self):
        """🎞️ The whole data-tree animation on the aggregated tower, following capsule 57."""
        self._def = _model.canvas("trees")
        self._sol = _model.evaluate(self._def)
        self._tree = self._sol.tree(SOURCE)
        self._slots = {}
        slots = _nakagin.slots()
        self._order = {(*slot.path, slot.item): number for number, slot in enumerate(slots)}
        probe = slots[57]
        self._probe = (*probe.path, probe.item)
        towers = {tower.index: tower for tower in _nakagin.towers()}
        per_storey = [len(self._sol.tree("pm", "Tree").branch(path)) for path in self._sol.tree("pm", "Tree").paths()]
        per_tower = [len(self._sol.tree("pt", "Tree").branch(path)) for path in self._sol.tree("pt", "Tree").paths()]
        kind = lambda tree: {(*b.tag["slot"],): b.tag["material"] for path in tree.paths() for b in tree.branch(path)}
        paint = {name: kind(self._sol.tree(name)) for name in ("cp1", "cp2")}
        total = self._tree.stats()["item_count"]
        width = lambda value: self.number(value, 0)
        sheet = self.gh_background(at=SHEET_AT, width=SHEET_W, height=SHEET_H)
        self._src = self.gh_param("Brep", at=SRC_AT, label=self.tr("Capsules", "Kapseln"))
        self._dock = _engine.VGroup(sheet, self._src)
        flat_tree = self._sol.tree("fl", "Tree")
        flip_tree = self._sol.tree("fp", "Data")
        graft_tree = self._sol.tree("gr", "Tree")
        simple_tree = self._sol.tree("sm", "Tree")
        storey_tree = self._sol.tree("pm", "Tree")
        tower_tree = self._sol.tree("pt", "Tree")
        branch_tree = _model.Tree({(0, 0, 0, 0): self._tree.branch((0, 0, 0, 0))})
        self._results = {"src": self._tree, "fl": flat_tree, "gr": graft_tree, "sm": simple_tree, "pm": storey_tree, "pt": tower_tree, "fp": flip_tree}
        dim = lambda color: _draft.mix(color, _draft.BACKGROUND, 0.5)

        self._slots["tower"] = tower = self._tower(ring=False)
        self._slots["pipe"] = pipe = self._pipe(None, None, rows=self._flat_rows(0))
        count_chip = self._say(self.tr(f"{width(total)} capsules", f"{width(total)} Kapseln"), (HEAD_X, HEAD_Y), color=INK, edge="l")
        self._slots["head"] = count_chip
        self.step("hook", _engine.FadeIn(self._dock), _engine.FadeIn(tower), _engine.FadeIn(pipe), _engine.FadeIn(count_chip), seconds=1.4)

        frame = _engine.Rectangle(width=2.0, height=0.33, stroke_color=DATA, stroke_width=3, fill_opacity=0)
        rows57 = [("item", str(index), "Brep") for index in range(54, 61)] + [("line", "", "...")]
        panel57 = self._pipe(None, None, rows=rows57)
        frame.move_to(panel57[0].parts["rows"][3].get_center()).match_x(panel57[0].body)
        question = self._say(self.tr("tower? storey? facade?", "Turm? Geschoss? Fassade?"), (2.2, 3.5), color=DATA)
        self.step("flat", self._swap("pipe", panel57), self._swap("tower", self._tower()), self._swap("thread", self._thread()), _engine.FadeIn(frame), _engine.FadeIn(question), seconds=1.4)

        full_diag = self._diagram(self._tree, FULL)
        self.step("tree", _engine.FadeOut(frame), _engine.FadeOut(question), self._swap("pipe", self._pipe(None, self._tree, count=2)), self._swap("head", self._head(self._count_text(self._tree))),
                  _engine.FadeIn(full_diag, lag_ratio=0.01), seconds=2.4)
        self._slots["diag"] = full_diag

        branch_pipe = self._pipe(None, branch_tree, rows=self._rows(branch_tree, 1))
        self.step("branch", self._swap("diag", self._diagram(branch_tree, FULL, big=True)), self._swap("pipe", branch_pipe), self._swap("tower", self._tower(lit=[(0, 0, 0, 0)])),
                  self._swap("head", self._head(self.tr("1 branch · 2 items", "1 Ast · 2 Elemente"))), seconds=1.4)

        segments, index57 = self._locate(self._tree)
        roles_c = (INK, TOWER_COLORS[segments[1] % 2], GOOD, PINK)
        tower_key = towers[segments[1]].key
        names = [self._word(LEVELS[0][1]), self.tr(f"Tower {tower_key}", f"Turm {tower_key}"), self.tr(f"Storey {segments[2]}", f"Geschoss {segments[2]}"),
                 self.tr(f"Facade {segments[3]} ({FACADES[segments[3]][0]})", f"Fassade {segments[3]} ({FACADES[segments[3]][1]})")]

        def shown(stage):
            """🎯 Path text and the words of its first ``stage`` segments on a dark plate."""
            group = _engine.VGroup(self._plate((-2.75, 2.25), 4.6, 2.4))
            parts = [("{", MUTED)]
            for number, value in enumerate(segments):
                parts += [(str(value), roles_c[number] if number < stage else MUTED), (";" if number < 3 else "}", MUTED)]
            group.add(self._tokens(parts, (-2.75, 3.05), size=40))
            for number in range(stage):
                y = 2.52 - 0.4 * number
                group.add(self._mono(str(segments[number]), (-4.7, y), size=LABEL, color=roles_c[number], edge="l"), self._say(names[number], (-4.25, y), size=LABEL, color=roles_c[number], edge="l"))
            return group

        def scope(stage):
            """🔭 Capsules of the first ``stage`` path segments in their segment colours; the last two stages magnify a slice of tower B."""
            prefix = segments[:stage]
            colour = lambda key: next((roles_c[n] for n in range(stage - 1, -1, -1) if key[:n + 1] == segments[:n + 1]), None)
            if stage < 3:
                return self._tower(tint=colour)
            return self._ring(self._detail([prefix], colour, *{3: (0.2, 2), 4: (0.3, 1)}[stage]))

        route = lambda stage: self._diagram(self._tree, FULL, lit=[segments[:stage]])
        stages = [_engine.AnimationGroup(self._swap("diag", route(stage)), self._swap("tower", scope(stage)), self._swap("pipe", shown(stage))) for stage in (1, 2, 3, 4)]
        self.step("path", self._swap("head", self._head(self.tr("path = address of a branch", "Pfad = Adresse eines Asts"))), self._swap("thread", self._thread(self._tree)),
                  _engine.Succession(*stages), seconds=7.0)

        item_rows = [("path", "", _model.format_path(segments)), ("item", "0", "Brep"), ("item", "1", "Brep")]
        item_pipe = _engine.VGroup(self.gh_panel(None, rows=item_rows, at=(-3.9, 2.4), max_lines=4, max_width=2.0), self._plate((-1.55, 2.4), 2.2, 0.8),
                                   self._tokens([("{" + ";".join(map(str, segments)) + "}", MUTED), (f"[{index57}]", ITEM_C)], (-1.55, 2.4), size=20))
        right_capsule = self._ring(self._detail(lambda key: key == self._probe, lambda key: ITEM_C if key == self._probe else None, 0.3, 1))
        self.step("item", self._swap("diag", self._diagram(self._tree, FULL, lit=[(*segments, index57)], big=True)), self._swap("pipe", item_pipe), self._swap("tower", right_capsule),
                  self._swap("head", self._head(self.tr("index = position in the branch", "Index = Position im Ast"))), seconds=1.4)

        viewer = self.gh_param_viewer(_model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:4]}), at=(-3.7, 2.45), max_items=2)
        panel_tree = self.gh_panel(None, rows=self._rows(self._tree, 3), at=(-1.3, 2.4), max_lines=8, max_width=1.7)
        viewer_pipe = _engine.VGroup(viewer, panel_tree, self.gh_wire(self._src.grip_out(), viewer.grip_in(), kind="tree"), self.gh_wire(self._src.grip_out(), panel_tree.grip_in(), kind="tree"),
                                     self._say(self.tr("Param Viewer: 70 branches", "Param Viewer: 70 Äste"), (-3.7, 1.15), size=SMALL, color=SHEET_INK))
        stats = self.gh_component("Tree Statistics", at=(-4.2, 2.4))
        p_panel = self.gh_panel(None, rows=[("line", "", _model.format_path(path)) for path in self._tree.paths()[:2]], at=(-1.5, 3.3), max_lines=2, max_width=1.6)
        l_panel = self.gh_panel(None, rows=self._rows(self._sol.tree("stl"), 1)[1:4] + [("line", "", "...")], at=(-1.5, 2.2), max_lines=4, max_width=1.6)
        c_panel = self.gh_panel(None, rows=[("line", "", str(self._sol.tree("stc").branch((0,))[0]))], at=(-1.5, 1.2), max_lines=1, max_width=1.6)
        stats_pipe = _engine.VGroup(stats, p_panel, l_panel, c_panel, self.gh_wire(self._src.grip_out(), stats.grip_in(0), kind="tree"),
                                    self.gh_wire(stats.grip_out("Paths"), p_panel.grip_in(), kind="tree"), self.gh_wire(stats.grip_out("Length"), l_panel.grip_in(), kind="tree"),
                                    self.gh_wire(stats.grip_out("Count"), c_panel.grip_in(), kind="tree"))
        top = lambda key: key[1] == 1 and key[2] == 10
        below = lambda key: key[1] == 1 and key[2] == 9
        uneven = self._tower(tint=lambda key: GOOD if top(key) else STOREY_LIGHT if below(key) else TOWER_COLORS[key[1]])
        tags = self._chips((self.tr("B, storey 9: 4 facades", "B, Geschoss 9: 4 Fassaden"), STOREY_LIGHT), (self.tr("B, storey 10: 3 facades", "B, Geschoss 10: 3 Fassaden"), GOOD))
        stage_a = _engine.AnimationGroup(self._swap("pipe", viewer_pipe), self._swap("diag", self._diagram(self._tree, FULL)), self._swap("tower", self._tower(tint=lambda key: TOWER_COLORS[key[1]])),
                                         self._swap("head", self._head(self._count_text(self._tree))))
        stage_b = _engine.AnimationGroup(self._swap("pipe", stats_pipe), self._swap("tower", uneven), self._swap("tags", tags))
        self.step("panel", _engine.Succession(stage_a, stage_b), seconds=3.6)

        def derived(op, tree, levels):
            """🧬 One restructuring result: component, Panel, diagram, count line and the address of capsule 57 all read from the evaluated tree."""
            return self._pipe(op, tree), self._diagram(tree, levels, big=len(tree.paths()) < 5), self._head(self._count_text(tree)), (self._thread(tree) if self._has(tree) else None)

        def operate(key, op, small, big, levels, small_lit, seconds, tower_big):
            """🔬 Introduce an operation on the smallest example in its own clause, then run it on the whole tower in the next."""
            first, second = derived(op, small, levels[0]), derived(op, big, levels[1])
            stage = lambda parts, tower: [self._swap("pipe", parts[0]), self._swap("diag", parts[1]), self._swap("head", parts[2]), self._swap("thread", parts[3]), self._swap("tower", tower)]
            self.step(key + "_one", self._swap("tags", None), *stage(first, self._tower(lit=[small_lit])), seconds=seconds[0])
            self.step(key, *stage(second, tower_big), seconds=seconds[1])

        one_storey = self._small(self._tree, (0, 0, 0))
        sweep = self._tower(tint=lambda key: INK)
        plain = lambda: self._tower(tint=lambda key: TOWER_COLORS[key[1]])
        operate("flatten", "Flatten Tree", one_storey.flatten(), flat_tree, ((5,), (5,)), (0, 0, 0), (1.6, 2.6), sweep)
        operate("graft", "Graft Tree", one_storey.graft(), graft_tree, ((0, 1, 2, 3, 4), (0, 1, 2, 3, 4)), (0, 0, 0), (1.6, 1.6), plain())
        operate("simplify", "Simplify Tree", one_storey.simplify(), simple_tree, ((3,), (1, 2, 3)), (0, 0, 0), (1.6, 1.6), plain())

        j57 = self._locate(storey_tree)
        group57 = (segments[0], segments[1], segments[2])
        binding = _engine.VGroup(self._plate((-3.55, 2.4), 6.1, 2.55), self._rule_tokens("{A;B;C;D}(i)", (-5.2, 3.35), SMALL))
        arrow = self.mono("→", (0, 0, 0), size=SMALL, color=MUTED)
        binding.add(self._put(arrow, (-3.7, 3.35)), self._rule_tokens("{A;B;C}(i)", (-2.2, 3.35), SMALL))
        letters_row = self._tokens([item for n, v, c in zip("ABCD", segments, roles_c) for item in ((f"{n}={v}", c), ("   ", MUTED))][:-1], (-3.55, 2.85), size=22)
        binding.add(letters_row)
        d_mob = _engine.VGroup(*letters_row[-1:])
        binding.add(_engine.Line(d_mob.get_left() + np.array([0, 0.02, 0]), d_mob.get_right() + np.array([0, 0.02, 0]), color=PINK, stroke_width=3))
        before = self._path_parts(segments, index57, roles_c)
        after = self._path_parts(j57[0], j57[1], roles_c)
        row = self._tokens([*before, ("  →  ", MUTED), *after], (-3.55, 2.3), size=22)
        binding.add(row, self._say(self.tr(f"Facade items join in order: index {index57} becomes {j57[1]}.", f"Die Elemente der Fassaden werden der Reihe nach verbunden: Index {index57} wird {j57[1]}."),
                                   (-3.55, 1.6), size=SMALL, color=INK, width=5.6))
        eight = len(storey_tree.branch(j57[0]))
        example_tower = self._tower(tint=lambda key: GOOD if key[:3] == group57 else None)
        self.step("mapper_example", self._swap("pipe", binding), self._swap("diag", self._diagram(self._tree, FULL, lit=[group57])), self._swap("tower", example_tower),
                  self._swap("tags", self._chips((self.tr(f"tower {tower_key}, storey {segments[2]}: {width(eight)} capsules", f"Turm {tower_key}, Geschoss {segments[2]}: {width(eight)} Kapseln"), GOOD))), seconds=2.6)

        map_pipe = self._pipe("Path Mapper", storey_tree, mapping="{A;B;C;D}(i)->{A;B;C}(i)")
        map_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B;C}(i)", (-3.1, 1.17)))
        self.step("mapper", self._swap("pipe", map_pipe), self._swap("diag", self._diagram(storey_tree, (0, 1, 2), big=True)), self._swap("head", self._head(self._count_text(storey_tree))),
                  self._swap("thread", self._thread(storey_tree)), self._swap("tower", plain()), self._swap("tags", None), seconds=1.6)

        sizes = {(0, 0, 0): per_storey[0], (0, 1, 4): per_storey[9 + 4], (0, 1, 10): per_storey[-1]}
        names_s = {(0, 0, 0): ("A, storey 0", "A, Geschoss 0"), (0, 1, 4): ("B, storey 4", "B, Geschoss 4"), (0, 1, 10): ("B, storey 10", "B, Geschoss 10")}
        stages = []
        for prefixes in (((0, 0, 0),), ((0, 1, 10),), ((0, 1, 4), (0, 1, 10))):
            chips = self._chips(*[(self.tr(f"{names_s[p][0]}: {width(sizes[p])} capsules", f"{names_s[p][1]}: {width(sizes[p])} Kapseln"), GOOD) for p in prefixes])
            drawing = self._tower(tint=lambda key, ps=prefixes: GOOD if any(key[:3] == p for p in ps) else None)
            stages.append(_engine.AnimationGroup(self._swap("tower", drawing), self._swap("tags", chips)))
        self.step("perstorey", _engine.Succession(*stages), seconds=4.0)

        tower_pipe = self._pipe("Path Mapper", tower_tree, count=2, mapping="{A;B;C;D}(i)->{A;B}(i)")
        tower_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B}(i)", (-3.1, 1.17)))
        stages = []
        for group in ((0,), (1,), (0, 1)):
            chips = self._chips(*[(self.tr(f"tower {towers[i].key}: {width(per_tower[i])} capsules", f"Turm {towers[i].key}: {width(per_tower[i])} Kapseln"), TOWER_COLORS[i]) for i in group])
            drawing = self._tower(tint=lambda key, g=group: TOWER_COLORS[key[1]] if key[1] in g else None)
            stages.append(_engine.AnimationGroup(self._swap("tower", drawing), self._swap("tags", chips)))
        self.step("pertower", self._swap("pipe", tower_pipe), self._swap("diag", self._diagram(tower_tree, (0, 1), big=True)), self._swap("head", self._head(self._count_text(tower_tree))),
                  self._swap("thread", self._thread(tower_tree)), _engine.Succession(*stages), seconds=3.2)

        item_tone = (ITEM_C, PEACH)
        side_tower = self._tower(tint=lambda key: item_tone[key[4]])
        before_diag = self._diagram(self._tree, FULL, leaf_color=lambda path, index: item_tone[index])
        after_diag = self._diagram(flip_tree, (5,), leaf_color=lambda path, index: item_tone[path[0]], big=True)
        flip_pipe = self._pipe("Flip Matrix", flip_tree)
        chips = self._chips((self.tr("left capsules: 70", "linke Kapseln: 70"), ITEM_C), (self.tr("right capsules: 70", "rechte Kapseln: 70"), PEACH))
        stage_a = _engine.AnimationGroup(self._swap("tower", side_tower), self._swap("diag", before_diag), self._swap("head", self._head(self.tr("70 branches of 2", "70 Äste mit je 2"))),
                                         self._swap("tags", chips), self._swap("pipe", None))
        stage_b = _engine.AnimationGroup(self._swap("pipe", flip_pipe), self._swap("diag", after_diag), self._swap("head", self._head(self.tr("70 × 2 → 2 × 70", "70 × 2 → 2 × 70"))),
                                         self._swap("thread", self._thread(flip_tree)))
        self.step("flip", _engine.Succession(stage_a, stage_b), seconds=3.6)

        words = (self.tr("teal", "türkis"), self.tr("orange", "orange"))
        preview = self.gh_component("Custom Preview", at=(-2.9, 2.4))
        colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-5.3, 1.42), max_lines=2, max_width=1.4)
        swatches = self._swatches(colors, (ARCH, WARN))
        match_pipe = _engine.VGroup(preview, colors, swatches, self.gh_wire(self._src.grip_out(), preview.grip_in(0), kind="tree"), self.gh_wire(colors.grip_out(), preview.grip_in(1), kind="list"))
        intent = self._chips((self.tr("tower A: teal", "Turm A: türkis"), ARCH), (self.tr("tower B: orange", "Turm B: orange"), WARN))
        self.step("match", self._swap("pipe", match_pipe), self._swap("diag", self._diagram(self._tree, FULL)), self._swap("tower", plain()),
                  self._swap("head", self._head(self._count_text(self._tree))), self._swap("thread", self._thread(self._tree)), self._swap("tags", intent), seconds=1.6)

        stripes = self._tower(tint=lambda key: MATERIALS[paint["cp1"][key]])
        warn_note = self._chips((self.tr("no error: 70 teal, 70 orange", "kein Fehler: 70 türkis, 70 orange"), INK))
        mistake_diag = self._diagram(self._tree, FULL, leaf_color=lambda path, index: MATERIALS[paint["cp1"][(*path, index)]])
        self.step("mistake", self._swap("tower", stripes), self._swap("tags", warn_note), self._swap("diag", mistake_diag), seconds=1.6)

        viewer_a = self.gh_param_viewer(_model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:3]}), at=(-3.85, 2.4), max_items=2)
        viewer_b = self.gh_param_viewer(_model.Tree({(0,): [None, None]}), at=(-1.4, 2.4), max_items=2)
        count_a = self._tree.stats()["path_count"]
        paths_plate = _engine.VGroup(self._plate((-2.75, 1.2), 5.0, 0.44),
                                     self._tokens([*self._path_parts(self._tree.paths()[0], 0, ROLE_COLORS)[:-1], (f" ×{count_a}", MUTED)], (-4.2, 1.2), size=SMALL),
                                     self._tokens([("{", MUTED), ("0", INK), ("}", MUTED), (" ×1", MUTED)], (-1.5, 1.2), size=SMALL))
        mini = lambda tint, x: self.tower_axon(at=(x, -0.95, 0.0), scale=0.05, view="axon", tint=tint, types=False)
        compare = _engine.VGroup(mini(lambda key: TOWER_COLORS[key[1]], -5.0), mini(lambda key: MATERIALS[paint["cp1"][key]], -2.2),
                                 self._say(self.tr("expected", "erwartet"), (-5.0, 0.42), size=SMALL, color=INK), self._say(self.tr("actual", "tatsächlich"), (-2.2, 0.42), size=SMALL, color=INK))
        self.step("why", self._swap("pipe", _engine.VGroup(viewer_a, viewer_b, paths_plate)), self._swap("diag", compare), self._swap("head", None),
                  self._swap("tags", self._chips((self.tr("different paths", "verschiedene Pfade"), SERV))), seconds=2.4)

        fix_map = self.gh_component("Path Mapper", at=(-2.9, 2.85), scale=0.8)
        fix_text = self.gh_panel(None, rows=[("line", "", "{A;B;C;D}(i)"), ("line", "", "->{A;B}(i)")], at=(-5.7, 1.38), max_lines=2, max_width=1.9)
        fix_colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-3.8, 1.4), max_lines=2, max_width=1.6)
        fix_swatches = self._swatches(fix_colors, (ARCH, WARN))
        fix_graft = self.gh_component("Graft Tree", at=(-2.25, 1.4), scale=0.8)
        fix_preview = self.gh_component("Custom Preview", at=(-0.95, 2.5), scale=0.8)
        first_part = _engine.VGroup(fix_map, fix_text, self.gh_wire(self._src.grip_out(), fix_map.grip_in(0), kind="tree"), self.gh_wire(fix_text.grip_out(), fix_map.grip_in(1), kind="item"))
        second_part = _engine.VGroup(fix_colors, fix_swatches, fix_graft, fix_preview, self.gh_wire(fix_colors.grip_out(), fix_graft.grip_in(0), kind="list"),
                                     self.gh_wire(fix_map.grip_out(0), fix_preview.grip_in(0), kind="tree"), self.gh_wire(fix_graft.grip_out(0), fix_preview.grip_in(1), kind="tree"))
        fix_diag = self._diagram(tower_tree, (0, 1), big=True, leaf_color=lambda path, index: MATERIALS[paint["cp2"][(*path, index)]] if (*path, index) in paint["cp2"] else ITEM_C)
        mapped_diag = self._diagram(tower_tree, (0, 1), big=True)
        dim_towers = self._tower(tint=lambda key: dim(TOWER_COLORS[key[1]]))
        stage_a = _engine.AnimationGroup(self._swap("pipe", first_part), self._swap("diag", mapped_diag), self._swap("tower", dim_towers), self._swap("tags", None),
                                         self._swap("head", self._head(self.tr("{0;0} and {0;1}", "{0;0} und {0;1}"))), self._swap("thread", self._thread(tower_tree)))
        self._slots["pipe"] = _engine.VGroup(first_part, second_part)
        stage_b = _engine.AnimationGroup(_engine.FadeIn(second_part), self._swap("diag", fix_diag), self._swap("tags", self._chips((self.tr("equal paths", "gleiche Pfade"), GOOD))),
                                         self._swap("head", self._head(self.tr("{0;0} meets {0;0} · {0;1} meets {0;1}", "{0;0} trifft {0;0} · {0;1} trifft {0;1}"))))
        self.step("fix", _engine.Succession(stage_a, stage_b), seconds=3.2)

        counts = {name: sum(1 for value in paint["cp2"].values() if value == name) for name in ("teal", "orange")}
        done = self._tower(tint=lambda key: MATERIALS[paint["cp2"][key]])
        result_tags = self._chips((self.tr(f"tower A: {width(counts['teal'])} teal", f"Turm A: {width(counts['teal'])} türkis"), ARCH),
                                  (self.tr(f"tower B: {width(counts['orange'])} orange", f"Turm B: {width(counts['orange'])} orange"), WARN))
        self.step("result", self._swap("tower", done), self._swap("tags", result_tags), seconds=1.6)

        self.step("recap", *self._recap(), seconds=2.0)

