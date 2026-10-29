import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def sub(old, new):
    global s
    assert old in s, old[:80]
    s = s.replace(old, new, 1)


def cut(start, end, new):
    global s
    a = s.index(start)
    b = s.index(end, a)
    s = s[:a] + new + s[b:]


# constants
sub('INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "DATA", "GOOD", "WARN"))','INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN, SERV = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "DATA", "GOOD", "WARN", "SERV"))')
sub('PINK, ITEM_C = "#ff9db0", "#9fb4ff"\nTOWER_B, STOREY_LIGHT, PEACH = "#b48cff", _draft.mix(GOOD, INK, 0.55), "#ffcf9f"\nROLE_COLORS = (INK, STRUCT, GOOD, PINK)\nLEVELS = ((INK, ("Building", "Gebäude")), (STRUCT, ("Tower", "Turm")),',
    'PINK, ITEM_C, TOWER_GREY = "#ff9db0", "#9fb4ff", "#d8d8d8"\nSTOREY_LIGHT, PEACH = _draft.mix(GOOD, INK, 0.55), "#ffcf9f"\nTOWER_COLORS = (ARCH, WARN)\nROLE_COLORS = (INK, TOWER_GREY, GOOD, PINK)\nLEVELS = ((INK, ("Building", "Gebäude")), (TOWER_GREY, ("Tower", "Turm")),')

# narration
cut('        ["path", "Read the path', '        ["flatten_one"', '''        ["path", "Capsule 57 has the path {0;1;0;1}: building 0, tower 1, which is B, storey 0, facade 1.",
         "Kapsel 57 hat den Pfad {0;1;0;1}: Gebäude 0, Turm 1, das ist B, Geschoss 0, Fassade 1."],
        ["item", "The branch is a list. An index addresses one item in it: {0;1;0;1}[1] is capsule 57, the right capsule of that facade.",
         "Der Ast ist eine Liste. Ein Index adressiert ein Element darin: {0;1;0;1}[1] ist Kapsel 57, die rechte Kapsel dieser Fassade."],
        ["panel", "A Panel in tree mode lists every branch with its path. Tree Statistics counts 70 branches of two items each.",
         "Ein Panel im Baummodus listet jeden Ast mit seinem Pfad. Tree Statistics zählt 70 Äste mit je zwei Elementen."],
''')
cut('        ["mapper", "Path Mapper rewrites', '        ["perstorey"', '''        ["mapper_example", "Path Mapper binds the letters to a real path. For capsule 57, A is 0, B is 1, C is 0 and D is 1. D, the facade, is dropped.",
         "Path Mapper bindet die Buchstaben an einen echten Pfad. Bei Kapsel 57 ist A gleich 0, B gleich 1, C gleich 0 und D gleich 1. D, die Fassade, entfällt."],
        ["mapper", "Applied to all branches, the rule {A;B;C;D}(i) to {A;B;C} merges the facades: each branch is a storey, 20 branches.",
         "Auf alle Äste angewendet, fasst die Regel {A;B;C;D}(i) nach {A;B;C} die Fassaden zusammen: Jeder Ast ist ein Geschoss, 20 Äste."],
''')
cut('        ["flip", "Flip Matrix swaps', '        ["match"', '''        ["flip", "Flip Matrix swaps branches and items, so all left capsules form one list and all right ones another: two branches of 70.",
         "Flip Matrix tauscht Äste und Elemente: Alle linken Kapseln bilden eine Liste, alle rechten eine andere, zwei Äste mit je 70."],
''')
cut('        ["why", "The Param Viewer shows why', '        ["fix"', '''        ["why", "The Param Viewer shows why: no path matches, so the short colour list repeats in every branch. Read both paths first.",
         "Der Param Viewer zeigt, warum: Kein Pfad passt, also wiederholt sich die kurze Farbliste in jedem Ast. Erst beide Pfade lesen."],
''')
sub(" Always read both paths first.\",\n         \"Mit passenden", "\",\n         \"Mit passenden")
sub(" Vorher beide Pfade lesen.\"", "\"")
sub("Flip Matrix. Ten terms, one address.", "Flip Matrix. Ten terms, one address.")
sub("Flip Matrix. Zehn Begriffe.", "Flip Matrix. Zehn Begriffe, eine Adresse.")

# helpers
sub('''    def _tower(self, lit=None, tint=None, types=False):
        """🗼 The full Nakagin tower drawing at the stage anchor."""
        return self.tower_axon(at=TOWER_AT, scale=TOWER_SCALE, view="axon", lit=lit, tint=tint, types=types)
''', '''    def _tower(self, lit=None, tint=None, types=False, ring=True):
        """🗼 The full Nakagin tower drawing at the stage anchor, capsule 57 ringed."""
        drawing = self.tower_axon(at=TOWER_AT, scale=TOWER_SCALE, view="axon", lit=lit, tint=tint, types=types)
        return self._ring(drawing) if ring else drawing

    def _ring(self, drawing):
        """⭕ A yellow ring around capsule 57, the object that runs through the whole video."""
        mob = drawing.capsules.get(self._probe)
        if mob is not None:
            drawing.add(_engine.Circle(radius=max(mob.width, mob.height) * 0.8, stroke_color=DATA, stroke_width=3.2, fill_opacity=0).move_to(mob.get_center()))
        return drawing

    def _locate(self, tree):
        """🔎 Path and index of capsule 57 in an engine tree."""
        for path in tree.paths():
            for index, item in enumerate(tree.branch(path)):
                if (*item.tag["slot"],) == self._probe:
                    return path, index
        raise KeyError("capsule 57")

    def _thread(self, tree=None):
        """🧵 The running chip: where capsule 57 sits in the tree on screen."""
        if tree is None:
            text = self.tr("capsule 57 = ?", "Kapsel 57 = ?")
        else:
            path, index = self._locate(tree)
            text = f"#57 = {_model.format_path(path)}[{index}]"
        return self._tag(text, (-0.35, HEAD_Y), DATA, "r")
''')

# diagram: tower colours
sub('''        colors = [*[LEVELS[i][0] for i in levels], ITEM_C]
        buckets, dots = {}, _engine.VGroup()''', '''        colors = [*[LEVELS[i][0] for i in levels], ITEM_C]
        tone_of = lambda prefix: TOWER_COLORS[prefix[len(prefix) - 1] % 2] if levels[len(prefix) - 1] == 1 else colors[len(prefix) - 1]
        buckets, dots = {}, _engine.VGroup()''')
sub('key = (colors[level - 1], 1.0 if bright(prefix) else 0.16, 2.6 if level == 1 else 1.6 if level < 3 else 1.1)', 'key = (tone_of(prefix), 1.0 if bright(prefix) else 0.16, 2.6 if level == 1 else 1.6 if level < 3 else 1.1)')
sub('radius=0.075 if level == 1 else 0.05 if level == 2 else 0.035, color=colors[level - 1],', 'radius=0.075 if level == 1 else 0.05 if level == 2 else 0.035, color=tone_of(prefix),')

# mapping letters
sub('letters = {"A": INK, "B": STRUCT, "C": GOOD, "D": PINK, "i": ITEM_C}', 'letters = {"A": INK, "B": TOWER_GREY, "C": GOOD, "D": PINK, "i": ITEM_C}')

# detail: centre storey, default view
cut("    def _detail(self, lit, tint, scale, span):", "    def _lit_all", '''    def _detail(self, lit, tint, scale, span):
        """🔍 A magnified slice of tower B around storey 0: only the storeys within ``span`` of it are drawn."""
        drawing = self.tower_axon(at=(4.3, 0.6, 0.0), scale=scale, view="axon", lit=lit, tint=tint, towers=(1,), core=False, capital=False, podium=False, types=False,
                                  where=lambda slot: slot.storey <= span)
        return drawing

''')

# recap + mini
cut("    def _recap(self):", "    def _mini(", '''    def _recap(self):
        """📚 The ten terms as a legend beside the tower, each with a mini tree computed by the engine; capsule 57 and its address stay on screen."""
        keep = {self._slots["tower"], self._slots["thread"], self._caption}
        gone = [_engine.FadeOut(mob) for mob in self.mobjects if mob not in keep]
        base = _model.Tree.from_dict({"{0;0}": ["a", "b"], "{0;1}": ["c", "d"]})
        tower = self._tree
        found = {name: self._locate(tree) for name, tree in self._results.items()}
        text = lambda name: f"{_model.format_path(found[name][0])}[{found[name][1]}]"
        rows = [
            (("Tree", "Datenbaum"), base, self.tr(f"{tower.path_count} branches · {tower.count} items", f"{tower.path_count} Äste · {tower.count} Elemente")),
            (("Branch", "Ast"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), self.tr("2 items per facade", "2 Elemente je Fassade")),
            (("Path", "Pfad"), _model.Tree.from_dict({"{0;1}": ["a"]}), _model.format_path(found["src"][0])),
            (("Item", "Element"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), self.tr("capsule 57", "Kapsel 57")),
            (("Index", "Index"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), f"[{found['src'][1]}]"),
            (("Flatten", "Flatten"), base.flatten(), f"{tower.path_count} × 2 → 1 × {tower.count}: {text('fl')}"),
            (("Graft", "Graft"), base.graft(), f"{tower.path_count} × 2 → {tower.count} × 1: {text('gr')}"),
            (("Simplify", "Simplify"), _model.Tree.from_dict({"{0;0;0}": ["a", "b"], "{0;0;1}": ["c", "d"]}).simplify(), text("sm")),
            (("Path Mapper", "Path Mapper"), base.map("{A;B}(i) -> {A}"), f"{text('pm')} · {text('pt')}"),
            (("Flip Matrix", "Flip Matrix"), base.flip(), f"{tower.path_count} × 2 → 2 × {tower.path_count}: {text('fp')}"),
        ]
        group = _engine.VGroup()
        for number, (term, tree, result) in enumerate(rows):
            y = 3.45 - 0.62 * number
            label = self._word(term) + ("" if term[0] == term[1] else " (" + self._word(term[::-1]) + ")")
            group.add(self._say(label, (HEAD_X, y), size=SMALL, color=INK, edge="l"), self._mini(tree, (-2.75, y), 0.62),
                      self._mono(result, (-2.15, y), size=SMALL, color=DATA, edge="l"))
        return [*gone, _engine.FadeIn(group, lag_ratio=0.1)]

''')
sub("    def _mini(self, tree, point):", "    def _mini(self, tree, point, scale=1.0):")
sub("        xs = np.linspace(-0.55, 0.55, count) if count > 1 else np.array([0.0])\n        ys = np.linspace(-0.42, 0.42, depth + 2)", "        xs = np.linspace(-0.55, 0.55, count) * scale if count > 1 else np.array([0.0])\n        ys = np.linspace(-0.42, 0.42, depth + 2) * scale")

# present_trees body
cut("    def present_trees(self):", "    def _recap(self):", open(sys.argv[2], encoding="utf-8").read())
open(p, "w", encoding="utf-8").write(s)
