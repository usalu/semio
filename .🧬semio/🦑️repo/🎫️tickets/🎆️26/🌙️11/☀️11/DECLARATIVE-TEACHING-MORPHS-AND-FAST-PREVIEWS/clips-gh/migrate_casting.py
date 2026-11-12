"""Replace the toolkit morph calls of the casting clip by declarative `hop(...)` transitions (one-time hand migration; kept as the audit trail)."""
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "AGENTS.md").exists())
FILE = ROOT / "🎓️teaching" / "🏛️architecture" / "🦗️grasshopper" / "🎬️clip" / "🫗️casting" / "🐍️.py"
text = FILE.read_text(encoding="utf-8", newline="")

PAIRS = [
    ('_morph = importlib.import_module(', '_silhouette = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🫥️silhouette.🐍️")\n_morph = importlib.import_module('),
    ('SMALL = 16.2\n', 'SMALL = 16.2\nTRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\nHABITS = ("recap.cells-to-habit-one", "recap.cells-to-habit-two", "recap.cells-to-habit-three")\n'),
    ('        self._d, self._cache, self._towers = _model.canvas("casting"), {}, {}\n',
     '        self._d, self._cache, self._towers = _model.canvas("casting"), {}, {}\n        morphs = _morph.stage(self, TRANSITIONS)\n        hop = lambda name, source, target: morphs.transition(f"grasshopper.casting.{name}", source=source, target=target)\n'),
    ('        for after, after_seconds in phases:\n            self.play(*after, run_time=after_seconds)\n',
     '        for after, after_seconds in phases:\n            self.play(*(after() if callable(after) else after), **({} if after_seconds is None else {"run_time": after_seconds}))\n'),
    ('_morph.scatter(sources, chips, lag=0.3, path_arc=0.5, run_time=1.4)', 'hop("types.sources-to-chips", sources, chips)'),
    ('_morph.retext(quoted, chip, run_time=0.7)', 'hop("text-number.quoted-to-chip", quoted, chip)'),
    ('_morph.morph(chip, built, pairs=[(chip[0], built[0]), (chip[1], built[1])], path_arc=0.4, dip=0.15, run_time=1.5)', 'hop("text-number.chip-to-capsule", chip, built)'),
    ('_morph.retext(sl_chip, int_chip, run_time=0.7)', 'hop("storeys.slider-to-integer", sl_chip, int_chip)'),
    ('_morph.split(int_chip, bands, lag=0.15, path_arc=-0.3, run_time=1.2)', 'hop("storeys.integer-to-bands", int_chip, bands)'),
    ('self._early(_morph.gather(bands, line_base.dot, lag=0.05, run_time=1.2), 0.4)', 'hop("ties.bands-to-dot", bands, line_base.dot)'),
    ('ramp(2.5, 3.5))), 4.0, 0.8))', 'ramp(2.5, 3.5))), None, 0.8))'),
    ('_morph.morph(eleventh, lost, path_arc=0.5, run_time=1.2)', 'hop("deliberate.storey-to-lost", eleventh, lost)'),
    ('origin_move = _morph.morph(origin_dot, origin_chip, ghost=0.3, path_arc=-0.5, run_time=1.3)', 'origin_move = hop("point-plane.origin-to-chip", origin_dot, origin_chip)'),
    ('self.then((origin_move,), 1.3, 1.2)', 'self.then((origin_move, origin_dot.animate.set_opacity(0.3)), 1.3, 1.2)'),
    ('origin_move.ghost_mobject', 'origin_dot'),
    ('_morph.morph(world_plane, red, path_arc=0.3, run_time=1.4)', 'hop("wrong-way.plane-to-red", world_plane, red)'),
    ('_morph.morph(line_mob, z_axes, path_arc=0.2, run_time=1.4)', 'hop("line-plane.line-to-axes", line_mob, z_axes)'),
    ('_morph.gather(inputs, built_plane, run_time=1.2)', 'hop("explicit.inputs-to-plane", inputs, built_plane)'),
    ('_morph.morph(anchor, world[0], path_arc=0.0, run_time=1.4)', 'hop("domain.anchor-to-line", anchor, world[0])'),
    ('_morph.morph(length_chip, world[1], path_arc=0.4, run_time=1.4)', 'hop("domain.length-to-interval", length_chip, world[1])'),
    ('_morph.morph(world7_rings, world7_chips[0], path_arc=0.4, run_time=1.4)', 'hop("geometry.rings-to-chip", world7_rings, world7_chips[0])'),
    ('_morph.morph(box_src, world7_chips[1], path_arc=-0.4, run_time=1.4)', 'hop("geometry.body-to-chip", box_src, world7_chips[1])'),
    ('((_morph.morph(one, ring_on, run_time=0.9),), 0.9)', '(lambda: (hop("bool-int.one-to-ring", one, ring_on),), 0.9)'),
    ('((_morph.morph(true_chip, ring_two, run_time=0.9),), 0.9)', '(lambda: (hop("int-bool.true-to-ring", true_chip, ring_two),), 0.9)'),
    ('((_morph.split(comma, [rest, red_comma], lag=0.1, path_arc=0.2, run_time=0.8), _mn.FadeIn(stop, scale=0.6)), 0.9)', '(lambda: (hop("locale.comma-split", comma, [rest, red_comma]), _mn.FadeIn(stop, scale=0.6)), 0.9)'),
    ('_morph.morph(ghost11, blank, pairs=_morph.capsule_pairs(ghost11, blank), path_arc=0.4, dip=0.15, run_time=1.4)', 'hop("fail.capsule-to-null", ghost11, blank)'),
    ('_morph.morph(typea, chip3, pairs=[(typea[0], chip3[0]), (typea[1], chip3[1])], path_arc=0.4, run_time=1.2)', 'hop("fix.type-to-chip", typea, chip3)'),
    ('_morph.morph(chip3, built3, pairs=[(chip3[0], built3[0]), (chip3[1], built3[1])], path_arc=0.4, dip=0.15, run_time=1.5)', 'hop("fix.chip-to-capsule", chip3, built3)'),
    ('(_mn.LaggedStart(*[_mn.FadeTransform(s, d) for s, d in zip(seen, cells)], lag_ratio=0.1, run_time=1.0),)', 'lambda: (hop("matrix-fill.chips-to-cells", seen, list(cells)),)'),
    ('_morph.gather([cells[i] for i in picks], _mn.VGroup(*habits[4 * k:4 * k + 4]), lag=0.1, run_time=1.4)', 'hop(HABITS[k], [cells[i] for i in picks], _mn.VGroup(*habits[4 * k:4 * k + 4]))'),
    ('_morph.silhouette(cap, color=color)', '_silhouette.silhouette(cap, color=color)'),
    ('_morph.silhouette(world7[0], color=INK_GOLD)', '_silhouette.silhouette(world7[0], color=INK_GOLD)'),
]
for old, new in PAIRS:
    if text.count(old) != 1:
        sys.exit(f"expected exactly one occurrence of: {old[:90]} (found {text.count(old)})")
    text = text.replace(old, new)
FILE.write_text(text, encoding="utf-8", newline="")
print("migrated", len(PAIRS), "replacements")
