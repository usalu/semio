"""One-shot exact-string migration of Beat2_References to declarative transitions (every edit must match exactly once)."""

import sys
from pathlib import Path

PATH = Path(sys.argv[1])

EDITS = [
    ('WK = \'0.00 "W/K"\'\n', 'WK = \'0.00 "W/K"\'\nTRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ('''    def _drop(self, *mobs, seconds=0.4):''', '''    def _to(self, name, source, target):
        """🔀 The declared transition `excel.references.<name>`: preview swap or full morph by the active render profile."""
        return self.morphs.transition(f"excel.references.{name}", source=source, target=target)

    def _still(self, mob):
        """🧊 A frame-frozen detached copy of a drawn part: the source of a morph while the live drawing stays."""
        frozen = mob.copy()
        for part in frozen.get_family():
            part.clear_updaters()
        return frozen

    def _ghosts(self, animation):
        """👻 The ghost a full morph leaves behind (none in a preview swap)."""
        ghost = getattr(animation, "ghost_mobject", None)
        return [] if ghost is None else [ghost]

    def _drop(self, *mobs, seconds=0.4):'''),
    ('    def _bar_retext(self, win, old, new, run_time=1.4):', '    def _bar_retext(self, win, old, new, transition):'),
    ('        return _morph.retext(before, after, run_time=run_time), after', '        return self._to(transition, before, after), after'),
    ('''        lang = self.language()
        base = _xl.workbook()
        relabel =''', '''        lang = self.language()
        self.morphs = _morph.stage(self, TRANSITIONS)
        base = _xl.workbook()
        relabel ='''),
    ('        lift = _morph.morph(dimension_token, cell_token, ghost=0.25, path_arc=0.5, run_time=1.6)', '        lift = self._to("hook-lift", dimension_token, cell_token)'),
    ('        self._drop(cell_token, lift.ghost_mobject)', '        self._drop(cell_token, *self._ghosts(lift))'),
    ('        face_shapes = [_morph.silhouette(self._face(faces, face)) for face, _ in hatched]', '        face_shapes = [self._still(self._face(faces, face)) for face, _ in hatched]'),
    ('_morph.scatter(face_shapes, area_tokens, lag=0.15, path_arc=0.5, run_time=1.8, pairs_fn=lambda s, d: [(s, d[0])], dip=0.15)', 'self._to("pointer-faces", face_shapes, area_tokens)'),
    ('_morph.scatter(edge_tokens, dimension_tokens, lag=0.3, path_arc=0.5, run_time=1.8)', 'self._to("pointer-edges", edge_tokens, dimension_tokens)'),
    ('_morph.scatter(dimension_tokens, marks, lag=0.2, path_arc=0.4, run_time=1.0)', 'self._to("pointer-marks", dimension_tokens, marks)'),
    ('        bar_shift, bar_after = self._bar_retext(win2, "=D3*E3", "=D4*E4")', '        bar_shift, bar_after = self._bar_retext(win2, "=D3*E3", "=D4*E4", "bar-shift")'),
    ('_morph.retext(mover, shifted, run_time=1.4)', 'self._to("shift-retext", mover, shifted)'),
    ('        surface_shapes = [_morph.silhouette(self._surface(faces, name, face)) for name, face in SURFACES]', '        surface_shapes = [self._still(self._surface(faces, name, face)) for name, face in SURFACES]'),
    ('_morph.scatter(surface_shapes, value_chips, lag=0.5, path_arc=0.5, run_time=1.0, limit=4.4, pairs_fn=lambda s, d: [(s, d[0])], dip=0.15)', 'self._to("lights-faces", surface_shapes, value_chips)'),
    ('        back_lift = _morph.morph(factor_cell, factor_face, ghost=0.25, path_arc=0.6, run_time=1.8)', '        back_lift = self._to("back-lift", factor_cell, factor_face)'),
    ('_m.FadeOut(back_lift.ghost_mobject)', '*(_m.FadeOut(ghost) for ghost in self._ghosts(back_lift))'),
    ('_morph.gather(parts, total_token, lag=0.12, run_time=1.6)', 'self._to("total-gather", parts, total_token)'),
    ('_morph.retext(movers[i - 1], form_rows[i], run_time=1.0)', 'self._to(("f4-second", "f4-third", "f4-fourth")[i - 1], movers[i - 1], form_rows[i])'),
    ('_morph.split(delta_chip, [inside_chip, outside_chip], lag=0.4, path_arc=0.5, run_time=1.6)', 'self._to("absolute-split", delta_chip, [inside_chip, outside_chip])'),
    ('_morph.scatter(loss_cells0, loss_faces0, lag=0.55, path_arc=0.5, run_time=0.9, limit=4.8)', 'self._to("absolute-losses", loss_cells0, loss_faces0)'),
    ('_morph.morph(slide_frames[0], slide_frames[1], keep_source=True, run_time=1.0)', 'self._to("slide-off-first", slide_frames[0], slide_frames[1])'),
    ('_morph.morph(slide_frames[0], slide_frames[2], keep_source=True, run_time=1.0)', 'self._to("slide-off-second", slide_frames[0], slide_frames[2])'),
    ('        bar_repair, bar_fixed = self._bar_retext(fixed, "=F4*D11", "=F4*$D$10")', '        bar_repair, bar_fixed = self._bar_retext(fixed, "=F4*D11", "=F4*$D$10", "bar-repair")'),
    ('_morph.retext(mover, repaired, run_time=1.4)', 'self._to("repair-retext", mover, repaired)'),
    ('_morph.scatter(loss_cells, loss_labels, lag=0.1, path_arc=0.5, run_time=2.0)', 'self._to("repair-losses", loss_cells, loss_labels)'),
    ('_morph.scatter(self.loss_labels, cell_chips, lag=0.08, path_arc=0.5, run_time=1.0)', 'self._to("mixed2-sync", self.loss_labels, cell_chips)'),
    ('        label_sources = [_morph.silhouette(self._surface(faces, name, face)) for name, face in (("roof", "top"), ("wall_left", "left"), ("window", "front"))]', '        label_sources = [self._still(self._surface(faces, name, face)) for name, face in (("roof", "top"), ("wall_left", "left"), ("window", "front"))]'),
    ('_morph.scatter(label_sources, label_targets, lag=0.4, path_arc=0.5, run_time=1.8, pairs_fn=lambda s, d: [(s, d[0])])', 'self._to("mixed2-labels", label_sources, label_targets)'),
    ('_morph.scatter(centre_cells, bars, lag=0.25, path_arc=0.4, run_time=1.6)', 'self._to("mixed2-bars", centre_cells, bars)'),
    ('        capsule_shapes = [_morph.silhouette(counted.capsules[key]) for key in sorted(self._tower_cell(base, "B11"))]', '        capsule_shapes = [self._still(counted.capsules[key]) for key in sorted(self._tower_cell(base, "B11"))]'),
    ('_morph.gather(capsule_shapes, count_token, lag=0.06, path_arc=0.5, run_time=1.8)', 'self._to("mixed-count", capsule_shapes, count_token)'),
    ('            shapes = [_morph.silhouette(built.capsules[key]) for key in sorted(keys)]', '            shapes = [self._still(built.capsules[key]) for key in sorted(keys)]'),
    ('_morph.split(chip, shapes, lag=0.04, path_arc=0.5, run_time=0.8)', 'self._to("fill2d-split", chip, shapes)'),
    ('_morph.gather(shapes, chip, lag=0.04, path_arc=0.5, run_time=0.8)', 'self._to("fill2d-gather", shapes, chip)'),
    ('        floor_shape = _morph.silhouette(plan.faces["top"])', '        floor_shape = self._still(plan.faces["top"])'),
    ('_morph.morph(floor_shape, area_token, pairs=[(floor_shape, area_token[0])], path_arc=0.6, run_time=1.6, dip=0.15)', 'self._to("sheet-floor", floor_shape, area_token)'),
    ('_morph.scatter(tab_chips, row_chips, lag=0.25, path_arc=0.5, run_time=1.6)', 'self._to("pertype-tabs", tab_chips, row_chips)'),
    ('_morph.scatter(self.row_chips, capsules, lag=0.15, path_arc=0.4, run_time=1.0)', 'self._to("threed-chips", self.row_chips, capsules)'),
    ('        sources = [_morph.silhouette(capsule) for capsule in capsules]', '        sources = [self._still(capsule) for capsule in capsules]'),
    ('_morph.scatter(sources, values, lag=0.15, path_arc=0.3, run_time=0.9)', 'self._to("threed-values", sources, values)'),
    ('_morph.gather(values, total_text, lag=0.2, path_arc=0.4, run_time=2.0)', 'self._to("threed-sum", values, total_text)'),
    ('_morph.scatter(capsules, capsules2, lag=0.1, path_arc=0.0, run_time=0.8)', 'self._to("span-chips", capsules, capsules2)'),
    ('        sources2 = [_morph.silhouette(capsule) for capsule in capsules2]', '        sources2 = [self._still(capsule) for capsule in capsules2]'),
    ('_morph.scatter(sources2, values2, lag=0.15, path_arc=0.3, run_time=0.9)', 'self._to("span-values", sources2, values2)'),
    ('_morph.morph(tab_src, tab_dst, path_arc=-0.6, run_time=0.9)', 'self._to("span-tab", tab_src, tab_dst)'),
    ('_morph.gather(counted2, total2, lag=0.2, path_arc=0.4, run_time=2.0)', 'self._to("span-sum", counted2, total2)'),
    ('_morph.morph(sum_source, sum_target, path_arc=0.5, run_time=1.4)', 'self._to("deleted-sum", sum_source, sum_target)'),
    ('_morph.morph(ref_source, ref_target, path_arc=0.5, run_time=0.9)', 'self._to("guard-ref", ref_source, ref_target)'),
    ('        bar_guard, bar_broken = self._bar_retext(win9, "=F3*$D$10", "=F3*#REF!", run_time=1.6)', '        bar_guard, bar_broken = self._bar_retext(win9, "=F3*$D$10", "=F3*#REF!", "bar-guard")'),
    ('_morph.retext(mover, broken_row, run_time=1.6)', 'self._to("guard-retext", mover, broken_row)'),
    ('        bar_names, bar_named = self._bar_retext(named, "=F3*(Climate!$B$2-Climate!$B$3)", "=F3*(theta_i-theta_e)", run_time=1.6)', '        bar_names, bar_named = self._bar_retext(named, "=F3*(Climate!$B$2-Climate!$B$3)", "=F3*(theta_i-theta_e)", "bar-names")'),
    ('_morph.retext(mover, readable, run_time=1.6)', 'self._to("names-retext", mover, readable)'),
    ('_morph.scatter(term_chips, examples, lag=0.3, path_arc=0.5, run_time=2.0)', 'self._to("recap-terms", term_chips, examples)'),
]

text = PATH.read_bytes().decode("utf-8")
crlf = "\r\n" in text
text = text.replace("\r\n", "\n")
for old, new in EDITS:
    n = text.count(old)
    if n != 1:
        print(f"EDIT FAILED ({n} matches): {old[:100]!r}")
        sys.exit(1)
    text = text.replace(old, new)
left = [line for line in text.splitlines() if "_morph." in line and "import_module" not in line]
if crlf:
    text = text.replace("\n", "\r\n")
PATH.write_bytes(text.encode("utf-8"))
print(f"ok {len(EDITS)} edits; remaining _morph. uses: {len(left)}")
for line in left:
    print(line.strip()[:140])
