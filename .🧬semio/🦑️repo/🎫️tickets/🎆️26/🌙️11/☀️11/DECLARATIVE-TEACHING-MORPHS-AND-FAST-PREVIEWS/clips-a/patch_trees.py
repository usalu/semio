"""One-shot patch of the trees clip to declarative transitions (ticket-local input)."""
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
FILE = ROOT / "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py"
text = FILE.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


def cut(start, end, new=""):
    global text
    a = text.index(start)
    b = text.index(end, a)
    text = text[:a] + new + text[b:]


sub('''PINK, ITEM_C, TOWER_GREY''', '''TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
PINK, ITEM_C, TOWER_GREY''')
cut("def _tower_flight(drawing, target, camera):", "class _DrawingRef:")
cut("class _Trip(_engine.Animation):", "def _sequence(scene, builders")
sub('''        scene.play(*(tuple(first) if index == 0 else ()), build(), run_time=seconds * weight)''',
    '''        scene.play(*(tuple(first) if index == 0 else ()), build(), run_time=seconds * weight)
        scene._settle()''')

cut("    def _flight(self, starts, ends, colors", "    def _centre(self, drawing, prefix):", '''    @staticmethod
    def _name(key):
        """🔑 Part name of a capsule path in a bound entity."""
        return "-".join(str(part) for part in key)

    def step(self, key, *animations, seconds=1.2):
        """⏱️ One narrated clause; flight targets that landed on identical drawn objects leave afterwards."""
        super().step(key, *animations, seconds=seconds)
        self._settle()

    def _fly(self, name, sources, targets):
        """✈️ Play the declared transition ``trees.<name>``; its target stays in ``self._born`` until ``_settle`` drops it from above the identical drawing."""
        if isinstance(sources, (dict, list)) and not sources:
            return _engine.Wait(0.01)
        animation = self.stage.transition(f"trees.{name}", source=sources, target=targets)
        self._born.append(self.stage.mobject(f"trees.{name}.target"))
        return animation

    def _copies(self, tower, keys):
        """🧸 Capsule copies of a tower drawing by part name; projected sprites when there are many."""
        if len(keys) <= 10:
            return {self._name(key): tower.capsules[key].copy() for key in keys}
        return {self._name(key): _sprites.capture(tower.capsules[key], self.camera) for key in keys}

''')
cut("    def _hatch(self, tower, keys, points, tones, size, label=\"\"):", "    def _settle(self):", '''    def _hatch(self, name, tower, keys, targets):
        """🌱 Capsule copies of a tower drawing fly into their ``{key: mobject}`` targets (one declared transition)."""
        return self._fly(name, self._copies(tower, keys), {self._name(key): targets[key] for key in keys})

''')
cut("    def _birth(self, tower, diagram):", "    def _gather(self, tower, keys, point, color):", '''    def _birth(self, name, tower, diagram):
        """🌱 Capsules of a tower drawing turn into the leaves of a tree diagram, in list order, keeping their tower colour."""
        keys = sorted((k for k in diagram.leaves if k in tower.capsules), key=self._order.get)
        many = len(keys) > 10
        leaf = lambda key: _morph.token("", kind="item", color=diagram.leaves[key][1], size=0.35).move_to(diagram.leaves[key][0]) if many else diagram.leaves[key][2].copy()
        return self._hatch(name, tower, keys, {key: leaf(key) for key in keys})

    def _regroup(self, name, old, new):
        """🔀 Leaves of an old tree diagram fly to the same capsules in a new diagram (Flatten, Graft, Simplify, Path Mapper, Flip Matrix)."""
        before, after = getattr(old, "leaves", {}), getattr(new, "leaves", {})
        keys = sorted((k for k in after if k in before), key=self._order.get)
        return self._fly(name, {self._name(k): before[k][2].copy() for k in keys}, {self._name(k): after[k][2].copy() for k in keys})

''')
cut("    def _gather(self, tower, keys, point, color):", "    def _tokens(self, parts, point", '''    def _gather(self, name, tower, keys, chip):
        """🧲 Capsules of a tower drawing collapse into one number or chip."""
        keys = sorted(keys, key=self._order.get)
        return self._fly(name, self._copies(tower, keys), chip.copy())

    def _paint(self, name, tower, swatches, colour_of):
        """🎨 Colour swatch copies leave their swatches and land on the capsules they paint."""
        keys = sorted((k for k in tower.capsules if k in self._order), key=self._order.get)
        return self._fly(name, {self._name(k): swatches[colour_of[k]].copy() for k in keys}, {self._name(k): tower.capsules[k].copy() for k in keys})

    def _fan(self, name, swatches, diagram, colour_of):
        """🎆 The grafted colour swatches fan out: a swatch copy flies to every leaf of the diagram and paints it in its item's colour."""
        keys = sorted((k for k in diagram.leaves if k in colour_of), key=self._order.get)
        return self._fly(name, {self._name(k): swatches[colour_of[k]].copy() for k in keys}, {self._name(k): diagram.leaves[k][2].copy() for k in keys})

''')
sub('''        if slot in ("head", "thread") and old is not None and new is not None:
            return _engine.Transform(old, new)''', '''        if slot in ("head", "thread") and old is not None and new is not None:
            self._slots[slot] = new
            return self.stage.transition(f"trees.{slot}-retext", source=old, target=new)''')

sub('''self._hatch(self._slots["tower"], [self._probe], [frame.get_center()], [DATA], 1.0, "21")''', '''self._hatch("flat-probe", self._slots["tower"], [self._probe], {self._probe: frame.copy()})''')
sub('''birth = self._birth(coloured, full_diag)''', '''birth = self._birth("birth-tree", coloured, full_diag)''')
sub('''self._birth(self._slots["tower"], branch_diag)''', '''self._birth("birth-branch", self._slots["tower"], branch_diag)''')
sub('''self._flight([self._centre(view, segments[:stage])], [digit.get_center()], [(roles_c[stage - 1], roles_c[stage - 1])], widths=(digit.width * 4, digit.width), arc=0.35, mobs=[digit.copy()])''',
    '''self._fly("path-segment", digit.copy().scale(4).move_to(self._centre(view, segments[:stage])), digit.copy())''')
sub('''        index_out = self._flight([seat], [index_chip.get_center()], [(ITEM_C, ITEM_C)], widths=(index_chip.width * 3, index_chip.width), arc=0.3, mobs=[index_chip.copy()])
        chip_back = self._flight([whole_chip.get_center()], [seat], [(DATA, DATA)], widths=(whole_chip.width, 0.35), arc=-0.3, mobs=[whole_chip.copy()])''',
    '''        index_out = self._fly("index-out", index_chip.copy().scale(3).move_to(seat), index_chip.copy())
        chip_back = self._fly("chip-back", whole_chip.copy(), whole_chip.copy().scale(0.35 / whole_chip.width).move_to(seat))''')
sub('''            pulls = [self._gather(uneven, [k for k in uneven.capsules if k[:3] == (0, 1, level)], tags[index].get_center(), color)
                     for level, index, color in ((top_level, 0, GOOD), (top_level - 1, 1, STOREY_LIGHT))]''',
    '''            pulls = [self._gather(f"gather-storey-{index}", uneven, [k for k in uneven.capsules if k[:3] == (0, 1, level)], tags[index])
                     for level, index in ((top_level, 0), (top_level - 1, 1))]''')
sub('''            stage = lambda parts, tower: [self._regroup(self._slots.get("diag"), parts[1]),''', '''            stage = lambda parts, tower, name: [self._regroup(f"regroup-{name}", self._slots.get("diag"), parts[1]),''')
sub('''*stage(first, self._tower(lit=[small_lit])), seconds=seconds[0])''', '''*stage(first, self._tower(lit=[small_lit]), key + "-one"), seconds=seconds[0])''')
sub('''self.step(key, *stage(second, tower_big), seconds=seconds[1])''', '''self.step(key, *stage(second, tower_big, key), seconds=seconds[1])''')
sub('''        letter_flight = self._flight([self._centre(example_tower, segments[:n + 1]) for n in range(4)], [letters_row[n].get_center() for n in range(4)], [(roles_c[n], roles_c[n]) for n in range(4)],
                                     widths=lambda m: (m.width * 3, m.width), lag=0.25, arc=0.3, mobs=[letters_row[n].copy() for n in range(4)])''',
    '''        letter_flight = self._fly("letters", {str(n): letters_row[n].copy().scale(3).move_to(self._centre(example_tower, segments[:n + 1])) for n in range(4)}, {str(n): letters_row[n].copy() for n in range(4)})''')
sub('''self.step("mapper", self._regroup(self._slots.get("diag"), storey_diag)''', '''self.step("mapper", self._regroup("regroup-mapper", self._slots.get("diag"), storey_diag)''')
sub('''            pulls = [self._gather(drawing, [k for k in drawing.capsules if k[:3] == p], chips[len(prefixes) - 1 - j].get_center(), GOOD) for j, p in enumerate(prefixes)]''',
    '''            pulls = [self._gather(f"gather-storey-{j}", drawing, [k for k in drawing.capsules if k[:3] == p], chips[len(prefixes) - 1 - j]) for j, p in enumerate(prefixes)]''')
sub('''            pulls = [self._gather(drawing, [k for k in drawing.capsules if k[1] == i], chips[len(group) - 1 - j].get_center(), TOWER_COLORS[i]) for j, i in enumerate(group)]''',
    '''            pulls = [self._gather(f"gather-tower-{j}", drawing, [k for k in drawing.capsules if k[1] == i], chips[len(group) - 1 - j]) for j, i in enumerate(group)]''')
sub('''first=[self._regroup(self._slots.get("diag"), tower_diag)''', '''first=[self._regroup("regroup-pertower", self._slots.get("diag"), tower_diag)''')
sub('''AnimationGroup(self._regroup(before_diag, after_diag)''', '''AnimationGroup(self._regroup("regroup-flip", before_diag, after_diag)''')
sub('''lambda: self._paint(stripes, {"teal": swatches[0], "orange": swatches[1]}, paint["cp1"])''', '''lambda: self._paint("paint-mistake", stripes, {"teal": swatches[0], "orange": swatches[1]}, paint["cp1"])''')
sub('''lambda: self._fan({"teal": fix_swatches[0], "orange": fix_swatches[1]}, fix_diag, paint["cp2"])''', '''lambda: self._fan("fan-fix", {"teal": fix_swatches[0], "orange": fix_swatches[1]}, fix_diag, paint["cp2"])''')
sub('''lambda: self._paint(done, {"teal": fix_swatches[0], "orange": fix_swatches[1]}, paint["cp2"])''', '''lambda: self._paint("paint-result", done, {"teal": fix_swatches[0], "orange": fix_swatches[1]}, paint["cp2"])''')
sub('''        short_list = self._flight([paths_plate[2].get_center()] * 2, [paths_plate[1].get_center()] * 2, [(INK, INK)] * 2, widths=lambda m: (m.width, m.width), lag=0.5, arc=0.3, mobs=[paths_plate[2].copy(), paths_plate[2].copy()])''',
    '''        short_list = self._fly("short-list", [paths_plate[2].copy(), paths_plate[2].copy()], [paths_plate[2].copy().move_to(paths_plate[1].get_center()) for _ in range(2)])''')
sub('''        home = self._flight([chip.get_center()], [self._slots["tower"].capsules[self._probe].get_center()], [(DATA, DATA)], widths=(chip.width, 0.4), arc=0.3, mobs=[chip.copy()])''',
    '''        home = self._fly("home-probe", chip.copy(), self._slots["tower"].capsules[self._probe].copy())''')
sub('''            if term[0] == "Flip Matrix":
                flight = _tower_flight(source, row[0], self.camera)
            else:
                flight = self._flight([source.get_center()], [row[0].get_center()], [(INK, INK)], widths=(source.width, max(row[0].width, 0.5)), arc=0.35, mobs=[source.copy()])''',
    '''            flier = _sprites.capture(source, self.camera) if term[0] == "Flip Matrix" else source.copy()
            flight = self._fly("recap-" + term[0].lower().replace(" ", "-"), flier, row[0].copy())''')
sub('''        self._def = _model.canvas("trees")
        self._sol''', '''        self.stage = _morph.stage(self, TRANSITIONS)
        self._def = _model.canvas("trees")
        self._sol''')
FILE.write_text(text, encoding="utf-8")
print("patched")
