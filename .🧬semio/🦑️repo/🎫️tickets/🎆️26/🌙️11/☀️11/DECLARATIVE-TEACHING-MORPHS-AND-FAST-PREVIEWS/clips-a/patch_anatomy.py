"""One-shot patch of the anatomy clip to declarative transitions (ticket-local input)."""
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
FILE = ROOT / "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py"
text = FILE.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:90])
    text = text.replace(old, new)


sub('''_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")''', '''_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")''')
sub('''NB, NA = (next(''', '''TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"
NB, NA = (next(''')

sub('''    def _hop(self, label, start, stop, color, kind="number", ghost=0.0, first=None, size=1.0):
        """🔀 A token born at ``start`` that morphs into its twin at ``stop``; returns the animation and the arriving token."""
        source, target = self._bit(label if first is None else first, start, color, kind, size), self._bit(label, stop, color, kind, size)
        self.add(source)
        return _morph.morph(source, target, ghost=ghost, path_arc=0.5, run_time=1.2, dip=0.12), target
''', '''    def _move(self, name, source, target):
        """🔀 The declared transition ``anatomy.<name>`` between two objects; the active profile decides how it plays."""
        return self.stage.transition(f"anatomy.{name}", source=source, target=target)

    def _hop(self, name, label, start, stop, color, kind="number", first=None, size=1.0):
        """🔀 A token born at ``start`` that moves into its twin at ``stop`` by the declared transition; returns the animation and the arriving token."""
        source, target = self._bit(label if first is None else first, start, color, kind, size), self._bit(label, stop, color, kind, size)
        self.add(source)
        return self._move(name, source, target), target
''')
sub('''    def _cap_to(self, key, towers, token):
        """🔀 A capsule copy that flies part-wise into a token (shell to ring, window to dot); returns the animation."""
        source = self._cap(key, towers)
        self.add(source)
        return _morph.morph(source, token, pairs=_morph.capsule_pairs(source, token), path_arc=0.5, run_time=1.2, dip=0.15)

    def _to_cap(self, token, key, towers, run_time=1.2, arc=0.5):
        """🔀 A token that flies part-wise into a capsule copy; returns the animation and the arriving capsule."""
        target = self._cap(key, towers)
        return _morph.morph(token, target, pairs=[(b, a) for a, b in _morph.capsule_pairs(target, token)], path_arc=arc, run_time=run_time, dip=0.15), target
''', '''    def _cap_to(self, name, key, towers, token):
        """🔀 A capsule copy that becomes a token by the declared capsule-item transition; returns the animation."""
        source = self._cap(key, towers)
        self.add(source)
        return self._move(name, source, token)
''')
sub('''    def _fly_discs(self, markers, table, order):
        """🔢 The numbered markers on the canvas fly into the number discs of their legend rows."""
        sources = []''', '''    def _fly_discs(self, name, markers, table, order):
        """🔢 The numbered markers on the canvas move into the number discs of their legend rows."""
        sources = []''')
sub('''        return _morph.scatter(sources, targets, lag=0.15, path_arc=0.8, run_time=1.3, limit=3.2, pairs_fn=lambda a, b: [(a[0], b[0]), (a[1], b[1])])''',
    '''        return self._move(name, sources, targets)''')
sub('''        self.play(_morph.retext(row, fixed, run_time=1.2), run_time=1.2)
        self.play(_engine.Transform(old.box, new.box), _engine.FadeOut(old.bubble), _engine.FadeOut(old.gloss), _engine.Transform(old.panel, new.panel), *[_engine.FadeOut(m) for m in extra], run_time=1.2)''',
    '''        self.play(self._move("repair-text", row, fixed), run_time=1.2)
        self.play(self._move("repair-box", old.box, new.box), _engine.FadeOut(old.bubble), _engine.FadeOut(old.gloss), self._move("repair-panel", old.panel, new.panel), *[_engine.FadeOut(m) for m in extra], run_time=1.2)''')

sub('''        self.play(_morph.morph(lit_capsule, chip, pairs=_morph.capsule_pairs(lit_capsule, chip), path_arc=0.4, run_time=1.4, dip=0.15), run_time=1.4)''',
    '''        self.play(self._move("script-capsule-to-chip", lit_capsule, chip), run_time=1.4)''')
sub('''self._hop(self.number(settings["w"]["value"], 1), _at(SL_X + 1.3, 1.0)''', '''self._hop("slider-width-hop", self.number(settings["w"]["value"], 1), _at(SL_X + 1.3, 1.0)''')
sub('''        hops = [self._hop(self.number(settings[k]["value"], 1), VIEW + np.array([-1.3 + 1.3 * i, -1.1, 0.0]), path[len(path) // 2], color, size=0.8)
                for i, ((k, color), path) in enumerate(zip((("w", cw), ("bl", cl), ("bh", ch)), wire_paths))]''',
    '''        hops = [self._hop(f"box-{axis}-hop", self.number(settings[k]["value"], 1), VIEW + np.array([-1.3 + 1.3 * i, -1.1, 0.0]), path[len(path) // 2], color, size=0.8)
                for i, ((k, color, axis), path) in enumerate(zip((("w", cw, "width"), ("bl", cl, "length"), ("bh", ch, "height")), wire_paths))]''')
sub('''*[_morph.morph(line, curve, path_arc=0.4, run_time=1.4, dip=0.1) for line, curve in zip(dimension_lines, wire_curves)]''',
    '''*[self._move(f"box-dimension-{axis}-to-wire", line, curve) for axis, line, curve in zip(("width", "length", "height"), dimension_lines, wire_curves)]''')
sub('''        self.play(_morph.morph(out_token, out_target, path_arc=0.5, run_time=1.4, dip=0.1), run_time=1.4)''',
    '''        self.play(self._move("box-output-token", out_token, out_target), run_time=1.4)''')
sub('''self._hop(self.number(l0 + 1.0, 1), l_slider''', '''self._hop("sliders-length-hop", self.number(l0 + 1.0, 1), l_slider''')
sub('''self._hop(self.number(h0 + 0.5, 1), h_slider''', '''self._hop("sliders-height-hop", self.number(h0 + 0.5, 1), h_slider''')
sub('''self._hop(vp_rows[-1][2], VIEW''', '''self._hop("volume-hop", vp_rows[-1][2], VIEW''')
sub('''_engine.Transform(eye, self._eye(cb, on=False))''', '''self._move("preview-eye-off", eye, self._eye(cb, on=False))''')
sub('''        self.play(_morph.morph(volume_end, volume_ghost, path_arc=0.7, run_time=1.4), run_time=1.4)''',
    '''        self.play(self._move("volume-ghost", volume_end, volume_ghost), run_time=1.4)''')
sub('''self._hop(self.number(settings["w"]["value"], 1), VIEW + np.array([-0.2, -1.1, 0.0]), wp.get_left()''', '''self._hop("panel-hop", self.number(settings["w"]["value"], 1), VIEW + np.array([-0.2, -1.1, 0.0]), wp.get_left()''')
sub('''self._hop(self.number(settings["w"]["value"], 1), wp.get_left() + np.array([-0.5, 0.0, 0.0]), fl.get_center()''', '''self._hop("params-hop", self.number(settings["w"]["value"], 1), wp.get_left() + np.array([-0.5, 0.0, 0.0]), fl.get_center()''')
sub('''_morph.morph(true_token, window_ring, path_arc=0.5)''', '''self._move("toggle-true-to-window", true_token, window_ring)''')
sub('''self._hop("C", _at(-1.9, 0.0)''', '''self._hop("valuelist-pick-hop", "C", _at(-1.9, 0.0)''')
sub('''_morph.morph(sun_ring, sun_token, pairs=[(sun_ring, sun_token[0])], path_arc=0.5, dip=0.12)''', '''self._move("plugin-ring-to-token", sun_ring, sun_token)''')
sub('''self._cap_to(level_key(1, 0), (1,), item_end)''', '''self._cap_to("item-capsule-to-token", level_key(1, 0), (1,), item_end)''')
sub('''        self.play(_morph.scatter(series_src, series_dst, lag=0.1, path_arc=0.5, run_time=1.0, limit=2.2, dip=0.12, pairs_fn=_morph.capsule_pairs))''',
    '''        self.play(self._move("series-capsules-to-tokens", series_src, series_dst))''')
sub('''        count_hops = [self._hop(count_texts[i], count_points[i], self._spot(level_key(0, i), (0,)) + np.array([0.95, 0.4 * (i - (NB + NA - 1) / 2), 0.0]), level_color(0, i)) for i in range(NB, NA)]
        self.play(*[a for a, _ in count_hops], run_time=1.6)
        self.play(*[_engine.FadeOut(t) for _, t in count_hops], run_time=0.4)''',
    '''        count_sources = [self._bit(count_texts[i], count_points[i], level_color(0, i), "number") for i in range(NB, NA)]
        count_targets = [self._bit(count_texts[i], self._spot(level_key(0, i), (0,)) + np.array([0.95, 0.4 * (i - (NB + NA - 1) / 2), 0.0]), level_color(0, i), "number") for i in range(NB, NA)]
        self.add(*count_sources)
        self.play(self._move("count-capsule-hops", count_sources, count_targets), run_time=1.6)
        self.play(*[_engine.FadeOut(t) for t in count_targets], run_time=0.4)''')
sub('''        leaf_hops = []
        for i, item in enumerate(chosen_items):
            row = tree_panel.parts["rows"][row_indices[chosen_path][i]]
            target = self._bit("", row.get_left() + np.array([-0.14, 0.0, 0.0]), _drawing.type_color(item.tag["type"]), "item")
            leaf_hops.append((self._cap_to(tuple(item.tag["slot"]), (0,), target), target))''',
    '''        leaf_sources, leaf_targets = [], []
        for i, item in enumerate(chosen_items):
            row = tree_panel.parts["rows"][row_indices[chosen_path][i]]
            leaf_targets.append(self._bit("", row.get_left() + np.array([-0.14, 0.0, 0.0]), _drawing.type_color(item.tag["type"]), "item"))
            leaf_sources.append(self._cap(tuple(item.tag["slot"]), (0,)))
        self.add(*leaf_sources)''')
sub('''        self.play(*[a for a, _ in leaf_hops], run_time=1.2)
        self.play(*[_engine.FadeOut(t) for _, t in leaf_hops], run_time=0.3)''',
    '''        self.play(self._move("tree-capsules-to-leaves", leaf_sources, leaf_targets), run_time=1.2)
        self.play(*[_engine.FadeOut(t) for t in leaf_targets], run_time=0.3)''')
sub('''        self.play(_morph.gather(segments, path_chip, lag=0.2, run_time=1.0))''', '''        self.play(self._move("path-segments-to-chip", segments, path_chip))''')
sub('''        self.play(_morph.split(index_token, flock, lag=0.04, path_arc=0.3, run_time=0.9, limit=2.4))''', '''        self.play(self._move("index-token-to-flock", index_token, flock))''')
sub('''self.play(self._fly_discs(marks, table, self.TOOL_ORDER))''', '''self.play(self._fly_discs("legend-tool-discs", marks, table, self.TOOL_ORDER))''')
sub('''self.play(self._fly_discs(data_marks, data_table, self.DATA_ORDER))''', '''self.play(self._fly_discs("legend-data-discs", data_marks, data_table, self.DATA_ORDER))''')
sub('''        defn = _gh.canvas("anatomy")
        full = _gh.evaluate(defn)
        settings = {k''', '''        self.stage = _morph.stage(self, TRANSITIONS)
        defn = _gh.canvas("anatomy")
        full = _gh.evaluate(defn)
        settings = {k''')
FILE.write_text(text, encoding="utf-8")
print("patched")
