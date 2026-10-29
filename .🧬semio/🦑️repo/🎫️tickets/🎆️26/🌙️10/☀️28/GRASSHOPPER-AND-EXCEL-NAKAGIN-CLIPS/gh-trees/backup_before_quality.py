"""🌳 Data trees on the aggregated Nakagin tower: paths, branches, items, Flatten, Graft, Simplify, Path Mapper, Flip Matrix and branch matching."""

import importlib
import sys
from pathlib import Path

import numpy as np

_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
_canvas = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🦗️canvas.🐍️")
_model = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
_nakagin = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_draft = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
_engine = importlib.import_module("manim")

INK, MUTED, ARCH, STRUCT, DATA, GOOD, WARN = (getattr(_visuals, name) for name in ("INK", "MUTED", "ARCH", "STRUCT", "DATA", "GOOD", "WARN"))
PINK, ITEM_C = "#ff9db0", "#9fb4ff"
TOWER_B, STOREY_LIGHT, PEACH = "#b48cff", _draft.mix(GOOD, INK, 0.55), "#ffcf9f"
ROLE_COLORS = (INK, STRUCT, GOOD, PINK)
LEVELS = ((INK, ("Building", "Gebäude")), (STRUCT, ("Tower", "Turm")), (GOOD, ("Storey", "Geschoss")), (PINK, ("Facade", "Fassade")), (ITEM_C, ("Index", "Index")), (INK, ("Branch", "Ast")))
FULL = (0, 1, 2, 3)
ROLES = (("Building", "Gebäude"), ("Tower", "Turm"), ("Storey", "Geschoss"), ("Facade", "Fassade"))
ITEM_WORD = ("Capsules", "Kapseln")
FACADES = (("north", "Norden"), ("east", "Osten"), ("south", "Süden"), ("west", "Westen"))
MATERIALS = {"teal": ARCH, "orange": WARN}
LABEL, SMALL = 19.2, 16.2
TOWER_AT, TOWER_SCALE = np.array([4.3, 0.62, 0.0]), 0.125
SHEET_AT, SHEET_W, SHEET_H = (-3.55, 2.4, 0), 6.5, 2.9
SRC_AT, COMP_AT, PANEL_AT = (-5.95, 2.4, 0), (-3.9, 2.4, 0), (-1.45, 2.4, 0)
HEAD_X = -6.8
FAN_LEFT, FAN_RIGHT, GROUND, CROWN = -5.35, -0.5, -2.3, 0.3
HEAD_Y = 0.66
SOURCE, FLAT_LINES = "src", 7
SHEET_INK, SHEET_GOOD = "#222222", "#1E6B2E"


def _fade(group, alpha):
    """🌫️ Scale every stroke and fill opacity of a freshly built group by an alpha."""
    for mob in group.get_family():
        mob.set_fill(opacity=float(mob.get_fill_opacity()) * alpha)
        mob.set_stroke(opacity=float(mob.get_stroke_opacity()) * alpha)
    return group


class Beat5_DataTrees(_visuals.ClipScene):
    """🌳 Animate Grasshopper data trees on the aggregated Nakagin tower with explicit English and German narration."""

    TITLE = {"en": "Data Trees on the Tower", "de": "Datenbäume am Turm"}
    NARRATION = [
        ["hook", "In this teaching model, the Nakagin Capsule Tower has two towers and 140 capsules. One list holds them all, but which one sits where?",
         "In diesem Lehrmodell hat der Nakagin Capsule Tower zwei Türme und 140 Kapseln. Eine Liste enthält alle, aber welche Kapsel sitzt wo?"],
        ["flat", "Item 57 is one capsule of 140. The list does not say its tower, its storey or its facade.",
         "Element 57 ist eine von 140 Kapseln. Die Liste verrät weder Turm noch Geschoss noch Fassade."],
        ["tree", "A data tree keeps the address: building, tower, storey, facade. Each level splits into more branches.",
         "Ein Datenbaum behält die Adresse: Gebäude, Turm, Geschoss, Fassade. Jede Ebene verzweigt sich weiter."],
        ["branch", "The smallest piece: tower A, storey 0, facade 0. Its two capsules form one branch, a list of two items.",
         "Das kleinste Stück: Turm A, Geschoss 0, Fassade 0. Seine zwei Kapseln bilden einen Ast, eine Liste aus zwei Elementen."],
        ["path", "Read the path {0;1;4;2} from left to right: building 0, tower 1, which is B, storey 4, facade 2.",
         "Lies den Pfad {0;1;4;2} von links nach rechts: Gebäude 0, Turm 1, das ist B, Geschoss 4, Fassade 2."],
        ["item", "The branch is a list. An index addresses one item in it: {0;1;4;2}[1] is the right capsule of that facade.",
         "Der Ast ist eine Liste. Ein Index adressiert ein Element darin: {0;1;4;2}[1] ist die rechte Kapsel dieser Fassade."],
        ["panel", "A Panel in tree mode lists every branch with its path. The Param Viewer draws the same tree as 70 branches.",
         "Ein Panel im Baummodus listet jeden Ast mit seinem Pfad. Der Param Viewer zeichnet denselben Baum als 70 Äste."],
        ["stats", "Tree Statistics counts 70 branches and 140 items. The top storey of tower B has only three facades.",
         "Tree Statistics zählt 70 Äste und 140 Elemente. Das oberste Geschoss von Turm B hat nur drei Fassaden."],
        ["flatten_one", "Flatten first, on one storey of tower A: its three branches become one list of six items.",
         "Zuerst Flatten an einem Geschoss von Turm A: Seine drei Äste werden zu einer Liste aus sechs Elementen."],
        ["flatten", "Flatten puts all 140 capsules into one list. Nothing is lost, except the structure.",
         "Flatten legt alle 140 Kapseln in eine Liste. Es geht nichts verloren, außer der Struktur."],
        ["graft_one", "Graft on the same storey: each of the six capsules gets its own branch, six branches of one item.",
         "Graft am selben Geschoss: Jede der sechs Kapseln bekommt einen eigenen Ast, sechs Äste mit je einem Element."],
        ["graft", "Graft does the opposite: every capsule gets its own branch, and the path grows by one index.",
         "Graft macht das Gegenteil: Jede Kapsel bekommt einen eigenen Ast, und der Pfad wird um einen Index länger."],
        ["simplify_one", "Simplify on that storey strips the path prefix {0;0;0} that its three branches share. The branches stay.",
         "Simplify entfernt am selben Geschoss das gemeinsame Pfadpräfix {0;0;0}. Die Äste bleiben."],
        ["simplify", "Simplify removes the leading 0 that every path shares, because there is only one building. The branches stay.",
         "Simplify entfernt die führende 0, die alle Pfade teilen, weil es nur ein Gebäude gibt. Die Äste bleiben."],
        ["mapper", "Path Mapper rewrites paths. The rule {A;B;C;D}(i) to {A;B;C} names the segments and drops the facade.",
         "Path Mapper schreibt Pfade um. Die Regel {A;B;C;D}(i) wird zu {A;B;C}: Sie benennt die Segmente und lässt die Fassade weg."],
        ["perstorey", "Now each branch is a storey: six capsules where it has three facades, eight where it has four.",
         "Jetzt ist jeder Ast ein Geschoss: sechs Kapseln bei drei Fassaden, acht bei vier."],
        ["pertower", "Mapping to {A;B} gives one branch per tower: 54 capsules in tower A, 86 in tower B.",
         "Das Abbilden auf {A;B} ergibt einen Ast je Turm: 54 Kapseln in Turm A, 86 in Turm B."],
        ["flip", "Flip Matrix swaps branches and items: 70 branches of two items become two branches of 70.",
         "Flip Matrix tauscht Äste und Elemente: 70 Äste mit je zwei Elementen werden zu zwei Ästen mit je 70."],
        ["match", "A component pairs data branch by branch. Custom Preview takes capsules and a colour, and the paths decide who pairs with whom.",
         "Eine Komponente verknüpft Daten Ast für Ast. Custom Preview nimmt Kapseln und eine Farbe, und die Pfade entscheiden, wer zu wem gehört."],
        ["mistake", "The classic mistake: the short colour list repeats in every branch. Left capsules turn teal, right ones orange, in both towers.",
         "Der klassische Fehler: Die kurze Farbliste wiederholt sich in jedem Ast. Die linken Kapseln werden türkis, die rechten orange, in beiden Türmen."],
        ["why", "The Param Viewer shows why: 70 branches of depth four against one branch of depth one. The paths never meet.",
         "Der Param Viewer zeigt, warum: 70 Äste der Tiefe vier gegen einen Ast der Tiefe eins. Die Pfade treffen sich nie."],
        ["fix", "The fix: map the capsules to {A;B} and graft the colour list. Now both trees have the paths {0;0} and {0;1}.",
         "Die Lösung: Kapseln auf {A;B} abbilden und die Farbliste mit Graft Tree verteilen. Nun tragen beide Bäume die Pfade {0;0} und {0;1}."],
        ["result", "With matching paths, each tower's colour reaches all its capsules: 54 teal in tower A, 86 orange in tower B. Always read both paths first.",
         "Mit passenden Pfaden erreicht die Farbe jedes Turms alle seine Kapseln: 54 türkise in Turm A, 86 orange in Turm B. Vorher beide Pfade lesen."],
        ["recap", "Recap: tree, branch, path, item, index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Ten terms, one address.",
         "Zusammenfassung: Datenbaum, Ast, Pfad, Element, Index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Zehn Begriffe."]
    ]

    def construct(self):
        """▶️ Present this chapter through the native Grasshopper visual interface."""
        self.present("trees")

    def _put(self, shape, point, edge="c"):
        """📌 Move a shape so its left edge, right edge or centre sits on ``point``."""
        dx = {"l": point[0] - shape.get_left()[0], "r": point[0] - shape.get_right()[0], "c": point[0] - shape.get_center()[0]}[edge]
        return shape.shift(np.array([dx, point[1] - shape.get_center()[1], 0.0]))

    def _say(self, string, point, *, size=LABEL, color=INK, edge="c", width=None):
        """🔤 One label aligned on its left edge, right edge or centre."""
        return self._put(self.text(string, (0, 0, 0), size=size, color=color, width=width), point, edge)

    def _mono(self, string, point, *, size=LABEL, color=INK, edge="c"):
        """⌨️ One monospace label aligned on its left edge, right edge or centre."""
        return self._put(self.mono(string, (0, 0, 0), size=size, color=color), point, edge)

    def _word(self, pair):
        """🈯 Localized word of an English and German pair."""
        return self.tr(*pair)

    def _tokens(self, parts, point, *, size=LABEL, edge="c"):
        """🎨 A monospace row of coloured ``(text, colour)`` tokens that touch each other."""
        row = _engine.VGroup(*[self.mono(text, (0, 0, 0), size=size, color=color) for text, color in parts]).arrange(_engine.RIGHT, buff=0.045)
        return self._put(row, point, edge)

    def _count_text(self, tree):
        """🔢 Branch and item count of a tree as one localized line."""
        stats = tree.stats()
        branches, items = stats["path_count"], stats["item_count"]
        return self.tr(f"{branches} {'branch' if branches == 1 else 'branches'} · {items} items", f"{branches} {'Ast' if branches == 1 else 'Äste'} · {items} Elemente")

    def _rows(self, tree, count=3, only=None):
        """📝 Panel rows (path header, then indexed items) of the first ``count`` branches or of ``only``."""
        rows, last, seen = [], None, 0
        for path, index, text in _model.panel_rows(tree, paths=True):
            key = path if path is not None else "{0}"
            if only is not None and key not in only:
                continue
            if key != last:
                seen += 1
                if only is None and seen > count:
                    break
                rows.append(("path", "", key))
                last = key
            rows.append(("item", str(index), text))
        return rows

    def _swap(self, slot, new):
        """🔁 Cross-fade the mobject held in a named slot with a new one."""
        old, self._slots[slot] = self._slots.get(slot), new
        moves = [*([_engine.FadeOut(old)] if old is not None else []), *([_engine.FadeIn(new)] if new is not None else [])]
        return _engine.AnimationGroup(*moves) if moves else _engine.Wait(0.01)

    def _tower(self, lit=None, tint=None, types=False):
        """🗼 The full Nakagin tower drawing at the stage anchor."""
        return self.tower_axon(at=TOWER_AT, scale=TOWER_SCALE, view="axon", lit=lit, tint=tint, types=types)

    def _tag(self, string, point, color=INK, edge="c", size=SMALL):
        """🏷️ A coloured outlined badge."""
        return self.badge(string, point, color=color, size=size, filled=False, anchor={"l": "l", "r": "r", "c": "c"}[edge])

    def _chips(self, *specs):
        """🏷️ Coloured badges in one row along the top right edge, the last spec at the edge."""
        row, edge = _engine.VGroup(), 6.8
        for text, color in reversed(specs):
            chip = self._tag(text, (edge, 3.65), color, "r")
            row.add(chip)
            edge = chip.get_left()[0] - 0.15
        return row

    def _plate(self, point, width, height):
        """⬛ A dark plate that keeps light text readable on the grey canvas."""
        return _engine.RoundedRectangle(corner_radius=0.08, width=width, height=height, fill_color=_draft.BACKGROUND, fill_opacity=0.96, stroke_width=0).move_to((*point[:2], 0))

    def _diagram(self, tree, levels, *, lit=None, labels=True, leaf_color=None, big=False):
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

    def _flat_rows(self, start, lit=None):
        """📝 Rows of the flat Panel list from item ``start``."""
        return [("item", str(index), "Brep") for index in range(start, start + FLAT_LINES)] + [("line", "", "...")]

    def _pipe(self, op, tree, *, rows=None, count=3, lines=7, mapping=None):
        """🧵 Source, one component and a Panel, wired as trees."""
        panel = self.gh_panel(None, rows=rows if rows is not None else self._rows(tree, count), at=PANEL_AT, max_lines=lines, max_width=2.0)
        if op is None:
            link = self.gh_wire(self._src.grip_out(), panel.grip_in(), kind="tree")
            return _engine.VGroup(panel, link)
        comp = self.gh_component(op, at=COMP_AT)
        group = _engine.VGroup(comp, panel, self.gh_wire(self._src.grip_out(), comp.grip_in(0), kind="tree"), self.gh_wire(comp.grip_out(0), panel.grip_in(), kind="tree"))
        if mapping:
            text = self.gh_panel(None, rows=[("line", "", line) for line in mapping.split("->")][:1] + [("line", "", "->" + mapping.split("->")[1])], at=(-5.7, 1.38), max_lines=2, max_width=1.9)
            group.add(text, self.gh_wire(text.grip_out(), comp.grip_in(1), kind="item"))
        group.comp, group.panel = comp, panel
        return group

    def _mapping(self, source, target, point):
        """🗺️ A Path Mapper rule with segment letters in the colours of their levels."""
        letters = {"A": INK, "B": STRUCT, "C": GOOD, "D": PINK, "i": ITEM_C}
        parts = lambda text: [(c, letters.get(c, MUTED)) for c in text]
        row = _engine.VGroup(self._tokens(parts(source), (0, 0, 0), size=SMALL), self.mono("→", (0, 0, 0), size=SMALL, color=MUTED),
                             self._tokens(parts(target), (0, 0, 0), size=SMALL)).arrange(_engine.RIGHT, buff=0.12)
        return _engine.VGroup(self._plate(point, row.width + 0.3, 0.42), self._put(row, point))

    def _detail(self, lit, tint, scale, span):
        """🔍 A magnified slice of tower B around storey 4, seen from the south: only the storeys within ``span`` of it are drawn."""
        return self.tower_axon(at=(4.3, 0.6, 0.0), scale=scale, view=(235, 24), lit=lit, tint=tint, towers=(1,), core=False, capital=False, podium=False, types=False,
                               where=lambda slot: abs(slot.storey - 4) <= span)

    def _lit_all(self, count):
        """🔦 Predicate lighting the first ``count`` capsules in flat list order."""
        order = self._order
        return lambda key: order[key] < count

    def present_trees(self):
        """🎞️ The whole data-tree animation on the aggregated tower."""
        self._m = _model
        self._def = _model.canvas("trees")
        self._sol = _model.evaluate(self._def)
        self._tree = self._sol.tree(SOURCE)
        self._slots = {}
        slots = _nakagin.slots()
        self._order = {(*slot.path, slot.item): number for number, slot in enumerate(slots)}
        towers = {tower.index: tower for tower in _nakagin.towers()}
        per_storey = [len(self._sol.tree("pm", "Tree").branch(path)) for path in self._sol.tree("pm", "Tree").paths()]
        per_tower = [len(self._sol.tree("pt", "Tree").branch(path)) for path in self._sol.tree("pt", "Tree").paths()]
        materials = {c: self._sol.tree(c) for c in ("cp1", "cp2")}
        kind = lambda tree: {(*b.tag["slot"],): b.tag["material"] for path in tree.paths() for b in tree.branch(path)}
        paint = {c: kind(materials[c]) for c in materials}
        total = self._tree.stats()["item_count"]
        width = lambda value: self.number(value, 0)
        sheet = self.gh_background(at=SHEET_AT, width=SHEET_W, height=SHEET_H)
        self._src = self.gh_param("Brep", at=SRC_AT, label=self.tr("Capsules", "Kapseln"))
        self._dock = _engine.VGroup(sheet, self._src)
        pick = self._order[(0, 1, 4, 2, 1)]

        probe_index = 57
        probe = slots[probe_index]
        probe_key = (*probe.path, probe.item)

        flat_tree = self._sol.tree("fl", "Tree")
        flip_tree = self._sol.tree("fp", "Data")
        graft_tree = self._sol.tree("gr", "Tree")
        simple_tree = self._sol.tree("sm", "Tree")
        storey_tree = self._sol.tree("pm", "Tree")
        tower_tree = self._sol.tree("pt", "Tree")
        branch_tree = _model.Tree({(0, 0, 0, 0): self._tree.branch((0, 0, 0, 0))})

        self._slots["tower"] = tower = self._tower()
        flat_rows = self._flat_rows(0)
        self._slots["pipe"] = pipe = self._pipe(None, None, rows=flat_rows)
        count_chip = self._say(self.tr(f"{width(total)} capsules", f"{width(total)} Kapseln"), (HEAD_X, HEAD_Y), color=INK, edge="l")
        self._slots["head"] = count_chip
        self.step("hook", _engine.FadeIn(self._dock), _engine.FadeIn(tower), _engine.FadeIn(pipe), _engine.FadeIn(count_chip), seconds=1.4)

        frame = _engine.Rectangle(width=2.0, height=0.33, stroke_color=DATA, stroke_width=3, fill_opacity=0)
        rows57 = [("item", str(index), "Brep") for index in range(probe_index - 3, probe_index + 4)] + [("line", "", "...")]
        panel57 = self._pipe(None, None, rows=rows57)
        frame.move_to(panel57[0].parts["rows"][3].get_center() + np.array([0.0, 0.0, 0.0])).match_x(panel57[0].body)
        question = self._say(self.tr("tower? storey? facade?", "Turm? Geschoss? Fassade?"), (2.2, 3.5), color=DATA)
        lit_one = self._tower(lit=lambda key: key == probe_key)
        self.step("flat", self._swap("pipe", panel57), self._swap("tower", lit_one), _engine.FadeIn(frame), _engine.FadeIn(question), seconds=1.2)

        full_diag = self._diagram(self._tree, FULL)
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
        tags = self._chips((self.tr("B, storey 9: 4 facades", "B, Geschoss 9: 4 Fassaden"), STOREY_LIGHT), (self.tr("B, storey 10: 3 facades", "B, Geschoss 10: 3 Fassaden"), GOOD))
        self.step("stats", self._swap("pipe", stats_pipe), self._swap("tower", uneven), self._swap("tags", tags), seconds=1.4)

        def derived(op, tree, levels, mapping=None):
            """🧬 One restructuring result: component, Panel, diagram and count line all read from the evaluated tree."""
            return self._pipe(op, tree, mapping=mapping), self._diagram(tree, levels, big=len(tree.paths()) < 5), self._head(self._count_text(tree))

        def operate(key, op, small, big, levels, small_lit, seconds, tower_big=None):
            """🔬 Introduce an operation on the smallest example in its own clause, then run it on the whole tower in the next."""
            first, second = derived(op, small, levels[0]), derived(op, big, levels[1])
            stage = lambda parts, tower: [self._swap("pipe", parts[0]), self._swap("diag", parts[1]), self._swap("head", parts[2]), self._swap("tower", tower)]
            self.step(key + "_one", self._swap("tags", None), *stage(first, self._tower(lit=[small_lit])), seconds=seconds[0])
            self.step(key, *stage(second, tower_big or self._tower()), seconds=seconds[1])

        one_storey = self._small(self._tree, (0, 0, 0))
        sweep = self._tower(lit=self._lit_all(total), tint=lambda key: INK)
        operate("flatten", "Flatten Tree", one_storey.flatten(), flat_tree, ((5,), (5,)), (0, 0, 0), (1.6, 2.6), sweep)
        operate("graft", "Graft Tree", one_storey.graft(), graft_tree, ((0, 1, 2, 3, 4), (0, 1, 2, 3, 4)), (0, 0, 0), (1.6, 1.6))
        operate("simplify", "Simplify Tree", one_storey.simplify(), simple_tree, ((3,), (1, 2, 3)), (0, 0, 0), (1.6, 1.6))

        map_pipe = self._pipe("Path Mapper", storey_tree, mapping="{A;B;C;D}(i)->{A;B;C}(i)")
        map_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B;C}(i)", (-3.1, 1.17)))
        storey_diag = self._diagram(storey_tree, (0, 1, 2), big=True)
        self.step("mapper", self._swap("pipe", map_pipe), self._swap("diag", storey_diag), self._swap("head", self._head(self._count_text(storey_tree))), self._swap("tower", self._tower()), seconds=1.6)

        stages = []
        sizes = {(0, 0, 0): per_storey[0], (0, 1, 4): per_storey[9 + 4], (0, 1, 10): per_storey[-1]}
        names_s = {(0, 0, 0): ("A, storey 0", "A, Geschoss 0"), (0, 1, 4): ("B, storey 4", "B, Geschoss 4"), (0, 1, 10): ("B, storey 10", "B, Geschoss 10")}
        for prefixes in (((0, 0, 0),), ((0, 1, 10),), ((0, 1, 4), (0, 1, 10))):
            chips = self._chips(*[(self.tr(f"{names_s[p][0]}: {width(sizes[p])} capsules", f"{names_s[p][1]}: {width(sizes[p])} Kapseln"), GOOD) for p in prefixes])
            lit = self._tower(lit=list(prefixes), tint=lambda key, ps=prefixes: GOOD if any(key[:3] == p for p in ps) else None)
            stages.append(_engine.AnimationGroup(self._swap("tower", lit), self._swap("tags", chips)))
        self.step("perstorey", _engine.Succession(*stages), seconds=4.0)

        tower_pipe = self._pipe("Path Mapper", tower_tree, count=2, mapping="{A;B;C;D}(i)->{A;B}(i)")
        tower_pipe.add(self._mapping("{A;B;C;D}(i)", "{A;B}(i)", (-3.1, 1.17)))
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

        viewer_a = self.gh_param_viewer(_model.Tree({path: self._tree.branch(path) for path in self._tree.paths()[:3]}), at=(-3.85, 2.4), max_items=2)
        viewer_b = self.gh_param_viewer(_model.Tree({(0,): [None, None]}), at=(-1.4, 2.4), max_items=2)
        depth_chips = _engine.VGroup(self._say(self.tr("70 branches, depth 4", "70 Äste, Tiefe 4"), (-3.85, 1.2), size=SMALL, color=SHEET_INK),
                                     self._say(self.tr("1 branch, depth 1", "1 Ast, Tiefe 1"), (-1.4, 1.2), size=SMALL, color=SHEET_INK))
        why_pipe = _engine.VGroup(viewer_a, viewer_b, depth_chips)
        self.step("why", self._swap("pipe", why_pipe), self._swap("diag", _engine.VGroup(self._diagram(self._tree, FULL, lit=[(0, 0, 0, 0)]))), self._swap("tags", None), seconds=1.6)

        fix_map = self.gh_component("Path Mapper", at=(-2.9, 3.0), scale=0.8)
        fix_text = self.gh_panel(None, rows=[("line", "", "{A;B;C;D}(i)"), ("line", "", "->{A;B}(i)")], at=(-5.7, 1.38), max_lines=2, max_width=1.9)
        fix_colors = self.gh_panel(None, rows=[("item", "0", words[0]), ("item", "1", words[1])], at=(-3.8, 1.4), max_lines=2, max_width=1.6)
        fix_graft = self.gh_component("Graft Tree", at=(-2.25, 1.4), scale=0.8)
        fix_preview = self.gh_component("Custom Preview", at=(-0.95, 2.5), scale=0.8)
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

        self.step("recap", *self._recap(), seconds=1.6)

    def _recap(self):
        """📚 Replace the stage with the legend of ten terms, each with a mini tree computed by the engine."""
        gone = [_engine.FadeOut(mob) for mob in (*self._slots.values(), self._dock) if mob is not None] + [_engine.FadeOut(mob) for mob in self.mobjects if mob is not self._caption]
        base = _model.Tree.from_dict({"{0;0}": ["a", "b"], "{0;1}": ["c", "d"]})
        tower = self._tree
        rows = [
            (("Tree", "Datenbaum"), base, "{0;0} {0;1}", self.tr(f"{tower.path_count} branches · {tower.count} items", f"{tower.path_count} Äste · {tower.count} Elemente")),
            (("Branch", "Ast"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), "{0;0}", self.tr("2 items per facade", "2 Elemente je Fassade")),
            (("Path", "Pfad"), _model.Tree.from_dict({"{0;1}": ["a"]}), "{0;1}", "{0;1;4;2}"),
            (("Item", "Element"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), "", self.tr("one capsule", "eine Kapsel")),
            (("Index", "Index"), _model.Tree.from_dict({"{0;0}": ["a", "b"]}), "", "{0;1;4;2}[1]"),
            (("Flatten", "Flatten"), base.flatten(), "", f"{tower.path_count} × 2 → 1 × {tower.count}"),
            (("Graft", "Graft"), base.graft(), "", f"{tower.path_count} × 2 → {tower.count} × 1"),
            (("Simplify", "Simplify"), _model.Tree.from_dict({"{0;0;0}": ["a", "b"], "{0;0;1}": ["c", "d"]}).simplify(), "", "{0;1;4;2} → {1;4;2}"),
            (("Path Mapper", "Path Mapper"), base.map("{A;B}(i) -> {A}"), "", "{A;B;C;D}(i) → {A;B;C}"),
            (("Flip Matrix", "Flip Matrix"), base.flip(), "", f"{tower.path_count} × 2 → 2 × {tower.path_count}"),
        ]
        group = _engine.VGroup()
        for number, (term, tree, _, result) in enumerate(rows):
            column, line = divmod(number, 5)
            x0, y = -6.9 + column * 7.1, 3.2 - line * 1.12
            icon = self._mini(tree, (x0 + 3.0, y))
            group.add(self._say(self._word(term), (x0, y + (0.18 if term[0] != term[1] else 0.0)), size=LABEL, color=INK, edge="l"), icon,
                      self._mono(result, (x0 + 3.9, y), size=SMALL, color=DATA, edge="l"))
            if term[0] != term[1]:
                group.add(self._say("(" + self._word(term[::-1]) + ")", (x0, y - 0.2), size=SMALL, color=MUTED, edge="l"))
        self._slots, self._dock = {}, None
        return [*gone, _engine.FadeIn(group, lag_ratio=0.08)]

    def _mini(self, tree, point):
        """🌱 A small tree sketch of an engine tree: trunk, branches and leaves."""
        paths = tree.paths()
        depth = len(paths[0])
        leaves = [(p, i) for p in paths for i in range(len(tree.branch(p)))]
        count = len(leaves)
        xs = np.linspace(-0.55, 0.55, count) if count > 1 else np.array([0.0])
        ys = np.linspace(-0.42, 0.42, depth + 2)
        prefix_x = {}
        for x, (path, _) in zip(xs, leaves):
            for level in range(1, depth + 1):
                prefix_x.setdefault(path[:level], []).append(x)
        node = lambda prefix: np.array([point[0] + (0.0 if not prefix else sum(prefix_x[prefix]) / len(prefix_x[prefix])), point[1] + ys[len(prefix)], 0.0])
        pairs, group = [], _engine.VGroup()
        for prefix in prefix_x:
            pairs.append((node(prefix[:-1]), node(prefix)))
        for x, (path, _) in zip(xs, leaves):
            top = np.array([point[0] + x, point[1] + ys[depth + 1], 0.0])
            pairs.append((node(path), top))
            group.add(_engine.Dot(top, radius=0.045, color=ITEM_C))
        return _engine.VGroup(_draft.segments(pairs, INK, 1.6), group)
